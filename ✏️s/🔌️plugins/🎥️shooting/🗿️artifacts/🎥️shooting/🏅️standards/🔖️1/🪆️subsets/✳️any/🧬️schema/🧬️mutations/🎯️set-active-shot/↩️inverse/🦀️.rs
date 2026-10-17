//! ↩ Inverse constructor for `SetActiveShot` — reconstructed from BASE state.

use super::SetActiveShot;
use crate::ShootingSnapshot;

pub fn inverse<Op>(_payload: &SetActiveShot, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_active_shot::SetActiveShot>,
{
    Ok((|| {
    let shot_id = if base.active_shot_id.is_empty() { None } else { Some(base.active_shot_id.clone()) };
    vec![Op::from(SetActiveShot { shot_id })]

    })())
}
