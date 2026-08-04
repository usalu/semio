//! 📦️ Procedural 2D app — binary document surface + laws (constitutional: pack).

use procedural_2d::Procedural2dDocument;
use store::PackError;

/// 📦️ Encodes a `Procedural2dDocument` to its binary pack form.
pub fn encode(document: &Procedural2dDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Procedural2dDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Procedural2dDocument, PackError> {
    <Procedural2dDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
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
        let projection = procedural_2d_dsl::parse_dsl(procedural_2d_dsl::PROCEDURAL2D_EXAMPLE_TEXT).expect("parse 🌀️default.procedural2d fixture");
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

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Procedural2dOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use procedural_2d_op::Procedural2dOperation;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<Procedural2dDocument, Procedural2dOperation> = DocumentStore::new(create_document_envelope(procedural_2d::PROCEDURAL_2D_SCHEMA, "procedural2d", Procedural2dDocument::default(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural2dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        let edit: &Edit<Procedural2dOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        test_support::assert_command_envelope_round_trip::<Procedural2dDocument, Procedural2dOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
