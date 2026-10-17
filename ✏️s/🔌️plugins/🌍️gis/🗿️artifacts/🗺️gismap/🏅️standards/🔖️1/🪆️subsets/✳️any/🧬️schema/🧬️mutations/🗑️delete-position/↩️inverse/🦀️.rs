//! ↩️ Inverse reconstruction for `delete-position` — reads the BASE item, never the diff.
use super::DeletePosition;
use crate::mutations::create_position::CreatePosition;
use crate::GisMapSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the feature at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()`, an empty inverse rather than a no-op sentinel mutation.
pub fn inverse<Op>(payload: &DeletePosition, base: &GisMapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_position::CreatePosition>,
{
    let Some(index) = base.positions.iter().position(|feature| feature.id == payload.id) else {
        return Ok(Vec::new());
    };
    Ok(vec![Op::from(CreatePosition { index, item: base.positions[index].clone() })])
}
//#endregion 🔹Inverse
