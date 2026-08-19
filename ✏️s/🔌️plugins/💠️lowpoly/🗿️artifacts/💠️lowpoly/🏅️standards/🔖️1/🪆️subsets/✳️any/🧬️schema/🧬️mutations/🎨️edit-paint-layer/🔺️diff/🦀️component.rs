//! 🔺️ `edit-paint-layer` — sparse diff construction (delegates to the existing paint-stroke field-delta
//! constructor). Error `target-missing` when the object or layer index is absent, Warning `no-op`
//! when there are no pixel runs to paint.

use super::mutation::EditPaintLayer;
use crate::artifacts::lowpoly::diff::diff_paint_stroke;
use crate::artifacts::lowpoly::diff::schema::PixelRun as SchemaPixelRun;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolySnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &EditPaintLayer, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    if payload.layer_index >= object.paint_layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Paint layer {} does not exist on object \"{}\".", payload.layer_index, payload.object_id), [payload.object_id.clone()]);
    }
    if payload.runs.is_empty() {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("No pixel runs to paint on layer {} of object \"{}\".", payload.layer_index, payload.object_id));
    }
    let runs = payload.runs.iter().map(|run| SchemaPixelRun { offset: run.offset, bytes: run.bytes.clone() }).collect();
    protocol::MutationOutcome::new(diff_paint_stroke(payload.object_id.clone(), payload.layer_index, runs))
}
//#endregion 🔖️Diff
