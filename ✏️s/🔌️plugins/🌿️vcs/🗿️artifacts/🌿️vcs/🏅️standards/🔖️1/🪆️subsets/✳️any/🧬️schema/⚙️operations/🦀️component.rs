//! ⚙️ VCS mutation protocol dispatch, codec bridges, and cross-mutation store laws.

use crate::artifacts::vcs::mutations::{add_tag, change_counter, change_notes, change_status, register_vcs_demo_mutation_descriptors, remove_tag, rename_vcs, AddTag, ChangeCounter, ChangeNotes, ChangeStatus, RemoveTag, RenameVcs, VcsDemoMutation};
use crate::artifacts::vcs::VcsSnapshot;

//#region 🏷️Roster
/// 🏷️ Language-neutral catalog roster in aggregate declaration order.
pub const KINDS: &[&str] = &["rename-vcs", "change-counter", "change-notes", "change-status", "add-tag", "remove-tag"];
//#endregion 🏷️Roster

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot` through its own diff — the artifact's single apply entry
/// point (mirrors dag's `apply_dag_mutation`/puzzle5d's `apply_puzzle5d_mutation`). A rejecting
/// diff carries an empty `VcsDiff`, so the snapshot is left untouched and `Ok(())` is still
/// returned; read [`protocol::MutationOutcome::messages`] to distinguish the two.
pub fn apply_vcs_mutation(snapshot: &mut VcsSnapshot, mutation: &VcsDemoMutation) -> protocol::MutationApplyResult<()> {
    use store::MutationDiff;
    let next = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(mutation, snapshot).diff().apply(snapshot)?;
    *snapshot = next;
    Ok(())
}

/// ↩️ The typed mutation steps that undo `mutation` against `snapshot`.
pub fn inverse_vcs_mutation(snapshot: &VcsSnapshot, mutation: &VcsDemoMutation) -> Vec<VcsDemoMutation> {
    <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::inverse(mutation, snapshot)
}
//#endregion 🔖️Apply

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "addTag", …}`, camelCase payload
/// fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json` specification vectors carry — into a real
/// [`VcsDemoMutation`]. The `mutate-vcs-1` adapter cannot reach `serde_json` (the generated test
/// host links only `semio-repo-test-host` and this crate) and cannot name this crate's private
/// `protocol`/`store` extern-crate aliases either, so the bridge belongs here rather than there.
pub fn decode_vcs_mutation_json(text: &str) -> Result<VcsDemoMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ [`apply_vcs_mutation`]'s reporting, non-async twin: applies `mutation` in place and returns
/// the diagnostic CODES it raised, in order. [`apply_vcs_mutation`] discards them and is `async`,
/// so neither the outcome-policy claim a committed `🎯️outcome/🔣️component.json` makes nor a
/// synchronous test adapter can be served by it.
pub fn apply_vcs_mutation_reporting(snapshot: &mut VcsSnapshot, mutation: &VcsDemoMutation) -> Vec<String> {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(mutation, snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| message.code.0.clone()).collect()
}

/// ↩️ [`inverse_vcs_mutation`]'s non-async twin — the mutation's OWN computed undo steps, which is
/// what an `inverse-<kind>` scenario has to apply for the metamorphic law to mean anything.
pub fn inverse_vcs_mutation_steps(mutation: &VcsDemoMutation, base: &VcsSnapshot) -> Vec<VcsDemoMutation> {
    <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::inverse(mutation, base)
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot;
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::{Mutation, MutationDiff, MutationKind, SemanticMutation};

    #[semio_framework_async_macros::async_test]
    fn vcs_demo_mutation_round_trips_store() {
        let mut store = store::ArtifactStore::<VcsSnapshot, VcsDemoMutation>::new(store::create_document_envelope("vcs.document", "vcs", empty_vcs_snapshot(), None)).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![change_counter(3)], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").counter, 3);
    }

    #[semio_framework_async_macros::async_test]
    fn rename_vcs_inverse_law_holds() {
        let base = empty_vcs_snapshot();
        let mutation = rename_vcs("Renamed".into());
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    fn change_counter_inverse_law_holds() {
        let base = empty_vcs_snapshot();
        let mutation = change_counter(42);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    fn add_tag_then_remove_tag_inverse_laws_hold() {
        let base = empty_vcs_snapshot();
        assert_mutation_inverse_law(&base, &add_tag("wip".into()));
        let mut with_tag = base.clone();
        with_tag.tags.push("wip".into());
        assert_mutation_inverse_law(&with_tag, &remove_tag("wip".into()));
    }

    #[semio_framework_async_macros::async_test]
    fn change_notes_diff_absorb_law_holds() {
        let base = empty_vcs_snapshot();
        let d1 = change_notes("first".into()).diff(&base).into_parts().0;
        let mid = d1.apply(&base).expect("valid mutation diff");
        let d2 = change_notes("second".into()).diff(&mid).into_parts().0;
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    fn add_tag_is_a_noop_when_base_already_has_the_tag() {
        let mut base = empty_vcs_snapshot();
        base.tags.push("wip".into());
        let payload = AddTag { tag: "wip".into() };
        let outcome = MutationKind::diff(&payload, &base);
        assert_eq!(outcome.diff(), &crate::artifacts::vcs::VcsDiff::default());
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"), "a duplicate add must carry a no-op message");
        assert!(MutationKind::inverse(&payload, &base).is_empty(), "inverse of a no-op add must have nothing to undo");
    }

    /// 🪧 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS Pass 3 — the `remove`
    /// family is this facet's only verb with a real Error `target-missing` path (`rename`/`change`/
    /// `add` are all root-scoped or no-op-only here — see this lane's report for the full verb→family
    /// mapping and reasoning). `assert_fatal_never_applies` has no meaningful call site: this facet
    /// introduces no Fatal path (no `duplicate-id`/`invariant` verb in its vocabulary).
    /// `assert_outcome_policy_matrix` is not landed under that name — only `assert_policy_matrix`.
    #[semio_framework_async_macros::async_test]
    fn remove_tag_missing_target_is_error() {
        let base = empty_vcs_snapshot();
        let mutation = VcsDemoMutation::RemoveTag(RemoveTag { tag: "gone".into() });
        protocol::testkit::assert_missing_target_is_error(&base, &mutation);
    }

    /// 🏷️ The three declarations of this vocabulary — the enum, [`KINDS`] and the committed
    /// catalog — must agree, in spelling AND in order. The framework never parses Rust, so without
    /// this test `KINDS` could drift from the enum and the catalog could keep measuring
    /// `mutate-vcs-1` against a vocabulary the artifact no longer has.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = VcsDemoMutation::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
        assert!(!manifest.contains("\"set-snapshot\"") && !manifest.contains("\"no-mutation\""), "whole-document replace is banned vocabulary here — the catalog must not smuggle it back in");
    }

    #[semio_framework_async_macros::async_test]
    fn dispatch_registers_semantic_descriptors() {
        register_vcs_demo_mutation_descriptors();
        for kind in VcsDemoMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(VcsDemoMutation::kinds().len(), 6);
    }
}
//#endregion 🧪️Tests
