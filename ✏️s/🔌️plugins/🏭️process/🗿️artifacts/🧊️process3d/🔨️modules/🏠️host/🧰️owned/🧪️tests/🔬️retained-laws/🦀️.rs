use super::*;
use crate::standards::v1::subsets::any::io::binary::mutations::*;

fn every_mutation() -> Vec<Process3dMutation> {
    process3d_all_retained_mutation_fixtures_for_test()
}

fn authority_fixture() -> Process3dPublicationLease {
    Process3dPublicationLease {
        operation: u64::MAX - 313,
        generation: 51,
        base_revision: 51,
        parent_revision: 51,
        live_revision: 51,
        maximum_items: PROCESS3D_MAXIMUM_DOMAIN_ITEMS,
        maximum_output_pages: PROCESS3D_MOUNTED_OUTPUT_CHANNELS,
        maximum_controls: PROCESS3D_MOUNTED_CONTROL_CREDITS,
        closing: false,
        terminal: false,
    }
}

/// 🔒️ `process3d_admit_publication_authority` fills ONE process-global fixed authority table, which
/// answers `process3d-publication.saturated` the moment two holders overlap. Every law that mints a
/// real publication lease takes this lane first, so the suite's own parallelism cannot starve one
/// law of the authority another law is holding. Poisoning is absorbed: a lane is a scheduling
/// device, and the panic that poisoned it is already the failure being reported.
fn publication_authority_lane() -> std::sync::MutexGuard<'static, ()> {
    process3d_publication_authority_lane()
}

fn close_store(mut store: store::ArtifactStore<Process3dSnapshot, Process3dMutation>) {
    use semio_framework_plugin::ArtifactOwnedDisposer;
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Process3dSnapshot, Process3dMutation>::new();
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        if matches!(disposer.close_step(&mut store, 1, PROCESS3D_OWNER_BYTES), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            break;
        }
    }
    assert!(disposer.terminal_is_empty(&store));
    drop(store);
}

/// 📏️ Budget the TYPICAL unit of one whole-document store-replacement phase must respect. A quarter of
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000us), the ceiling the OS runtime's
/// cooperative-maintenance clock faults an instance on: this runs at opt-level 0, far slower than
/// the release wasm that ceiling guards, so a native phase already eating a quarter of it is the
/// defect and not the noise.
const REPLACEMENT_PHASE_BUDGET_US: u64 = 2_000;

/// 🚨️ Mirror of `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` — that crate is not a
/// dependency of this one, and no single unit of a maintenance stage may reach it.
const REPLACEMENT_STEP_CEILING_US: u64 = 8_000;

/// 🧭️ The store replacement job's own phases, in the order `drive_store_replacement_jobs` runs
/// them. Each is one cooperative-maintenance unit and is budgeted on its own.
const REPLACEMENT_PHASES: [&str; 3] = ["Initializing", "CandidateReady", "RetiringCommittedStore"];

/// 📐️ Units retained per phase for the typical-cost statistic. Fixed, and taken from the FIRST
/// units a phase runs — the busy ones, where a phase that has real work does it.
const REPLACEMENT_UNIT_SAMPLES: usize = 64;

/// ⏱️ Measured wall cost of one run's replacement units, per [`REPLACEMENT_PHASES`] entry. Fixed
/// capacity by construction.
///
/// Two statistics, because one cannot carry both halves of the framework's law. [`worst`] is the
/// per-phase MEDIAN — a phase that overruns because of its OWN work overruns unit after unit, so a
/// median catches a systemic regression while ignoring the machine (the runtime's ceiling verdict
/// times the unit on the wall clock of a contended thread). [`worst_unit`] is the plain maximum,
/// which is what the ceiling itself actually bounds, and is therefore budgeted against the ceiling
/// rather than against the far tighter typical-unit budget. Median precedent:
/// `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️fixed-operation-registry/🦀️.rs`.
struct ReplacementPhaseBudget {
    samples_us: [[u64; REPLACEMENT_UNIT_SAMPLES]; REPLACEMENT_PHASES.len()],
    sampled: [usize; REPLACEMENT_PHASES.len()],
    worst_us: [u64; REPLACEMENT_PHASES.len()],
    units: [u32; REPLACEMENT_PHASES.len()],
}

impl Default for ReplacementPhaseBudget {
    fn default() -> Self {
        Self { samples_us: [[0; REPLACEMENT_UNIT_SAMPLES]; REPLACEMENT_PHASES.len()], sampled: [0; REPLACEMENT_PHASES.len()], worst_us: [0; REPLACEMENT_PHASES.len()], units: [0; REPLACEMENT_PHASES.len()] }
    }
}

