//! 🛠️ Process3d mutation — `Machines` collection edit.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dDocument, WorkshopMachine, WorkshopMachinePatch};
use protocol::{apply_collection_mutation, CollectionMutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🛠️ `Machines` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Machines {
    pub collection: CollectionMutation<String, WorkshopMachine, WorkshopMachinePatch>,
}

pub fn machines(collection: CollectionMutation<String, WorkshopMachine, WorkshopMachinePatch>) -> Process3dMutation {
    Process3dMutation::Machines { collection }
}

pub fn apply(doc: &mut Process3dDocument, collection: &CollectionMutation<String, WorkshopMachine, WorkshopMachinePatch>) {
    apply_collection_mutation(&mut doc.workshop.machines, collection);
}
//#endregion 🔖️Mutation
