//! 🔺 Diff constructor for `ReorderShots`. Error `target-missing` when absent, Warning `no-op`
//! when the resulting order is unchanged.

use super::mutation::ReorderShots;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotsDelta, ShootingDiff};

pub async fn diff(payload: &ReorderShots, base: &ShootingSnapshot) -> protocol::MutationOutcome<ShootingDiff> {
    if !base.shots.iter().any(|shot| shot.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shot \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let original: Vec<String> = base.shots.iter().map(|shot| shot.id.clone()).collect();
    let mut ids = original.clone();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    if ids == original {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Shot \"{}\" order is unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(ShootingDiff { shots: Some(ShootingShotsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() })
}
