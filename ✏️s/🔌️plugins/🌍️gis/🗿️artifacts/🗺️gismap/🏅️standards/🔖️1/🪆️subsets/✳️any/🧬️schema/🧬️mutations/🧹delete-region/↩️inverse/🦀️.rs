//! ↩️ Inverse reconstruction for `delete-region` — reads the BASE item, never the diff.
use super::DeleteRegion;
use crate::mutations::create_region::CreateRegion;
use crate::mutations::GisMapMutation;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the feature at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()`, an empty inverse rather than a no-op sentinel mutation.
pub fn inverse(payload: &DeleteRegion, base: &GisMapSnapshot) -> Result<Vec<GisMapMutation>, semio_framework_value::ValueError> {
    let Some(index) = base.regions.iter().position(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![GisMapMutation::CreateRegion(CreateRegion { index, item: base.regions[index].clone() })])
}
//#endregion 🔹Inverse
