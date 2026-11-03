//! ↩️ `change-joint-edge-distance` inverse: restores the previous `edge_distance`.

use crate::change_joint_edge_distance::ChangeJointEdgeDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointEdgeDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_edge_distance::ChangeJointEdgeDistance> {
    Ok(match base.joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointEdgeDistance { id: payload.id.clone(), new_edge_distance: row.edge_distance })],
        None => Vec::new(),
    })
}
