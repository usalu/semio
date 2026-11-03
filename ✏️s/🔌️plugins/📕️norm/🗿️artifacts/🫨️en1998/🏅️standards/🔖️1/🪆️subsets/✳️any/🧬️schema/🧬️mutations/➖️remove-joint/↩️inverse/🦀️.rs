//! ↩️ `remove-joint` inverse: inserts the removed row back at its position.

use crate::remove_joint::RemoveJoint;
use crate::insert_joint::InsertJoint;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveJoint, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_joint::InsertJoint> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveJoint, base: &En1998Snapshot) -> Option<InsertJoint> {
    let (index, row) = base.joints.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertJoint { index: Some(index), joint: row.clone() })
}
