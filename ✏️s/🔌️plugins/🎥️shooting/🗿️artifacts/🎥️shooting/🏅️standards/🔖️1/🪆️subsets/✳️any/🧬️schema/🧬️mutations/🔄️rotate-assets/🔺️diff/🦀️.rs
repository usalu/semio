//! 🔺 Diff constructor for `RotateAssets`. Fatal `invariant` when the axis-angle is non-finite,
//! Error `target-missing` when none of the addressed assets exist, Warning `partial` when some do
//! not.

use super::RotateAssets;
use crate::diff::ShootingDiff;
use crate::ShootingSnapshot;
use crate::{quat_from_axis_angle, quat_mul, ShootingAssetPatch, ShootingAssigned};

pub fn diff(payload: &RotateAssets, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if [payload.ax, payload.ay, payload.az, payload.angle].iter().any(|value| !value.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Rotation axis-angle must be finite, got ({}, {}, {}, {}).", payload.ax, payload.ay, payload.az, payload.angle), payload.asset_ids.clone());
    }
    let delta = quat_from_axis_angle(payload.ax, payload.ay, payload.az, payload.angle);
    let patched: Vec<(String, ShootingAssetPatch)> = base
        .assets
        .iter()
        .filter(|asset| payload.asset_ids.contains(&asset.id))
        .map(|asset| {
            let current = asset.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            (asset.id.clone(), ShootingAssetPatch { orientation: Some(ShootingAssigned::new(Some(quat_mul(delta, current)))), ..Default::default() })
        })
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
