
use super::*;
use std::time::Instant;

pub(super) fn caller_worker_close_policy(phase:WorkerJobClosePhase)->RetainedCloneGrant{
    match phase{
        WorkerJobClosePhase::AuthorityRelease=>RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:65536,maximum_depth:64},
        _=>RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:16384,maximum_depth:64},
    }
}

use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

fn params(operation: OperationId, generation: Generation, cancel: CancelToken) -> BatchJobParams {
    BatchJobParams { operation, generation, cancel, config: BatchDriveConfig { retained:crate::component::TEST_RETAINED_POLICY, site: "test.retained-job", stage: InteractiveStage::InteractiveStep, fuel_per_step: 1, step_budget_us: 1_000 }, now_us: logical_now_us }
}

struct ShortGrantCloseJob {
    backing: Option<Box<u8>>,
    closing: bool,
}

impl InteractiveJob for ShortGrantCloseJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure> {JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture requires original yielded descriptor"))}}
    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        self.begin_close();if self.backing.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=RetirementDemand{release_bytes:self.next_close_release_byte_demand()?,depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{drop(self.backing.take());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.backing.is_some(){size_of::<u8>()}else{0})}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.backing.is_some()))}
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none()
    }
}

/// 🔭️ Logical work remains one item while each named phase declares its physical release extent.
#[test]
fn mounted_close_on_a_short_byte_grant_walks_every_named_phase_to_terminal() {
    let _slots = super::worker_session_slots_shared();
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../🧪️tests/🧫️fixtures/♻️mounted-close/🔣️.json")).unwrap();
    let profile=fixture["profiles"].as_array().unwrap().iter().find(|profile|profile["wordBytes"].as_u64()==Some(size_of::<usize>()as u64)).expect("canonical physical layout profile for this native address space");
    let expected=profile["turns"].as_array().unwrap();
    let grant = 4_096;
    assert!(grant < JOB_PAYLOAD_PAGE_BYTES, "this law is about a grant shorter than one physical page");
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_030),Generation(23),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut mounted = admit_original_fixture_owner!(issuer,MountedWorkerJobSession,ShortGrantCloseJob { backing: Some(Box::new(51)), closing: false }, params(OperationId(90_030), Generation(23), issuer.admit_alias(source)))
        .unwrap_or_else(|_| panic!("short-grant close fixture admission"));
    assert_eq!(mounted.close_phase(), WorkerJobClosePhase::Open);
    mounted.begin_close();
    let mut ladder = Vec::new();
    let mut charged = 0;
    let mut turns = 0;
    let mut observed=Vec::new();
    for _ in 0..64 {
        let phase = mounted.close_phase();
        if ladder.last() != Some(&phase) {
            ladder.push(phase);
        }
        if mounted.terminal_is_empty() {
            break;
        }
        turns += 1;
        let caller=caller_worker_close_policy(phase);
        assert_eq!(caller.maximum_items,fixture["perTurnItems"].as_u64().unwrap()as usize);
        let demand=mounted.retirement_demands(caller.maximum_copy_bytes).expect("exclusive mounted fixture close query");
        assert!(demand.copy_bytes<=caller.maximum_copy_bytes && demand.capacity_bytes<=caller.maximum_capacity_bytes && demand.release_bytes<=caller.maximum_release_bytes && demand.depth<=caller.maximum_depth);
        if phase==WorkerJobClosePhase::PreadmittedFault && demand.release_bytes==JOB_PAYLOAD_PAGE_BYTES{let denied=RetainedCloneGrant{maximum_release_bytes:grant,..caller};let step=issuer.close_owner(denied,&mut mounted).unwrap();assert_eq!(step.progress(),Default::default());assert_eq!(mounted.close_phase(),phase);}
        let release_grant=caller.maximum_release_bytes;
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||issuer.close_owner(caller,&mut mounted).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(heap.requested_bytes,heap.released_bytes));
        let kind=if matches!(step,WorkerJobCloseStep::Complete{..}){"complete"}else{"pending"};
        let actual=serde_json::json!({"phase":format!("{phase:?}"),"next":format!("{:?}",mounted.close_phase()),"kind":kind,"receipt":step.progress()});
        observed.push(actual);
        assert!(step.progress().fits(caller));
        match step {
            WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:released_items,released_bytes,..}} => {
                assert!(released_items <= 1, "a one-item grant releases at most one owner: {released_items}");
                assert!(released_bytes <= release_grant, "a close turn never spends more than its explicit physical release grant: {released_bytes}");
                if matches!(phase,WorkerJobClosePhase::PreadmittedFault|WorkerJobClosePhase::Job){charged += released_bytes;}
            }
            WorkerJobCloseStep::Complete {progress} => {}
            WorkerJobCloseStep::Blocked => panic!("a mounted close on a positive grant is never blocked"),
        }
    }
    assert!(mounted.terminal_is_empty(), "every named phase completes under its exact queried physical release authority");
    assert_eq!(charged, JOB_PAYLOAD_PAGE_BYTES + size_of::<u8>(), "fault page and domain Box each spend their actual allocation extent");
    assert_eq!(observed,expected.clone(),"original exact funded phase/receipt vector; all per-turn and physical conservation checks retained");
    assert_eq!(turns, expected.len(), "canonical original phase and exact physical owner receipts each receive an admitted turn");
    assert_eq!(ladder.iter().map(|phase|format!("{phase:?}")).collect::<Vec<_>>(),fixture["phases"].as_array().unwrap().iter().map(|phase|phase.as_str().unwrap().to_string()).collect::<Vec<_>>(),"the canonical close cursor walks its named physical phases in order and never revisits one");

    issuer.retire(source);
}

