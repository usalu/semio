use super::*;
use crate::schema::snapshot::{AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge};
use semio_framework::ToolJobFactory as _;
use semio_framework_job::{allocate_operation_id, root_cancel_token, CommitValidation, Generation, InteractiveJob, Operation, RevisionId, StepBudget, StepContext, StepOutcome};
use semio_framework_plugin::app::{WireArtifactInferenceBudget, WireArtifactInferenceCacheMode, WireArtifactInferenceRequest, WireArtifactInferenceResult, ARTIFACT_INFERENCE_WIRE_VERSION};
use semio_framework_plugin::reactor::jobs::{cancel_job, checkpoint_jobs, restore_job, start_job, step_job as step_reactor_job, JobBudget, JobStep, JOB_KIND_INFER};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;

fn kit_child(id: &str) -> store::ArtifactChild<SemioKitSnapshot> {
    store::ArtifactChild::new(id.to_string(), store::os_io::ArtifactRef { artifact_id: id.to_string(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() } })
}

/// 🧸 Two slots, one edge, two modules ("a","b") mutually allowed to be adjacent — a WFC
/// instance small enough to solve deterministically by hand: any seed must find SOME solution.
fn two_slot_two_module_snapshot() -> AssemblySnapshot {
    let mut snapshot = AssemblySnapshot::default();
    snapshot.seed = 7;
    snapshot.slots = vec![AssemblySlot { id: "s1".into(), x: 0.0, y: 0.0, z: 0.0, pinned_module_id: None }, AssemblySlot { id: "s2".into(), x: 1.0, y: 0.0, z: 0.0, pinned_module_id: None }];
    snapshot.edges = vec![AssemblySlotEdge { id: "e1".into(), from_slot_id: "s1".into(), to_slot_id: "s2".into() }];
    snapshot.modules = vec![kit_child("a"), kit_child("b")];
    snapshot.rules = vec![AssemblyRule { id: "r1".into(), module_a_id: "a".into(), module_b_id: "b".into(), allowed: true, params: SemioValue::default() }];
    snapshot
}

fn operation(seed: u64) -> Operation {
    Operation::new(allocate_operation_id(), RevisionId(11), Generation(7), seed)
}

struct MountedCompetingJob;

impl InteractiveJob for MountedCompetingJob {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: context.payload_from_bytes(semio_framework_job::JobPayloadStream::CommitOutput, b"replacement").expect("competing fixture output credit"),
        })
    }

    fn begin_close(&mut self) {}

    fn close_step(&mut self, _: usize, _: usize) -> semio_framework_job::InteractiveJobCloseStep {
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        true
    }
}

struct MountedCompetingFactory {
    keys: [semio_framework::ToolFactoryKey; 1],
}

impl semio_framework::ToolJobFactory for MountedCompetingFactory {
    type Payload = AssemblyInferenceRequest;
    type Job = MountedCompetingJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        "competing.assembly.inference.v1"
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::bounded_first_step(1, 1, 1, 16, 100)
    }

    fn create_job(&mut self, _operation: Operation, _payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(MountedCompetingJob)
    }
}

fn cold_route_request(snapshot: AssemblySnapshot, cancellation_id: &str, revision: u64, generation: u64) -> Vec<u8> {
    let request = WireArtifactInferenceRequest {
        wire_version: ARTIFACT_INFERENCE_WIRE_VERSION,
        owner: "procedural".into(),
        artifact_kind: "s.procedural.assembly".into(),
        artifact_schema: "s.assembly".into(),
        artifact_schema_version: 1,
        document_schema: "s.assembly".into(),
        document_schema_version: 1,
        inference_schema: ASSEMBLY_INFERENCE_TOOL_ID.into(),
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        revision,
        generation,
        source_dialect: "s.assembly.standard.v1.dialect.canonical".into(),
        policy: Vec::new(),
        budgets: WireArtifactInferenceBudget { allocation_bytes: 4 << 20, work_units: 1, recursion_depth: 4 },
        cancellation_id: cancellation_id.into(),
        previous_state: None,
        requested_cache_mode: WireArtifactInferenceCacheMode::Cold,
        canonical_payload: dsl::json::to_json_string(&AssemblyInferenceRequest { snapshot, checkpoint: None }).into_bytes(),
        dependencies: Vec::new(),
    };
    dsl::json::to_json_string(&request).into_bytes()
}

