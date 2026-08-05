//! ⚡️ VCS artifact — operation enum + laws (was: constitutional `op`).

use crate::artifacts::vcs::diff::VcsDemoDiff;
use crate::artifacts::vcs::VcsDemoProjection;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum VcsDemoOperation {
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
}

impl Operation<VcsDemoProjection> for VcsDemoOperation {
    type Diff = VcsDemoDiff;

    fn diff(&self, _projection: &VcsDemoProjection) -> Self::Diff {
        match self {
            VcsDemoOperation::SetCounter { counter } => VcsDemoDiff::SetCounter { counter: *counter },
            VcsDemoOperation::SetTitle { title } => VcsDemoDiff::SetTitle { title: title.clone() },
            VcsDemoOperation::SetNotes { notes } => VcsDemoDiff::SetNotes { notes: notes.clone() },
            VcsDemoOperation::SetStatus { status } => VcsDemoDiff::SetStatus { status: status.clone() },
            VcsDemoOperation::AddTag { tag } => VcsDemoDiff::AddTag { tag: tag.clone() },
            VcsDemoOperation::RemoveTag { tag } => VcsDemoDiff::RemoveTag { tag: tag.clone() },
        }
    }

    fn backwards(&self, projection: &VcsDemoProjection) -> Vec<Self> {
        match self {
            VcsDemoOperation::SetCounter { .. } => vec![VcsDemoOperation::SetCounter { counter: projection.counter }],
            VcsDemoOperation::SetTitle { .. } => vec![VcsDemoOperation::SetTitle { title: projection.title.clone() }],
            VcsDemoOperation::SetNotes { .. } => vec![VcsDemoOperation::SetNotes { notes: projection.notes.clone() }],
            VcsDemoOperation::SetStatus { .. } => vec![VcsDemoOperation::SetStatus { status: projection.status.clone() }],
            VcsDemoOperation::AddTag { tag } => vec![VcsDemoOperation::RemoveTag { tag: tag.clone() }],
            VcsDemoOperation::RemoveTag { tag } => vec![VcsDemoOperation::AddTag { tag: tag.clone() }],
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
/// 🔺️ Shared by [`VcsDemoDiff::apply`] (the diff is a thin wrapper around the same field write).
pub fn apply_vcs_demo_operation(projection: &VcsDemoProjection, operation: &VcsDemoOperation) -> VcsDemoProjection {
    let mut next = projection.clone();
    match operation {
        VcsDemoOperation::SetCounter { counter } => next.counter = *counter,
        VcsDemoOperation::SetTitle { title } => next.title = title.clone(),
        VcsDemoOperation::SetNotes { notes } => next.notes = notes.clone(),
        VcsDemoOperation::SetStatus { status } => next.status = status.clone(),
        VcsDemoOperation::AddTag { tag } => {
            if !next.tags.contains(tag) {
                next.tags.push(tag.clone());
            }
        }
        VcsDemoOperation::RemoveTag { tag } => next.tags.retain(|entry| entry != tag),
    }
    next
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::SetCounter { counter: 3 });
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::SetTitle { title: "Untitled".into() });
        store::test_support::assert_op_line_round_trip(&VcsDemoOperation::AddTag { tag: "draft".into() });
    }
}
//#endregion 🧪️Tests
