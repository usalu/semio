//! ↩️ `change-member-design-temperature` inverse: restores the previous `design_temperature`.

use crate::change_member_design_temperature::ChangeMemberDesignTemperature;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeMemberDesignTemperature, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_member_design_temperature::ChangeMemberDesignTemperature> {
    Ok(match base.members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeMemberDesignTemperature { id: payload.id.clone(), new_design_temperature: row.design_temperature })],
        None => Vec::new(),
    })
}
