//! 🔺 Diff constructor for `CreateShot`.

use super::mutation::CreateShot;
use crate::artifacts::shooting::ShootingSnapshot;
use crate::artifacts::shooting::diff::{ShootingShotsDelta, ShootingDiff};

pub fn diff(payload: &CreateShot, base: &ShootingSnapshot) -> ShootingDiff {
    ShootingDiff { shots: Some(ShootingShotsDelta { added: vec![payload.shot.clone()], ..Default::default() }), ..Default::default() }
}
