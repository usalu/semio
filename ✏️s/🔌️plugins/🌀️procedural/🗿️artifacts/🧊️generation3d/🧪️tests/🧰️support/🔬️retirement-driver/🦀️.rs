//! 🧪️ The grant drivers every retained-clone close and step test of this artifact shares: each currency is granted exactly its quoted demand, so a test that passes proves the quote itself.

#![allow(dead_code)]

use semio_framework_job::{CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobOutcomeBorrow, OperationId, OriginalJobFixtureCaller, OriginalJobReturn, OriginalJobReturnOrigin, StepBudget, StepContext};
use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};

/// 🎟️ The retained policy one driven step runs under.
pub const STEP_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// ⛽️ The fuel each driven preparation turn is lent, so a step's own fuel and deadline are spent on the job and never on its receiving frame.
pub const PREPARATION_FUEL: u64 = 1 << 16;

/// 🪙️ The finite original source stock one driven fixture caller lends its turns.
pub const SOURCE_STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 32, maximum_capacity_bytes: 1 << 32, maximum_release_bytes: 1 << 32, maximum_depth: 128 };

/// 🎟️ Admits the original fixture caller one driven close or step runs under.
fn fixture_caller(operation: OperationId, generation: Generation, deadline_us: u64, fuel: u64) -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(operation, generation, SOURCE_STOCK, semio_framework_job::default_now_us, deadline_us, fuel)
}

/// 🎟️ A one-item grant paying exactly `demand` in every currency.
pub fn exact_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 📏️ The next close quote of one erased retirement.
pub fn erased_demand(owner: &dyn store::ErasedSnapshotRetirement) -> Result<RetirementDemand, ValueError> {
    let copy = owner.next_copy_byte_demand()?;
    Ok(RetirementDemand { copy_bytes: copy, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
}

/// 🧹️ Drives one erased retirement to its terminal-empty witness under exactly its quoted grants and answers the released bytes.
pub fn drive_erased_released(owner: &mut dyn store::ErasedSnapshotRetirement, label: &str) -> usize {
    let mut released = 0usize;
    for _ in 0..1_000_000 {
        if owner.terminal_is_empty() {
            return released;
        }
        let grant = exact_grant(erased_demand(owner).unwrap_or_else(|error| panic!("{label} refused its close quote: {error}")));
        let step = paid(grant, |effect| owner.close_step(grant.maximum_copy_bytes, effect), label);
        admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "driven test retirement").unwrap_or_else(|error| panic!("{label} overspent its grant: {error}"));
        released += step.progress().released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(owner.terminal_is_empty(), "{label} reported Complete without its exact terminal-empty witness");
            return released;
        }
    }
    panic!("{label} did not reach its terminal-empty close witness")
}

/// 🪙️ Runs one effect-shaped retirement step inside a finite wallet of exactly `grant`; a pending or unaffordable step paid nothing.
fn paid(grant: RetainedCloneGrant, run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::RetainedWorkPoll<RetainedCloneStep, semio_framework_value::OriginalRetirementFailure>, label: &str) -> RetainedCloneStep {
    let polled = semio_framework_job::OriginalRetirementPump::new().turn(semio_plugin::artifact_app_laws::FIXTURE_EFFECT_WORK, u64::MAX, semio_framework_job::logical_now_us, grant).and_then(|mut turn| turn.with_effect(run)).expect("driven retirement has a finite original Source turn").map(|(polled, _)| polled);
    match polled {
        Some(semio_framework_value::RetainedWorkPoll::Complete(turn)) => turn.result.unwrap_or_else(|error| panic!("{label} retirement faulted: {error:?}")),
        Some(semio_framework_value::RetainedWorkPoll::Pending { .. }) | None => RetainedCloneStep::Progress(Default::default()),
    }
}

/// 🎟️ Admits a pending value as an owned retirement, each admission turn paying exactly its quoted frame birth.
fn admit_birth<T>(mut pending: Option<T>, demand: impl Fn(&Option<T>) -> Result<RetirementDemand, ValueError>, admit: impl Fn(&mut Option<T>, &mut Option<Box<dyn store::ErasedSnapshotRetirement>>, &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::RetainedWorkOutcome, label: &str) -> Box<dyn store::ErasedSnapshotRetirement> {
    let mut active = None;
    for _ in 0..64 {
        let grant = exact_grant(demand(&pending).unwrap_or_else(|error| panic!("{label} refused its own birth quote: {error}")));
        let grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth.max(2), ..grant };
        semio_plugin::artifact_app_laws::run_fixture_effect(grant, |effect| admit(&mut pending, &mut active, effect).result.unwrap_or_else(|error| panic!("{label} refused its own birth: {error}")));
        if let Some(owner) = active.take() {
            return owner;
        }
    }
    panic!("{label} never admitted its own birth")
}

