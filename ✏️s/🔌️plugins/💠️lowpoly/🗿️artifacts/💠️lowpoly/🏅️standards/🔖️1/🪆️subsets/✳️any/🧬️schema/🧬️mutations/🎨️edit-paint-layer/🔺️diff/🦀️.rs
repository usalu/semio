//! 🔺️ `edit-paint-layer` — sparse diff construction (delegates to the existing paint-stroke field-delta
//! constructor). Error `target-missing` when the object or layer index is absent, Warning `no-op`
//! when there are no pixel runs to paint.

use super::EditPaintLayer;
use semio_s_artifact_lowpoly_lowpoly_model_diff::diff_paint_stroke;
use semio_s_artifact_lowpoly_lowpoly_model_diff::LowpolyDiff;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Diff
pub fn diff(payload: &EditPaintLayer, base: &LowpolySnapshot) -> protocol::MutationOutcome<LowpolyDiff> {
    let Some(object) = base.objects.iter().find(|object| object.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object \"{}\" does not exist.", payload.object_id), [payload.object_id.clone()]);
    };
    if payload.layer_index >= object.paint_layers.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Paint layer {} does not exist on object \"{}\".", payload.layer_index, payload.object_id), [payload.object_id.clone()]);
    }
    if let Err(error)=object.paint_layers[payload.layer_index].validate_pixels(){return protocol::MutationOutcome::fatal("lowpoly.paint.invalid-texture",error.to_string(),[payload.object_id.clone()]);}
    if payload.runs.iter().any(|run|(run.offset as usize).checked_add(run.bytes.len()).is_none_or(|end|end>semio_s_artifact_lowpoly_lowpoly_model::paint_texture::LOWPOLY_PAINT_TEXTURE_BYTES)){return protocol::MutationOutcome::fatal("mutation.invariant","paint run exceeds canonical texture extent",[payload.object_id.clone()]);}
    if payload.runs.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("No pixel runs to paint on layer {} of object \"{}\".", payload.layer_index, payload.object_id));
    }
    let runs = payload.runs.clone();
    protocol::MutationOutcome::new(diff_paint_stroke(payload.object_id.clone(), payload.layer_index, runs))
}
//#endregion 🔖️Diff
