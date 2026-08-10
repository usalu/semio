//! ⏱️ Process3d mutation — `SetCursor`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ⏱️ `SetCursor` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCursor {
    pub resolved_up_to: Option<usize>,
}

pub fn set_cursor(resolved_up_to: Option<usize>) -> Process3dMutation {
    Process3dMutation::SetCursor { resolved_up_to }
}

pub fn apply(doc: &mut Process3dSnapshot, resolved_up_to: Option<usize>) {
    doc.resolved_up_to = resolved_up_to;
    if let Some(cursor) = doc.resolved_up_to {
        doc.resolved_up_to = Some(cursor.min(doc.steps.len()));
    }
}
//#endregion 🔖️Mutation
