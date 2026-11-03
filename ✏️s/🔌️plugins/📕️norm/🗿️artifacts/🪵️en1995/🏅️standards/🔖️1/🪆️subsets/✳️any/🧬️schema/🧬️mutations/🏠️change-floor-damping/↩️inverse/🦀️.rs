use crate::change_floor_damping::ChangeFloorDamping;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorDamping, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_damping::ChangeFloorDamping> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorDamping { floor_id: payload.floor_id.clone(), new_value: item.damping_ratio })],
        None => Vec::new(),
    })
}
