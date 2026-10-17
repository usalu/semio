//! ↩ Inverse constructor for `ChangeShotHeight` — reconstructed from BASE state.

use super::ChangeShotHeight;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ChangeShotHeight, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_shot_height::ChangeShotHeight>,
{
    Ok((|| {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![Op::from(ChangeShotHeight { id: payload.id.clone(), new_height: shot.height })],
        None => Vec::new(),
    }

    })())
}