async fn drive_cold_route(job: u64) -> WireArtifactInferenceResult {
    for _ in 0..200_000 {
        match step_reactor_job(job, JobBudget { fuel: 1, deadline_ms: 2 }).await {
            JobStep::Running(_) => {}
            JobStep::Done(bytes) => return dsl::json::from_json_str(std::str::from_utf8(&bytes).expect("cold route UTF-8")).expect("cold route result"),
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                panic!("cold route fault: {} {}", fault.code.0, fault.message);
            }
        }
    }
    panic!("cold route did not terminate");
}

use crate::wfc_engine::job::tests::{close_job, payload_bytes, retire_outcome};

fn step_job(job: &mut AssemblyInferenceJob, token: semio_framework_job::CancelToken, sequence: &mut u64) -> StepOutcome {
    let operation = job.operation();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), token, || Some(0), sequence);
    job.step(&mut context)
}

fn drive_job(job: &mut AssemblyInferenceJob) -> Vec<u8> {
    let mut sequence = 0;
    for _ in 0..4_000_000 {
        let mut outcome = step_job(job, root_cancel_token(), &mut sequence);
        let result = match &outcome {
            StepOutcome::Complete(candidate) => Some(Ok(payload_bytes(&candidate.output))),
            StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            StepOutcome::Cancelled => Some(Err("assembly inference unexpectedly cancelled".into())),
            _ => None,
        };
        retire_outcome(&mut outcome);
        if let Some(result) = result {
            close_job(job);
            return result.expect("assembly inference completed");
        }
    }
    close_job(job);
    panic!("assembly inference did not complete");
}

fn checkpoint_job(job: &mut AssemblyInferenceJob) -> Vec<u8> {
    let mut sequence = 0;
    for _ in 0..4_000_000 {
        let mut outcome = step_job(job, root_cancel_token(), &mut sequence);
        let checkpoint = match &outcome {
            StepOutcome::CheckpointReady(checkpoint) => Some(payload_bytes(&checkpoint.state)),
            _ => None,
        };
        retire_outcome(&mut outcome);
        if let Some(bytes) = checkpoint {
            return bytes;
        }
        if outcome.is_terminal() {
            close_job(job);
            panic!("assembly inference terminated before checkpoint");
        }
    }
    close_job(job);
    panic!("assembly inference did not checkpoint");
}

#[test]
fn solve_over_an_always_allowed_pair_finds_an_assignment_for_every_slot() {
    let snapshot = two_slot_two_module_snapshot();
    let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
    match &values["assembly"] {
        AssemblySolveResult::Solved { assignments } => assert_eq!(assignments.len(), 2, "both slots must be assigned"),
        AssemblySolveResult::Unsolved => panic!("a trivially satisfiable spec must solve"),
    }
}

#[test]
fn contradiction_field_agrees_with_solve_field_on_a_satisfiable_spec() {
    let snapshot = two_slot_two_module_snapshot();
    let satisfiable = store::infer_field::<AssemblySnapshot, AssemblyContradiction>(&snapshot, None);
    assert_eq!(satisfiable["assembly"], true);
}

#[test]
fn an_unsatisfiable_spec_is_reported_as_a_contradiction_not_a_panic() {
    let mut snapshot = two_slot_two_module_snapshot();
    // 🚫 No rule allows "a" next to "a" or "b" next to "b" AND no rule allows "a"-"b" either
    // once we remove it — an edge with a fully closed-world empty allow-set is unsatisfiable.
    snapshot.rules.clear();
    let satisfiable = store::infer_field::<AssemblySnapshot, AssemblyContradiction>(&snapshot, None);
    assert_eq!(satisfiable["assembly"], false);
    let solved = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
    assert_eq!(solved["assembly"], AssemblySolveResult::Unsolved);
}

#[test]
fn pinned_slot_always_resolves_to_its_pinned_module() {
    let mut snapshot = two_slot_two_module_snapshot();
    snapshot.slots[0].pinned_module_id = Some("a".into());
    snapshot.slots[1].pinned_module_id = Some("b".into());
    let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
    match &values["assembly"] {
        AssemblySolveResult::Solved { assignments } => {
            assert_eq!(assignments["s1"], "a");
            assert_eq!(assignments["s2"], "b");
        }
        AssemblySolveResult::Unsolved => panic!("a pinned-and-allowed pair must solve"),
    }
}

#[test]
fn empty_assembly_solves_trivially_with_no_assignments() {
    let snapshot = AssemblySnapshot::default();
    let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
    assert_eq!(values["assembly"], AssemblySolveResult::Solved { assignments: BTreeMap::new() });
}

