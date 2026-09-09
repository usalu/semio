use super::*;
use semio_framework_plugin::{EditorApp, HistoryView};

const RETAINED_LIMITS: &str = include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json");

#[test]
fn create_playground_editor_builds_a_definition_for_the_editor_role() {
    let def = create_playground_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PLAYGROUND_DIALECT.into());
}

#[test]
fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<PlaygroundEditor as ArtifactEditor>::DIALECT, PLAYGROUND_DIALECT);
}

#[test]
fn change_schema_factory_declares_the_exact_bounded_contract() {
    let factory = PlaygroundCommandJobFactory::new("s.demonstrator.playground@1/*#editor");
    assert_eq!(factory.keys(), &[ToolFactoryKey::new("s.demonstrator.playground@1/*#editor", "changeSchema")]);
    assert_eq!(factory.payload_schema_id(), PLAYGROUND_RETAINED_PAYLOAD_SCHEMA);
    assert_eq!(factory.execution_contract(), ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
}

#[test]
fn change_schema_admission_matches_the_language_neutral_limit_oracle() {
    let fixture: Value = parse(RETAINED_LIMITS).expect("retained command limits decode");
    let maximum = fixture.get("maximumSchemaBytes").and_then(Value::as_u64).expect("maximumSchemaBytes") as usize;
    let additional = fixture.get("rejectedAdditionalBytes").and_then(Value::as_u64).expect("rejectedAdditionalBytes") as usize;
    let expected_items = fixture.get("expectedWorkItems").and_then(Value::as_u64).expect("expectedWorkItems") as usize;
    assert_eq!(maximum, PLAYGROUND_RETAINED_RAW_BYTES);
    assert_eq!(expected_items, PLAYGROUND_RETAINED_WORK_ITEMS);
    let accepted = PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: "s".repeat(maximum) });
    let rejected = PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: "s".repeat(maximum + additional) });
    let snapshot = empty_playground_snapshot();
    let interaction = protocol::InteractionState::default();
    assert_eq!(playground_retained_extent(&accepted, &snapshot, &interaction), Some(expected_items));
    assert_eq!(playground_retained_extent(&rejected, &snapshot, &interaction), None);
    assert!(PlaygroundEditor::command_from_action("changeSchema", Some(&dsl::DslValue::object([("newSchema".to_string(), dsl::DslValue::String("s".repeat(maximum)))]))).is_ok());
    assert!(PlaygroundEditor::command_from_action("changeSchema", Some(&dsl::DslValue::object([("newSchema".to_string(), dsl::DslValue::String("s".repeat(maximum + additional)))]))).is_err());
}

#[semio_framework_async_macros::async_test]
async fn change_schema_command_mutates_the_schema_field() {
    let document = empty_playground_snapshot();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let command = PlaygroundCommand::ChangeSchema(change_schema::ChangeSchema { new_schema: "playground.custom".into() });
    let emit = command.dispatch(&doc, &cfg).expect("dispatch");
    assert_eq!(emit.artifact_mutations, vec![PlaygroundMutation::ChangeSchema(crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema { new_schema: "playground.custom".into() })]);
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_editor_installs_its_exact_bounded_command_proof() {
    let _app = semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<PlaygroundEditor>>(testkit::playground_editor_manifest_for_testkit).await;
}

#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_the_declared_action_and_rejects_unknown_ones() {
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<PlaygroundEditor>>(testkit::playground_editor_manifest_for_testkit).await;
    assert!(PlaygroundEditor::command_from_action("noSuchAction", None).is_err());
}
