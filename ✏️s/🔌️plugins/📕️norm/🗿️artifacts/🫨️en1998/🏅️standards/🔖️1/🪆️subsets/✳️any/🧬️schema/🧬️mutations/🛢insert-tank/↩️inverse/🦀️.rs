//! Inverse for `insert-tank`.
use super::InsertTank;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_tank;

pub fn inverse(payload: &InsertTank, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::RemoveTank(remove_tank::RemoveTank { index: payload.index.min(base.tanks.len()) })]
}