#[test]
fn pinned_slot_has_zero_entropy_unpinned_slot_does_not() {
    let mut snapshot = two_slot_two_module_snapshot();
    snapshot.slots[0].pinned_module_id = Some("a".into());
    let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
    assert_eq!(entropy["s1"], 0.0);
    assert!(entropy["s2"] > 0.0, "an unpinned slot over two equally-weighted modules must have positive entropy");
}

#[test]
fn uniform_weights_over_two_modules_yield_ln2_entropy() {
    let snapshot = two_slot_two_module_snapshot();
    let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
    assert!((entropy["s1"] - std::f64::consts::LN_2).abs() < 1e-9);
}

#[test]
fn skewed_weights_lower_entropy_than_uniform() {
    let mut snapshot = two_slot_two_module_snapshot();
    snapshot.weights = vec![AssemblyModuleWeight { module_id: "a".into(), weight: 100.0 }, AssemblyModuleWeight { module_id: "b".into(), weight: 0.01 }];
    let entropy = store::infer_field::<AssemblySnapshot, AssemblyEntropy>(&snapshot, None);
    assert!(entropy["s1"] < std::f64::consts::LN_2, "a skewed distribution must have lower entropy than the uniform case");
}

/// 🔁 Determinism law: identical snapshots (same seed) must produce byte-identical solve
/// results — `InferredField::compute` must be a pure function of `snapshot`, WFC's internal
/// randomness notwithstanding, since the seed itself lives in the snapshot.
#[test]
fn identical_seed_and_spec_always_produce_the_same_solution() {
    let snapshot = two_slot_two_module_snapshot();
    let first = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
    let second = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
    assert_eq!(first, second);
}

#[test]
fn changing_only_the_seed_still_solves_a_trivially_satisfiable_spec() {
    let mut snapshot = two_slot_two_module_snapshot();
    snapshot.seed = 999;
    let values = store::infer_field::<AssemblySnapshot, AssemblySolve>(&snapshot, None);
    assert!(matches!(values["assembly"], AssemblySolveResult::Solved { .. }));
}

#[test]
fn maximum_admission_is_moved_without_clone_and_previews_on_fixed_cadence() {
    let mut snapshot = AssemblySnapshot::default();
    snapshot.weights = vec![AssemblyModuleWeight { module_id: String::new(), weight: 1.0 }; MAX_ASSEMBLY_WEIGHTS];
    let allocation = snapshot.weights.as_ptr();
    let mut factory = AssemblyInferenceJobFactory::default();
    let mut job = factory.create_job(operation(113), AssemblyInferenceRequest { snapshot, checkpoint: None }).expect("maximum admitted request");
    assert_eq!(allocation, job.snapshot.weights.as_ptr());
    assert!(job.weight_by_id.is_empty() && job.output.as_ref().is_some_and(|output| output.page_count() == 0) && job.model_build.is_none());
    let mut sequence = 0;
    let mut units_since_preview = 0;
    let mut previews = 0;
    for _ in 0..65 {
        units_since_preview += 1;
        let mut outcome = step_job(&mut job, root_cancel_token(), &mut sequence);
        let preview = match &outcome {
            StepOutcome::PreviewReady(bytes) => Some(dsl::os_pack::from_json_str::<AssemblyInferencePreview>(std::str::from_utf8(&payload_bytes(bytes)).expect("UTF-8 preview"))),
            _ => None,
        };
        retire_outcome(&mut outcome);
        match outcome {
            StepOutcome::PreviewReady(_) => {
                let preview = preview.expect("preview outcome").expect("preview");
                assert_eq!(preview.stage, AssemblyInferenceStage::Weights);
                assert!(units_since_preview <= PARENT_PREVIEW_UNIT_INTERVAL as usize);
                if previews == 0 {
                    assert_eq!(units_since_preview, 1);
                }
                units_since_preview = 0;
                previews += 1;
            }
            StepOutcome::Yield => {}
            outcome => panic!("unexpected maximum-admission outcome: {outcome:?}"),
        }
    }
    close_job(&mut job);
    assert!(previews >= 5);
}

