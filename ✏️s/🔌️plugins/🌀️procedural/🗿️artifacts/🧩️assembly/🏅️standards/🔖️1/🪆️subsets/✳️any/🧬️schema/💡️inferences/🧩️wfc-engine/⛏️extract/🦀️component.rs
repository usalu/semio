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
    pub async fn new(width: usize, height: usize, tiles: Vec<TileId>) -> Self {
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
    async fn default() -> Self {
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
    pub async fn window(&self) -> usize {
        self.window
    }

    pub async fn anchor_tile(&self, p: PatternId) -> TileId {
        self.pattern_windows[p.index()][0]
    }

    pub async fn window_of(&self, p: PatternId) -> &[TileId] {
        &self.pattern_windows[p.index()]
    }

    /// 🧪️ Decodes a full grid assignment to its anchor-tile image, row-major.
    pub async fn decode(&self, assignment: &[PatternId]) -> Vec<TileId> {
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

async fn window_at(sample: &Sample2d, x: usize, y: usize, n: usize, periodic: bool) -> Option<Vec<TileId>> {
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
async fn windows_overlap_compatible(a: &[TileId], b: &[TileId], n: usize, dx: i32, dy: i32) -> bool {
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
pub async fn extract_2d(samples: &[Sample2d], cfg: &Extract2dConfig) -> Result<ExtractedModel2d, crate::wfc_engine::error::ModelError> {
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
mod tests {
    use super::*;

    async fn checkerboard_sample(size: usize) -> Sample2d {
        let mut tiles = vec![TileId(0); size * size];
        for y in 0..size {
            for x in 0..size {
                tiles[y * size + x] = TileId(((x + y) % 2) as u32);
            }
        }
        Sample2d::new(size, size, tiles)
    }

    #[test]
    async fn extraction_rejects_empty_sample_list() {
        let cfg = Extract2dConfig::default();
        assert!(extract_2d(&[], &cfg).is_err());
    }

    #[test]
    async fn window_one_extracts_one_pattern_per_distinct_tile() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 1, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        assert_eq!(extracted.model.pattern_count(), 2);
    }

    #[test]
    async fn window_two_deduplicates_repeated_windows() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        // A periodic 2-color checkerboard has exactly 2 distinct 2x2 windows under periodic wrap.
        assert_eq!(extracted.model.pattern_count(), 2);
    }

    #[test]
    async fn symmetry_expansion_can_only_add_patterns_never_remove() {
        let sample = checkerboard_sample(4);
        let cfg_none = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let cfg_d4 = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::D4 };
        let none = extract_2d(std::slice::from_ref(&sample), &cfg_none).unwrap();
        let d4 = extract_2d(&[sample], &cfg_d4).unwrap();
        assert!(d4.model.pattern_count() >= none.model.pattern_count());
    }

    #[test]
    async fn extracted_model_relations_match_von_neumann_stencil() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        assert_eq!(extracted.model.relation_count(), 4);
    }

    #[test]
    async fn periodic_sample_solves_on_a_same_size_wrapped_grid() {
        // The canonical WFC sanity check: a periodic training sample's own tiling must remain a
        // satisfiable solution of the extracted model on a same-size, wrap-boundary grid — if
        // extraction/compatibility were buggy, even the sample's own arrangement could become
        // unsolvable.
        use crate::wfc_engine::grid2d::{Boundary, Grid2dTopology};
        use crate::wfc_engine::solver_grid2d::Grid2dSolverBuilder;

        let size = 4;
        let sample = checkerboard_sample(size);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();

        let relations = Stencil2d::VonNeumann.offsets().iter().enumerate().map(|(i, _)| crate::wfc_engine::ids::RelationId(i as u32)).collect::<Vec<_>>();
        let topo = Grid2dTopology::new(size, size, &Stencil2d::VonNeumann, relations, Boundary::Wrap, Boundary::Wrap, None).unwrap();
        let mut solver = Grid2dSolverBuilder::new(extracted.model, topo).build().unwrap();
        let outcome = solver.solve(1);
        assert!(matches!(outcome, crate::wfc_engine::outcome::SolveOutcome::Solved(_)), "extracted model must remain solvable on a same-size wrapped grid");
    }

    #[test]
    async fn window_content_is_preserved_for_decode() {
        let sample = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let extracted = extract_2d(&[sample], &cfg).unwrap();
        for p in 0..extracted.model.pattern_count() {
            let pid = PatternId::from_index(p);
            assert_eq!(extracted.decoder.window_of(pid).len(), 4);
        }
    }

    #[test]
    async fn multiple_samples_merge_frequencies() {
        let a = checkerboard_sample(4);
        let b = checkerboard_sample(4);
        let cfg = Extract2dConfig { window: 1, periodic_input: true, symmetry: SymmetryGroup2d::None };
        let single = extract_2d(std::slice::from_ref(&a), &cfg).unwrap();
        let merged = extract_2d(&[a, b], &cfg).unwrap();
        assert_eq!(single.model.pattern_count(), merged.model.pattern_count());
        // Each pattern's weight should double when the same sample is provided twice.
        for p in 0..merged.model.pattern_count() {
            let pid = PatternId::from_index(p);
            assert!((merged.model.pattern_info(pid).weight - 2.0 * single.model.pattern_info(pid).weight).abs() < 1e-9);
        }
    }
}
// #endregion 🔖️Tests
