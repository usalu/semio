//! Inverse for `remove-bridge`.
use super::RemoveBridge;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_bridge;

pub fn inverse(payload: &RemoveBridge, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.bridges.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertBridge(insert_bridge::InsertBridge { index: Some(payload.index), bridge: item.clone() })],
        None => Vec::new(),
    }

    })())
}
