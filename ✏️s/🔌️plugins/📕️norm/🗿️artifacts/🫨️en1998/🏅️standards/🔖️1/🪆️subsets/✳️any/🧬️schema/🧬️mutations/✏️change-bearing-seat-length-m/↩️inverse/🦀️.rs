//! ↩️ `change-bearing-seat-length-m` inverse: restores the previous `seat_length_m`.

use crate::change_bearing_seat_length_m::ChangeBearingSeatLengthM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeBearingSeatLengthM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bearing_seat_length_m::ChangeBearingSeatLengthM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeBearingSeatLengthM, base: &En1998Snapshot) -> Option<ChangeBearingSeatLengthM> {
    let r0 = base.bearings.iter().find(|row| row.id == payload.id)?;
    Some(ChangeBearingSeatLengthM { id: payload.id.clone(), new_seat_length_m: r0.seat_length_m })
}
