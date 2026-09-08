//! 🧬️ presentation artifact — document mutation dispatch.

use crate::diff::PresentationDiff;
use crate::PresentationSnapshot;
use protocol::Mutation;

//#region 🔖️MutationLeaves
// 🧵️ Each `🧬️mutations/<kind>/` triad leaf (🦠️mutation/🔺️diff/↩️inverse) is `#[path]`-mounted as a
// sibling module of this dispatch file directly in the plugin's `🦀️.rs` (this facet's fan-out
// ticket, SEMANTIC-MUTATIONS-OVERHAUL wave-C, owns `🦀️.rs` for this plugin), matching the shape
// sibling plugins (`gis`, `cad`, `fem`) use. `use super::<kind>;` below just brings each sibling
// into this file's scope so the enum body can reference `<kind>::mutation::<Type>`.
use super::create_tile;
use super::delete_tile;
use super::delete_tiles;
use super::rename_tile;
use super::reorder_tiles;
use super::replace_source;
use super::replace_tiles;
use super::resize_source_frame;
use super::resize_tile_crop;
//#endregion 🔖️MutationLeaves

//#region 🔖️Mutations
/// 🎬️ Typed, invertible, semantic presentation-deck mutation vocabulary — every variant wraps exactly
/// one `protocol::MutationKind` payload struct declared in its own `🧬️mutations/<kind>/🦠️mutation`
/// triad leaf; `#[derive(dsl::Mutations)]` wires `Mutation`/`SemanticMutation` from those leaves.
/// `source` (a singleton facet) gets `replace-source`/`resize-source-frame`; `tiles` (an id-keyed
/// ordered collection) gets `create`/`delete`/`delete-tiles`/`rename`/`resize-tile-crop`/`reorder`/
/// `replace-tiles` per `derivation-rules.md`'s per-id-keyed-collection recipe. Replaces the former
/// generic whole-collection `Tiles(...)`/`SetSource`/`SetTiles`/whole-document-replacement
/// vocabulary — whole-document replacement is not expressible as an in-history mutation at all
/// (goes through `ArtifactStore::reset`, an app-level concern outside this enum).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslEnum, dsl::Mutations)]
#[mutations(snapshot = PresentationSnapshot, diff = PresentationDiff, schema = "animate.presentation")]
pub enum PresentationMutation {
    ResizeSourceFrame(resize_source_frame::ResizeSourceFrame),
    ReplaceSource(replace_source::ReplaceSource),
    CreateTile(create_tile::CreateTile),
    DeleteTile(delete_tile::DeleteTile),
    DeleteTiles(delete_tiles::DeleteTiles),
    RenameTile(rename_tile::RenameTile),
    ResizeTileCrop(resize_tile_crop::ResizeTileCrop),
    ReorderTiles(reorder_tiles::ReorderTiles),
    ReplaceTiles(replace_tiles::ReplaceTiles),
}

/// 🏷️ The kebab spelling of every [`PresentationMutation`] variant, in DECLARATION ORDER — the one list
/// the language-neutral test platform is measured against. It is duplicated in exactly two other
/// places on purpose: this subset's own oracle manifest catalog `presentation-1-any`
/// (`../../🔣️oracle.json`), which the completeness gate counts, and the
/// `🧭️mutate-presentation-1` case adapter, which must not link this crate in the oracle role.
/// [`tests::kinds_match_the_enum_and_the_catalog`] is what keeps all three honest.
pub const KINDS: &[&str] = &["resize-source-frame", "replace-source", "create-tile", "delete-tile", "delete-tiles", "rename-tile", "resize-tile-crop", "reorder-tiles", "replace-tiles"];
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[allow(clippy::items_after_test_module)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Apply
/// 📦️ Applies `mutation` onto `snapshot`, returning the resulting snapshot.
pub fn apply_presentation_mutation(snapshot: &PresentationSnapshot, mutation: &PresentationMutation) -> protocol::MutationApplyResult<PresentationSnapshot> {
    vcs::apply_mutation(snapshot, mutation).map(|(next, _messages)| next)
}

/// ↩️ Computes `mutation`'s inverse mutations against `snapshot` (pre-state).
pub fn inverse_presentation_mutation(snapshot: &PresentationSnapshot, mutation: &PresentationMutation) -> Vec<PresentationMutation> {
    mutation.inverse(snapshot)
}

/// 📥️ Decodes this facet's own externally-tagged (`{"CreateTile": { … }}`) JSON projection — the
/// shape the `🧭️mutate-presentation-1` case's `Examples` rows carry — into a real [`PresentationMutation`]. A
/// thin first-party `dsl::os_pack::json` wrapper, so the test adapter reads the committed feature row
/// instead of re-declaring it as a Rust literal beside it.
pub fn decode_presentation_mutation_json(text: &str) -> Result<PresentationMutation, String> {
    let json = dsl::os_pack::json::parse(text).map_err(|error| error.to_string())?;
    let value = dsl::os_pack::json::to_dsl_value(&json);
    dsl::FromValue::from_value(value).map_err(|error| error.to_string())
}

/// ⚖️ The SEMANTIC PROJECTION this subset is compared through — `(schema, source, tiles)` read back
/// off the composed presentation child's working scene. It belongs to the subset rather than to a
/// test adapter, because what counts as this document's meaning is this subset's ruling, not a
/// case's. The two child handles are deliberately absent: `presentation_child_handle`
/// content-addresses exactly this `(source, tiles)` pair through `std`'s deliberately unspecified
/// `DefaultHasher`, so projecting one would compare the same content twice and pin a value the
/// standard library does not promise. `animation` carries no content at all today.
pub fn encode_presentation_projection_json(snapshot: &PresentationSnapshot) -> String {
    let (source, tiles) = crate::presentation_working_scene(snapshot);
    let source_json = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&source));
    let tiles_json = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(&tiles));
    let value = dsl::os_pack::json::object([
        ("schema".to_string(), dsl::os_pack::json::Value::from(snapshot.schema.clone())),
        ("source".to_string(), source_json),
        ("tiles".to_string(), tiles_json),
    ]);
    dsl::os_pack::json::to_string(&value)
}
//#endregion 🔖️Apply
