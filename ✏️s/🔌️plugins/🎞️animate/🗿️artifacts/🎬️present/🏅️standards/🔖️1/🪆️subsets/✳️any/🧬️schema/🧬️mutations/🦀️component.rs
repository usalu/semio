//! 🧬️ present artifact — document mutation dispatch.

use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️MutationLeaves
// 🧵️ Each `🧬️mutations/<kind>/` triad leaf (🦠️mutation/🔺️diff/↩️inverse) is `#[path]`-mounted as a
// sibling module of this dispatch file directly in the plugin's `📦️glue.rs` (this facet's fan-out
// ticket, SEMANTIC-MUTATIONS-OVERHAUL wave-C, owns `📦️glue.rs` for this plugin), matching the shape
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
/// 🎬️ Typed, invertible, semantic present-deck mutation vocabulary — every variant wraps exactly
/// one `protocol::MutationKind` payload struct declared in its own `🧬️mutations/<kind>/🦠️mutation`
/// triad leaf; `#[derive(dsl::Mutations)]` wires `Mutation`/`SemanticMutation` from those leaves.
/// `source` (a singleton facet) gets `replace-source`/`resize-source-frame`; `tiles` (an id-keyed
/// ordered collection) gets `create`/`delete`/`delete-tiles`/`rename`/`resize-tile-crop`/`reorder`/
/// `replace-tiles` per `derivation-rules.md`'s per-id-keyed-collection recipe. Replaces the former
/// generic whole-collection `Tiles(...)`/`SetSource`/`SetTiles`/whole-document-replacement
/// vocabulary — whole-document replacement is not expressible as an in-history mutation at all
/// (goes through `ArtifactStore::reset`, an app-level concern outside this enum).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[mutations(snapshot = PresentSnapshot, diff = PresentDiff, schema = "animate.present")]
pub enum PresentMutation {
    ResizeSourceFrame(resize_source_frame::mutation::ResizeSourceFrame),
    ReplaceSource(replace_source::mutation::ReplaceSource),
    CreateTile(create_tile::mutation::CreateTile),
    DeleteTile(delete_tile::mutation::DeleteTile),
    DeleteTiles(delete_tiles::mutation::DeleteTiles),
    RenameTile(rename_tile::mutation::RenameTile),
    ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop),
    ReorderTiles(reorder_tiles::mutation::ReorderTiles),
    ReplaceTiles(replace_tiles::mutation::ReplaceTiles),
}