/// ⏳️ A liveness bound, not a performance one: the pool worker that owns the step may be
/// descheduled for a long time on a loaded machine (a hostile panic also unwinds and captures a
/// backtrace there), so the wait is bounded by wall time rather than by a count of yields.
const WORKER_LIVENESS_BOUND: std::time::Duration = std::time::Duration::from_secs(30);

fn wait_for(session: &WorkerJobSession<HostileJob>, expected: WorkerJobPoll) {
    let deadline = std::time::Instant::now() + WORKER_LIVENESS_BOUND;
    while std::time::Instant::now() < deadline {
        if session.poll() == expected {
            return;
        }
        std::thread::yield_now();
    }
    panic!("worker session did not reach {expected:?}");
}

struct HostileJob {
    output: Option<JobPayloadSlot>,
    backing: Option<Box<u8>>,
    steps: Option<Arc<AtomicUsize>>,
    panic: bool,
    closing: bool,
}

impl InteractiveJob for HostileJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure> {
        let step = self.steps.as_ref().expect("hostile step counter").fetch_add(1, AtomicOrdering::AcqRel);
        if self.panic {
            panic!("hostile worker panic");
        }
        if step == 0 {
            return JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from);
        }
        let output = cx.payload_from_bytes(JobPayloadStream::CommitOutput, &[**self.backing.as_ref().expect("hostile backing")]).expect("hostile output page");
        self.output=Some(JobPayloadSlot::from_payload(output));
        JobOutcomeBorrow::admit_complete(cx,None,self.output.as_ref().and_then(JobPayloadSlot::original)).map_err(OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Complete=>descriptor.complete(None,self.output.as_ref().and_then(JobPayloadSlot::original)),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"hostile fixture requires original yielded or terminal descriptor"))}}
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
        self.begin_close();if let Some(output)=self.output.as_mut(){if !output.is_empty(){return close_original_retained_step(cx,output.retirement_demands()?,|grant|output.close_step(grant))}}
        if self.backing.is_none()&&self.steps.is_none(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=RetirementDemand{copy_bytes:self.next_close_copy_byte_demand()?,release_bytes:self.next_close_release_byte_demand()?,depth:1,..Default::default()};close_original_retained_step(cx,demand,|_|{
            let progress=if let Some(backing)=self.backing.take(){drop(backing);RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Box<u8>>(),released_bytes:size_of::<u8>(),..Default::default()}}
            else{let steps=self.steps.take().expect("original hostile counter remains retained");let released_bytes=if let Some(counter)=Arc::into_inner(steps){drop(counter);semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{0};RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Arc<AtomicUsize>>(),released_bytes,..Default::default()}};
            Ok(RetainedCloneStep::Progress(progress))
        })
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.copy_bytes);}}Ok(if self.backing.is_some(){size_of::<Box<u8>>()}else if self.steps.is_some(){size_of::<Arc<AtomicUsize>>()}else{0})}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.release_bytes);}}Ok(if self.backing.is_some(){size_of::<u8>()}else if self.steps.is_some(){semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>()}else{0})}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(output)=self.output.as_ref(){if !output.is_empty(){return Ok(output.retirement_demands()?.depth);}}Ok(usize::from(self.backing.is_some()||self.steps.is_some()))}

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.backing.is_none() && self.steps.is_none() && self.output.as_ref().is_none_or(JobPayloadSlot::is_empty)
    }
}

