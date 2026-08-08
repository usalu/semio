//! ↩️ Inverse for `Shots`.
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingShot, ShootingShotPatch, ShootingFixture};
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &ShootingFixture, mutation: &CollectionMutation<String, ShootingShot, ShootingShotPatch>) -> Vec<ShootingMutation> {
    vec![ShootingMutation::Shots(inverse_collection_mutation(&base.shots, mutation))]
}
//#endregion 🔖️Inverse