/// 🎟️ Admits `value` as an owned retirement under a grant that pays exactly its frame birth.
pub fn retire_owned_for_test<T: semio_framework_value::retirement::RetireOwned>(value: T, label: &str) -> Box<dyn store::ErasedSnapshotRetirement> {
    admit_birth(Some(value), store::os_store::snapshot_read::artifact_retirement_owned_birth_demands, store::os_store::snapshot_read::artifact_retirement_admit_owned, label)
}

/// 🔗️ Admits `value` as a shared (`Arc`) retirement lease under a grant that pays exactly its frame birth.
pub fn retire_shared_for_test<T: semio_framework_value::retirement::RetireOwned + Sync>(value: std::sync::Arc<T>, label: &str) -> Box<dyn store::ErasedSnapshotRetirement> {
    admit_birth(Some(value), store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands, store::os_store::snapshot_read::artifact_retirement_admit_shared_lease, label)
}

/// 📏️ The next close grant of one interactive job.
pub fn job_grant<J: InteractiveJob + ?Sized>(job: &J) -> RetainedCloneGrant {
    let copy = job.next_close_copy_byte_demand().expect("the job quotes its copy demand");
    exact_grant(RetirementDemand { copy_bytes: copy, capacity_bytes: job.next_close_capacity_byte_demand(copy).expect("the job quotes its capacity demand"), release_bytes: job.next_close_release_byte_demand().expect("the job quotes its release demand"), depth: job.next_close_depth_demand().expect("the job quotes its depth demand") })
}

/// 🧹️ Begins the close of one interactive job and drives it to its terminal-empty witness under exactly its quoted grants; answers the released bytes.
pub fn close_job<J: InteractiveJob + ?Sized>(job: &mut J, label: &str) -> usize {
    job.begin_close();
    let mut released = 0usize;
    let mut caller = fixture_caller(semio_framework_job::allocate_operation_id(), Generation(1), u64::MAX, 1 << 40);
    for _ in 0..10_000_000 {
        let grant = job_grant(job);
        match caller.turn(grant, |cx| job.close_step(cx)).expect("original fixture close turn") {
            InteractiveJobCloseStep::Complete { progress } => {
                assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant");
                released += progress.released_bytes;
                assert!(job.terminal_is_empty(), "{label} completed without its terminal-empty witness");
                caller.retire(SOURCE_STOCK);
                return released;
            }
            InteractiveJobCloseStep::Pending { progress } => {
                assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant");
                released += progress.released_bytes;
            }
            InteractiveJobCloseStep::Blocked => panic!("{label} close blocked under its exact quoted grant"),
        }
    }
    panic!("{label} close never completed")
}

/// 🧹️ Begins the close of one command work and drives it to its terminal-empty witness under exactly its quoted grants.
pub fn close_work<A: semio_plugin::ArtifactApp, W: semio_plugin::retained_command::ArtifactCommandWork<A> + ?Sized>(work: &mut W, label: &str) {
    work.begin_close();
    let mut caller = fixture_caller(semio_framework_job::allocate_operation_id(), Generation(1), u64::MAX, 1 << 40);
    for _ in 0..10_000_000 {
        let copy = work.next_close_copy_byte_demand().unwrap_or_else(|error| panic!("{label} copy quote refused: {error}"));
        let grant = exact_grant(RetirementDemand {
            copy_bytes: copy,
            capacity_bytes: work.next_close_capacity_byte_demand(copy).unwrap_or_else(|error| panic!("{label} capacity quote refused: {error}")),
            release_bytes: work.next_close_release_byte_demand().unwrap_or_else(|error| panic!("{label} release quote refused: {error}")),
            depth: work.next_close_depth_demand().unwrap_or_else(|error| panic!("{label} depth quote refused: {error}")),
        });
        match caller.turn(grant, |cx| work.close_step(cx)).expect("original fixture close turn") {
            InteractiveJobCloseStep::Complete { progress } => {
                assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant");
                assert!(work.terminal_is_empty(), "{label} completed without its terminal-empty witness");
                caller.retire(SOURCE_STOCK);
                return;
            }
            InteractiveJobCloseStep::Pending { progress } => assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant"),
            InteractiveJobCloseStep::Blocked => panic!("{label} close blocked under its exact quoted grant"),
        }
    }
    panic!("{label} close never completed")
}