#[test]
fn registered_public_factory_routes_exact_key_and_preserves_commit_freshness() {
    struct CompetingFactory {
        keys: [semio_framework::ToolFactoryKey; 1],
    }
    impl semio_framework::ToolJobFactory for CompetingFactory {
        type Payload = AssemblyInferenceRequest;
        type Job = MountedCompetingJob;

        fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
            &self.keys
        }

        fn payload_schema_id(&self) -> &str {
            "competing.assembly.inference.v1"
        }

        fn classification(&self) -> semio_framework::InteractiveJobClassification {
            semio_framework::InteractiveJobClassification::Migrated
        }

        fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
            semio_framework::ToolExecutionContract::bounded_first_step(1, 1, 1, 16, 100)
        }

        fn create_job(&mut self, _operation: Operation, _payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
            Ok(MountedCompetingJob)
        }
    }

    let bus = semio_framework::ActionBus::new();
    register_assembly_inference_factory(&bus).expect("factory registration");
    register_assembly_inference_factory(&bus).expect("idempotent factory registration");
    let key = semio_framework::ToolFactoryKey::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID);
    assert!(bus.contains(&key));
    assert!(matches!(bus.register(CompetingFactory { keys: [key.clone()] }), Err(semio_framework::ToolRegistrationError::DuplicateKey { key: rejected }) if rejected == key));
    assert_eq!(bus.keys(), vec![key]);
    assert_eq!(bus.dispatch_count(), 0);
    let operation = operation(127);
    let spec = semio_framework::ToolOperationSpec::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID, ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA, AssemblyInferenceRequest { snapshot: AssemblySnapshot::default(), checkpoint: None }, operation);
    let mut dispatch = bus.dispatch(spec).expect("registered inference lookup");
    assert_eq!(dispatch.spec.operation.base_revision, RevisionId(11));
    assert_eq!(dispatch.spec.operation.generation, Generation(7));
    let mut sequence = 0;
    let output = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
        let mut outcome = dispatch.job.step(&mut context);
        let result = match &outcome {
            StepOutcome::Complete(candidate) => Some(Ok(payload_bytes(&candidate.output))),
            StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            StepOutcome::Cancelled => Some(Err("registered inference cancelled".into())),
            _ => None,
        };
        retire_outcome(&mut outcome);
        if let Some(result) = result {
            dispatch.job.begin_close();
            while !dispatch.job.terminal_is_empty() {
                dispatch.job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            break result.expect("registered inference completed");
        }
    };
    assert_eq!(dsl::json::from_json_str::<AssemblyInferenceCommit>(std::str::from_utf8(&output).expect("commit UTF-8")).expect("commit"), AssemblyInferenceCommit::default());
    assert_eq!(semio_framework_job::validate_commit(&operation, RevisionId(11), Generation(7)), CommitValidation::Accepted);
    assert!(matches!(semio_framework_job::validate_commit(&operation, RevisionId(12), Generation(7)), CommitValidation::Stale { .. }));
    assert!(matches!(semio_framework_job::validate_commit(&operation, RevisionId(11), Generation(8)), CommitValidation::Stale { .. }));
}

#[test]
fn registered_factory_job_rejects_stale_generation_before_first_unit() {
    let bus = semio_framework::ActionBus::new();
    register_assembly_inference_factory(&bus).expect("factory registration");
    let operation = operation(131);
    let spec = semio_framework::ToolOperationSpec::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID, ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA, AssemblyInferenceRequest { snapshot: two_slot_two_module_snapshot(), checkpoint: None }, operation);
    let mut dispatch = bus.dispatch(spec).expect("registered inference lookup");
    let mut sequence = 0;
    let mut context = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX), root_cancel_token(), || Some(0), &mut sequence);
    let mut outcome = dispatch.job.step(&mut context);
    retire_outcome(&mut outcome);
    dispatch.job.begin_close();
    while !dispatch.job.terminal_is_empty() {
        dispatch.job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    assert!(matches!(outcome, StepOutcome::Fault(_)));
}

#[test]
fn restart_rebuilds_all_maps_from_owned_snapshot_after_process_state_loss() {
    let mut snapshot = two_slot_two_module_snapshot();
    snapshot.edges.clear();
    let operation = operation(snapshot.seed);
    let mut original = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: None }).expect("original job");
    let checkpoint = checkpoint_job(&mut original);
    close_job(&mut original);
    drop(original);
    let mut restarted = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: Some(checkpoint) }).expect("restarted job");
    assert!(restarted.weight_by_id.is_empty() && restarted.pattern_of.is_empty() && restarted.node_of.is_empty() && restarted.module_ids.is_empty());
    let resumed = drive_job(&mut restarted);
    let mut fresh = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot, checkpoint: None }).expect("fresh job");
    let expected = drive_job(&mut fresh);
    assert_eq!(resumed, expected);
}

