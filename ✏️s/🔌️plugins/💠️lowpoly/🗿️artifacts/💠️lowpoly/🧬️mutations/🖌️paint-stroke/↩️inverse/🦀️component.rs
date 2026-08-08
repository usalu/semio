//! ↩️ Inverse for `PaintStroke`.
use crate::artifacts::lowpoly::mutations::{LowpolyMutation, PixelRun};
use crate::artifacts::lowpoly::LowpolyProjection;

//#region 🔖️Inverse
pub fn inverse(base: &LowpolyProjection, object_id: &str, layer_index: usize, runs: &[PixelRun]) -> Vec<LowpolyMutation> {
    let pixels = crate::artifacts::lowpoly::engine::layer_pixels_at(base, object_id, layer_index);
    let inverse_runs = runs
        .iter()
        .map(|run| {
            let start = run.offset as usize;
            let bytes = pixels
                .map(|buffer| {
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
    vec![LowpolyMutation::PaintStroke { object_id: object_id.to_string(), layer_index, runs: inverse_runs }]
}
//#endregion 🔖️Inverse
