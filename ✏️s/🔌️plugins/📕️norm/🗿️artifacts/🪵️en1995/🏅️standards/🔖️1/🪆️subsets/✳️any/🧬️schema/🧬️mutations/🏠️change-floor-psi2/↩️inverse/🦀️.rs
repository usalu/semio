use crate::change_floor_psi_2::ChangeFloorPsi2;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorPsi2, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_psi_2::ChangeFloorPsi2> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorPsi2 { floor_id: payload.floor_id.clone(), new_value: item.psi_2 })],
        None => Vec::new(),
    })
}
