//! 📌️ `pin-solution` — commits one finished solve into the document by pinning every output cell to
//! the colour the solve chose. The solve's pixels are inferred and never persisted on their own, so
//! pinning them is the one document edit that makes a solution durable and shared: every collaborator
//! sees the pins, and re-solving reproduces exactly this output.
//!
//! It is the commit action `s.wfc.bitmap.solve` declares (`BITMAP_INFERENCE_CONTRACT.commit`), so its
//! arguments are the solve's own output fields (`BitmapInferenceCommit`): `pixels` and
//! `contradiction`. Destructive by declaration, because it replaces every pin the document already
//! carries on a cell the solve coloured differently.

use crate::mutations::pin_pixel;
use crate::schema::snapshot::{decode_base64, pin_index, BitmapSnapshot};
use crate::BitmapMutation;
use semio_framework_plugin::{ArtifactView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
pub const PIN_SOLUTION_ACTION_ID: &str = crate::inferences::BITMAP_INFERENCE_COMMIT_ACTION;

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "pin-solution")]
pub struct PinSolution {
    pub pixels: String,
    pub contradiction: bool,
}
//#endregion 🔖️Payload

//#region 🔖️Handler
/// 📌️ Pins the solve's pixels into the open document. A contradiction, a buffer that is not exactly
/// one palette index per output cell, or an index the palette does not hold is REFUSED by name —
/// pinning a partial or foreign buffer would persist an output the solve never produced.
pub fn handle(payload: &PinSolution, doc: &ArtifactView<'_, BitmapSnapshot>) -> Result<Emit<BitmapMutation>, Fault> {
    let mutations = solution_operations(doc.snapshot, payload)?;
    match mutations.is_empty() {
        true => Ok(Emit::default()),
        false => Ok(Emit { artifact_mutations: mutations, description: Some(format!("Pin solution ({}×{})", doc.snapshot.output.width, doc.snapshot.output.height)), ..Default::default() }),
    }
}

/// 🔁️ One `pin-pixel` per output cell whose pin differs from the solve, in row-major order. A document
/// already pinned to exactly this solution answers an EMPTY set.
pub fn solution_operations(snapshot: &BitmapSnapshot, payload: &PinSolution) -> Result<Vec<BitmapMutation>, Fault> {
    if payload.contradiction {
        return Err(Fault::from("wfc-bitmap-pin-solution-contradiction"));
    }
    let indices = decode_base64(&payload.pixels).ok_or_else(|| Fault::from("wfc-bitmap-pin-solution-pixels-not-base64"))?;
    let (width, height) = (snapshot.output.width, snapshot.output.height);
    if indices.len() != (width as usize) * (height as usize) {
        return Err(Fault::from(format!("wfc-bitmap-pin-solution-extent:{}!={width}x{height}", indices.len())));
    }
    let palette = snapshot.input.palette.len();
    let mut mutations = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let color = u32::from(indices[(y as usize) * (width as usize) + (x as usize)]);
            if color as usize >= palette {
                return Err(Fault::from("wfc-bitmap-unknown-palette-color"));
            }
            if pin_index(snapshot, x, y).is_some_and(|at| snapshot.pinned[at].color == color) {
                continue;
            }
            mutations.push(pin_pixel(x, y, color));
        }
    }
    Ok(mutations)
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
