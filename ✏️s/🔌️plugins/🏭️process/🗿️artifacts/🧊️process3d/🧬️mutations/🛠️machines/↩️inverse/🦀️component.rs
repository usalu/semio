//! ↩️ Inverse for `Machines`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dDocument, WorkshopMachine, WorkshopMachinePatch};
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &Process3dDocument, collection: &CollectionMutation<String, WorkshopMachine, WorkshopMachinePatch>) -> Vec<Process3dMutation> {
    vec![Process3dMutation::Machines { collection: inverse_collection_mutation(&base.workshop.machines, collection) }]
}
//#endregion 🔖️Inverse
