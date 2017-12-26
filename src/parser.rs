#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SSHConfigParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::inputs::FileInput;
    use pest::Parser;
    use std::rc::Rc;

    #[test]
    fn parse_config() {
        let config_file = FileInput::new("test/ssh_config").unwrap();

        let pairs = SSHConfigParser::parse(Rule::config_file, Rc::new(config_file))
            .unwrap_or_else(|e| panic!("{}", e));

        for a in pairs {
            println!("{:?}", a.as_str());
        }
    }
}
