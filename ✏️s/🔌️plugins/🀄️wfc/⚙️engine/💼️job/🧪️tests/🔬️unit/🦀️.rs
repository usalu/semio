use std::time::{Duration, Instant};

use semio_framework_job::{allocate_operation_id, Generation, JobOutcomeBorrow, RevisionId};
use semio_s_plugin_wfc_engine_oracle::step_context::{TestJobDriver, TEST_SOURCE};

use super::*;
use crate::model::ModelBuilder;
use crate::topology::{GraphTopology, GraphTopologyBuilder};

const PUBLICATION_STEP_BOUND: usize = semio_framework_job_payload::JOB_PAYLOAD_OPERATION_PAGES + 64;

#[derive(Debug, PartialEq, Eq)]
enum Seen {
    Yield,
    Preview(Vec<u8>),
    Checkpoint(Vec<u8>),
    Fault(Vec<u8>),
    Cancelled,
    Complete { state: Vec<u8>, output: Vec<u8> },
}

impl Seen {
    fn is_terminal(&self) -> bool {
        matches!(self, Seen::Fault(_) | Seen::Cancelled | Seen::Complete { .. })
    }
}

fn seen_of(result: Option<&JobOutcomeBorrow<'_>>) -> Seen {
    match result {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Seen::Yield,
        Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => Seen::Preview(payload_bytes(payload)),
        Some(JobOutcomeBorrow::CheckpointReady { state, .. }) => Seen::Checkpoint(payload_bytes(state)),
        Some(JobOutcomeBorrow::Fault { detail, .. }) => Seen::Fault(payload_bytes(detail)),
        Some(JobOutcomeBorrow::Cancelled { .. }) => Seen::Cancelled,
        Some(JobOutcomeBorrow::Complete { state, output, .. }) => Seen::Complete { state: state.map(payload_bytes).unwrap_or_default(), output: output.map(payload_bytes).unwrap_or_default() },
    }
}

fn assert_fault(outcome: Seen, expected: &[u8]) {
    assert_eq!(outcome, Seen::Fault(expected.to_vec()));
}

fn checkerboard(nodes: usize, seed: u64) -> WfcJob<GraphTopology> {
    let mut model = ModelBuilder::new();
    let black = model.add_pattern(1.0);
    let white = model.add_pattern(2.0);
    let adjacent = model.add_relation("adjacent");
    model.allow_mirrored(adjacent, black, white);
    let model = model.compile().expect("model");
    let mut topology = GraphTopologyBuilder::new(nodes);
    for node in 0..nodes.saturating_sub(1) {
        topology.arc(NodeId::from_index(node), NodeId::from_index(node + 1), adjacent);
        topology.arc(NodeId::from_index(node + 1), NodeId::from_index(node), adjacent);
    }
    let operation = Operation::new(allocate_operation_id(), RevisionId(3), Generation(5), seed);
    WfcJob::new(operation, model, topology.build().expect("topology"), WfcJobConfig::default(), None, Vec::new())
}

fn run_step<J: InteractiveJob>(job: &mut J, driver: &mut TestJobDriver, fuel: u64) -> Seen {
    driver.step(job, fuel, seen_of)
}

fn close_job<J: InteractiveJob>(job: &mut J, driver: &mut TestJobDriver) {
    driver.close(job);
}

fn test_headless_site() -> HeadlessSite {
    HeadlessSite { site: "wfc.checkpoint.restore.test", label: "checkpoint-restore-test", stage: semio_framework_job::InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 2000, retained_stock: TEST_SOURCE, lifetime_fuel: 100_000_000, lifetime_budget_us: 60_000_000 }
}

fn drive(job: &mut WfcJob<GraphTopology>, driver: &mut TestJobDriver, fuel: u64) -> Seen {
    for _ in 0..60_000_000 {
        let outcome = run_step(job, driver, fuel);
        if outcome.is_terminal() { return outcome; }
    }
    panic!("WFC job did not terminate");
}

fn checkpoint(job: &mut WfcJob<GraphTopology>, driver: &mut TestJobDriver) -> Vec<u8> {
    for _ in 0..60_000_000 {
        let outcome = run_step(job, driver, 1);
        if let Seen::Checkpoint(bytes) = outcome { return bytes; }
        assert!(!outcome.is_terminal(), "job terminated before checkpoint");
    }
    panic!("WFC job did not checkpoint");
}