/// 🔁️ [`admitted`] for a mounted session, whose own pump takes a rejected opportunity back and
/// resumes it (`WorkerJobPoll::Rejected`) on the pump after a contended submit.
fn mounted_admitted<J: InteractiveJob + Send + 'static>(mounted: &mut MountedWorkerJobSession<J>, pool: &WorkerPool, lane: Lane) -> Result<WorkerJobPoll, MountedWorkerJobPumpFault> {
    loop {
        match mounted.pump_one(pool, lane,crate::component::TEST_RETAINED_POLICY) {
            Err(MountedWorkerJobPumpFault::Submit(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Contended))) | Ok(WorkerJobPoll::Rejected) => std::thread::yield_now(),
            answer => return answer,
        }
    }
}

#[test]
fn worker_authority_keeps_one_heap_identity_through_mounted_submit_and_checkout() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_011),Generation(17),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut mounted = admit_original_fixture_owner!(issuer,MountedWorkerJobSession,
        HostileJob { output:None, backing: Some(Box::new(73)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false },
        params(OperationId(90_011), Generation(17), issuer.admit_alias(source)),
    )
    .unwrap_or_else(|_| panic!("heap authority fixture admission"));
    assert_eq!(size_of::<OriginalWorkerJobAuthority<HostileJob>>(),size_of::<Vec<()>>());
    let admitted_identity = unsafe { (&*mounted.session.inner.authority.get()).as_ref().expect("idle session owns its authority").native_identity() };
    assert!(matches!(mounted_admitted(&mut mounted, &pool, Lane::Background), Ok(WorkerJobPoll::Submitted)), "a non-interactive lane submits to the pool");
    let deadline = std::time::Instant::now() + WORKER_LIVENESS_BOUND;
    while mounted.poll() != WorkerJobPoll::Outcome && std::time::Instant::now() < deadline {
        std::thread::yield_now();
    }
    assert!(matches!(mounted.pump_one(&pool, Lane::Background,crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Outcome)));
    let checked_out_identity = mounted.checked_out.as_ref().and_then(|outcome| outcome.authority.as_ref()).expect("mounted outcome owns exact authority").native_identity();
    assert_eq!(checked_out_identity, admitted_identity);
    let original_receipt=mounted.take_checked_out_retained_step_receipt().expect("original mounted physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(mounted.checked_out_outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    for _ in 0..2 { assert!(mounted.acknowledge_checked_out_outcome(crate::component::TEST_RETAINED_POLICY).progress().fits(crate::component::TEST_RETAINED_POLICY)); }
    mounted.resume().expect("empty yielded outcome returns the same authority");
    assert!(matches!(mounted.pump_one(&pool, Lane::Interactive,crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Terminal)), "the interactive lane runs the step on the caller and checks its terminal out in one pump");
    let receipt=mounted.take_checked_out_retained_step_receipt().expect("original mounted terminal turn receipt");assert!(receipt.1.fits(receipt.0));assert!(mounted.take_checked_out_retained_step_receipt().is_none());
    let terminal_identity = mounted.checked_out.as_ref().and_then(|outcome| outcome.authority.as_ref()).expect("mounted terminal owns exact authority").native_identity();
    assert_eq!(terminal_identity, admitted_identity, "the caller-run interactive step keeps the same heap authority as the pooled one");
    mounted.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !mounted.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(mounted.close_phase()),&mut mounted).unwrap();
    }
    let _ = pool.shutdown();

    issuer.retire(source);
}

