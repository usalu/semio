use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value::RetirementDemand;
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_mutations::standards::v1::subsets::any::io::binary::mutations::*;

const PROCESS3D_MAXIMUM_DOMAIN_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

const PROCESS3D_RETAINED_STACK_CAPACITY: usize = 64;

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

fn every_mutation() -> Vec<Process3dMutation> {
    process3d_all_retained_mutation_fixtures_for_test()
}

/// ♻️ Admits one original value to the neutral retained-owner retirement and returns the boxed owner.
fn admitted<T: semio_framework_value::retirement::RetireOwned>(value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), maximum_depth: 2, ..Default::default() };
    match semio_framework_value::retirement::admit_owned_retirement(value, birth) {
        Ok((owner, _)) => owner,
        Err((error, _value)) => panic!("retirement admission refused its own quote: {error}"),
    }
}

/// 📏️ The grant that funds exactly the next turn a quote advertises.
fn grant_of(quote: &RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: quote.copy_bytes, maximum_capacity_bytes: quote.capacity_bytes, maximum_release_bytes: quote.release_bytes, maximum_depth: quote.depth }
}

/// 🧹️ Drives an admitted owner to terminal-empty through grants sized exactly by its own quotes; answers the bytes it released.
fn close_to_empty(mut owner: Box<dyn store::ErasedSnapshotRetirement>) -> usize {
    let mut released = 0usize;
    let mut turns = 0usize;
    while !owner.terminal_is_empty() {
        let quote = owner.next_demand(PROCESS3D_MAXIMUM_DOMAIN_BYTES).expect("the owner quotes its next turn");
        assert!(quote.release_bytes <= PROCESS3D_MAXIMUM_DOMAIN_BYTES, "a quoted release stays inside the admitted domain bytes");
        let progress = owner.close_step(grant_of(&quote)).expect("the owner closes within its quoted grant").progress();
        assert!(progress.released_bytes <= quote.release_bytes);
        released += progress.released_bytes;
        turns += 1;
        assert!(turns < 1 << 20, "retirement made no progress");
    }
    released
}

/// 🧪️ Loads the retained-laws fixture shared by the retirement laws.
fn retained_fixture() -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🔬️retained-laws/🧫️fixtures/🔣️.json")).expect("retained-laws fixture")).unwrap()
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
        let demand = disposer.retirement_demands(&store, PROCESS3D_OWNER_BYTES).expect("the store quotes its next close turn");
        if matches!(disposer.close_step(&mut store, process3d_demand_grant(demand)), Ok(semio_framework_plugin::PluginLifecycleStep::Complete(_))) {
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
    let mut snapshot = semio_s_artifact_process_process3d_model::snapshot::empty_process3d_snapshot();
    snapshot.stock_label = label.into();
    let envelope = store::os_store::document_materialize::create_document_envelope(semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA, label, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut authority = store::ArtifactStoreReplayInitializer::<Process3dSnapshot, Process3dMutation, store::os_store::document_store_owners::BoundedDocumentStoreOwnerProducer>::new(envelope, semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA, operation, generation, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()));
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut complete = false;
    for _ in 0..PROCESS3D_MAXIMUM_DOMAIN_ITEMS {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(4_096, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
        let turn = measure_phase(budget, 0, || match semio_framework_plugin::ArtifactStoreInitializationAuthority::step(&mut authority, &mut context).expect("fixture initializer turn") {
            Some(semio_framework_job::JobOutcomeBorrow::Complete { .. }) => Some(true),
            Some(semio_framework_job::JobOutcomeBorrow::Cancelled { .. }) => panic!("fixture initializer cancelled"),
            Some(semio_framework_job::JobOutcomeBorrow::Fault { .. }) => panic!("fixture initializer faulted"),
            _ => None,
        });
        if turn == Some(true) {
            complete = true;
            break;
        }
    }
    assert!(complete, "fixture initializer must converge");
    let candidate = semio_framework_plugin::ArtifactStoreInitializationAuthority::take_candidate(&mut authority).expect("fixture candidate handoff");
    assert!(semio_framework_plugin::ArtifactStoreInitializationAuthority::terminal_is_empty(&authority));
    drop(authority);
    candidate
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
        let demand = disposer.retirement_demands(&displaced, PROCESS3D_OWNER_BYTES).expect("the displaced store quotes its next close turn");
        if matches!(measure_phase(&mut budget, 2, || disposer.close_step(&mut displaced, process3d_demand_grant(demand))), Ok(semio_framework_plugin::PluginLifecycleStep::Complete(_))) {
            break;
        }
    }
    assert!(disposer.terminal_is_empty(&displaced), "the displaced store must reach terminal-empty ownership within its bounded close");
    drop(displaced);
    close_store(live);
    budget
}

