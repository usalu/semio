//! ↩️ Restores a shot's previous drawing settings.

use super::SetShotDrawing;
use crate::ShootingSnapshot;

pub fn inverse<Op: From<SetShotDrawing>>(payload: &SetShotDrawing, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.shots.iter().find(|shot| shot.id == payload.id).map(|shot| {
        Op::from(SetShotDrawing { id: payload.id.clone(), patch: payload.patch.inverse(shot.drawing.as_ref()) })
    }).into_iter().collect())
}