/// 🔁️ `try_submit` never waits for queue ownership: a lane queue the pool's own worker holds for
/// a moment answers `Contended`, the session retains the exact rejected opportunity, and its owner
/// takes it back and resumes before retrying. Tests that need an admitted opportunity follow exactly
/// that protocol for exactly that answer, and consume the wake their own resume raised, so a test's
/// wake assertions see only the transitions it is asserting.
fn admitted<J: InteractiveJob + Send + 'static>(session: &WorkerJobSession<J>, pool: &WorkerPool, lane: Lane) -> Result<WorkerJobTicket, WorkerJobSubmitFault> {
    loop {
        match session.try_submit_step(pool, lane,crate::component::TEST_RETAINED_POLICY) {
            Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Contended)) => {
                session.take_rejected().unwrap_or_else(|_| panic!("a contended opportunity is retained for its owner")).resume();
                let _ = session.take_wake();
                std::thread::yield_now();
            }
            answer => return answer,
        }
    }
}

#[test]
fn worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let operation = OperationId(90_004);
    let generation = Generation(10);
    let steps = Arc::new(AtomicUsize::new(0));
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(operation,generation,WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut session =
        admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(91)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(operation, generation, issuer.admit_alias(source))).unwrap_or_else(|_| panic!("worker session slot"));
    let first = admitted(&session, &pool, Lane::Interactive).expect("first opportunity submitted");
    assert!(matches!(session.try_submit_step(&pool, Lane::Interactive,crate::component::TEST_RETAINED_POLICY), Err(WorkerJobSubmitFault::Contention(WorkerJobContention::Submitted(_)))));
    wait_for(&session, WorkerJobPoll::Outcome);
    let mut first_owner = session.take_outcome(first).expect("first exact outcome");
    let original_receipt=first_owner.take_retained_step_receipt().expect("original pool physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(first_owner.outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    for _ in 0..2 { assert!(first_owner.acknowledge_outcome(crate::component::TEST_RETAINED_POLICY).progress().fits(crate::component::TEST_RETAINED_POLICY)); }
    first_owner.resume().unwrap_or_else(|_| panic!("yield owner resumes exact generation"));
    let second = admitted(&session, &pool, Lane::Interactive).expect("second opportunity submitted");
    wait_for(&session, WorkerJobPoll::Terminal);
    let mut terminal = session.take_terminal().expect("terminal owner is take-only");
    let receipt=terminal.take_retained_step_receipt().expect("original terminal turn receipt");assert!(receipt.1.fits(receipt.0));assert!(terminal.take_retained_step_receipt().is_none());
    let terminal_pointer = terminal.job().backing.as_deref().expect("terminal hostile backing") as *const u8;
    drop(terminal);
    let terminal = session.take_terminal().expect("dropped checkout hands exact terminal back");
    assert_eq!(terminal.job().backing.as_deref().expect("returned hostile backing") as *const u8, terminal_pointer);
    assert_eq!(second.generation, generation);
    terminal.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }
    assert_eq!(steps.load(AtomicOrdering::Acquire), 2);
    pool.shutdown();

    issuer.retire(source);
}

