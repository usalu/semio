//! ↩️ `change-joint-beam-id` inverse: restores the previous `beam_id`.

use crate::change_joint_beam_id::ChangeJointBeamId;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeJointBeamId, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_beam_id::ChangeJointBeamId> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeJointBeamId, base: &En1998Snapshot) -> Option<ChangeJointBeamId> {
    let r0 = base.joints.iter().find(|row| row.id == payload.id)?;
    Some(ChangeJointBeamId { id: payload.id.clone(), new_beam_id: r0.beam_id.clone() })
}
