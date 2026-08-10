//! ↩️ Inverse for `Assets`.
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::{ShootingAsset, ShootingAssetPatch, ShootingSnapshot};
use protocol::{inverse_collection_mutation, CollectionMutation};

//#region 🔖️Inverse
pub fn inverse(base: &ShootingSnapshot, mutation: &CollectionMutation<String, ShootingAsset, ShootingAssetPatch>) -> Vec<ShootingMutation> {
    vec![ShootingMutation::Assets(inverse_collection_mutation(&base.assets, mutation))]
}
//#endregion 🔖️Inverse
