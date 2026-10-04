//! ⚡️ Architect program artifact — OpText/OpBinary codecs + grammar for serializing `ProgramMutation`.
//! Mutation apply/inverse live in `🧬️mutations`.

pub use crate::schema::mutations::ProgramMutation;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// 📝️ Compact JSON-line OpText for `ProgramMutation` (collection wrappers block DslEnum).
impl protocol::OpText for ProgramMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line.trim(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("invalid program mutation: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
}

//#region 🏷️WireTags
/// 🏷️ `ProgramMutation`'s wire protocol: its `record <kind> tag=<n>` lines are the only source of the op tags.
const WIRE_PROTOCOL: &str = include_str!("../💾️binary/📡️.protocol.semio");
//#endregion 🏷️WireTags

/// 🌱️ Binary twin of the OpText escape hatch — plain JSON bytes.
impl protocol::OpBinary for ProgramMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::tagged_value_binary::encode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::tagged_value_binary::decode_op(WIRE_PROTOCOL, dsl::tagged_value_binary::VariantTag::Field("mutation"), bytes)
    }
}

//#endregion 🔖️HandcraftedOpCodecs
