
use super::*;

#[test]
fn tile_to_pattern_is_one_to_one() {
    let mut b = TiledModelBuilder::new();
    let grass = b.tile(3.0);
    let water = b.tile(1.0);
    assert_ne!(b.pattern_of(grass), b.pattern_of(water));
    assert_eq!(b.tile_count(), 2);
}

#[test]
fn allow_and_deny_compile_correctly() {
    let mut b = TiledModelBuilder::new();
    let a = b.tile(1.0);
    let c = b.tile(1.0);
    let d = b.tile(1.0);
    let r = b.relation("r");
    b.allow_mirrored(r, a, c);
    b.allow(r, a, d);
    b.deny(r, a, d);
    let (pa, pc, pd) = (b.pattern_of(a), b.pattern_of(c), b.pattern_of(d));
    let m = b.compile().unwrap();
    assert!(m.allowed(r, pa).get(pc));
    assert!(!m.allowed(r, pa).get(pd));
}

#[test]
fn allow_where_compiles_predicate_eagerly() {
    let mut b = TiledModelBuilder::new();
    let tiles: Vec<TileId> = (0..4).map(|_| b.tile(1.0)).collect();
    let r = b.relation("le");
    b.allow_where(r, &tiles, |x, y| x.get() <= y.get());
    let pattern_of: Vec<_> = tiles.iter().map(|&t| b.pattern_of(t)).collect();
    let m = b.compile().unwrap();
    for (xi, &x) in tiles.iter().enumerate() {
        for (yi, &y) in tiles.iter().enumerate() {
            let expected = x.get() <= y.get();
            assert_eq!(m.allowed(r, pattern_of[xi]).get(pattern_of[yi]), expected);
        }
    }
}

#[test]
fn tags_round_trip_through_tiles() {
    let mut b = TiledModelBuilder::new();
    let t = b.tile(1.0);
    let id = b.tag(t, "solid");
    b.relation("r");
    let pt = b.pattern_of(t);
    let m = b.compile().unwrap();
    assert_eq!(m.pattern_info(pt).tags, vec![id]);
}
