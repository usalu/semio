//! ↩ Inverse constructor for `CreateShot` — reconstructed from BASE state.

use super::CreateShot;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &CreateShot, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_shot::DeleteShot>,
{
    Ok(match base.shots.iter().any(|entry| entry.id == payload.shot.id) {
        true => Vec::new(),
        false => vec![Op::from(crate::mutations::delete_shot::DeleteShot { id: payload.shot.id.clone() })],
    })
}
