//! Inverse for `change-storey-permanent-gk-n`.
use crate::change_storey_permanent_gk_n::ChangeStoreyPermanentGkN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeStoreyPermanentGkN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_storey_permanent_gk_n::ChangeStoreyPermanentGkN> {
    Ok((|| {
    match base.buildings.get(payload.building_index).and_then(|b| b.storeys.get(payload.storey_index)) {
        Some(st) => vec![Op::from(ChangeStoreyPermanentGkN {
            building_index: payload.building_index,
            storey_index: payload.storey_index,
            new_permanent_gk_n: st.permanent_gk_n,
        })],
        None => Vec::new(),
    }

    })())
}
