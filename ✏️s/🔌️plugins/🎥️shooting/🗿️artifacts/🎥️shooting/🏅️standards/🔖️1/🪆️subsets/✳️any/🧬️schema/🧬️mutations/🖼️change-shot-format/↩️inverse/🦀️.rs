//! ↩ Inverse constructor for `ChangeShotFormat` — reconstructed from BASE state.

use super::ChangeShotFormat;
use crate::ShootingSnapshot;

pub fn inverse<Op>(payload: &ChangeShotFormat, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_shot_format::ChangeShotFormat>,
{
    Ok((|| {
    match base.shots.iter().find(|shot| shot.id == payload.id) {
        Some(shot) => vec![Op::from(ChangeShotFormat { id: payload.id.clone(), new_format: shot.format.clone() })],
        None => Vec::new(),
    }

    })())
}