#[test]
fn worker_pool_rejection_returns_exact_job_before_resume() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    pool.shutdown();
    let backing = Box::new(33u8);
    let backing_pointer = backing.as_ref() as *const u8;
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_005),Generation(11),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut session = admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(backing), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_005), Generation(11), issuer.admit_alias(source)))
        .unwrap_or_else(|_| panic!("worker session slot"));
    assert_eq!(session.try_submit_step(&pool, Lane::Interactive,crate::component::TEST_RETAINED_POLICY), Err(WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Shutdown)));
    let rejected = session.take_rejected().expect("pool rejection retained exact owner");
    assert_eq!(rejected.job().backing.as_deref().expect("rejected hostile backing") as *const u8, backing_pointer);
    rejected.resume();
    assert_eq!(session.poll(), WorkerJobPoll::Idle);
    session.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn worker_panic_and_quiet_wake_publish_one_durable_terminal_intent() {
    let _slots = super::worker_session_slots_shared();
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::HeadlessBatch, 1));
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_006),Generation(12),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut session = admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(1)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: true, closing: false }, params(OperationId(90_006), Generation(12), issuer.admit_alias(source)))
        .unwrap_or_else(|_| panic!("worker session slot"));
    let preadmitted_fault_pointer = unsafe {
        (&*session.inner.authority.get())
            .as_ref()
            .and_then(|authority| authority.admission_page_identity())
            .expect("panic fault backing is admitted before submission")
    };
    session.register_wake(Waker::noop()).expect("quiet wake registration");
    let _ = admitted(&session, &pool, Lane::Interactive).expect("panic opportunity submitted");
    let deadline = std::time::Instant::now() + WORKER_LIVENESS_BOUND;
    while !session.take_wake() {
        assert!(std::time::Instant::now() < deadline, "the worker panic never raised its durable wake intent");
        std::thread::yield_now();
    }
    assert_eq!(session.poll(), WorkerJobPoll::Terminal, "a raised wake intent is published after the terminal it announces");
    assert!(!session.take_wake(), "redundant quiet poll raises no wake");
    let mut terminal = session.take_terminal().expect("panic becomes retained terminal");let original_receipt=terminal.take_retained_step_receipt().expect("original panic physical receipt");assert!(original_receipt.1.fits(original_receipt.0));
    let Some(JobOutcomeView::Fault{detail:fault,..}) = terminal.outcome().unwrap() else { panic!("panic publishes the pre-admitted fault") };
    let returned_fault_pointer = fault.page(0).map(|page|page.as_ptr().cast::<MaybeUninit<u8>>()).expect("terminal fault retains exact backing");
    assert_eq!(returned_fault_pointer, preadmitted_fault_pointer);
    terminal.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }
    pool.shutdown();

    issuer.retire(source);
}

