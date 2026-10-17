//! ↩️ Inverse reconstruction for `remove-region-property` — reads the BASE payload, never the diff.
use super::RemoveRegionProperty;
use crate::mutations::set_region_property::SetRegionProperty;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo sets the removed property back with its BASE value, inserted before the key that followed it so the payload's
/// key order survives (appended when it was last) — a missing target, non-object payload or absent key returns `Vec::new()`.
pub fn inverse<Op>(payload: &RemoveRegionProperty, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_region_property::SetRegionProperty>,
{
    let Some(entries) = base.regions.iter().find(|feature| feature.id == payload.feature).and_then(|feature| feature.data.as_object()) else {
        return Ok(Vec::new());
    };
    let Some(at) = entries.iter().position(|(key, _)| *key == payload.key) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(SetRegionProperty { feature: payload.feature.clone(), key: payload.key.clone(), value: entries[at].1.clone(), before: entries.get(at + 1).map(|(key, _)| key.clone()) })])
}
//#endregion 🔹Inverse
