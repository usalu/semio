//! 🧪️ Overlapping-pattern extraction from 2D tile samples: `N × N` windows become patterns,
//! frequency becomes weight, and overlap agreement under the four unit grid offsets becomes
//! compatibility — the classic overlapping-WFC pipeline. Deliberately reuses
//! [`crate::wfc_engine::grid2d::declare_stencil_relations`] so an extracted model's relations line up exactly
//! with a [`crate::wfc_engine::grid2d::Grid2dTopology`] built with the same (von Neumann) stencil.

use crate::wfc_engine::grid2d::{declare_stencil_relations, Stencil2d};
use crate::wfc_engine::ids::{PatternId, TileId};
use crate::wfc_engine::model::{CompiledModel, ModelBuilder};
use crate::wfc_engine::symmetry::SymmetryGroup2d;

// #region 🔖️Sample
/// 🧪️ A row-major tile-id matrix to learn patterns from.
#[derive(Clone, Debug)]
pub struct Sample2d {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileId>,
}

impl Sample2d {
    pub fn new(width: usize, height: usize, tiles: Vec<TileId>) -> Self {
        debug_assert_eq!(tiles.len(), width * height);
        Self { width, height, tiles }
    }
}
// #endregion 🔖️Sample

// #region 🔖️Config
/// 🧪️ Options for [`extract_2d`].
#[derive(Clone, Debug)]
pub struct Extract2dConfig {
    /// 🧪️ Window side length (patterns are `window × window`).
    pub window: usize,
    /// 🧪️ Whether windows wrap around sample edges (periodic input) or are only taken from
    /// fully-in-bounds positions.
    pub periodic_input: bool,
    pub symmetry: SymmetryGroup2d,
}

impl Default for Extract2dConfig {
    fn default() -> Self {
        Self { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None }
    }
}
// #endregion 🔖️Config

// #region 🔖️Decoder
/// 🧪️ Maps extracted `PatternId`s back to the tile at each pattern window's top-left corner (the
/// anchor convention: overlap-agreement between neighboring patterns guarantees every cell's
/// anchor tile is mutually consistent with its neighbors' anchor tiles).
#[derive(Clone, Debug)]
pub struct PatternDecoder2d {
    window: usize,
    pattern_windows: Vec<Vec<TileId>>,
}

impl PatternDecoder2d {
    pub fn window(&self) -> usize {
        self.window
    }

    pub fn anchor_tile(&self, p: PatternId) -> TileId {
        self.pattern_windows[p.index()][0]
    }

    pub fn window_of(&self, p: PatternId) -> &[TileId] {
        &self.pattern_windows[p.index()]
    }

    /// 🧪️ Decodes a full grid assignment to its anchor-tile image, row-major.
    pub fn decode(&self, assignment: &[PatternId]) -> Vec<TileId> {
        assignment.iter().map(|&p| self.anchor_tile(p)).collect()
    }
}
// #endregion 🔖️Decoder

// #region 🔖️Extract
/// 🧪️ The compiled model plus everything needed to decode its patterns back to tiles.
#[derive(Clone, Debug)]
pub struct ExtractedModel2d {
    pub model: CompiledModel,
    pub decoder: PatternDecoder2d,
}

fn window_at(sample: &Sample2d, x: usize, y: usize, n: usize, periodic: bool) -> Option<Vec<TileId>> {
    if !periodic && (x + n > sample.width || y + n > sample.height) {
        return None;
    }
    let mut w = vec![TileId(0); n * n];
    for wy in 0..n {
        for wx in 0..n {
            let sx = if periodic { (x + wx) % sample.width } else { x + wx };
            let sy = if periodic { (y + wy) % sample.height } else { y + wy };
            w[wy * n + wx] = sample.tiles[sy * sample.width + sx];
        }
    }
    Some(w)
}

/// 🧪️ `a` placed at the origin, `b` placed at grid offset `(dx, dy)` — compatible iff every cell
/// where their `n × n` footprints overlap holds the same tile.
fn windows_overlap_compatible(a: &[TileId], b: &[TileId], n: usize, dx: i32, dy: i32) -> bool {
    for y in 0..n as i32 {
        for x in 0..n as i32 {
            let bx = x - dx;
            let by = y - dy;
            if bx >= 0 && bx < n as i32 && by >= 0 && by < n as i32 && a[y as usize * n + x as usize] != b[by as usize * n + bx as usize] {
                return false;
            }
        }
    }
    true
}

/// 🧪️ Extracts overlapping patterns from one or more samples (frequencies merge across samples),
/// expanding each window under `cfg.symmetry` before deduplication, and compiles a model whose
/// relations are exactly [`Stencil2d::VonNeumann`]'s four unit offsets.
pub fn extract_2d(samples: &[Sample2d], cfg: &Extract2dConfig) -> Result<ExtractedModel2d, crate::wfc_engine::error::ModelError> {
    use crate::wfc_engine::error::ModelError;
    let n = cfg.window;
    if n == 0 {
        return Err(ModelError::CapacityOverflow { what: "extract_2d window size" });
    }

    let mut window_freq: std::collections::HashMap<Vec<TileId>, u64> = std::collections::HashMap::new();
    for sample in samples {
        let (x_positions, y_positions): (usize, usize) = if cfg.periodic_input { (sample.width, sample.height) } else { (sample.width.saturating_sub(n - 1), sample.height.saturating_sub(n - 1)) };
        for y in 0..y_positions {
            for x in 0..x_positions {
                let Some(base) = window_at(sample, x, y, n, cfg.periodic_input) else { continue };
                for transform in cfg.symmetry.elements() {
                    let (tw, th, tiles) = transform.apply_window(n, n, &base);
                    debug_assert_eq!((tw, th), (n, n), "square windows are invariant under D4 dimension swap");
                    *window_freq.entry(tiles).or_insert(0) += 1;
                }
            }
        }
    }
    if window_freq.is_empty() {
        return Err(ModelError::EmptyPatternUniverse);
    }

    let mut windows: Vec<(Vec<TileId>, u64)> = window_freq.into_iter().collect();
    windows.sort_by(|a, b| a.0.cmp(&b.0));

    let mut builder = ModelBuilder::new();
    for &(_, freq) in &windows {
        builder.add_pattern(freq as f64);
    }
    let relations = declare_stencil_relations(&mut builder, &Stencil2d::VonNeumann)?;
    let offsets = Stencil2d::VonNeumann.offsets();

    for i in 0..windows.len() {
        for j in 0..windows.len() {
            for (k, &(dx, dy)) in offsets.iter().enumerate() {
                if windows_overlap_compatible(&windows[i].0, &windows[j].0, n, dx, dy) {
                    builder.allow(relations[k], PatternId::from_index(i), PatternId::from_index(j));
                }
            }
        }
    }

    let model = builder.compile()?;
    let decoder = PatternDecoder2d { window: n, pattern_windows: windows.into_iter().map(|(w, _)| w).collect() };
    Ok(ExtractedModel2d { model, decoder })
}
// #endregion 🔖️Extract

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
