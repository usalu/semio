//! ⚡ VCS app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use vcs::VcsDemoProjection;

//#region 🔖Types
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum VcsDemoDiff {
    #[default]
    Empty,
    SetCounter { counter: i64 },
    SetTitle { title: String },
    SetNotes { notes: String },
    SetStatus { status: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
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
        apply_vcs_demo_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, VcsDemoDiff::Empty) {
            *self = other;
        }
    }
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
            VcsDemoOperation::SetCounter { .. } => vec![VcsDemoOperation::SetCounter {
                counter: projection.counter,
            }],
            VcsDemoOperation::SetTitle { .. } => vec![VcsDemoOperation::SetTitle {
                title: projection.title.clone(),
            }],
            VcsDemoOperation::SetNotes { .. } => vec![VcsDemoOperation::SetNotes {
                notes: projection.notes.clone(),
            }],
            VcsDemoOperation::SetStatus { .. } => vec![VcsDemoOperation::SetStatus {
                status: projection.status.clone(),
            }],
            VcsDemoOperation::AddTag { tag } => vec![VcsDemoOperation::RemoveTag { tag: tag.clone() }],
            VcsDemoOperation::RemoveTag { tag } => vec![VcsDemoOperation::AddTag { tag: tag.clone() }],
        }
    }
}
//#endregion 🔖Types

//#region 🔖DocumentHelpers
fn apply_vcs_demo_operation(projection: &VcsDemoProjection, operation: &VcsDemoOperation) -> VcsDemoProjection {
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
//#endregion 🔖DocumentHelpers

//#region 🧪Tests
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
//#endregion 🧪Tests
