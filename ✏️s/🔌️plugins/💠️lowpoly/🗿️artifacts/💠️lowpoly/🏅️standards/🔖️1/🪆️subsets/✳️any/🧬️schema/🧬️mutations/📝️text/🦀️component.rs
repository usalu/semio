//! ⚡️ Lowpoly artifact — OpText/OpBinary codecs + grammar for serializing `LowpolyMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.

pub use crate::artifacts::lowpoly::schema::mutations::{
    apply_lowpoly_mutation, inverse_lowpoly_mutation, LowpolyMutation, LowpolyPaintLayerPatch, PixelRun,
};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for LowpolyMutation {
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

impl protocol::OpBinary for LowpolyMutation {
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
    use crate::artifacts::lowpoly::engine::default_snapshot;
    use crate::artifacts::lowpoly::{LowpolyObject, LowpolyObjectPatch, LowpolyPaintLayer};

    fn tiny_mesh_json() -> String {
        semio_s_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("mesh json")
    }

    fn tiny_object(id: &str, name: &str) -> LowpolyObject {
        LowpolyObject { id: id.into(), name: name.into(), transform: Default::default(), smooth_shading: false, mesh_json: tiny_mesh_json(), paint_layers: vec![LowpolyPaintLayer::new("Base")] }
    }

    #[test]
    fn op_text_round_trip_objects_add() {
        let mutation = LowpolyMutation::ObjectsAdd { index: 1, item: tiny_object("obj-100", "Box") };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_objects_remove() {
        let mutation = LowpolyMutation::ObjectsRemove { id: "obj-1".into() };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_objects_move() {
        let mutation = LowpolyMutation::ObjectsMove { id: "obj-1".into(), to_index: 2 };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_objects_patch_without_mesh() {
        let mutation = LowpolyMutation::ObjectsPatch {
            id: "obj-1".into(),
            patch: LowpolyObjectPatch { name: Some("Renamed".into()), smooth_shading: Some(true), transform: Some(crate::artifacts::lowpoly::LowpolyTransform { position: [1.0, 2.0, 3.0], ..Default::default() }), mesh_json: None },
        };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_objects_patch_with_mesh() {
        let mutation = LowpolyMutation::ObjectsPatch { id: "obj-1".into(), patch: LowpolyObjectPatch { mesh_json: Some(tiny_mesh_json()), ..Default::default() } };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_add_paint_layer() {
        let mutation = LowpolyMutation::AddPaintLayer { object_id: "obj-1".into(), index: 1, layer: LowpolyPaintLayer::new("Detail") };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_remove_paint_layer() {
        let mutation = LowpolyMutation::RemovePaintLayer { object_id: "obj-1".into(), index: 0 };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_patch_paint_layer() {
        let mutation = LowpolyMutation::PatchPaintLayer { object_id: "obj-1".into(), index: 0, patch: LowpolyPaintLayerPatch { name: Some("Top".into()), visible: Some(false), opacity: Some(0.5), blend_mode: Some("multiply".into()) } };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_paint_stroke() {
        let mutation = LowpolyMutation::PaintStroke { object_id: "obj-1".into(), layer_index: 0, runs: vec![PixelRun { offset: 12, bytes: vec![255, 0, 0, 255] }, PixelRun { offset: 400, bytes: vec![0, 255, 0, 255, 0, 0, 0, 128] }] };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_round_trip_set_snapshot() {
        let mutation = LowpolyMutation::SetSnapshot { snapshot: default_snapshot() };
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&mutation);
    }

    #[test]
    fn op_text_parse_rejects_unknown_mutation_kind() {
        let result = <LowpolyMutation as protocol::OpText>::parse_op("bogusMutation foo=bar");
        assert!(result.is_err());
    }

    #[test]
    fn op_text_parse_rejects_unknown_objects_submutation() {
        let result = <LowpolyMutation as protocol::OpText>::parse_op("objects.frobnicate id=obj-1");
        assert!(result.is_err());
    }
}
//#endregion 🧪️Tests
