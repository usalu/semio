//! 🧬️ sequence artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<SequenceSnapshot>` and `impl protocol::SemanticMutation<SequenceSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.
//!
//! `📦️glue.rs` (outside this facet's boundary, must not be touched by this migration) still
//! hand-lists the OLD generic triad-dir names under `pub mod mutations { ... }`; this rewrite
//! deleted those 8 dirs and created 8 real per-mutation ones with dir names matching their kind
//! slug exactly, so `glue.rs`'s `#[path]` entries now point at nonexistent files. See this ticket's
//! wave2 report for the exact replacement block and the `blocked-mechanism` status.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{SequenceFixture, SequenceSnapshot};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type SequenceEnvelope = store::ArtifactEnvelope<SequenceSnapshot, SequenceMutation>;
pub type SequenceStore = store::ArtifactStore<SequenceSnapshot, SequenceMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic sequence document mutation vocabulary: id-keyed step create/delete/move/edit-params/
/// change-collapsed, plus relationship connect/disconnect between steps, plus duplicate-step. The
/// canvas camera is session-only runtime state now (never a document field — see
/// `crate::editor::sequence::config`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SequenceSnapshot, diff = SequenceDiff, schema = "sequence.sequence")]
pub enum SequenceMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    MoveStep(MoveStep),
    EditStepParams(EditStepParams),
    ChangeStepCollapsed(ChangeStepCollapsed),
    ConnectSteps(ConnectSteps),
    DisconnectSteps(DisconnectSteps),
    DuplicateStep(DuplicateStep),
}

/// 🏷️ The kebab spelling of every [`SequenceMutation`] variant, in DECLARATION ORDER — the one list
/// the language-neutral test platform is measured against. It is duplicated in exactly two other
/// places on purpose: this subset's own oracle manifest catalog `sequence-1-any`
/// (`../../🧪️oracle/🔣️component.json`), which the completeness gate counts, and the
/// `mutate-sequence-1` case adapter, which must not link this crate in the oracle role.
/// [`tests::kinds_match_the_enum_and_the_catalog`] is what keeps all three honest.
pub const KINDS: &[&str] = &["create-step", "delete-step", "move-step", "edit-step-params", "change-step-collapsed", "connect-steps", "disconnect-steps", "duplicate-step"];
//#endregion 🔖️Mutations

pub use super::change_step_collapsed::mutation::{change_step_collapsed, ChangeStepCollapsed};
pub use super::connect_steps::mutation::{connect_steps, ConnectSteps};
pub use super::create_step::mutation::{create_step, CreateStep};
pub use super::delete_step::mutation::{delete_step, DeleteStep};
pub use super::disconnect_steps::mutation::{disconnect_steps, DisconnectSteps};
pub use super::duplicate_step::mutation::{duplicate_step, DuplicateStep};
pub use super::edit_step_params::mutation::{edit_step_params, EditStepParams};
pub use super::move_step::mutation::{move_step, MoveStep};

