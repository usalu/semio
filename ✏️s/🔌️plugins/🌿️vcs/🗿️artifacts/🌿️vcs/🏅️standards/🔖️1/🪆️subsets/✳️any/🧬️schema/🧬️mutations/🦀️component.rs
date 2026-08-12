//! 🧬️ VCS artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload (see the `🧬️mutations/<slug>/`
//! triad leaves); `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<VcsSnapshot>` and
//! `impl protocol::SemanticMutation<VcsSnapshot>` from those payloads — no hand-written
//! apply/diff/inverse dispatch here. Whole-document replace (the old `SetSnapshot`) is banned
//! vocabulary per the taxonomy; it has no mutation replacement and goes through the store's
//! non-history `reset` path instead (see this report's `sharedFileRequests`).

use crate::artifacts::vcs::VcsSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = VcsSnapshot, diff = crate::artifacts::vcs::VcsDiff, schema = "vcs.vcs")]
pub enum VcsDemoMutation {
    RenameVcs(RenameVcs),
    ChangeCounter(ChangeCounter),
    ChangeNotes(ChangeNotes),
    ChangeStatus(ChangeStatus),
    AddTag(AddTag),
    RemoveTag(RemoveTag),
}

pub use super::add_tag::mutation::{add_tag, AddTag};
pub use super::change_counter::mutation::{change_counter, ChangeCounter};
pub use super::change_notes::mutation::{change_notes, ChangeNotes};
pub use super::change_status::mutation::{change_status, ChangeStatus};
pub use super::remove_tag::mutation::{remove_tag, RemoveTag};
pub use super::rename_vcs::mutation::{rename_vcs, RenameVcs};
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::vcs::engine;
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::{Mutation, MutationDiff, MutationKind, SemanticMutation};

    #[test]
    fn vcs_demo_mutation_round_trips_store() {
        let mut store = store::ArtifactStore::<VcsSnapshot, VcsDemoMutation>::new(store::create_document_envelope("vcs.document", "vcs", engine::empty_vcs_snapshot(), None));
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![change_counter(3)], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").counter, 3);
    }

    #[test]
    fn rename_vcs_inverse_law_holds() {
        let base = engine::empty_vcs_snapshot();
        let mutation = rename_vcs("Renamed".into());
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn change_counter_inverse_law_holds() {
        let base = engine::empty_vcs_snapshot();
        let mutation = change_counter(42);
        assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn add_tag_then_remove_tag_inverse_laws_hold() {
        let base = engine::empty_vcs_snapshot();
        assert_mutation_inverse_law(&base, &add_tag("wip".into()));
        let mut with_tag = base.clone();
        with_tag.tags.push("wip".into());
        assert_mutation_inverse_law(&with_tag, &remove_tag("wip".into()));
    }

    #[test]
    fn change_notes_diff_absorb_law_holds() {
        let base = engine::empty_vcs_snapshot();
        let d1 = change_notes("first".into()).diff(&base);
        let mid = d1.apply(&base);
        let d2 = change_notes("second".into()).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn add_tag_is_a_noop_when_base_already_has_the_tag() {
        let mut base = engine::empty_vcs_snapshot();
        base.tags.push("wip".into());
        let payload = AddTag { tag: "wip".into() };
        assert_eq!(MutationKind::diff(&payload, &base), crate::artifacts::vcs::VcsDiff::default());
        assert!(MutationKind::inverse(&payload, &base).is_empty(), "inverse of a no-op add must have nothing to undo");
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_vcs_demo_mutation_descriptors();
        for kind in VcsDemoMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(VcsDemoMutation::kinds().len(), 6);
    }
}
//#endregion 🧪️Tests
