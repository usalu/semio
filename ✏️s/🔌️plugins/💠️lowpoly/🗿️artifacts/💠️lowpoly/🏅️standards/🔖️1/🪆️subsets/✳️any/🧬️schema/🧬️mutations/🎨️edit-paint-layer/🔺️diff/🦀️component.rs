//! 🔺️ `edit-paint-layer` — sparse diff construction (delegates to the existing paint-stroke field-delta
//! constructor).

use super::mutation::EditPaintLayer;
use crate::artifacts::lowpoly::diff::diff_paint_stroke;
use crate::artifacts::lowpoly::diff::schema::PixelRun as SchemaPixelRun;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &EditPaintLayer, _base: &LowpolySnapshot) -> LowpolyDiff {
    let runs = payload.runs.iter().map(|run| SchemaPixelRun { offset: run.offset, bytes: run.bytes.clone() }).collect();
    diff_paint_stroke(payload.object_id.clone(), payload.layer_index, runs)
}
//#endregion 🔖️Diff
