//! Inverse for `change-storey-stiffness-x`.
use crate::change_storey_stiffness_x::ChangeStoreyStiffnessX;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyStiffnessX, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_stiffness_x::ChangeStoreyStiffnessX> {
    Ok((|| {
    match base.buildings.get(payload.building_index).and_then(|b| b.storeys.get(payload.storey_index)) {
        Some(st) => vec![Op::from(ChangeStoreyStiffnessX { building_index: payload.building_index, storey_index: payload.storey_index, new_stiffness_x: st.stiffness_x })],
        None => Vec::new(),
    }

    })())
}
