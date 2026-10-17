//! ↩️ Inverse reconstruction for `replace-region-data` — reads the BASE payload, never the diff.
use super::ReplaceRegionData;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base`'s prior `data` payload for this id — missing target returns `Vec::new()`.
pub fn inverse<Op>(payload: &ReplaceRegionData, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_region_data::ReplaceRegionData>,
{
    let Some(feature) = base.regions.iter().find(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(ReplaceRegionData { id: payload.id.clone(), new_data: feature.data.clone() })])
}
//#endregion 🔹Inverse