fn terminal_checkpoint(job: &mut WfcJob<GraphTopology>, driver: &mut TestJobDriver) -> Vec<u8> {
    for _ in 0..60_000_000 {
        if let Seen::Complete { state, .. } = run_step(job, driver, 1) { return state; }
    }
    panic!("WFC job did not complete");
}

fn prepare_maximum_checkpoint_state(job: &mut WfcJob<GraphTopology>, driver: &mut TestJobDriver) -> usize {
    let _ = terminal_checkpoint(job, driver);
    job.state.trail.clear();
    job.state.decisions.clear();
    job.state.observed.clear();
    job.state.observations = 0;
    let base_bytes = CheckpointCounts::from_state(&job.state, job.model.pattern_count()).checked_bytes().expect("base checkpoint size");
    let remaining = MAX_CHECKPOINT_BYTES.checked_sub(base_bytes).expect("maximum admits base checkpoint");
    assert_eq!(remaining % CHECKPOINT_OBSERVED_ENTRY_BYTES, 0);
    let observed_count = remaining / CHECKPOINT_OBSERVED_ENTRY_BYTES;
    job.state.observations = u64::try_from(observed_count).expect("observation count");
    job.state.observed = vec![(NodeId(0), PatternId(0)); observed_count];
    assert_eq!(CheckpointCounts::from_state(&job.state, job.model.pattern_count()).checked_bytes(), Some(MAX_CHECKPOINT_BYTES));
    observed_count
}

fn maximum_checkpoint(job: &mut WfcJob<GraphTopology>, driver: &mut TestJobDriver) -> Vec<u8> {
    prepare_maximum_checkpoint_state(job, driver);
    job.begin_checkpoint(true).expect("maximum checkpoint build");
    assert_eq!(job.checkpoint_build.as_ref().expect("checkpoint build").byte_limit, MAX_CHECKPOINT_BYTES);
    for _ in 0..60_000_000 {
        if let Some(bytes) = job.checkpoint_one().expect("maximum checkpoint unit") {
            assert_eq!(bytes.len(), MAX_CHECKPOINT_BYTES);
            return bytes;
        }
    }
    panic!("maximum WFC checkpoint did not materialize");
}

#[test]
fn batch_size_and_replay_are_deterministic() {
    let mut one = checkerboard(127, 19);
    let mut one_driver = TestJobDriver::new(one.operation.operation, one.operation.generation);
    let mut many = checkerboard(127, 19);
    let mut many_driver = TestJobDriver::new(many.operation.operation, many.operation.generation);
    assert!(matches!(drive(&mut one, &mut one_driver, 1), Seen::Complete { .. }));
    assert!(matches!(drive(&mut many, &mut many_driver, 64), Seen::Complete { .. }));
    assert_eq!(one.commit(), many.commit());
    close_job(&mut one, &mut one_driver);
    close_job(&mut many, &mut many_driver);
}

#[test]
fn checkpoint_resume_preserves_rng_trail_and_progress() {
    let mut original = checkerboard(41, 71);
    let mut original_driver = TestJobDriver::new(original.operation.operation, original.operation.generation);
    let bytes = checkpoint(&mut original, &mut original_driver);
    let mut restored = WfcJob::from_checkpoint(original.operation, original.model.clone(), original.topology.clone(), original.config, original.initial_domains.clone(), original.fixed.clone(), &bytes, semio_framework_job::logical_now_us, test_headless_site()).expect("restore");
    let mut restored_driver = TestJobDriver::new(restored.operation.operation, restored.operation.generation);
    assert!(matches!(drive(&mut original, &mut original_driver, 3), Seen::Complete { .. }));
    assert!(matches!(drive(&mut restored, &mut restored_driver, 11), Seen::Complete { .. }));
    assert_eq!(original.commit(), restored.commit());
    close_job(&mut original, &mut original_driver);
    close_job(&mut restored, &mut restored_driver);
}

#[test]
fn checkpoint_restore_rejects_foreign_operation_and_topology() {
    let mut original = checkerboard(11, 71);
    let mut original_driver = TestJobDriver::new(original.operation.operation, original.operation.generation);
    let bytes = checkpoint(&mut original, &mut original_driver);
    let foreign_operation = Operation::new(allocate_operation_id(), original.operation.base_revision, original.operation.generation, original.operation.seed);
    assert!(WfcJob::from_checkpoint(foreign_operation, original.model.clone(), original.topology.clone(), original.config, original.initial_domains.clone(), original.fixed.clone(), &bytes, semio_framework_job::logical_now_us, test_headless_site()).is_err());
    let foreign_topology = GraphTopologyBuilder::new(12).build().expect("foreign topology");
    assert!(WfcJob::from_checkpoint(original.operation, original.model.clone(), foreign_topology, original.config, original.initial_domains.clone(), original.fixed.clone(), &bytes, semio_framework_job::logical_now_us, test_headless_site()).is_err());
    close_job(&mut original, &mut original_driver);
}

