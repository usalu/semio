//! 🔺 Diff constructor for `ReorderShots`.

use super::mutation::ReorderShots;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotsDelta, ShootingDiff};

pub fn diff(payload: &ReorderShots, base: &ShootingSnapshot) -> ShootingDiff {
    let mut ids: Vec<String> = base.shots.iter().map(|shot| shot.id.clone()).collect();
    if let Some(from) = ids.iter().position(|id| id == &payload.id) {
        let item = ids.remove(from);
        let to = payload.to_index.min(ids.len());
        ids.insert(to, item);
    }
    ShootingDiff { shots: Some(ShootingShotsDelta { reordered: Some(ids), ..Default::default() }), ..Default::default() }
}