/// 🏷️ The kebab spelling of every [`PresentMutation`] variant, in DECLARATION ORDER — the one list
/// the language-neutral test platform is measured against. It is duplicated in exactly two other
/// places on purpose: this subset's own oracle manifest catalog `present-1-any`
/// (`../../🧪️oracle/🔣️component.json`), which the completeness gate counts, and the
/// `mutate-present-1` case adapter, which must not link this crate in the oracle role.
/// [`tests::kinds_match_the_enum_and_the_catalog`] is what keeps all three honest.
pub const KINDS: &[&str] = &["resize-source-frame", "replace-source", "create-tile", "delete-tile", "delete-tiles", "rename-tile", "resize-tile-crop", "reorder-tiles", "replace-tiles"];
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod tests {
    use super::*;
    use crate::artifacts::present::{default_present_snapshot, present_snapshot_with_tiles, present_working_scene, FigureTileDraft, FigureTileFrame};
    use protocol::os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error};
    use protocol::SemanticMutation;

    fn tile(id: &str) -> FigureTileDraft {
        FigureTileDraft { id: id.into(), name: id.into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }
    }

    async fn round_trip(base: &PresentSnapshot, mutation: &PresentMutation) -> PresentSnapshot {
        let (forward, _messages) = vcs::apply_mutation(base, mutation).await.expect("valid mutation");
        let mut backward = mutation.inverse(base);
        backward.reverse();
        let mut restored = forward.clone();
        for undo in &backward {
            let (next, _messages) = vcs::apply_mutation(&restored, undo).await.expect("valid inverse mutation");
            restored = next;
        }
        // 🔒️ Structural equality, not just working-scene equality: `presentation_child_handle_and_cache`
        // content-addresses deterministically off `(source, tiles)`, so restoring the exact pre-mutation
        // working-scene content also restores the exact pre-mutation child handle byte-for-byte.
        assert_eq!(&restored, base, "inverse (reversed) must exactly restore the pre-mutation snapshot");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn tiles_create_rename_resize_delete_round_trip() {
        let base = default_present_snapshot();
        let created = round_trip(&base, &PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile: tile("t1") })).await;
        assert_eq!(present_working_scene(&created).1.len(), 1);
        let renamed = round_trip(&created, &PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Hero".into() })).await;
        assert_eq!(present_working_scene(&renamed).1[0].name, "Hero");
        let resized = round_trip(&renamed, &PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "t1".into(), new_crop: FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 } })).await;
        assert_eq!(present_working_scene(&resized).1[0].crop.width, 0.4);
        let deleted = round_trip(&resized, &PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "t1".into() })).await;
        assert!(present_working_scene(&deleted).1.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_tiles_removes_the_multi_select_and_reorder_tiles_moves_by_id() {
        let (source, _) = present_working_scene(&default_present_snapshot());
        let base = present_snapshot_with_tiles(&source, &[tile("t1"), tile("t2"), tile("t3")]);
        let reordered = round_trip(&base, &PresentMutation::ReorderTiles(reorder_tiles::mutation::ReorderTiles { id: "t1".into(), to_index: 2 })).await;
        assert_eq!(present_working_scene(&reordered).1.iter().map(|item| item.id.clone()).collect::<Vec<_>>(), vec!["t2", "t3", "t1"]);
        let culled = round_trip(&base, &PresentMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: vec!["t1".into(), "t3".into()] })).await;
        assert_eq!(present_working_scene(&culled).1.iter().map(|item| item.id.clone()).collect::<Vec<_>>(), vec!["t2"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_tiles_and_replace_source_and_resize_source_frame_round_trip() {
        let base = default_present_snapshot();
        let seeded = round_trip(&base, &PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: vec![tile("t1"), tile("t2")] })).await;
        assert_eq!(present_working_scene(&seeded).1.len(), 2);
        let cleared = round_trip(&seeded, &PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() })).await;
        assert!(present_working_scene(&cleared).1.is_empty());
        let (base_source, _) = present_working_scene(&base);
        let mut next_source = base_source.clone();
        next_source.kind = "video".into();
        let replaced = round_trip(&base, &PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: next_source.clone() })).await;
        assert_eq!(present_working_scene(&replaced).0.kind, "video");
        let resized = round_trip(&base, &PresentMutation::ResizeSourceFrame(resize_source_frame::mutation::ResizeSourceFrame { new_frame: FigureTileFrame { x: 0.2, y: 0.2, width: 0.5, height: 0.5 } })).await;
        assert_eq!(present_working_scene(&resized).0.frame.width, 0.5);
    }

    #[semio_framework_async_macros::async_test]
    async fn missing_targets_invert_to_nothing() {
        let base = default_present_snapshot();
        assert!(PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "gone".into() }).inverse(&base).is_empty());
        assert!(PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "gone".into(), new_name: "x".into() }).inverse(&base).is_empty());
        assert!(PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "gone".into(), new_crop: FigureTileFrame { x: 0.0, y: 0.0, width: 0.1, height: 0.1 } }).inverse(&base).is_empty());
        assert!(PresentMutation::ReorderTiles(reorder_tiles::mutation::ReorderTiles { id: "gone".into(), to_index: 0 }).inverse(&base).is_empty());
        assert!(PresentMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: vec!["gone".into()] }).inverse(&base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn create_tile_obeys_the_inverse_and_diff_absorb_laws() {
        let (source, _) = present_working_scene(&default_present_snapshot());
        let base = present_snapshot_with_tiles(&source, &[tile("t1")]);
        let mutation = PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 1, tile: tile("t2") });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 2, tile: tile("t3") }).diff(&base).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_tile_obeys_the_inverse_law() {
        let (source, _) = present_working_scene(&default_present_snapshot());
        let base = present_snapshot_with_tiles(&source, &[tile("t1")]);
        let mutation = PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Hero".into() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_source_obeys_the_inverse_and_diff_absorb_laws() {
        let base = default_present_snapshot();
        let (base_source, _) = present_working_scene(&base);
        let mut source_a = base_source.clone();
        source_a.kind = "video".into();
        let mut source_b = base_source.clone();
        source_b.kind = "figure".into();
        source_b.src = "/other.png".into();
        let mutation = PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: source_a });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: source_b }).diff(&base).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn semantic_kinds_cover_every_variant() {
        let kinds: Vec<&str> = PresentMutation::kinds().iter().map(|descriptor| descriptor.kind).collect();
        for expected in ["resize-source-frame", "replace-source", "create-tile", "delete-tile", "delete-tiles", "rename-tile", "resize-tile-crop", "reorder-tiles", "replace-tiles"] {
            assert!(kinds.contains(&expected), "missing semantic kind {expected}");
        }
    }

    //#region 🔖️OutcomeLaws
    // 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS — one law test per verb
    // family present in this facet (`assert_missing_target_is_error`/`assert_fatal_never_applies`,
    // landed in `📡️spr/🧪️testkit`). `replace` has no addressable target here (whole-collection
    // `replace-tiles` / singleton `replace-source`), so it has no missing-target case to exercise.
    // `assert_outcome_policy_matrix` is NOT landed under that name (only the generic closure-based
    // `assert_policy_matrix` exists) — see this ticket's report.
    #[semio_framework_async_macros::async_test]
    async fn create_family_fatal_never_applies() {
        let base = present_snapshot_with_tiles(&present_working_scene(&default_present_snapshot()).0, &[tile("t1")]);
        let outcome = PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile: tile("t1") }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_family_missing_target_is_error() {
        let base = default_present_snapshot();
        assert_missing_target_is_error(&base, &PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "missing".into() })).await;
        assert_missing_target_is_error(&base, &PresentMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: vec!["missing".into()] })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_family_missing_target_is_error() {
        let base = default_present_snapshot();
        assert_missing_target_is_error(&base, &PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "missing".into(), new_name: "x".into() })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn resize_family_missing_target_is_error() {
        let base = default_present_snapshot();
        assert_missing_target_is_error(&base, &PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "missing".into(), new_crop: FigureTileFrame { x: 0.0, y: 0.0, width: 0.1, height: 0.1 } })).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn resize_family_fatal_never_applies() {
        let base = default_present_snapshot();
        let outcome = PresentMutation::ResizeSourceFrame(resize_source_frame::mutation::ResizeSourceFrame { new_frame: FigureTileFrame { x: 0.0, y: 0.0, width: -1.0, height: 1.0 } }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_family_missing_target_is_error() {
        let base = default_present_snapshot();
        assert_missing_target_is_error(&base, &PresentMutation::ReorderTiles(reorder_tiles::mutation::ReorderTiles { id: "missing".into(), to_index: 0 })).await;
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🔖️KindsCatalog
    /// 🏷️ [`KINDS`] is the bridge between this enum and the language-neutral test platform, which
    /// never parses Rust. This proves it names every variant, in declaration order, with the same
    /// kebab spelling `#[derive(dsl::Mutations)]` derives — and that this subset's own committed
    /// catalog declares exactly the same set, so the completeness gate cannot be measuring a
    /// vocabulary that has drifted away from the code.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let declared: Vec<&str> = <PresentMutation as SemanticMutation<PresentSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        assert_eq!(KINDS, declared.as_slice(), "KINDS must name every PresentMutation variant, in declaration order, spelled as its own MutationKind::SEMANTICS.kind");
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in this subset's committed oracle manifest catalog present-1-any");
        }
    }
    //#endregion 🔖️KindsCatalog
}
//#endregion 🧪️Tests