#[test]
fn previews_report_monotonic_sequences_and_progress() {
    let mut job = checkerboard(31, 83);
    let mut job_driver = TestJobDriver::new(job.operation.operation, job.operation.generation);

    let mut previews = Vec::new();
    loop {

        let outcome = run_step(&mut job, &mut job_driver, 1);
        let preview = match &outcome {
            Seen::Preview(bytes) => Some(semio_framework_pack_json::from_json_str::<WfcPreview>(std::str::from_utf8(bytes).expect("preview UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject)),
            _ => None,
        };
        if let Some(preview) = preview {
            previews.push(preview.expect("preview"));
        }
        if outcome.is_terminal() {
            break;
        }
    }
    assert!(!previews.is_empty());
    assert!(previews.windows(2).all(|pair| pair[1].sequence == pair[0].sequence + 1 && pair[1].observations >= pair[0].observations && pair[1].compatibility_edges >= pair[0].compatibility_edges && pair[1].backtracks >= pair[0].backtracks));
    close_job(&mut job, &mut job_driver);
}

#[test]
fn every_interactive_solver_stage_is_preview_eligible_at_fixed_cadence() {
    let mut job = checkerboard(17, 83);
    let mut job_driver = TestJobDriver::new(job.operation.operation, job.operation.generation);
    for stage in [WfcStage::InitializeDomains, WfcStage::FindMinimumEntropySlot, WfcStage::ChooseCandidate, WfcStage::PropagateCompatibilityEdge, WfcStage::DetectContradiction, WfcStage::BacktrackTrailEntry] {
        job.state.stage = stage;
        job.preview_units = PREVIEW_UNIT_INTERVAL;
        job.last_preview_ms = Some(0);
        assert!(job.preview_stage());
        assert!(job.preview_due(0));
    }
    close_job(&mut job, &mut job_driver);
}

#[test]
fn first_preview_and_continuous_gap_include_bounded_publication() {
    let mut job = checkerboard(4_096, 83);
    let mut job_driver = TestJobDriver::new(job.operation.operation, job.operation.generation);

    let mut units_since_preview = 0;
    let mut preview_count = 0;
    for _ in 0..100_000 {

        units_since_preview += 1;
        let outcome = run_step(&mut job, &mut job_driver, 1);
        match outcome {
            Seen::Preview(_) => {
                assert!(units_since_preview <= PREVIEW_UNIT_INTERVAL as usize + PUBLICATION_STEP_BOUND);
                units_since_preview = 0;
                preview_count += 1;
                if preview_count == 64 {
                    break;
                }
            }
            outcome if outcome.is_terminal() => break,
            _ => {}
        }
    }
    assert_eq!(preview_count, 64);
    close_job(&mut job, &mut job_driver);
}

#[test]
fn uniform_sampling_consumes_exactly_one_rng_word() {
    let mut ranged = JobRng::from_seed(0x5eed);
    let mut direct = ranged;
    let value = ranged.range(u64::MAX - 58);
    let _ = direct.next_u64();
    assert!(value < u64::MAX - 58);
    assert_eq!(ranged.state, direct.state);
}

#[test]
fn checkpoint_resume_preserves_preview_sequence() {
    let mut job = checkerboard(31, 89);
    let mut job_driver = TestJobDriver::new(job.operation.operation, job.operation.generation);
    job.topology = GraphTopologyBuilder::new(31).build().expect("disjoint topology");
    let bytes = checkpoint(&mut job, &mut job_driver);
    let previous = job.operation.preview_sequence;
    let mut restored = WfcJob::from_checkpoint(job.operation, job.model.clone(), job.topology.clone(), job.config, job.initial_domains.clone(), job.fixed.clone(), &bytes, semio_framework_job::logical_now_us, test_headless_site()).expect("restore");
    let mut restored_driver = TestJobDriver::new(restored.operation.operation, restored.operation.generation);

    let resumed = loop {

        let outcome = run_step(&mut restored, &mut restored_driver, 1);
        let preview = match &outcome {
            Seen::Preview(bytes) => Some(semio_framework_pack_json::from_json_str::<WfcPreview>(std::str::from_utf8(bytes).expect("preview UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject)),
            _ => None,
        };
        if let Some(preview) = preview {
            break preview.expect("preview");
        }
    };
    assert_eq!(resumed.sequence + 1, previous + 1);
    close_job(&mut job, &mut job_driver);
    close_job(&mut restored, &mut restored_driver);
}

#[test]
fn disjoint_and_adversarial_graphs_finish() {
    let mut disjoint = checkerboard(0, 1);
    let mut disjoint_driver = TestJobDriver::new(disjoint.operation.operation, disjoint.operation.generation);
    assert!(matches!(drive(&mut disjoint, &mut disjoint_driver, 1), Seen::Complete { .. }));
    let mut long = checkerboard(4_096, 2);
    let mut long_driver = TestJobDriver::new(long.operation.operation, long.operation.generation);
    assert!(matches!(drive(&mut long, &mut long_driver, 32), Seen::Complete { .. }));
    assert_eq!(long.commit().expect("commit").assignment.len(), 4_096);
    close_job(&mut disjoint, &mut disjoint_driver);
    close_job(&mut long, &mut long_driver);
}

#[test]
fn cancellation_and_generation_freshness_do_not_mutate_progress() {
    let mut job = checkerboard(10, 3);
    let mut job_driver = TestJobDriver::new(job.operation.operation, job.operation.generation);
    let before = job.metrics();
    let cancelled = job_driver.turn(|cx| {
        cx.observe_original_cancellation()?;
        Ok(seen_of(job.step_value(cx)?.as_ref()))
    }).expect("original cancellation observation");
    assert_eq!(cancelled, Seen::Cancelled);
    assert_eq!(job.metrics(), before);
    close_job(&mut job, &mut job_driver);
    let mut stale_job = checkerboard(10, 3);
    let before = stale_job.metrics();
    let mut stale_driver = TestJobDriver::new(stale_job.operation.operation, Generation(stale_job.operation.generation.0 + 1));
    let mut faulted = false;
    for _ in 0..PUBLICATION_STEP_BOUND {
        if matches!(run_step(&mut stale_job, &mut stale_driver, 10), Seen::Fault(_)) {
            faulted = true;
            break;
        }
    }
    assert!(faulted, "a stale generation publishes its fault within the bounded publication");
    assert_eq!(stale_job.metrics(), before);
    close_job(&mut stale_job, &mut stale_driver);
}

#[test]
fn cancellation_interrupts_checkpoint_and_commit_materialization_without_progress() {
    let mut job = checkerboard(31, 97);
    let mut job_driver = TestJobDriver::new(job.operation.operation, job.operation.generation);

    let mut checked_checkpoint = false;
    let mut checked_commit = false;
    for _ in 0..60_000_000 {
        let stage = job.state.stage;
        if matches!(stage, WfcStage::MaterializeCheckpoint | WfcStage::MaterializeCommit) {
            let before = match stage {
                WfcStage::MaterializeCheckpoint => job.checkpoint_build.as_ref().map(|build| build.bytes.len()).unwrap_or(0),
                WfcStage::MaterializeCommit => job.commit_build.as_ref().map(|build| build.cursor).unwrap_or(0),
                _ => unreachable!(),
            };
            let cancelled = job_driver.turn(|cx| {
                cx.observe_original_cancellation()?;
                Ok(seen_of(job.step_value(cx)?.as_ref()))
            }).expect("original materialization cancellation");
            assert_eq!(cancelled, Seen::Cancelled);
            let after = match stage {
                WfcStage::MaterializeCheckpoint => job.checkpoint_build.as_ref().map(|build| build.bytes.len()).unwrap_or(0),
                WfcStage::MaterializeCommit => job.commit_build.as_ref().map(|build| build.cursor).unwrap_or(0),
                _ => unreachable!(),
            };
            assert_eq!(before, after);
            checked_checkpoint |= stage == WfcStage::MaterializeCheckpoint;
            checked_commit |= stage == WfcStage::MaterializeCommit;
        }

        if run_step(&mut job, &mut job_driver, 1).is_terminal() {
            break;
        }
    }
    assert!(checked_checkpoint && checked_commit);
    close_job(&mut job, &mut job_driver);
}

#[test]
fn maximum_checkpoint_restore_is_bounded_and_cancellable_in_every_phase() {
    let mut source = checkerboard(1, 101);
    let mut source_driver = TestJobDriver::new(source.operation.operation, source.operation.generation);
    let bytes = maximum_checkpoint(&mut source, &mut source_driver);
    let mut restore = WfcRestore::new(source.operation, source.model.clone(), source.topology.clone(), source.config, None, Vec::new(), bytes).expect("maximum admitted restore");
    let mut restore_driver = TestJobDriver::new(restore.operation.operation, restore.operation.generation);
    assert!(restore.domains.is_empty());
    assert!(restore.domain_counts.is_empty());
    assert!(restore.entropy_heap.is_empty());
    let mut cancelled = Vec::new();

    for _ in 0..60_000_000 {
        let stage = restore.stage;
        if !cancelled.contains(&stage) {
            let before = (restore.cursor, restore.domains.len(), restore.trail.len(), restore.decisions.len(), restore.observed.len(), restore.domain_cursor, restore.restored.is_some());
            let observed = restore_driver.turn(|cx| {
                cx.observe_original_cancellation()?;
                Ok(seen_of(restore.step_value(cx)?.as_ref()))
            }).expect("original restore cancellation");
            assert_eq!(observed, Seen::Cancelled);
            let after = (restore.cursor, restore.domains.len(), restore.trail.len(), restore.decisions.len(), restore.observed.len(), restore.domain_cursor, restore.restored.is_some());
            assert_eq!(before, after);
            cancelled.push(stage);
        }

        if matches!(run_step(&mut restore, &mut restore_driver, 1), Seen::Complete { .. }) {
            break;
        }
    }
    assert_eq!(cancelled, vec![RestoreStage::Header, RestoreStage::Domains, RestoreStage::Trail, RestoreStage::Decisions, RestoreStage::Observed, RestoreStage::Verify, RestoreStage::Rebuild, RestoreStage::Complete]);
    let mut restored = restore.take_job().expect("the completed restore yields its job");
    let mut restored_driver = TestJobDriver::new(restored.operation.operation, restored.operation.generation);
    close_job(&mut restored, &mut restored_driver);
    close_job(&mut restore, &mut restore_driver);
    close_job(&mut source, &mut source_driver);
}

#[test]
fn checkpoint_admission_rejects_one_byte_over_the_fixed_maximum() {
    let mut source = checkerboard(1, 103);
    let mut source_driver = TestJobDriver::new(source.operation.operation, source.operation.generation);
    let bytes = vec![0; MAX_CHECKPOINT_BYTES.checked_add(1).expect("maximum plus one")];
    assert!(matches!(WfcRestore::new(source.operation, source.model.clone(), source.topology.clone(), source.config, None, Vec::new(), bytes), Err(error) if error == "wfc-checkpoint-admission-exceeded"));
    close_job(&mut source, &mut source_driver);
}

#[test]
fn minimum_checkpoint_is_exactly_the_fixed_header_and_restores() {
    let mut source = checkerboard(0, 105);
    let mut source_driver = TestJobDriver::new(source.operation.operation, source.operation.generation);
    let bytes = terminal_checkpoint(&mut source, &mut source_driver);
    assert_eq!(bytes.len(), CHECKPOINT_FIXED_HEADER_BYTES);
    assert_eq!(CheckpointCounts::from_state(&source.state, source.model.pattern_count()).checked_bytes(), Some(CHECKPOINT_FIXED_HEADER_BYTES));
    let mut restored = WfcJob::from_checkpoint(source.operation, source.model.clone(), source.topology.clone(), source.config, None, Vec::new(), &bytes, semio_framework_job::logical_now_us, test_headless_site()).expect("minimum checkpoint restore");
    let mut restored_driver = TestJobDriver::new(restored.operation.operation, restored.operation.generation);
    assert!(restored.state.domains.is_empty());
    close_job(&mut restored, &mut restored_driver);
    close_job(&mut source, &mut source_driver);
}

#[test]
fn checkpoint_restore_rejects_size_arithmetic_overflow() {
    let mut source = checkerboard(0, 106);
    let mut source_driver = TestJobDriver::new(source.operation.operation, source.operation.generation);
    let mut bytes = terminal_checkpoint(&mut source, &mut source_driver);
    let observed_count_offset = CHECKPOINT_FIXED_HEADER_BYTES.checked_sub(size_of::<u64>()).expect("observed count offset");
    bytes[observed_count_offset..CHECKPOINT_FIXED_HEADER_BYTES].copy_from_slice(&u64::MAX.to_le_bytes());
    let mut restore = WfcRestore::new(source.operation, source.model.clone(), source.topology.clone(), source.config, None, Vec::new(), bytes).expect("admitted overflow fixture");
    let mut restore_driver = TestJobDriver::new(restore.operation.operation, restore.operation.generation);

    let outcome = (0..PUBLICATION_STEP_BOUND).find_map(|_| {

        Some(run_step(&mut restore, &mut restore_driver, 1)).filter(Seen::is_terminal)
    });
    assert_fault(outcome.expect("the fault is published within the bounded publication"), b"wfc-checkpoint-capacity");
    close_job(&mut restore, &mut restore_driver);
    close_job(&mut source, &mut source_driver);
    assert_eq!(CheckpointCounts { domain_count: usize::MAX, pattern_count: usize::MAX, trail_count: usize::MAX, decision_count: usize::MAX, observed_count: usize::MAX }.checked_bytes(), None);
}

#[test]
fn maximum_admitted_checkpoint_and_commit_allocation_stay_below_watchdog() {
    let mut source = checkerboard(1, 107);
    let mut source_driver = TestJobDriver::new(source.operation.operation, source.operation.generation);
    prepare_maximum_checkpoint_state(&mut source, &mut source_driver);
    let pressure = vec![vec![0u8; 4_096]; 64];
    let start = Instant::now();
    let checkpoint = CheckpointBuild::new(&source.state, source.model.pattern_count(), true).expect("maximum checkpoint allocation");
    let checkpoint_elapsed = start.elapsed();
    let start = Instant::now();
    let mut commit = CommitBuild::new(MAX_COMMIT_ITEMS).expect("maximum commit allocation");
    let commit_elapsed = start.elapsed();
    assert!(checkpoint.bytes.capacity() >= MAX_CHECKPOINT_BYTES);
    assert_eq!(checkpoint.byte_limit, MAX_CHECKPOINT_BYTES);
    assert!(commit.bytes.capacity() >= COMMIT_FIXED_MAX_BYTES + MAX_COMMIT_ITEMS * COMMIT_ITEM_MAX_BYTES);
    assert!(commit.byte_limit <= MAX_COMMIT_BYTES);
    assert!(commit.assignment.capacity() >= MAX_COMMIT_ITEMS);
    let assignment_capacity = commit.assignment.capacity();
    for _ in 0..MAX_COMMIT_ITEMS {
        commit.assignment.push(0);
    }
    assert_eq!(commit.assignment.len(), MAX_COMMIT_ITEMS);
    assert_eq!(commit.assignment.capacity(), assignment_capacity, "the exact-maximum lossless side vector must never grow while materializing");
    assert!(CommitBuild::new(MAX_COMMIT_ITEMS + 1).is_err(), "one item over the maximum must be refused admission");
    source.state.observed.push((NodeId(0), PatternId(0)));
    assert!(CheckpointBuild::new(&source.state, source.model.pattern_count(), true).is_err(), "one observation over the maximum must be refused admission");
    assert!(checkpoint_elapsed < Duration::from_millis(8), "maximum checkpoint allocation exceeded watchdog: {checkpoint_elapsed:?}");
    assert!(commit_elapsed < Duration::from_millis(8), "maximum commit allocation exceeded watchdog: {commit_elapsed:?}");
    assert_eq!(pressure.len(), 64);
    close_job(&mut source, &mut source_driver);
}

#[test]
fn every_large_domain_unit_including_checkpoint_stays_below_watchdog() {
    let mut job = checkerboard(8_192, 4);
    let mut job_driver = TestJobDriver::new(job.operation.operation, job.operation.generation);

    let mut samples = Vec::new();
    let mut saw_checkpoint = false;
    for _ in 0..6_000_000 {

        let start = Instant::now();
        let outcome = run_step(&mut job, &mut job_driver, 1);
        samples.push(start.elapsed());
        saw_checkpoint |= matches!(outcome, Seen::Checkpoint(_));
        if outcome.is_terminal() {
            break;
        }
    }
    let ceiling = Duration::from_micros(semio_framework_job::INTERACTIVE_STEP_CEILING_US);
    let sustained = samples.iter().fold((0u32, 0u32), |(run, longest), sample| if *sample >= ceiling { (run + 1, longest.max(run + 1)) } else { (0, longest) }).1;
    assert!(sustained < semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS, "WFC units overran the {ceiling:?} watchdog ceiling {sustained} times in a row, the framework's quarantine law");
    samples.sort_unstable();
    let p99 = samples[samples.len() * 99 / 100];
    assert!(saw_checkpoint);
    assert!(p99 < Duration::from_millis(2), "WFC unit p99 exceeded 2 ms: {p99:?}");
    close_job(&mut job, &mut job_driver);
}
