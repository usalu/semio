use super::set_active_example;

/// 🌱️ The genesis precondition of the composed-child load path, as a law rather than an assumption:
/// `crate::genesis_sequence_child_pack` answers a `content` slot ONLY for the child id
/// `default_snapshot()` mints, and the react shell sends the whole-document archive member-less, so
/// a committed example whose content child id drifts from that one loads as `InvalidReference`.
#[semio_framework_async_macros::async_test]
async fn committed_example_carries_the_genesis_content_child() {
    let document = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("committed example parses");
    let genesis = neural_engine::ColdOwner::new(crate::snapshot::schema::default_snapshot());
    assert_eq!(document.content.child_id, genesis.content.child_id);
    assert_eq!(document.content.child_id, document.content.target.artifact_id);
    assert!(crate::genesis_sequence_child_pack(&document, "content", &document.content.child_id).is_some());
}

/// 🎬️ The boot verb emits exactly one host-applied whole-document load and no mutation.
#[semio_framework_async_macros::async_test]
async fn set_active_example_emits_one_load_document_effect() {
    let emit = set_active_example::emit(crate::examples::demo::ID).expect("demo example emits");
    assert!(emit.artifact_mutations.is_empty());
    assert_eq!(emit.effects.iter().filter(|effect| matches!(effect, semio_framework_plugin::Effect::LoadDocument { .. })).count(), 1);
}

/// 📚️ The playground navbar dispatches `setActiveExample` on boot; undeclared, the shell drops it
/// before dispatch. It must be on the manifest, carry its own retained route, and bridge from the
/// shell's `{action, args}` vocabulary.
#[semio_framework_async_macros::async_test]
async fn set_active_example_is_declared_and_bridged() {
    use semio_framework_plugin::ArtifactEditor;
    let definition = crate::editor::sequence::create_sequence_app();
    assert!(
        definition.actions.iter().any(|action| action.id == "setActiveExample") || definition.window_kinds.iter().any(|window| window.actions.iter().any(|action| action.id == "setActiveExample")),
        "setActiveExample must be reachable from the manifest"
    );
    let command = crate::editor::sequence::SequencePlayApp::command_from_action("setActiveExample", Some(&dsl::json::to_dsl_value(&dsl::json!({ "exampleId": "demo" })))).expect("setActiveExample bridges");
    assert_eq!(crate::editor::sequence::SequencePlayApp::command_id(&command), "setActiveExample");
}

/// 🧭️ An id belonging to another app is an empty emit, never a fault — the playground navbar
/// dispatches whatever its combobox holds.
#[semio_framework_async_macros::async_test]
async fn a_foreign_example_id_is_an_empty_emit() {
    let emit = set_active_example::emit("some-other-apps-example").expect("foreign id is not a fault");
    assert!(emit.effects.is_empty());
    assert!(emit.artifact_mutations.is_empty());
}
