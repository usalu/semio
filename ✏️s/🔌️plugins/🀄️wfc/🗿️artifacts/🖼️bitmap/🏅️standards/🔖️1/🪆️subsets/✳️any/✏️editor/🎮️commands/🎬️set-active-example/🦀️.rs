//! 🎬️ `setActiveExample` — loads one of this subset's committed examples into the open document.
//! The shell's navbar example picker and its automatic boot announcement both dispatch this verb, so
//! without it every bitmap playground boot dropped the action on the undeclared-action gate and the
//! picker was inert.
//!
//! The replay is a DECLARED-STATE DIFF, not a whole-document replace: `BitmapMutation` carries no
//! replace variant on purpose, and a document that already IS the requested example must answer an
//! EMPTY mutation set so the boot announcement and a re-selection write no edit at all
//! (`canUndo=false`).
//!
//! Mutation ORDER is the whole difficulty. `remove-palette-color` is fatal while the colour is still
//! painted or pinned, and `set-input-pixels` is fatal on an index the palette does not hold, so the
//! sequence is: release the pins that do not survive → GROW the palette → recolour the shared prefix
//! → resize the sample → rewrite the whole buffer → SHRINK the palette (now provably unused) → output
//! spec → model → seed → the example's own pins.

use crate::mutations::{add_palette_color, change_model, change_palette_color, change_seed, pin_pixel, remove_palette_color, resize_input, resize_output, set_input_pixels, unpin_pixel};
use crate::schema::snapshot::{encode_base64, BitmapSnapshot};
use crate::BitmapMutation;
use semio_framework_plugin::{ArtifactView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Registry
/// 🚪️ The example the editor boots on — the id an empty `exampleId` argument means.
pub const BITMAP_EXAMPLE_BOOT_ID: &str = crate::examples::rooms_16::ID;

/// 📚️ The committed snapshot behind one example id. An unknown id answers `None`, which the handler
/// turns into a no-op rather than a fault: the picker is a navigation affordance, not a destructive
/// verb.
pub fn example_snapshot(example_id: &str) -> Option<BitmapSnapshot> {
    match example_id {
        crate::examples::rooms_16::ID => Some(crate::examples::rooms_16::snapshot()),
        crate::examples::flowers_24::ID => Some(crate::examples::flowers_24::snapshot()),
        _ => None,
    }
}
//#endregion 🔖️Registry

//#region 🔖️Handler
/// 🎬️ Replaces the open document's declared state with the named example's.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, BitmapSnapshot>) -> Result<Emit<BitmapMutation>, Fault> {
    let example_id = match payload.example_id.trim() {
        "" => BITMAP_EXAMPLE_BOOT_ID,
        id => id,
    };
    let Some(next) = example_snapshot(example_id) else { return Ok(Emit::default()) };
    let mutations = replace_document_operations(doc.snapshot, &next);
    match mutations.is_empty() {
        true => Ok(Emit::default()),
        false => Ok(Emit { artifact_mutations: mutations, description: Some(format!("Load example {example_id}")), ..Default::default() }),
    }
}

/// 🔁️ The field-granular diff between the open document's declared state and the example's, in the
/// only order every leaf's own refusal law admits. A document that already equals the example answers
/// an EMPTY set.
pub fn replace_document_operations(current: &BitmapSnapshot, next: &BitmapSnapshot) -> Vec<BitmapMutation> {
    let mut mutations = Vec::new();
    for pin in &current.pinned {
        if !next.pinned.contains(pin) {
            mutations.push(unpin_pixel(pin.x, pin.y));
        }
    }
    for index in current.input.palette.len()..next.input.palette.len() {
        mutations.push(add_palette_color(index, next.input.palette[index]));
    }
    for index in 0..current.input.palette.len().min(next.input.palette.len()) {
        if current.input.palette[index] != next.input.palette[index] {
            mutations.push(change_palette_color(index, next.input.palette[index]));
        }
    }
    if current.input.width != next.input.width || current.input.height != next.input.height {
        mutations.push(resize_input(next.input.width, next.input.height));
    }
    if let Some(indices) = next.input.indices() {
        let resized = current.input.width == next.input.width && current.input.height == next.input.height;
        if !resized || current.input.pixels != next.input.pixels {
            mutations.push(set_input_pixels(0, 0, next.input.width, next.input.height, encode_base64(&indices)));
        }
    }
    for index in (next.input.palette.len()..current.input.palette.len()).rev() {
        mutations.push(remove_palette_color(index));
    }
    if current.output != next.output {
        mutations.push(resize_output(next.output.width, next.output.height, next.output.periodic));
    }
    if current.model != next.model {
        mutations.push(change_model(next.model.pattern_size, next.model.symmetry, next.model.periodic_input, next.model.ground));
    }
    if current.seed != next.seed {
        mutations.push(change_seed(next.seed));
    }
    for pin in &next.pinned {
        if !current.pinned.contains(pin) {
            mutations.push(pin_pixel(pin.x, pin.y, pin.color));
        }
    }
    mutations
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
