//! 🧬️ present artifact — document mutation dispatch.

use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;
use protocol::{Mutation, SemanticMutation};
use serde::{Deserialize, Serialize};

//#region 🔖️MutationLeaves
// 🧵️ Each block below is exactly one `🧬️mutations/<kind>/` triad leaf (🦠️mutation/🔺️diff/
// ↩️inverse), wired here (relative to THIS file's own directory) instead of in the plugin's
// `📦️glue.rs` — this facet's fan-out ticket (SEMANTIC-MUTATIONS-OVERHAUL) explicitly scopes
// `📦️glue.rs` out (shared across every facet of this plugin, edited by other concurrent sessions);
// `#[path = "."]` on the group module keeps the base directory unchanged for its children exactly
// like `📦️glue.rs`'s own wiring blocks do, so every leaf's `pub use component::*;` still surfaces
// at `crate::artifacts::present::mutations::<kind>::{mutation,diff,inverse}`, matching the shape
// sibling plugins (`gis`, `cad`, `fem`) wire directly in their `📦️glue.rs`. See this facet's report
// (`sharedFileRequests`) for the follow-up `📦️glue.rs` reconciliation.
#[path = "."]
pub mod resize_source_frame {
    #[path = "🔲resize-source-frame/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔲resize-source-frame/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔲resize-source-frame/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod replace_source {
    #[path = "🖼replace-source/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🖼replace-source/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🖼replace-source/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod create_tile {
    #[path = "🆕create-tile/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🆕create-tile/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🆕create-tile/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_tile {
    #[path = "🗑delete-tile/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🗑delete-tile/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🗑delete-tile/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod delete_tiles {
    #[path = "🗑delete-tiles/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🗑delete-tiles/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🗑delete-tiles/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod rename_tile {
    #[path = "✏rename-tile/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "✏rename-tile/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "✏rename-tile/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod resize_tile_crop {
    #[path = "✂resize-tile-crop/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "✂resize-tile-crop/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "✂resize-tile-crop/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod reorder_tiles {
    #[path = "🔀reorder-tiles/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔀reorder-tiles/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔀reorder-tiles/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
#[path = "."]
pub mod replace_tiles {
    #[path = "🔁replace-tiles/🦠️mutation/🦀️component.rs"]
    pub mod mutation;
    #[path = "🔁replace-tiles/🔺️diff/🦀️component.rs"]
    pub mod diff;
    #[path = "🔁replace-tiles/↩️inverse/🦀️component.rs"]
    pub mod inverse;
}
//#endregion 🔖️MutationLeaves

//#region 🔖️Mutations
/// 🎬️ Typed, invertible, semantic present-deck mutation vocabulary — every variant wraps exactly
/// one `protocol::MutationKind` payload struct declared in its own `🧬️mutations/<kind>/🦠️mutation`
/// triad leaf; `#[derive(dsl::Mutations)]` wires `Mutation`/`SemanticMutation` from those leaves.
/// `source` (a singleton facet) gets `replace-source`/`resize-source-frame`; `tiles` (an id-keyed
/// ordered collection) gets `create`/`delete`/`delete-tiles`/`rename`/`resize-tile-crop`/`reorder`/
/// `replace-tiles` per `derivation-rules.md`'s per-id-keyed-collection recipe. Replaces the former
/// generic `Tiles(CollectionMutation<..>)`/`SetSource`/`SetTiles`/`SetSnapshot` vocabulary —
/// whole-document replacement is not expressible as an in-history mutation at all (goes through
/// `ArtifactStore::reset`, an app-level concern outside this enum).
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
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::present::{default_present_snapshot, FigureTileDraft, FigureTileFrame};

    fn tile(id: &str) -> FigureTileDraft {
        FigureTileDraft { id: id.into(), name: id.into(), crop: FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }
    }

    fn round_trip(base: &PresentSnapshot, mutation: &PresentMutation) -> PresentSnapshot {
        let forward = vcs::apply_mutation(base, mutation);
        let mut backward = mutation.inverse(base);
        backward.reverse();
        let mut restored = forward.clone();
        for undo in &backward {
            restored = vcs::apply_mutation(&restored, undo);
        }
        assert_eq!(&restored, base, "inverse (reversed) must exactly restore the pre-mutation snapshot");
        forward
    }

