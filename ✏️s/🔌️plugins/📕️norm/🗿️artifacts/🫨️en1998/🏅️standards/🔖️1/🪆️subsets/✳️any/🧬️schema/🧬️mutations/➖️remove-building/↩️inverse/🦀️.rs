//! Inverse for `remove-building`.
use crate::remove_building::RemoveBuilding;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_building;

pub fn inverse<Op>(payload: &RemoveBuilding, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_building::InsertBuilding> {
    Ok((|| {
    match base.buildings.get(payload.index) {
        Some(b) => vec![Op::from(insert_building::InsertBuilding { index: Some(payload.index), building: b.clone() })],
        None => Vec::new(),
    }

    })())
}