#[test]
fn interrupted_snapshot_close_reaches_terminal_empty() {
    let owner = admitted(semio_s_artifact_process_process3d_model::snapshot::empty_process3d_snapshot());
    assert!(!owner.terminal_is_empty(), "an admitted snapshot has work left");
    close_to_empty(owner);
}

#[test]
fn mounted_mutation_region_has_zero_whole_string_reader_edges() {
    let source = include_str!("../../🧬️mutation-decode/🦀️.rs");
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
            let mut grant_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut grant = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut grant_retained);
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

        close_to_empty(admitted(decoded));

        for interruption in 1..grants {
            let mut interrupted = Process3dRetainedMutationReader::new();
            for _ in 0..interruption {
                let mut grant_retained = semio_framework_job::RetainedCloneProgress::default();
                let mut grant = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut grant_retained);
                assert!(!interrupted.step(&bytes, &mut grant).expect("interrupted retained semantic grant"), "pre-terminal interruption must remain resumable");
                assert_eq!(grant.fuel_remaining(), 0, "one retained mutation substate consumes one grant");
            }
            let partial = interrupted.take_rejected().expect("every interrupted mutation substate hands back its exact partial owner");
            assert!(interrupted.terminal_is_empty());
            drop(interrupted);
            close_to_empty(admitted(partial));
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

/// 🧮️ The snapshot decoder quotes and funds the complete physical backing of a large retained timeline, one quoted turn at a time.
#[test]
fn snapshot_decoder_publishes_and_funds_large_timeline_backing() {
    use store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority as _;
    let fixture = retained_fixture();
    let row = &fixture["decoderBacking"];
    assert_eq!(row["queryEachDemand"].as_bool(), Some(true));
    let backing: Vec<ProcessStep> = Vec::with_capacity(row["capacityItems"].as_u64().unwrap() as usize);
    let extent = backing.capacity() * size_of::<ProcessStep>();
    assert!(extent > fixture["releaseBytes"].as_u64().unwrap() as usize && extent < PROCESS3D_MAXIMUM_DOMAIN_BYTES);
    let mut decoder = Process3dSnapshotDecodeAuthority::new(semio_framework_job::OperationId(7001), semio_framework_job::Generation(1), store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT);
    *decoder.retirement = Some(admitted(backing));
    decoder.state = Process3dSnapshotDecodeState::Closing;
    let mut released = 0usize;
    let mut turns = 0u64;
    while !decoder.terminal_is_empty() && turns < row["maximumAdvertisedTurns"].as_u64().unwrap() {
        let quote = decoder.close_demands(PROCESS3D_MAXIMUM_DOMAIN_BYTES).expect("the decoder quotes its next turn");
        assert!(quote.release_bytes <= PROCESS3D_MAXIMUM_DOMAIN_BYTES);
        released += decoder.close_step(grant_of(&quote)).expect("the decoder closes within its quoted grant").progress().released_bytes;
        turns += 1;
    }
    let terminal = decoder.terminal_is_empty();
    decoder.state = Process3dSnapshotDecodeState::Complete;
    assert!(released >= extent, "the quoted turns fund the whole physical backing");
    assert_eq!(terminal, row["expectedTerminal"].as_bool().unwrap());
}

