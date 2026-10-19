//! ↩️ Inverse for `insert-tower`.
use crate::insert_tower::InsertTower;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_tower;

pub fn inverse<Op>(payload: &InsertTower, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tower::RemoveTower> {
    Ok((|| {
    vec![Op::from(remove_tower::RemoveTower { index: payload.index.unwrap_or(usize::MAX).min(base.towers.len()) })]

    })())
}
