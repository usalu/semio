//! 🏃️ Synchronous headless driver over the same retained worker session the interactive hosts submit.

use semio_framework_job::{
    root_cancel_token, BatchDriveConfig, BatchJobParams, BatchJobSession, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobOutcomeView, Operation, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedJobPayload, StepBudget,
    WorkerJobAdmissionContext, WorkerJobCloseStep, WorkerJobPoll,
};

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
pub fn admit_payload_authority(operation: Operation) -> semio_framework_job::JobPayloadAuthority {
    semio_framework_job::JobPayloadAuthority::admit(operation.operation, operation.generation, HEADLESS_GRANT)
        .expect("the headless wallet quotes its payload authority")
        .expect("the headless wallet admits its payload authority")
        .0
}

/// 🧹️ Closes the authority through its own quoted grants once every payload sharing its ledger is closed.
pub fn close_payload_authority(authority: &mut semio_framework_job::JobPayloadAuthority) {
    while !authority.terminal_is_empty() {
        let step = authority.close_step(self_funded(authority.retirement_demands()));
        assert!(step.is_ok(), "a locally owned payload authority closes without an external owner");
    }
}

/// 🧪️ Test-only step context whose operation keeps one leaked payload authority across steps, as the worker does.
#[cfg(test)]
pub(crate) fn test_step_context<'a>(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, budget: StepBudget, cancel: semio_framework_job::CancelToken, sequence: &'a mut u64, receipt: &'a mut RetainedCloneProgress) -> semio_framework_job::StepContext<'a> {
    use std::cell::RefCell;
    use std::collections::HashMap;
    thread_local! {
        static AUTHORITIES: RefCell<HashMap<(u64, u64), &'static semio_framework_job::JobPayloadAuthority>> = RefCell::new(HashMap::new());
    }
    let authority = AUTHORITIES.with(|map| {
        *map.borrow_mut().entry((operation.0, generation.0)).or_insert_with(|| Box::leak(Box::new(admit_payload_authority(Operation::new(operation, semio_framework_job::RevisionId(0), generation, 0)))))
    });
    let cancel: &'static semio_framework_job::CancelToken = Box::leak(Box::new(cancel));
    semio_framework_job::StepContext::with_payload_authority(operation, generation, budget, cancel, || Some(0), sequence, receipt, authority).expect("test context admits its payload authority")
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
        let step = job.close_step(self_funded(demand));
        assert!(step.progress().fits(self_funded(demand)), "a job close receipt must fit its own quoted grant");
        assert!(!matches!(step, InteractiveJobCloseStep::Blocked | InteractiveJobCloseStep::Refused { .. }), "a locally owned job close has no external owner and no refusal");
    }
    panic!("job did not close");
}

/// 🏁️ Drives `job` through one retained worker session to its terminal outcome. `finish` runs while the completed job is still
/// checked out, with the completed output bytes, and its value becomes the result.
pub fn run_headless<J, R>(job: J, operation: Operation, now_us: fn() -> Option<u64>, site: HeadlessSite, finish: impl FnOnce(&mut J, &[u8]) -> Result<R, String>) -> Result<R, String>
where
    J: InteractiveJob + Send + 'static,
{
    let label = site.label;
    let params = BatchJobParams {
        operation: operation.operation,
        generation: operation.generation,
        cancel: root_cancel_token(),
        config: BatchDriveConfig { retained: HEADLESS_GRANT, site: site.site, stage: site.stage, fuel_per_step: site.fuel_per_step, step_budget_us: site.step_budget_us },
        now_us,
    };
    let mut job_slot = Some(job);
    let mut params_slot = Some(params);
    let mut recipient = RetainedCloneProgress::default();
    let admitted = match WorkerJobAdmissionContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, HEADLESS_GRANT), now_us, &mut recipient) {
        Ok(mut control) => BatchJobSession::try_admit_owned(&mut job_slot, &mut params_slot, &mut control),
        Err(error) => Err(error),
    };
    let mut session = match admitted {
        Ok(Some((session, _admission))) => session,
        Ok(None) | Err(_) => {
            if let Some(mut rejected) = job_slot.take() {
                close_job(&mut rejected);
            }
            return Err(format!("{label}-admission-rejected"));
        }
    };
    let result = drive_session(&mut session, label, finish);
    session.begin_close();
    for _ in 0..2_000_000 {
        if session.terminal_is_empty() {
            return result;
        }
        let demand = match session.retirement_demands(HEADLESS_GRANT.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => panic!("the {label} session quoted a refused close demand: {error:?}"),
        };
        let grant = self_funded(demand);
        match session.close_step(grant) {
            WorkerJobCloseStep::Pending { progress } | WorkerJobCloseStep::Complete { progress } => assert!(progress.fits(grant), "the {label} session close receipt exceeded its quoted grant"),
            WorkerJobCloseStep::Blocked => std::thread::yield_now(),
            WorkerJobCloseStep::Refused { kind, .. } => panic!("the {label} session close was refused: {kind:?}"),
        }
    }
    panic!("the {label} session's close ladder never reached terminal-empty");
}

fn drive_session<J, R>(session: &mut BatchJobSession<J>, label: &'static str, finish: impl FnOnce(&mut J, &[u8]) -> Result<R, String>) -> Result<R, String>
where
    J: InteractiveJob + Send + 'static,
{
    let acknowledgement = HEADLESS_GRANT;
    let mut finish = Some(finish);
    loop {
        match session.step(HEADLESS_GRANT) {
            Ok(WorkerJobPoll::Outcome | WorkerJobPoll::Terminal) => {}
            Ok(_) => continue,
            Err(error) => return Err(format!("{label}-contention:{error:?}")),
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
        for _ in 0..1_000_000 {
            if matches!(session.acknowledge_outcome(acknowledgement), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        if let Some(result) = result {
            return result;
        }
        session.resume().map_err(|error| format!("{label}-resume:{error:?}"))?;
    }
}
