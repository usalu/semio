//! ↩ Inverse constructor for `ChangeShotWidth` — reconstructed from BASE state.

use super::ChangeShotWidth;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ChangeShotWidth, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_shot_width::ChangeShotWidth>,
{
    Ok((|| {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![Op::from(ChangeShotWidth { id: payload.id.clone(), new_width: shot.width })],
        None => Vec::new(),
    }

    })())
}
