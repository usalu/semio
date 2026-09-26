//! 🧬️ Bitmap artifact — semantic document mutation dispatch enum. Every variant is a single-field
//! tuple wrapping a handcrafted `protocol::MutationKind` payload, one per `🧬️mutations/<slug>/`
//! triad leaf wired by the crate root's own `#[path]` mount tree. `#[derive(dsl::Mutations)]`
//! generates `impl protocol::Mutation<BitmapSnapshot>` and `impl
//! protocol::SemanticMutation<BitmapSnapshot>` from those payloads — no hand-written apply/diff/
//! inverse dispatch here.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[mutations(snapshot = BitmapSnapshot, diff = BitmapDiff, schema = "wfcbitmap")]
pub enum BitmapMutation {
    ChangeSeed(super::change_seed::ChangeSeed),
    ResizeInput(super::resize_input::ResizeInput),
    SetInputPixels(super::set_input_pixels::SetInputPixels),
    AddPaletteColor(super::add_palette_color::AddPaletteColor),
    ChangePaletteColor(super::change_palette_color::ChangePaletteColor),
    RemovePaletteColor(super::remove_palette_color::RemovePaletteColor),
    ResizeOutput(super::resize_output::ResizeOutput),
    ChangeModel(super::change_model::ChangeModel),
    PinPixel(super::pin_pixel::PinPixel),
    UnpinPixel(super::unpin_pixel::UnpinPixel),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`BitmapMutation`] variant, in declaration order — the exact
/// vocabulary the `wfc-bitmap-1-any` mutation catalog (`../../🔮️oracles/🔣️.json`) declares and the
/// `🧩️mutate-bitmap-1` exhaustive case measures itself against. The framework never parses Rust, so
/// the unit test beside this file is what keeps the list honest against both.
pub const KINDS: &[&str] = &["change-seed", "resize-input", "set-input-pixels", "add-palette-color", "change-palette-color", "remove-palette-color", "resize-output", "change-model", "pin-pixel", "unpin-pixel"];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️Builders
pub use super::add_palette_color::add_palette_color;
pub use super::change_model::change_model;
pub use super::change_palette_color::change_palette_color;
pub use super::change_seed::change_seed;
pub use super::pin_pixel::pin_pixel;
pub use super::remove_palette_color::remove_palette_color;
pub use super::resize_input::resize_input;
pub use super::resize_output::resize_output;
pub use super::set_input_pixels::set_input_pixels;
pub use super::unpin_pixel::unpin_pixel;
//#endregion 🔖️Builders

pub type BitmapEnvelope = store::ArtifactEnvelope<BitmapSnapshot, BitmapMutation>;
pub type BitmapStore = store::ArtifactStore<BitmapSnapshot, BitmapMutation>;

/// 🧬️ Applies a mutation to a projection — generic over every variant.
pub fn apply_bitmap_mutation(projection: &mut BitmapSnapshot, mutation: &BitmapMutation) -> protocol::MutationApplyResult<()> {
    let (next, _) = vcs::apply_mutation(projection, mutation)?;

    *projection = next;
    Ok(())
}

/// ↩️ Computes a mutation's inverse against a projection — generic over every variant.
pub fn inverse_bitmap_mutation(projection: &BitmapSnapshot, mutation: &BitmapMutation) -> Vec<BitmapMutation> {
    mutation.inverse(projection)
}

//#region 🌉️TestBridge
/// 🌉️ The language-neutral report of one committed specification vector — decoded, diffed, applied and inverted
/// through this subset's production JSON codec and `Mutation` implementation — that the `mutate-bitmap` case's
/// subject half judges with `law::vector`. Its signature names only `str`, so a generated test host reaches it.
/// @see store::os_store::test_support::mutation_report_json
pub fn bitmap_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    store::os_store::test_support::mutation_report_json::<BitmapSnapshot, BitmapMutation>(base_json, mutation_json, after_json)
}

/// 🔁️ Decodes one snapshot through this subset's production JSON codec and re-encodes it — the subject half of the
/// case's `identity-round-trip` scenario.
pub fn bitmap_snapshot_json_round_trip(text: &str) -> Result<String, String> {
    let snapshot: BitmapSnapshot = dsl::json::from_json_str(text).map_err(|error| error.to_string())?;
    Ok(dsl::json::to_json_string(&snapshot))
}
//#endregion 🌉️TestBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
