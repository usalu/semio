//! ↩️ Inverse for `change-foundation-base-friction-deg`.
use crate::change_foundation_base_friction_deg::ChangeFoundationBaseFrictionDeg;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeFoundationBaseFrictionDeg, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_foundation_base_friction_deg::ChangeFoundationBaseFrictionDeg> {
    Ok(match base.foundations.get(payload.index) {
        Some(row) => vec![Op::from(ChangeFoundationBaseFrictionDeg { index: payload.index, new_base_friction_angle_deg: row.base_friction_angle_deg })],
        None => Vec::new(),
    })
}
