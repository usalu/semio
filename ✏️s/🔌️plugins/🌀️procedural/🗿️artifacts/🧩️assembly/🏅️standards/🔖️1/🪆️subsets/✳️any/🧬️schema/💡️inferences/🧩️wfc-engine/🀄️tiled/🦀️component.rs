//! 🧱️ Explicit tiled model construction: one authored tile ↔ one pattern (until [`crate::wfc_engine::symmetry`]
//! expands orbits in a later phase), with allow/deny pair lists and eagerly-compiled predicates.
//! A thin `TileId`-facing wrapper over [`crate::wfc_engine::model::ModelBuilder`]'s `PatternId`-facing API.

use crate::wfc_engine::error::ModelError;
use crate::wfc_engine::ids::{RelationId, TileId};
use crate::wfc_engine::model::{CompiledModel, ModelBuilder};

// #region 🔖️Builder
/// 🧱️ Builds a [`CompiledModel`] from tiles, weights, and directional allow/deny pairs.
#[derive(Clone, Debug, Default)]
pub struct TiledModelBuilder {
    builder: ModelBuilder,
    tile_pattern: Vec<crate::wfc_engine::ids::PatternId>,
}

impl TiledModelBuilder {
    pub async fn new() -> Self {
        Self { builder: ModelBuilder::new(), tile_pattern: Vec::new() }
    }

    /// 🧱️ Registers a new tile with the given sampling weight.
    pub async fn tile(&mut self, weight: f64) -> TileId {
        let p = self.builder.add_pattern(weight);
        let id = TileId::from_index(self.tile_pattern.len());
        self.tile_pattern.push(p);
        self.builder.set_tile(p, id);
        id
    }

    pub async fn tag(&mut self, tile: TileId, name: &str) -> u32 {
        self.builder.add_tag(self.tile_pattern[tile.index()], name)
    }

    pub async fn relation(&mut self, name: &str) -> RelationId {
        self.builder.add_relation(name)
    }

    pub async fn set_relation_inverse(&mut self, a: RelationId, b: RelationId) {
        self.builder.set_relation_inverse(a, b);
    }

    pub async fn allow(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.allow(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    /// 🧱️ `deny` always wins over `allow`, regardless of call order.
    pub async fn deny(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.deny(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    pub async fn allow_mirrored(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.allow_mirrored(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    /// 🧱️ Bulk allow from a predicate over every pair in `tiles`, compiled eagerly right now (the
    /// predicate itself is never stored — only its resolved allow pairs survive into the model).
    pub async fn allow_where(&mut self, r: RelationId, tiles: &[TileId], pred: impl Fn(TileId, TileId) -> bool) {
        for &a in tiles {
            for &b in tiles {
                if pred(a, b) {
                    self.allow(r, a, b);
                }
            }
        }
    }

    pub async fn pattern_of(&self, tile: TileId) -> crate::wfc_engine::ids::PatternId {
        self.tile_pattern[tile.index()]
    }

    pub async fn tile_count(&self) -> usize {
        self.tile_pattern.len()
    }

    pub async fn compile(self) -> Result<CompiledModel, ModelError> {
        self.builder.compile()
    }
}
// #endregion 🔖️Builder

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn tile_to_pattern_is_one_to_one() {
        let mut b = TiledModelBuilder::new();
        let grass = b.tile(3.0);
        let water = b.tile(1.0);
        assert_ne!(b.pattern_of(grass), b.pattern_of(water));
        assert_eq!(b.tile_count(), 2);
    }

    #[test]
    async fn allow_and_deny_compile_correctly() {
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
    async fn allow_where_compiles_predicate_eagerly() {
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
    async fn tags_round_trip_through_tiles() {
        let mut b = TiledModelBuilder::new();
        let t = b.tile(1.0);
        let id = b.tag(t, "solid");
        b.relation("r");
        let pt = b.pattern_of(t);
        let m = b.compile().unwrap();
        assert_eq!(m.pattern_info(pt).tags, vec![id]);
    }
}
// #endregion 🔖️Tests
