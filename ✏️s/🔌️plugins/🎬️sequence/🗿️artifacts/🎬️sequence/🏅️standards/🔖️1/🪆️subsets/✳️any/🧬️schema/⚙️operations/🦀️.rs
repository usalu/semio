//! ⚙️ Sequence generic detection assembly, store bridges, and cross-kind laws.

use crate::schema::mutations::*;
use crate::{SequenceEdge, SequenceFixture, SequenceSnapshot, SequenceStep};
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
pub fn sequence_snapshot_mutations(before: &SequenceFixture, after: &SequenceFixture) -> Vec<SequenceMutation> {
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
pub fn apply_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> protocol::MutationApplyResult<SequenceSnapshot> {
    protocol::MutationDiff::apply(mutation.diff(snapshot).diff(), snapshot)
}

pub fn inverse_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    mutation.inverse(snapshot)
}

//#region 🔖️CaseBridges
/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "createStep", …}`) JSON projection —
/// the shape the `mutate-sequence-1` case's `Examples` rows carry — into a real
/// [`SequenceMutation`]. A thin `serde_json` wrapper (already a direct dependency of this crate, used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule, never a new
/// one), so the case reads the committed feature row instead of re-declaring it as a Rust literal.
pub fn decode_sequence_mutation_json(text: &str) -> Result<SequenceMutation, String> {
    dsl::os_pack::from_json_str(text).map_err(|error| error.to_string())
}

/// 📥️ Decodes a committed `{"steps": [...], "edges": [...]}` document into the real step/edge values
/// a composed content child is seeded with. `StepParams` wraps a `Dictionary` whose integer/decimal
/// distinction only its own `serde` round trip preserves, so a caller outside this crate cannot
/// rebuild a step by hand without losing exactly the fidelity `edit-step-params` exists to move.
pub fn decode_sequence_scene_json(text: &str) -> Result<(Vec<SequenceStep>, Vec<SequenceEdge>), String> {
    #[derive(dsl::FromValue)]
    struct CommittedScene {
        #[value(default)]
        steps: Vec<SequenceStep>,
        #[value(default)]
        edges: Vec<SequenceEdge>,
    }
    let scene: CommittedScene = dsl::os_pack::from_json_str(text).map_err(|error| error.to_string())?;
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
    let scene = crate::sequence_working_scene(snapshot);
    dsl::os_pack::to_json_string(&SequenceFixture { schema: snapshot.schema.clone(), steps: scene.steps, edges: scene.edges })
}
//#endregion 🔖️CaseBridges

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
