//! ↩️ Inverse for `change-storey-drift-x-m`.
use crate::change_storey_drift_xm::ChangeStoreyDriftXM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyDriftXM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_drift_xm::ChangeStoreyDriftXM> {
    Ok((|| {
    match base.buildings.get(payload.building_index).and_then(|b| b.storeys.get(payload.storey_index)) {
        Some(st) => vec![Op::from(ChangeStoreyDriftXM { building_index: payload.building_index, storey_index: payload.storey_index, new_drift_x_m: st.drift_x_m })],
        None => Vec::new(),
    }

    })())
}
