//! ↩️ `change-profile-shaft-resistance` inverse: restores the previous `shaft_resistance`.

use crate::change_profile_shaft_resistance::ChangeProfileShaftResistance;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeProfileShaftResistance, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_profile_shaft_resistance::ChangeProfileShaftResistance> {
    Ok(match base.piles.iter().find(|row| row.id == payload.pile_id).and_then(|row| row.test_profiles.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeProfileShaftResistance { pile_id: payload.pile_id.clone(), id: payload.id.clone(), new_shaft_resistance: row.shaft_resistance })],
        None => Vec::new(),
    })
}
