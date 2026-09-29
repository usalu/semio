use super::*;

trait FormsChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonFormsChildOwnerOracle;

impl FormsChildOwnerOracle for SerdeJsonFormsChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral Forms child-owner fixture")
    }
}

/// 🪪️ `artifact_kind().schema` IS `FORMS_DOCUMENT_SCHEMA`: a document kind has ONE schema identity — the hub's codec rows, document-open targets and genesis, the MCP workspace
/// store and host-media contributions all key on it (ticket 26/09/23 W4: a distinct "media schema" left the package without a
/// codec owner, so the trusted catalog refused it). The former media string stays declared as `source_format`.
#[semio_framework_async_macros::async_test]
async fn artifact_kind_names_the_store_schema() {
    assert_eq!(artifact_kind().schema, FORMS_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().source_format, "form.dictionary");
}

#[semio_framework_async_macros::async_test]
async fn question_fields_roundtrip() {
    let json = r#"{
            "id":"q1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
    let question: FormQuestion = serde_json::from_str(json).expect("question json");
    assert_eq!(question.min, Some(1.0));
    assert_eq!(question.unit.as_deref(), Some("people"));
    assert!(question.required.unwrap_or(false));
}

#[semio_framework_async_macros::async_test]
async fn forms_definition_is_durable_and_owned_by_each_snapshot() {
    let original = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "test".into(), "1".into(), None, &[FormStep { id: "step-one".into(), title: "One".into(), description: None, blocks: Vec::new() }]);
    let mut cloned = original.clone();
    cloned.definition.steps[0].title = "Changed".into();
    let wire = dsl::os_pack::json::to_json_string(&original);
    let decoded: FormsSnapshot = dsl::os_pack::json::from_json_str(&wire).unwrap();
    let observed = serde_json::json!({
        "ownerIsolation": original.definition.steps[0].title == "One",
        "wireKeepsDefinition": decoded.definition == original.definition,
        "childReferencesMatch": decoded.structure == original.structure && decoded.results == original.results,
    });
    assert_eq!(observed, SerdeJsonFormsChildOwnerOracle::expected());
}

/// 🧬️ The projection names exactly the snapshot's declared child slots — what the live envelope load checks
/// before a decoded document may replace the store.
#[test]
fn the_child_restore_projection_names_every_declared_child_slot() {
    let snapshot = crate::FormsSnapshot::default();
    let projection = crate::forms_child_restore_projection(&snapshot).expect("the loaded-parent child projection");
    assert_eq!(projection.len(), <crate::FormsSnapshot as store::os_schema_composition::ArtifactCompositionFields>::child_slots().len());
}
