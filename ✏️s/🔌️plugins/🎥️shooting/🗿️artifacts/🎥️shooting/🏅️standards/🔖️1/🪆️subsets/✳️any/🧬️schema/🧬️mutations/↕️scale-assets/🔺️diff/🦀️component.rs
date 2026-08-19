//! 🔺 Diff constructor for `ScaleAssets`. Fatal `invariant` when a scale factor is non-finite or
//! non-positive, Error `target-missing` when none of the addressed assets exist, Warning `partial`
//! when some do not.

use super::mutation::ScaleAssets;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetPatchEntry, ShootingAssetsDelta, ShootingDiff};
use crate::artifacts::shooting::{shooting_asset_scale, ShootingAssetPatch};

pub async fn diff(payload: &ScaleAssets, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if [payload.sx, payload.sy, payload.sz].iter().any(|value| !value.is_finite() || *value <= 0.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Scale factors must be finite and positive, got ({}, {}, {}).", payload.sx, payload.sy, payload.sz), payload.asset_ids.clone());
    }
    let patched: Vec<ShootingAssetPatchEntry> = base
        .assets
        .iter()
        .filter(|asset| payload.asset_ids.contains(&asset.id))
        .map(|asset| {
            let current = shooting_asset_scale(asset);
            ShootingAssetPatchEntry { id: asset.id.clone(), patch: ShootingAssetPatch { scale: Some([current[0] * payload.sx, current[1] * payload.sy, current[2] * payload.sz]), ..Default::default() } }
        })
        .collect();
    if patched.is_empty() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("None of the {} requested asset(s) exist.", payload.asset_ids.len()), payload.asset_ids.clone());
    }
    let found: Vec<String> = patched.iter().map(|entry| entry.id.clone()).collect();
    let missing: Vec<String> = payload.asset_ids.iter().filter(|id| !found.contains(id)).cloned().collect();
    let outcome = protocol::MutationOutcome::new(ShootingDiff { assets: Some(ShootingAssetsDelta { patched, ..Default::default() }), ..Default::default() });
    if missing.is_empty() {
        outcome
    } else {
        outcome.absorb_messages([protocol::MutationMessage::warn("mutation.partial", format!("{} of {} requested asset(s) did not exist and were skipped.", missing.len(), payload.asset_ids.len())).at(missing)])
    }
}
