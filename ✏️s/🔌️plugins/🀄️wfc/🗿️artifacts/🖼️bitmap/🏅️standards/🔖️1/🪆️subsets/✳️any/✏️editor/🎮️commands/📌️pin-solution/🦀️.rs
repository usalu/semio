//! 📌️ `pin-solution` — commits one finished solve into the document by pinning every output cell to
//! the colour the solve chose. The solve's pixels are inferred and never persisted on their own, so
//! pinning them is the one document edit that makes a solution durable and shared: every collaborator
//! sees the pins, and re-solving reproduces exactly this output.
//!
//! It is the commit action `s.wfc.bitmap.solve` declares (`BITMAP_INFERENCE_CONTRACT.commit`), so its
//! arguments are the solve's own output fields (`BitmapInferenceCommit`): `pixels` and
//! `contradiction`. Destructive by declaration, because it replaces every pin the document already
//! carries on a cell the solve coloured differently.

use crate::schema::snapshot::BitmapPinnedPixel;
use crate::schema::snapshot::BitmapSnapshot;
use crate::standards::v1::subsets::any::io::text::snapshot::{decode_base64};
use crate::BitmapMutation;
use semio_plugin::{ArtifactView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
pub const PIN_SOLUTION_ACTION_ID: &str = crate::inferences::BITMAP_INFERENCE_COMMIT_ACTION;

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
        false => Ok(Emit::mutations(mutations)),
    }
}

/// 🔁️ One semantic solution replacement over the complete row-major output, or no edit when every pin already matches.
pub fn solution_operations(snapshot: &BitmapSnapshot, payload: &PinSolution) -> Result<Vec<BitmapMutation>, Fault> {
    if payload.contradiction {
        return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.solution.contradiction"), "wfc.bitmap.solution.contradiction"));
    }
    let indices = decode_base64(&payload.pixels).ok_or_else(|| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.solution.pixels-invalid"), "wfc.bitmap.solution.pixels-invalid"))?;
    let (width, height) = (snapshot.output.width, snapshot.output.height);
    if indices.len() != (width as usize) * (height as usize) {
        return Err(Fault::from(format!("wfc-bitmap-pin-solution-extent:{}!={width}x{height}", indices.len())));
    }
    let palette = snapshot.input.palette.len();
    let mut pins = Vec::with_capacity(indices.len());
    let mut changed = snapshot.pinned.len() != indices.len();
    for y in 0..height {
        for x in 0..width {
            let color = u32::from(indices[(y as usize) * (width as usize) + (x as usize)]);
            if color as usize >= palette {
                return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.palette.unknown-color"), "wfc.bitmap.palette.unknown-color"));
            }
            let pin = BitmapPinnedPixel { x, y, color };
            changed |= snapshot.pinned.get(pins.len()) != Some(&pin);
            pins.push(pin);
        }
    }
    Ok(if changed { vec![crate::mutations::pin_solution(pins)] } else { Vec::new() })
}
//#endregion 🔖️Handler

//#region 🧪️Tests

//#endregion 🧪️Tests

/// 🪟️ Keeping a preview is an action of an addressed output pane.
pub fn require_preview_window(view: Option<&semio_plugin::ViewModel>, output_window_kind: &str) -> Result<(), Fault> {
    let view = view.ok_or_else(|| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.output.window-required"), "wfc.bitmap.output.window-required"))?;
    let id = view.window_id.as_deref().ok_or_else(|| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.output.window-required"), "wfc.bitmap.output.window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.window.stale"), "wfc.bitmap.window.stale"))?;
    if kind != output_window_kind {
        return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.output.window-kind-required"), "wfc.bitmap.output.window-kind-required"));
    }
    Ok(())
}

/// 🩺 Localized refusals shared by preview validation and the retained command route.
pub fn preview_fault(reason: &str) -> Fault {
    let code = format!("wfc.bitmap.preview.{reason}");
    Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(code.clone()), code)
}

/// 🧷️ Pins only a finished preview of this exact canonical source through the existing pin operations.
pub fn preview_operations(snapshot: &BitmapSnapshot, transient: &crate::editor::bitmap::transient::BitmapTransient, revision: &[u8; 32]) -> Result<Vec<BitmapMutation>, Fault> {
    if transient.contradiction {
        return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.solution.contradiction"), "wfc.bitmap.solution.contradiction"));
    }
    let pixels = preview_pixels(snapshot, transient, revision)?;
    solution_operations(snapshot, &PinSolution { pixels: pixels.to_string(), contradiction: false })
}

/// 🧮️ Quotes the full canonical pin loop to retained preflight before any pixel is materialized.
pub fn pin_work_items(snapshot: &BitmapSnapshot) -> Option<usize> {
    pin_cells(snapshot)?;
    semio_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(1).rows_for_items(1)
}

#[path = "🧵️cursor/🦀️.rs"]
mod cursor;
pub use cursor::BitmapPinCursor;

pub const BITMAP_PIN_WORK_UNITS_PER_TURN: usize = 16;
pub const BITMAP_MAXIMUM_OUTPUT_EDGE: usize = 512;
pub const BITMAP_MAXIMUM_OUTPUT_CELLS: usize = BITMAP_MAXIMUM_OUTPUT_EDGE * BITMAP_MAXIMUM_OUTPUT_EDGE;

/// 📐️ Output capacity follows BitmapOutputSpec in the authoritative snapshot schema.
pub fn pin_cells(snapshot: &BitmapSnapshot) -> Option<usize> {
    let width = usize::try_from(snapshot.output.width).ok()?;
    let height = usize::try_from(snapshot.output.height).ok()?;
    if width == 0 || height == 0 || width > BITMAP_MAXIMUM_OUTPUT_EDGE || height > BITMAP_MAXIMUM_OUTPUT_EDGE { return None; }
    width.checked_mul(height)
}

/// 🧾️ Compares a canonical encoded revision without allocating an intermediate buffer.
pub fn revision_matches(source: &str, revision: &[u8; 32]) -> bool {
    source.len() == 44 && (0..32).all(|index| cursor::pixel_at(source.as_bytes(), index, 32) == Some(revision[index]))
}

/// 🖼️ Borrows the exact completed cache only while its source and extent are current.
pub fn preview_pixels<'a>(snapshot: &BitmapSnapshot, transient: &'a crate::editor::bitmap::transient::BitmapTransient, revision: &[u8; 32]) -> Result<&'a str, Fault> {
    if transient.contradiction { return Err(Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("wfc.bitmap.solution.contradiction"), "wfc.bitmap.solution.contradiction")); }
    let pixels = transient.output_pixels.as_deref().filter(|pixels| !pixels.is_empty()).ok_or_else(|| preview_fault("unavailable"))?;
    if transient.output_width != snapshot.output.width || transient.output_height != snapshot.output.height { return Err(preview_fault("extent")); }
    if !revision_matches(&transient.source_revision, revision) { return Err(preview_fault("stale")); }
    Ok(pixels)
}

/// 📨️ A completion may replace a cache only under matching current command authority.
pub fn completion_matches(source: &str, revision: Option<&[u8;32]>) -> bool { revision.is_some_and(|revision| revision_matches(source, revision)) }
