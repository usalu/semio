use crate::insert_bridge_fatigue::InsertBridgeFatigue;
use crate::remove_bridge_fatigue;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertBridgeFatigue, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_bridge_fatigue::RemoveBridgeFatigue> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.bridge_fatigue.len());
    vec![Op::from(remove_bridge_fatigue::RemoveBridgeFatigue { index: at })]

    })())
}
