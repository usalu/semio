//! ↩️ Inverse reconstruction for `delete-region` — reads the BASE item, never the diff.
use super::mutation::DeleteRegion;
use crate::artifacts::gismap::mutations::create_region::mutation::CreateRegion;
use crate::artifacts::gismap::mutations::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the feature at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()`, an empty inverse rather than a no-op sentinel mutation.
pub fn inverse(payload: &DeleteRegion, base: &GisMapSnapshot) -> Vec<GisMapMutation> {
    let Some(index) = base.regions.iter().position(|feature| feature.id == payload.id) else {
        return Vec::new();
    };
    vec![GisMapMutation::CreateRegion(CreateRegion { index, item: base.regions[index].clone() })]
}
//#endregion 🔹Inverse
