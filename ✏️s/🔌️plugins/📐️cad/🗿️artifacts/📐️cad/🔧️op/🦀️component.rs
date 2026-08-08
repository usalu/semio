//! 🔧️ CAD artifact — OpText/OpBinary codecs + grammar for serializing `CadMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

pub use crate::artifacts::cad::mutations::{
    CadMutation, CadNodePatch, CadObjectPatch, CadReferencePatch,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for CadMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
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
    use super::*;
    use crate::artifacts::cad::mutations::tests::every_mutation;
    use crate::artifacts::cad::mutations::{CadMutation, CadObjectPatch, CadReferencePatch};
    use crate::artifacts::cad::CadPaneId;

    #[test]
    fn cad_mutation_print_op_round_trips_every_variant_as_one_line() {
        for op in every_mutation() {
            store::os_store::test_support::assert_op_line_round_trip(&op);
            store::os_store::test_support::assert_op_text_binary_equivalence(&op);
        }
    }

    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |op: &CadMutation| -> String { protocol::OpBinary::encode_op(op).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect() };
        assert_eq!(hex(&CadMutation::RemoveObject { pane: CadPaneId::Shape, object_id: "object-1".into() }), "010101086f626a6563742d3102000a00010600");
        assert_eq!(
            hex(&CadMutation::PatchObject { pane: CadPaneId::Building, object_id: "object-1".into(), patch: CadObjectPatch { label: Some("Renamed".into()), visible: Some(false), ..Default::default() } }),
            "0102020752656e616d6564086f626a6563742d3103000a01010601020e0d020006000201"
        );
        assert_eq!(hex(&CadMutation::PatchObject { pane: CadPaneId::Building, object_id: "object-1".into(), patch: CadObjectPatch::default() }), "010201086f626a6563742d3103000a01010600020e0d00");
        assert_eq!(
            hex(&CadMutation::PatchReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), patch: CadReferencePatch { hidden: Some(true), ..Default::default() } }),
            "010a02057265662d310d7370617469616c2e736861706503000601010600020e0d010602"
        );
        assert_eq!(hex(&CadMutation::PatchReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), patch: CadReferencePatch::default() }), "010a02057265662d310d7370617469616c2e736861706503000601010600020e0d00");
        assert_eq!(hex(&CadMutation::SetActiveModelDefinition { model_definition_id: "aec.building".into() }), "010c010c6165632e6275696c64696e6701000600");
    }
}
//#endregion 🧪️Tests
