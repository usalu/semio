//! 🧪️ Drives the resumable [`WfcJob`] over the two dense grid topologies and over an
//! overlapping-extraction model, proving the un-gated grid/extract modules are reachable from the
//! same production job the graph artifacts use.

use semio_framework_job::{allocate_operation_id, root_cancel_token, Generation, InteractiveJob, InteractiveJobCloseStep, Operation, RevisionId, StepBudget, StepContext, StepOutcome, JOB_PAYLOAD_PAGE_BYTES};

use crate::extract::{extract_2d, Extract2dConfig, Sample2d};
use crate::grid2d::{declare_stencil_relations, declare_stencil_relations_tiled, Boundary, Grid2dTopology, Stencil2d};
use crate::grid3d::{declare_stencil_relations_3d_tiled, Grid3dTopology, Stencil3d};
use crate::ids::{PatternId, RelationId, TileId};
use crate::job::{WfcCommit, WfcJob, WfcJobConfig};
use crate::model::{CompiledModel, ModelBuilder};
use crate::symmetry::SymmetryGroup2d;
use crate::tiled::TiledModelBuilder;
use crate::topology::Topology;

const STEP_LIMIT: usize = 4_000_000;

fn operation(seed: u64) -> Operation {
    Operation::new(allocate_operation_id(), RevisionId(1), Generation(1), seed)
}

/// 🧵️ Steps the job to its terminal outcome, retiring every payload the way a worker session does.
fn solve<T: Topology + Clone + Send>(job: &mut WfcJob<T>, operation: Operation, fuel: u64) -> WfcCommit {
    let mut sequence = operation.preview_sequence;
    let mut terminal = None;
    for _ in 0..STEP_LIMIT {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(fuel, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        let mut outcome = job.step(&mut context);
        let complete = matches!(outcome, StepOutcome::Complete(_));
        while !outcome.terminal_is_empty() {
            outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES);
        }
        if outcome.is_terminal() {
            assert!(complete, "WFC job ended without a commit");
            terminal = job.commit();
            break;
        }
    }
    let commit = terminal.expect("WFC job did not complete");
    job.begin_close();
    for _ in 0..STEP_LIMIT {
        if job.terminal_is_empty() {
            return commit;
        }
        assert_ne!(job.close_step(1, JOB_PAYLOAD_PAGE_BYTES), InteractiveJobCloseStep::Blocked, "a locally owned job close has no external owner");
    }
    panic!("WFC job did not close");
}

/// 🀄️ Two tiles that may never sit next to their own kind — a checkerboard on any stencil.
fn checkerboard_tiles_2d() -> (CompiledModel, Vec<RelationId>) {
    let mut builder = TiledModelBuilder::new();
    let black = builder.tile(1.0);
    let white = builder.tile(1.0);
    let relations = declare_stencil_relations_tiled(&mut builder, &Stencil2d::VonNeumann).expect("2d stencil relations");
    for &relation in &relations {
        builder.allow_mirrored(relation, black, white);
    }
    (builder.compile().expect("2d tiled model"), relations)
}

/// 🧊️ The three-axis sibling of [`checkerboard_tiles_2d`].
fn checkerboard_tiles_3d() -> (CompiledModel, Vec<RelationId>) {
    let mut builder = TiledModelBuilder::new();
    let black = builder.tile(1.0);
    let white = builder.tile(1.0);
    let relations = declare_stencil_relations_3d_tiled(&mut builder, &Stencil3d::Face6).expect("3d stencil relations");
    for &relation in &relations {
        builder.allow_mirrored(relation, black, white);
    }
    (builder.compile().expect("3d tiled model"), relations)
}

#[test]
fn wfc_job_drives_a_grid_2d_topology_to_a_valid_commit() {
    let (model, relations) = checkerboard_tiles_2d();
    let topology = Grid2dTopology::new(6, 6, &Stencil2d::VonNeumann, relations, Boundary::Open, Boundary::Open, None).expect("2d topology");
    let op = operation(4_211);
    let mut job = WfcJob::new(op, model, topology.clone(), WfcJobConfig::default(), None, Vec::new());
    let commit = solve(&mut job, op, 8);
    assert_eq!(commit.assignment.len(), topology.node_count());
    for y in 0..topology.height() {
        for x in 0..topology.width() {
            let node = topology.node_at(x, y).expect("dense cell");
            let parity = ((x + y) % 2) as u32;
            assert_eq!(commit.assignment[node.index()], commit.assignment[0] ^ parity, "checkerboard parity at ({x}, {y})");
        }
    }
}

