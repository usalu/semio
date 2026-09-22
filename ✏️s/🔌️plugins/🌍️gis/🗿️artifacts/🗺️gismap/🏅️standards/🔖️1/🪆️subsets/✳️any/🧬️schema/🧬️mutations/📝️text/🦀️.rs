//! ⚡️ GIS map artifact — OpText/OpBinary codecs + grammar for `GisMapMutation`.

pub use crate::schema::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits). `GisMapMutation`
/// derives `dsl::DslEnum` directly (every variant wraps a local `dsl::DslRecord` payload, no more
/// foreign `protocol::CollectionMutation` in its shape), so this is a pure `DslVariants` pass-through
/// — no local DSL-mirror type needed, unlike the pre-taxonomy-overhaul version of this file.
impl protocol::OpText for GisMapMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for GisMapMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
