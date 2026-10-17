//! ↩️ Inverse reconstruction for `replace-position-data` — reads the BASE payload, never the diff.
use super::ReplacePositionData;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base`'s prior `data` payload for this id — missing target returns `Vec::new()`.
pub fn inverse<Op>(payload: &ReplacePositionData, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::replace_position_data::ReplacePositionData>,
{
    let Some(feature) = base.positions.iter().find(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(ReplacePositionData { id: payload.id.clone(), new_data: feature.data.clone() })])
}
//#endregion 🔹Inverse
