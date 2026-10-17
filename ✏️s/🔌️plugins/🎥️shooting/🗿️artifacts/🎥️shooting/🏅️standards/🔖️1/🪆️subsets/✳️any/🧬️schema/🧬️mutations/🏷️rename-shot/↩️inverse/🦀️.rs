//! ↩ Inverse constructor for `RenameShot` — reconstructed from BASE state.

use super::RenameShot;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &RenameShot, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_shot::RenameShot>,
{
    Ok((|| {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![Op::from(RenameShot { id: payload.id.clone(), new_label: shot.label.clone() })],
        None => Vec::new(),
    }

    })())
}
