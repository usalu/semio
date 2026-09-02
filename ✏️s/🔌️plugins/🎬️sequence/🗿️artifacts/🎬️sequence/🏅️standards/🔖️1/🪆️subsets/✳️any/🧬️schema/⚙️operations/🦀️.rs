//! ⚙️ Sequence generic detection assembly, store bridges, and cross-kind laws.

use crate::artifacts::sequence::schema::mutations::*;
use crate::artifacts::sequence::{SequenceEdge, SequenceFixture, SequenceSnapshot, SequenceStep};
use protocol::Mutation;
use std::collections::BTreeMap;

//#region 🔖️Store
pub type SequenceEnvelope = store::ArtifactEnvelope<SequenceSnapshot, SequenceMutation>;
pub type SequenceStore = store::ArtifactStore<SequenceSnapshot, SequenceMutation>;
//#endregion 🔖️Store

//#region 🔎️DetectionAssembly
/// 🗂️ Indexed before/after scenes shared by independent leaf detection contributions.
pub struct SequenceDetectionContext<'a> {
    pub before: &'a SequenceFixture,
    pub after: &'a SequenceFixture,
    pub before_steps: BTreeMap<&'a str, &'a SequenceStep>,
    pub after_steps: BTreeMap<&'a str, &'a SequenceStep>,
    pub before_edges: BTreeMap<&'a str, &'a SequenceEdge>,
    pub after_edges: BTreeMap<&'a str, &'a SequenceEdge>,
}

/// 🔢️ A leaf-owned mutation with its stable cross-contribution application order.
pub struct SequenceDetectedMutation {
    pub order: (u8, usize, u8),
    pub mutation: SequenceMutation,
}

pub type SequenceMutationDetector = for<'a> fn(&SequenceDetectionContext<'a>) -> Vec<SequenceDetectedMutation>;

/// 🔀️ Assembles ordered leaf-owned detection contributions without concrete mutation branches.
pub async fn sequence_snapshot_mutations(before: &SequenceFixture, after: &SequenceFixture) -> Vec<SequenceMutation> {
    let context = SequenceDetectionContext {
        before,
        after,
        before_steps: before.steps.iter().rev().map(|step| (step.id.as_str(), step)).collect(),
        after_steps: after.steps.iter().map(|step| (step.id.as_str(), step)).collect(),
        before_edges: before.edges.iter().rev().map(|edge| (edge.id.as_str(), edge)).collect(),
        after_edges: after.edges.iter().map(|edge| (edge.id.as_str(), edge)).collect(),
    };
    let mut detected: Vec<_> = DETECTORS.iter().flat_map(|detect| detect(&context)).collect();
    detected.sort_by_key(|entry| entry.order);
    detected.into_iter().map(|entry| entry.mutation).collect()
}
//#endregion 🔎️DetectionAssembly

/// 🏷️ Catalog vocabulary in aggregate declaration order.
pub const KINDS: &[&str] = &[
    <CreateStep as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
    <DeleteStep as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
    <MoveStep as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
    <EditStepParams as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
    <ChangeStepCollapsed as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
    <ConnectSteps as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
    <DisconnectSteps as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
    <DuplicateStep as protocol::MutationKind<SequenceSnapshot, SequenceMutation>>::SEMANTICS.kind,
];

/// ▶️ Applies `mutation` via its diff.
pub async fn apply_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> protocol::MutationApplyResult<SequenceSnapshot> {
    protocol::MutationDiff::apply(mutation.diff(snapshot).diff(), snapshot)
}

pub async fn inverse_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    mutation.inverse(snapshot)
}

//#region 🔖️CaseBridges
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "createStep", …}`) JSON projection —
/// the shape the `mutate-sequence-1` case's `Examples` rows carry — into a real
/// [`SequenceMutation`]. A thin `serde_json` wrapper (already a direct dependency of this crate, used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule, never a new
/// one), so the case reads the committed feature row instead of re-declaring it as a Rust literal.
pub fn decode_sequence_mutation_json(text: &str) -> Result<SequenceMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `{"steps": [...], "edges": [...]}` document into the real step/edge values
/// a composed content child is seeded with. `StepParams` wraps a `Dictionary` whose integer/decimal
/// distinction only its own `serde` round trip preserves, so a caller outside this crate cannot
/// rebuild a step by hand without losing exactly the fidelity `edit-step-params` exists to move.
pub fn decode_sequence_scene_json(text: &str) -> Result<(Vec<crate::artifacts::sequence::SequenceStep>, Vec<crate::artifacts::sequence::SequenceEdge>), String> {
    #[derive(serde::Deserialize)]
    struct CommittedScene {
        #[serde(default)]
        steps: Vec<crate::artifacts::sequence::SequenceStep>,
        #[serde(default)]
        edges: Vec<crate::artifacts::sequence::SequenceEdge>,
    }
    let scene: CommittedScene = serde_json::from_str(text).map_err(|error| error.to_string())?;
    Ok((scene.steps, scene.edges))
}

/// ⚖️ The SEMANTIC PROJECTION this subset is compared through — `(schema, steps, edges)` read back
/// off the composed content child's working scene. It belongs to the subset rather than to a test
/// adapter, because what counts as this document's meaning is this subset's ruling, not a case's.
/// The content handle is deliberately absent: `sequence_content_child_handle` content-addresses
/// exactly this step/edge pair through `std`'s deliberately unspecified `DefaultHasher`, so
/// projecting it would compare the same content twice and pin a value the standard library does not
/// promise.
pub fn encode_sequence_projection_json(snapshot: &SequenceSnapshot) -> String {
    let scene = crate::artifacts::sequence::sequence_working_scene(snapshot);
    serde_json::json!({ "schema": snapshot.schema, "steps": scene.steps, "edges": scene.edges }).to_string()
}
//#endregion 🔖️CaseBridges

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_snapshot, SequenceStep, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
    use protocol::testkit::assert_mutation_inverse_law;
    use protocol::SemanticMutation;
    use store::{create_document_envelope, ArtifactCommand};

    #[semio_framework_async_macros::async_test]
    async fn leaf_detection_preserves_language_neutral_plan_vectors() {
        let suite: serde_json::Value = serde_json::from_str(include_str!("🧪️tests/🔣️.json")).expect("detection fixture JSON");
        for case in suite["cases"].as_array().expect("detection cases") {
            let before: SequenceFixture = serde_json::from_value(case["before"].clone()).expect("before fixture");
            let after: SequenceFixture = serde_json::from_value(case["after"].clone()).expect("after fixture");
            let expected: Vec<SequenceMutation> = serde_json::from_value(case["expected"].clone()).expect("expected mutations");
            assert_eq!(sequence_snapshot_mutations(&before, &after), expected, "{}", case["id"]);
        }
    }

    async fn round_trip(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> SequenceSnapshot {
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
        let mut store = SequenceStore::new(create_document_envelope(SEQUENCE_DOCUMENT_SCHEMA, "sequence", default_snapshot(), None)).expect("valid artifact store fixture");
        store
            .dispatch(ArtifactCommand::Apply { mutations: vec![create_step(SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false })], description: None })
            .expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").to_fixture().steps.len(), 3);
    }

    //#region 🔖️MutationLaws

    #[semio_framework_async_macros::async_test]
    async fn connect_disconnect_steps_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &connect_steps("edge-99".into(), "step-1".into(), "step-2".into()));
        assert_mutation_inverse_law(&base, &disconnect_steps("edge-1".into()));
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
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog sequence-1-any");
        }
    }
    //#endregion 🔖️KindsCatalog
}
//#endregion 🧪️Tests
