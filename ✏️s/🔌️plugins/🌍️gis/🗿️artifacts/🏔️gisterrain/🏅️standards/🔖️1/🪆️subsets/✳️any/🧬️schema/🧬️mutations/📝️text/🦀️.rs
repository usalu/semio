//! ⚡️ Gis3dTerrain artifact — OpText/OpBinary codecs + grammar for `GisTerrainMutation`.

pub use crate::schema::mutations::{apply_gis_terrain_mutation, inverse_gis_terrain_mutation, GisTerrainMutation};

pub const TEXT_OPCODES: &[(&str, &str)] = &[("ChangeExaggeration", super::change_exaggeration::text::TEXT_OPCODE), ("ChangeImportedFeatures", super::change_imported_features::text::TEXT_OPCODE)];

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for GisTerrainMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for GisTerrainMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
