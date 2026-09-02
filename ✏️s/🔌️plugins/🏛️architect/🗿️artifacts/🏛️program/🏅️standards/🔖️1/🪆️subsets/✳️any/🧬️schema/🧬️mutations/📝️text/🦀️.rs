//! ⚡️ Architect program artifact — OpText/OpBinary codecs + grammar for serializing `ProgramMutation`.
//! Mutation apply/inverse live in `🧬️mutations`.

pub use crate::artifacts::program::schema::mutations::ProgramMutation;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// @emoji 📝️ Compact JSON-line OpText for `ProgramMutation` (collection wrappers block DslEnum).
impl protocol::OpText for ProgramMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line.trim()).map_err(|e| store::TextError::new(format!("invalid program mutation: {e}"), store::TextSpan::at(1, 1)))
    }

    async fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
}

/// @emoji 🌱️ Binary twin of the OpText escape hatch — plain JSON bytes.
impl protocol::OpBinary for ProgramMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(dsl::json::to_json_string(self).into_bytes())
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "program operation", offset: 0, detail: error.to_string() })?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Malformed { what: "program operation", offset: 0, detail: error.to_string() })
    }
}

//#endregion 🔖️HandcraftedOpCodecs
