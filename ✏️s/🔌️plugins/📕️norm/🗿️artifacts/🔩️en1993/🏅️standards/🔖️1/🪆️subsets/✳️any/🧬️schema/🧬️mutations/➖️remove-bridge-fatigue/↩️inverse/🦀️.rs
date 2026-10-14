use crate::remove_bridge_fatigue::RemoveBridgeFatigue;
use crate::insert_bridge_fatigue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveBridgeFatigue, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bridge_fatigue::InsertBridgeFatigue> {
    Ok((|| {
    if payload.index >= base.bridge_fatigue.len() { return Vec::new(); }
    vec![Op::from(insert_bridge_fatigue::InsertBridgeFatigue { index: Some(payload.index), bridge_fatigue_item: base.bridge_fatigue[payload.index].clone() })]

    })())
}
