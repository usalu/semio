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
    pub fn new() -> Self {
        Self { builder: ModelBuilder::new(), tile_pattern: Vec::new() }
    }

    /// 🧱️ Registers a new tile with the given sampling weight.
    pub fn tile(&mut self, weight: f64) -> TileId {
        let p = self.builder.add_pattern(weight);
        let id = TileId::from_index(self.tile_pattern.len());
        self.tile_pattern.push(p);
        self.builder.set_tile(p, id);
        id
    }

    pub fn tag(&mut self, tile: TileId, name: &str) -> u32 {
        self.builder.add_tag(self.tile_pattern[tile.index()], name)
    }

    pub fn relation(&mut self, name: &str) -> RelationId {
        self.builder.add_relation(name)
    }

    pub fn set_relation_inverse(&mut self, a: RelationId, b: RelationId) {
        self.builder.set_relation_inverse(a, b);
    }

    pub fn allow(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.allow(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    /// 🧱️ `deny` always wins over `allow`, regardless of call order.
    pub fn deny(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.deny(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    pub fn allow_mirrored(&mut self, r: RelationId, a: TileId, b: TileId) {
        self.builder.allow_mirrored(r, self.tile_pattern[a.index()], self.tile_pattern[b.index()]);
    }

    /// 🧱️ Bulk allow from a predicate over every pair in `tiles`, compiled eagerly right now (the
    /// predicate itself is never stored — only its resolved allow pairs survive into the model).
    pub fn allow_where(&mut self, r: RelationId, tiles: &[TileId], pred: impl Fn(TileId, TileId) -> bool) {
        for &a in tiles {
            for &b in tiles {
                if pred(a, b) {
                    self.allow(r, a, b);
                }
            }
        }
    }

    pub fn pattern_of(&self, tile: TileId) -> crate::wfc_engine::ids::PatternId {
        self.tile_pattern[tile.index()]
    }

    pub fn tile_count(&self) -> usize {
        self.tile_pattern.len()
    }

    pub fn compile(self) -> Result<CompiledModel, ModelError> {
        self.builder.compile()
    }
}
// #endregion 🔖️Builder

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