/// 🧹️ Spends one exact-grant close turn of the instance owner `O` behind `handle`; answers whether it reached its terminal-empty witness.
pub fn close_instance_owner_turn<O: semio_plugin::ArtifactInstanceOperationOwner + 'static>(handle: &semio_plugin::ArtifactInstanceOperationOwnerHandle) -> bool {
    handle
        .with_mut::<O, _>(|owner| {
            let demand = owner.retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| semio_plugin::Fault::from(error.to_string()))?;
            let grant = exact_grant(demand);
            let mut sequence = 0;
            let mut receipt = RetainedCloneProgress::default();
            let mut cx = StepContext::new(semio_framework_job::allocate_operation_id(), Generation(1), StepBudget::new(1, u64::MAX, grant), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence, &mut receipt);
            let step = owner.close_step(&mut cx, grant).map_err(|error| semio_plugin::Fault::from(format!("{error:?}")))?;
            Ok(matches!(step, semio_plugin::PluginLifecycleStep::Complete(_)))
        })
        .expect("the owner lends itself to its close ladder")
}

/// 🧹️ Closes the instance owner `O` behind `handle` to its terminal-empty witness.
pub fn close_instance_owner<O: semio_plugin::ArtifactInstanceOperationOwner + 'static>(handle: &semio_plugin::ArtifactInstanceOperationOwnerHandle, label: &str) {
    for _ in 0..1_000_000 {
        if close_instance_owner_turn::<O>(handle) {
            return;
        }
    }
    panic!("{label} did not reach terminal-empty")
}

/// 🧭️ What one driven call of a job lent to its caller.
#[derive(Debug, PartialEq, Eq)]
pub enum Stepped {
    Yield,
    Preview(Vec<u8>),
    Checkpoint,
    Complete,
    Fault,
    Cancelled,
}

/// 🦶️ Drives one step of `job` under `fuel` and `deadline_us` through its own original source and return receiver, and answers what it lent; a step that is unfunded or past its deadline cannot enter the job and yields.
pub fn step_once<J: InteractiveJob + ?Sized>(job: &mut J, fuel: u64, deadline_us: u64, _cancel: &CancelToken, operation: OperationId, generation: Generation, _sequence: &mut u64) -> Stepped {
    let mut session = StepSession::open(operation, generation, fuel.saturating_add(1 << 20));
    let answered = session.step(job, fuel, deadline_us);
    session.close();
    answered
}

/// 🪙️ One original source that lends its turns to many consecutive driven steps of one job, so work a job constructs across steps stays bound to the source that admitted it.
pub struct StepSession {
    caller: OriginalJobFixtureCaller,
    decoder: semio_framework_job::OriginalJobDecodeOwner,
    replacement: Option<semio_framework_job::ReplacementIdentity>,
}

impl StepSession {
    /// 🎟️ Admits the source and its return table once, with `fuel` as its whole lifetime work.
    pub fn open(operation: OperationId, generation: Generation, fuel: u64) -> Self {
        let mut caller = fixture_caller(operation, generation, u64::MAX, fuel);
        caller.admit_return_table();
        let decoder = caller.admitted_decoder();
        Self { caller, decoder, replacement: None }
    }

    /// 🧷️ Installs `current` as the store's current replacement identity for every later step of this session.
    pub fn under_replacement(mut self, current: semio_framework_job::ReplacementIdentity) -> Self {
        self.replacement = Some(current);
        self
    }

    /// 🏁️ Retires the decoder, then the source stock.
    pub fn close(mut self) {
        for _ in 0..65_536 {
            if self.decoder.terminal_is_empty() {
                break;
            }
            let (quote, _) = semio_framework_job::original_decoder_teardown_quote(&self.decoder).expect("the decoder quotes its teardown");
            let decoder = &mut self.decoder;
            self.caller.turn(quote, |cx| { decoder.close_on_source(cx)?; Ok(()) }).expect("original decoder teardown turn");
        }
        assert!(self.decoder.terminal_is_empty(), "the step decoder did not reach terminal-empty");
        self.caller.retire(SOURCE_STOCK);
    }

