//! Inverse for `remove-tower`.
use crate::remove_tower::RemoveTower;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_tower;

pub fn inverse<Op>(payload: &RemoveTower, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tower::InsertTower> {
    Ok((|| {
    match base.towers.get(payload.index) {
        Some(item) => vec![Op::from(insert_tower::InsertTower { index: Some(payload.index), tower: item.clone() })],
        None => Vec::new(),
    }

    })())
}