impl ReplacementPhaseBudget {
    /// ⏱️ Typical cost of one unit of `phase`: the median of its retained readings.
    fn median(&self, phase: usize) -> u64 {
        let sampled = self.sampled[phase];
        if sampled == 0 {
            return 0;
        }
        let mut ordered = self.samples_us[phase];
        ordered[..sampled].sort_unstable();
        ordered[sampled / 2]
    }

    /// ⏱️ The phase whose typical unit is the most expensive, with that median in microseconds.
    fn worst(&self) -> (&'static str, u64) {
        let mut worst = (REPLACEMENT_PHASES[0], 0);
        for phase in 0..REPLACEMENT_PHASES.len() {
            let median = self.median(phase);
            if median > worst.1 {
                worst = (REPLACEMENT_PHASES[phase], median);
            }
        }
        worst
    }

    /// ⏱️ The phase that ran the single most expensive unit, with that maximum in microseconds.
    fn worst_unit(&self) -> (&'static str, u64) {
        let mut worst = (REPLACEMENT_PHASES[0], 0);
        for phase in 0..REPLACEMENT_PHASES.len() {
            if self.worst_us[phase] > worst.1 {
                worst = (REPLACEMENT_PHASES[phase], self.worst_us[phase]);
            }
        }
        worst
    }

    /// 🎲️ Folds one more independent round of the same scenario in by keeping, per phase, the
    /// SMALLEST median and the SMALLEST maximum any round observed. Real work costs the same in
    /// every round; a run that happened to share the machine with a heavier neighbour is dropped.
    fn keep_best_round(&mut self, round: &Self) {
        for phase in 0..REPLACEMENT_PHASES.len() {
            if round.units[phase] == 0 {
                continue;
            }
            if self.sampled[phase] == 0 || round.median(phase) < self.median(phase) {
                self.samples_us[phase] = round.samples_us[phase];
                self.sampled[phase] = round.sampled[phase];
            }
            if self.units[phase] == 0 || round.worst_us[phase] < self.worst_us[phase] {
                self.worst_us[phase] = round.worst_us[phase];
            }
            self.units[phase] = self.units[phase].max(round.units[phase]);
        }
    }

    /// 📊️ Per-phase `phase=median/worst/units` breakdown of every unit measured.
    fn report(&self) -> String {
        let mut report = String::new();
        for phase in 0..REPLACEMENT_PHASES.len() {
            report.push_str(&format!("{}={}/{}us/{}u ", REPLACEMENT_PHASES[phase], self.median(phase), self.worst_us[phase], self.units[phase]));
        }
        report
    }
}

/// ⏱️ Runs one replacement phase unit under the framework's own clock and records the reading.
fn measure_phase<T>(budget: &mut ReplacementPhaseBudget, phase: usize, work: impl FnOnce() -> T) -> T {
    let started_us = semio_framework_job::default_now_us();
    let value = work();
    budget.units[phase] += 1;
    if let (Some(started_us), Some(finished_us)) = (started_us, semio_framework_job::default_now_us()) {
        let elapsed_us = finished_us.saturating_sub(started_us);
        if budget.sampled[phase] < REPLACEMENT_UNIT_SAMPLES {
            budget.samples_us[phase][budget.sampled[phase]] = elapsed_us;
            budget.sampled[phase] += 1;
        }
        budget.worst_us[phase] = budget.worst_us[phase].max(elapsed_us);
    }
    value
}

fn owned_store(label: &str, operation_value: u64) -> store::ArtifactStore<Process3dSnapshot, Process3dMutation> {
    owned_store_measured(label, operation_value, &mut ReplacementPhaseBudget::default())
}

fn owned_store_measured(label: &str, operation_value: u64, budget: &mut ReplacementPhaseBudget) -> store::ArtifactStore<Process3dSnapshot, Process3dMutation> {
    let operation = semio_framework_job::OperationId(operation_value);
    let generation = semio_framework_job::Generation(51);
    process3d_admit_publication_authority(operation, generation, generation.0, generation.0, generation.0, crate::host::owned::Process3dPublicationLimits { maximum_items: PROCESS3D_MAXIMUM_DOMAIN_ITEMS, maximum_output_pages: PROCESS3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: PROCESS3D_MOUNTED_CONTROL_CREDITS }).expect("fixture publication authority");
    let mut snapshot = crate::empty_process3d_snapshot();
    snapshot.stock_label = label.into();
    let envelope = store::create_document_envelope(crate::PROCESS_3D_SCHEMA, label, snapshot, None);
    let mut authority = Process3dStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut complete = false;
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
        match measure_phase(budget, 0, || semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context)) {
            semio_framework_job::StepOutcome::Complete(_) => {
                complete = true;
                break;
            }
            semio_framework_job::StepOutcome::Yield | semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) => {}
            semio_framework_job::StepOutcome::Cancelled => panic!("fixture initializer cancelled"),
            semio_framework_job::StepOutcome::Fault(_) => panic!("fixture initializer faulted"),
        }
    }
    assert!(complete, "fixture initializer must converge");
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("fixture candidate handoff");
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    assert!(process3d_release_publication_authority(operation, generation));
    candidate
}

