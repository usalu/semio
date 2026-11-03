//! ↩️ `insert-joint` inverse: removes the inserted row again.

use crate::insert_joint::InsertJoint;
use crate::remove_joint::RemoveJoint;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertJoint, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_joint::RemoveJoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertJoint, base: &En1998Snapshot) -> Option<RemoveJoint> {
    Some(RemoveJoint { id: payload.joint.id.clone() })
}
