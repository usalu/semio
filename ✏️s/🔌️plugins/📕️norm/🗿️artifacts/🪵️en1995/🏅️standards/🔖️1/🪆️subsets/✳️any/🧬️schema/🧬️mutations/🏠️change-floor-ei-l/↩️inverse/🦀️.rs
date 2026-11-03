use crate::change_floor_ei_l::ChangeFloorEiL;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFloorEiL, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_floor_ei_l::ChangeFloorEiL> {
    Ok(match base.floors.iter().find(|item| item.id == payload.floor_id) {
        Some(item) => vec![Op::from(ChangeFloorEiL { floor_id: payload.floor_id.clone(), new_value: item.ei_l_n_m2_per_m })],
        None => Vec::new(),
    })
}