#[test]
fn cancellation_is_lossless_during_compile_restore_and_authoritative_mapping() {
    let mut snapshot = two_slot_two_module_snapshot();
    snapshot.edges.clear();
    let operation = operation(snapshot.seed);
    let mut compile = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: None }).expect("compile job");
    let token = root_cancel_token();
    token.cancel_now();
    let mut sequence = 0;
    assert_eq!(step_job(&mut compile, token, &mut sequence), StepOutcome::Cancelled);
    assert_eq!((compile.stage, compile.cursor), (AssemblyInferenceStage::Weights, 0));

    let mut source = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot: snapshot.clone(), checkpoint: None }).expect("source job");
    let checkpoint = checkpoint_job(&mut source);
    let mut restored = AssemblyInferenceJob::new(operation, AssemblyInferenceRequest { snapshot, checkpoint: Some(checkpoint) }).expect("restore job");
    let mut cancelled_restore = false;
    for _ in 0..4_000_000 {
        if restored.stage == AssemblyInferenceStage::Restore {
            let token = root_cancel_token();
            token.cancel_now();
            let before = (restored.stage, restored.cursor, restored.assignments.len(), restored.output.as_ref().map_or(0, |writer| writer.page_count()));
            assert_eq!(step_job(&mut restored, token, &mut sequence), StepOutcome::Cancelled);
            assert_eq!(before, (restored.stage, restored.cursor, restored.assignments.len(), restored.output.as_ref().map_or(0, |writer| writer.page_count())));
            cancelled_restore = true;
            break;
        }
        retire_outcome(&mut step_job(&mut restored, root_cancel_token(), &mut sequence));
    }
    assert!(cancelled_restore);
    for target in [AssemblyInferenceStage::MapCommit, AssemblyInferenceStage::EncodeCommit] {
        let mut cancelled_target = false;
        for _ in 0..4_000_000 {
            if restored.stage == target {
                let token = root_cancel_token();
                token.cancel_now();
                let before = (restored.cursor, restored.assignments.len(), restored.output.as_ref().map_or(0, |writer| writer.page_count()));
                assert_eq!(step_job(&mut restored, token, &mut sequence), StepOutcome::Cancelled);
                assert_eq!(before, (restored.cursor, restored.assignments.len(), restored.output.as_ref().map_or(0, |writer| writer.page_count())));
                cancelled_target = true;
                break;
            }
            let mut outcome = step_job(&mut restored, root_cancel_token(), &mut sequence);
            retire_outcome(&mut outcome);
            assert!(!outcome.is_terminal(), "restored inference terminated before {target:?}");
        }
        assert!(cancelled_target, "did not reach {target:?}");
    }
    assert!(matches!(drive_job(&mut restored).as_slice(), [b'{', ..]));
    close_job(&mut compile);
    close_job(&mut source);
}

#[semio_framework_async_macros::async_test]
async fn mounted_semio_infer_routes_exact_assembly_job_through_checkpoint_restart() {
    register_assembly_inference_factory(&semio_framework::ActionBus::production()).expect("production assembly registration");
    let request = cold_route_request(two_slot_two_module_snapshot(), "assembly-mounted-restart", 41, 9);
    start_job(8_101, JOB_KIND_INFER, &request).await;

    let checkpoint = loop {
        match step_reactor_job(8_101, JobBudget { fuel: 1, deadline_ms: 2 }).await {
            JobStep::Running(_) => {
                let entries = checkpoint_jobs().await;
                if let Some(checkpoint) = entries.iter().find(|entry| entry.job == 8_101).and_then(|entry| entry.checkpoint.clone()) {
                    break checkpoint;
                }
            }
            JobStep::Done(_) => panic!("mounted route completed before publishing a restart checkpoint"),
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                panic!("mounted route failed before checkpoint: {} {}", fault.code.0, fault.message);
            }
        }
    };
    cancel_job(8_101).await;
    restore_job(8_101, JOB_KIND_INFER, &request, Some(checkpoint)).await;
    let result = drive_cold_route(8_101).await;
    assert_eq!(result.inference_schema, ASSEMBLY_INFERENCE_TOOL_ID);
    assert_eq!(result.revision, 41);
    assert_eq!(result.generation, 9);
    assert!(result.complete);
    let commit: AssemblyInferenceCommit = dsl::json::from_json_str(std::str::from_utf8(&result.canonical_payload).expect("assembly commit UTF-8")).expect("assembly commit");
    assert_eq!(commit.assignments.len(), 2);
}

