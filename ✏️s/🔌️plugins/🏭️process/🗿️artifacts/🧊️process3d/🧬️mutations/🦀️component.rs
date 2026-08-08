//! 🧬️ Process3d artifact — document mutation dispatch enum.

use crate::artifacts::process3d::diff::{
    diff_set_snapshot, steps_delta_from_collection_mutation, workshop_after_machines_mutation, Process3dDiff,
};
use crate::artifacts::process3d::{Process3dSnapshot, ProcessStep, ProcessStepPatch, Stock, WorkshopMachine, WorkshopMachinePatch};
use protocol::{inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧱 Process 3d document mutation: an ordered-step collection edit, a workshop-machines collection
/// edit, a stock swap, a cursor move, or a whole-snapshot replacement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Process3dMutation {
    Steps {
        collection: CollectionMutation<String, ProcessStep, ProcessStepPatch>,
    },
    Machines {
        collection: CollectionMutation<String, WorkshopMachine, WorkshopMachinePatch>,
    },
    SetStock {
        stock: Stock,
    },
    SetCursor {
        resolved_up_to: Option<usize>,
    },
    /// 🔁️ Wholesale snapshot swap (loading a different example fixture).
    SetSnapshot {
        snapshot: Process3dSnapshot,
    },
}

impl Mutation<Process3dSnapshot> for Process3dMutation {
    type Diff = Process3dDiff;

    fn diff(&self, snapshot: &Process3dSnapshot) -> Self::Diff {
        match self {
            Process3dMutation::Steps { collection } => Process3dDiff {
                steps: Some(steps_delta_from_collection_mutation(&snapshot.steps, collection)),
                ..Default::default()
            },
            Process3dMutation::Machines { collection } => Process3dDiff {
                workshop: Some(workshop_after_machines_mutation(&snapshot.workshop, collection)),
                ..Default::default()
            },
            Process3dMutation::SetStock { stock } => Process3dDiff { stock: Some(stock.clone()), ..Default::default() },
            Process3dMutation::SetCursor { resolved_up_to } => Process3dDiff { resolved_up_to: Some(*resolved_up_to), ..Default::default() },
            Process3dMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &Process3dSnapshot) -> Vec<Self> {
        match self {
            Process3dMutation::Steps { collection } => super::steps::inverse::inverse(snapshot, collection),
            Process3dMutation::Machines { collection } => super::machines::inverse::inverse(snapshot, collection),
            Process3dMutation::SetStock { stock } => super::set_stock::inverse::inverse(snapshot, stock),
            Process3dMutation::SetCursor { resolved_up_to } => super::set_cursor::inverse::inverse(snapshot, *resolved_up_to),
            Process3dMutation::SetSnapshot { .. } => super::set_snapshot::inverse::inverse(snapshot),
        }
    }
}

pub use super::steps::mutation::{steps, Steps};
pub use super::machines::mutation::{machines, Machines};
pub use super::set_stock::mutation::{set_stock, SetStock};
pub use super::set_cursor::mutation::{set_cursor, SetCursor};
pub use super::set_snapshot::mutation::{set_snapshot, SetSnapshot};
//#endregion 🔖️Mutations
