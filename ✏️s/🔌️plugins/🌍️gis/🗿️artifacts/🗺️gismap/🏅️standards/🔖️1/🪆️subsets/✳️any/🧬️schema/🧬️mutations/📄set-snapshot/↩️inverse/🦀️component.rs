//! Inverse for `SetSnapshot`.
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
pub fn inverse(base: &GisMapSnapshot, _replacement: &GisMapSnapshot) -> Vec<GisMapMutation> {
    vec![GisMapMutation::SetSnapshot { snapshot: base.clone() }]
}
//#endregion 🔹Inverse
