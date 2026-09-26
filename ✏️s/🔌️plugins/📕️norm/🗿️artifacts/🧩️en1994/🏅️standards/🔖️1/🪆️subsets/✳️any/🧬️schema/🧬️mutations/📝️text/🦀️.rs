//! ⚡️ En1994 mutations — OpText/OpBinary via JSON tokens (hierarchical subject).

pub use crate::artifact_schema::mutations::En1994Mutation;

use protocol::OpText;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

impl protocol::OpText for En1994Mutation {
    fn print_op(&self) -> String {
        pack::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        pack::json::from_json_str(line).map_err(|e| store::TextError::new(e.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for En1994Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(text).map_err(|e| protocol::ProtocolError::Malformed { what: "json", offset: 0, detail: e.to_string() })
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use protocol::{OpBinary, OpText};
    use crate::artifact_schema::mutations::demo_mutation_cases;

    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line");
            let parsed = <En1994Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed: {e}"));
            assert_eq!(parsed, mutation);
            let encoded = mutation.encode_op().unwrap();
            let decoded = <En1994Mutation as OpBinary>::decode_op(&encoded).unwrap();
            assert_eq!(decoded, mutation);
        }
    }
}
