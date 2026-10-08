use super::*;
use crate::default_presentation_snapshot;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::standards::v1::subsets::any::schema::mutations::replace_source;
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn replace_source_diff_applies_onto_the_base_snapshot() {
    let base = default_presentation_snapshot();
    let (source, _tiles) = crate::presentation_working_scene(&base);
    let mut next_source = source;
    next_source.kind = "video".into();
    let operation = PresentationMutation::ReplaceSource(replace_source::ReplaceSource { new_source: next_source });
    let diff: PresentationDiff = operation.diff(&base).into_parts().0;
    assert!(diff.source.is_some());
    assert!(diff.tiles.is_none(), "a source replacement names no tile row");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&operation, &base).await;
    let (applied_source, _) = crate::presentation_working_scene(&protocol::apply_diff(&diff, &base).expect("valid mutation diff"));
    assert_eq!(applied_source.kind, "video");
}

fn tile(id: &str) -> crate::FigureTileDraft {
    crate::FigureTileDraft { id: id.into(), name: id.into(), crop: crate::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } }
}

fn snapshot_of(ids: &[&str]) -> PresentationSnapshot {
    let mut snapshot = default_presentation_snapshot();
    snapshot.tiles = ids.iter().map(|id| tile(id)).collect();
    snapshot
}

fn ids(snapshot: &PresentationSnapshot) -> Vec<&str> {
    snapshot.tiles.iter().map(|tile| tile.id.as_str()).collect()
}

fn diff_of(tiles: PresentationTilesDelta) -> PresentationDiff {
    PresentationDiff { tiles: Some(tiles), ..Default::default() }
}

#[test]
fn positional_tile_delta_applies_inverts_and_sums_at_a_middle_row() {
    let base = snapshot_of(&["a", "b", "c"]);
    let insert = diff_of(PresentationTilesDelta::insertion(1, tile("x")));
    let inserted = protocol::apply_diff(&insert, &base).expect("valid insertion");
    assert_eq!(ids(&inserted), ["a", "x", "b", "c"]);
    let relocate = diff_of(PresentationTilesDelta::relocation(&inserted.tiles, 0, 3));
    let moved = protocol::apply_diff(&relocate, &inserted).expect("valid relocation");
    assert_eq!(ids(&moved), ["x", "b", "c", "a"]);
    let remove = diff_of(PresentationTilesDelta::removal(&moved.tiles, 1));
    let removed = protocol::apply_diff(&remove, &moved).expect("valid removal");
    assert_eq!(ids(&removed), ["x", "c", "a"]);
    let mut sum = insert;
    sum.absorb(relocate);
    sum.absorb(remove);
    assert_eq!(protocol::apply_diff(&sum, &base).expect("valid sum"), removed);
    let inverse = protocol::DiffAlgebra::inverse(&sum, &base);
    assert_eq!(protocol::apply_diff(&inverse, &removed).expect("valid inverse"), base);
}

#[test]
fn positional_tile_delta_refuses_a_removal_that_is_not_at_its_base_index() {
    let base = snapshot_of(&["a", "b"]);
    let mut removal = PresentationTilesDelta::removal(&base.tiles, 0);
    removal.removed[0].index = 1;
    assert!(protocol::apply_diff(&diff_of(removal), &base).is_err());
}
