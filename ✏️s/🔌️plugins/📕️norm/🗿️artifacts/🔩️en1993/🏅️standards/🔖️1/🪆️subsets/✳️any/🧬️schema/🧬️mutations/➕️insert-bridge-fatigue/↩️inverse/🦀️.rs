use super::InsertBridgeFatigue;
use crate::mutations::{remove_bridge_fatigue, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertBridgeFatigue, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.bridge_fatigue.len());
    vec![En1993Mutation::RemoveBridgeFatigue(remove_bridge_fatigue::RemoveBridgeFatigue { index: at })]

    })())
}
