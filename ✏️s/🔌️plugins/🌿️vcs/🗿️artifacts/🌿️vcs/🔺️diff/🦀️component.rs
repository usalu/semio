//! 🔺️ VCS artifact — per-field diff type + `OperationDiff` law (was: part of constitutional `op`).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::vcs::op::VcsDemoOperation;
use crate::artifacts::vcs::VcsDemoProjection;
use protocol::OperationDiff;
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

impl OperationDiff<VcsDemoProjection> for VcsDemoDiff {
    fn apply(&self, projection: &VcsDemoProjection) -> VcsDemoProjection {
        let operation = match self {
            VcsDemoDiff::Empty => return projection.clone(),
            VcsDemoDiff::SetCounter { counter } => VcsDemoOperation::SetCounter { counter: *counter },
            VcsDemoDiff::SetTitle { title } => VcsDemoOperation::SetTitle { title: title.clone() },
            VcsDemoDiff::SetNotes { notes } => VcsDemoOperation::SetNotes { notes: notes.clone() },
            VcsDemoDiff::SetStatus { status } => VcsDemoOperation::SetStatus { status: status.clone() },
            VcsDemoDiff::AddTag { tag } => VcsDemoOperation::AddTag { tag: tag.clone() },
            VcsDemoDiff::RemoveTag { tag } => VcsDemoOperation::RemoveTag { tag: tag.clone() },
        };
        crate::artifacts::vcs::op::apply_vcs_demo_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, VcsDemoDiff::Empty) {
            *self = other;
        }
    }
}
//#endregion 🔖️Types
