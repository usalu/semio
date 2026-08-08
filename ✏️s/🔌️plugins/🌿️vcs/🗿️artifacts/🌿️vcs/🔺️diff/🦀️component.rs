//! 🔺️ VCS artifact — per-field diff type + `MutationDiff` law (was: part of constitutional `op`).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::vcs::mutations::VcsDemoMutation;
use crate::artifacts::vcs::VcsDemoProjection;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum VcsDemoDiff {
    #[default]
    Empty,
    SetCounter {
        counter: i64,
    },
    SetTitle {
        title: String,
    },
    SetNotes {
        notes: String,
    },
    SetStatus {
        status: String,
    },
    AddTag {
        tag: String,
    },
    RemoveTag {
        tag: String,
    },
}

impl MutationDiff<VcsDemoProjection> for VcsDemoDiff {
    fn apply(&self, projection: &VcsDemoProjection) -> VcsDemoProjection {
        let operation = match self {
            VcsDemoDiff::Empty => return projection.clone(),
            VcsDemoDiff::SetCounter { counter } => VcsDemoMutation::SetCounter { counter: *counter },
            VcsDemoDiff::SetTitle { title } => VcsDemoMutation::SetTitle { title: title.clone() },
            VcsDemoDiff::SetNotes { notes } => VcsDemoMutation::SetNotes { notes: notes.clone() },
            VcsDemoDiff::SetStatus { status } => VcsDemoMutation::SetStatus { status: status.clone() },
            VcsDemoDiff::AddTag { tag } => VcsDemoMutation::AddTag { tag: tag.clone() },
            VcsDemoDiff::RemoveTag { tag } => VcsDemoMutation::RemoveTag { tag: tag.clone() },
        };
        let mut next = projection.clone();
        crate::artifacts::vcs::mutations::apply_vcs_demo_mutation(&mut next, &operation);
        next
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, VcsDemoDiff::Empty) {
            *self = other;
        }
    }
}
//#endregion 🔖️Types
