//! 🏃️ Synchronous headless driver over the same retained worker session the interactive hosts submit.

use semio_framework_job::{root_cancel_token, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobOutcomeView, Operation, OperationId, OriginalTurnFailure, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, StepBudget, StepContext};
use semio_framework_job_payload::{RetainedJobPayload};
use semio_framework_job_worker::{BatchJobSession,WorkerJobDemandError,WorkerJobPoll};use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams,WorkerJobCloseStep};
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🎟️ The bounded five-currency authority one headless caller supplies for every admitted turn.
pub const HEADLESS_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🧭️ Names one headless caller for trace sites and error text.
#[derive(Clone, Copy)]
pub struct HeadlessSite {
    pub site: &'static str,
    pub label: &'static str,
    pub stage: InteractiveStage,
    pub fuel_per_step: u64,
    pub step_budget_us: u64,
}

/// 📦️ Concatenates every page of a retained payload — the byte view a worker session reassembles.
pub fn payload_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).expect("retained payload page").iter().copied()).collect()
}

enum Seen {
    Pending,
    Complete(Vec<u8>),
    Cancelled,
    Fault(Vec<u8>),
}

fn self_funded(demand: semio_framework_value::RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 🧾️ Admits the one payload authority a locally driven operation keeps across every step, so its publications share one ledger.
pub fn admit_payload_authority(operation: Operation) -> semio_framework_job_payload::JobPayloadAuthority {
    semio_framework_job_payload::JobPayloadAuthority::admit(operation.operation, operation.generation, HEADLESS_GRANT)
        .expect("the headless wallet quotes its payload authority")
        .expect("the headless wallet admits its payload authority")
        .0
}

/// 🧹️ Closes the authority through its own quoted grants once every payload sharing its ledger is closed.
pub fn close_payload_authority(authority: &mut semio_framework_job_payload::JobPayloadAuthority) {
    while !authority.terminal_is_empty() {
        let step = authority.close_step(self_funded(authority.retirement_demands()));
        assert!(step.is_ok(), "a locally owned payload authority closes without an external owner");
    }
}

/// 🚪️ Runs a job's own close ladder to completion with grants it quotes itself — every job must be closed before it is dropped.
pub fn close_job(job: &mut impl InteractiveJob) {
    job.begin_close();
    for _ in 0..2_000_000 {
        if job.terminal_is_empty() {
            return;
        }
        let demand = semio_framework_value::RetirementDemand {
            copy_bytes: job.next_close_copy_byte_demand().expect("a locally owned job quotes its copy demand"),
            capacity_bytes: job.next_close_capacity_byte_demand(HEADLESS_GRANT.maximum_copy_bytes).expect("a locally owned job quotes its capacity demand"),
            release_bytes: job.next_close_release_byte_demand().expect("a locally owned job quotes its release demand"),
            depth: job.next_close_depth_demand().expect("a locally owned job quotes its depth demand"),
        };
        let step = semio_framework_job::close_with_self_funded_context(self_funded(demand), |cx| job.close_step(cx)).expect("a locally owned job close turn");
        assert!(step.progress().fits(self_funded(demand)), "a job close receipt must fit its own quoted grant");
        assert!(!matches!(step, InteractiveJobCloseStep::Blocked), "a locally owned job close has no external owner and no refusal");
    }
    panic!("job did not close");
}

const HEADLESS_TURNS: usize = 8;
const HEADLESS_ACKNOWLEDGE_TURNS: usize = 1_000_000;
const HEADLESS_CLOSE_TURNS: usize = 2_000_000;

/// 🪪️ The original caller of one headless run: it owns the operation's one cancellation root and payload authority and lends a fresh
/// self-funded context per turn, so admission, every step, every acknowledgement and the whole close ladder are paid by the same caller.
struct HeadlessCaller {
    operation: OperationId,
    generation: Generation,
    now_us: fn() -> Option<u64>,
    cancel: CancelToken,
    authority: semio_framework_job_payload::JobPayloadAuthority,
    sequence: u64,
}

impl HeadlessCaller {
    fn new(operation: Operation, now_us: fn() -> Option<u64>) -> Self {
        Self { operation: operation.operation, generation: operation.generation, now_us, cancel: root_cancel_token(), authority: admit_payload_authority(operation), sequence: 0 }
    }

    fn turn<R>(&mut self, budget: StepBudget, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        let mut receipt = RetainedCloneProgress::default();
        let mut cx = StepContext::with_payload_authority(self.operation, self.generation, budget, &self.cancel, self.now_us, &mut self.sequence, &mut receipt, &self.authority)?;
        run(&mut cx)
    }

    fn funded<R>(&mut self, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        self.turn(StepBudget::new(u64::MAX, u64::MAX, HEADLESS_GRANT), run)
    }

    fn step_turn<R>(&mut self, site: &HeadlessSite, run: impl FnOnce(&mut StepContext<'_>) -> Result<R, OriginalTurnFailure>) -> Result<R, OriginalTurnFailure> {
        let now = (self.now_us)().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "headless caller has no monotonic clock"))?;
        let budget = StepBudget::from_duration(site.fuel_per_step, now, site.step_budget_us, HEADLESS_GRANT).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, "headless step deadline leaves the clock domain"))?;
        self.turn(budget, run)
    }

    fn admit_alias(&mut self) -> Result<Option<CancelToken>, OriginalTurnFailure> {
        self.funded(|cx| {
            let alias = cx.admit_original_cancel_alias()?;
            if alias.is_some() {
                cx.consume_fuel(1);
            }
            Ok(alias)
        })
    }

    fn close_owner<J: InteractiveJob + 'static>(&mut self, session: &mut BatchJobSession<J>, grant: RetainedCloneGrant) -> Result<WorkerJobCloseStep, OriginalTurnFailure> {
        self.turn(StepBudget::new(u64::MAX, u64::MAX, grant), |cx| {
            let remaining = cx.retained_grant();
            if cx.should_yield() || remaining.maximum_items == 0 || remaining.maximum_depth == 0 {
                return Ok(WorkerJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
            }
            let witnessed = match session.has_original_cancel_alias_witness(cx.original_cancel_token()) {
                Ok(witnessed) => witnessed,
                Err(WorkerJobDemandError::Contention(_)) => return Ok(WorkerJobCloseStep::Blocked),
                Err(WorkerJobDemandError::Refused(error)) => return Err(error.into()),
            };
            if !witnessed {
                return session.close_step(cx);
            }
            match session.return_original_cancel_alias_step(cx.original_cancel_token(), remaining) {
                Ok(Some(step)) => {
                    cx.consume_retained(step.progress())?;
                    cx.consume_fuel(1);
                    Ok(WorkerJobCloseStep::Pending { progress: step.progress() })
                }
                Ok(None) | Err(WorkerJobDemandError::Contention(_)) => Ok(WorkerJobCloseStep::Blocked),
                Err(WorkerJobDemandError::Refused(error)) => Err(OriginalTurnFailure::debit_original(error, cx)),
            }
        })
    }

    fn retire(mut self) {
        close_payload_authority(&mut self.authority);
    }
}

