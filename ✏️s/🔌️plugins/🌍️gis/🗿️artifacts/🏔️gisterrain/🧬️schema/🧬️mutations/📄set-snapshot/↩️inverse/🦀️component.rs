//! Inverse for `SetSnapshot`.
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use crate::artifacts::gisterrain::GisTerrainSnapshot;

//#region 🔹Inverse
pub fn inverse(base: &GisTerrainSnapshot, _replacement: &GisTerrainSnapshot) -> Vec<GisTerrainMutation> {
    vec![GisTerrainMutation::SetSnapshot { snapshot: base.clone() }]
}
//#endregion 🔹Inverse
