//! 🔺️ `apply-paint-stroke` — sparse diff construction: the stroke's dabs stamped onto the base layer and written as
//! pixel runs of one `stroke` paint edit, the edit `edit-paint-layer` writes too. Fatal `invariant` for a payload no
//! stroke can satisfy (non-positive radius, hardness or opacity outside `[0, 1]`, no dab, a dab off the texture) or a
//! layer buffer that is not a square RGBA texture; Error `target-missing` when the object or layer is absent; Warning
//! `no-op` when the stroke changes no pixel (an eraser over a clear layer).

use super::ApplyPaintStroke;
use crate::diff::diff_paint_stroke;
use crate::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ApplyPaintStroke, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    if let Some(reason) = payload.invariant_violation() {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, [payload.object_id.clone()]);
    }
    let Some(object) = base.objects.iter().find(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    if payload.layer_index >= object.paint_layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Paint layer {} does not exist on object \"{}\".", payload.layer_index, payload.object_id), [payload.object_id.clone()]);
    }
    let Some(runs) = payload.runs(base) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Paint layer {} of object \"{}\" is not a square RGBA texture.", payload.layer_index, payload.object_id), [payload.object_id.clone()]);
    };
    if runs.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The stroke changes no pixel of layer {} of object \"{}\".", payload.layer_index, payload.object_id));
    }
    protocol::MutationOutcome::new(diff_paint_stroke(payload.object_id.clone(), payload.layer_index, runs))
}
//#endregion 🔖️Diff
