//! Inverse for `insert-tower`.
use super::InsertTower;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_tower;

pub fn inverse(payload: &InsertTower, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::RemoveTower(remove_tower::RemoveTower { index: payload.index.unwrap_or(usize::MAX).min(base.towers.len()) })]

    })())
}
