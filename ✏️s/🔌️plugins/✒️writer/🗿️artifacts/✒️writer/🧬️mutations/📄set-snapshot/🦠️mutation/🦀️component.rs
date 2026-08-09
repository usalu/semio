//! 📄 Writer mutation — `SetSnapshot` apply.
use crate::artifacts::writer::WriterSnapshot;
use crate::artifacts::writer::mutations::WriterMutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 📄 `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: WriterSnapshot,
}

pub fn set_snapshot(snapshot: WriterSnapshot) -> WriterMutation {
    WriterMutation::SetSnapshot { snapshot }
}

pub fn apply(target: &mut WriterSnapshot, replacement: &WriterSnapshot) {
    *target = replacement.clone();
}
//#endregion 🔖️Mutation
