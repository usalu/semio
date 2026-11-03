//! ↩️ `change-joint-plate-edge-distance` inverse: restores the previous `edge_distance`.

use crate::change_joint_plate_edge_distance::ChangeJointPlateEdgeDistance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeJointPlateEdgeDistance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_joint_plate_edge_distance::ChangeJointPlateEdgeDistance> {
    Ok(match base.joint_plates.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeJointPlateEdgeDistance { id: payload.id.clone(), new_edge_distance: row.edge_distance })],
        None => Vec::new(),
    })
}