    /// 🦶️ Drives one step of `job` through a fresh return receiver of this source.
    pub fn step<J: InteractiveJob + ?Sized>(&mut self, job: &mut J, fuel: u64, deadline_us: u64) -> Stepped {
        step_in(&mut self.caller, &mut self.decoder, self.replacement, job, fuel, deadline_us)
    }
}

fn step_in<J: InteractiveJob + ?Sized>(caller: &mut OriginalJobFixtureCaller, decoder: &mut semio_framework_job::OriginalJobDecodeOwner, replacement: Option<semio_framework_job::ReplacementIdentity>, job: &mut J, fuel: u64, deadline_us: u64) -> Stepped {
    let mut origin = OriginalJobReturnOrigin::empty();
    let one = RetainedCloneGrant { maximum_items: 1, ..SOURCE_STOCK };
    let mut ready = false;
    for _ in 0..64 {
        caller.refreshed_turn(one, |cx| { let quote = cx.original_source_parent()?.loan_admission_work_demand()?; cx.with_original_work_loan(quote, cx.original_absolute_deadline_us(), |cx| { ready = origin.admit(cx)?; Ok(()) }) }).expect("original return source admission");
        if ready {
            break;
        }
    }
    assert!(ready, "the original return source never became ready");
    let mut prepared = false;
    for _ in 0..64 {
        caller.refreshed_turn(STEP_GRANT, |cx| cx.with_original_work_loan(PREPARATION_FUEL, cx.original_absolute_deadline_us(), |cx| { prepared = origin.receiver()?.prepare(cx)?; Ok(()) })).expect("original return preparation");
        if prepared {
            break;
        }
    }
    assert!(prepared, "the original return receiver never claimed its outcome row");
    let mut answered = None;
    let mut verdict = None;
    let mut pending = None;
    caller
        .refreshed_turn_with_decoder(STEP_GRANT, decoder, |cx| {
            let mut run = |cx: &mut StepContext<'_>| cx.with_original_work_loan(fuel, deadline_us, |cx| {
                match semio_framework_job::drive_step(job, cx, origin.receiver()?, "procedural.test", InteractiveStage::InteractiveStep, &mut verdict)? {
                    OriginalJobReturn::Preparing | OriginalJobReturn::Pending => {}
                    OriginalJobReturn::Outcome(reply) => {
                        answered = Some(match reply.original() {
                            None | Some(JobOutcomeBorrow::Yield { .. }) => Stepped::Yield,
                            Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => Stepped::Preview((0..payload.page_count()).flat_map(|index| payload.page(index).expect("preview page").to_vec()).collect()),
                            Some(JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Checkpoint,
                            Some(JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
                            Some(JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
                            Some(JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
                        });
                        pending = Some(reply.into_descriptor());
                    }
                    OriginalJobReturn::Continuation(continuation) => panic!("job step returned a continuation: {:?}", continuation.view().callback.map(|failure| format!("{failure:?}"))),
                }
                if pending.is_some() {
                    origin.receive_outcome(cx, &mut pending)?;
                }
                Ok(())
            });
            match replacement {
                Some(current) => cx.with_replacement_current(current, run),
                None => run(cx),
            }
        })
        .expect("job step admission");
    for _ in 0..64 {
        if origin.outcome().is_none() {
            break;
        }
        caller.refreshed_turn(one, |cx| { let quote = cx.original_source_parent()?.loan_admission_work_demand()?; cx.with_original_work_loan(quote, cx.original_absolute_deadline_us(), |cx| origin.acknowledge_outcome(cx).map(|_| ())) }).expect("original outcome acknowledgement");
    }
    for _ in 0..64 {
        if origin.terminal_is_empty() {
            break;
        }
        caller.refreshed_turn(one, |cx| origin.close_step(cx).map(|_| ())).expect("original return source close");
    }
    assert!(origin.terminal_is_empty(), "the original return source did not close");
    answered.unwrap_or(Stepped::Yield)
}

/// 🦶️ Steps `job` with a fresh operation until it leaves `Yield` and answers the terminal (or preview) it lent.
pub fn step_until_settled<J: InteractiveJob + ?Sized>(job: &mut J, fuel: u64, limit: usize) -> Stepped {
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    for _ in 0..limit {
        let now = semio_framework_job::default_now_us().expect("clock");
        match step_once(job, fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4, &cancel, operation, generation, &mut sequence) {
            Stepped::Yield => {}
            settled => return settled,
        }
    }
    panic!("the job never left Yield")
}
