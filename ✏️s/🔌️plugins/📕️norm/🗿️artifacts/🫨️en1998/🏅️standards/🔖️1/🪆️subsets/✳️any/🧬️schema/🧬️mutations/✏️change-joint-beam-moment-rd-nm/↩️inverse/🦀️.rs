//! ↩️ `change-joint-beam-moment-rd-nm` inverse: restores the previous `beam_moment_rd_nm`.

use crate::change_joint_beam_moment_rd_nm::ChangeJointBeamMomentRdNm;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamMomentRdNm, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_moment_rd_nm::ChangeJointBeamMomentRdNm> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointBeamMomentRdNm, base: &En1998Snapshot) -> Option<ChangeJointBeamMomentRdNm> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointBeamMomentRdNm { id: payload.id.clone(), new_beam_moment_rd_nm: r0.beam_moment_rd_nm })
}
