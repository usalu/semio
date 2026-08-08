//! 🧬️ Process3d artifact — document mutation dispatch enum.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::{Process3dDocument, ProcessStep, ProcessStepPatch, Stock, WorkshopMachine, WorkshopMachinePatch};
use protocol::{CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 🧱 Process 3d document mutation: an ordered-step collection edit, a workshop-machines collection
/// edit, a stock swap, or a cursor move.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
    /// 🔁️ Wholesale document swap (loading a different example fixture) — a true inverse restores the
    /// exact prior document.
    SetDocument {
        document: Process3dDocument,
    },
}

impl Mutation<Process3dDocument> for Process3dMutation {
    type Diff = Process3dDiff;

    fn diff(&self, _projection: &Process3dDocument) -> Self::Diff {
        match self {
            Process3dMutation::Steps { collection } => Process3dDiff { steps: Some(collection.clone()), ..Default::default() },
            Process3dMutation::Machines { collection } => Process3dDiff { machines: Some(collection.clone()), ..Default::default() },
            Process3dMutation::SetStock { stock } => Process3dDiff { stock: Some(stock.clone()), ..Default::default() },
            Process3dMutation::SetCursor { resolved_up_to } => Process3dDiff { cursor: Some(*resolved_up_to), ..Default::default() },
            Process3dMutation::SetDocument { document } => Process3dDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &Process3dDocument) -> Vec<Self> {
        match self {
            Process3dMutation::Steps { collection } => super::steps::inverse::inverse(projection, collection),
            Process3dMutation::Machines { collection } => super::machines::inverse::inverse(projection, collection),
            Process3dMutation::SetStock { stock } => super::set_stock::inverse::inverse(projection, stock),
            Process3dMutation::SetCursor { resolved_up_to } => super::set_cursor::inverse::inverse(projection, *resolved_up_to),
            Process3dMutation::SetDocument { document } => super::set_document::inverse::inverse(projection, document),
        }
    }
}

pub use super::steps::mutation::{steps, Steps};
pub use super::machines::mutation::{machines, Machines};
pub use super::set_stock::mutation::{set_stock, SetStock};
pub use super::set_cursor::mutation::{set_cursor, SetCursor};
pub use super::set_document::mutation::{set_document, SetDocument};
//#endregion 🔖️Mutations
