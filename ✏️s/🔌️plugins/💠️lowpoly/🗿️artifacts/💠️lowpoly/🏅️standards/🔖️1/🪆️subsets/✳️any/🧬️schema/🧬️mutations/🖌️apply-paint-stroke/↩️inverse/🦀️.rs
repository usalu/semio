//! ↩️ `apply-paint-stroke` — undo writes back the pixels the stroke overwrote: the stroke's own runs, computed against
//! `base`, inverted as ONE `edit-paint-layer` (point-invertible: one row however many dabs the stroke holds). A stroke
//! that cannot apply inverts to nothing.

use super::ApplyPaintStroke;
use crate::mutations::edit_paint_layer::EditPaintLayer;
use crate::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &ApplyPaintStroke, base: &LowpolySnapshot) -> Result<Vec<LowpolyMutation>, semio_framework_value::ValueError> {
    Ok({
    let Some(runs) = payload.runs(base).filter(|runs| !runs.is_empty() && payload.invariant_violation().is_none()) else {
        return Ok(Vec::new());
    };
    crate::mutations::edit_paint_layer::inverse::inverse(&EditPaintLayer { object_id: payload.object_id.clone(), layer_index: payload.layer_index, runs }, base)?

    })
}
//#endregion 🔖️Inverse
