//! ↩ Inverse constructor for `ReorderAssets` — reconstructed from BASE state.

use super::mutation::ReorderAssets;
use crate::artifacts::shooting::mutations::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;


pub async fn inverse(payload: &ReorderAssets, base: &ShootingSnapshot) -> Vec<ShootingMutation> {
    match base.assets.iter().position(|asset| asset.id == payload.id) {
        Some(original_index) => vec![ShootingMutation::ReorderAssets(ReorderAssets { id: payload.id.clone(), to_index: original_index })],
        None => Vec::new(),
    }
}
