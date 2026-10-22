//! 🧬️ Bitmap artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload, one per `🧬️mutations/<slug>/`
//! triad leaf wired by the crate root's own `#[path]` mount tree. `#[derive(dsl::Mutations)]`
//! generates `impl protocol::Mutation<BitmapSnapshot>` and `impl
//! protocol::SemanticMutation<BitmapSnapshot>` from those payloads — no hand-written apply/diff/
//! inverse dispatch here.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::{ColdMutation, Mutation};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = BitmapSnapshot, diff = BitmapDiff, schema = "wfcbitmap", reference = BitmapSnapshot, reference_diff = BitmapDiff)]
pub enum BitmapMutation {
    ChangeSeed(crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed),
    ResizeInput(crate::standards::v1::subsets::any::schema::mutations::resize_input::ResizeInput),
    SetInputPixels(crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::SetInputPixels),
    AddPaletteColor(crate::standards::v1::subsets::any::schema::mutations::add_palette_color::AddPaletteColor),
    ChangePaletteColor(crate::standards::v1::subsets::any::schema::mutations::change_palette_color::ChangePaletteColor),
    RemovePaletteColor(crate::standards::v1::subsets::any::schema::mutations::remove_palette_color::RemovePaletteColor),
    ResizeOutput(crate::standards::v1::subsets::any::schema::mutations::resize_output::ResizeOutput),
    ChangeModel(crate::standards::v1::subsets::any::schema::mutations::change_model::ChangeModel),
    PinPixel(crate::standards::v1::subsets::any::schema::mutations::pin_pixel::PinPixel),
    UnpinPixel(crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::UnpinPixel),
    PaintInputStroke(crate::standards::v1::subsets::any::schema::mutations::paint_input_stroke::PaintInputStroke),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`BitmapMutation`] variant, in declaration order — the exact
/// vocabulary the `wfc-bitmap-1-any` mutation catalog (`../../🔮️oracles/🔣️.json`) declares and the
/// `🧩️mutate-bitmap-1` exhaustive case measures itself against. The framework never parses Rust, so
/// the unit test beside this file is what keeps the list honest against both.
pub const KINDS: &[&str] = &["change-seed", "resize-input", "set-input-pixels", "add-palette-color", "change-palette-color", "remove-palette-color", "resize-output", "change-model", "pin-pixel", "unpin-pixel", "paint-input-stroke"];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use crate::standards::v1::subsets::any::schema::mutations::add_palette_color::add_palette_color;
pub use crate::standards::v1::subsets::any::schema::mutations::change_palette_color::change_palette_color;
pub use crate::standards::v1::subsets::any::schema::mutations::paint_input_stroke::{stroke_cells, stroke_extent, BitmapStrokePoint, BITMAP_STROKE_MAXIMUM_POINTS};
//#endregion 🔖️Builders

pub type BitmapEnvelope = store::os_store::store_schema::ArtifactEnvelope<BitmapSnapshot, BitmapMutation>;
pub type BitmapStore = store::ArtifactStore<BitmapSnapshot, BitmapMutation>;

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_bitmap_mutation(projection: &BitmapSnapshot, mutation: &BitmapMutation) -> Result<Vec<BitmapMutation>, semio_framework_value::ValueError> {
    Ok({
    mutation.inverse(projection)?

    })
}

//#region 🌉️TestBridge



//#endregion 🌉️TestBridge

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️ConcreteBuilders
/// 🏗️ Builder of the `change_model` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_model(pattern_size: u32, symmetry: u32, periodic_input: bool, ground: Option<u32>) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_model::change_model::<BitmapMutation>(pattern_size, symmetry, periodic_input, ground)
}

/// 🏗️ Builder of the `change_seed` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn change_seed(seed: u64) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed::<BitmapMutation>(seed)
}

/// 🏗️ Builder of the `paint_input_stroke` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn paint_input_stroke(points: Vec<BitmapStrokePoint>, color: u32) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::paint_input_stroke::paint_input_stroke::<BitmapMutation>(points, color)
}

/// 🏗️ Builder of the `pin_pixel` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn pin_pixel(x: u32, y: u32, color: u32) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::pin_pixel::pin_pixel::<BitmapMutation>(x, y, color)
}

/// 🏗️ Builder of the `remove_palette_color` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn remove_palette_color(index: usize) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::remove_palette_color::remove_palette_color::<BitmapMutation>(index)
}

/// 🏗️ Builder of the `resize_input` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn resize_input(width: u32, height: u32) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::resize_input::resize_input::<BitmapMutation>(width, height)
}

/// 🏗️ Builder of the `resize_output` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn resize_output(width: u32, height: u32, periodic: bool) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::resize_output::resize_output::<BitmapMutation>(width, height, periodic)
}

/// 🏗️ Builder of the `set_input_pixels` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn set_input_pixels(x: u32, y: u32, width: u32, height: u32, pixels: Vec<u8>) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::set_input_pixels::<BitmapMutation>(x, y, width, height, pixels)
}

/// 🏗️ Builder of the `unpin_pixel` leaf for this aggregate; the leaf builder stays generic over its aggregate.
pub fn unpin_pixel(x: u32, y: u32) -> BitmapMutation {
    crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::unpin_pixel::<BitmapMutation>(x, y)
}
//#endregion 🔖️ConcreteBuilders
