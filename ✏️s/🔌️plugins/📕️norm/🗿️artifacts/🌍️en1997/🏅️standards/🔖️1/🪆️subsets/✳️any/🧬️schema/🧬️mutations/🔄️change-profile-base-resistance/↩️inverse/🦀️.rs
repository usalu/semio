//! ↩️ `change-profile-base-resistance` inverse: restores the previous `base_resistance`.

use crate::change_profile_base_resistance::ChangeProfileBaseResistance;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeProfileBaseResistance, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_profile_base_resistance::ChangeProfileBaseResistance> {
    Ok(match base.piles.iter().find(|row| row.id == payload.pile_id).and_then(|row| row.test_profiles.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeProfileBaseResistance { pile_id: payload.pile_id.clone(), id: payload.id.clone(), new_base_resistance: row.base_resistance })],
        None => Vec::new(),
    })
}
