//! 📄 Process3d mutation — `SetSnapshot`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📄 `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    pub snapshot: Process3dSnapshot,
}

pub fn set_snapshot(snapshot: Process3dSnapshot) -> Process3dMutation {
    Process3dMutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut Process3dSnapshot, replacement: &Process3dSnapshot) {
    *base = replacement.clone();
}
//#endregion 🔖️Mutation
