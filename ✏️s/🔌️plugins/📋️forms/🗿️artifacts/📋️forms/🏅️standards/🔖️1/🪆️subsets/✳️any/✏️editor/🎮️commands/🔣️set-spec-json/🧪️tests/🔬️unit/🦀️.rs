use super::*;
use crate::forms_steps;
use crate::editor::forms::commands::set_active_example::SetActiveExample;
use crate::editor::forms::unit_tests::context::{dispatch, forms_app};
use crate::editor::forms::FormsCommand;
use crate::standards::v1::subsets::any::io::text::snapshot::onboarding_example_spec;
use SetSpecJson;

#[semio_framework_async_macros::async_test]
async fn set_active_example_switches_to_the_onboarding_fixture() {
    let mut app = forms_app().await;
    dispatch(&mut app, FormsCommand::SetActiveExample(SetActiveExample { example_id: "onboarding".into() })).await;
    let spec = app.snapshot().expect("projection");
    assert_eq!(forms_steps(&spec).len(), 3);
    assert_eq!(spec.title, onboarding_example_spec().title);
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_with_blank_id_clears_the_document() {
    let mut app = forms_app().await;
    dispatch(&mut app, FormsCommand::SetActiveExample(SetActiveExample { example_id: "".into() })).await;
    let spec = app.snapshot().expect("projection");
    assert!(crate::schema::flatten_questions(&spec).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn set_spec_json_replaces_the_document() {
    let mut app = forms_app().await;
    let onboarding_snapshot = onboarding_example_spec();
    let onboarding = semio_framework_pack_json::to_json_string(&onboarding_snapshot);
    dispatch(&mut app, FormsCommand::SetSpecJson(SetSpecJson { json: onboarding })).await;
    let spec = app.snapshot().expect("projection");
    assert_eq!(forms_steps(&spec).len(), 3);
    assert_eq!(spec.title, onboarding_example_spec().title);
}

#[semio_framework_async_macros::async_test]
async fn set_spec_json_with_invalid_json_reports_a_fault() {
    let mut app = forms_app().await;
    let before = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let config = FormsConfig::default();
    let outcome = handle(&SetSpecJson { json: "not json".into() }, &ArtifactView::new(&before, &history), &ConfigView { snapshot: &config, window: None });
    assert!(outcome.is_err());
    let unknown_template = crate::editor::forms::commands::set_active_example::handle(&SetActiveExample { example_id: "missing".into() }, &ArtifactView::new(&before, &history), &ConfigView { snapshot: &config, window: None });
    assert!(unknown_template.is_err());
    assert_eq!(app.snapshot().expect("projection"), before);
}

#[test]
fn canonical_import_vectors_preserve_document_identity_and_answers() {
    use protocol::Mutation;
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧬️schema/🧫️fixtures/📥️import/🔣️.json")).unwrap();
    let current: FormsSnapshot = semio_framework_pack_json::from_json_str(include_str!("../../../../../🧬️schema/🧫️fixtures/💾️persistence/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let history = semio_framework_plugin::HistoryView::empty();
    let config = FormsConfig::default();
    for case in vectors["cases"].as_array().unwrap() {
        let outcome = handle(&SetSpecJson { json: case["source"].as_str().unwrap().into() }, &ArtifactView::new(&current, &history), &ConfigView { snapshot: &config, window: None });
        assert_eq!(outcome.is_ok(), case["valid"].as_bool().unwrap(), "{}", case["name"]);
        if let Ok(emit) = outcome {
            let mut result = current.clone();
            for mutation in emit.artifact_mutations { mutation.diff(&result).apply_to(&mut result); }
            assert_eq!(result.id, current.id);
            assert_eq!(result.responses, current.responses);
            let expected: FormsSnapshot = semio_framework_pack_json::from_json_str(case["source"].as_str().unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            assert_eq!(result.definition, expected.definition);
            assert_eq!(result.title, expected.title);
        }
    }
}
