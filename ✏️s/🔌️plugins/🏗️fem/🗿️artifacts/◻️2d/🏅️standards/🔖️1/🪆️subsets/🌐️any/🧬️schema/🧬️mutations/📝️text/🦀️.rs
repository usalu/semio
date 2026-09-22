//! ⚡️ Fem2d artifact — OpText/OpBinary codecs + grammar for `Fem2dMutation`.

pub use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation, Fem2dMutation};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for Fem2dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        dsl::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for Fem2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
