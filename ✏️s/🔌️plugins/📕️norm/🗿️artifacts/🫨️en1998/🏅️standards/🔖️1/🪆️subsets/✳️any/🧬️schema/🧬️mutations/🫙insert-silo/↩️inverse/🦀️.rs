//! Inverse for `insert-silo`.
use super::InsertSilo;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_silo;

pub fn inverse(payload: &InsertSilo, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::RemoveSilo(remove_silo::RemoveSilo { index: payload.index.min(base.silos.len()) })]
}
