//! ↩️ `change-crane-wheel-contact-length` inverse: restores the previous `wheel_contact_length`.

use crate::change_crane_wheel_contact_length::ChangeCraneWheelContactLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneWheelContactLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_wheel_contact_length::ChangeCraneWheelContactLength> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneWheelContactLength { id: payload.id.clone(), new_wheel_contact_length: row.wheel_contact_length })],
        None => Vec::new(),
    })
}