//#region 🔖️Apply
/// 📦️ Applies `mutation` onto `snapshot`, returning the resulting snapshot.
pub fn apply_present_mutation(snapshot: &PresentSnapshot, mutation: &PresentMutation) -> protocol::MutationApplyResult<PresentSnapshot> {
    semio_framework_plugin::resolve_ready(vcs::apply_mutation(snapshot, mutation)).map(|(next, _messages)| next)
}

/// ↩️ Computes `mutation`'s inverse mutations against `snapshot` (pre-state).
pub fn inverse_present_mutation(snapshot: &PresentSnapshot, mutation: &PresentMutation) -> Vec<PresentMutation> {
    mutation.inverse(snapshot)
}

/// 📥️ Decodes this facet's own externally-tagged (`{"CreateTile": { … }}`) JSON projection — the
/// shape the `mutate-present-1` case's `Examples` rows carry — into a real [`PresentMutation`]. A
/// thin `serde_json` wrapper (already a direct dependency of this crate, used behind this interface
/// per CLAUDE.md's "external libraries behind an interface" rule, never a new one), so the test
/// adapter reads the committed feature row instead of re-declaring it as a Rust literal beside it.
pub fn decode_present_mutation_json(text: &str) -> Result<PresentMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ⚖️ The SEMANTIC PROJECTION this subset is compared through — `(schema, source, tiles)` read back
/// off the composed presentation child's working scene. It belongs to the subset rather than to a
/// test adapter, because what counts as this document's meaning is this subset's ruling, not a
/// case's. The two child handles are deliberately absent: `presentation_child_handle`
/// content-addresses exactly this `(source, tiles)` pair through `std`'s deliberately unspecified
/// `DefaultHasher`, so projecting one would compare the same content twice and pin a value the
/// standard library does not promise. `animation` carries no content at all today.
pub fn encode_present_projection_json(snapshot: &PresentSnapshot) -> String {
    let (source, tiles) = crate::artifacts::present::present_working_scene(snapshot);
    serde_json::json!({ "schema": snapshot.schema, "source": source, "tiles": tiles }).to_string()
}
//#endregion 🔖️Apply
