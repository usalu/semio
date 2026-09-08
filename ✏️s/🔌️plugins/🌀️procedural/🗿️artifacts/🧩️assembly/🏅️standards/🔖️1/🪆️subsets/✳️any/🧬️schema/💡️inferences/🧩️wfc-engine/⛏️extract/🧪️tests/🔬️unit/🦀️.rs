
use super::*;

fn checkerboard_sample(size: usize) -> Sample2d {
    let mut tiles = vec![TileId(0); size * size];
    for y in 0..size {
        for x in 0..size {
            tiles[y * size + x] = TileId(((x + y) % 2) as u32);
        }
    }
    Sample2d::new(size, size, tiles)
}

#[test]
fn extraction_rejects_empty_sample_list() {
    let cfg = Extract2dConfig::default();
    assert!(extract_2d(&[], &cfg).is_err());
}

#[test]
fn window_one_extracts_one_pattern_per_distinct_tile() {
    let sample = checkerboard_sample(4);
    let cfg = Extract2dConfig { window: 1, periodic_input: true, symmetry: SymmetryGroup2d::None };
    let extracted = extract_2d(&[sample], &cfg).unwrap();
    assert_eq!(extracted.model.pattern_count(), 2);
}

#[test]
fn window_two_deduplicates_repeated_windows() {
    let sample = checkerboard_sample(4);
    let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
    let extracted = extract_2d(&[sample], &cfg).unwrap();
    // A periodic 2-color checkerboard has exactly 2 distinct 2x2 windows under periodic wrap.
    assert_eq!(extracted.model.pattern_count(), 2);
}

#[test]
fn symmetry_expansion_can_only_add_patterns_never_remove() {
    let sample = checkerboard_sample(4);
    let cfg_none = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
    let cfg_d4 = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::D4 };
    let none = extract_2d(std::slice::from_ref(&sample), &cfg_none).unwrap();
    let d4 = extract_2d(&[sample], &cfg_d4).unwrap();
    assert!(d4.model.pattern_count() >= none.model.pattern_count());
}

#[test]
fn extracted_model_relations_match_von_neumann_stencil() {
    let sample = checkerboard_sample(4);
    let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
    let extracted = extract_2d(&[sample], &cfg).unwrap();
    assert_eq!(extracted.model.relation_count(), 4);
}

#[test]
fn periodic_sample_solves_on_a_same_size_wrapped_grid() {
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
fn window_content_is_preserved_for_decode() {
    let sample = checkerboard_sample(4);
    let cfg = Extract2dConfig { window: 2, periodic_input: true, symmetry: SymmetryGroup2d::None };
    let extracted = extract_2d(&[sample], &cfg).unwrap();
    for p in 0..extracted.model.pattern_count() {
        let pid = PatternId::from_index(p);
        assert_eq!(extracted.decoder.window_of(pid).len(), 4);
    }
}

#[test]
fn multiple_samples_merge_frequencies() {
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