#[test]
fn retained_initializer_replays_every_neutral_timeline_mutation() {
    let _lane = publication_authority_lane();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    let cases = ["🌱create-step/🪚️accepts", "🗑️delete-step/🚫️accepts", "🏷️rename-step/🔤️accepts", "🔘change-step-enabled/⏸️accepts", "🧷change-step-origin/🏭️accepts", "📐replace-step-measure/🕳️accepts", "🔀reorder-steps/🔀️accepts"];
    let mut results = Vec::new();
    for (index, case) in cases.iter().enumerate() {
        let folder = root.join(case);
        let before = std::fs::read_to_string(folder.join("📸️snapshot/⬅️before/🔣️.json")).unwrap();
        let mutation = std::fs::read_to_string(folder.join("🦠️mutation/🔣️.json")).unwrap();
        let after = std::fs::read_to_string(folder.join("📸️snapshot/➡️after/🔣️.json")).unwrap();
        let snapshot: Process3dSnapshot = semio_framework_pack_json::from_json_str(&before, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mutation: Process3dMutation = semio_framework_pack_json::from_json_str(&mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let mut envelope = store::create_document_envelope(crate::PROCESS_3D_SCHEMA, case, snapshot, None);
        envelope.cursor = None;
        envelope.vcs.edits.try_push(protocol::Edit { id: format!("timeline-{index}"), actor: Some("neutral-law".into()), line: None, forwards: vec![mutation], inverse: Default::default(), mutation_meta: Vec::new(), verb: None, sequence_number: 1, started_at: "1".into(), finished_at: None }).unwrap();
        let operation = semio_framework_job::OperationId(u64::MAX - 4_000 - index as u64);
        let generation = semio_framework_job::Generation(51);
        process3d_admit_publication_authority(operation, generation, 51, 51, 51, Process3dPublicationLimits { maximum_items: PROCESS3D_MAXIMUM_DOMAIN_ITEMS, maximum_output_pages: PROCESS3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: PROCESS3D_MOUNTED_CONTROL_CREDITS }).unwrap();
        let mut authority = Process3dStoreInitializationAuthority::new(envelope, operation, generation, protocol::ActorId("neutral-law".into()));
        let cancel = semio_framework_job::CancelToken::root_now();
        let mut sequence = 0;
        let mut complete = false;
        for _ in 0..100_000 {
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut sequence);
            match semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context) {
                semio_framework_job::StepOutcome::Complete(_) => { complete = true; break; }
                semio_framework_job::StepOutcome::Fault(fault) => panic!("neutral initializer refused {case}: {fault:?}"),
                semio_framework_job::StepOutcome::Cancelled => panic!("neutral initializer cancelled"),
                _ => {}
            }
        }
        assert!(complete);
        let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).unwrap();
        assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
        drop(authority);
        assert!(process3d_release_publication_authority(operation, generation));
        let actual: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(candidate.snapshot_root().as_ref())).unwrap();
        let expected: serde_json::Value = serde_json::from_str(&after).unwrap();
        close_store(candidate);
        eprintln!("[DEBUG] Retained timeline oracle {case}: {} steps", actual["stepPayloads"].as_array().unwrap().len());
        results.push((case, actual, expected));
    }
    for (case, actual, expected) in results { assert_eq!(actual, expected, "retained neutral timeline {case}"); }
}

