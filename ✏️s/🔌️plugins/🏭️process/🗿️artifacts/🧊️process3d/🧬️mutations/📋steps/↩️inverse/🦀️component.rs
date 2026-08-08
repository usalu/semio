//! ↩️ Inverse for `Steps`.
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessStep, ProcessStepPatch};
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &Process3dSnapshot, collection: &CollectionMutation<String, ProcessStep, ProcessStepPatch>) -> Vec<Process3dMutation> {
    vec![Process3dMutation::Steps { collection: inverse_collection_mutation(&base.steps, collection) }]
}
//#endregion 🔖️Inverse
