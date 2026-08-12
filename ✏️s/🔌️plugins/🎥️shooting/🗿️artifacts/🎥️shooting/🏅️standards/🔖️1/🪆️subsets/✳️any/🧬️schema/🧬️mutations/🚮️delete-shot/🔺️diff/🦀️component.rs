//! 🔺 Diff constructor for `DeleteShot`.

use super::mutation::DeleteShot;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotsDelta, ShootingDiff};

pub fn diff(payload: &DeleteShot, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { shots: Some(ShootingShotsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() }
}
