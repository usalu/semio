//! ↩️ Inverse reconstruction for `set-region-property` — reads the BASE payload, never the diff.
use super::SetRegionProperty;
use crate::mutations::{remove_region_property::RemoveRegionProperty, set_region_property::SetRegionProperty as SetProperty};
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the displaced value in place when the key existed, and removes the key when the set created it —
/// a missing target or a non-object payload returns `Vec::new()`.
pub fn inverse<Op>(payload: &SetRegionProperty, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_region_property::RemoveRegionProperty> + From<crate::standards::v1::subsets::any::schema::mutations::set_region_property::SetRegionProperty>,
{
    let Some(entries) = base.regions.iter().find(|feature| feature.id == payload.feature).and_then(|feature| feature.data.as_object()) else {
        return Ok(Vec::new());
    };
    Ok(match entries.iter().find(|(key, _)| *key == payload.key) {
        Some((_, previous)) if *previous == payload.value => Vec::new(),
        Some((_, previous)) => vec![Op::from(SetProperty { feature: payload.feature.clone(), key: payload.key.clone(), value: previous.clone(), before: None })],
        None => vec![Op::from(RemoveRegionProperty { feature: payload.feature.clone(), key: payload.key.clone() })],
    })
}
//#endregion 🔹Inverse
