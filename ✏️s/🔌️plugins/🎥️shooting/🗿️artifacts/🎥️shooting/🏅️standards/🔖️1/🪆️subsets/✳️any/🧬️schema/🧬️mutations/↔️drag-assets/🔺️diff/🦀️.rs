//! 🔺 Diff constructor for `DragAssets`. Error `target-missing` when none of the addressed assets
//! exist, Warning `partial` when some do not.

use super::DragAssets;
use crate::diff::ShootingDiff;
use crate::ShootingAssetPatch;
use crate::ShootingSnapshot;

pub fn diff(payload: &DragAssets, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    let patched: Vec<(String, ShootingAssetPatch)> = base
        .assets
        .iter()
        .filter(|asset| payload.asset_ids.contains(&asset.id))
        .map(|asset| (asset.id.clone(), ShootingAssetPatch { origin: Some([asset.origin[0] + payload.dx, asset.origin[1] + payload.dy, asset.origin[2] + payload.dz]), ..Default::default() }))
        .collect();
    if patched.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} requested asset(s) exist.", payload.asset_ids.len()), payload.asset_ids.clone());
    }
    let found: Vec<String> = patched.iter().map(|(id, _)| id.clone()).collect();
    let missing: Vec<String> = payload.asset_ids.iter().filter(|id| !found.contains(id)).cloned().collect();
    let outcome = protocol::MutationOutcome::new(ShootingDiff::asset_patches(patched));
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warning("mutation.partial", format!("{} of {} requested asset(s) did not exist and were skipped.", missing.len(), payload.asset_ids.len())).at(missing)])
    }
}
