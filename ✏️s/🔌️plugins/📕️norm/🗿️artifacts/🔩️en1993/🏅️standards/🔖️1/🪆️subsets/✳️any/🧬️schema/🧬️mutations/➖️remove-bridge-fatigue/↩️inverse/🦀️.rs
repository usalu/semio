use super::RemoveBridgeFatigue;
use crate::mutations::{insert_bridge_fatigue, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveBridgeFatigue, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if payload.index >= base.bridge_fatigue.len() { return Vec::new(); }
    vec![En1993Mutation::InsertBridgeFatigue(insert_bridge_fatigue::InsertBridgeFatigue { index: Some(payload.index), bridge_fatigue_item: base.bridge_fatigue[payload.index].clone() })]

    })())
}
