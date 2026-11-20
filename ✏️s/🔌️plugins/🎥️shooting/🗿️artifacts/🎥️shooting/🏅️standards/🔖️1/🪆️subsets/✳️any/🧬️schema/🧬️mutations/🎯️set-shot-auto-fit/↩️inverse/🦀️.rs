//! ↩️ Restores the exact previous auto-fit override, including inheritance.

use super::SetShotAutoFit;
use crate::ShootingSnapshot;

pub fn inverse<Op: From<SetShotAutoFit>>(payload: &SetShotAutoFit, base: &ShootingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.shots.iter().find(|shot| shot.id == payload.id).map(|shot| {
        Op::from(SetShotAutoFit { id: payload.id.clone(), auto_fit: shot.auto_fit })
    }).into_iter().collect())
}
