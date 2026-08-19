//! ↩️ `edit-paint-layer` — self-inverse: reads the pre-edit bytes at each run's offset from `base`
//! and re-emits an `edit-paint-layer` that writes them back (a stroke on a missing layer reads back
//! empty runs, matching the original write's own out-of-range no-op behavior).

use super::mutation::EditPaintLayer;
use crate::artifacts::lowpoly::mutations::PixelRun;
use crate::artifacts::lowpoly::{LowpolyMutation, LowpolySnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &EditPaintLayer, base: &LowpolySnapshot) -> Vec<LowpolyMutation> {
    let pixels = crate::artifacts::lowpoly::schema::layer_pixels_at(base, &payload.object_id, payload.layer_index);
    let inverse_runs = payload
        .runs
        .iter()
        .map(|run| {
            let start = run.offset as usize;
            let bytes = pixels
                .map(|buffer| {
                    let end = (start + run.bytes.len()).min(buffer.len());
                    if start < buffer.len() { buffer[start..end].to_vec() } else { Vec::new() }
                })
                .unwrap_or_default();
            PixelRun { offset: run.offset, bytes }
        })
        .collect();
    vec![LowpolyMutation::EditPaintLayer(EditPaintLayer { object_id: payload.object_id.clone(), layer_index: payload.layer_index, runs: inverse_runs })]
}
//#endregion 🔖️Inverse
