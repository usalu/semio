//! 🧬️ VCS artifact — document mutation dispatch enum.

use crate::artifacts::vcs::schema::diff::text::{diff_set_snapshot, VcsDiff, VcsTagsDelta};
use crate::artifacts::vcs::VcsSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant, reason = "SetSnapshot.snapshot carries the whole document")]
pub enum VcsDemoMutation {
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
    SetSnapshot { snapshot: VcsSnapshot },
}

pub fn apply_vcs_demo_mutation(snapshot: &mut VcsSnapshot, mutation: &VcsDemoMutation) {
    match mutation {
        VcsDemoMutation::SetCounter { counter } => super::set_counter::mutation::apply(snapshot, *counter),
        VcsDemoMutation::SetTitle { title } => super::set_title::mutation::apply(snapshot, title),
        VcsDemoMutation::SetNotes { notes } => super::set_notes::mutation::apply(snapshot, notes),
        VcsDemoMutation::SetStatus { status } => super::set_status::mutation::apply(snapshot, status),
        VcsDemoMutation::AddTag { tag } => super::add_tag::mutation::apply(snapshot, tag),
        VcsDemoMutation::RemoveTag { tag } => super::remove_tag::mutation::apply(snapshot, tag),
        VcsDemoMutation::SetSnapshot { snapshot: replacement } => *snapshot = replacement.clone(),
    }
}

pub fn inverse_vcs_demo_mutation(snapshot: &VcsSnapshot, mutation: &VcsDemoMutation) -> Vec<VcsDemoMutation> {
    match mutation {
        VcsDemoMutation::SetCounter { counter } => super::set_counter::inverse::inverse(snapshot, *counter),
        VcsDemoMutation::SetTitle { title } => super::set_title::inverse::inverse(snapshot, title),
        VcsDemoMutation::SetNotes { notes } => super::set_notes::inverse::inverse(snapshot, notes),
        VcsDemoMutation::SetStatus { status } => super::set_status::inverse::inverse(snapshot, status),
        VcsDemoMutation::AddTag { tag } => super::add_tag::inverse::inverse(snapshot, tag),
        VcsDemoMutation::RemoveTag { tag } => super::remove_tag::inverse::inverse(snapshot, tag),
        VcsDemoMutation::SetSnapshot { .. } => vec![VcsDemoMutation::SetSnapshot { snapshot: snapshot.clone() }],
    }
}

impl Mutation<VcsSnapshot> for VcsDemoMutation {
    type Diff = VcsDiff;

    fn diff(&self, _snapshot: &VcsSnapshot) -> Self::Diff {
        match self {
            VcsDemoMutation::SetCounter { counter } => VcsDiff { counter: Some(*counter), ..Default::default() },
            VcsDemoMutation::SetTitle { title } => VcsDiff { title: Some(title.clone()), ..Default::default() },
            VcsDemoMutation::SetNotes { notes } => VcsDiff { notes: Some(notes.clone()), ..Default::default() },
            VcsDemoMutation::SetStatus { status } => VcsDiff { status: Some(status.clone()), ..Default::default() },
            VcsDemoMutation::AddTag { tag } => VcsDiff {
                tags: Some(VcsTagsDelta { added: vec![tag.clone()], ..Default::default() }),
                ..Default::default()
            },
            VcsDemoMutation::RemoveTag { tag } => VcsDiff {
                tags: Some(VcsTagsDelta { removed: vec![tag.clone()], ..Default::default() }),
                ..Default::default()
            },
            VcsDemoMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &VcsSnapshot) -> Vec<Self> {
        inverse_vcs_demo_mutation(snapshot, self)
    }
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::vcs::engine;

    #[test]
    fn vcs_demo_mutation_round_trips_store() {
        let mut store = store::ArtifactStore::<VcsSnapshot, VcsDemoMutation>::new(store::create_document_envelope("vcs.document", "vcs", engine::empty_vcs_snapshot(), None));
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 3 }], description: None }).expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").counter, 3);
    }
}
//#endregion 🧪️Tests
