//! Inverse for `remove-tower`.
use super::RemoveTower;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_tower;

pub fn inverse(payload: &RemoveTower, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.towers.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertTower(insert_tower::InsertTower { index: payload.index, tower: item.clone() })],
        None => Vec::new(),
    }

    })())
}
