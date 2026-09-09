use super::*;

#[test]
fn grid_seed_produces_tiles() {
    let source = crate::default_figure_tile_source();
    let tiles = populate_tile_drafts_from_grid(FigureTileGridSeedSpec { source: &source, rows: 3, columns: 5, gap: 0.0, key_prefix: "tile" });
    assert_eq!(tiles.len(), 15);
    assert_eq!(tiles[0].id, "tile-r0-c0");
}

#[test]
fn parse_grid_engagement_accepts_cross() {
    assert_eq!(parse_grid_engagement("3×5"), Some((3, 5)));
    assert_eq!(parse_grid_engagement("2x2"), Some((2, 2)));
}

#[test]
fn morph_prompt_lists_tiles() {
    let source = crate::default_figure_tile_source();
    let tiles = vec![crate::FigureTileDraft { id: "t1".into(), name: "t1".into(), crop: crate::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 } }];
    let prompt = build_tile_morph_prompt(&source, &tiles);
    assert!(prompt.contains("t1"));
    assert!(prompt.contains("Source media"));
}