#[test]
fn retained_timeline_cancellation_closes_partial_rows_and_json_with_exact_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["maximumAdmittedReleaseBytes"].as_u64().unwrap() as usize, PROCESS3D_MAXIMUM_DOMAIN_BYTES);
    let backing = &fixture["physicalBacking"];
    let mut owner = Process3dTimelineOwner::empty();
    owner.steps = Vec::with_capacity(backing["capacityItems"].as_u64().unwrap() as usize);
    owner.phase = 3;
    let extent = owner.steps.capacity() * size_of::<ProcessStep>();
    let mut retirement = Process3dRetirementStack::new(Process3dRetirementOwner::Timeline { value: owner });
    let initial_demand = retirement.next_close_byte_demand();
    let blocked = retirement.advance(extent - backing["underGrantOffset"].as_u64().unwrap() as usize).unwrap();
    let blocked_demand = retirement.next_close_byte_demand();
    let retained = match retirement.slots[0].as_ref().unwrap() { Process3dRetirementOwner::Timeline { value } => value.steps.capacity() * size_of::<ProcessStep>(), _ => unreachable!() };
    let released = retirement.advance(extent).unwrap();
    while !retirement.terminal_is_empty() { retirement.advance(PROCESS3D_OWNER_BYTES).unwrap(); }
    assert_eq!(initial_demand, extent);
    assert_eq!(blocked_demand, extent);
    assert_eq!(retained, extent);
    assert_eq!(blocked, store::SnapshotRetirementStep::Pending { released_items: backing["expectedUnderGrantReleasedItems"].as_u64().unwrap() as usize, released_bytes: backing["expectedUnderGrantReleasedBytes"].as_u64().unwrap() as usize });
    assert_eq!(released, store::SnapshotRetirementStep::Pending { released_items: backing["expectedFullGrantReleasedItems"].as_u64().unwrap() as usize, released_bytes: extent });
    eprintln!("[DEBUG] Timeline backing retained below exact physical extent {extent} and released once with the complete grant");
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌱create-step/🪚️accepts");
    let snapshot: Process3dSnapshot = semio_framework_pack_json::from_json_str(&std::fs::read_to_string(root.join("📸️snapshot/⬅️before/🔣️.json")).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mutation: Process3dMutation = semio_framework_pack_json::from_json_str(&std::fs::read_to_string(root.join("🦠️mutation/🔣️.json")).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let release_bytes = fixture["releaseBytes"].as_u64().unwrap() as usize;
    for stop in fixture["stops"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let mut cursor = Process3dTimelineCursor::new();
        assert_eq!(fixture["maximumWorkItems"].as_u64().unwrap(), 1);
        for _ in 0..stop { if cursor.step(&snapshot, &mutation).unwrap() { break; } }
        let phase = cursor.phase;
        assert!(!cursor.close_step(fixture["zeroReleaseBytes"].as_u64().unwrap() as usize).unwrap());
        assert!(cursor.phase == phase);
        let mut complete = false;
        let mut largest_release = 0;
        for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap() {
            let demand = cursor.next_close_byte_demand();
            assert!(demand <= fixture["maximumAdmittedReleaseBytes"].as_u64().unwrap() as usize);
            let physical_grant = if demand > release_bytes { demand } else { release_bytes };
            if cursor.close_step(physical_grant).unwrap() { complete = true; break; }
            assert!(cursor.release_progress.0 <= 1 && cursor.release_progress.1 <= physical_grant);
            largest_release = largest_release.max(cursor.release_progress.1);
        }
        assert!(complete, "partial timeline retirement at {stop}");
        assert!(cursor.payload.steps.is_empty() && cursor.payload.steps.capacity() == 0);
        assert!(cursor.payload.tools.is_empty() && cursor.payload.tools.capacity() == 0);
        assert!(cursor.payload.flow.is_none() && cursor.writer.is_none() && cursor.output.is_none() && cursor.json_retirement.is_none() && cursor.payload_retirement.is_none());
        let actual = serde_json::json!({ "stepLength": cursor.payload.steps.len(), "stepCapacity": cursor.payload.steps.capacity(), "toolLength": cursor.payload.tools.len(), "toolCapacity": cursor.payload.tools.capacity(), "flow": cursor.payload.flow.is_some(), "writer": cursor.writer.is_some(), "output": cursor.output.is_some(), "jsonRetirement": cursor.json_retirement.is_some(), "payloadRetirement": cursor.payload_retirement.is_some() });
        assert_eq!(actual, fixture["expected"]);
        drop(cursor);
        eprintln!("[DEBUG] Partial timeline stopped at {stop}: declared {release_bytes}-byte release floor, exact largest physical release {largest_release}, reached neutral terminal-empty");
    }
}

#[test]
fn actual_atomic_publication_is_fail_closed_and_retires_stale_candidate() {
    let _lane = publication_authority_lane();
    let authority = authority_fixture();
    let operation = semio_framework_job::OperationId(authority.operation);
    let generation = semio_framework_job::Generation(authority.generation);
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { operation: authority.operation + 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-operation"));
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { generation: authority.generation + 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-generation"));
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { base_revision: authority.base_revision - 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-base"));
    assert_eq!(process3d_validate_atomic_lease(Process3dPublicationLease { parent_revision: authority.parent_revision - 1, ..authority }, operation, generation, generation), Err("process3d-publication.wrong-parent"));

    let mut live = owned_store("last-valid", u64::MAX - 316);
    let stale = owned_store("stale-candidate", u64::MAX - 315);
    let accepted = owned_store("accepted-candidate", u64::MAX - 314);

    let stale = match semio_framework_plugin::publish_document_store_candidate_if_authoritative(&mut live, stale, || {
        process3d_validate_atomic_lease(Process3dPublicationLease { parent_revision: authority.parent_revision - 1, ..authority }, operation, generation, generation)
            .map_err(|code| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), "hostile stale publication"))
    }) {
        Err((_fault, stale)) => stale,
        Ok(displaced) => {
            close_store(displaced);
            panic!("wrong parent swapped the stale candidate")
        }
    };
    assert_eq!(live.snapshot_root().stock_label, "last-valid");
    close_store(stale);

    let displaced = match semio_framework_plugin::publish_document_store_candidate_if_authoritative(&mut live, accepted, || {
        process3d_validate_atomic_lease(authority, operation, generation, generation).map_err(|code| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), "valid publication"))
    }) {
        Ok(displaced) => displaced,
        Err((_fault, rejected)) => {
            close_store(rejected);
            panic!("fresh authority rejected the accepted candidate")
        }
    };
    assert_eq!(live.snapshot_root().stock_label, "accepted-candidate");
    close_store(displaced);
    close_store(live);
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave R2: `drive_store_replacement_jobs` is ONE
/// cooperative-maintenance unit of the OS runtime's live-cleanup clock, which faults its instance
/// with `plugin.internal.interactive-ceiling` the moment a unit overruns
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US`. Each of the replacement's three phases is
/// therefore its own unit and must fit alone: `Initializing`'s worker step, `CandidateReady`'s
/// atomic swap (`build_document_store_disposer` + `validate_document_store_publication` +
/// `publish_document_store_candidate_if_authoritative`), and `Retiring*`'s one-item displaced-store
/// close step. Measured on the real initializer/publication/disposer path, naming the phase that
/// overruns.
#[test]
fn every_store_replacement_phase_unit_fits_the_interactive_step_budget() {
    let _lane = publication_authority_lane();
    let mut best = ReplacementPhaseBudget::default();
    for round in 0..REPLACEMENT_BUDGET_ROUNDS {
        best.keep_best_round(&measure_one_store_replacement(round as u64));
    }
    let (typical_phase, median_us) = best.worst();
    let (peak_phase, peak_us) = best.worst_unit();
    assert!(median_us <= REPLACEMENT_PHASE_BUDGET_US, "store replacement phase {typical_phase}'s typical unit cost {median_us}us in every round, over the {REPLACEMENT_PHASE_BUDGET_US}us typical-unit budget (per-phase median/worst/units: {})", best.report());
    assert!(peak_us < REPLACEMENT_STEP_CEILING_US, "store replacement phase {peak_phase} ran one unit for {peak_us}us in every round, at or over the framework's {REPLACEMENT_STEP_CEILING_US}us interactive step ceiling (per-phase median/worst/units: {})", best.report());
}

/// 🎲️ Independent repetitions of the whole measured replacement, folded per phase by
/// [`ReplacementPhaseBudget::keep_best_round`].
const REPLACEMENT_BUDGET_ROUNDS: u64 = 3;

/// ⏱️ Drives one whole real store replacement — initializer to displaced-store retirement — and
/// hands back what each phase's single most expensive unit cost.
fn measure_one_store_replacement(round: u64) -> ReplacementPhaseBudget {
    use semio_framework_plugin::ArtifactOwnedDisposer;
    let authority = authority_fixture();
    let operation = semio_framework_job::OperationId(authority.operation);
    let generation = semio_framework_job::Generation(authority.generation);
    let mut budget = ReplacementPhaseBudget::default();

    let mut live = owned_store_measured("budget-live", u64::MAX - 318 - round * 2, &mut budget);
    let candidate = owned_store_measured("budget-candidate", u64::MAX - 317 - round * 2, &mut budget);

    let displaced = match measure_phase(&mut budget, 1, || {
        let disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Process3dSnapshot, Process3dMutation>::new();
        let published = semio_framework_plugin::publish_document_store_candidate_if_authoritative(&mut live, candidate, || {
            process3d_validate_atomic_lease(authority, operation, generation, generation).map_err(|code| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(code), "budget publication"))
        });
        (disposer, published)
    }) {
        (_disposer, Ok(displaced)) => displaced,
        (_disposer, Err((_fault, rejected))) => {
            close_store(rejected);
            close_store(live);
            panic!("fresh authority rejected the budget candidate")
        }
    };
    assert_eq!(live.snapshot_root().stock_label, "budget-candidate");

    let mut displaced = displaced;
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Process3dSnapshot, Process3dMutation>::new();
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        if matches!(measure_phase(&mut budget, 2, || disposer.close_step(&mut displaced, 1, PROCESS3D_OWNER_BYTES)), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            break;
        }
    }
    assert!(disposer.terminal_is_empty(&displaced), "the displaced store must reach terminal-empty ownership within its bounded close");
    drop(displaced);
    close_store(live);
    budget
}

#[test]
fn owner_census_rejects_zero_depth_and_maximum_plus_one() {
    let mut zero = Process3dOwnerTotals::default();
    assert_eq!(zero.admit(0, 0, 0), Ok(()));

    let mut exact = Process3dOwnerTotals::default();
    assert_eq!(exact.admit(PROCESS3D_MAXIMUM_DOMAIN_ITEMS, PROCESS3D_MAXIMUM_DOMAIN_BYTES, PROCESS3D_RETAINED_STACK_CAPACITY - 1), Ok(()));

    let mut item_overflow = Process3dOwnerTotals::default();
    assert_eq!(item_overflow.admit(PROCESS3D_MAXIMUM_DOMAIN_ITEMS + 1, 0, 0), Err("process3d-owner.items-capacity"));
    let mut byte_overflow = Process3dOwnerTotals::default();
    assert_eq!(byte_overflow.admit(0, PROCESS3D_MAXIMUM_DOMAIN_BYTES + 1, 0), Err("process3d-owner.bytes-capacity"));
    let mut depth_overflow = Process3dOwnerTotals::default();
    assert_eq!(depth_overflow.admit(0, 0, PROCESS3D_RETAINED_STACK_CAPACITY), Err("process3d-owner.combined-depth"));
}

#[test]
fn interrupted_snapshot_close_reaches_terminal_empty() {
    let mut owner = Process3dOwnedRetirement::snapshot(crate::empty_process3d_snapshot());
    assert!(matches!(store::ErasedSnapshotRetirement::close_step(&mut owner, 0, 0), Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })));
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        if matches!(store::ErasedSnapshotRetirement::close_step(&mut owner, 1, PROCESS3D_OWNER_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
            break;
        }
    }
    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&owner));
}

#[test]
fn mounted_mutation_region_has_zero_whole_string_reader_edges() {
    let source = include_str!("../../🦀️.rs");
    let retained = source.split_once("enum Process3dRetainedMutationPhase").expect("retained mutation region start").1.split_once("enum Process3dMutationDecodeState").expect("retained mutation region end").0;
    assert_eq!(retained.matches(concat!("read_str", "_lp")).count(), 0, "mounted mutation reader must have no whole-string edge");
}

#[test]
fn every_mutation_uses_retained_grants_and_incremental_terminal_retirement() {
    let operation = semio_framework_job::OperationId(u64::MAX - 91);
    let generation = semio_framework_job::Generation(23);
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mutations = every_mutation();
    assert_eq!(mutations.len(), PROCESS3D_MUTATION_VARIANT_COUNT);

    for mutation in mutations {
        let bytes = encode_op(&mutation).expect("mutation fixture encoding");
        let mut reader = Process3dRetainedMutationReader::new();
        let mut grants = 0;
        loop {
            grants += 1;
            let mut grant = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
            if reader.step(&bytes, &mut grant).expect("retained semantic grant") {
                break;
            }
            assert!(grants <= bytes.len() + PROCESS3D_RETAINED_STACK_CAPACITY, "retained cursor stopped advancing");
        }
        assert!(grants > 2, "a mutation crossed the mounted route without field-level suspension");
        let decoded = reader.take().expect("exact mutation handoff");
        assert_eq!(decoded, mutation);
        assert!(reader.take().is_none());
        drop(reader);

        let mut retirement = Process3dOwnedRetirement::mutation(decoded);
        for _ in 0..128 {
            if matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, PROCESS3D_OWNER_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
                break;
            }
        }
        assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));

        for interruption in 1..grants {
            let mut interrupted = Process3dRetainedMutationReader::new();
            for _ in 0..interruption {
                let mut grant = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence);
                assert!(!interrupted.step(&bytes, &mut grant).expect("interrupted retained semantic grant"), "pre-terminal interruption must remain resumable");
                assert_eq!(grant.fuel_remaining(), 0, "one retained mutation substate consumes one grant");
            }
            let partial = interrupted.take_rejected().expect("every interrupted mutation substate hands back its exact partial owner");
            assert!(interrupted.terminal_is_empty());
            drop(interrupted);
            let mut retirement = Process3dOwnedRetirement::mutation(partial);
            for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
                if matches!(store::ErasedSnapshotRetirement::close_step(&mut retirement, 1, PROCESS3D_OWNER_BYTES), Ok(store::SnapshotRetirementStep::Complete)) {
                    break;
                }
            }
            assert!(store::ErasedSnapshotRetirement::terminal_is_empty(&retirement));
            drop(retirement);
        }
    }
}

#[test]
fn retained_string_cursor_enforces_one_byte_grants_and_hostile_boundaries() {
    fn grant(cursor: &mut Process3dRetainedStringCursor, byte: u8) -> Result<Option<String>, String> {
        let mut reader = store::ByteReader::new(std::slice::from_ref(&byte));
        let value = cursor.step(&mut reader);
        assert_eq!(reader.position(), 1, "one retained string grant consumes one byte opportunity");
        value
    }

    let mut exact = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert_eq!(grant(&mut exact, 3).expect("exact length admission"), None);
    assert_eq!(grant(&mut exact, b'a').expect("exact byte one"), None);
    assert_eq!(grant(&mut exact, b'b').expect("exact byte two"), None);
    assert_eq!(grant(&mut exact, b'c').expect("exact byte three"), Some("abc".into()));
    assert!(exact.terminal_is_empty());

    let mut plus_one = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert!(grant(&mut plus_one, 4).expect_err("maximum plus one must fail before producer copy").contains("fixed byte credit"));
    assert_eq!(plus_one.take_partial(), "", "maximum plus one returns its empty pre-copy owner");
    assert!(plus_one.terminal_is_empty());

    let mut malformed = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert_eq!(grant(&mut malformed, 1).expect("malformed length admission"), None);
    assert!(grant(&mut malformed, 0xff).expect_err("malformed UTF-8 must fail at its byte boundary").contains("utf-8"));
    assert!(malformed.terminal_is_empty());

    let mut truncated = Process3dRetainedStringCursor::with_maximum_bytes(3);
    assert_eq!(grant(&mut truncated, 2).expect("truncated length admission"), None);
    assert_eq!(grant(&mut truncated, b'x').expect("truncated first byte"), None);
    assert!(truncated.step(&mut store::ByteReader::new(&[])).is_err(), "truncation must remain a resumable read failure");
    assert_eq!(truncated.take_partial(), "x", "interrupted string bytes return through the exact handback owner");
    assert!(truncated.terminal_is_empty());

    let mut overflowing_length = Process3dRetainedStringCursor::with_maximum_bytes(3);
    for _ in 0..(usize::BITS / 7) {
        assert_eq!(grant(&mut overflowing_length, 0x80).expect("bounded length byte"), None);
    }
    assert!(grant(&mut overflowing_length, 0x80).is_err(), "overlong retained length must fail without payload allocation");
    assert_eq!(overflowing_length.take_partial(), "");
    assert!(overflowing_length.terminal_is_empty());
}

/// 🧾️ The envelope snapshot decoder publishes and forwards an admitted timeline's complete backing extent.
#[test]
fn snapshot_decoder_publishes_and_funds_large_timeline_backing() {
    use store::ArtifactEnvelopeSnapshotFieldAuthority as _;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["decoderBacking"];
    assert_eq!(row["queryEachDemand"].as_bool(), Some(true));
    let mut timeline = Process3dTimelineOwner::empty();
    timeline.steps = Vec::with_capacity(row["capacityItems"].as_u64().unwrap() as usize);
    timeline.phase = 3;
    let extent = timeline.steps.capacity() * size_of::<ProcessStep>();
    assert!(extent > fixture["releaseBytes"].as_u64().unwrap() as usize && extent < PROCESS3D_MAXIMUM_DOMAIN_BYTES);
    let mut decoder = Process3dSnapshotDecodeAuthority::new(semio_framework_job::OperationId(7001), semio_framework_job::Generation(1), store::OwnedSchemaPath::ROOT);
    *decoder.retirement = Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementOwner::Timeline { value: timeline })));
    decoder.state = Process3dSnapshotDecodeState::Closing;
    let advertised = decoder.next_close_byte_demand().unwrap();
    let funded = decoder.close_step(1, extent).unwrap();
    let mut turns = 1;
    for _ in 1..row["maximumAdvertisedTurns"].as_u64().unwrap() { if decoder.terminal_is_empty() { break; } decoder.close_step(1, decoder.next_close_byte_demand().unwrap()).unwrap(); turns += 1; }
    let terminal = decoder.terminal_is_empty();
    while decoder.retirement.is_some() { process3d_retire_erased_step(&mut decoder.retirement, PROCESS3D_MAXIMUM_DOMAIN_BYTES).unwrap(); }
    decoder.state = Process3dSnapshotDecodeState::Complete;
    eprintln!("[DEBUG] Process snapshot decoder backing={extent} advertised={advertised} exact-funded-step={funded:?} bounded-turns={turns} terminal={terminal}");
    assert_eq!(advertised, extent);
    assert_eq!(funded, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent });
    assert_eq!(terminal, row["expectedTerminal"].as_bool().unwrap());
}

/// 🧮️ A nested mutation decoder forwards the same complete physical backing grant as its snapshot sibling.
#[test]
fn mutation_decoder_forwards_large_retained_backing_grant() {
    use store::ArtifactEnvelopeMutationFieldAuthority as _;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let row = &fixture["decoderBacking"];
    assert_eq!(row["queryEachDemand"].as_bool(), Some(true));
    let mut timeline = Process3dTimelineOwner::empty();
    timeline.steps = Vec::with_capacity(row["capacityItems"].as_u64().unwrap() as usize);
    timeline.phase = 3;
    let extent = timeline.steps.capacity() * size_of::<ProcessStep>();
    let mut decoder = Process3dMutationDecodeAuthority::new(semio_framework_job::OperationId(7004), semio_framework_job::Generation(1), store::OwnedSchemaPath::ROOT);
    *decoder.retirement = Some(Box::new(Process3dOwnedRetirement::owner(Process3dRetirementOwner::Timeline { value: timeline })));
    decoder.state = Process3dMutationDecodeState::Closing;
    let funded = decoder.close_step(1, extent).unwrap();
    let mut demands = vec![extent];
    let mut undergrant_retained = true;
    let mut turns = 1;
    for _ in 1..row["maximumAdvertisedTurns"].as_u64().unwrap() {
        if decoder.terminal_is_empty() { break; }
        let demand = decoder.next_close_byte_demand().unwrap();
        demands.push(demand);
        if demand > extent {
            undergrant_retained &= matches!(decoder.close_step(1, extent).unwrap(), store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }) && decoder.retirement.is_some();
        }
        decoder.close_step(1, demand).unwrap();
        turns += 1;
    }
    let terminal = decoder.terminal_is_empty();
    while decoder.retirement.is_some() { process3d_retire_erased_step(&mut decoder.retirement, PROCESS3D_MAXIMUM_DOMAIN_BYTES).unwrap(); }
    decoder.state = Process3dMutationDecodeState::Complete;
    eprintln!("[DEBUG] Process mutation decoder backing={extent} exact-funded-step={funded:?} demands={demands:?} undergrant-retained={undergrant_retained} bounded-turns={turns} terminal={terminal}");
    assert_eq!(funded, store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: extent });
    assert!(undergrant_retained);
    assert_eq!(terminal, row["expectedTerminal"].as_bool().unwrap());
}

/// 🧵️ The retirement stack retains string backing under insufficient grants and advertises one unit for structural work.
#[test]
fn retained_owner_demand_distinguishes_structural_work_from_exact_string_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut value = String::new(); value.try_reserve_exact(extent).unwrap(); assert_eq!(value.capacity(), extent);
        let mut owner = Process3dRetirementStack::new(Process3dRetirementStack::one_string(value));
        let demand = owner.next_close_byte_demand();
        let step = owner.advance(caller);
        let retained = owner.len != 0;
        let retained_demand = owner.next_close_byte_demand();
        let reported = match &step { Ok(store::SnapshotRetirementStep::Pending { released_bytes, .. }) => *released_bytes, _ => usize::MAX };
        for _ in 0..16 { if owner.terminal_is_empty() { break; } owner.advance(admission).unwrap(); }
        assert!(owner.terminal_is_empty());
        eprintln!("[DEBUG] Process string extent={extent} demand={demand} caller={caller} step={step:?} retained={retained}");
        if demand != extent || reported != expected || (expected == 0 && (!retained || retained_demand != extent)) { failures.push((extent, demand, reported, retained)); }
    }
    let fields = Process3dMutationFields::from_mutation(Process3dMutation::ChangeStockLabel(crate::standards::v1::subsets::any::schema::mutations::change_stock_label::ChangeStockLabel { new_label: "neutral-label".into() }));
    let mut structural = Process3dRetirementStack::new(Process3dRetirementOwner::MutationFields { value: fields, phase: 0 });
    let work_demand = structural.next_close_byte_demand();
    let domain_fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let first_step = structural.advance(admission).unwrap();
    let structural_release = match first_step { store::SnapshotRetirementStep::Pending { released_bytes, .. } => released_bytes, _ => usize::MAX };
    for _ in 0..64 { if structural.terminal_is_empty() { break; } structural.advance(admission).unwrap(); }
    assert!(structural.terminal_is_empty());
    assert_eq!(structural_release, domain_fixture["structuralWork"]["physicalReleaseBytes"].as_u64().unwrap() as usize);
    assert_eq!(work_demand, domain_fixture["structuralWork"]["nextByteDemand"].as_u64().unwrap() as usize, "structural ownership advancement has no page-sized physical allocation to free");
    assert!(failures.is_empty(), "string owners retain their physical allocation under every neutral grant: {failures:?}");
}
