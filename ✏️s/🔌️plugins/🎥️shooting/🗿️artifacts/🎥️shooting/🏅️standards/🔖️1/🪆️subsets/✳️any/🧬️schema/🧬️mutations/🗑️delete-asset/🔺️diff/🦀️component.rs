//! 🔺 Diff constructor for `DeleteAsset`.

use super::mutation::DeleteAsset;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetsDelta, ShootingDiff};

pub fn diff(payload: &DeleteAsset, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { assets: Some(ShootingAssetsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
