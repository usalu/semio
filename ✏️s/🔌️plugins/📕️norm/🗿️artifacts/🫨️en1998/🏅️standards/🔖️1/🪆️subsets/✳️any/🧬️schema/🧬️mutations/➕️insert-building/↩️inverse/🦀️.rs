//! ↩️ Inverse for `insert-building`.
use crate::insert_building::InsertBuilding;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_building;

pub fn inverse<Op>(payload: &InsertBuilding, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_building::RemoveBuilding> {
    Ok((|| {
    vec![Op::from(remove_building::RemoveBuilding { index: payload.index.unwrap_or(usize::MAX).min(base.buildings.len()) })]

    })())
}
