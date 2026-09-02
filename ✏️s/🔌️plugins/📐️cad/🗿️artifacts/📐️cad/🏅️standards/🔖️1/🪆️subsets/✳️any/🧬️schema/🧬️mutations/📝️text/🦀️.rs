//! 🔧️ CAD artifact — OpText/OpBinary codecs + grammar for serializing `CadMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

pub use crate::artifacts::cad::mutations::{CadMutation, CadNodePatch, CadReferencePatch};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for CadMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for CadMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::artifacts::cad::mutations::tests::every_mutation;

    /// ⚖️ The pinned pre-migration byte fixture retired with the generic `Patch*`/`Set*` variants
    /// it exercised (SEMANTIC-MUTATIONS-OVERHAUL, 26/08/12): the wire format legitimately changed —
    /// greenfield, no backward compat — so round-tripping every current variant (below) is the law
    /// that matters now, not byte-for-byte parity with a vocabulary that no longer exists.
    #[semio_framework_async_macros::async_test]
    async fn cad_mutation_print_op_round_trips_every_variant_as_one_line() {
        for op in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&op);
            store::os_store::test_support::assert_op_text_binary_equivalence(&op);
        }
    }
}
//#endregion 🧪️Tests