#[test]
fn worker_quiet_wake_sequence_exhaustion_is_permanent_and_typed() {
    let _slots = super::worker_session_slots_shared();
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_009),Generation(15),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut session = admit_original_fixture_owner!(issuer,WorkerJobSession,HostileJob { output:None, backing: Some(Box::new(2)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_009), Generation(15), issuer.admit_alias(source)))
        .unwrap_or_else(|_| panic!("worker session slot"));
    session.inner.wake_sequence.store(u64::MAX, Ordering::Release);
    session.inner.wake_pending.store(false, Ordering::Release);
    session.inner.raise_wake();
    assert_eq!(session.register_wake(Waker::noop()), Err(WorkerJobContention::WakeExhausted(Generation(15))));
    session.begin_close();
    let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn batch_session_advances_exactly_one_external_opportunity() {
    let steps = Arc::new(AtomicUsize::new(0));
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_007),Generation(13),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut batch = admit_original_fixture_owner!(issuer,BatchJobSession,HostileJob { output:None, backing: Some(Box::new(7)), steps: Some(Arc::clone(&steps)), panic: false, closing: false }, params(OperationId(90_007), Generation(13), issuer.admit_alias(source)))
        .unwrap_or_else(|_| panic!("batch fault page is pre-admitted"));
    assert_eq!(batch.step(crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Outcome));
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1);
    assert!(batch.checkout_outcome());
    let original_receipt=batch.take_checked_out_retained_step_receipt().expect("original batch physical receipt");assert!(original_receipt.1.fits(original_receipt.0));assert!(matches!(batch.checked_out_outcome().unwrap(), Some(JobOutcomeView::Yield{..})));
    for _ in 0..2 { assert!(batch.acknowledge_outcome(crate::component::TEST_RETAINED_POLICY).progress().fits(crate::component::TEST_RETAINED_POLICY)); }
    batch.resume().expect("caller explicitly resumes after first opportunity");
    assert_eq!(steps.load(AtomicOrdering::Acquire), 1, "batch adapter never drains itself to terminal");
    batch.begin_close();
    assert_eq!(issuer.close_owner(RetainedCloneGrant{maximum_items:0,maximum_release_bytes:0,maximum_depth:64,..RetainedCloneGrant::default()},&mut batch).unwrap(), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !batch.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn checked_out_and_worker_begin_close_transitions_report_exact_zero_release() {
    let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_010),Generation(16),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut batch = admit_original_fixture_owner!(issuer,BatchJobSession,HostileJob { output:None, backing: Some(Box::new(7)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false }, params(OperationId(90_010), Generation(16), issuer.admit_alias(source)))
        .unwrap_or_else(|_| panic!("batch session authority"));
    assert_eq!(batch.step(crate::component::TEST_RETAINED_POLICY), Ok(WorkerJobPoll::Outcome));
    assert!(batch.checkout_outcome());
    assert_eq!(issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap(), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:1,released_bytes:0,..RetainedCloneProgress::default()}});
    let receipt=batch.session.take_retained_step_receipt().unwrap().expect("original checked out receipt follows same-owner close handback");assert!(receipt.1.fits(receipt.0));assert!(batch.session.take_retained_step_receipt().unwrap().is_none());
    let header=issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    assert_eq!(header,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()}});
    assert_eq!(batch.close_phase(),WorkerJobClosePhase::Outcome);
    let presence=issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    assert_eq!(presence,WorkerJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()}});
    assert_eq!(batch.close_phase(),WorkerJobClosePhase::BeginClose);
    assert_eq!(issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap(), WorkerJobCloseStep::Pending {progress:RetainedCloneProgress{copied_items:0,released_bytes:0,..RetainedCloneProgress::default()}});
    let close_started=Instant::now();let mut close_attempts=0;
    while !batch.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
        let _ = issuer.close_owner(caller_worker_close_policy(batch.close_phase()),&mut batch).unwrap();
    }

    issuer.retire(source);
}

