//! ↩ Inverse constructor for `CreateShot` — reconstructed from BASE state.

use super::CreateShot;
use crate::mutations::ShootingMutation;
use crate::ShootingSnapshot;

pub fn inverse(payload: &CreateShot, _base: &ShootingSnapshot) -> Result<Vec<ShootingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![ShootingMutation::DeleteShot(crate::mutations::delete_shot::DeleteShot { id: payload.shot.id.clone() })]

    })())
}
