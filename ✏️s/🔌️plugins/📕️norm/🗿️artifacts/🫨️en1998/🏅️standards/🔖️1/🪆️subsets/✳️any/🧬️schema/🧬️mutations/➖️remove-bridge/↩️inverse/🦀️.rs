//! Inverse for `remove-bridge`.
use super::RemoveBridge;
use crate::{En1998Mutation, En1998Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::insert_bridge;

pub fn inverse(payload: &RemoveBridge, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    match base.bridges.get(payload.index) {
        Some(item) => vec![En1998Mutation::InsertBridge(insert_bridge::InsertBridge { index: payload.index, bridge: item.clone() })],
        None => Vec::new(),
    }
}
