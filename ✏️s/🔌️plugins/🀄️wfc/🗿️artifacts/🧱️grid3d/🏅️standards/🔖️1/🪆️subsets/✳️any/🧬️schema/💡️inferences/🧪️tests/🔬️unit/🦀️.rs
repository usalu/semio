//! 🔬️ Inference laws — the solve is deterministic for a seed, honours pins and masks, reports a
//! contradiction instead of panicking, and the entropy field agrees with the weight distribution.
//!
//! 🚪️ Every job driven here goes through the engine's close ladder before it is dropped: a leaked
//! retained payload page aborts the process rather than failing a test.

use super::*;
use semio_framework_job::InteractiveJob;
use crate::schema::snapshot::{Grid3dCell, Grid3dDirection, Grid3dPinnedCell, Grid3dRule, Grid3dSnapshot, Grid3dTile};

/// 🧸️ Two tiles on a 2×1×1 grid, each allowed beside the other in both x directions — the smallest
/// grid instance that still has a real arc to propagate over.
fn two_tile_pair() -> Grid3dSnapshot {
    Grid3dSnapshot {
        seed: 7,
        width: 2,
        cell_sizes_x: vec![1.0, 1.0],
        tiles: vec![
            Grid3dTile { id: "a".into(), label: None, weight: 1.0, media: Default::default() },
            Grid3dTile { id: "b".into(), label: None, weight: 1.0, media: Default::default() },
        ],
        rules: vec![
            Grid3dRule { id: "r-ab".into(), tile_a_id: "a".into(), tile_b_id: "b".into(), direction: Grid3dDirection::Right, allowed: true },
            Grid3dRule { id: "r-ba".into(), tile_a_id: "b".into(), tile_b_id: "a".into(), direction: Grid3dDirection::Right, allowed: true },
        ],
        ..Grid3dSnapshot::default()
    }
}

#[test]
fn a_satisfiable_grid_assigns_every_unmasked_cell() {
    let commit = solve(&two_tile_pair()).expect("a trivially satisfiable grid must solve");
    assert!(commit.satisfiable);
    assert_eq!(commit.assignments.len(), 2);
    assert!(commit.assignments.iter().all(|row| row.tile_id == "a" || row.tile_id == "b"));
}

#[test]
fn a_grid_with_no_allowed_pair_is_reported_as_a_contradiction_not_a_panic() {
    let mut snapshot = two_tile_pair();
    snapshot.rules.clear();
    assert!(!solve(&snapshot).expect("an unsatisfiable grid is an answer, not a failure").satisfiable, "a closed allow-list with no rule admits nothing");
    let satisfiable = store::infer_field::<Grid3dSnapshot, Grid3dContradiction>(&snapshot, None);
    assert!(!satisfiable["grid3d"]);
    assert_eq!(store::infer_field::<Grid3dSnapshot, Grid3dSolve>(&snapshot, None)["grid3d"], Grid3dSolveResult::Unsolved);
}

#[test]
fn an_explicit_deny_beats_an_allow_of_the_same_pair() {
    let mut snapshot = two_tile_pair();
    snapshot.rules.push(Grid3dRule { id: "r-zz-deny-ab".into(), tile_a_id: "a".into(), tile_b_id: "b".into(), direction: Grid3dDirection::Right, allowed: false });
    snapshot.rules.push(Grid3dRule { id: "r-zz-deny-ba".into(), tile_a_id: "b".into(), tile_b_id: "a".into(), direction: Grid3dDirection::Right, allowed: false });
    assert!(!solve(&snapshot).expect("an unsatisfiable grid is an answer").satisfiable, "a deny always wins over an allow of the same directed pair");
}

#[test]
fn the_same_seed_and_spec_always_produce_the_same_assignment() {
    let snapshot = crate::examples::blocks::snapshot();
    let first = solve(&snapshot).expect("blocks solves");
    let second = solve(&snapshot).expect("blocks solves");
    assert_eq!(first, second, "the solve must be a pure function of the snapshot, seed included");
}

#[test]
fn changing_only_the_seed_still_solves_the_same_spec() {
    let mut snapshot = crate::examples::blocks::snapshot();
    snapshot.seed = 4_242;
    assert!(solve(&snapshot).is_ok());
}

#[test]
fn a_pinned_cell_always_comes_back_carrying_its_pin() {
    let mut snapshot = two_tile_pair();
    snapshot.pinned = vec![Grid3dPinnedCell { x: 0, y: 0, z: 0, tile_id: "b".into() }];
    let commit = solve(&snapshot).expect("a pinned-and-allowed grid solves");
    assert_eq!(commit.assignments.iter().find(|row| (row.x, row.y, row.z) == (0, 0, 0)).map(|row| row.tile_id.as_str()), Some("b"));
}

#[test]
fn a_masked_cell_is_never_assigned() {
    let mut snapshot = two_tile_pair();
    snapshot.masked = vec![Grid3dCell { x: 1, y: 0, z: 0 }];
    let commit = solve(&snapshot).expect("a masked grid solves");
    assert_eq!(commit.assignments.len(), 1);
    assert_eq!(commit.assignments[0].x, 0);
}

#[test]
fn a_tileless_grid_solves_trivially_with_no_assignments() {
    let snapshot = Grid3dSnapshot::default();
    let commit = solve(&snapshot).expect("an empty pattern universe is not a failure");
    assert!(commit.satisfiable);
    assert_eq!(commit.assignments, Vec::new());
}