#[test]
fn wfc_job_drives_a_grid_3d_topology_to_a_valid_commit() {
    let (model, relations) = checkerboard_tiles_3d();
    let topology = Grid3dTopology::new(4, 4, 4, &Stencil3d::Face6, relations, Boundary::Open, Boundary::Open, Boundary::Open, None).expect("3d topology");
    let op = operation(9_007);
    let mut job = WfcJob::new(op, model, topology.clone(), WfcJobConfig::default(), None, Vec::new());
    let commit = solve(&mut job, op, 8);
    assert_eq!(commit.assignment.len(), topology.node_count());
    for z in 0..topology.depth() {
        for y in 0..topology.height() {
            for x in 0..topology.width() {
                let node = topology.node_at(x, y, z).expect("dense cell");
                let parity = ((x + y + z) % 2) as u32;
                assert_eq!(commit.assignment[node.index()], commit.assignment[0] ^ parity, "checkerboard parity at ({x}, {y}, {z})");
            }
        }
    }
}

/// 🧪️ The 4×4 two-colour sample: 2×2 blocks of one colour, so a window size of 2 sees a small,
/// richly-overlapping pattern universe.
fn sample_4x4() -> Sample2d {
    let rows = [[0u32, 0, 1, 1], [0, 0, 1, 1], [1, 1, 0, 0], [1, 1, 0, 0]];
    let tiles = rows.iter().flat_map(|row| row.iter().map(|&colour| TileId(colour))).collect();
    Sample2d::new(4, 4, tiles)
}

/// 🧪️ Every periodic `window × window` view of the sample, deduplicated.
fn sample_windows(sample: &Sample2d, window: usize) -> Vec<Vec<TileId>> {
    let mut windows = Vec::new();
    for y in 0..sample.height {
        for x in 0..sample.width {
            let mut cells = Vec::with_capacity(window * window);
            for wy in 0..window {
                for wx in 0..window {
                    cells.push(sample.tiles[((y + wy) % sample.height) * sample.width + (x + wx) % sample.width]);
                }
            }
            if !windows.contains(&cells) {
                windows.push(cells);
            }
        }
    }
    windows
}

#[test]
fn extract_2d_output_is_locally_similar_to_its_sample() {
    const WINDOW: usize = 2;
    const SIDE: usize = 8;
    let sample = sample_4x4();
    let extracted = extract_2d(&[sample.clone()], &Extract2dConfig { window: WINDOW, periodic_input: true, symmetry: SymmetryGroup2d::None }).expect("overlapping extraction");
    let relations = declare_stencil_relations(&mut ModelBuilder::new(), &Stencil2d::VonNeumann).expect("extraction relation order");
    let topology = Grid2dTopology::new(SIDE, SIDE, &Stencil2d::VonNeumann, relations, Boundary::Wrap, Boundary::Wrap, None).expect("periodic output topology");
    let op = operation(31_337);
    let mut job = WfcJob::new(op, extracted.model.clone(), topology.clone(), WfcJobConfig::default(), None, Vec::new());
    let commit = solve(&mut job, op, 16);
    assert_eq!(commit.assignment.len(), SIDE * SIDE);

    let patterns: Vec<PatternId> = commit.assignment.iter().map(|&pattern| PatternId(pattern)).collect();
    let bitmap = extracted.decoder.decode(&patterns);
    assert_eq!(bitmap.len(), SIDE * SIDE);
    assert!(bitmap.iter().any(|&tile| tile == TileId(0)) && bitmap.iter().any(|&tile| tile == TileId(1)), "output uses both sample colours");

    let allowed = sample_windows(&sample, WINDOW);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let mut cells = Vec::with_capacity(WINDOW * WINDOW);
            for wy in 0..WINDOW {
                for wx in 0..WINDOW {
                    cells.push(bitmap[((y + wy) % SIDE) * SIDE + (x + wx) % SIDE]);
                }
            }
            assert!(allowed.contains(&cells), "output window at ({x}, {y}) does not occur in the sample");
        }
    }
}
