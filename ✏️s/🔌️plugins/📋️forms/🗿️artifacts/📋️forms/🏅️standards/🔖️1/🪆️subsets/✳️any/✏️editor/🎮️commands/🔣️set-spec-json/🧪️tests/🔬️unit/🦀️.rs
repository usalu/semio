use super::*;
use crate::editor::forms::commands::set_active_example::SetActiveExample;
use crate::editor::forms::unit_tests::context::{dispatch, forms_app};
use crate::editor::forms::FormsCommand;
use crate::schema::onboarding_example_spec;
use SetSpecJson;

#[semio_framework_async_macros::async_test]
async fn set_active_example_switches_to_the_onboarding_fixture() {
    // 🩹️ `replace_spec_operations` deliberately never touches `id` (only title/steps — `id` is the
    // document's own stable identity, not part of the "example" content it swaps) — assert on the
    // steps/title it does replace, not on `id`.
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
    // 🩹️ `SetSpecJson`'s payload is raw `semio_framework_artifact_playbook_playbook::PlaybookSpec`-shaped JSON (see
    // `handle`'s own doc comment, ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM)
    // — `onboarding_example_spec()` itself now serializes as `FormsSnapshot`'s OWN composed
    // `structure`/`results`-handle shape, so the test input is built from the playbook spec
    // directly, not from `serde_json::to_string(&onboarding_example_spec())`.
    let mut app = forms_app().await;
    let onboarding_snapshot = onboarding_example_spec();
    let onboarding_playbook = semio_framework_artifact_playbook_playbook::PlaybookSpec {
        schema: onboarding_snapshot.schema.clone(),
        id: onboarding_snapshot.id.clone(),
        version: onboarding_snapshot.version.clone(),
        title: onboarding_snapshot.title.clone(),
        steps: forms_steps(&onboarding_snapshot),
    };
    let onboarding = dsl::os_pack::json::to_json_string(&onboarding_playbook);
    dispatch(&mut app, FormsCommand::SetSpecJson(SetSpecJson { json: onboarding })).await;
    let spec = app.snapshot().expect("projection");
    assert_eq!(forms_steps(&spec).len(), 3);
    assert_eq!(spec.title, onboarding_example_spec().title);
}

#[semio_framework_async_macros::async_test]
async fn set_spec_json_with_invalid_json_is_a_no_operation() {
    let mut app = forms_app().await;
    let before = app.snapshot().expect("projection");
    dispatch(&mut app, FormsCommand::SetSpecJson(SetSpecJson { json: "not json".into() })).await;
    assert_eq!(app.snapshot().expect("projection"), before);
}