/// 🏁️ Drives `job` through one retained worker session to its terminal outcome. `finish` runs while the completed job is still
/// checked out, with the completed output bytes, and its value becomes the result.
pub fn run_headless<J, R>(job: J, operation: Operation, now_us: fn() -> Option<u64>, site: HeadlessSite, finish: impl FnOnce(&mut J, &[u8]) -> Result<R, String>) -> Result<R, String>
where
    J: InteractiveJob + Send + 'static,
{
    let label = site.label;
    let mut caller = HeadlessCaller::new(operation, now_us);
    let alias = (0..HEADLESS_TURNS).find_map(|_| caller.admit_alias().transpose());
    let Some(Ok(alias)) = alias else {
        let mut unadmitted = job;
        close_job(&mut unadmitted);
        caller.retire();
        return Err(format!("{label}-admission-rejected"));
    };
    let mut job_slot = Some(job);
    let mut params_slot = Some(BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: alias,
        config: BatchDriveConfig { retained: HEADLESS_GRANT, site: site.site, stage: site.stage, fuel_per_step: site.fuel_per_step, step_budget_us: site.step_budget_us },
        now_us,
    });
    let admitted = caller.funded(|cx| BatchJobSession::try_admit_owned(&mut job_slot, &mut params_slot, cx).map_err(OriginalTurnFailure::from));
    let mut session = match admitted {
        Ok(Some((session, _admission))) => session,
        Ok(None) | Err(_) => {
            drop(params_slot.take());
            if let Some(mut rejected) = job_slot.take() {
                close_job(&mut rejected);
            }
            caller.retire();
            return Err(format!("{label}-admission-rejected"));
        }
    };
    let result = drive_session(&mut caller, &mut session, &site, finish);
    session.begin_close();
    for _ in 0..HEADLESS_CLOSE_TURNS {
        if session.terminal_is_empty() {
            caller.retire();
            return result;
        }
        let demand = match session.retirement_demands(HEADLESS_GRANT.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(WorkerJobDemandError::Contention(_)) => {
                std::thread::yield_now();
                continue;
            }
            Err(WorkerJobDemandError::Refused(error)) => panic!("the {label} session quoted a refused close demand: {error:?}"),
        };
        let grant = RetainedCloneGrant {
            maximum_items: HEADLESS_GRANT.maximum_items,
            maximum_copy_bytes: HEADLESS_GRANT.maximum_copy_bytes.max(demand.copy_bytes),
            maximum_capacity_bytes: HEADLESS_GRANT.maximum_capacity_bytes.max(demand.capacity_bytes),
            maximum_release_bytes: HEADLESS_GRANT.maximum_release_bytes.max(demand.release_bytes),
            maximum_depth: HEADLESS_GRANT.maximum_depth.max(demand.depth),
        };
        match caller.close_owner(&mut session, grant) {
            Ok(WorkerJobCloseStep::Pending { progress } | WorkerJobCloseStep::Complete { progress }) => assert!(progress.fits(grant), "the {label} session close receipt exceeded its quoted grant"),
            Ok(WorkerJobCloseStep::Blocked) => std::thread::yield_now(),
            Err(failure) => panic!("the {label} session close was refused: {failure:?}"),
        }
    }
    panic!("the {label} session's close ladder never reached terminal-empty");
}