/// 🔀️ Diffs two fixtures into a minimal typed semantic mutation set. Operates on the plain
/// `SequenceFixture` shape (not `SequenceSnapshot` directly) since the composed `content` child is
/// opaque — callers pass `before.to_fixture()`/a live `SequenceHost.snapshot` as `after`.
pub async fn sequence_snapshot_mutations(before: &SequenceFixture, after: &SequenceFixture) -> Vec<SequenceMutation> {
    let mut mutations = Vec::new();
    let mut deleted_step_ids: Vec<String> = Vec::new();
    for step in &before.steps {
        if !after.steps.iter().any(|entry| entry.id == step.id) {
            mutations.push(delete_step(step.id.clone()));
            deleted_step_ids.push(step.id.clone());
        }
    }
    for step in &after.steps {
        match before.steps.iter().find(|entry| entry.id == step.id) {
            None => mutations.push(create_step(step.clone())),
            Some(prior) => {
                if prior.x != step.x || prior.y != step.y {
                    mutations.push(move_step(step.id.clone(), step.x, step.y));
                }
                if prior.params != step.params {
                    mutations.push(edit_step_params(step.id.clone(), step.params.clone()));
                }
                if prior.collapsed != step.collapsed {
                    mutations.push(change_step_collapsed(step.id.clone(), step.collapsed));
                }
            }
        }
    }
    // 🐛️ Pre-migration bug (confirmed by this ticket's mandatory green-test-run pass — these
    // command tests could never run before, the crate did not compile): `DeleteStep`'s own diff
    // (`🧬️mutations/🗑️delete-step/🔺️diff`) is already a cascade — it drops every edge touching the
    // deleted step as part of THAT mutation. Emitting a separate `DisconnectSteps` for the same
    // edge here is redundant and, worse, invalid: by the time it applies, `DeleteStep` has already
    // removed the edge, so `DisconnectSteps` rejects with "Edge ... does not exist." and the whole
    // batch fails. Skip an edge whose endpoint is one of THIS diff's own deleted steps — `DeleteStep`
    // already accounts for it.
    for edge in &before.edges {
        if deleted_step_ids.iter().any(|id| id == &edge.from || id == &edge.to) {
            continue;
        }
        if !after.edges.iter().any(|entry| entry.id == edge.id) {
            mutations.push(disconnect_steps(edge.id.clone()));
        }
    }
    for edge in &after.edges {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => mutations.push(connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone())),
            Some(prior) if prior.from != edge.from || prior.to != edge.to => {
                mutations.push(disconnect_steps(edge.id.clone()));
                mutations.push(connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone()));
            }
            Some(_) => {}
        }
    }
    mutations
}

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
    use protocol::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;
    use store::{create_document_envelope, ArtifactCommand};

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
    async fn create_step_inverse_law() {
        let base = default_snapshot();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        assert_mutation_inverse_law(&base, &create_step(step));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &delete_step("step-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &move_step("step-1".into(), 42.0, -8.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_disconnect_steps_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &connect_steps("edge-99".into(), "step-1".into(), "step-2".into()));
        assert_mutation_inverse_law(&base, &disconnect_steps("edge-1".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &duplicate_step("step-1".into(), "step-1-copy".into(), 10.0, 10.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_step_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let d1 = move_step("step-1".into(), 10.0, 10.0).diff(&base).into_parts().0;
        let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
        let d2 = move_step("step-1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn dispatch_registers_semantic_descriptors() {
        register_sequence_mutation_descriptors();
        for kind in SequenceMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(SequenceMutation::kinds().len(), 8);
    }
    //#endregion 🔖️MutationLaws

    //#region 🔖️OutcomeLaws
    // 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — one law test per verb
    // family present in this facet (`assert_missing_target_is_error`/`assert_fatal_never_applies`,
    // landed in `📡️spr/🧪️testkit`). `assert_outcome_policy_matrix` is NOT landed under that name
    // (only the generic closure-based `assert_policy_matrix` exists) — see this ticket's report.
    #[semio_framework_async_macros::async_test]
    async fn create_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = create_step(SequenceStep { id: "step-1".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &delete_step("missing".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &move_step("missing".into(), 1.0, 1.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn move_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = move_step("step-1".into(), f64::NAN, 0.0).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }

    #[semio_framework_async_macros::async_test]
    async fn edit_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &edit_step_params("missing".into(), StepParams::new()));
    }

    #[semio_framework_async_macros::async_test]
    async fn change_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &change_step_collapsed("missing".into(), true));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &connect_steps("edge-99".into(), "missing".into(), "step-2".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn connect_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = connect_steps("edge-99".into(), "step-1".into(), "step-1".into()).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }

    #[semio_framework_async_macros::async_test]
    async fn disconnect_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &disconnect_steps("missing".into()));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &duplicate_step("missing".into(), "step-1-copy".into(), 0.0, 0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = duplicate_step("step-1".into(), "step-2".into(), 0.0, 0.0).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }
    //#endregion 🔖️OutcomeLaws

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
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog sequence-1-any");
        }
    }
    //#endregion 🔖️KindsCatalog
}
//#endregion 🧪️Tests
