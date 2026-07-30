//! 📦 Procedural 2D app — binary document surface + laws (constitutional: pack).

use procedural_2d::Procedural2dDocument;
use store::PackError;

/// 📦 Encodes a `Procedural2dDocument` to its binary pack form.
pub fn encode(document: &Procedural2dDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖 Decodes a `Procedural2dDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Procedural2dDocument, PackError> {
    <Procedural2dDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use store::test_support;

    #[test]
    fn dsl_pack_equivalence_empty_projection() {
        test_support::assert_dsl_pack_equivalence(&Procedural2dDocument::default());
    }

    #[test]
    fn dsl_pack_equivalence_example_fixture() {
        let projection = procedural_2d_dsl::parse_dsl(procedural_2d_dsl::PROCEDURAL2D_EXAMPLE_TEXT).expect("parse 🌀default.procedural2d fixture");
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_with_generation_state() {
        let mut projection = Procedural2dDocument::default();
        let mut values = serde_json::Map::new();
        values.insert("count".into(), serde_json::json!(3.0));
        projection.generation.generations.push(playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values });
        projection.generation.selected_generation_id = Some("generation-1".into());
        projection.generation.preview_text = Some("42".into());
        test_support::assert_dsl_pack_equivalence(&projection);
    }

    #[test]
    fn dsl_pack_equivalence_covers_every_widget_kind() {
        let mut projection = Procedural2dDocument::default();
        projection.fixture.widgets = vec![
            Widget::InputSlider { id: "slider".into(), value: 2.0, min: 0.0, max: 10.0, step: 0.5 },
            Widget::InputImage { id: "image".into(), src: "data:image/png;base64,abc".into() },
            Widget::Variable { id: "variable".into(), name: "value".into(), schema: "dictionary".into() },
            Widget::OutputAction { id: "action".into(), action: "export".into() },
            Widget::OutputExport { id: "export".into(), format: "svg".into() },
            Widget::Cluster { id: "cluster".into(), name: "Group".into(), tree: Default::default(), flow: Default::default() },
        ];
        projection.fixture.synapses = vec![];
        test_support::assert_dsl_pack_equivalence(&projection);
    }
}
//#endregion 🧪Tests
