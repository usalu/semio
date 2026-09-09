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

#[semio_framework_async_macros::async_test]
async fn artifact_kind_uses_the_dictionary_media_kind_as_both_id_and_schema() {
    assert_eq!(artifact_kind().id, "form.dictionary");
    assert_eq!(artifact_kind().schema, "form.dictionary");
    assert_eq!(FORMS_DOCUMENT_SCHEMA, "forms.form");
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
async fn forms_working_scene_is_owned_by_the_exact_snapshot_child() {
    let (owned, _) = forms_children_from_steps(&[FormStep { id: "step-one".into(), title: "One".into(), description: None, blocks: Vec::new() }]);
    let wire = dsl::os_pack::json::to_json_string(&owned).into_bytes();
    let reconstructed: FormsStructureChild = dsl::os_pack::json::from_json_str(std::str::from_utf8(&wire).expect("child JSON UTF-8")).expect("Forms child wire roundtrip");
    let observed = serde_json::json!({
        "ownedHasScene": owned.local_owner::<FormsWorkingScene>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasScene": reconstructed.local_owner::<FormsWorkingScene>().is_some(),
    });

    assert_eq!(observed, SerdeJsonFormsChildOwnerOracle::expected());
}