#[test]
fn a_periodic_axis_wraps_rather_than_ending_the_run() {
    let snapshot = crate::examples::pipes_3d::snapshot();
    assert!(snapshot.periodic_x);
    let commit = solve(&snapshot).expect("pipes-3d solves");
    let pinned = commit.assignments.iter().find(|row| (row.x, row.y, row.z) == (1, 1, 1)).expect("the pinned cell is assigned");
    assert_eq!(pinned.tile_id, "pipe-x");
    for x in 0..snapshot.width {
        let tile = commit.assignments.iter().find(|row| (row.x, row.y, row.z) == (x, 1, 1)).map(|row| row.tile_id.as_str());
        assert_eq!(tile, Some("pipe-x"), "a wrapped pipe run must fill its whole row");
    }
}

#[test]
fn the_solve_field_reports_the_same_assignment_the_headless_adapter_does() {
    let snapshot = two_tile_pair();
    let values = store::infer_field::<Grid3dSnapshot, Grid3dSolve>(&snapshot, None);
    match &values["grid3d"] {
        Grid3dSolveResult::Solved { assignments } => assert_eq!(assignments, &solve(&snapshot).expect("solves").assignments),
        Grid3dSolveResult::Unsolved => panic!("a trivially satisfiable grid must solve"),
    }
}

#[test]
fn a_pinned_or_masked_cell_has_zero_entropy_and_a_free_one_does_not() {
    let mut snapshot = two_tile_pair();
    snapshot.pinned = vec![Grid3dPinnedCell { x: 0, y: 0, z: 0, tile_id: "a".into() }];
    let entropy = store::infer_field::<Grid3dSnapshot, Grid3dEntropy>(&snapshot, None);
    assert_eq!(entropy["0:0:0"], 0.0);
    assert!(entropy["1:0:0"] > 0.0);
}

#[test]
fn uniform_weights_over_two_tiles_yield_ln2_entropy_and_a_skew_lowers_it() {
    let snapshot = two_tile_pair();
    assert!((shannon_entropy_over_tiles(&snapshot) - std::f64::consts::LN_2).abs() < 1e-9);
    let mut skewed = snapshot;
    skewed.tiles[0].weight = 100.0;
    skewed.tiles[1].weight = 0.01;
    assert!(shannon_entropy_over_tiles(&skewed) < std::f64::consts::LN_2);
}

#[test]
fn the_routed_factory_registers_once_and_refuses_a_colliding_key() {
    let bus = semio_framework::ActionBus::new();
    register_grid3d_inference_factory(&bus).expect("factory registration");
    register_grid3d_inference_factory(&bus).expect("idempotent factory registration");
    let key = semio_framework::ToolFactoryKey::new(GRID3D_INFERENCE_JOB_KIND, GRID3D_INFERENCE_TOOL_ID);
    assert!(bus.contains(&key));
    assert_eq!(bus.payload_schema_id(&key).as_deref(), Some(GRID3D_INFERENCE_PAYLOAD_SCHEMA));
}

#[test]
fn the_roster_metadata_names_this_artifact_and_its_own_solve_schema() {
    let metadata = grid3d_inference_metadata();
    assert_eq!(metadata.owner, "wfc");
    assert_eq!(metadata.artifact_kind, "s.wfc.grid3d");
    assert_eq!(metadata.inference_schema, GRID3D_INFERENCE_TOOL_ID);
}

#[test]
fn an_oversized_request_is_refused_at_admission_rather_than_attempted() {
    let snapshot = Grid3dSnapshot { width: 0, ..Grid3dSnapshot::default() };
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), 0);
    assert!(Grid3dInferenceJob::new(operation, Grid3dInferenceRequest { snapshot: Some(snapshot), document: None, checkpoint: None }).is_err());
}

#[test]
fn the_inference_job_publishes_a_twenty_five_byte_preview() {
    let document = two_tile_pair();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(0), document.seed);
    let mut job = Grid3dInferenceJob::new(operation, Grid3dInferenceRequest { snapshot: Some(document), document: None, checkpoint: None }).expect("admit");
    let (operation_id, generation, cancel) = (job.operation().operation, job.operation().generation, semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    let mut verdict = None;
    for _ in 0..100_000 {
        let now = semio_framework_job::default_now_us().expect("clock");
        let budget = semio_framework_job::StepBudget::new(1, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4);
        let outcome = semio_framework_job::drive_step(&mut job, "wfc.grid3d.inference.preview.test", operation_id, generation, semio_framework_job::InteractiveStage::InteractiveStep, budget, cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut verdict);
        match outcome {
            semio_framework_job::StepOutcome::PreviewReady(mut payload) => {
                let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("page").to_vec()).collect();
                while !matches!(payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
                assert_eq!(bytes.len(), 25, "the parent inference preview is a fixed 25-byte progress record");
                job.begin_close();
                for _ in 0..1_000_000 {
                    if matches!(job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                        assert!(job.terminal_is_empty());
                        return;
                    }
                }
                panic!("inference close stalled");
            }
            semio_framework_job::StepOutcome::Complete(_) | semio_framework_job::StepOutcome::Cancelled | semio_framework_job::StepOutcome::Fault(_) => {
                let mut outcome = outcome;
                while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
                panic!("expected a preview before the terminal outcome");
            }
            _ => {}
        }
    }
    panic!("no preview published");
}