/// 🧮️ A nested mutation decoder forwards the same complete physical backing grant as its snapshot sibling and retains the backing under an insufficient grant.
#[test]
fn mutation_decoder_forwards_large_retained_backing_grant() {
    use store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority as _;
    let fixture = retained_fixture();
    let row = &fixture["decoderBacking"];
    assert_eq!(row["queryEachDemand"].as_bool(), Some(true));
    let backing: Vec<ProcessStep> = Vec::with_capacity(row["capacityItems"].as_u64().unwrap() as usize);
    let extent = backing.capacity() * size_of::<ProcessStep>();
    let mut decoder = Process3dMutationDecodeAuthority::new(semio_framework_job::OperationId(7004), semio_framework_job::Generation(1), store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT);
    *decoder.retirement = Some(admitted(backing));
    decoder.state = Process3dMutationDecodeState::Closing;
    let mut released = 0usize;
    let mut undergrant_retained = true;
    let mut turns = 0u64;
    while !decoder.terminal_is_empty() && turns < row["maximumAdvertisedTurns"].as_u64().unwrap() {
        let quote = decoder.close_demands(PROCESS3D_MAXIMUM_DOMAIN_BYTES).expect("the decoder quotes its next turn");
        if quote.release_bytes > 0 {
            let before = decoder.retirement.is_some();
            let step = decoder.close_step(RetainedCloneGrant { maximum_release_bytes: quote.release_bytes - 1, ..grant_of(&quote) }).expect("an insufficient grant is a refusal to advance, not a fault");
            undergrant_retained &= matches!(step, RetainedCloneStep::Progress(progress) if progress.released_bytes == 0) && decoder.retirement.is_some() == before;
        }
        released += decoder.close_step(grant_of(&quote)).expect("the decoder closes within its quoted grant").progress().released_bytes;
        turns += 1;
    }
    let terminal = decoder.terminal_is_empty();
    decoder.state = Process3dMutationDecodeState::Complete;
    assert!(released >= extent, "the quoted turns fund the whole physical backing");
    assert!(undergrant_retained);
    assert_eq!(terminal, row["expectedTerminal"].as_bool().unwrap());
}

/// 🧵️ A retained string owner quotes its exact physical allocation, keeps it under every insufficient grant and releases it under a sufficient one.
#[test]
fn retained_owner_demand_distinguishes_structural_work_from_exact_string_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../../../../🧰️framework/🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).expect("close-demand fixture")).unwrap();
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut value = String::new();
        value.try_reserve_exact(extent).unwrap();
        assert_eq!(value.capacity(), extent);
        let mut owner = admitted(value);
        let quote = owner.next_demand(PROCESS3D_MAXIMUM_DOMAIN_BYTES).expect("the owner quotes its next turn");
        let step = owner.close_step(RetainedCloneGrant { maximum_release_bytes: caller, ..grant_of(&quote) });
        let reported = match &step { Ok(step) => step.progress().released_bytes, Err(_) => 0 };
        let retained = !owner.terminal_is_empty();
        let retained_quote = owner.next_demand(PROCESS3D_MAXIMUM_DOMAIN_BYTES).map(|quote| quote.release_bytes).unwrap_or(usize::MAX);
        close_to_empty(owner);
        if quote.release_bytes != extent || reported != expected || (expected == 0 && (!retained || retained_quote != extent)) {
            failures.push((extent, quote.release_bytes, reported, retained));
        }
    }
    assert!(failures.is_empty(), "string owners retain their physical allocation under every neutral grant: {failures:?}");
    let structural = admitted(Process3dMutation::ChangeStockLabel(semio_s_artifact_process_process3d_mut_stock::change_stock_label::ChangeStockLabel { new_label: "neutral-label".into() }));
    let work = structural.next_demand(PROCESS3D_MAXIMUM_DOMAIN_BYTES).expect("the structural owner quotes its next turn");
    assert!(work.release_bytes < PROCESS3D_MAXIMUM_DOMAIN_BYTES, "structural ownership advancement has no page-sized physical allocation to free");
    assert!(close_to_empty(structural) < PROCESS3D_MAXIMUM_DOMAIN_BYTES);
}

