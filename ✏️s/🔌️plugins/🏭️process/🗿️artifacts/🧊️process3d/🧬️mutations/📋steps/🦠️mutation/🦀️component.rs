//! 📋 Process3d mutation — `Steps` collection edit.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dDocument, ProcessStep, ProcessStepPatch};
use protocol::{apply_collection_mutation, CollectionMutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 📋 `Steps` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Steps {
    pub collection: CollectionMutation<String, ProcessStep, ProcessStepPatch>,
}

pub fn steps(collection: CollectionMutation<String, ProcessStep, ProcessStepPatch>) -> Process3dMutation {
    Process3dMutation::Steps { collection }
}

pub fn apply(doc: &mut Process3dDocument, collection: &CollectionMutation<String, ProcessStep, ProcessStepPatch>) {
    apply_collection_mutation(&mut doc.steps, collection);
}
//#endregion 🔖️Mutation