#[test]
fn worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned() {
    let _slots = super::worker_session_slots_exclusive();let source=crate::component::TEST_RETAINED_POLICY;let mut rejected_caller=OriginalJobFixtureCaller::admit(OperationId(92_000),Generation(500),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    for _ in 0..WORKER_JOB_SESSION_SLOTS * 64 {
        if !worker_job_retirements_are_parked() {
            break;
        }
        let _ = rejected_caller.turn(caller_worker_close_policy(worker_job_retirement_phase(WORKER_JOB_SESSION_SLOTS).expect("parked original owner")),|cx|pump_worker_job_retirements(WORKER_JOB_SESSION_SLOTS,cx)).unwrap();
    }
    assert!(!worker_job_retirements_are_parked(), "earlier tests' dropped sessions retire before this test owns every slot");
    let mut sessions = Vec::with_capacity(WORKER_JOB_SESSION_SLOTS);
    for index in 0..WORKER_JOB_SESSION_SLOTS {
        let job = HostileJob { output:None, backing: Some(Box::new(index as u8)), steps: Some(Arc::new(AtomicUsize::new(0))), panic: false, closing: false };
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(91_000+index as u64),Generation(index as u64+1),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);let session=admit_original_fixture_owner!(caller,WorkerJobSession,job, params(OperationId(91_000 + index as u64), Generation(index as u64 + 1), caller.admit_alias(source))).unwrap_or_else(|_| panic!("each fixed session slot admits once"));sessions.push((session,caller));
    }
    let rejected_backing = Box::new(211u8);
    let rejected_pointer = rejected_backing.as_ref() as *const u8;
    let mut rejected_job=Some(HostileJob{output:None,backing:Some(rejected_backing),steps:Some(Arc::new(AtomicUsize::new(0))),panic:false,closing:false});
    let mut rejected_params=Some(params(OperationId(92_000),Generation(500),rejected_caller.admit_alias(source)));
    let original_grant=rejected_params.as_ref().unwrap().config.retained;
    let mut recipient=RetainedCloneProgress::default();
    let mut control=WorkerJobAdmissionContext::new(OperationId(92_000),Generation(500),StepBudget::new(1,u64::MAX,original_grant),rejected_params.as_ref().unwrap().now_us,&mut recipient).unwrap();
    assert!(WorkerJobSession::try_admit_owned(&mut rejected_job,&mut rejected_params,&mut control).unwrap().is_none());
    drop(control);
    assert_eq!(recipient,RetainedCloneProgress::default());
    assert_eq!(rejected_job.as_ref().unwrap().backing.as_deref().unwrap()as*const u8,rejected_pointer);
    assert_eq!(rejected_params.as_ref().unwrap().operation,OperationId(92_000));
    assert_eq!(rejected_params.as_ref().unwrap().generation,Generation(500));
    let (dropped,mut dropped_caller) = sessions.pop().expect("last fixed session");
    drop(dropped);
    assert!(take_worker_job_retirement_wake());
    for _ in 0..8 {
        let _ = dropped_caller.turn(caller_worker_close_policy(worker_job_retirement_phase(1).expect("parked original owner")),|cx|pump_worker_job_retirements(1,cx)).unwrap();
    }
    let close_started=Instant::now();let mut close_attempts=0;
    while worker_job_retirements_are_parked(){
        close_attempts+=1;assert!(close_attempts<=65536&&close_started.elapsed()<WORKER_LIVENESS_BOUND,"parked original owner retained beyond finite caller control");
        let phase=worker_job_retirement_phase(1).expect("parked original owner phase");
        let step=dropped_caller.turn(caller_worker_close_policy(phase),|cx|pump_worker_job_retirements(1,cx)).unwrap();assert!(step.progress().fits(caller_worker_close_policy(phase)));
    }
    dropped_caller.retire(source);let mut control=WorkerJobAdmissionContext::new(OperationId(92_000),Generation(500),StepBudget::new(1,u64::MAX,original_grant),rejected_params.as_ref().unwrap().now_us,&mut recipient).unwrap();
    let(replacement,progress)=WorkerJobSession::try_admit_owned(&mut rejected_job,&mut rejected_params,&mut control).unwrap().expect("same original rejected owners enter returned slot");
    drop(control);
    assert_eq!(recipient,progress);
    assert!(rejected_job.is_none()&&rejected_params.is_none());
    sessions.push((replacement,rejected_caller));
    for (mut session,mut caller) in sessions {
        let _ = session.begin_close();
        let close_started=Instant::now();let mut close_attempts=0;
    while !session.terminal_is_empty() {
        close_attempts+=1;assert!(close_attempts<=65536 && close_started.elapsed()<WORKER_LIVENESS_BOUND,"original owner retained beyond declared finite close control");
            let _ = caller.close_owner(caller_worker_close_policy(session.close_phase()),&mut session).unwrap();
        }
        caller.retire(source);
    }
}

