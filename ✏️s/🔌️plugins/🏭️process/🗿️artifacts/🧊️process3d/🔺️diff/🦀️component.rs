//! 🔺️ Process3d artifact — the `Process3dDiff` projection-patch type + its `OperationDiff` impl,
//! extracted from the old `🔧️op` crate's combined operation+diff region.

use crate::artifacts::process3d::{Process3dDocument, ProcessStep, ProcessStepPatch, Stock, WorkshopMachine, WorkshopMachinePatch};
use protocol::{apply_collection_operation, CollectionOperation, OperationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<CollectionOperation<String, ProcessStep, ProcessStepPatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub machines: Option<CollectionOperation<String, WorkshopMachine, WorkshopMachinePatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stock: Option<Stock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Process3dDocument>,
}

impl OperationDiff<Process3dDocument> for Process3dDiff {
    fn apply(&self, projection: &Process3dDocument) -> Process3dDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(operation) = &self.steps {
            apply_collection_operation(&mut next.steps, operation);
        }
        if let Some(operation) = &self.machines {
            apply_collection_operation(&mut next.workshop.machines, operation);
        }
        if let Some(stock) = &self.stock {
            next.stock = stock.clone();
        }
        if let Some(cursor) = &self.cursor {
            next.resolved_up_to = *cursor;
        }
        if let Some(cursor) = next.resolved_up_to {
            next.resolved_up_to = Some(cursor.min(next.steps.len()));
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
            self.steps = None;
            self.machines = None;
            self.stock = None;
            self.cursor = None;
            return;
        }
        if other.steps.is_some() {
            self.steps = other.steps;
        }
        if other.machines.is_some() {
            self.machines = other.machines;
        }
        if other.stock.is_some() {
            self.stock = other.stock;
        }
        if other.cursor.is_some() {
            self.cursor = other.cursor;
        }
    }
}
//#endregion 🔖️Diff
