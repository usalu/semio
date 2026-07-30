//! 📦 Procedural 3D app — binary document surface + laws (constitutional: pack).

use procedural_3d::Procedural3dDocument;
use store::PackError;

/// 📦 Encodes a `Procedural3dDocument` to its binary pack form.
pub fn encode(document: &Procedural3dDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes a `Procedural3dDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Procedural3dDocument, PackError> {
    <Procedural3dDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use store::{test_support, DocumentDsl};

    #[test]
    fn dsl_pack_equivalence_empty_projection() {
        test_support::assert_dsl_pack_equivalence(&Procedural3dDocument::default());
    }

    #[test]
    fn dsl_pack_equivalence_hexagonal_mushroom_column_fixture() {
        let projection = Procedural3dDocument::parse_dsl(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_HEX_COLUMN_TEXT).expect("parse hexagonal-mushroom-column.procedural3d fixture");
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_rectangle_extrude_volume_fixture() {
        let projection = Procedural3dDocument::parse_dsl(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_RECT_EXTRUDE_TEXT).expect("parse rectangle-extrude-volume.procedural3d fixture");
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_sphere_cut_with_torus_fixture() {
        let projection = Procedural3dDocument::parse_dsl(procedural_3d_dsl::PROCEDURAL3D_EXAMPLE_SPHERE_TORUS_TEXT).expect("parse sphere-cut-with-torus.procedural3d fixture");
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_with_generation_state() {
        let mut projection = Procedural3dDocument::default();
        let mut values = serde_json::Map::new();
        values.insert("count".into(), serde_json::json!(3.0));
        projection.generation.generations.push(playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_covers_remaining_widget_kinds() {
        let mut projection = Procedural3dDocument::default();
        projection.fixture.widgets = vec![
            Widget::InputNote { id: "note-1".into(), text: "hello \"world\"".into() },
            Widget::InputImage { id: "image-1".into(), src: "https://example.test/a.png".into() },
            Widget::Variable { id: "variable-1".into(), name: "height".into(), schema: "number".into() },
            Widget::OutputAction { id: "action-1".into(), action: "export".into() },
            Widget::OutputExport { id: "export-1".into(), format: "gltf".into() },
            Widget::Cluster { id: "cluster-1".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() },
        ];
        test_support::assert_dsl_pack_equivalence(&projection);
    }
}
//#endregion 🧪Tests
