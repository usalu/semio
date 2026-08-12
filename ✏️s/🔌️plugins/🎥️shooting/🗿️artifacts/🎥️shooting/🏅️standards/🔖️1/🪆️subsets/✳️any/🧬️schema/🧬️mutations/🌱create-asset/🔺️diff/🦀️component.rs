//! 🔺 Diff constructor for `CreateAsset`.

use super::mutation::CreateAsset;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingAssetsDelta, ShootingDiff};

pub fn diff(payload: &CreateAsset, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { assets: Some(ShootingAssetsDelta { added: vec![payload.asset.clone()], ..Default::default() }), ..Default::default() }
}
