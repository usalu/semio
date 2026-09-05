//! 🔺 Diff constructor for `CreateAsset`. Fatal `duplicate-id` on an existing id.

use super::CreateAsset;
use crate::artifacts::shooting::diff::{ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::ShootingSnapshot;

pub async fn diff(payload: &CreateAsset, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if base.assets.iter().any(|asset| asset.id == payload.asset.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An asset with id \"{}\" already exists.", payload.asset.id), [payload.asset.id.clone()]);
    }
    protocol::MutationOutcome::new(ShootingDiff { assets: Some(ShootingAssetsDelta { added: vec![payload.asset.clone()], ..Default::default() }), ..Default::default() })
}