#[test]
fn hostile_retained_box_and_arc_require_complete_independent_physical_grants(){
    let source=crate::component::TEST_RETAINED_POLICY;let mut caller=OriginalJobFixtureCaller::admit(OperationId(97842),Generation(1),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);
    let mut job=HostileJob{output:None,backing:Some(Box::new(9)),steps:Some(Arc::new(AtomicUsize::new(0))),panic:false,closing:false};
    let pointer=job.backing.as_deref().unwrap() as *const u8;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:64,maximum_depth:64};
    let denied=RetainedCloneGrant{maximum_release_bytes:0,..grant};
    assert_eq!(caller.turn(denied,|cx|job.close_step(cx)).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(job.backing.as_deref().unwrap() as *const u8,pointer);
    let box_step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert!(box_step.progress().fits(grant));
    assert_eq!(box_step.progress().released_bytes,1);
    let arc_step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert!(arc_step.progress().fits(grant));
    assert_eq!(arc_step.progress().released_bytes,semio_framework_value::shared_retirement_allocation_bytes::<AtomicUsize>());
    assert!(matches!(caller.turn(grant,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Complete{..}));assert!(job.terminal_is_empty());
    let shared=Arc::new(AtomicUsize::new(17));
    let mut job=HostileJob{output:None,backing:None,steps:Some(Arc::clone(&shared)),panic:false,closing:false};
    let step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert_eq!(step.progress().released_bytes,0);assert_eq!(shared.load(AtomicOrdering::Acquire),17);
    assert!(matches!(caller.turn(grant,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Complete{..}));assert!(job.terminal_is_empty());
    caller.retire(source);
}

#[test]
fn mounted_original_cancel_alias_returns_only_to_its_still_borrowed_matching_root(){
 let _slots=super::worker_session_slots_shared();let source=crate::component::TEST_RETAINED_POLICY;let mut issuer=OriginalJobFixtureCaller::admit(OperationId(90_031),Generation(23),WORKER_TEST_STOCK,logical_now_us,u64::MAX,20000);let mut mounted=admit_original_fixture_owner!(issuer,MountedWorkerJobSession,ShortGrantCloseJob{backing:Some(Box::new(51)),closing:false},params(OperationId(90_031),Generation(23),issuer.admit_alias(source))).unwrap_or_else(|_|panic!("original witnessed alias fixture admission"));mounted.begin_close();let mut handbacks=0;
 for _ in 0..64{let phase=mounted.close_phase();if phase==WorkerJobClosePhase::Empty{break;}let grant=caller_worker_close_policy(phase);if let Some(step)=issuer.turn(grant,|cx|{let step=mounted.return_original_cancel_alias_step(cx.original_cancel_token(),cx.retained_grant()).map_err(|error|match error{WorkerJobDemandError::Refused(error)=>OriginalTurnFailure::from(error),WorkerJobDemandError::Contention(_)=>OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original mounted witness remains checked out"))})?;if let Some(step)=step{cx.consume_retained(step.progress())?;cx.consume_fuel(1);}Ok(step)}).unwrap(){let observed=step.progress();assert!(observed.fits(grant));assert_eq!(observed,RetainedCloneProgress{copied_items:1,copied_bytes:0,..Default::default()});assert!(matches!(step,RetainedCloneStep::Complete(_)));handbacks+=1;}else{let step=issuer.turn(grant,|cx|mounted.close_step(cx)).unwrap();assert!(step.progress().fits(grant));assert!(!matches!(step,WorkerJobCloseStep::Blocked));}}
 assert!(mounted.terminal_is_empty());assert_eq!(handbacks,1);let released=issuer.retire(source);assert!(released>0);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
