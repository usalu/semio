//! 🔺️ Process3d artifact — the `Process3dDiff` projection-patch type + its `MutationDiff` impl,
//! extracted from the old `🔧️op` crate's combined operation+diff region.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::process3d::{Process3dDocument, ProcessStep, ProcessStepPatch, Stock, WorkshopMachine, WorkshopMachinePatch};
use protocol::{apply_collection_mutation, CollectionMutation, MutationDiff};
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<CollectionMutation<String, ProcessStep, ProcessStepPatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub machines: Option<CollectionMutation<String, WorkshopMachine, WorkshopMachinePatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stock: Option<Stock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Process3dDocument>,
}

impl MutationDiff<Process3dDocument> for Process3dDiff {
    fn apply(&self, projection: &Process3dDocument) -> Process3dDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(mutation) = &self.steps {
            apply_collection_mutation(&mut next.steps, mutation);
        }
        if let Some(mutation) = &self.machines {
            apply_collection_mutation(&mut next.workshop.machines, mutation);
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
