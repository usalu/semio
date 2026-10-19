//! ↩️ `edit-paint-layer` — self-inverse: reads the pre-edit bytes at each run's offset from `base`
//! and re-emits an `edit-paint-layer` that writes them back (a stroke on a missing layer reads back
//! empty runs, matching the original write's own out-of-range no-op behavior).

use super::EditPaintLayer;
use semio_s_artifact_lowpoly_lowpoly_model::PixelRun;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::edit_paint_layer::EditPaintLayer>>(payload: &EditPaintLayer, base: &LowpolySnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let pixels = semio_s_artifact_lowpoly_lowpoly_model::layer_pixels_at(base, &payload.object_id, payload.layer_index);
    let inverse_runs = payload
        .runs
        .iter()
        .map(|run| {
            let start = run.offset as usize;
            let bytes = pixels
                .map(|buffer| {
                    if buffer.is_empty() {
                        return vec![255; run.bytes.len()];
                    }
                    let end = (start + run.bytes.len()).min(buffer.len());
                    if start < buffer.len() {
                        buffer[start..end].to_vec()
                    } else {
                        Vec::new()
                    }
                })
                .unwrap_or_default();
            PixelRun { offset: run.offset, bytes }
        })
        .collect();
    vec![Op::from(EditPaintLayer { object_id: payload.object_id.clone(), layer_index: payload.layer_index, runs: inverse_runs })]

    })())
}
//#endregion 🔖️Inverse
