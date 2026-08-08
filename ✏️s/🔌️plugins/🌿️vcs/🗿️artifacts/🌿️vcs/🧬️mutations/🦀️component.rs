//! 🧬️ VCS artifact — document mutation dispatch enum.

use crate::artifacts::vcs::diff::VcsDemoDiff;
use crate::artifacts::vcs::VcsDemoProjection;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum VcsDemoMutation {
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
}

pub fn apply_vcs_demo_mutation(projection: &mut VcsDemoProjection, mutation: &VcsDemoMutation) {
    match mutation {
        VcsDemoMutation::SetCounter { counter } => super::set_counter::mutation::apply(projection, *counter),
        VcsDemoMutation::SetTitle { title } => super::set_title::mutation::apply(projection, title),
        VcsDemoMutation::SetNotes { notes } => super::set_notes::mutation::apply(projection, notes),
        VcsDemoMutation::SetStatus { status } => super::set_status::mutation::apply(projection, status),
        VcsDemoMutation::AddTag { tag } => super::add_tag::mutation::apply(projection, tag),
        VcsDemoMutation::RemoveTag { tag } => super::remove_tag::mutation::apply(projection, tag),
    }
}

pub fn inverse_vcs_demo_mutation(projection: &VcsDemoProjection, mutation: &VcsDemoMutation) -> Vec<VcsDemoMutation> {
    match mutation {
        VcsDemoMutation::SetCounter { counter } => super::set_counter::inverse::inverse(projection, *counter),
        VcsDemoMutation::SetTitle { title } => super::set_title::inverse::inverse(projection, title),
        VcsDemoMutation::SetNotes { notes } => super::set_notes::inverse::inverse(projection, notes),
        VcsDemoMutation::SetStatus { status } => super::set_status::inverse::inverse(projection, status),
        VcsDemoMutation::AddTag { tag } => super::add_tag::inverse::inverse(projection, tag),
        VcsDemoMutation::RemoveTag { tag } => super::remove_tag::inverse::inverse(projection, tag),
    }
}

impl Mutation<VcsDemoProjection> for VcsDemoMutation {
    type Diff = VcsDemoDiff;

    fn diff(&self, _projection: &VcsDemoProjection) -> Self::Diff {
        match self {
            VcsDemoMutation::SetCounter { counter } => VcsDemoDiff::SetCounter { counter: *counter },
            VcsDemoMutation::SetTitle { title } => VcsDemoDiff::SetTitle { title: title.clone() },
            VcsDemoMutation::SetNotes { notes } => VcsDemoDiff::SetNotes { notes: notes.clone() },
            VcsDemoMutation::SetStatus { status } => VcsDemoDiff::SetStatus { status: status.clone() },
            VcsDemoMutation::AddTag { tag } => VcsDemoDiff::AddTag { tag: tag.clone() },
            VcsDemoMutation::RemoveTag { tag } => VcsDemoDiff::RemoveTag { tag: tag.clone() },
        }
    }

    fn inverse(&self, projection: &VcsDemoProjection) -> Vec<Self> {
        inverse_vcs_demo_mutation(projection, self)
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
        let mut store = store::DocumentStore::<VcsDemoProjection, VcsDemoMutation>::new(store::create_document_envelope("vcs.document", "vcs", engine::empty_vcs_demo_projection(), None));
        store.dispatch(store::DocumentCommand::Apply { mutations: vec![VcsDemoMutation::SetCounter { counter: 3 }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").counter, 3);
    }
}
//#endregion 🧪️Tests