#[test]
fn production_route_registration_is_idempotent_and_collision_safe() {
    let bus = semio_framework::ActionBus::production();
    register_assembly_inference_factory(&bus).expect("first production registration");
    register_assembly_inference_factory(&bus).expect("idempotent production registration");
    let key = semio_framework::ToolFactoryKey::new(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID);
    assert!(matches!(bus.register(MountedCompetingFactory { keys: [key.clone()] }), Err(semio_framework::ToolRegistrationError::DuplicateKey { key: rejected }) if rejected == key));
    assert_eq!(bus.payload_schema_id(&key).as_deref(), Some(ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA));
}

#[semio_framework_async_macros::async_test]
async fn mounted_semio_infer_cancel_discards_the_live_job() {
    register_assembly_inference_factory(&semio_framework::ActionBus::production()).expect("production assembly registration");
    let request = cold_route_request(two_slot_two_module_snapshot(), "assembly-mounted-cancel", 42, 10);
    start_job(8_102, JOB_KIND_INFER, &request).await;
    let _ = step_reactor_job(8_102, JobBudget { fuel: 1, deadline_ms: 2 }).await;
    cancel_job(8_102).await;
    assert!(checkpoint_jobs().await.iter().all(|entry| entry.job != 8_102));
    assert!(matches!(step_reactor_job(8_102, JobBudget { fuel: 1, deadline_ms: 2 }).await, JobStep::Failed(_)));
}

async fn run_exact_factory_on_pool(workers: usize) -> Vec<u8> {
    let bus = semio_framework::ActionBus::new();
    register_assembly_inference_factory(&bus).expect("assembly registration");
    let operation = operation(177);
    let payload = dsl::json::to_json_string(&AssemblyInferenceRequest { snapshot: two_slot_two_module_snapshot(), checkpoint: None }).into_bytes();
    let dispatch = bus.dispatch_wire(ASSEMBLY_INFERENCE_JOB_KIND, ASSEMBLY_INFERENCE_TOOL_ID, ASSEMBLY_INFERENCE_PAYLOAD_SCHEMA, &payload, None, operation).expect("wire dispatch");
    let pool = semio_framework_job::WorkerPool::new(semio_framework_job::WorkerPoolConfig::new(semio_framework_job::ProcessKind::HeadlessBatch, workers));
    let params = semio_framework_job::BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: root_cancel_token(),
        config: semio_framework_job::BatchDriveConfig { site: "assembly.wfc.worker-count", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 2000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = match semio_framework_job::MountedWorkerJobSession::try_new(dispatch.job, params) {
        Ok(session) => session,
        Err(mut rejected) => {
            rejected.begin_close();
            while !rejected.terminal_is_empty() {
                rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            panic!("worker fixture admission rejected");
        }
    };
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    for _ in 0..200_000 {
        while session.checked_out_outcome().is_none() {
            assert!(std::time::Instant::now() < deadline, "worker-count route exceeded its deadline");
            session.pump_one(&pool, semio_framework_job::Lane::UserVisible).unwrap_or_else(|_| panic!("worker pump"));
            std::thread::yield_now();
        }
        let mut outcome = session.take_checked_out_outcome().expect("worker outcome");
        let result = match &outcome {
            StepOutcome::Complete(candidate) => Some(Ok(payload_bytes(&candidate.output))),
            StepOutcome::Fault(fault) => Some(Err(String::from_utf8_lossy(&payload_bytes(&fault.detail)).into_owned())),
            StepOutcome::Cancelled => Some(Err("worker-count route cancelled".to_string())),
            _ => None,
        };
        while !outcome.terminal_is_empty() {
            outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
        if let Some(result) = result {
            session.begin_close();
            while !session.terminal_is_empty() {
                assert!(std::time::Instant::now() < deadline, "worker-count close exceeded its deadline");
                session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                std::thread::yield_now();
            }
            return result.expect("worker-count route completed");
        }
        session.resume().expect("worker outcome resumes");
    }
    session.begin_close();
    while !session.terminal_is_empty() {
        session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        std::thread::yield_now();
    }
    panic!("worker-count route did not terminate");
}

#[semio_framework_async_macros::async_test]
async fn exact_factory_replays_byte_identically_on_actual_worker_pools() {
    let default = std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1);
    let one = run_exact_factory_on_pool(1).await;
    assert_eq!(run_exact_factory_on_pool(2).await, one);
    assert_eq!(run_exact_factory_on_pool(4).await, one);
    assert_eq!(run_exact_factory_on_pool(default).await, one);
}
