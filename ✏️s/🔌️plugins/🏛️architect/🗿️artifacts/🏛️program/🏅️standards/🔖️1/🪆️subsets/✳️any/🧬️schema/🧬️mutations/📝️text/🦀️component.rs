//! ⚡️ Architect program artifact — OpText/OpBinary codecs + grammar for serializing `ProgramMutation`.
//! Mutation apply/inverse live in `🧬️mutations`.

pub use crate::artifacts::program::schema::mutations::ProgramMutation;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// @emoji 📝️ Compact JSON-line OpText for `ProgramMutation` (collection wrappers block DslEnum).
impl protocol::OpText for ProgramMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| store::TextError::new(format!("invalid program mutation: {e}"), store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        serde_json::to_string(self).expect("ProgramMutation always serializes")
    }
}

/// @emoji 🌱️ Binary twin of the OpText escape hatch — plain JSON bytes.
impl protocol::OpBinary for ProgramMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Malformed { what: "program operation", offset: 0, detail: error.to_string() })
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "program operation", offset: 0, detail: error.to_string() })
    }
}

//#endregion 🔖️HandcraftedOpCodecs
