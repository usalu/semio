//! ↩️ `change-fastened-joint-edge-distance` inverse: restores the previous `edge_distance`.

use crate::change_fastened_joint_edge_distance::ChangeFastenedJointEdgeDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFastenedJointEdgeDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastened_joint_edge_distance::ChangeFastenedJointEdgeDistance> {
    Ok(match base.fastened_joints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFastenedJointEdgeDistance { id: payload.id.clone(), new_edge_distance: row.edge_distance })],
        None => Vec::new(),
    })
}