fn drive_session<J, R>(caller: &mut HeadlessCaller, session: &mut BatchJobSession<J>, site: &HeadlessSite, finish: impl FnOnce(&mut J, &[u8]) -> Result<R, String>) -> Result<R, String>
where
    J: InteractiveJob + Send + 'static,
{
    let label = site.label;
    let mut finish = Some(finish);
    loop {
        match caller.step_turn(site, |cx| session.step_on_original_caller(cx)) {
            Ok(WorkerJobPoll::Outcome | WorkerJobPoll::Terminal) => {}
            Ok(_) => continue,
            Err(failure) => return Err(format!("{label}-turn:{failure:?}")),
        }
        if !session.checkout_outcome() {
            continue;
        }
        let Some((issued, progress)) = session.take_checked_out_retained_step_receipt() else {
            return Err(format!("{label}-omitted-step-receipt"));
        };
        if !progress.fits(issued) {
            return Err(format!("{label}-step-receipt-exceeded-grant"));
        }
        let seen = match session.checked_out_outcome() {
            Ok(None) | Ok(Some(JobOutcomeView::Yield { .. } | JobOutcomeView::PreviewReady { .. } | JobOutcomeView::CheckpointReady { .. })) => Seen::Pending,
            Ok(Some(JobOutcomeView::Complete { output, .. })) => Seen::Complete(output.map(payload_bytes).unwrap_or_default()),
            Ok(Some(JobOutcomeView::Cancelled { .. })) => Seen::Cancelled,
            Ok(Some(JobOutcomeView::Fault { detail, .. })) => Seen::Fault(payload_bytes(detail)),
            Err(error) => return Err(format!("{label}-outcome:{}", error.into_message())),
        };
        let result = match seen {
            Seen::Pending => None,
            Seen::Complete(output) => Some(match (session.checked_out_job_mut(), finish.take()) {
                (Some(job), Some(finish)) => finish(job, &output),
                _ => Err(format!("{label}-completed-without-job")),
            }),
            Seen::Cancelled => Some(Err(format!("{label}-cancelled"))),
            Seen::Fault(detail) => Some(Err(String::from_utf8_lossy(&detail).into_owned())),
        };
        for _ in 0..HEADLESS_ACKNOWLEDGE_TURNS {
            let step = caller.funded(|cx| {
                let step = session.acknowledge_outcome(cx.retained_grant());
                cx.consume_retained(step.progress())?;
                cx.consume_fuel(1);
                Ok(step)
            });
            match step {
                Ok(RetainedCloneStep::Complete(_)) => break,
                Ok(_) => {}
                Err(failure) => return Err(format!("{label}-acknowledge:{failure:?}")),
            }
        }
        if let Some(result) = result {
            return result;
        }
        session.resume().map_err(|error| format!("{label}-resume:{error:?}"))?;
    }
}
