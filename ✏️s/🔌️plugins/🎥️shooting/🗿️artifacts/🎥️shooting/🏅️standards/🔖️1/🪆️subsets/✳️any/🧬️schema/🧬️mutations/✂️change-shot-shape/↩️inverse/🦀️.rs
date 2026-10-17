//! ↩ Inverse constructor for `ChangeShotShape` — reconstructed from BASE state.

use super::ChangeShotShape;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ChangeShotShape, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_shot_shape::ChangeShotShape>,
{
    Ok((|| {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![Op::from(ChangeShotShape { id: payload.id.clone(), new_shape: shot.shape.clone() })],
        None => Vec::new(),
    }

    })())
}
