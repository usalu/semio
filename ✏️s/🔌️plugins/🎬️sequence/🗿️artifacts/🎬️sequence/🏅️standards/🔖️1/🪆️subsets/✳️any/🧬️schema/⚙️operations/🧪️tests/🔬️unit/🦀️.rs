
use super::*;
use crate::{SEQUENCE_DOCUMENT_SCHEMA, SequenceStep, StepParams, default_snapshot};
use protocol::SemanticMutation;
use protocol::os_spr::testkit::assert_mutation_inverse_law;
use store::{ArtifactCommand, create_document_envelope};

#[semio_framework_async_macros::async_test]
async fn leaf_detection_preserves_language_neutral_plan_vectors() {
    let suite: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).expect("detection fixture JSON");
    for case in suite["cases"].as_array().expect("detection cases") {
        let before: SequenceFixture = dsl::os_pack::from_json_str(&case["before"].to_string()).expect("before fixture");
        let after: SequenceFixture = dsl::os_pack::from_json_str(&case["after"].to_string()).expect("after fixture");
        let expected: Vec<SequenceMutation> = dsl::os_pack::from_json_str(&case["expected"].to_string()).expect("expected mutations");
        assert_eq!(sequence_snapshot_mutations(&before, &after), expected, "{}", case["id"]);
    }
}

fn round_trip(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> SequenceSnapshot {
    let (forward, _messages) = vcs::apply_mutation(snapshot, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    let mut backward = mutation.inverse(snapshot);
    backward.reverse();
    for back in backward {
        let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, snapshot, "inverse must restore the pre-mutation snapshot");
    forward
}

#[semio_framework_async_macros::async_test]
async fn create_edit_delete_step_round_trip() {
    let snapshot = default_snapshot();
    let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
    let added = round_trip(&snapshot, &create_step(step));
    assert_eq!(added.to_fixture().steps.len(), 3);
    let moved = round_trip(&added, &move_step("step-99".into(), 120.0, 6.0));
    assert_eq!(moved.to_fixture().steps.iter().find(|step| step.id == "step-99").unwrap().x, 120.0);
    let removed = round_trip(&moved, &delete_step("step-99".into()));
    assert!(!removed.to_fixture().steps.iter().any(|step| step.id == "step-99"));
}

#[semio_framework_async_macros::async_test]
async fn delete_step_severs_and_reconnects_edges() {
    let snapshot = default_snapshot();
    assert!(snapshot.to_fixture().edges.iter().any(|edge| edge.from == "step-1" && edge.to == "step-2"));
    round_trip(&snapshot, &delete_step("step-1".into()));
}

#[semio_framework_async_macros::async_test]
async fn snapshot_mutations_capture_move_and_connect() {
    // 🧭️ Built by hand rather than via `SequenceHost` (that editing host now lives in
    // `the sibling editor module` — an artifact must never depend on an app): a step add is enough
    // to exercise `sequence_snapshot_mutations`'s before/after diff directly.
    let before = default_snapshot().to_fixture();
    let id = "step-99".to_string();
    let mut after = before.clone();
    after.steps.push(SequenceStep { id: id.clone(), kind: "math.add".into(), params: StepParams::new(), x: 40.0, y: 40.0, slot: None, collapsed: false });
    let mutations = sequence_snapshot_mutations(&before, &after);
    assert!(mutations.iter().any(|mutation| matches!(mutation, SequenceMutation::CreateStep(payload) if payload.step.id == id)));
}

#[semio_framework_async_macros::async_test]
async fn store_applies_and_undoes_step_create() {
    let mut store = SequenceStore::new(create_document_envelope(SEQUENCE_DOCUMENT_SCHEMA, "sequence", default_snapshot(), None)).await.expect("valid artifact store fixture");
    store
        .dispatch(ArtifactCommand::Apply { mutations: vec![create_step(SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false })], description: None })
        .await
        .expect("apply");
    assert_eq!(store.snapshot().expect("snapshot").to_fixture().steps.len(), 3);
}

//#region 🔖️MutationLaws

#[semio_framework_async_macros::async_test]
async fn connect_disconnect_steps_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &connect_steps("edge-99".into(), "step-1".into(), "step-2".into())).await;
    assert_mutation_inverse_law(&base, &disconnect_steps("edge-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn dispatch_registers_semantic_descriptors() {
    register_sequence_mutation_descriptors(::semio_framework_os_kernel::StateClass::Artifact).expect("mutation descriptor registration");
    for kind in SequenceMutation::kinds() {
        assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
    }
    assert_eq!(SequenceMutation::kinds().len(), 8);
}
//#endregion 🔖️MutationLaws

//#region 🔖️KindsCatalog
/// 🏷️ [`KINDS`] is the bridge between this enum and the language-neutral test platform, which
/// never parses Rust. This proves it names every variant, in declaration order, with the same
/// kebab spelling `#[derive(dsl::Mutations)]` derives — and that this subset's own committed
/// catalog declares exactly the same set, so the completeness gate cannot be measuring a
/// vocabulary that has drifted away from the code.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let declared: Vec<&str> = <SequenceMutation as SemanticMutation<SequenceSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    assert_eq!(KINDS, declared.as_slice(), "KINDS must name every SequenceMutation variant, in declaration order, spelled as its own MutationKind::SEMANTICS.kind");
    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog sequence-1-any");
    }
}
//#endregion 🔖️KindsCatalog
