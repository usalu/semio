//! Inverse for `insert-bridge`.
use super::InsertBridge;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::remove_bridge;

pub fn inverse(payload: &InsertBridge, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1998Mutation::RemoveBridge(remove_bridge::RemoveBridge { index: payload.index.unwrap_or(usize::MAX).min(base.bridges.len()) })]

    })())
}
