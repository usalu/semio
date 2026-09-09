use super::*;

fn sample() -> DuplicateWidget {
    DuplicateWidget { source_id: "note-1".into(), new_id: "note-2".into(), synapse_id: "note-1-to-note-2".into(), from_port: "out".into(), to_port: "in".into() }
}

#[semio_framework_async_macros::async_test]
async fn label_and_target_are_sensible() {
    let payload = sample();
    assert_eq!(CompositeMutationKind::label(&payload), "Duplicate widget \"note-1\"");
    assert_eq!(CompositeMutationKind::target(&payload), vec!["note-1".to_string(), "note-2".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn semantics_kind_and_verb_match_the_directory() {
    let semantics = <DuplicateWidget as CompositeMutationKind<FlowSnapshot, FlowMutation>>::SEMANTICS;
    assert_eq!(semantics.kind, "duplicate-widget");
    assert_eq!(semantics.verb, "duplicate");
}