    #[test]
    fn tiles_create_rename_resize_delete_round_trip() {
        let base = default_present_snapshot();
        let created = round_trip(&base, &PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 0, tile: tile("t1") }));
        assert_eq!(created.tiles.len(), 1);
        let renamed = round_trip(&created, &PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Hero".into() }));
        assert_eq!(renamed.tiles[0].name, "Hero");
        let resized = round_trip(
            &renamed,
            &PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "t1".into(), new_crop: FigureTileFrame { x: 0.3, y: 0.3, width: 0.4, height: 0.4 } }),
        );
        assert_eq!(resized.tiles[0].crop.width, 0.4);
        let deleted = round_trip(&resized, &PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "t1".into() }));
        assert!(deleted.tiles.is_empty());
    }

    #[test]
    fn delete_tiles_removes_the_multi_select_and_reorder_tiles_moves_by_id() {
        let base = PresentSnapshot { tiles: vec![tile("t1"), tile("t2"), tile("t3")], ..default_present_snapshot() };
        let reordered = round_trip(&base, &PresentMutation::ReorderTiles(reorder_tiles::mutation::ReorderTiles { id: "t1".into(), to_index: 2 }));
        assert_eq!(reordered.tiles.iter().map(|item| item.id.clone()).collect::<Vec<_>>(), vec!["t2", "t3", "t1"]);
        let culled = round_trip(&base, &PresentMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: vec!["t1".into(), "t3".into()] }));
        assert_eq!(culled.tiles.iter().map(|item| item.id.clone()).collect::<Vec<_>>(), vec!["t2"]);
    }

    #[test]
    fn replace_tiles_and_replace_source_and_resize_source_frame_round_trip() {
        let base = default_present_snapshot();
        let seeded = round_trip(&base, &PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: vec![tile("t1"), tile("t2")] }));
        assert_eq!(seeded.tiles.len(), 2);
        let cleared = round_trip(&seeded, &PresentMutation::ReplaceTiles(replace_tiles::mutation::ReplaceTiles { new_tiles: Vec::new() }));
        assert!(cleared.tiles.is_empty());
        let mut next_source = base.source.clone();
        next_source.kind = "video".into();
        let replaced = round_trip(&base, &PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: next_source.clone() }));
        assert_eq!(replaced.source.kind, "video");
        let resized = round_trip(&base, &PresentMutation::ResizeSourceFrame(resize_source_frame::mutation::ResizeSourceFrame { new_frame: FigureTileFrame { x: 0.2, y: 0.2, width: 0.5, height: 0.5 } }));
        assert_eq!(resized.source.frame.width, 0.5);
    }

    #[test]
    fn missing_targets_invert_to_nothing() {
        let base = default_present_snapshot();
        assert!(PresentMutation::DeleteTile(delete_tile::mutation::DeleteTile { id: "gone".into() }).inverse(&base).is_empty());
        assert!(PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "gone".into(), new_name: "x".into() }).inverse(&base).is_empty());
        assert!(PresentMutation::ResizeTileCrop(resize_tile_crop::mutation::ResizeTileCrop { id: "gone".into(), new_crop: FigureTileFrame { x: 0.0, y: 0.0, width: 0.1, height: 0.1 } })
            .inverse(&base)
            .is_empty());
        assert!(PresentMutation::ReorderTiles(reorder_tiles::mutation::ReorderTiles { id: "gone".into(), to_index: 0 }).inverse(&base).is_empty());
        assert!(PresentMutation::DeleteTiles(delete_tiles::mutation::DeleteTiles { ids: vec!["gone".into()] }).inverse(&base).is_empty());
    }

    #[test]
    fn create_tile_obeys_the_inverse_and_diff_absorb_laws() {
        let base = PresentSnapshot { tiles: vec![tile("t1")], ..default_present_snapshot() };
        let mutation = PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 1, tile: tile("t2") });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = PresentMutation::CreateTile(create_tile::mutation::CreateTile { index: 2, tile: tile("t3") }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn rename_tile_obeys_the_inverse_law() {
        let base = PresentSnapshot { tiles: vec![tile("t1")], ..default_present_snapshot() };
        let mutation = PresentMutation::RenameTile(rename_tile::mutation::RenameTile { id: "t1".into(), new_name: "Hero".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
    }

    #[test]
    fn replace_source_obeys_the_inverse_and_diff_absorb_laws() {
        let base = default_present_snapshot();
        let mut source_a = base.source.clone();
        source_a.kind = "video".into();
        let mut source_b = base.source.clone();
        source_b.kind = "figure".into();
        source_b.src = "/other.png".into();
        let mutation = PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: source_a });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = PresentMutation::ReplaceSource(replace_source::mutation::ReplaceSource { new_source: source_b }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn semantic_kinds_cover_every_variant() {
        let kinds: Vec<&str> = PresentMutation::kinds().iter().map(|descriptor| descriptor.kind).collect();
        for expected in [
            "resize-source-frame",
            "replace-source",
            "create-tile",
            "delete-tile",
            "delete-tiles",
            "rename-tile",
            "resize-tile-crop",
            "reorder-tiles",
            "replace-tiles",
        ] {
            assert!(kinds.contains(&expected), "missing semantic kind {expected}");
        }
    }
}
//#endregion 🧪️Tests

//#region 🔖️Apply
/// 📦️ Applies `mutation` onto `snapshot`, returning the resulting snapshot.
pub fn apply_present_mutation(snapshot: &PresentSnapshot, mutation: &PresentMutation) -> PresentSnapshot {
    vcs::apply_mutation(snapshot, mutation)
}

/// ↩️ Computes `mutation`'s inverse mutations against `snapshot` (pre-state).
pub fn inverse_present_mutation(snapshot: &PresentSnapshot, mutation: &PresentMutation) -> Vec<PresentMutation> {
    mutation.inverse(snapshot)
}
//#endregion 🔖️Apply
