# Higher TextError Framing Cause Binding

Actual private operation framing, hexadecimal decoding, HTML/JSON source grammar and spreadsheet scalar admission helpers explicitly author InvalidValue. Typed owned JSON errors in those framing chains pass directly to from_value_error instead of being converted to text and reconstructed. Imported already-owned TextError parsing retains its complete original kind, message, expected set and span.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-framing-authored-1.json. Native admission is pending.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs

```rust
use super::*;
use std::sync::Arc as StdArc;

async fn rejected_engine_open_error(mut rejected: ArtifactEngineOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

#[test]
fn artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Parked as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let turns = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let job_handoff = handoff.clone();
    let job_turns = turns.clone();
    *handoff.terminal_job.lock().unwrap() = Some((
        semio_framework_async::WorkerSubmitErrorKind::Saturated,
        Box::new(move || {
            assert_eq!(job_handoff.driver.compare_exchange(ArtifactRunnerDriver::Queued as u8, ArtifactRunnerDriver::Polling as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::Queued as u8));
            job_turns.fetch_add(1, Ordering::AcqRel);
            job_handoff.driver.store(ArtifactRunnerDriver::RunnableIdle as u8, Ordering::Release);
        }),
    ));
    let external = handoff.terminal_job.lock().unwrap().take().map(|owner| ArtifactRunnerTerminalJob { handoff: handoff.clone(), owner: Some(owner) }).unwrap();
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Parked as u8);
    assert!(handoff.driver.compare_exchange(ArtifactRunnerDriver::RunnableIdle as u8, ArtifactRunnerDriver::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_err());
    drop(external);
    assert!(handoff.terminal_job.lock().unwrap().is_some());
    let resumed = handoff.terminal_job.lock().unwrap().take().map(|owner| ArtifactRunnerTerminalJob { handoff: handoff.clone(), owner: Some(owner) }).unwrap();
    let mut retained = Some(resumed);
    for _ in 0..1_024 {
        match retained.take().expect("terminal resume fixture lost its exact cursor").resume() {
            Ok(()) => break,
            Err(cursor) => {
                retained = Some(cursor);
                std::thread::yield_now();
            }
        }
    }
    assert!(retained.is_none(), "terminal resume fixture did not admit its exact cursor");
    let (done_tx, done_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms(), semio_framework_async::Lane::Maintenance, Box::new(move || done_tx.send(()).unwrap()));
    done_rx.recv().unwrap();
    assert_eq!(turns.load(Ordering::Acquire), 1);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::RunnableIdle as u8);
    assert!(handoff.terminal_job.lock().unwrap().is_none());
    drop(handoff);
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[test]
fn artifact_runner_terminal_resume_refusal_returns_exact_cursor_for_close() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    assert_eq!(pool.shutdown(), Ok(()));
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool,
        pool_use: std::sync::Mutex::new(None),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Parked as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let turns = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let job_turns = turns.clone();
    let cursor = ArtifactRunnerTerminalJob {
        handoff: handoff.clone(),
        owner: Some((
            semio_framework_async::WorkerSubmitErrorKind::Saturated,
            Box::new(move || {
                job_turns.fetch_add(1, Ordering::AcqRel);
            }),
        )),
    };
    let close_handoff = handoff.clone();
    *handoff.close_runner.lock().unwrap() = Some(Arc::new(move || {
        assert_eq!(close_handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::ClosingReady as u8);
        close_handoff.terminal.store(true, Ordering::Release);
        true
    }));
    let cursor = cursor.resume().unwrap_err();
    assert_eq!(turns.load(Ordering::Acquire), 0);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Parked as u8);
    assert!(handoff.terminal_job.lock().unwrap().is_none());
    assert!(cursor.close().is_ok());
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Terminal as u8);
}

#[semio_framework_async_macros::async_test]
async fn artifact_authority_drop_transfers_parked_terminal_job_to_registered_close_owner() {
    let (authority, storage, pool) = journal_authority().await;
    let ready_deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while authority.handoff.driver.load(std::sync::atomic::Ordering::Acquire) != ArtifactRunnerDriver::RunnableIdle as u8 && std::time::Instant::now() < ready_deadline {
        semio_framework_async::yield_once().await;
    }
    assert_eq!(authority.handoff.driver.load(std::sync::atomic::Ordering::Acquire), ArtifactRunnerDriver::RunnableIdle as u8, "artifact authority readiness did not complete its internal driver handoff");
    authority.handoff.driver.store(ArtifactRunnerDriver::Parked as u8, std::sync::atomic::Ordering::Release);
    *authority.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) =
        Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, Box::new(|| panic!("parked terminal job must be retired, not executed after authority Drop"))));
    let done = authority.take_terminal_signal().expect("artifact retirement fixture owns its exact terminal acknowledgement");
    let retirement = authority.retirement.as_ref().expect("artifact retirement fixture owns its reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let handoff = authority.handoff.clone();
    drop(authority);
    done.await.expect("registered authority retirement did not produce its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, std::time::Instant::now() + std::time::Duration::from_secs(10)).await;
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
    assert_ne!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
}

#[semio_framework_async_macros::async_test]
async fn artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::ClosingReady as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let reservation = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let index = reservation.index;
    let generation = reservation.generation;
    let attempts = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let close_attempts = attempts.clone();
    let close_handoff = handoff.clone();
    let close: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(move || {
        if close_attempts.fetch_add(1, std::sync::atomic::Ordering::AcqRel) == 0 {
            panic!("injected artifact retirement close panic");
        }
        close_handoff.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        close_handoff.terminal.store(true, std::sync::atomic::Ordering::Release);
        true
    });
    reservation.commit(close, handoff.clone(), pool_use.clone());
    drop(pool_use);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let restored =
            ARTIFACT_RUNNER_RETIREMENTS[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| owner.generation == generation && Arc::ptr_eq(&owner.handoff, &handoff) && Arc::ptr_eq(&owner.pool, &pool));
        if attempts.load(std::sync::atomic::Ordering::Acquire) != 0 && restored {
            break;
        }
        assert!(std::time::Instant::now() < deadline, "injected retirement panic did not restore its exact cursor");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(attempts.load(std::sync::atomic::Ordering::Acquire), 1);
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    handoff.request_retirement_maintenance();
    while ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire) == generation {
        assert!(std::time::Instant::now() < deadline, "explicit retirement retry did not reach terminal acknowledgement");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(attempts.load(std::sync::atomic::Ordering::Acquire), 2);
    assert_eq!(pool.shutdown(), Ok(()));
}

/// ⏰️ A maintenance turn that arrives before its reservation committed the retirement cursor keeps
/// the hook (`Idle`) instead of retiring it: retiring there stranded the cursor, its slot and its pool
/// use for the life of the process once the commit's own request found the hook gone. Every reservation
/// on one pool shares that pool's one hook, which retires with the pool's last slot.
#[test]
fn retirement_turn_before_its_commit_keeps_the_hook_for_the_committed_cursor() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let reservation = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let second = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let (index, generation) = (reservation.index, reservation.generation);
    let hook = reservation.hook.unwrap();
    let shared = second.hook.unwrap();
    assert_eq!((shared.row, shared.generation), (hook.row, hook.generation), "reservations on one pool share its one hook");
    let context = [hook.row as u64, hook.generation];
    assert_eq!(artifact_runner_retirement_step(context), semio_framework_async::WorkerMaintenanceStep::Idle);
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation, "an early turn keeps the reservation's slot");
    drop(reservation);
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), 0);
    assert_eq!(artifact_runner_retirement_step(context), semio_framework_async::WorkerMaintenanceStep::Idle, "the pool's other slot keeps the shared hook");
    drop(second);
    assert_eq!(artifact_runner_retirement_step(context), semio_framework_async::WorkerMaintenanceStep::Retire, "the hook of a pool without slots retires");
    assert_eq!(pool.shutdown(), Ok(()));
}

async fn await_retirement_slot_release(handoff: &ArtifactRunnerHandoff, index: usize, generation: u64, deadline: std::time::Instant) {
    while ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire) == generation {
        if std::time::Instant::now() < deadline {
            semio_framework_async::yield_once().await;
            continue;
        }
        let close_error = handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let close_retry = handoff.close_retry_progress();
        panic!(
            "artifact retirement did not release its slot: driver={} terminal={} close_error={:?} retry={:?} turns={} polls={} maintenance={}",
            handoff.driver.load(std::sync::atomic::Ordering::Acquire),
            handoff.terminal.load(std::sync::atomic::Ordering::Acquire),
            close_error,
            close_retry,
            handoff.retirement_turns.load(std::sync::atomic::Ordering::Acquire),
            handoff.close_polls.load(std::sync::atomic::Ordering::Acquire),
            handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
        );
    }
}

#[semio_framework_async_macros::async_test]
async fn a_runner_that_turns_terminal_after_its_retirement_went_idle_wakes_the_retirement() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(None),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Polling as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let reservation = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let (index, generation) = (reservation.index, reservation.generation);
    let observed = Arc::downgrade(&handoff);
    reservation.commit(Arc::new(move || observed.upgrade().is_some_and(|handoff| handoff.terminal.load(Ordering::Acquire))), handoff.clone(), pool_use.clone());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while handoff.retirement_turns.load(Ordering::Acquire) == 0 || ARTIFACT_RUNNER_RETIREMENTS[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
        assert!(std::time::Instant::now() < deadline, "the committed retirement never took its first turn");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(Ordering::Acquire), generation, "a runner still polling keeps its retirement cursor");
    handoff.enter_terminal();
    await_retirement_slot_release(&handoff, index, generation, deadline).await;
    assert_eq!(Arc::strong_count(&pool_use), 1, "the retired cursor released its WorkerPoolUse");
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal() {
    let (authority, storage, pool) = journal_authority().await;
    let handoff = authority.handoff.clone();
    let retirement = authority.retirement.as_ref().expect("artifact engine close-fault fixture owns one retirement reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let done = authority.take_terminal_signal().expect("artifact engine close-fault fixture owns its terminal acknowledgement");
    handoff.close_faults.store(3, std::sync::atomic::Ordering::Release);
    drop(authority);
    done.await.expect("the pool timer did not drive the faulted close to its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, std::time::Instant::now() + std::time::Duration::from_secs(10)).await;
    assert!(handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
    let retry = handoff.close_retry.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_eq!((retry.state, retry.attempts), (ArtifactCloseRetryState::Clear, 0), "a healthy close step clears the retry owner");
    drop(retry);
    assert_eq!(handoff.close_faults.load(std::sync::atomic::Ordering::Acquire), 0);
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission() {
    let (authority, storage, pool) = journal_authority().await;
    let handoff = authority.handoff.clone();
    let retirement = authority.retirement.as_ref().expect("artifact engine close-fault fixture owns one retirement reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let done = authority.take_terminal_signal().expect("artifact engine close-fault fixture owns its terminal acknowledgement");
    handoff.close_faults.store(usize::MAX, std::sync::atomic::Ordering::Release);
    drop(authority);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while artifact_close_retry_progress(&pool).is_none_or(|progress| !progress.is_blocked()) {
        assert!(std::time::Instant::now() < deadline, "faulted close never exhausted its bounded retry budget: {:?}", handoff.close_retry_progress());
        semio_framework_async::yield_once().await;
    }
    let exhausted = artifact_close_retry_progress(&pool).unwrap();
    assert_eq!(exhausted, ArtifactCloseRetryProgress { state: ArtifactCloseRetryState::Exhausted, attempts: ARTIFACT_CLOSE_RETRY_LIMIT, limit: ARTIFACT_CLOSE_RETRY_LIMIT });
    let polls = handoff.close_polls.load(std::sync::atomic::Ordering::Acquire);
    assert_eq!(polls, 1 + usize::from(ARTIFACT_CLOSE_RETRY_LIMIT), "one initial poll plus exactly one poll per armed retry");
    handoff.request_retirement_maintenance();
    let (barrier_tx, barrier_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms() + ARTIFACT_CLOSE_RETRY_CAP_MS, semio_framework_async::Lane::UserVisible, Box::new(move || barrier_tx.send(()).unwrap()));
    barrier_rx.recv().unwrap();
    assert_eq!(handoff.close_polls.load(std::sync::atomic::Ordering::Acquire), polls, "an exhausted faulted close was re-polled without re-admission");
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    handoff.close_faults.store(0, std::sync::atomic::Ordering::Release);
    assert_eq!(artifact_close_retry_readmit(&pool), 1);
    done.await.expect("re-admitted close did not reach its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, deadline).await;
    assert!(artifact_close_retry_progress(&pool).is_none());
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_close_fault_cancel_stops_the_timer_until_readmission() {
    let (authority, storage, pool) = journal_authority().await;
    let handoff = authority.handoff.clone();
    let retirement = authority.retirement.as_ref().expect("artifact engine close-fault fixture owns one retirement reservation");
    let (index, generation) = (retirement.index, retirement.generation);
    let done = authority.take_terminal_signal().expect("artifact engine close-fault fixture owns its terminal acknowledgement");
    handoff.close_faults.store(usize::MAX, std::sync::atomic::Ordering::Release);
    drop(authority);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while artifact_close_retry_cancel(&pool) == 0 {
        assert!(std::time::Instant::now() < deadline, "faulted close never scheduled its first retry");
        semio_framework_async::yield_once().await;
    }
    let cancelled = handoff.close_retry_progress().expect("cancelled close keeps its fault");
    assert_eq!(cancelled.state, ArtifactCloseRetryState::Cancelled);
    let polls = handoff.close_polls.load(std::sync::atomic::Ordering::Acquire);
    let (barrier_tx, barrier_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms() + ARTIFACT_CLOSE_RETRY_CAP_MS, semio_framework_async::Lane::UserVisible, Box::new(move || barrier_tx.send(()).unwrap()));
    barrier_rx.recv().unwrap();
    assert_eq!(handoff.close_polls.load(std::sync::atomic::Ordering::Acquire), polls, "a cancelled retry timer re-polled the faulted close");
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(std::sync::atomic::Ordering::Acquire), generation);
    handoff.close_faults.store(0, std::sync::atomic::Ordering::Release);
    assert_eq!(artifact_close_retry_readmit(&pool), 1);
    done.await.expect("re-admitted close did not reach its terminal acknowledgement");
    await_retirement_slot_release(&handoff, index, generation, deadline).await;
    assert_eq!(pool.shutdown(), Ok(()));
    drop(storage);
}

/// 🪝️ Live authorities are not maintenance hooks: one pool holds more open authorities at once than it has maintenance
/// hooks in all (spread over two fixed-capacity memory backends), each dropped authority still retires within its turn
/// budget and releases its WAL writer, and afterwards the pool's whole hook capacity is free again — the shared retirement
/// hook retired with the last slot.
#[semio_framework_async_macros::async_test]
async fn more_live_authorities_than_pool_maintenance_hooks_retire_through_one_shared_hook() {
    const ARTIFACT_RETIREMENT_LIVENESS_WATCHDOG: std::time::Duration = std::time::Duration::from_secs(60);
    const ARTIFACT_RETIREMENT_EVENT_TURN_BUDGET: usize = 96;
    fn idle(_: [u64; 2]) -> semio_framework_async::WorkerMaintenanceStep {
        semio_framework_async::WorkerMaintenanceStep::Idle
    }
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let storages = [storage().await, storage().await];
    let mut authorities = Vec::new();
    for ordinal in 0..=semio_framework_async::WORKER_MAINTENANCE_CAPACITY {
        let storage = storages[ordinal % storages.len()].clone();
        let engine_storage = storage.clone();
        let document = protocol::ArtifactId(format!("retirement-shared-{ordinal}"));
        let engine_document = document.clone();
        let authority = ArtifactAuthority::spawn(
            pool.clone(),
            move || async move { ArtifactEngine::create_retained(engine_document, engine_storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) },
            MailboxCapacities::uniform(4),
        )
        .await
        .unwrap_or_else(|rejected| panic!("authority {ordinal} spawns beside the others: {:?}", rejected.error()));
        authorities.push((storage, to_core_document_id(&document).await, authority));
    }
    for (storage, core, authority) in authorities {
        let handoff = authority.handoff.clone();
        let retirement = authority.retirement.as_ref().expect("spawned authority owns its retirement reservation");
        let (index, generation) = (retirement.index, retirement.generation);
        drop(authority);
        let deadline = std::time::Instant::now() + ARTIFACT_RETIREMENT_LIVENESS_WATCHDOG;
        await_retirement_slot_release(&handoff, index, generation, deadline).await;
        let turns = handoff.retirement_turns.load(std::sync::atomic::Ordering::Acquire);
        assert!(turns <= ARTIFACT_RETIREMENT_EVENT_TURN_BUDGET, "dropped authority retirement took {turns} hook turns: the hook re-requested itself instead of waking on progress");
        let wal = storage.wal().await;
        let writer = loop {
            match wal.acquire_writer(&core).await {
                Ok(writer) => break writer,
                Err(DbError::Conflict(_)) => {
                    assert!(std::time::Instant::now() < deadline, "artifact authority retirement did not release its WAL writer");
                    semio_framework_async::yield_once().await;
                }
                Err(error) => panic!("artifact authority retirement writer probe failed: {error}"),
            }
        };
        writer.release().await.unwrap();
    }
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let mut probes = Vec::with_capacity(semio_framework_async::WORKER_MAINTENANCE_CAPACITY);
        let mut complete = true;
        for _ in 0..semio_framework_async::WORKER_MAINTENANCE_CAPACITY {
            match pool.install_maintenance_hook(semio_framework_async::Lane::Io, idle, [0; 2]) {
                Ok(ticket) => probes.push(ticket),
                Err(semio_framework_async::WorkerMaintenanceError::Capacity) => {
                    complete = false;
                    break;
                }
                Err(error) => panic!("artifact retirement capacity probe failed: {error:?}"),
            }
        }
        for ticket in probes {
            assert!(pool.remove_maintenance_hook(ticket).unwrap());
        }
        if complete {
            break;
        }
        assert!(std::time::Instant::now() < deadline, "artifact authority retirement leaked a maintenance callback slot");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(pool.shutdown(), Ok(()));
}

#[test]
fn artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(true),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Parked as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let polls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let close_handoff = handoff.clone();
    let close_polls = polls.clone();
    *handoff.close_runner.lock().unwrap() = Some(Arc::new(move || {
        assert_eq!(close_handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingReady as u8));
        if close_polls.fetch_add(1, Ordering::AcqRel) == 0 {
            close_handoff.driver.store(ArtifactRunnerDriver::ClosingParked as u8, Ordering::Release);
            return false;
        }
        close_handoff.active_history.store(false, Ordering::Release);
        close_handoff.pool_use.lock().unwrap().take();
        close_handoff.terminal.store(true, Ordering::Release);
        true
    }));
    *handoff.terminal_job.lock().unwrap() = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, Box::new(|| {})));
    let cursor = handoff.terminal_job.lock().unwrap().take().map(|owner| ArtifactRunnerTerminalJob { handoff: handoff.clone(), owner: Some(owner) }).unwrap();
    let cursor = cursor.close().unwrap_err();
    assert_eq!(polls.load(Ordering::Acquire), 1);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::ClosingParked as u8);
    assert!(handoff.terminal_job.lock().unwrap().is_none());
    let cursor = cursor.close().unwrap_err();
    assert_eq!(polls.load(Ordering::Acquire), 1, "close cursor must not repoll before a retained wake");
    assert_eq!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingParked as u8, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingParked as u8));
    assert!(cursor.close().is_ok());
    assert_eq!(polls.load(Ordering::Acquire), 2);
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::Terminal as u8);
    drop(handoff);
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[test]
fn artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(true),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::ClosingPolling as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    drop(ArtifactRunnerClosePoll { handoff: handoff.clone() });
    assert_eq!(handoff.driver.load(Ordering::Acquire), ArtifactRunnerDriver::ClosingParked as u8);
    assert!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire).is_err());
    assert_eq!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingParked as u8, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingParked as u8));
    assert_eq!(handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire,), Ok(ArtifactRunnerDriver::ClosingReady as u8));
    handoff.driver.store(ArtifactRunnerDriver::Terminal as u8, Ordering::Release);
    handoff.pool_use.lock().unwrap().take();
    drop(handoff);
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn artifact_open_ignores_neutral_aborted_command_snapshot_and_cas() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "aborted-commands-snapshot-cas-have-no-effects").unwrap();
    let document = ArtifactId::from("committed-artifact");
    let memory = db_wal::tests::committed_fixture_storage(row, &document).await;
    let storage = StdArc::new(db_storage::DbBackend::Memory(memory));
    let (mut engine, report) = ArtifactEngine::open_retained(protocol::ArtifactId(document.0.clone()), storage, ArtifactEngineConfig::default(), 1).await.unwrap();
    assert_eq!(report.commands_replayed, 0);
    assert!(!report.from_snapshot);
    assert_eq!(engine.frontier.head_seq, 0);
    assert!(engine.state.values.is_empty());
    assert!(engine.applied.is_empty());
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_create_rejection_propagates_exact_wal_release_owner() {
    let inner = StdArc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()));
    let fault = crate::db_fault_testing::FaultStorage::new(inner).await;
    fault.set_script(crate::db_fault_testing::FaultScript { fail_nth_write: Some(1), ..crate::db_fault_testing::FaultScript::default() }).await;
    let storage = StdArc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let document = document_id().await;
    let core_document = to_core_document_id(&document).await;
    let rejected = match ArtifactEngine::create_retained(document, storage.clone(), ArtifactEngineConfig::default(), 0).await {
        Err(rejected) => rejected,
        Ok(mut engine) => {
            engine.wal.close().await.unwrap();
            panic!("faulted engine create was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(rejected.has_retained_writer());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))));
    assert!(matches!(rejected_engine_open_error(rejected).await, DbError::Io(_)));
    storage.wal().await.acquire_writer(&core_document).await.unwrap().release().await.unwrap();
}

fn history_construction_test_lock() -> std::sync::MutexGuard<'static, ()> {
    history_capacity_test_lock()
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for (name, compacted, hole) in [
        ("high-water-across-segments", false, false),
        ("high-water-across-segments", true, false),
        ("high-water-across-segments", false, true),
        ("aborted-commands-snapshot-cas-have-no-effects", false, false),
        ("commit-outside-transaction", false, false),
        ("active-incomplete-needs-durable-abort", false, false),
    ] {
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == name).unwrap();
        let document = ArtifactId::from("committed-history");
        let memory = db_wal::tests::committed_fixture_storage(row, &document).await;
        if compacted || hole {
            let writer = db_storage::WalStorage::acquire_writer(&memory, &document).await.unwrap();
            if compacted {
                db_storage::WalStorage::delete_segment(&memory, &writer, 0).await.unwrap();
            }
            if hole {
                db_storage::WalStorage::create_segment(&memory, &writer, 3).await.unwrap();
            }
            writer.release().await.unwrap();
        }
        let storage = StdArc::new(db_storage::DbBackend::Memory(memory));
        let mut replay = HistoryReplayFuture::new(storage, document, 1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        let result = (&mut replay).await;
        assert!(replay.terminal_is_empty(), "{name}");
        if !hole && row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null() {
            let mut view = result.unwrap();
            assert!(view.entries.is_empty(), "{name}");
            assert!(view.operation_ids.is_empty(), "{name}");
            assert_eq!(view.result_len, 0, "{name}");
            while view.close_step() {}
        } else {
            assert!(matches!(result, Err(DbError::Corrupt(_))), "{name}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let projection = &fixture["historyProjection"];
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let mut commands = Vec::new();
    for row in projection["commands"].as_array().unwrap() {
        commands.push(envelope(row["id"].as_str().unwrap(), &[], "history-author", &[(row["path"].as_str().unwrap(), row["value"].clone())]).await);
    }
    let receipt = engine.submit(CommandBatch::new(commands).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
    let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
    let mut view = (&mut replay).await.unwrap();
    assert!(replay.terminal_is_empty());
    assert_eq!(view.entries.len() as u64, projection["expected"]["entries"].as_u64().unwrap());
    let entry = view.entries[0];
    assert_eq!(entry.operation_count, 2);
    for (index, id) in projection["expected"]["operationIds"].as_array().unwrap().iter().enumerate() {
        assert!(view.operation_id_eq(0, index, id.as_str().unwrap()));
    }
    assert_eq!(entry.head_seq, projection["expected"]["headSeq"].as_u64().unwrap());
    assert_eq!(entry.commit_seq, projection["expected"]["commitSeq"].as_u64().unwrap());
    assert_eq!((entry.head_seq, entry.commit_seq, entry.chain_hash, entry.epoch), (receipt.frontier.head_seq, receipt.frontier.commit_seq, receipt.frontier.chain_hash, receipt.frontier.epoch));
    while view.close_step() {}
    assert!(view.terminal_is_empty());
    for checkpoint in ["verify", "committed", "copied", "published"] {
        let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(false));
        let mut replay = engine.history_replay(1, cancelled.clone(), HistoryReplayReservation::try_new().unwrap());
        let mut turns = 0;
        let reached = std::future::poll_fn(|context| {
            let target = match checkpoint {
                "verify" => matches!(replay.phase.as_ref(), Some(HistoryReplayPhase::Verify { .. })),
                "committed" => matches!(replay.phase.as_ref(), Some(HistoryReplayPhase::CommittedBody { .. })),
                "copied" => replay.result_len > 0 && replay.reservation.as_ref().is_some_and(|owner| owner.operation_ids.len() == 1),
                "published" => replay.reservation.as_ref().is_some_and(|owner| owner.entries.len() == 1),
                _ => unreachable!(),
            };
            if target {
                return std::task::Poll::Ready(true);
            }
            turns += 1;
            assert!(turns < 100_000, "history cancellation checkpoint stopped progressing");
            match Pin::new(&mut replay).poll(context) {
                std::task::Poll::Pending => std::task::Poll::Pending,
                std::task::Poll::Ready(result) => {
                    if let Ok(mut view) = result {
                        while view.close_step() {}
                    }
                    std::task::Poll::Ready(false)
                }
            }
        })
        .await;
        assert!(reached);
        assert!(replay.authenticated.is_some());
        assert_eq!(replay.reservation.as_ref().unwrap().entries.len(), usize::from(checkpoint == "published"));
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert!(matches!((&mut replay).await, Err(DbError::Closed)));
        assert!(replay.terminal_is_empty());
    }
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../📝️wal/🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["historyProjection"]["rejections"].as_array().unwrap() {
        let backing = storage().await;
        let mut engine = ArtifactEngine::create_retained(document_id().await, backing.clone(), ArtifactEngineConfig::default(), 0).await.unwrap();
        let mut records = db_wal::WalRecordBatch::new();
        for record in row["records"].as_array().unwrap() {
            let document = if record["document"] == "foreign" { "foreign-history-document".to_string() } else { engine.document.0.clone() };
            let record = match record["kind"].as_str().unwrap() {
                "command" => {
                    let mut command = envelope("hostile-history", &[], "history-author", &[("entry", serde_json::json!("value"))]).await;
                    command.document_id = protocol::ArtifactId(document);
                    db_wal::WalRecord::Command(retained_wal_envelope(&command).await)
                }
                "frontier" => db_wal::WalRecord::Frontier(Frontier { document: ArtifactId(document), head_seq: 1, commit_seq: 1, chain_hash: [7; 32], epoch: 3 }),
                _ => unreachable!(),
            };
            records.push(record).unwrap_or_else(|mut record| {
                while record.close_step().unwrap() {}
                panic!("hostile history fixture exceeded record capacity");
            });
        }
        let facet = backing.wal().await;
        engine.wal.submit(&facet, &[], &records, DurabilityClass::Fsync, 1).await.unwrap();
        while records.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        drop(records);
        drop(facet);
        let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        let rejected = match (&mut replay).await {
            Err(DbError::Corrupt(_)) => true,
            Ok(mut view) => {
                while view.close_step() {}
                false
            }
            Err(_) => false,
        };
        assert!(replay.terminal_is_empty());
        engine.wal.close().await.unwrap();
        while engine.state.values.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        drop(engine);
        assert!(rejected, "history admitted {}", row["name"]);
        let rejected = match ArtifactEngine::open_retained(document_id().await, backing, ArtifactEngineConfig::default(), 2).await {
            Err(rejected) => matches!(rejected_engine_open_error(rejected).await, DbError::Corrupt(_)),
            Ok((mut engine, _)) => {
                engine.wal.close().await.unwrap();
                while engine.state.values.close_step().unwrap() {
                    semio_framework_async::yield_once().await;
                }
                false
            }
        };
        assert!(rejected, "opener admitted {}", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_staging_retirement_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_are_lossless") {
        return;
    }
    while artifact_state_retirement_maintenance_step().unwrap() {}
    let accepted_cancel = StdArc::new(std::sync::atomic::AtomicBool::new(false));
    let mut accepted_control = db_state::StateCursorControl::new(accepted_cancel, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let mut accepted = db_state::StateEntry::try_admit("accepted", vec![0x41], 1, &mut accepted_control).await.unwrap();
    assert!(accepted.close_step().unwrap());
    assert!(accepted.close_step().unwrap());
    assert!(accepted.close_step().unwrap());
    assert!(!accepted.close_step().unwrap());
    assert!(accepted.terminal_is_empty());

    let cancelled = StdArc::new(std::sync::atomic::AtomicBool::new(true));
    let mut control = db_state::StateCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let rejected = db_state::StateEntry::try_admit("path", vec![0x42], 1, &mut control).await.unwrap_err();
    assert!(matches!(rejected.error(), DbError::Unavailable(message) if message == "state cursor cancelled"));
    let source = rejected.source().expect("exact refused source").as_ptr();
    let mut second_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(true)), std::time::Instant::now() + std::time::Duration::from_secs(30), 8).unwrap();
    let second_rejected = db_state::StateEntry::try_admit("second-path", vec![0x44], 1, &mut second_control).await.unwrap_err();
    let second_source = second_rejected.source().expect("second exact refused source").as_ptr();
    let initial_retirement = reserve_artifact_state_retirement().expect("artifact retirement preflight");
    let mut cursor = ArtifactStateRetirementCursor::rejected(initial_retirement, rejected, std::array::from_fn(|_| None));
    assert!(cursor.close_step().unwrap());
    assert_eq!(cursor.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr), Some(source));
    release_artifact_state_retirement(&mut cursor.retirement);
    {
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    {
        let mut overflow = ARTIFACT_STATE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in overflow.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
    cursor.retirement = Some(reserve_artifact_state_retirement().expect("artifact quarantine preflight"));
    let second_retirement = reserve_artifact_state_retirement().expect("second artifact quarantine preflight");
    assert!(retire_artifact_state_owner(cursor).is_ok());
    assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(second_retirement, second_rejected, std::array::from_fn(|_| None))).is_ok());
    assert!(ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
    {
        let quarantine = ARTIFACT_STATE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(quarantine.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
        assert!(quarantine.iter().flatten().any(|owner| owner.rejected.as_ref().and_then(db_state::StateEntryRejected::source).map(Vec::as_ptr) == Some(second_source)));
    }
    {
        let mut retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in retired.iter_mut() {
            *slot = None;
        }
    }
    for _ in 0..ARTIFACT_STATE_RETIREMENT_SLOTS * 2 {
        assert!(artifact_state_retirement_maintenance_step().unwrap());
    }
    assert!(artifact_state_retirement_maintenance_step().unwrap());
    {
        let retired = ARTIFACT_STATE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(retired.iter().flatten().find_map(|owner| owner.rejected.as_ref()?.source().map(Vec::as_ptr)), Some(source));
    }
    while artifact_state_retirement_maintenance_step().unwrap() {}

    let mut deadline_control = db_state::StateCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now(), 1).unwrap();
    let deadline = db_state::StateEntry::try_admit("deadline", vec![0x43], 1, &mut deadline_control).await.unwrap_err();
    assert!(matches!(deadline.error(), DbError::Unavailable(message) if message == "state cursor deadline reached"));
    let deadline_retirement = reserve_artifact_state_retirement().expect("deadline artifact retirement preflight");
    assert!(retire_artifact_state_owner(ArtifactStateRetirementCursor::rejected(deadline_retirement, deadline, std::array::from_fn(|_| None))).is_ok());
    while artifact_state_retirement_maintenance_step().unwrap() {}

    for tier in [&ARTIFACT_STATE_RETIREMENT, &ARTIFACT_STATE_RETIREMENT_OVERFLOW, &ARTIFACT_STATE_RETIREMENT_QUARANTINE] {
        let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for slot in owners.iter_mut() {
            *slot = Some(ArtifactStateRetirementCursor::empty());
        }
    }
    let mut state = DocumentState::new();
    let entries = vec![("parked-owners-retire-first".to_string(), None)];
    state.apply_entries(&protocol::MutationId("parked-owners-retire-first".to_string()), entries).await.expect("an apply first retires every parked owner, so parked owners alone never refuse it");
    for reservations in &ARTIFACT_STATE_RETIREMENT_RESERVATIONS {
        reservations.store(u64::MAX, std::sync::atomic::Ordering::Release);
    }
    let entries = vec![("exact-all-tier-state-refusal".to_string(), None)];
    let refusal = match state.apply_entries(&protocol::MutationId("exact-all-tier-state-refusal".to_string()), entries).await {
        Err(error) => error,
        Ok(_) => panic!("all-tier artifact retirement saturation admitted a state mutation"),
    };
    assert!(matches!(refusal, DbError::Unavailable(message) if message == "artifact state retirement pressure refused admission"));
    for reservations in &ARTIFACT_STATE_RETIREMENT_RESERVATIONS {
        reservations.store(0, std::sync::atomic::Ordering::Release);
    }
}

struct HistoryReplayTestWake;

impl std::task::Wake for HistoryReplayTestWake {
    fn wake(self: StdArc<Self>) {}
}

async fn storage() -> StdArc<db_storage::DbBackend> {
    StdArc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()))
}

async fn document_id() -> protocol::ArtifactId {
    protocol::ArtifactId("doc-1".to_string())
}

async fn stored_json(mut bytes: db_query::QueryBytes) -> serde_json::Value {
    let mut raw = Vec::with_capacity(bytes.len());
    for fragment in bytes.fragments() {
        raw.extend_from_slice(fragment);
    }
    while bytes.close_step().unwrap().is_some() {}
    decode_pathmap_json(&raw).await.expect("stored json value")
}

async fn envelope(id: &str, deps: &[&str], actor: &str, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let object: Vec<(String, DslValue)> = entries.iter().map(|(path, value)| (path.to_string(), DslValue::from(value))).collect();
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(object)).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::Object(vec![])).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}

async fn retained_wal_envelope(envelope: &protocol::MutationEnvelope) -> db_wal::WalBytes {
    let mut encoded = Vec::new();
    protocol::encode_envelope(envelope, &mut encoded);
    encoded.shrink_to_fit();
    let mut control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    db_wal::WalBytes::try_admit(encoded, (db_storage::DB_IO_OPERATION_PAGES * db_storage::DB_IO_PAGE_BYTES) as u64, &mut control).await.unwrap()
}

//#region 🔖️Command
#[semio_framework_async_macros::async_test]
async fn command_batch_rejects_empty_and_mixed_documents() {
    assert!(CommandBatch::new(Vec::new()).await.is_err());
    let mut mismatched = envelope("op-2", &[], "alice", &[("x", serde_json::json!(1))]).await;
    mismatched.document_id = protocol::ArtifactId("other-doc".to_string());
    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await, mismatched]).await;
    assert!(batch.is_err());
}
//#endregion 🔖️Command

//#region 🔖️Bridge
mod bridge {
    use super::*;

    #[derive(Clone, serde::Serialize, serde::Deserialize, store::ToValue, store::FromValue)]
    struct Counter {
        value: i64,
    }

    #[derive(Clone, Default, serde::Serialize, serde::Deserialize, store::ToValue, store::FromValue)]
    struct AddDiff {
        amount: i64,
    }

    impl protocol::MutationDiff<Counter> for AddDiff {
        fn apply(&self, base: &Counter) -> protocol::MutationApplyResult<Counter> {
            Ok(Counter { value: base.value + self.amount })
        }
        fn absorb(&mut self, other: Self) {
            self.amount += other.amount;
        }
    }

    #[derive(Clone, serde::Serialize, serde::Deserialize, store::ToValue, store::FromValue)]
    struct Add {
        amount: i64,
    }

    const ADD_DESCRIPTOR: protocol::MutationLeafDescriptor = protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "framework/os/db/artifact/test-add",
        semantic_kind: "add",
        display_name: "Add",
        emoji: "➕️",
        aggregate_variant: "Add",
        payload_schema: "db.artifact.test-add/v1",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    };

    impl protocol::Mutation<Counter> for Add {
        type Diff = AddDiff;
        const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[ADD_DESCRIPTOR];
        fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
            &ADD_DESCRIPTOR
        }
        fn diff(&self, _base: &Counter) -> protocol::MutationOutcome<AddDiff> {
            protocol::MutationOutcome::new(AddDiff { amount: self.amount })
        }
        fn inverse(&self, _base: &Counter) -> Vec<Self> {
            vec![Add { amount: -self.amount }]
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn envelope_from_operation_uses_operation_and_diff_traits() {
        let base = Counter { value: 10 };
        let op = Add { amount: 5 };
        let envelope = envelope_from_operation(document_id().await, "counter", &op, &base, protocol::ActorId("alice".to_string()), protocol::MutationId("op-add-1".to_string()), protocol::HybridLogicalTimestamp::new(1, 0)).await.unwrap();
        let entries = diff_entries(&envelope.diff).await.unwrap();
        assert_eq!(entries.len(), 1);
        let (path, value) = &entries[0];
        assert_eq!(path, "counter");
        let new_value: Counter = semio_framework_value::FromValue::from_value(value.clone().unwrap()).unwrap();
        assert_eq!(new_value.value, 15);
    }
}
//#endregion 🔖️Bridge

//#region 🔖️Engine submit + materialize + WAL replay
#[semio_framework_async_macros::async_test]
async fn submit_persists_to_wal_and_updates_materialized_state_and_frontier() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
    let receipt = engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();

    assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.frontier.head_seq, 1);
    assert_eq!(receipt.frontier.commit_seq, 1);
    assert!(receipt.conflicts.is_empty());
    assert!(receipt.state_hash.is_some());

    let stored = engine.get("name").await.unwrap().unwrap();
    let value: serde_json::Value = stored_json(stored).await;
    assert_eq!(value, serde_json::json!("hello"));
    assert_eq!(engine.frontier().await.head_seq, 1);
    assert_eq!(engine.checkpoint_publication_snapshot().await.head_edit_id, Some(protocol::MutationId("op-1".to_string())));
}

#[semio_framework_async_macros::async_test]
async fn open_replays_the_wal_and_reconstructs_state_and_frontier_identically() {
    let storage = storage().await;
    let (before_count, before_frontier) = {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        let batch1 = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
        engine.submit(batch1, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
        let batch2 = CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("count", serde_json::json!(2))]).await]).await.unwrap();
        engine.submit(batch2, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 2).await.unwrap();
        let count = stored_json(engine.get("count").await.unwrap().unwrap()).await;
        assert!(store::pack_rt::json_values_equal(&count, &serde_json::json!(2)));
        let frontier = engine.frontier().await;
        engine.wal.close().await.unwrap();
        while engine.state.values.close_step().unwrap() {
            semio_framework_async::yield_once().await;
        }
        (count, frontier)
    };

    let (mut reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 3).unwrap();
    assert_eq!(report.torn_tail_bytes, 0);
    assert_eq!(reopened.frontier().await.head_seq, 2);
    assert_eq!(reopened.frontier().await.commit_seq, 2);
    assert_eq!(reopened.frontier().await, before_frontier);
    assert_eq!(reopened.checkpoint_publication_snapshot().await.head_edit_id, Some(protocol::MutationId("op-2".to_string())));

    let name: serde_json::Value = stored_json(reopened.get("name").await.unwrap().unwrap()).await;
    assert_eq!(name, serde_json::json!("hello"));
    let count: serde_json::Value = stored_json(reopened.get("count").await.unwrap().unwrap()).await;
    assert_eq!(count, before_count);
    reopened.wal.close().await.unwrap();
    while reopened.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn ledger_tail_returns_exact_committed_batches_between_two_published_points() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    let options = || SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() };
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("a", serde_json::json!(1))]).await]).await.unwrap(), options(), 1).await.unwrap();
    let first = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    engine
        .submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "bob", &[("b", serde_json::json!(2))]).await, envelope("op-3", &["op-2"], "bob", &[("c", serde_json::json!(3))]).await]).await.unwrap(), options(), 2)
        .await
        .unwrap();
    let second = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    engine.submit(CommandBatch::new(vec![envelope("op-4", &["op-3"], "alice", &[("a", serde_json::json!(4))]).await]).await.unwrap(), options(), 3).await.unwrap();
    let head = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    let core = ArtifactId(document_id().await.0);
    let wal = storage.wal().await;
    let cancelled = || StdArc::new(std::sync::atomic::AtomicBool::new(false));
    let ids = |commits: &[ArtifactLedgerCommit]| commits.iter().map(|commit| commit.envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let whole = artifact_ledger_tail(&wal, &core, &ArtifactLedgerPoint::genesis(), &head, cancelled()).await.unwrap();
    assert_eq!(ids(&whole), vec![vec!["op-1".to_string()], vec!["op-2".to_string(), "op-3".to_string()], vec!["op-4".to_string()]]);
    assert_eq!(whole.iter().map(|commit| commit.point.clone()).collect::<Vec<_>>(), vec![first.clone(), second.clone(), head.clone()]);
    assert_eq!(whole[1].point.head_seq, 3);
    assert_eq!(whole[1].point.head_edit_id, Some(protocol::MutationId("op-3".to_string())));

    let tail = artifact_ledger_tail(&wal, &core, &first, &second, cancelled()).await.unwrap();
    assert_eq!(ids(&tail), vec![vec!["op-2".to_string(), "op-3".to_string()]]);
    assert_eq!(tail[0].envelopes[1], envelope("op-3", &["op-2"], "bob", &[("c", serde_json::json!(3))]).await);

    let mut forged = second.clone();
    forged.chain_hash[0] ^= 1;
    assert!(matches!(artifact_ledger_tail(&wal, &core, &first, &forged, cancelled()).await, Err(DbError::NotFound(_))));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &forged, &head, cancelled()).await, Err(DbError::NotFound(_))));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &head, &head, cancelled()).await, Err(DbError::InvalidArgument(_))));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &second, &first, cancelled()).await, Err(DbError::InvalidArgument(_))));
    let stopped = StdArc::new(std::sync::atomic::AtomicBool::new(true));
    assert!(matches!(artifact_ledger_tail(&wal, &core, &ArtifactLedgerPoint::genesis(), &head, stopped).await, Err(DbError::Unavailable(_))));
    drop(wal);
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

#[semio_framework_async_macros::async_test]
async fn materialize_from_snapshot_plus_wal_suffix_matches_full_replay() {
    let storage = storage().await;
    {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        for i in 0..3 {
            let key = format!("path-{i}");
            let value = format!("value-{i}");
            let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
            engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
        }
        engine.snapshot_now(10).await.unwrap();
        for i in 3..6 {
            let key = format!("path-{i}");
            let value = format!("value-{i}");
            let batch = CommandBatch::new(vec![envelope(&format!("op-{i}"), &[], "alice", &[(&key, serde_json::json!(value))]).await]).await.unwrap();
            engine.submit(batch, SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, i).await.unwrap();
        }
    }

    let (reopened, report) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 20).unwrap();
    assert!(report.from_snapshot);
    assert_eq!(report.commands_replayed, 3);
    assert_eq!(reopened.frontier().await.head_seq, 6);
    for i in 0..6 {
        let value: serde_json::Value = stored_json(reopened.get(&format!("path-{i}")).await.unwrap().unwrap()).await;
        assert_eq!(value, serde_json::json!(format!("value-{i}")));
    }
}

#[semio_framework_async_macros::async_test]
async fn deletion_via_json_null_tombstones_a_path() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_some());
    engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "alice", &[("x", serde_json::Value::Null)]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_none());
}
//#endregion 🔖️Engine submit + materialize + WAL replay

//#region 🔖️Deps + Dedupe
#[semio_framework_async_macros::async_test]
async fn submit_rejects_an_envelope_whose_dependency_was_never_applied() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let batch = CommandBatch::new(vec![envelope("op-2", &["op-1-never-applied"], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap();
    let result = engine.submit(batch, SubmitOptions::default(), 0);
    assert!(matches!(result.await, Err(DbError::InvalidArgument(_))));
    assert!(engine.get("x").await.unwrap().is_none(), "a rejected batch must not have partially applied");
}

#[semio_framework_async_macros::async_test]
async fn resubmitting_the_same_batch_returns_the_cached_receipt_without_advancing_the_frontier() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let batch = || db_actor::block_on(CommandBatch::new(vec![db_actor::block_on(envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]))])).unwrap();

    let first = engine.submit(batch(), SubmitOptions::default(), 0).await.unwrap();
    let frontier_after_first = engine.frontier().await;
    let second = engine.submit(batch(), SubmitOptions::default(), 1).await.unwrap();

    assert_eq!(first, second);
    assert_eq!(engine.frontier().await, frontier_after_first, "a deduped resubmit must not move the frontier");
}
/// 🪪️ Fixture law `🧫️fixtures/🪪️mutation-id-collision`: a replayed id is idempotent only while its
/// content matches the committed envelope; a colliding id is refused instead of silently dropped.
#[semio_framework_async_macros::async_test]
async fn a_committed_mutation_id_replays_idempotently_but_refuses_colliding_content() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪪️mutation-id-collision/🔣️.json")).unwrap();
    let row_envelope = |row: &serde_json::Value| db_actor::block_on(envelope(row["id"].as_str().unwrap(), &[], row["actor"].as_str().unwrap(), &[(row["path"].as_str().unwrap(), row["value"].clone())]));
    let mut engine = ArtifactEngine::create(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![row_envelope(&fixture["committed"])]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    for (tick, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let name = case["name"].as_str().unwrap();
        let before = engine.frontier().await;
        let outcome = engine.submit(CommandBatch::new(vec![row_envelope(case)]).await.unwrap(), SubmitOptions::default(), tick as u64 + 1).await;
        let after = engine.frontier().await;
        match case["outcome"].as_str().unwrap() {
            "replayed" => assert!(outcome.is_ok() && after == before, "{name}"),
            "conflict" => assert!(matches!(outcome, Err(DbError::Conflict(_))) && after == before, "{name}: {outcome:?}"),
            "committed" => assert!(outcome.is_ok() && after.commit_seq == before.commit_seq + 1, "{name}"),
            other => panic!("unknown fixture outcome {other}"),
        }
        assert_eq!(after.head_seq, case["headSeq"].as_u64().unwrap(), "{name}");
    }
}

/// ⚖️ C12 P1 (ticket 26/09/23 session 14): a link cut's whole outbox reaches the authority as ONE batch. The largest
/// batch of the smallest db envelope the document backbone declares legal (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES`,
/// admitted by the wire's own exact decoder) holds more envelopes than both former capacity refusals (the per-operation
/// DB I/O credit and the 4 096-command batch limit); it commits as one transaction and replays to the same frontier.
#[semio_framework_async_macros::async_test]
async fn a_declared_legal_byte_maximal_batch_commits_as_one_transaction_and_replays() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    let mut envelopes = Vec::new();
    let mut bytes = 0usize;
    loop {
        let next = envelope(&format!("{:x}", envelopes.len()), &[], "a", &[]).await;
        let mut encoded = Vec::new();
        protocol::encode_envelope(&next, &mut encoded);
        let count_bytes = if envelopes.len() + 1 < 128 { 1 } else if envelopes.len() + 1 < 16_384 { 2 } else { 3 };
        if envelopes.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
            let mut over = envelopes.clone();
            over.push(next);
            assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&over)).is_err(), "one more envelope leaves the declared batch");
            break;
        }
        bytes += encoded.len();
        envelopes.push(next);
    }
    let wire = protocol::encode_envelopes(&envelopes);
    assert_eq!(protocol::decode_document_backbone_envelopes_exact(&wire).unwrap().len(), envelopes.len(), "the batch is declared legal by the wire's own decoder");
    assert!(envelopes.len() > 4_096, "the byte-maximal batch ({} envelopes, {} bytes) exceeds the former 4 096-command limit", envelopes.len(), wire.len());
    let count = envelopes.len() as u64;
    let receipt = engine.submit(CommandBatch::new(envelopes).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }, 1).await.unwrap();
    assert_eq!((receipt.frontier.head_seq, receipt.frontier.commit_seq), (count, 1), "one transaction holds the whole batch");
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
    let (mut reopened, _) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 2).unwrap();
    assert_eq!(reopened.frontier().await, receipt.frontier);
    reopened.wal.close().await.unwrap();
    while reopened.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

/// ⚖️ C12 P1: a batch refused after admission (here: its second envelope names an unknown dependency) records nothing
/// in the replay guard, so its envelope commits when resent; a resend that merges an already-committed envelope with a
/// new one commits only the new one (the committed envelope is idempotent, never a replay refusal); one operation twice
/// in one batch is refused.
#[semio_framework_async_macros::async_test]
async fn an_envelope_of_a_refused_batch_commits_when_resent_and_a_merged_resend_is_idempotent() {
    let mut engine = ArtifactEngine::create(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).unwrap();
    let op = |id: &str, deps: &[&str], path: &str| db_actor::block_on(envelope(id, deps, "alice", &[(path, serde_json::json!(1))]));
    let refused = engine.submit(CommandBatch::new(vec![op("op-1", &[], "a"), op("op-2", &["never"], "b")]).await.unwrap(), SubmitOptions::default(), 1).await;
    assert!(matches!(refused, Err(DbError::InvalidArgument(_))));
    assert_eq!(engine.frontier().await.head_seq, 0);
    let resent = engine.submit(CommandBatch::new(vec![op("op-1", &[], "a")]).await.unwrap(), SubmitOptions::default(), 2).await.unwrap();
    assert_eq!((resent.frontier.head_seq, resent.frontier.commit_seq), (1, 1));
    let merged = engine.submit(CommandBatch::new(vec![op("op-1", &[], "a"), op("op-3", &["op-1"], "c")]).await.unwrap(), SubmitOptions::default(), 3).await.unwrap();
    assert_eq!((merged.frontier.head_seq, merged.frontier.commit_seq), (2, 2));
    let twice = engine.submit(CommandBatch::new(vec![op("op-4", &[], "d"), op("op-4", &[], "d")]).await.unwrap(), SubmitOptions::default(), 4).await;
    assert!(matches!(twice, Err(DbError::Conflict(_))));
    assert_eq!(engine.frontier().await.head_seq, 2);
}
//#endregion 🔖️Deps + Dedupe

//#region 🔖️Conflict
#[semio_framework_async_macros::async_test]
async fn concurrent_write_to_the_same_path_without_a_dependency_is_recorded_as_a_conflict() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    // op-2 writes the same path but does NOT declare op-1 as a dependency: a real concurrent
    // write from `op-2`'s author's point of view.
    let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert_eq!(receipt.conflicts.len(), 1);
    assert_eq!(receipt.conflicts[0].conflicting_with, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.conflicts[0].path, "x");
    // Last-writer-wins: the conflicting write still applies.
    let x: serde_json::Value = stored_json(engine.get("x").await.unwrap().unwrap()).await;
    assert_eq!(x, serde_json::json!(2));
}

#[semio_framework_async_macros::async_test]
async fn declaring_the_prior_writer_as_a_dependency_avoids_the_conflict() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    let receipt = engine.submit(CommandBatch::new(vec![envelope("op-2", &["op-1"], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert!(receipt.conflicts.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn preview_conflicts_uses_real_db_conflict_detector_against_recent_history() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    let probe = CommandBatch::new(vec![envelope("op-2", &[], "bob", &[("x", serde_json::json!(2))]).await]).await.unwrap();
    let conflicts = engine.preview_conflicts(&probe).await.unwrap();
    assert!(!conflicts.is_empty(), "db_conflict must detect the same-path intersection against recent history");
}
//#endregion 🔖️Conflict

//#region 🔖️Undo
#[semio_framework_async_macros::async_test]
async fn undo_applies_the_recorded_inverse_and_produces_a_fresh_commit() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let original = protocol::MutationEnvelope {
        mutation_id: protocol::MutationId("op-1".to_string()),
        document_id: document_id().await,
        actor: protocol::ActorId("alice".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::from(&serde_json::json!({ "x": 1 }))).await },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&DslValue::from(&serde_json::json!({ "x": null }))).await },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    };
    engine.submit(CommandBatch::new(vec![original]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    assert!(engine.get("x").await.unwrap().is_some());

    let receipt = engine.undo(&protocol::MutationId("op-1".to_string()), protocol::MutationId("op-1-undo".to_string()), protocol::ActorId("alice".to_string()), 1).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, 2);
    assert!(engine.get("x").await.unwrap().is_none(), "undo must have applied the recorded inverse (delete x)");
}

#[semio_framework_async_macros::async_test]
async fn undo_of_an_unknown_operation_errs_not_found() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let never_applied = protocol::MutationId("never-applied".to_string());
    let result = engine.undo(&never_applied, protocol::MutationId("undo-1".to_string()), protocol::ActorId("alice".to_string()), 0);
    assert!(matches!(result.await, Err(DbError::NotFound(_))));
}
//#endregion 🔖️Undo

//#region 🔖️Preview
#[semio_framework_async_macros::async_test]
async fn preview_is_never_durable_and_a_conflicting_commit_supersedes_it() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let preview_id = engine.publish_preview(&[("y".to_string(), Some(serde_json::json!("preview-value")))], 0).await.unwrap();
    assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Active);
    let preview_value: serde_json::Value = stored_json(engine.preview_get(&preview_id, "y").await.unwrap().unwrap()).await;
    assert_eq!(preview_value, serde_json::json!("preview-value"));
    assert!(engine.get("y").await.unwrap().is_none(), "a preview must never be visible in committed state");

    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("y", serde_json::json!("committed-value"))]).await]).await.unwrap(), SubmitOptions::default(), 1).await.unwrap();
    assert_eq!(engine.preview_status(&preview_id).await.unwrap(), db_preview::PreviewState::Superseded, "an intersecting real commit must supersede the preview");

    let committed: serde_json::Value = stored_json(engine.get("y").await.unwrap().unwrap()).await;
    assert_eq!(committed, serde_json::json!("committed-value"));
}

#[semio_framework_async_macros::async_test]
async fn preview_withdraw_and_expire_transitions() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();

    let withdrawn_id = engine.publish_preview(&[("a".to_string(), Some(serde_json::json!(1)))], 0).await.unwrap();
    engine.withdraw_preview(&withdrawn_id).await.unwrap();
    assert_eq!(engine.preview_status(&withdrawn_id).await.unwrap(), db_preview::PreviewState::Withdrawn);

    let dummy_id = db_preview::PreviewId("does-not-exist".to_string());
    assert!(matches!(engine.preview_status(&dummy_id).await, Err(DbError::NotFound(_))));
}
//#endregion 🔖️Preview

//#region 🔖️Security
#[semio_framework_async_macros::async_test]
async fn security_gate_rejects_a_principal_denied_by_its_policy() {
    // An empty `RoleBasedPolicy` (no grants at all) denies every action, per its own doc — a
    // default-deny policy, matching `db_engine`'s own equivalent test of the same gate.
    let security = db_security::SecurityGate::new(db_security::RoleBasedPolicy::new(), db_security::ReplayGuard::new(60_000, 1_024), db_security::BudgetRegistry::new(100_000, 100_000), Arc::new(NullEmit));
    let storage = storage().await;
    let config = ArtifactEngineConfig { security, ..ArtifactEngineConfig::default() };
    let mut engine = ArtifactEngine::create(document_id().await, storage, config, 0).unwrap();
    let result = engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "bob", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0);
    assert!(matches!(result.await, Err(DbError::Unauthorized(_))));
    assert!(engine.get("x").await.unwrap().is_none());
}
//#endregion 🔖️Security

//#region 🔖️Query
#[semio_framework_async_macros::async_test]
async fn query_finds_a_committed_row_by_path() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("greeting", serde_json::json!("hello"))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();

    let query = db_query::Query::new().filter(db_query::Predicate::Eq(db_query::Path::empty().push_field("path"), db_query::Value::Text("greeting".to_string())));
    let result = engine.query(query, db_query::Consistency::Canonical).await.unwrap();
    assert_eq!(result.rows.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn live_query_refresh_reports_no_further_diff_right_after_submit_already_refreshed_it() {
    let storage = storage().await;
    let mut engine = ArtifactEngine::create(document_id().await, storage, ArtifactEngineConfig::default(), 0).unwrap();
    let id = engine.subscribe(db_query::LiveQuerySpec { query: db_query::Query::new(), consistency: db_query::Consistency::Canonical }).await;
    engine.submit(CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("x", serde_json::json!(1))]).await]).await.unwrap(), SubmitOptions::default(), 0).await.unwrap();
    let diffs = engine.refresh_live_queries().await;
    assert!(diffs.is_empty());
    engine.unsubscribe(id).await;
}
//#endregion 🔖️Query

//#region 🔖️IndexBacklog + Receipts
async fn single_envelope_commit(engine: &mut ArtifactEngine, index: u64, durability: DurabilityClass) -> CommandReceipt {
    let id = format!("op-{index}");
    engine.submit(CommandBatch::new(vec![envelope(&id, &[], "alice", &[]).await]).await.unwrap(), SubmitOptions { durability, ..Default::default() }, index).await.unwrap()
}

async fn fs_storage(name: &str) -> (StdArc<db_storage::DbBackend>, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!("db_artifact_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    (StdArc::new(db_storage::DbBackend::Fs(db_storage::FsStorage::open(db_storage::db_io_test_pool(), &dir).await.unwrap())), dir)
}

async fn close_engine(engine: &mut ArtifactEngine) {
    if engine.close_flush_pending() {
        engine.close_flush().await.unwrap();
    }
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}

/// ⚖️ A commit's receipt never waits for index I/O: 300 acknowledged single-envelope commits leave no index run behind
/// (the runner writes them after replying) — the state a crash after the Ack and before the index write leaves. The
/// reopened document refills its backlog from the WAL, its maintenance writes every full run, and later commits continue
/// the same runs: every indexed seq resolves to its WAL position, nothing twice, nothing missing.
#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn index_runs_follow_the_receipt_and_a_reopen_refills_them_from_the_wal() {
    let (storage, dir) = fs_storage("index_follows_receipt").await;
    let before = {
        let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
        for index in 1..=300 {
            single_envelope_commit(&mut engine, index, DurabilityClass::Os).await;
        }
        let frontier = engine.frontier().await;
        close_engine(&mut engine).await;
        frontier
    };
    let core = to_core_document_id(&document_id().await).await;
    let index_facet = storage.index().await;
    assert_eq!(db_index::CommandIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 0, "no receipt waited for an index run");
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 0);

    let (mut reopened, _) = ArtifactEngine::open(document_id().await, &storage, ArtifactEngineConfig::default(), 301).unwrap();
    assert_eq!(reopened.frontier().await, before);
    reopened.maintain_index().await.unwrap();
    let commands = db_index::CommandIndex::new(&index_facet, core.clone()).await;
    assert_eq!(commands.indexed_through().await.unwrap(), 256, "four full runs from the WAL suffix; the partial tail waits");
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core.clone()).await.indexed_through().await.unwrap(), 256);
    for index in 301..=400 {
        single_envelope_commit(&mut reopened, index, DurabilityClass::Os).await;
        reopened.maintain_index().await.unwrap();
    }
    assert_eq!(commands.indexed_through().await.unwrap(), 384);
    for seq in [1u64, 64, 65, 256, 257, 300, 301, 384] {
        assert_eq!(commands.lookup(seq).await.unwrap().map(|location| location.offset), Some(seq), "seq {seq}");
    }
    assert_eq!(commands.lookup(385).await.unwrap(), None);
    let mut control = db_index::IndexHandle::new(&index_facet, core.clone(), db_index::IndexKind::Command).await.operation_control(65_536).unwrap();
    db_index::IndexHandle::new(&index_facet, core, db_index::IndexKind::Command).await.verify(&mut control).await.unwrap();
    close_engine(&mut reopened).await;
    drop(reopened);
    drop(index_facet);
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}

/// ⚖️ The engine keeps the receipts of its newest `APPLIED_RECEIPTS_MAX` batches only; a whole-batch resend inside the window
/// answers its original receipt, an older one is answered by the per-envelope dedupe without a new commit.
#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn applied_receipts_keep_a_bounded_window_and_older_resends_stay_idempotent() {
    let (storage, dir) = fs_storage("receipt_window").await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    let mut last = None;
    for index in 1..=(APPLIED_RECEIPTS_MAX as u64 + 10) {
        last = Some(single_envelope_commit(&mut engine, index, DurabilityClass::Os).await);
    }
    assert_eq!(engine.applied_receipts.len(), APPLIED_RECEIPTS_MAX);
    assert_eq!(engine.applied_receipt_order.len(), APPLIED_RECEIPTS_MAX);
    let frontier = engine.frontier().await;
    let newest = single_envelope_commit(&mut engine, APPLIED_RECEIPTS_MAX as u64 + 10, DurabilityClass::Os).await;
    assert_eq!(Some(newest), last, "a resend inside the window answers its original receipt");
    let oldest = single_envelope_commit(&mut engine, 1, DurabilityClass::Os).await;
    assert_eq!(oldest.frontier, frontier, "a resend beyond the window commits nothing");
    assert_eq!(engine.frontier().await, frontier);
    assert_eq!(engine.applied_receipts.len(), APPLIED_RECEIPTS_MAX);
    close_engine(&mut engine).await;
    drop(engine);
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}

/// ⚖️ A document stays writable far past the old 4 096-run listing ceiling (4 runs per 64 single-envelope commits: every
/// commit failed after ~65.5 k of them): 70 016 single-envelope commits on the filesystem backend, each followed by the
/// runner's index maintenance, all commit; the four kinds hold a few hundred runs, every indexed seq resolves.
#[cfg(feature = "fs")]
#[semio_framework_async_macros::async_test]
async fn seventy_thousand_single_envelope_commits_stay_writable_with_bounded_index_runs() {
    const COMMITS: u64 = 70_016;
    let (storage, dir) = fs_storage("seventy_thousand").await;
    let mut engine = ArtifactEngine::create(document_id().await, storage.clone(), ArtifactEngineConfig::default(), 0).unwrap();
    for index in 1..=COMMITS {
        single_envelope_commit(&mut engine, index, DurabilityClass::Memory).await;
        engine.maintain_index().await.unwrap_or_else(|error| panic!("index maintenance after commit {index}: {error}"));
    }
    assert_eq!(engine.frontier().await.head_seq, COMMITS);
    let core = to_core_document_id(&document_id().await).await;
    let index_facet = storage.index().await;
    let mut runs = 0usize;
    for kind in [db_index::IndexKind::Command, db_index::IndexKind::Inverse, db_index::IndexKind::ActorSeq, db_index::IndexKind::Frontier] {
        let handle = db_index::IndexHandle::new(&index_facet, core.clone(), kind).await;
        let mut control = handle.operation_control(1_000_000).unwrap();
        let stats = handle.stats(&mut control).await.unwrap();
        assert!(stats.entry_count >= COMMITS, "{kind:?} holds every entry");
        handle.verify(&mut control).await.unwrap();
        runs += stats.run_count;
    }
    assert!(runs <= 300, "{runs} runs for {COMMITS} commits");
    let commands = db_index::CommandIndex::new(&index_facet, core.clone()).await;
    assert_eq!(commands.indexed_through().await.unwrap(), COMMITS);
    for seq in [COMMITS - 63, COMMITS] {
        assert_eq!(commands.lookup(seq).await.unwrap().map(|location| location.offset), Some(seq), "seq {seq}");
    }
    assert_eq!(db_index::FrontierIndex::new(&index_facet, core).await.indexed_through().await.unwrap(), COMMITS);
    close_engine(&mut engine).await;
    drop(engine);
    drop(index_facet);
    drop(storage);
    let _ = std::fs::remove_dir_all(&dir);
}
//#endregion 🔖️IndexBacklog + Receipts

//#region 🔖️Actor
async fn journal_authority() -> (ArtifactAuthority, StdArc<db_storage::DbBackend>, StdArc<semio_framework_async::WorkerPool>) {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let storage = storage().await;
    let engine_storage = storage.clone();
    let authority = ArtifactAuthority::spawn(pool.clone(), move || async move { ArtifactEngine::create_retained(protocol::ArtifactId("map-a".to_string()), engine_storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) }, MailboxCapacities::uniform(16)).await.unwrap();
    (authority, storage, pool)
}

fn journal_grant() -> store::ArtifactStoreOneItemGrant {
    store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::durable_group::DURABLE_OWNED_GROUP_EVENT_MAX_BYTES }
}

//#region 🔖️RecoveryFixtureStore
/// #⃣ The smallest concrete projection/operation pair a durable-group recovery law can open real
/// `store::ArtifactStore`s over: a projection that IS one content hash and an operation that
/// overwrites it (its `inverse` restores the prior hash). Test-only fixture.
#[derive(Clone, Debug, Default, PartialEq, store::ToValue, store::FromValue)]
pub struct HashProjection {
    pub latest_hash: [u8; 32],
}

impl store::os_schema_composition::ArtifactCompositionFields for HashProjection {
    fn visit_child_refs<'a, V: store::os_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl store::ArtifactDsl for HashProjection {
    const EXTENSION: &'static str = "dbhash";

    fn parse_dsl(text: &str) -> Result<HashProjection, semio_framework_diagnostic::TextError> {
        let trimmed = text.trim();
        if trimmed.len() != 64 || !trimmed.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected 64 lowercase hex characters", semio_framework_diagnostic::TextSpan::at(1, 1)));
        }
        let mut latest_hash = [0u8; 32];
        for (index, slot) in latest_hash.iter_mut().enumerate() {
            *slot = u8::from_str_radix(&trimmed[index * 2..index * 2 + 2], 16).map_err(|_| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "invalid hex byte", semio_framework_diagnostic::TextSpan::at(1, (index * 2 + 1) as u32)))?;
        }
        Ok(HashProjection { latest_hash })
    }

    fn print_dsl(&self) -> String {
        let mut out = String::with_capacity(64);
        for byte in self.latest_hash {
            use std::fmt::Write;
            let _ = write!(out, "{byte:02x}");
        }
        out
    }
}

impl store::ArtifactPack for HashProjection {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(self.latest_hash.to_vec())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let latest_hash: [u8; 32] = bytes.try_into().map_err(|_| store::PackError::Schema("HashProjection pack must be exactly 32 bytes".to_string()))?;
        Ok(HashProjection { latest_hash })
    }
}

#[derive(Clone, Debug, Default, store::ToValue, store::FromValue)]
pub struct HashDiff {
    pub hash: Option<[u8; 32]>,
}

impl protocol::MutationDiff<HashProjection> for HashDiff {
    fn apply(&self, base: &HashProjection) -> protocol::MutationApplyResult<HashProjection> {
        Ok(match self.hash {
            Some(hash) => HashProjection { latest_hash: hash },
            None => base.clone(),
        })
    }

    fn absorb(&mut self, other: HashDiff) {
        if other.hash.is_some() {
            self.hash = other.hash;
        }
    }
}

#[derive(Clone, Debug, PartialEq, store::ToValue, store::FromValue)]
pub struct HashMutation {
    pub hash: [u8; 32],
    pub author: Option<protocol::ActorId>,
    pub timestamp: Option<protocol::HybridLogicalTimestamp>,
}

const HASH_MUTATION_DESCRIPTOR: protocol::MutationLeafDescriptor = protocol::MutationLeafDescriptor {
    schema_version: 1,
    owner: "framework/os/db/tests/recovery-fixture/hash-mutation",
    semantic_kind: "set-hash",
    display_name: "Set Hash",
    emoji: "#️⃣",
    aggregate_variant: "HashMutation",
    payload_schema: "db.hash/v1",
    text_opcode: None,
    binary_tag: None,
    invertibility: protocol::MutationInvertibility::ExplicitMutation,
    diff_participation: protocol::MutationDiffParticipation::Detect,
    outcome_classes: &[protocol::MutationOutcomeClass::Applied],
    composition: protocol::MutationComposition::Atomic,
    required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
};

impl protocol::Mutation<HashProjection> for HashMutation {
    type Diff = HashDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[HASH_MUTATION_DESCRIPTOR];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &HASH_MUTATION_DESCRIPTOR
    }

    fn diff(&self, _base: &HashProjection) -> protocol::MutationOutcome<HashDiff> {
        protocol::MutationOutcome::new(HashDiff { hash: Some(self.hash) })
    }

    /// ↩️ The true inverse: an operation that would restore `base`'s hash — not a
    /// no-op placeholder.
    fn inverse(&self, base: &HashProjection) -> Vec<HashMutation> {
        vec![HashMutation { hash: base.latest_hash, author: self.author.clone(), timestamp: self.timestamp }]
    }

    fn author_id(&self) -> Option<protocol::ActorId> {
        self.author.clone()
    }

    fn timestamp(&self) -> Option<protocol::HybridLogicalTimestamp> {
        self.timestamp
    }
}

// 🚫️async: E1 pure accessor consumed synchronously inside `format!` — see R9
fn hex_encode(bytes: &[u8; 32]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(64);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn hex_decode(text: &str) -> Result<[u8; 32], String> {
    if text.len() != 64 || !text.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("expected 64 lowercase hex characters".to_string());
    }
    let mut out = [0u8; 32];
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).map_err(|error| error.to_string())?;
    }
    Ok(out)
}

/// 🎯️ Single-line text form: `hash=<hex64>[ author=<id>][ ts=<actor>,<physical_ms>,<logical>]`.
impl protocol::OpText for HashMutation {
    fn print_op(&self) -> String {
        let mut out = format!("hash={}", hex_encode(&self.hash));
        if let Some(author) = &self.author {
            out.push_str(&format!(" author={}", author.0));
        }
        if let Some(ts) = &self.timestamp {
            out.push_str(&format!(" ts={},{},{}", ts.actor, ts.physical_ms, ts.logical));
        }
        out
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let err = |detail: String| semio_framework_diagnostic::TextError::new(detail, semio_framework_diagnostic::TextSpan::at(1, 1));
        let mut hash = None;
        let mut author = None;
        let mut timestamp = None;
        for token in line.split_whitespace() {
            let (key, value) = token.split_once('=').ok_or_else(|| err(format!("malformed token '{token}'")))?;
            match key {
                "hash" => hash = Some(hex_decode(value).map_err(err)?),
                "author" => author = Some(protocol::ActorId(value.to_string())),
                "ts" => {
                    let parts: Vec<&str> = value.split(',').collect();
                    if parts.len() != 3 {
                        return Err(err(format!("malformed ts '{value}'")));
                    }
                    let actor = parts[0].parse::<u64>().map_err(|error| err(error.to_string()))?;
                    let physical_ms = parts[1].parse::<u64>().map_err(|error| err(error.to_string()))?;
                    let logical = parts[2].parse::<u64>().map_err(|error| err(error.to_string()))?;
                    timestamp = Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical });
                }
                other => return Err(err(format!("unknown key '{other}'"))),
            }
        }
        Ok(HashMutation { hash: hash.ok_or_else(|| err("missing hash".to_string()))?, author, timestamp })
    }
}

/// 🎯️ Binary form: `hash 32 bytes | presence u8 (bit0=author, bit1=timestamp) | [author
/// len varint + utf8 bytes] | [timestamp: actor/physical_ms/logical varint each]`.
impl protocol::OpBinary for HashMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = self.hash.to_vec();
        let presence = (self.author.is_some() as u8) | ((self.timestamp.is_some() as u8) << 1);
        out.push(presence);
        if let Some(author) = &self.author {
            pack::os_pack::write_varint_u64(&mut out, author.0.len() as u64);
            out.extend_from_slice(author.0.as_bytes());
        }
        if let Some(ts) = &self.timestamp {
            pack::os_pack::write_varint_u64(&mut out, ts.actor);
            pack::os_pack::write_varint_u64(&mut out, ts.physical_ms);
            pack::os_pack::write_varint_u64(&mut out, ts.logical);
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |detail: String| protocol::ProtocolError::Malformed { what: "hash op", offset: 0, detail };
        if bytes.len() < 33 {
            return Err(malformed("truncated hash op".to_string()));
        }
        let hash: [u8; 32] = bytes[..32].try_into().expect("checked len");
        let presence = bytes[32];
        let mut pos = 33usize;
        let author = if presence & 0b01 != 0 {
            let len = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))? as usize;
            let end = pos + len;
            let text = std::str::from_utf8(bytes.get(pos..end).ok_or_else(|| malformed("truncated author".to_string()))?).map_err(|error| malformed(error.to_string()))?.to_string();
            pos = end;
            Some(protocol::ActorId(text))
        } else {
            None
        };
        let timestamp = if presence & 0b10 != 0 {
            let actor = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
            let physical_ms = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
            let logical = pack::os_pack::read_varint_u64(bytes, &mut pos).map_err(|error| malformed(error.to_string()))?;
            Some(protocol::HybridLogicalTimestamp { actor, physical_ms, logical })
        } else {
            None
        };
        Ok(HashMutation { hash, author, timestamp })
    }
}

struct HashOwnedRetirement<T>(Option<T>);

impl<T: Send> store::ErasedSnapshotRetirement for HashOwnedRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

struct HashOwnedRetirementFactory;

impl<T: Send + 'static> store::ArtifactOwnedValueRetirementFactory<T> for HashOwnedRetirementFactory {
    fn retire_owned(&self, value: T) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(HashOwnedRetirement(Some(value)))
    }
}

struct HashSnapshotRetirement(Option<Arc<HashProjection>>);

impl store::ErasedSnapshotRetirement for HashSnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

struct HashSnapshotRetirementFactory;

impl store::SnapshotRetirementFactory<HashProjection> for HashSnapshotRetirementFactory {
    fn retire(&self, snapshot: Arc<HashProjection>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(HashSnapshotRetirement(Some(snapshot)))
    }
}

impl semio_framework_value::retirement::RetireOwned for HashProjection {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        semio_framework_value::retirement::RetireOwned::retirement(self.latest_hash)
    }
}

impl store::MemberStoreOwner<HashMutation> for HashProjection {
    /// 📦️ The fixture projection opens as an owned member through its own `ArtifactPack` codec.
    type SnapshotOpen = store::PackMemberSnapshotOpen<Self>;

    fn member_store_owners() -> store::DocumentStoreOwners<Self, HashMutation> {
        store::DocumentStoreOwners::new(
            Arc::new(HashSnapshotRetirementFactory),
            Arc::new(HashOwnedRetirementFactory),
            Arc::new(HashOwnedRetirementFactory),
            Box::new(store::ArtifactStoreCursorDisposer::<HashProjection, HashMutation>::new()),
        )
    }
}
//#endregion 🔖️RecoveryFixtureStore

async fn committed_recovery_hash_store(id: &str, dialect: store::os_io::ArtifactDialect, owner: Option<store::OwnerRef>) -> store::ArtifactStore<HashProjection, HashMutation> {
    use store::MemberStoreOwner as _;
    let mut envelope = store::create_document_envelope::<HashProjection, HashMutation>("db.hash/v1", id, HashProjection::default(), None);
    envelope.dialect = Some(dialect);
    envelope.owner = owner;
    let mut store = store::ArtifactStore::new(envelope).await.expect("committed recovery fixture creates one exact Store");
    store.install_document_store_owners_exact(HashProjection::member_store_owners());
    store
}

fn close_committed_recovery_hash_store(store: &mut store::ArtifactStore<HashProjection, HashMutation>) {
    for _ in 0..4_096 {
        if store::SpaceMember::close_owned_step(store, 1, 4_096).expect("committed recovery fixture Store closes") == store::SnapshotRetirementStep::Complete {
            assert!(store::SpaceMember::close_owned_terminal_is_empty(store));
            return;
        }
    }
    panic!("committed recovery fixture Store did not reach terminal handoff");
}

fn close_journal_commit(commit: &mut dyn store::durable_group::DurableOwnedGroupJournalCommitV1) {
    commit.begin_close();
    for _ in 0..8 {
        if commit.close_step(journal_grant()).unwrap() == store::SnapshotRetirementStep::Complete {
            assert!(commit.terminal_is_empty());
            return;
        }
    }
    panic!("journal commit did not reach its terminal-empty witness");
}

async fn shutdown_journal_authority(authority: &ArtifactAuthority, pool: &semio_framework_async::WorkerPool) {
    let mut terminal_job = None;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while std::time::Instant::now() < deadline {
        if authority.shutdown_step() {
            pool.shutdown();
            return;
        }
        if terminal_job.is_none() {
            terminal_job = authority.take_terminal_job();
        }
        if let Some(job) = terminal_job.take() {
            if let Err(retained) = job.close() {
                terminal_job = Some(retained);
            }
        }
        semio_framework_async::yield_once().await;
    }
    panic!("journal authority did not terminate: {}", authority.shutdown_debug_witness());
}

async fn durable_group_witness_batch(kinds: &[serde_json::Value], canonical_pack: &[u8]) -> db_wal::WalRecordBatch {
    let mut control = db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
    let mut records = db_wal::WalRecordBatch::new();
    for kind in kinds {
        let bytes = admit_wal_bytes(canonical_pack.to_vec(), db_storage::DB_IO_MAX_READ_BYTES, &mut control).await.unwrap();
        let record = match kind.as_str().unwrap() {
            "event" => db_wal::WalRecord::Event(bytes),
            "command" => db_wal::WalRecord::Command(bytes),
            other => panic!("unknown durable witness record kind {other}"),
        };
        push_wal_record(&mut records, record, &mut control).await.unwrap();
    }
    records
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_commits_one_exact_fsync_event() {
    let (authority, storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json"));
    let canonical_pack = record.canonical_pack().to_vec();
    let decision_sha256 = record.decision_sha256().to_string();
    let anchor_sha256 = record.anchor_sha256().to_string();
    let durable_head_edit_id = protocol::MutationId(record.parent_edit_id().to_string());
    let durable_chain_hash = record.parent_post_revision();
    let mut sink = authority.durable_group_journal_sink(7);
    let mut commit = sink.begin_commit(canonical_pack.clone(), decision_sha256.clone());
    let receipt = loop {
        match commit.advance(journal_grant()).unwrap() {
            store::durable_group::DurableOwnedGroupJournalAdvanceV1::Pending => semio_framework_async::yield_once().await,
            store::durable_group::DurableOwnedGroupJournalAdvanceV1::Committed(receipt) => break receipt,
            other => panic!("exact durable journal was not committed: {other:?}"),
        }
    };
    assert_eq!(receipt.anchor_sha256, anchor_sha256);
    assert_eq!(receipt.decision_sha256, decision_sha256);
    let durable_snapshot = authority.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(durable_snapshot.frontier.head_seq, 1);
    assert_eq!(durable_snapshot.frontier.commit_seq, 1);
    assert_eq!(durable_snapshot.frontier.chain_hash, durable_chain_hash);
    assert_eq!(durable_snapshot.head_edit_id, Some(durable_head_edit_id.clone()));
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    let mut ordinary = envelope("ordinary-after-durable-decision", &[], "map-owner", &[("/ordinary", serde_json::json!(true))]).await;
    ordinary.document_id = protocol::ArtifactId("map-a".to_string());
    authority.submit(CommandBatch::new(vec![ordinary]).await.unwrap(), SubmitOptions { durability: DurabilityClass::Fsync, ..SubmitOptions::default() }, 8).await.unwrap();
    let ordinary_snapshot = authority.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(ordinary_snapshot.frontier.head_seq, 2);
    assert_eq!(ordinary_snapshot.frontier.commit_seq, 2);
    assert_eq!(ordinary_snapshot.head_edit_id, Some(protocol::MutationId("ordinary-after-durable-decision".to_string())));
    shutdown_journal_authority(&authority, &pool).await;

    let (mut reopened, report) = ArtifactEngine::open_retained(protocol::ArtifactId("map-a".to_string()), storage.clone(), ArtifactEngineConfig::default(), 9).await.unwrap();
    assert_eq!(report.commands_replayed, 1);
    let reopened_snapshot = reopened.checkpoint_publication_snapshot().await;
    assert_eq!(reopened_snapshot, ordinary_snapshot, "reopen must reconstruct the exact Event-plus-Command publication frontier");
    reopened.close().await.unwrap();
    drop(reopened);

    let wal_facet = storage.wal().await;
    let mut replay = db_wal::replay_committed_document(
        &wal_facet,
        &ArtifactId::from("map-a"),
        db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap(),
    )
    .await
    .unwrap();
    let replay_document = ArtifactId::from("map-a");
    let mut witnesses = Vec::new();
    let mut committed_transactions = 0;
    loop {
        let transaction = match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Transaction(transaction) => transaction,
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => break,
        };
        committed_transactions += 1;
        if let Some(witness) = committed_durable_group_decision_from_transaction(&replay_document, transaction).await.unwrap() {
            witnesses.push(witness);
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert_eq!(committed_transactions, 2);
    assert_eq!(witnesses.len(), 1);
    let witness = witnesses.pop().unwrap();
    assert_eq!(witness.document(), &replay_document);
    assert_eq!(witness.transaction_id(), receipt.transaction_id);
    assert_eq!(witness.segment_index(), receipt.segment_index);
    assert_eq!(witness.receipt(), receipt);
    assert_eq!(witness.record.canonical_pack(), canonical_pack);
    assert_eq!(witness.record.decision_sha256(), decision_sha256);
    assert_eq!(receipt.transaction_id, 1);
    let source = include_str!("../../🦀️.rs");
    let witness_api = &source[source.find("pub struct ArtifactCommittedDurableGroupDecisionV1").unwrap()..source.find("pub(crate) async fn committed_durable_group_decision_from_transaction").unwrap()];
    assert!(witness_api.contains("into_store_owned_recovery") && witness_api.contains("take_rejected_terminal"));
    assert!(!witness_api.contains("fn record(&self)") && !witness_api.contains("fn into_record("));
    assert!(!witness_api.contains("fn cancel("));
}

#[semio_framework_async_macros::async_test]
async fn committed_durable_group_decision_accepts_only_one_exact_event_transaction() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📓️durable-group-journal/🔣️.json")).unwrap();
    let document = ArtifactId::from(fixture["record"]["document"].as_str().unwrap());
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json"));
    let canonical_pack = record.canonical_pack().to_vec();
    let backing = storage().await;
    let wal_storage = backing.wal().await;
    let mut wal = db_wal::ArtifactWal::create(&wal_storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0).await.unwrap();
    for (ordinal, row) in fixture["committedDecisionWitnessCases"].as_array().unwrap().iter().filter(|row| row["transaction"] == "committed").enumerate() {
        let mut records = durable_group_witness_batch(row["recordKinds"].as_array().unwrap(), &canonical_pack).await;
        let receipt = wal.submit(&wal_storage, &[], &records, DurabilityClass::Fsync, ordinal as u64 + 1).await.unwrap();
        assert!(receipt.committed);
        close_wal_record_batch(&mut records).await.unwrap();
    }
    wal.close().await.unwrap();

    let mut replay =
        db_wal::replay_committed_document(&wal_storage, &document, db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap())
            .await
            .unwrap();
    for row in fixture["committedDecisionWitnessCases"].as_array().unwrap().iter().filter(|row| row["transaction"] == "committed") {
        let transaction = loop {
            match replay.next_transaction_step().await.unwrap() {
                db_wal::WalCommittedStep::Transaction(transaction) => break transaction,
                db_wal::WalCommittedStep::Yield => continue,
                db_wal::WalCommittedStep::Done => panic!("durable witness fixture lost a committed transaction"),
            }
        };
        let replay_document = if row["replayDocument"] == "foreign" { ArtifactId::from("foreign-map") } else { document.clone() };
        let outcome = committed_durable_group_decision_from_transaction(&replay_document, transaction).await;
        match row["expected"].as_str().unwrap() {
            "witness" => {
                let witness = outcome.unwrap().expect("one exact Event must produce a committed decision witness");
                assert_eq!(witness.document(), &replay_document);
                assert_eq!(witness.record.canonical_pack(), canonical_pack);
            }
            "ignored" => assert!(outcome.unwrap().is_none()),
            "rejected" => assert!(matches!(outcome, Err(DbError::Corrupt(_)))),
            other => panic!("unknown durable witness expectation {other}"),
        }
    }
    loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => break,
            db_wal::WalCommittedStep::Transaction(_) => panic!("durable witness fixture replayed an unexpected committed transaction"),
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert!(replay.terminal_is_empty());

    let aborted = db_wal::tests::aborted_event_fixture_storage(&document, &canonical_pack).await;
    let mut replay =
        db_wal::replay_committed_document(&aborted, &document, db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap()).await.unwrap();
    loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => break,
            db_wal::WalCommittedStep::Transaction(_) => panic!("aborted durable witness fixture exposed a committed transaction"),
        }
    }
    while replay.close_owner_step().unwrap() {}
    assert!(replay.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection() {
    let document = ArtifactId::from("map-a");
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json"));
    let anchor_sha256 = record.anchor_sha256().to_string();
    let decision_sha256 = record.decision_sha256().to_string();
    let canonical_pack = record.into_canonical_pack();
    let backing = storage().await;
    let wal_storage = backing.wal().await;
    let mut wal = db_wal::ArtifactWal::create(&wal_storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0).await.unwrap();
    let mut records = durable_group_witness_batch(&[serde_json::Value::String("event".to_string())], &canonical_pack).await;
    wal.submit(&wal_storage, &[], &records, DurabilityClass::Fsync, 1).await.unwrap();
    close_wal_record_batch(&mut records).await.unwrap();
    wal.close().await.unwrap();
    let mut replay =
        db_wal::replay_committed_document(&wal_storage, &document, db_wal::WalCursorControl::new(StdArc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap())
            .await
            .unwrap();
    let transaction = loop {
        match replay.next_transaction_step().await.unwrap() {
            db_wal::WalCommittedStep::Transaction(transaction) => break transaction,
            db_wal::WalCommittedStep::Yield => continue,
            db_wal::WalCommittedStep::Done => panic!("committed recovery fixture lost its decision transaction"),
        }
    };
    let witness = committed_durable_group_decision_from_transaction(&document, transaction).await.unwrap().expect("one committed Event produces one opaque DB witness");
    while replay.close_owner_step().unwrap() {}

    let parent_dialect = store::os_io::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() };
    let child_dialect = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let parent_reference = store::os_io::ArtifactRef { artifact_id: "map-a".into(), dialect: parent_dialect.clone() };
    let parent = committed_recovery_hash_store("map-a", parent_dialect, None).await;
    let drawing = committed_recovery_hash_store("gismap-drawing", child_dialect("drawing"), Some(store::OwnerRef { parent: parent_reference.clone(), slot: "drawing".into(), child_id: "gismap-drawing".into() })).await;
    let value = committed_recovery_hash_store("gismap-value", child_dialect("value"), Some(store::OwnerRef { parent: parent_reference, slot: "value".into(), child_id: "gismap-value".into() })).await;
    let parent_before = (parent.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&parent), store::SpaceMember::owner_ref(&parent));
    let drawing_before = (drawing.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&drawing), store::SpaceMember::owner_ref(&drawing));
    let value_before = (value.snapshot_pack().await.unwrap(), store::SpaceMember::artifact_ref(&value), store::SpaceMember::owner_ref(&value));
    let admission = store::durable_group::DurableOwnedMapRecoveryAdmissionV1::new(parent, drawing, value);
    let mut recovery = witness.into_store_owned_recovery(admission);
    assert_eq!(recovery.advance(journal_grant()), ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault(store::durable_group::DurableOwnedGroupDecisionError::InvalidFrontier));
    let mut terminal = recovery.take_rejected_terminal().expect("pre-mutation rejection consumes the witness and returns all three exact Store owners");
    assert!(recovery.terminal_is_empty());
    assert_eq!(terminal.document, document);
    assert_eq!(terminal.receipt.transaction_id, 1);
    assert_eq!(terminal.receipt.anchor_sha256, anchor_sha256);
    assert_eq!(terminal.receipt.decision_sha256, decision_sha256);
    assert_eq!(terminal.error, store::durable_group::DurableOwnedGroupDecisionError::InvalidFrontier);
    assert_eq!([terminal.owners.parent.generation(), terminal.owners.drawing.generation(), terminal.owners.value.generation()], [0, 0, 0]);
    let parent_after = terminal.owners.parent.snapshot_pack().await.unwrap();
    let drawing_after = terminal.owners.drawing.snapshot_pack().await.unwrap();
    let value_after = terminal.owners.value.snapshot_pack().await.unwrap();
    assert_eq!((parent_after.pack, parent_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.parent), store::SpaceMember::owner_ref(&terminal.owners.parent)), (parent_before.0.pack, parent_before.0.spr, parent_before.1, parent_before.2));
    assert_eq!(
        (drawing_after.pack, drawing_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.drawing), store::SpaceMember::owner_ref(&terminal.owners.drawing)),
        (drawing_before.0.pack, drawing_before.0.spr, drawing_before.1, drawing_before.2)
    );
    assert_eq!((value_after.pack, value_after.spr, store::SpaceMember::artifact_ref(&terminal.owners.value), store::SpaceMember::owner_ref(&terminal.owners.value)), (value_before.0.pack, value_before.0.spr, value_before.1, value_before.2));
    close_committed_recovery_hash_store(&mut terminal.owners.value);
    close_committed_recovery_hash_store(&mut terminal.owners.drawing);
    close_committed_recovery_hash_store(&mut terminal.owners.parent);
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_cancellation_before_handoff_is_absent() {
    let (authority, _storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json"));
    let mut sink = authority.durable_group_journal_sink(9);
    let mut commit = sink.begin_commit(record.canonical_pack().to_vec(), record.decision_sha256().to_string());
    commit.cancel();
    assert_eq!(commit.advance(journal_grant()).unwrap(), store::durable_group::DurableOwnedGroupJournalAdvanceV1::Absent);
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    shutdown_journal_authority(&authority, &pool).await;
}

#[semio_framework_async_macros::async_test]
async fn document_authority_durable_group_journal_rejects_hash_before_mailbox() {
    let (authority, _storage, pool) = journal_authority().await;
    let record = store::durable_group::durable_owned_group_journal_test_record(include_str!("../../../../🏪️store/🧩️composition/🗄️durable-group/🧫️fixtures/🔣️.json"));
    let mut sink = authority.durable_group_journal_sink(11);
    let mut commit = sink.begin_commit(record.canonical_pack().to_vec(), "0".repeat(64));
    assert!(matches!(commit.advance(journal_grant()).unwrap(), store::durable_group::DurableOwnedGroupJournalAdvanceV1::Rejected(_)));
    close_journal_commit(commit.as_mut());
    drop(commit);
    drop(sink);
    shutdown_journal_authority(&authority, &pool).await;
}

#[semio_framework_async_macros::async_test]
async fn document_authority_submits_and_queries_over_finite_pool_turns() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let storage = storage().await;
    let document = document_id().await;
    let authority = ArtifactAuthority::spawn(pool.clone(), move || async move { ArtifactEngine::create_retained(document, storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) }, MailboxCapacities::uniform(16)).await.unwrap();

    let batch = CommandBatch::new(vec![envelope("op-1", &[], "alice", &[("name", serde_json::json!("hi"))]).await]).await.unwrap();
    let receipt = authority.submit(batch, SubmitOptions::default(), 0).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, 1);

    let queried: serde_json::Value = stored_json(authority.query("name").await.unwrap().unwrap()).await;
    assert_eq!(queried, serde_json::json!("hi"));

    let frontier = authority.frontier().await.unwrap();
    assert_eq!(frontier.head_seq, 1);

    let generation = authority.snapshot_now(1).await.unwrap();
    assert_eq!(generation, 0);

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !authority.shutdown_step() {
        assert!(std::time::Instant::now() < deadline, "authority close parked: {}", authority.shutdown_debug_witness());
        semio_framework_async::yield_once().await;
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn document_authority_close_drains_pending_group_commit_for_every_durability_class() {
    for durability in [DurabilityClass::Memory, DurabilityClass::Os, DurabilityClass::Fsync] {
        let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
        let storage = storage().await;
        let reopen_storage = storage.clone();
        let document = document_id().await;
        let reopen_document = document.clone();
        let authority = ArtifactAuthority::spawn(pool.clone(), move || async move { ArtifactEngine::create_retained(document, storage, ArtifactEngineConfig::default(), 0).await.map(Box::new) }, MailboxCapacities::uniform(16)).await.unwrap();
        let batch = CommandBatch::new(vec![envelope("close-drain", &[], "alice", &[("name", serde_json::json!("kept"))]).await]).await.unwrap();
        let receipt = authority.submit(batch, SubmitOptions { durability, ..Default::default() }, 0).await.unwrap();
        assert_eq!(receipt.frontier.head_seq, 1);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !authority.shutdown_step() {
            assert!(std::time::Instant::now() < deadline, "{durability:?} close parked: {}", authority.shutdown_debug_witness());
            semio_framework_async::yield_once().await;
        }
        assert!(authority.handoff.close_error.lock().unwrap().is_none());
        drop(authority);
        let (engine, report) = ArtifactEngine::open_retained(reopen_document, reopen_storage, ArtifactEngineConfig::default(), 1).await.unwrap();
        assert_eq!(report.torn_tail_bytes, 0);
        assert_eq!(engine.frontier.head_seq, 1, "{durability:?} close must make the acknowledged transaction durable");
        let mut engine = engine;
        engine.close().await.unwrap();
        assert_eq!(pool.shutdown(), Ok(()));
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_engine_submit_future_stays_within_the_worker_stack_budget() {
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let batch = CommandBatch::new(vec![envelope("frame-budget", &[], "alice", &[("name", serde_json::json!("x"))]).await]).await.unwrap();
    let submit = engine.submit(batch, SubmitOptions::default(), 0);
    assert!(std::mem::size_of_val(&submit) <= 512 * 1024, "submit future reserves {} B; fixed-capacity owners must stay boxed off the poll stack", std::mem::size_of_val(&submit));
    drop(submit);
    assert!(size_of::<db_wal::WalRecordBatch>() <= 32 && size_of::<db_index::RunEntries>() <= 32);
    engine.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn document_authority_spawn_propagates_a_build_failure_synchronously() {
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)));
    let result = ArtifactAuthority::spawn(
        pool.clone(),
        || async { Err::<Box<ArtifactEngine>, ArtifactEngineOpenRejected>(ArtifactEngineOpenRejected::BeforeWal(DbError::InvalidArgument("boom".to_string()))) },
        MailboxCapacities::uniform(4),
    );
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("artifact authority admitted a rejected builder"),
    };
    assert!(matches!(rejected_engine_open_error(rejected).await, DbError::InvalidArgument(_)));
    pool.shutdown();
}

#[test]
fn artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_backend_token_crc_fault_retire_1024_pages_one_grant_each") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let mut reservation = HistoryReplayReservation::try_new().unwrap();
    for _ in 0..HISTORY_REPLAY_SEGMENT_PAGES {
        reservation.retain_source_page(vec![0]).unwrap();
    }
    let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
    for remaining in (0..HISTORY_REPLAY_SEGMENT_PAGES as usize).rev() {
        assert!(cursor.close_step());
        assert_eq!(cursor.source_page_count, remaining);
    }
    assert!(cursor.close_step(), "fault retirement continues with one result page/range/scalar owner");
    for _ in 0..HISTORY_REPLAY_RESULT_PAGES + 8 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
    let replay = include_str!("../../🦀️.rs");
    for fault in ["history backend page ownership", "history envelope has trailing bytes", "history source finished without authentication"] {
        assert!(replay.contains(fault), "missing retained fault source {fault}");
    }
    assert!(replay.contains("HistoryReplayTransition::FaultRetire"));
    assert!(include_str!("../../../📝️wal/🦀️.rs").contains("wal chain crc or frame length differs"), "history CRC faults are authenticated by the WAL chain");
}

#[test]
fn artifact_history_scratch_result_boundary_plus_one_preserves_exact_owner() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_scratch_result_boundary_plus_one_preserves_exact_owner") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let reservation = HistoryReplayReservation::try_new().unwrap();
    let first_result_owner = reservation.result_pages[0].as_ref().unwrap().as_ptr();
    assert_eq!(reservation.source_pages.len(), HISTORY_REPLAY_SEGMENT_PAGES as usize);
    assert_eq!(reservation.result_pages.len(), HISTORY_REPLAY_RESULT_PAGES);
    assert_eq!(reservation.operation_ids.capacity(), HISTORY_REPLAY_MAX_OPERATION_IDS);
    assert_eq!(reservation.entries.capacity(), HISTORY_REPLAY_MAX_ENTRIES);
    assert_eq!(reservation.scratch.as_ref().unwrap().len(), HISTORY_REPLAY_MAX_FIELD_BYTES);
    assert_eq!(reservation.result_pages[0].as_ref().unwrap().as_ptr(), first_result_owner);
    assert_eq!(reservation.preflight_result_range(HISTORY_REPLAY_RESULT_BYTES - 1, 1).unwrap(), HISTORY_REPLAY_RESULT_BYTES);
    assert!(matches!(reservation.preflight_result_range(HISTORY_REPLAY_RESULT_BYTES, 1), Err(DbError::LimitExceeded("history result byte credit"))));
    assert_eq!(reservation.result_pages[0].as_ref().unwrap().as_ptr(), first_result_owner, "cap+1 rejection must return the exact preadmitted page owner");
    let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
    for _ in 0..HISTORY_REPLAY_RESULT_PAGES + 8 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
}

#[test]
fn artifact_history_reservation_construction_fault_cap_plus_one_and_each_page_retire_one_owner() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_reservation_construction_fault_cap_plus_one_and_each_page_retire_one_owner") {
        return;
    }
    let _guard = history_construction_test_lock();
    for failure_after in [0, 1, HISTORY_REPLAY_RESULT_PAGES / 2, HISTORY_REPLAY_RESULT_PAGES - 1, HISTORY_REPLAY_RESULT_PAGES] {
        let mut fault = HistoryReplayReservation::try_new_with_result_page_failure(failure_after).unwrap_err();
        let error = fault.take_error().expect("exact construction error");
        assert!(matches!(error, DbError::Unavailable(_)));
        let retained_pages = fault.retained_result_page_count();
        assert_eq!(retained_pages, failure_after, "failure must retain every page allocated before its exact boundary");
        let mut previous_pages = retained_pages;
        while !fault.terminal_is_empty() {
            assert!(fault.close_step());
            let current_pages = fault.retained_result_page_count();
            assert!(previous_pages.saturating_sub(current_pages) <= 1, "one construction-fault grant may retire at most one result page");
            previous_pages = current_pages;
        }
    }

    let reservation = HistoryReplayReservation::try_new_with_result_page_failure(HISTORY_REPLAY_RESULT_PAGES + 1).expect("a failure boundary beyond the fixed page cap cannot fabricate an extra owner");
    assert_eq!(reservation.result_pages.len(), HISTORY_REPLAY_RESULT_PAGES);
    let mut cursor = HistoryReplayReservationCloseCursor::new(reservation);
    while !cursor.terminal_is_empty() {
        assert!(cursor.close_step());
    }

    for page_count in 0..=HISTORY_REPLAY_RESULT_PAGES {
        let result_pages = (0..page_count).map(|_| Some(Vec::new())).collect();
        let token = claim_history_replay_reservation_construction().expect("fixed construction slot");
        let close = HistoryReplayReservationCloseCursor {
            source_pages: Some(Vec::new()),
            source_page_count: 0,
            result_pages: Some(result_pages),
            operation_ids: Some(Vec::new()),
            entries: Some(Vec::new()),
            scratch: None,
            retained_operation_bytes: 0,
            retained_result_bytes: 0,
            started: true,
        };
        {
            let mut registry = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let slot = &mut registry.slots[token.slot];
            let mut claimed = slot.cursor.replace(close).expect("construction claim installs its exact empty owner");
            while claimed.close_step() {}
        }
        let mut fault = HistoryReplayReservationConstructionFault { token: Some(token), unregistered_error: None };
        let mut retired_pages = 0;
        while fault.retained_result_page_count() != 0 {
            let before = fault.retained_result_page_count();
            assert!(fault.close_step());
            let after = fault.retained_result_page_count();
            assert_eq!(before - after, 1, "failure after page {page_count} must retire exactly one allocated page per grant");
            retired_pages += 1;
        }
        assert_eq!(retired_pages, page_count);
        while !fault.terminal_is_empty() {
            assert!(fault.close_step());
        }
    }
}

#[test]
fn artifact_history_unchecked_construction_error_and_checked_out_drop_hand_back_exact_pages() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_unchecked_construction_error_and_checked_out_drop_hand_back_exact_pages") {
        return;
    }
    let _guard = history_construction_test_lock();
    let generation = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation;
    let dropped = std::panic::catch_unwind(|| {
        let _ = HistoryReplayReservation::try_new_with_result_page_failure(3);
    });
    assert!(dropped.is_ok(), "unchecked construction error Drop must not panic");
    let fault = take_history_replay_reservation_construction_fault(generation).expect("unchecked error registered exact partial owner");
    assert_eq!(fault.retained_result_page_count(), 3);
    drop(fault);
    let mut resumed = take_history_replay_reservation_construction_fault(generation).expect("checked-out Drop returned exact partial owner");
    assert!(matches!(resumed.take_error(), Some(DbError::Unavailable(_))));
    let mut previous_pages = 3usize;
    while !resumed.terminal_is_empty() {
        assert!(resumed.close_step());
        let pages = resumed.retained_result_page_count();
        assert!(previous_pages.saturating_sub(pages) <= 1);
        previous_pages = pages;
    }
    assert!(take_history_replay_reservation_construction_fault(generation).is_none());
}

#[test]
fn artifact_history_construction_unwind_hands_partial_owner_to_registry_without_bulk_drop() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_construction_unwind_hands_partial_owner_to_registry_without_bulk_drop") {
        return;
    }
    let _guard = history_construction_test_lock();
    let generation = history_replay_reservation_construction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).next_generation;
    let unwind = std::panic::catch_unwind(|| {
        let mut builder = HistoryReplayReservationConstructionBuilder::new().expect("fixed construction authority");
        let mut page = Vec::new();
        page.try_reserve_exact(HISTORY_REPLAY_PAGE_BYTES as usize).expect("fixture page");
        page.resize(HISTORY_REPLAY_PAGE_BYTES as usize, 0);
        assert!(builder
            .edit_cursor(|cursor| cursor.result_pages.as_mut().is_some_and(|owners| {
                owners.push(Some(page));
                true
            }))
            .unwrap_or(false));
        panic!("injected construction unwind");
    });
    assert!(unwind.is_err());
    let mut fault = take_history_replay_reservation_construction_fault(generation).expect("builder Drop registered partial owner");
    assert_eq!(fault.retained_result_page_count(), 1);
    assert!(matches!(fault.take_error(), Some(DbError::Unavailable(_))));
    let before = fault.retained_result_page_count();
    assert!(fault.close_step());
    let after = fault.retained_result_page_count();
    assert_eq!(before - after, 1);
    while !fault.terminal_is_empty() {
        assert!(fault.close_step());
    }
}

#[test]
fn artifact_history_construction_registry_saturation_rejects_before_partial_owner_and_reuses_with_fresh_generation() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_construction_registry_saturation_rejects_before_partial_owner_and_reuses_with_fresh_generation") {
        return;
    }
    let _guard = history_construction_test_lock();
    let mut faults = Vec::with_capacity(HISTORY_REPLAY_CONSTRUCTION_SLOTS);
    for _ in 0..HISTORY_REPLAY_CONSTRUCTION_SLOTS {
        faults.push(HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err());
    }
    let mut rejected = HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err();
    assert!(rejected.generation().is_none());
    assert!(matches!(rejected.take_error(), Some(DbError::Unavailable(_))));
    let released_generation = faults[0].generation().expect("registered generation");
    let mut released = faults.remove(0);
    while !released.terminal_is_empty() {
        assert!(released.close_step());
    }
    let replacement = HistoryReplayReservation::try_new_with_result_page_failure(0).unwrap_err();
    assert_ne!(replacement.generation(), Some(released_generation));
    faults.push(replacement);
    for mut fault in faults {
        while !fault.terminal_is_empty() {
            assert!(fault.close_step());
        }
    }
}

#[test]
fn artifact_history_construction_handback_rejects_stale_duplicate_and_aba_without_owner_overwrite() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_construction_handback_rejects_stale_duplicate_and_aba_without_owner_overwrite") {
        return;
    }
    let _guard = history_construction_test_lock();
    let mut first = HistoryReplayReservation::try_new_with_result_page_failure(1).unwrap_err();
    let first_token = first.token.as_ref().map(|token| (token.slot, token.generation)).expect("first linear construction token");
    while !first.terminal_is_empty() {
        assert!(first.close_step());
    }

    let replacement = HistoryReplayReservation::try_new_with_result_page_failure(1).unwrap_err();
    let replacement_token = replacement.token.as_ref().map(|token| (token.slot, token.generation)).expect("replacement linear construction token");
    assert_eq!(replacement_token.0, first_token.0);
    assert_ne!(replacement_token.1, first_token.1);
    let page_pointer = replacement.retained_result_page_pointer(0).expect("replacement page owner");
    let error_pointer = replacement.retained_error_pointer().expect("replacement error owner");

    let stale = HistoryReplayReservationConstructionToken { slot: first_token.0, generation: first_token.1 };
    assert_eq!(handback_history_replay_reservation_construction(&stale), Err(HistoryReplayReservationConstructionHandbackRejection { slot: first_token.0, generation: first_token.1 }));
    assert_eq!(replacement.retained_result_page_pointer(0), Some(page_pointer));
    assert_eq!(replacement.retained_error_pointer(), Some(error_pointer));

    let out_of_bounds = HistoryReplayReservationConstructionToken { slot: HISTORY_REPLAY_CONSTRUCTION_SLOTS, generation: replacement_token.1 };
    assert_eq!(handback_history_replay_reservation_construction(&out_of_bounds), Err(HistoryReplayReservationConstructionHandbackRejection { slot: HISTORY_REPLAY_CONSTRUCTION_SLOTS, generation: replacement_token.1 }));
    assert_eq!(replacement.retained_result_page_pointer(0), Some(page_pointer));
    assert_eq!(replacement.retained_error_pointer(), Some(error_pointer));

    drop(replacement);
    let duplicate = HistoryReplayReservationConstructionToken { slot: replacement_token.0, generation: replacement_token.1 };
    assert_eq!(handback_history_replay_reservation_construction(&duplicate), Err(HistoryReplayReservationConstructionHandbackRejection { slot: replacement_token.0, generation: replacement_token.1 }));
    let mut resumed = take_history_replay_reservation_construction_fault(replacement_token.1).expect("current generation remained resumable");
    assert_eq!(resumed.retained_result_page_pointer(0), Some(page_pointer));
    assert_eq!(resumed.retained_error_pointer(), Some(error_pointer));
    while !resumed.terminal_is_empty() {
        assert!(resumed.close_step());
    }
}

#[test]
fn artifact_history_fixed_owner_accounting_has_no_capacity_scan() {
    let source = include_str!("../../🦀️.rs");
    let replay = &source[source.find("//#region 🔖️HistoryReplay").unwrap()..source.find("//#endregion 🔖️HistoryReplay").unwrap()];
    for forbidden in [".rposition(", "result_pages.iter()", "source_pages.iter().all", "pages.iter().all", "pages.iter().filter"] {
        assert!(!replay.contains(forbidden), "retained history accounting scanned fixed capacity through {forbidden}");
    }
    for retained in ["source_page_count", "retained_operation_bytes", "retained_result_bytes"] {
        assert!(replay.contains(retained), "retained history accounting omitted {retained}");
    }
}

fn history_phase_fixture_page() -> Result<db_storage::DbIoPages, DbError> {
    let mut writer = db_storage::DbIoPageWriter::try_reserve(1).map_err(|rejected| rejected.into_error())?;
    writer.write_fragment(&[0])?;
    writer.seal().map_err(|rejected| rejected.into_error())
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_panic_at_each_phase_transition_retains_then_fault_retires() {
    if !crate::db_storage::process_isolated_law("db_artifact::tests::artifact_history_panic_at_each_phase_transition_retains_then_fault_retires") {
        return;
    }
    let _history_capacity = history_construction_test_lock();
    let mut engine = ArtifactEngine::create_retained(document_id().await, storage().await, ArtifactEngineConfig::default(), 0).await.unwrap();
    let phases = Vec::from([
        HistoryReplayPhase::Probe,
        HistoryReplayPhase::SegmentLen { index: 0, future: Box::pin(async { Err(DbError::NotFound("phase fixture".to_string())) }) },
        HistoryReplayPhase::PageStart { index: 0, len: 1, offset: 0 },
        HistoryReplayPhase::PageRead { index: 0, len: 1, offset: 0, requested: 1, future: Box::pin(async { history_phase_fixture_page() }) },
        HistoryReplayPhase::PageClose { index: 0, len: 1, offset: 1, retained: history_phase_fixture_page().unwrap() },
        HistoryReplayPhase::Inventory { future: Box::pin(async { Ok(db_storage::DbIoU64List::new()) }) },
        HistoryReplayPhase::Verify { index: 0 },
        HistoryReplayPhase::Frame { index: 0 },
        HistoryReplayPhase::CommittedBody { index: 0 },
        HistoryReplayPhase::Envelope { index: 0, cursor: HistoryEnvelopeCursor { pos: 0, end: 0, field: HistoryEnvelopeField::MutationId, dependencies: 0, target_segments: 0, verb: false, line: false, mutation_id: None } },
        HistoryReplayPhase::CopyMutation { index: 0, range: 0..0, copied: 0, result_start: 0 },
        HistoryReplayPhase::Frontier { index: 0, cursor: HistoryFrontierCursor { pos: 0, end: 0, field: HistoryFrontierField::Document, head_seq: 0, commit_seq: 0, chain_hash: [0; 32] } },
        HistoryReplayPhase::Publish { index: 0, head_seq: 0, commit_seq: 0, chain_hash: [0; 32], epoch: 0 },
        HistoryReplayPhase::Retire,
        HistoryReplayPhase::InventoryClose,
        HistoryReplayPhase::FinalizeSuccess,
    ]);
    let waker = std::task::Waker::from(StdArc::new(HistoryReplayTestWake));
    let mut context = std::task::Context::from_waker(&waker);
    for phase in phases {
        let mut replay = engine.history_replay(1, StdArc::new(std::sync::atomic::AtomicBool::new(false)), HistoryReplayReservation::try_new().unwrap());
        replay.phase = Some(phase);
        replay.transition = HistoryReplayTransition::InProgress;
        replay.panic_before_transition_commit = true;
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Pin::new(&mut replay).poll(&mut context))).is_err());
        assert!(replay.phase.is_some(), "panic must retain the exact active phase owner");
        assert!(matches!(replay.transition, HistoryReplayTransition::InProgress));
        replay.request_close(DbError::Internal("phase panic fixture".to_string()));
        let mut terminal = false;
        for _ in 0..50_000 {
            if Pin::new(&mut replay).poll(&mut context).is_ready() {
                terminal = true;
                break;
            }
        }
        assert!(terminal);
        assert!(replay.terminal_is_empty());
    }
    engine.wal.close().await.unwrap();
    while engine.state.values.close_step().unwrap() {
        semio_framework_async::yield_once().await;
    }
}
//#endregion 🔖️Actor

/// 🪪️ LAW: an undo belongs to its author on the hub's event log too (`ArtifactEngine::submit`, the twin of
/// `protocol::fold_history`'s refusal, `🔗️causal/🧫️fixtures/🗄️durable-collaborative-redo-v1`): a `Revert` or `Reinstate`
/// naming any operation another actor wrote is refused with the one `history.foreign-transition` message whatever the merge
/// policy and leaves the log untouched; the author's own undo/redo, one naming an operation of the same batch, and one naming
/// an operation the log does not know are admitted (measured live on hub 7800 p33: B's crafted `Revert` of A's note edit was
/// persisted and only the replicas refused it).
#[semio_framework_async_macros::async_test]
async fn a_history_transition_naming_another_actors_operation_is_refused_before_the_log() {
    let document = protocol::ArtifactId("doc-1".to_string());
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage::MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.expect("memory storage")));
    let mut engine = ArtifactEngine::create(document.clone(), storage, ArtifactEngineConfig::default(), 0).expect("engine");
    let write = |id: &str, actor: &str, at: u64| protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: vec![id.to_string()],
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("fixture.opaque.v1".into()), payload: id.as_bytes().to_vec() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("fixture.opaque.v1".into()), payload: Vec::new() },
        timestamp: protocol::HybridLogicalTimestamp::new(at, 0),
        transaction: None, verb: None, line: None,
    };
    let transition = |revert: bool, names: &[&str], actor: &str, at: u64| {
        let mutation_ids = names.iter().map(|name| protocol::MutationId((*name).to_string())).collect();
        let transition = if revert { protocol::HistoryTransition::Revert { mutation_ids } } else { protocol::HistoryTransition::Reinstate { mutation_ids } };
        protocol::history_transition_envelope(&transition, &document, &protocol::ActorId(actor.to_string()), Vec::new(), protocol::HybridLogicalTimestamp::new(at, 0))
    };
    let cases: Vec<(&str, Vec<protocol::MutationEnvelope>, bool)> = vec![
        ("A writes", vec![write("a1", "actor-a", 1)], true),
        ("B writes", vec![write("b1", "actor-b", 2)], true),
        ("B reverts A's operation", vec![transition(true, &["a1"], "actor-b", 3)], false),
        ("B reinstates A's operation", vec![transition(false, &["a1"], "actor-b", 4)], false),
        ("B reverts its own and A's operation", vec![transition(true, &["b1", "a1"], "actor-b", 5)], false),
        ("B reverts its own operation", vec![transition(true, &["b1"], "actor-b", 6)], true),
        ("B reinstates its own operation", vec![transition(false, &["b1"], "actor-b", 7)], true),
        ("A reverts its own operation", vec![transition(true, &["a1"], "actor-a", 8)], true),
        ("C writes and reverts in one batch", vec![write("c1", "actor-c", 9), transition(true, &["c1"], "actor-c", 10)], true),
        ("B reverts an operation the log does not know", vec![transition(true, &["ghost"], "actor-b", 11)], true),
    ];
    for (index, (label, envelopes, admitted)) in cases.into_iter().enumerate() {
        for policy in [protocol::MergePolicy::LaissezFaire, protocol::MergePolicy::Vigilant] {
            let before = engine.frontier().await;
            let batch = CommandBatch::new(envelopes.clone()).await.expect("batch");
            match engine.submit(batch, SubmitOptions { policy, ..Default::default() }, index as u64 + 1).await {
                Ok(_) => assert!(admitted, "{label} ({policy:?}) was admitted"),
                Err(DbError::Rejected { messages, .. }) => {
                    assert!(!admitted, "{label} ({policy:?}) was refused: {messages:?}");
                    assert_eq!(messages.iter().map(|message| message.code.0.as_str()).collect::<Vec<_>>(), vec![FOREIGN_HISTORY_TRANSITION_CODE], "{label}");
                    assert_eq!(engine.frontier().await.head_seq, before.head_seq, "{label}: a refused transition leaves the log untouched");
                }
                Err(other) => panic!("{label} ({policy:?}): {other:?}"),
            }
            if admitted {
                break;
            }
        }
    }
}

```

## ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! generation2d <- json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see the paired export
//! leaf's doc comment and w5b-close-report.md): `JsonSnapshot::to_serde_value`/stdio's own real
//! `parse_json_text` do the structural conversion — no hand-rolled bridge needed here.
use crate::Generation2dSnapshot;
use semio_s_artifact_stdio_json::schema::snapshot::{parse_json_text, JsonSnapshot};
use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let snap = <Generation2dSnapshot as protocol::FromValue>::from_value(protocol::DslValue::from(from.to_serde_value())).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("generation2d<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;

    Ok(snap)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text).map_err(|e| semio_framework_diagnostic::TextError::new(e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}

```

## ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs

```rust
//! 🚪️ IO s.lowpoly (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.obj", "stdio.ply", "stdio.stl", "stdio.txt"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl", "stdio.txt"]
}
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::LowpolyAnalyzer;
    use crate::LowpolySnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PLY: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct LowpolyComposerComposition;

    impl ArtifactComposition for LowpolyComposerComposition {
        type Snapshot = LowpolySnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_JSON, DEP_OBJ, DEP_PLY, DEP_STL, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = LowpolyAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PLY {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "LowpolyComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v1::subsets::any::schema::LowpolyBuilder as LowpolyAnyBuilder;
    use crate::standards::v1::subsets::any::schema::LowpolyComposer as LowpolyAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ArtifactBuilder, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource, IoConfidence, IoPayload, StandardId, SubsetId};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const LOWPOLY_DIALECT: Dialect = Dialect { artifact_kind: "s.lowpoly.lowpoly", standard: StandardId("1"), subset: SubsetId("*") };
    const LOWPOLY_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::LowpolySnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == LOWPOLY_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => LowpolyAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => LowpolyAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "LowpolyComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == LOWPOLY_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "LowpolyComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_LAS_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_las(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::las::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_LAS_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PLY_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    fn compose_export_ply(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::ply::v1_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PLY_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    fn compose_export_stl(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::stl::v_ascii::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_STL_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_GLTF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    fn compose_export_gltf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    fn compose_export_obj(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::obj::v3_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_OBJ_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    fn compose_export_txt(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_TXT_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<LowpolyAnyComposer>(),
                    ComposerEntry { writes: EXPORT_LAS_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_las },
                    ComposerEntry { writes: EXPORT_PLY_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_ply },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_dwg },
                    ComposerEntry { writes: EXPORT_STL_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_stl },
                    ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_gltf },
                    ComposerEntry { writes: EXPORT_OBJ_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_obj },
                    ComposerEntry { writes: EXPORT_TXT_DIALECT, reads: &[LOWPOLY_DIALECT], compose: compose_export_txt },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🕸️MeshGeometry
/// 🕸️ Shared real-geometry plumbing for the OBJ/STL/PLY leaves (ticket
/// 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS): building `LowpolyObject`s from polygon
/// soups on import, and flattening each object's persisted `mesh_content` into world-space polygons
/// (scale → Euler-degree XYZ rotation, the editor's own quaternion convention → translation) on
/// export.
pub mod mesh_geometry {
    use crate::{mesh_child_handle, LowpolyObject, LowpolyPaintLayer, LowpolySnapshot, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
    use semio_framework_3d::mesh::{FaceId, HalfedgeMesh, VertexId};

    /// 🧮 Document object ceiling; more imported parts than this are merged into one object.
    pub const MAX_IMPORTED_OBJECTS: usize = 64;

    /// 🧩 One named polygon soup: positions plus 0-based n-gon index lists into them.
    #[derive(Clone, Debug, Default)]
    pub struct PolygonPart {
        pub name: String,
        pub positions: Vec<[f32; 3]>,
        pub faces: Vec<Vec<u32>>,
    }

    pub fn text_error(message: impl Into<String>) -> semio_framework_diagnostic::TextError {
        semio_framework_diagnostic::TextError::new(message.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
    }

    /// 🧹 Drops consecutive duplicate indices (incl. wrap-around); `None` when fewer than 3 remain.
    pub fn clean_polygon(indices: &[u32]) -> Option<Vec<u32>> {
        let mut out: Vec<u32> = Vec::with_capacity(indices.len());
        for &index in indices {
            if out.last() != Some(&index) {
                out.push(index);
            }
        }
        while out.len() > 1 && out.first() == out.last() {
            out.pop();
        }
        (out.len() >= 3).then_some(out)
    }

    /// 🔁 Remaps a subset of global polygons (global vertex indices) into a compact local part.
    pub fn compact_part(name: &str, global_positions: &[[f32; 3]], polygons: &[Vec<u32>]) -> Result<PolygonPart, String> {
        let mut remap: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        let mut part = PolygonPart { name: name.to_string(), ..Default::default() };
        for polygon in polygons {
            let mut local = Vec::with_capacity(polygon.len());
            for &global in polygon {
                let position = *global_positions.get(global as usize).ok_or_else(|| format!("vertex index {} out of range ({} vertices)", global + 1, global_positions.len()))?;
                let index = *remap.entry(global).or_insert_with(|| {
                    part.positions.push(position);
                    (part.positions.len() - 1) as u32
                });
                local.push(index);
            }
            if let Some(clean) = clean_polygon(&local) {
                part.faces.push(clean);
            }
        }
        Ok(part)
    }

    /// 🧱️ Merges every part into one (used past the 64-object ceiling).
    pub fn merge_parts(parts: Vec<PolygonPart>, name: &str) -> PolygonPart {
        let mut merged = PolygonPart { name: name.to_string(), ..Default::default() };
        for part in parts {
            let offset = merged.positions.len() as u32;
            merged.positions.extend(part.positions);
            merged.faces.extend(part.faces.into_iter().map(|face| face.into_iter().map(|i| i + offset).collect()));
        }
        merged
    }

    /// 📥 Builds a lowpoly document (`obj-1`, `obj-2`, … identity transforms) from polygon parts.
    /// Empty parts are skipped; no faces at all is a loud error, never an empty document.
    pub fn snapshot_from_parts(format: &str, parts: Vec<PolygonPart>) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
        let mut parts: Vec<PolygonPart> = parts.into_iter().filter(|part| !part.faces.is_empty()).collect();
        if parts.is_empty() {
            return Err(text_error(format!("{format}->lowpoly: the file contains no polygon faces to import")));
        }
        if parts.len() > MAX_IMPORTED_OBJECTS {
            let name = parts[0].name.clone();
            parts = vec![merge_parts(parts, &name)];
        }
        let mut objects = Vec::with_capacity(parts.len());
        for (index, part) in parts.into_iter().enumerate() {
            let id = format!("obj-{}", index + 1);
            let mesh = HalfedgeMesh::from_faces(&part.positions, &part.faces).map_err(|e| text_error(format!("{format}->lowpoly: object '{}' is not a valid polygon mesh: {e:?}", part.name)))?;
            let mesh_content = mesh.to_json().map_err(|e| text_error(format!("{format}->lowpoly: mesh json: {e:?}")))?;
            let name = if part.name.trim().is_empty() { format!("Object {}", index + 1) } else { part.name };
            objects.push(LowpolyObject {
                mesh: Some(mesh_child_handle(&id, &mesh_content)),
                id,
                name,
                transform: LowpolyTransform::default(),
                smooth_shading: false,
                paint_layers: vec![LowpolyPaintLayer::new("Base")],
                mesh_content,
            });
        }
        Ok(LowpolySnapshot { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects })
    }

    /// 🔄 Editor convention (`✏️editor/🧭️view::euler_degrees_to_quaternion`): XYZ Euler degrees → `[x,y,z,w]`.
    pub fn euler_degrees_to_quaternion(rotation: [f32; 3]) -> [f64; 4] {
        let to_rad = std::f64::consts::PI / 180.0;
        let (sx, cx) = (f64::from(rotation[0]) * to_rad * 0.5).sin_cos();
        let (sy, cy) = (f64::from(rotation[1]) * to_rad * 0.5).sin_cos();
        let (sz, cz) = (f64::from(rotation[2]) * to_rad * 0.5).sin_cos();
        [sx * cy * cz + cx * sy * sz, cx * sy * cz - sx * cy * sz, cx * cy * sz + sx * sy * cz, cx * cy * cz - sx * sy * sz]
    }

    pub fn rotate(q: [f64; 4], v: [f64; 3]) -> [f64; 3] {
        let [x, y, z, w] = q;
        let t = [2.0 * (y * v[2] - z * v[1]), 2.0 * (z * v[0] - x * v[2]), 2.0 * (x * v[1] - y * v[0])];
        [v[0] + w * t[0] + (y * t[2] - z * t[1]), v[1] + w * t[1] + (z * t[0] - x * t[2]), v[2] + w * t[2] + (x * t[1] - y * t[0])]
    }

    /// 🌍 Local position → world: scale, then rotation, then translation.
    pub fn apply_transform(transform: &LowpolyTransform, local: [f32; 3]) -> [f64; 3] {
        let scaled = [f64::from(local[0] * transform.scale[0]), f64::from(local[1] * transform.scale[1]), f64::from(local[2] * transform.scale[2])];
        let rotated = rotate(euler_degrees_to_quaternion(transform.rotation), scaled);
        [rotated[0] + f64::from(transform.position[0]), rotated[1] + f64::from(transform.position[1]), rotated[2] + f64::from(transform.position[2])]
    }

    /// 🧩 One exported object: world-space positions and 0-based n-gon faces.
    #[derive(Clone, Debug, Default)]
    pub struct WorldPart {
        pub name: String,
        pub positions: Vec<[f64; 3]>,
        pub faces: Vec<Vec<u32>>,
    }

    /// 📤 Every object with non-empty `mesh_content`, transformed into world space.
    pub fn world_parts(format: &str, snapshot: &LowpolySnapshot) -> Result<Vec<WorldPart>, semio_framework_diagnostic::TextError> {
        let mut parts = Vec::new();
        for object in &snapshot.objects {
            if object.mesh_content.trim().is_empty() {
                continue;
            }
            let mesh = HalfedgeMesh::from_json(&object.mesh_content).map_err(|e| text_error(format!("lowpoly->{format}: object '{}' has unreadable mesh content: {e:?}", object.name)))?;
            let mut part = WorldPart { name: object.name.clone(), ..Default::default() };
            for vertex in 0..mesh.vertex_count() {
                let local = mesh.vertex_position(VertexId(vertex as u32)).map_err(|e| text_error(format!("lowpoly->{format}: vertex {vertex}: {e:?}")))?;
                part.positions.push(apply_transform(&object.transform, local.0));
            }
            for face in 0..mesh.face_count() {
                let ids = mesh.face_vertex_ids(FaceId(face as u32)).map_err(|e| text_error(format!("lowpoly->{format}: face {face}: {e:?}")))?;
                if ids.len() >= 3 {
                    part.faces.push(ids.into_iter().map(|id| id.0).collect());
                }
            }
            parts.push(part);
        }
        Ok(parts)
    }

    /// 📐 Unit normal of triangle `(a, b, c)` (zero for a degenerate triangle).
    pub fn triangle_normal(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> [f64; 3] {
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if length < 1e-12 {
            [0.0; 3]
        } else {
            [n[0] / length, n[1] / length, n[2] / length]
        }
    }
}
//#endregion 🕸️MeshGeometry

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ HtmlSnapshot schema — own `HtmlNode` recursive tree model + a from-scratch WHATWG-inspired
//! HTML5 tokenizer/parser and serializer. HTML is NOT XML: no shared types with `📰️xml`/`🎨️svg`
//! (only the general "recursive node tree" *structural pattern* is borrowed, per the ticket brief)
//! — own element/text/comment/raw-text node kinds, own void-element handling, own (deliberately
//! small) entity table.
//!
//! ## Honest boundary (documented per the ticket brief)
//! `✳️any` accepts **well-formed HTML5 documents only**. Full HTML5 "error recovery" parsing (the
//! WHATWG parsing algorithm's tree-construction insertion modes, implied end tags, the adoption
//! agency algorithm, foster parenting, etc.) is genuinely out of scope for a from-scratch
//! implementation — malformed/tag-soup markup is rejected with a `TextError`, never silently
//! "fixed up". A second, small honest boundary: only the five XML-equivalent named character
//! references (`&amp; &lt; &gt; &quot; &apos;`) plus numeric character references (`&#DD;` /
//! `&#xHH;`) are decoded — the full WHATWG named-character-reference table (~2200 entries, e.g.
//! `&nbsp;`) is not reproduced here; any other `&name;`-shaped sequence is passed through literally
//! as raw text (never an error, never silently corrupted).
//!
//! Out of scope is not the same as free: for the WELL-FORMED documents this subset does accept, the
//! tree it builds must be the tree every other HTML5 implementation builds, because the mutation
//! vocabulary addresses nodes by child index. The two normative placements a purely literal reader
//! gets wrong are applied by [`normalize_html_root_whitespace`] — see its own doc comment.

use semio_framework_diagnostic::TextSpan;
use framework_schema::ArtifactSchema;
use semio_framework_diagnostic::TextError;

//#region 🔖️Ids
pub const STDIO_HTML_DOCUMENT_SCHEMA: &str = "stdio.html";
//#endregion 🔖️Ids

//#region 🔖️Model
/// 🏷️ One element attribute. `value: None` is a valueless boolean attribute (`<p disabled>`),
/// distinct from an attribute that isn't present at all.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct HtmlAttr {
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl HtmlAttr {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: Some(value.into()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn boolean(name: impl Into<String>) -> Self {
        Self { name: name.into(), value: None }
    }
}

/// 🍃️ Which RAWTEXT element a [`HtmlNode::RawText`] node's content belongs to — `<script>` and
/// `<style>` are the only two RAWTEXT-content-model elements this subset models (HTML5 also gives
/// `<textarea>`/`<title>` a related-but-distinct RCDATA content model, out of scope here: their
/// content is parsed as plain `Text`, entity-decoded like everywhere else).
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum RawTextKind {
    Script,
    Style,
}

impl RawTextKind {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tag_name(self) -> &'static str {
        match self {
            RawTextKind::Script => "script",
            RawTextKind::Style => "style",
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn from_tag_name(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case("script") {
            Some(RawTextKind::Script)
        } else if name.eq_ignore_ascii_case("style") {
            Some(RawTextKind::Style)
        } else {
            None
        }
    }
}

/// 🌳 A node in the HTML5 document tree.
// NOTE: every non-unit variant MUST be a struct variant (named field), never a bare tuple variant
// -- serde's internally-tagged (`tag = "kind"`) representation can only merge the tag into
// map-shaped content; a tuple variant wrapping a non-map type compiles but fails at RUNTIME
// serialization ("can only flatten structs and maps"). Same real finding already on record for
// `stdio.json`'s `JsonValue` (see that file's identical NOTE) -- `Text`/`Comment` are therefore
// `{ text: String }` struct variants, not the bare-tuple `Text(String)`/`Comment(String)` shorthand
// used in the ticket brief's conceptual shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum HtmlNode {
    Element {
        name: String,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        attributes: Vec<HtmlAttr>,
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        children: Vec<HtmlNode>,
    },
    Text {
        text: String,
    },
    Comment {
        text: String,
    },
    RawText {
        parent_kind: RawTextKind,
        text: String,
    },
}

impl HtmlNode {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn element(name: impl Into<String>) -> Self {
        HtmlNode::Element { name: name.into(), attributes: Vec::new(), children: Vec::new() }
    }
}

/// 🧭️ Path from the document root to a node: chain of child indices at each nesting level. `[]`
/// addresses the root itself.
pub type NodePath = Vec<usize>;

/// 📸️ Persisted `stdio.html` snapshot: `doctype` (raw content between `<!` and `>`, e.g.
/// `"DOCTYPE html"`, `None` if the document has no doctype declaration) + the recursive `root`
/// element tree.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.html")]
pub struct HtmlSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<String>,
    #[state(artifact)]
    pub root: HtmlNode,
}

impl Default for HtmlSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype: Some("DOCTYPE html".into()), root: HtmlNode::element("html") }
    }
}
//#endregion 🔖️Model

//#region 🔖️VoidElements
/// 🚪️ The HTML5/WHATWG void-element set (14 elements) — these never have a closing tag and never
/// carry children; the encoder must not emit `</tag>` (or self-close `/>`) for them.
const VOID_ELEMENTS: &[&str] = &["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn is_void_element(name: &str) -> bool {
    VOID_ELEMENTS.iter().any(|v| v.eq_ignore_ascii_case(name))
}
//#endregion 🔖️VoidElements

//#region 🔖️Entities
/// 🔓️ Decodes the small honest entity subset (see module doc comment) inside text/attribute
/// content. Any `&`-sequence outside that subset (malformed, or a real named reference like
/// `&nbsp;` this subset doesn't model) is passed through byte-for-byte, never dropped or errored.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_entities(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut out = String::with_capacity(raw.len());
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            // 🧭️ Advance by one CHAR (not byte) to stay on UTF-8 boundaries.
            let ch_len = utf8_char_len(bytes[i]);
            out.push_str(&raw[i..i + ch_len]);
            i += ch_len;
            continue;
        }
        if let Some((decoded, consumed)) = try_decode_entity(&raw[i..]) {
            out.push(decoded);
            i += consumed;
        } else {
            out.push('&');
            i += 1;
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn utf8_char_len(lead: u8) -> usize {
    if lead >= 0xF0 {
        4
    } else if lead >= 0xE0 {
        3
    } else if lead >= 0xC0 {
        2
    } else {
        1
    }
}

/// 🔓️ Attempts to decode ONE entity starting at `s[0] == '&'`. Returns `(char, bytes_consumed)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn try_decode_entity(s: &str) -> Option<(char, usize)> {
    let named: &[(&str, char)] = &[("&amp;", '&'), ("&lt;", '<'), ("&gt;", '>'), ("&quot;", '"'), ("&apos;", '\'')];
    for (lit, ch) in named {
        if s.starts_with(lit) {
            return Some((*ch, lit.len()));
        }
    }
    if let Some(rest) = s.strip_prefix("&#x").or_else(|| s.strip_prefix("&#X")) {
        let hex: String = rest.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
        if !hex.is_empty() && rest[hex.len()..].starts_with(';') {
            let code = u32::from_str_radix(&hex, 16).ok()?;
            let ch = char::from_u32(code)?;
            return Some((ch, 3 + hex.len() + 1));
        }
        return None;
    }
    if let Some(rest) = s.strip_prefix("&#") {
        let dec: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !dec.is_empty() && rest[dec.len()..].starts_with(';') {
            let code: u32 = dec.parse().ok()?;
            let ch = char::from_u32(code)?;
            return Some((ch, 2 + dec.len() + 1));
        }
    }
    None
}

/// 🔒️ Escapes text-node content for re-serialization (`&`, `<`, `>`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            c => out.push(c),
        }
    }
    out
}

/// 🔒️ Escapes an attribute value for re-serialization. Attribute values are ALWAYS re-emitted
/// double-quoted (see module doc comment on quote-style normalization), so only `&` and `"` need
/// escaping.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_attr_value(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}
//#endregion 🔖️Entities

//#region 🔖️Parser
/// 🚶️ Byte-cursor recursive-descent parser with 1-based line/column tracking for `TextError`
/// spans, same shape as `stdio.json`'s `Parser`. Operates on the UTF-8 byte slice of a valid
/// `&str` — multi-byte characters are never mistaken for an ASCII delimiter (continuation bytes
/// are always `0x80..=0xBF`), so every slice point found by scanning for `<`/`>`/`&`/quotes/etc.
/// is a valid UTF-8 boundary.
struct Parser<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: u32,
    col: u32,
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(text: &'a str) -> Self {
        Self { src: text, bytes: text.as_bytes(), pos: 0, line: 1, col: 1 }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn advance(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.pos += 1;
        if byte == b'\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(byte)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn span(&self) -> TextSpan {
        TextSpan::at(self.line, self.col)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn err(&self, message: impl Into<String>) -> TextError {
        TextError::new(message, self.span())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r' | 0x0C)) {
            self.advance();
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn expect(&mut self, byte: u8) -> Result<(), TextError> {
        match self.peek() {
            Some(b) if b == byte => {
                self.advance();
                Ok(())
            }
            Some(other) => Err(self.err(format!("expected '{}', found '{}'", byte as char, other as char))),
            None => Err(self.err(format!("expected '{}', found end of input", byte as char))),
        }
    }

    /// 🔎 Literal, case-sensitive prefix check at the current position (no consumption).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek_str(&self, lit: &str) -> bool {
        self.src[self.pos..].starts_with(lit)
    }

    /// 🔎 ASCII case-insensitive prefix check at the current position (no consumption). Compares
    /// raw BYTES (not a `&str` slice) — `pos + lit.len()` is an arithmetically-computed end offset
    /// with no guarantee of landing on a UTF-8 char boundary, and `&str` slicing at a non-boundary
    /// offset panics where `&[u8]` slicing does not (same rationale as
    /// `read_raw_text_until_close`'s probe).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn peek_str_ci(&self, lit: &str) -> bool {
        let end = self.pos + lit.len();
        end <= self.bytes.len() && self.bytes[self.pos..end].eq_ignore_ascii_case(lit.as_bytes())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.src[start..end]
    }

    /// 🔎 Whether the byte at `pos + offset` is a "tag boundary" (whitespace, `>`, `/`) — used to
    /// avoid matching `</script2>` when looking for `</script>`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_boundary_at(&self, offset: usize) -> bool {
        match self.peek_at(offset) {
            None => true,
            Some(b) => matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | b'>' | b'/'),
        }
    }
}

/// 🔤️ Valid HTML5 tag-name / attribute-name continuation character (permissive superset: ASCII
/// alnum plus the common `-`/`:`/`_`/`.` seen in custom elements and `data-*`/namespaced attrs).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b':' | b'.')
}

impl<'a> Parser<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_name(&mut self) -> Result<String, TextError> {
        let start = self.pos;
        while matches!(self.peek(), Some(b) if is_name_byte(b)) {
            self.advance();
        }
        if self.pos == start {
            return Err(self.err("expected a name"));
        }
        Ok(self.slice(start, self.pos).to_string())
    }

    /// 🏷️ `name` / `name=value` / `name="value"` / `name='value'`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_attribute(&mut self) -> Result<HtmlAttr, TextError> {
        let name = self.parse_name()?;
        let save = self.pos;
        self.skip_ws();
        if self.peek() == Some(b'=') {
            self.advance();
            self.skip_ws();
            let raw = match self.peek() {
                Some(q @ (b'"' | b'\'')) => {
                    self.advance();
                    let start = self.pos;
                    while self.peek() != Some(q) {
                        if self.advance().is_none() {
                            return Err(self.err(format!("unterminated attribute value for '{name}'")));
                        }
                    }
                    let raw = self.slice(start, self.pos).to_string();
                    self.advance(); // closing quote
                    raw
                }
                Some(_) => {
                    let start = self.pos;
                    while matches!(self.peek(), Some(b) if !matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C | b'>')) {
                        self.advance();
                    }
                    self.slice(start, self.pos).to_string()
                }
                None => return Err(self.err(format!("unterminated tag: expected value for attribute '{name}'"))),
            };
            Ok(HtmlAttr { name, value: Some(decode_entities(&raw)) })
        } else {
            // ↩️ No '=' -- rewind past any whitespace we speculatively skipped; the caller's own
            // loop re-does `skip_ws()` before deciding what comes next.
            self.pos = save;
            Ok(HtmlAttr { name, value: None })
        }
    }

    /// 🏗️ `<name attr...>` or `<name attr.../>`, returning `(name, attributes, self_closed)`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_start_tag(&mut self) -> Result<(String, Vec<HtmlAttr>, bool), TextError> {
        self.expect(b'<')?;
        let name = self.parse_name()?;
        let mut attributes = Vec::new();
        let self_closed = loop {
            self.skip_ws();
            match self.peek() {
                Some(b'>') => {
                    self.advance();
                    break false;
                }
                Some(b'/') => {
                    self.advance();
                    self.expect(b'>').map_err(|_| self.err(format!("expected '>' after '/' in tag '<{name}'")))?;
                    break true;
                }
                Some(_) => attributes.push(self.parse_attribute()?),
                None => return Err(self.err(format!("unterminated start tag '<{name}'"))),
            }
        };
        Ok((name, attributes, self_closed))
    }

    /// 🚪️ `</name>` (whitespace before `>` tolerated). Returns the closing tag's own name (NOT
    /// forced to match the caller's expectation — the caller compares case-insensitively).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_end_tag(&mut self) -> Result<String, TextError> {
        self.expect(b'<')?;
        self.expect(b'/')?;
        let name = self.parse_name()?;
        self.skip_ws();
        self.expect(b'>').map_err(|_| self.err(format!("expected '>' to close end tag '</{name}'")))?;
        Ok(name)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_comment(&mut self) -> Result<String, TextError> {
        // 🎯 Assumes "<!--" already consumed by the caller.
        let start = self.pos;
        while !self.peek_str("-->") {
            if self.advance().is_none() {
                return Err(self.err("unterminated comment, expected '-->'"));
            }
        }
        let content = self.slice(start, self.pos).to_string();
        self.advance();
        self.advance();
        self.advance();
        Ok(content)
    }

    /// 📄️ Reads RAWTEXT content verbatim (no entity decoding, no nested-markup parsing — matches
    /// HTML5's RAWTEXT content model for `<script>`/`<style>`) up to (not including) the matching
    /// case-insensitive `</tag` close-tag boundary. Compares raw BYTES (not `&str` slices) for the
    /// probe — `probe_end` is an arithmetically-computed offset (`pos + 2 + tag.len()`) with no
    /// guarantee of landing on a UTF-8 char boundary when the source contains multi-byte content
    /// right after a stray `</`, and `&str` slicing at a non-boundary offset panics; `&[u8]`
    /// slicing never does, so byte comparison keeps this parser panic-free on adversarial input
    /// (falls through to "not a match, keep scanning" instead of crashing).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_raw_text_until_close(&mut self, tag: &str) -> Result<String, TextError> {
        let start = self.pos;
        loop {
            if self.peek() == Some(b'<') && self.peek_at(1) == Some(b'/') {
                let probe_start = self.pos + 2;
                let probe_end = probe_start + tag.len();
                if probe_end <= self.bytes.len() && self.bytes[probe_start..probe_end].eq_ignore_ascii_case(tag.as_bytes()) {
                    let saved_offset = probe_end - self.pos;
                    if self.is_boundary_at(saved_offset) {
                        break;
                    }
                }
            }
            if self.advance().is_none() {
                return Err(self.err(format!("unterminated raw text content, expected '</{tag}>'")));
            }
        }
        Ok(self.slice(start, self.pos).to_string())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn read_text_until_lt(&mut self) -> String {
        let start = self.pos;
        while !matches!(self.peek(), Some(b'<') | None) {
            self.advance();
        }
        decode_entities(self.slice(start, self.pos))
    }

    /// 🌳 Parses one element and its full subtree, starting at `<`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn parse_element(&mut self) -> Result<HtmlNode, TextError> {
        let open_span = self.span();
        let (name, attributes, self_closed) = self.parse_start_tag()?;

        if is_void_element(&name) {
            return Ok(HtmlNode::Element { name, attributes, children: Vec::new() });
        }
        if self_closed {
            return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("'/>' self-closing syntax is only supported on void elements, found on non-void '<{name}/>'"), open_span));
        }

        if let Some(kind) = RawTextKind::from_tag_name(&name) {
            let raw = self.read_raw_text_until_close(&name)?;
            let close_name = self.parse_end_tag()?;
            if !close_name.eq_ignore_ascii_case(&name) {
                return Err(self.err(format!("mismatched close tag: expected '</{name}>', found '</{close_name}>'")));
            }
            let children = if raw.is_empty() { Vec::new() } else { vec![HtmlNode::RawText { parent_kind: kind, text: raw }] };
            return Ok(HtmlNode::Element { name, attributes, children });
        }

        let mut children = Vec::new();
        loop {
            match self.peek() {
                None => return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unterminated element '<{name}>', expected '</{name}>'"), open_span)),
                Some(b'<') => {
                    if self.peek_str("<!--") {
                        self.pos += 4;
                        self.col += 4;
                        children.push(HtmlNode::Comment { text: self.read_comment()? });
                    } else if self.peek_at(1) == Some(b'/') {
                        let close_name = self.parse_end_tag()?;
                        if !close_name.eq_ignore_ascii_case(&name) {
                            return Err(self.err(format!("mismatched close tag: expected '</{name}>', found '</{close_name}>'")));
                        }
                        break;
                    } else {
                        children.push(self.parse_element()?);
                    }
                }
                Some(_) => children.push(HtmlNode::Text { text: self.read_text_until_lt() }),
            }
        }
        Ok(HtmlNode::Element { name, attributes, children })
    }
}

/// 🧽 Whitespace-only character data, per the WHATWG definition (TAB, LF, FF, CR, SPACE).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_whitespace_text(node: &HtmlNode) -> bool {
    matches!(node, HtmlNode::Text { text } if text.chars().all(|c| matches!(c, '\t' | '\n' | '\u{000C}' | '\r' | ' ')))
}

/// 🧹 The three places where a well-formed HTML5 document's SOURCE whitespace is not where the
/// document's TREE carries it. All three are normative WHATWG tree construction, not error recovery
/// (which stays out of scope per the module header): §13.2.6.4.3 "before head" *ignores*
/// whitespace-only character tokens, so `<html>\n  <head>` has no text node before `<head>` in any
/// conformant DOM; §13.2.6.4.20 "after body" and §13.2.6.4.22 "after after body" both process them
/// with the "in body" rules, whose insertion point is still the `body` element (`</body>` and
/// `</html>` switch the insertion mode but never pop the stack), so the newlines in
/// `</body>\n</html>\n` both belong to `body`, merged onto its last text node by the tree
/// construction's own "append to existing text node" rule — not to `html`, and not discarded.
/// `after_root` carries the document-level tail the caller consumed after `</html>`.
///
/// Reading them literally is what this parser used to do, and it put every path index inside `<html>`
/// one place off from every other HTML5 implementation's: `[2]` addressed a whitespace text node here
/// where `html5ever`, every browser, and this subset's own mutation oracle all address `<body>`.
/// Found by `../../../../../🧪️tests/🌐️mutate-html-5`'s parity phase the first time it ran
/// (ticket 26/08/23/END-TO-END-TESTING-REFACTOR), where all seven path-addressed kinds were refused
/// with `mutation.apply.conflicting-target` — "element diff targets a non-element node".
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn normalize_html_root_whitespace(root: &mut HtmlNode, after_root: &str) {
    let HtmlNode::Element { name, children, .. } = root else { return };
    if !name.eq_ignore_ascii_case("html") {
        return;
    }
    let Some(first_element) = children.iter().position(|child| matches!(child, HtmlNode::Element { .. })) else { return };
    let mut seen = 0;
    children.retain(|child| {
        let keep = seen >= first_element || !is_whitespace_text(child);
        seen += 1;
        keep
    });
    let Some(body) = children.iter().position(|child| matches!(child, HtmlNode::Element { name, .. } if name.eq_ignore_ascii_case("body"))) else { return };
    let mut trailing = String::new();
    let mut seen = 0;
    children.retain(|child| {
        let keep = seen <= body || !is_whitespace_text(child);
        if !keep {
            if let HtmlNode::Text { text } = child {
                trailing.push_str(text);
            }
        }
        seen += 1;
        keep
    });
    trailing.push_str(after_root);
    if trailing.is_empty() {
        return;
    }
    let HtmlNode::Element { children: body_children, .. } = &mut children[body] else { return };
    match body_children.last_mut() {
        Some(HtmlNode::Text { text }) => text.push_str(&trailing),
        _ => body_children.push(HtmlNode::Text { text: trailing }),
    }
}

/// 🔓️ Parses a complete well-formed HTML5 document (optional leading `<!DOCTYPE ...>` + exactly
/// one root element + only whitespace before/after). See the module doc comment for the "honest
/// boundary" this subset draws, and [`normalize_html_root_whitespace`] for the two normative
/// tree-construction whitespace placements applied to an `<html>` root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_html_document(text: &str) -> Result<HtmlSnapshot, TextError> {
    let mut p = Parser::new(text);
    p.skip_ws();

    let doctype = if p.peek_str_ci("<!doctype") {
        p.expect(b'<')?;
        p.expect(b'!')?;
        let start = p.pos;
        while p.peek() != Some(b'>') {
            if p.advance().is_none() {
                return Err(p.err("unterminated <!DOCTYPE ...> declaration, expected '>'"));
            }
        }
        let content = p.slice(start, p.pos).to_string();
        p.advance();
        Some(content)
    } else if p.peek_str("<!") && !p.peek_str("<!--") {
        return Err(p.err("unsupported '<!' construct at document top level (only <!DOCTYPE ...> and <!-- comments --> are supported)"));
    } else {
        None
    };

    p.skip_ws();
    let mut root = p.parse_element()?;
    let tail_start = p.pos;
    p.skip_ws();
    if p.pos != p.bytes.len() {
        return Err(p.err("trailing content after the root element"));
    }
    let after_root = p.slice(tail_start, p.pos).to_string();
    normalize_html_root_whitespace(&mut root, &after_root);
    Ok(HtmlSnapshot { schema: STDIO_HTML_DOCUMENT_SCHEMA.into(), doctype, root })
}
//#endregion 🔖️Parser

//#region 🔖️Writer
/// 🖊️ Serializes a snapshot back to HTML5 text. Canonical/normalized form (documented per the
/// ticket brief's "documented-honest normalization" allowance — the fixed `{doctype, root}`
/// snapshot shape has no slot for the raw bytes between the doctype and the root element, so those
/// are normalized to a single `\n`): `<!doctype>\n` (if present) + the root element, verbatim inside
/// its own subtree (all inter-tag whitespace INSIDE the root IS a real `Text` node and round-trips
/// exactly). Attribute values are always re-emitted double-quoted regardless of the source's
/// original quote style (a second documented normalization — the `HtmlAttr{name,value}` shape has no
/// slot to remember which quote character was used).
///
/// ⚠️ Nothing follows the root element — not even a courtesy `\n`. Whitespace after `</html>` is NOT
/// inert in HTML: WHATWG §13.2.6.4.22 "after after body" processes it with the "in body" rules, so a
/// trailing newline re-enters `<body>`'s last text node on the very next read by any conformant
/// parser. Emitting one made `write` → `html5ever::parse` grow a newline inside `body` on every
/// cycle (found by `🌐️mutate-html-5`'s `set-snapshot` parity row, ticket
/// 26/08/23/END-TO-END-TESTING-REFACTOR); [`parse_html_document`] carries that whitespace into the
/// model instead, where it round-trips as the real text node it is.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_html_document(snapshot: &HtmlSnapshot) -> String {
    let mut out = String::new();
    if let Some(doctype) = &snapshot.doctype {
        out.push_str("<!");
        out.push_str(doctype);
        out.push_str(">\n");
    }
    write_node(&snapshot.root, &mut out);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_node(node: &HtmlNode, out: &mut String) {
    enum Action<'a> { Node(&'a HtmlNode), Close(&'a str) }
    let mut stack = vec![Action::Node(node)];
    while let Some(action) = stack.pop() {
        match action {
            Action::Close(name) => { out.push_str("</"); out.push_str(name); out.push('>'); },
            Action::Node(node) => match node {
                HtmlNode::Text { text } => out.push_str(&encode_text(text)),
                HtmlNode::Comment { text } => { out.push_str("<!--"); out.push_str(text); out.push_str("-->"); },
                HtmlNode::RawText { text, .. } => out.push_str(text),
                HtmlNode::Element { name, attributes, children } => {
                    out.push('<'); out.push_str(name);
                    for attr in attributes {
                        out.push(' '); out.push_str(&attr.name);
                        if let Some(value) = &attr.value { out.push_str("=\""); out.push_str(&encode_attr_value(value)); out.push('"'); }
                    }
                    out.push('>');
                    if !is_void_element(name) { stack.push(Action::Close(name)); for child in children.iter().rev() { stack.push(Action::Node(child)); } }
                },
            },
        }
    }
}
//#endregion 🔖️Writer

//#region 🔖️Navigation
/// 🧭️ Resolves `path` (a chain of child indices from the document root) against `snapshot`,
/// erroring on any non-`Element` intermediate node or out-of-range index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at<'a>(snapshot: &'a HtmlSnapshot, path: &[usize]) -> Result<&'a HtmlNode, String> {
    let mut current = &snapshot.root;
    for &index in path {
        match current {
            HtmlNode::Element { children, .. } => {
                current = children.get(index).ok_or_else(|| format!("node path index {index} out of range"))?;
            }
            other => return Err(format!("node path descends into a non-element node: {other:?}")),
        }
    }
    Ok(current)
}

/// 🔎 Reads attribute `name`'s value from an `Element` node — `None` both when the attribute is
/// absent and when `node` isn't an `Element` (callers that need to distinguish those already have
/// `node_at`'s own `Result`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn element_attr<'a>(node: &'a HtmlNode, name: &str) -> Option<&'a Option<String>> {
    match node {
        HtmlNode::Element { attributes, .. } => attributes.iter().find(|a| a.name == name).map(|a| &a.value),
        _ => None,
    }
}
//#endregion 🔖️Navigation

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for HtmlSnapshot {
    const EXTENSION: &'static str = "html";
    fn envelope_id() -> &'static str {
        STDIO_HTML_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_html_document(body)
    }

    fn print_dsl(&self) -> String {
        let body = write_html_document(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for HtmlSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_html_document(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let _ = options;
        let text = std::str::from_utf8(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        parse_html_document(text).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[path = "🪶️sqlite/🦀️.rs"]
mod sqlite;

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic framing and descriptor roster for the transparent SvgMutation.
use crate::schema::mutations::SvgMutation;
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub const TEXT_OPCODES: &[&str] = &["set-declaration", "set-doctype", "insert-element", "remove-element", "set-element-name", "set-attribute", "set-text", "set-view-box", "set-transform", "set-snapshot", "patch-snapshot"];
fn error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}
fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}
fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    if !value.len().is_multiple_of(2) {
        return Err("payload must be lowercase hexadecimal".to_string());
    }
    value.as_bytes().as_chunks::<2>().0.iter().map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect()
}
impl protocol::OpText for SvgMutation {
    fn print_op(&self) -> String {
        format!("svg-mutation payload={}", encode_hex(semio_framework_pack_json::to_json_string(self).as_bytes()))
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let value = line.strip_prefix("svg-mutation payload=").ok_or_else(|| error("expected aggregate payload"))?;
        let bytes = decode_hex(value).map_err(error)?;
        let text = std::str::from_utf8(&bytes).map_err(|cause| error(cause.to_string()))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|cause| error(cause.to_string()))
    }
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic text framing for the visible glTF mutation aggregate.

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

use crate::schema::mutations::GltfMutation;

const GLTF_MUTATION_MAX_PAYLOAD_BYTES: usize = 64 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > GLTF_MUTATION_MAX_PAYLOAD_BYTES * 2 {
        return Err("GLTF mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "GLTF mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "GLTF mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl protocol::OpText for GltfMutation {
    fn print_op(&self) -> String {
        format!("gltf-mutation payload={}", encode_hex(semio_framework_pack_json::to_json_string(self).as_bytes()))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("gltf-mutation payload=").ok_or_else(|| text_error("expected canonical GLTF mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let text = std::str::from_utf8(&bytes).map_err(|error| text_error(error.to_string()))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))
    }
}

```

## ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs

```rust
//! shooting <- json
//!
//! 🩹️ w5b-close fix (stdio_gap/foreign-lag, not svg/dwg-pattern scope — see w5b-close-report.md):
//! see the paired export leaf's doc comment. Mirrors it going through stdio's own
//! `JsonSnapshot::to_serde_value` bridge and stdio's own real `parse_json_text`.
use crate::ShootingSnapshot;
use crate::SHOOTING_DOCUMENT_SCHEMA;
use semio_s_artifact_stdio_json::schema::snapshot::{parse_json_text, JsonSnapshot};

pub fn register() {}

pub fn deserialize(from: &JsonSnapshot) -> Result<ShootingSnapshot, semio_framework_diagnostic::TextError> {
    let _ = SHOOTING_DOCUMENT_SCHEMA;
    let dsl_value: dsl::DslValue = from.to_serde_value().into();
    let mut out: ShootingSnapshot = dsl::FromValue::from_value(dsl_value).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("shooting<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = SHOOTING_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ShootingSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text).map_err(|e| semio_framework_diagnostic::TextError::new(e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    deserialize(&JsonSnapshot::from_value(value))
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic text framing and direct-owner registry for the visible PDF/UA mutation aggregate.

use super::PdfUaMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("SetMarkInfo", super::set_mark_info::text::TEXT_OPCODE),
    ("RemoveMarkInfo", super::remove_mark_info::text::TEXT_OPCODE),
    ("SetStructTreeRoot", super::set_struct_tree_root::text::TEXT_OPCODE),
    ("RemoveStructTreeRoot", super::remove_struct_tree_root::text::TEXT_OPCODE),
    ("SetLang", super::set_lang::text::TEXT_OPCODE),
    ("RemoveLang", super::remove_lang::text::TEXT_OPCODE),
    ("SetDisplayDocTitle", super::set_display_doc_title::text::TEXT_OPCODE),
    ("RemoveDisplayDocTitle", super::remove_display_doc_title::text::TEXT_OPCODE),
    ("SetInfoTitle", super::set_info_title::text::TEXT_OPCODE),
    ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
    ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF/UA mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        if (b'a'..=b'f').contains(&value) {
            Some(value - b'a' + 10)
        } else {
            None
        }
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF/UA mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF/UA mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfUaMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        format!("pdf-ua-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-ua-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/UA mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        <Self as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic text framing and direct-owner registry for the visible PDF/VT mutation aggregate.

use super::PdfVtMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
    ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
    ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
    ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
    ("SetTrimBox", super::set_trim_box::text::TEXT_OPCODE),
    ("RemoveTrimBox", super::remove_trim_box::text::TEXT_OPCODE),
    ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
    ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
    ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
    ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
    ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
    ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
    ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
    ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
    ("SetDpartRoot", super::set_dpart_root::text::TEXT_OPCODE),
    ("RemoveDpartRoot", super::remove_dpart_root::text::TEXT_OPCODE),
    ("SetDpartMetadata", super::set_dpart_metadata::text::TEXT_OPCODE),
    ("RemoveDpartMetadata", super::remove_dpart_metadata::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF/VT mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        if (b'a'..=b'f').contains(&value) {
            Some(value - b'a' + 10)
        } else {
            None
        }
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF/VT mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF/VT mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfVtMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&dsl::ToValue::to_value(self))).into_bytes();
        format!("pdf-vt-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-vt-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/VT mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        <PdfVtMutation as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic framing and descriptor roster for the transparent XmlMutation.
use crate::schema::mutations::XmlMutation;
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
pub const TEXT_OPCODES: &[&str] = &["set-declaration", "set-doctype", "insert-element", "remove-element", "set-attribute", "set-text", "set-snapshot", "patch-snapshot"];
fn error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}
fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}
fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    if !value.len().is_multiple_of(2) {
        return Err("payload must be lowercase hexadecimal".to_string());
    }
    value.as_bytes().as_chunks::<2>().0.iter().map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect()
}
impl protocol::OpText for XmlMutation {
    fn print_op(&self) -> String {
        format!("xml-mutation payload={}", encode_hex(semio_framework_pack_json::to_json_string(self).as_bytes()))
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let value = line.strip_prefix("xml-mutation payload=").ok_or_else(|| error("expected aggregate payload"))?;
        let bytes = decode_hex(value).map_err(error)?;
        let text = std::str::from_utf8(&bytes).map_err(|cause| error(cause.to_string()))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|cause| error(cause.to_string()))
    }
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/🧬️mutations/🦀️.rs

```rust
//! 🧬️ SemioAnimationMutation — full named-variant vocabulary (gif 89a / docx precedent), replacing
//! the W1b `SetSnapshot`-only scaffold. Every variant's `diff()`/`inverse()` is HAND-WRITTEN
//! (apply-and-capture is banned per `🧬️schema-design.md`'s svg infinite-recursion warning) —
//! `diff()` builds the exact sparse `SemioAnimationDiff` directly via the `diff_*` helpers below,
//! never by diffing a mutated clone against `base`.
//!
//! 🪆️ Mutation-leaf migration (ticket 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL): each variant now wraps
//! its own `dsl::MutationLeaf` payload type (`🧬️mutations/<emoji><kind>/🦀️.rs`), and
//! `#[derive(dsl::Mutations)]` synthesizes `DESCRIPTORS`/`descriptor()` from that leaf roster —
//! required by `protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:105`).
//! `NoMutation` is dropped: the derive requires every variant to wrap exactly one leaf payload, and
//! `no` is not an approved semantic verb.

use crate::standards::v1::subsets::animation::schema::diff::{diff_set_snapshot, AnimChannelDiff, AnimKeyframeDiff, AnimTimelineDiff, SemioAnimationDiff};
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::Mutation;
/// 🔧️ `MutationDiff` added — the `#[cfg(test)] mod tests` block below calls `diff.apply(&base)`
/// via method syntax on `SemioAnimationDiff`, which needs `MutationDiff` in scope (W2b closer fix).
#[cfg(test)]
use protocol::MutationDiff;
use protocol::{OpBinary, OpText};

//#region 🔖️Mutation
#[path = "📻insert-channel/🦀️.rs"]
pub mod insert_channel;
#[path = "🔑insert-keyframe/🦀️.rs"]
pub mod insert_keyframe;
#[path = "🎬insert-timeline/🦀️.rs"]
pub mod insert_timeline;
#[path = "🗑️remove-channel/🦀️.rs"]
pub mod remove_channel;
#[path = "🔓remove-keyframe/🦀️.rs"]
pub mod remove_keyframe;
#[path = "🧹remove-timeline/🦀️.rs"]
pub mod remove_timeline;
#[path = "📈set-channel-interpolation/🦀️.rs"]
pub mod set_channel_interpolation;
#[path = "🎯set-channel-target/🦀️.rs"]
pub mod set_channel_target;
#[path = "🕐set-keyframe-time/🦀️.rs"]
pub mod set_keyframe_time;
#[path = "🔢set-keyframe-value/🦀️.rs"]
pub mod set_keyframe_value;
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-timeline-name/🦀️.rs"]
pub mod set_timeline_name;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioAnimationSnapshot, diff = SemioAnimationDiff, schema = "SemioAnimationMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAnimationMutation {
    /// 📦️ Full-snapshot replace, still sparse under the hood (`diff_set_snapshot` = `between`).
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    InsertTimeline(insert_timeline::InsertTimeline),
    RemoveTimeline(remove_timeline::RemoveTimeline),
    /// 🏷️ `name: None` clears the timeline's display name.
    SetTimelineName(set_timeline_name::SetTimelineName),
    InsertChannel(insert_channel::InsertChannel),
    RemoveChannel(remove_channel::RemoveChannel),
    SetChannelTarget(set_channel_target::SetChannelTarget),
    SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation),
    InsertKeyframe(insert_keyframe::InsertKeyframe),
    RemoveKeyframe(remove_keyframe::RemoveKeyframe),
    SetKeyframeTime(set_keyframe_time::SetKeyframeTime),
    SetKeyframeValue(set_keyframe_value::SetKeyframeValue),
}

/// 🏷️ The declared kebab-case mutation vocabulary of `s.stdio.semio.animation`, in enum
/// declaration order — what the `🎞️mutate-semio-animation` case's completeness gate counts against
/// and what `../../🔮️oracles/🔣️.json`'s catalog repeats. Unlike its audio/video siblings
/// this subset's wire keywords are the two-letter `TEXT_KEYWORDS` heads (`IT`, `KV`, …), so the two
/// tables are related only by position; `kinds_match_the_enum_and_the_catalog` below asserts that
/// positional agreement rather than string equality.
pub const KINDS: &[&str] =
    &["set-snapshot", "insert-timeline", "remove-timeline", "set-timeline-name", "insert-channel", "remove-channel", "set-channel-target", "set-channel-interpolation", "insert-keyframe", "remove-keyframe", "set-keyframe-time", "set-keyframe-value"];
//#endregion 🔖️Mutation

//#region 🔖️DiffBuilders
/// 🧱️ Wraps a per-timeline `AnimTimelineDiff` into a full `SemioAnimationDiff` — the innermost
/// layer of the nested-diff tree every non-collection-root mutation ultimately builds on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_timeline_field(index: usize, diff: AnimTimelineDiff) -> SemioAnimationDiff {
    SemioAnimationDiff { timelines: Some(IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_channel_collection(timeline_index: usize, channels: IndexedTripleDiff<AnimChannelDiff, AnimChannel>) -> SemioAnimationDiff {
    diff_timeline_field(timeline_index, AnimTimelineDiff { name: None, channels: Some(channels) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_channel_field(timeline_index: usize, index: usize, diff: AnimChannelDiff) -> SemioAnimationDiff {
    diff_channel_collection(timeline_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_keyframe_collection(timeline_index: usize, channel_index: usize, keyframes: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe>) -> SemioAnimationDiff {
    diff_channel_field(timeline_index, channel_index, AnimChannelDiff { target: None, interpolation: None, keyframes: Some(keyframes) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_keyframe_field(timeline_index: usize, channel_index: usize, index: usize, diff: AnimKeyframeDiff) -> SemioAnimationDiff {
    diff_keyframe_collection(timeline_index, channel_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}
//#endregion 🔖️DiffBuilders

//#region 🔖️BaseAccessors
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn timeline_at(base: &SemioAnimationSnapshot, i: usize) -> Option<&AnimTimeline> {
    base.timelines.get(i)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn channel_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize) -> Option<&AnimChannel> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn keyframe_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize, ki: usize) -> Option<&AnimKeyframe> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci)).and_then(|c| c.keyframes.get(ki))
}
//#endregion 🔖️BaseAccessors

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_animation_mutation(snapshot: &mut SemioAnimationSnapshot, mutation: &SemioAnimationMutation) -> protocol::MutationOutcome<SemioAnimationDiff> {
    let outcome = <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`SemioAnimationMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with [`apply_semio_animation_mutation`] it makes the
/// undo law reachable without importing a trait the caller cannot name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_animation_mutation(mutation: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> Vec<SemioAnimationMutation> {
    <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(mutation, base)
}

/// 📥️ Decodes this subset's internally tagged (`{"mutation": "<camelCaseVariant>", ...}`) wire value — the shape
/// `🎞️mutate-semio-animation`'s committed specification vectors and doc strings carry — into a real [`SemioAnimationMutation`]. A thin
/// `pack::from_json_str` wrapper over `ToValue`/`FromValue`, so the test adapter reads the committed wire value instead of
/// re-declaring it field by field beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_animation_mutation_json(text: &str) -> Result<SemioAnimationMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    protocol::MutationOutcome::new(match this {
        PatchSnapshot(payload) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation>>::diff(payload, base),
        SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        InsertTimeline(insert_timeline::InsertTimeline { index, timeline }) => SemioAnimationDiff { timelines: Some(IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: timeline.clone() }], ..Default::default() }) },
        RemoveTimeline(remove_timeline::RemoveTimeline { index }) => SemioAnimationDiff { timelines: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }) },
        SetTimelineName(set_timeline_name::SetTimelineName { index, name }) => diff_timeline_field(*index, AnimTimelineDiff { name: Some(name.clone()), channels: None }),
        InsertChannel(insert_channel::InsertChannel { timeline_index, index, channel }) => diff_channel_collection(*timeline_index, IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: channel.clone() }], ..Default::default() }),
        RemoveChannel(remove_channel::RemoveChannel { timeline_index, index }) => diff_channel_collection(*timeline_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
        SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index, index, target }) => diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: Some(target.clone()), interpolation: None, keyframes: None }),
        SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index, index, interpolation }) => {
            diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: None, interpolation: Some(*interpolation), keyframes: None })
        }
        InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, keyframe }) => {
            diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: keyframe.clone() }], ..Default::default() })
        }
        RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index }) => diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
        SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, t }) => diff_keyframe_field(*timeline_index, *channel_index, *index, AnimKeyframeDiff { t: Some(*t), value: None }),
        SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, value }) => diff_keyframe_field(*timeline_index, *channel_index, *index, AnimKeyframeDiff { t: None, value: Some(value.clone()) }),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> Vec<SemioAnimationMutation> {
    use SemioAnimationMutation::*;
    match this {
        PatchSnapshot(payload) => <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation>>::inverse(payload, base),
        SetSnapshot(_) => vec![SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        InsertTimeline(insert_timeline::InsertTimeline { index, .. }) => vec![RemoveTimeline(remove_timeline::RemoveTimeline { index: *index })],
        RemoveTimeline(remove_timeline::RemoveTimeline { index }) => match timeline_at(base, *index) {
            Some(t) => vec![InsertTimeline(insert_timeline::InsertTimeline { index: *index, timeline: t.clone() })],
            None => Vec::new(),
        },
        SetTimelineName(set_timeline_name::SetTimelineName { index, .. }) => vec![SetTimelineName(set_timeline_name::SetTimelineName { index: *index, name: timeline_at(base, *index).and_then(|t| t.name.clone()) })],
        InsertChannel(insert_channel::InsertChannel { timeline_index, index, .. }) => vec![RemoveChannel(remove_channel::RemoveChannel { timeline_index: *timeline_index, index: *index })],
        RemoveChannel(remove_channel::RemoveChannel { timeline_index, index }) => match channel_at(base, *timeline_index, *index) {
            Some(c) => vec![InsertChannel(insert_channel::InsertChannel { timeline_index: *timeline_index, index: *index, channel: c.clone() })],
            None => Vec::new(),
        },
        SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index, index, .. }) => match channel_at(base, *timeline_index, *index) {
            Some(c) => vec![SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index: *timeline_index, index: *index, target: c.target.clone() })],
            None => Vec::new(),
        },
        SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index, index, .. }) => match channel_at(base, *timeline_index, *index) {
            Some(c) => vec![SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: *timeline_index, index: *index, interpolation: c.interpolation })],
            None => Vec::new(),
        },
        InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, .. }) => {
            vec![RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index })]
        }
        RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index }) => match keyframe_at(base, *timeline_index, *channel_index, *index) {
            Some(k) => vec![InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, keyframe: k.clone() })],
            None => Vec::new(),
        },
        SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, .. }) => match keyframe_at(base, *timeline_index, *channel_index, *index) {
            Some(k) => vec![SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, t: k.t })],
            None => Vec::new(),
        },
        SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, .. }) => match keyframe_at(base, *timeline_index, *channel_index, *index) {
            Some(k) => vec![SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, value: k.value.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region SnapshotLit
/// 🧩️ `SetSnapshot`'s whole-snapshot payload — `[hex(schema),[timeline,...]]`, reusing the diff
/// facet's own `pub(crate)` `enc_timeline`/`dec_timeline`/`enc_str`/`dec_str`/`enc_list`/`dec_list`
/// value codecs (one source of truth, not a third independent copy). W2c closer fix: this REPLACES
/// the old whole-enum `serde_json::to_string`/`from_str` passthrough — a real JSON-transfer-ban
/// violation the brief specifically flagged as a recurring pattern to check for (confirmed present
/// here, unlike the sibling `🔺️diff` facet, which was already fully real pre-wave).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_animation_snapshot(s: &SemioAnimationSnapshot) -> String {
    use crate::standards::v1::subsets::animation::schema::diff::{enc_list, enc_str, enc_timeline};
    format!("[{},{}]", enc_str(&s.schema), enc_list(&s.timelines, enc_timeline))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_animation_snapshot(s: &str) -> Result<SemioAnimationSnapshot, String> {
    use crate::standards::v1::subsets::animation::schema::diff::{dec_list, dec_str, dec_timeline};
    use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, timelines] = parts.as_slice() else { return Err(format!("snapshot-lit: expected 2 fields, got {}", parts.len())) };
    Ok(SemioAnimationSnapshot { schema: dec_str(schema)?, timelines: dec_list(timelines, dec_timeline)? })
}
//#endregion SnapshotLit

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary` — one `TAG:payload` line per variant, reusing the diff
/// module's `pub(crate)` value codecs (`enc_timeline`/`enc_channel`/`enc_keyframe`/`enc_target`/
/// `enc_value`/`enc_interpolation`/hex-string helpers) instead of re-deriving a second parallel
/// grammar. `SetSnapshot` reuses the `enc_animation_snapshot`/`dec_animation_snapshot` whole-
/// snapshot codec above (W2c closer fix — was `serde_json`, see that region's doc comment).
impl OpText for SemioAnimationMutation {
    fn print_op(&self) -> String {
        use crate::standards::v1::subsets::animation::schema::diff::{enc_channel, enc_interpolation, enc_keyframe, enc_str, enc_target, enc_timeline, enc_value};
        use SemioAnimationMutation::*;
        match self {
            PatchSnapshot(payload) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(&payload.patch),
            SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("S:{}", enc_animation_snapshot(snapshot)),
            InsertTimeline(insert_timeline::InsertTimeline { index, timeline }) => format!("IT:{index},{}", enc_timeline(timeline)),
            RemoveTimeline(remove_timeline::RemoveTimeline { index }) => format!("RT:{index}"),
            SetTimelineName(set_timeline_name::SetTimelineName { index, name }) => format!(
                "TN:{index},{}",
                match name {
                    None => "[0]".to_string(),
                    Some(n) => format!("[1,{}]", enc_str(n)),
                }
            ),
            InsertChannel(insert_channel::InsertChannel { timeline_index, index, channel }) => format!("IC:{timeline_index},{index},{}", enc_channel(channel)),
            RemoveChannel(remove_channel::RemoveChannel { timeline_index, index }) => format!("RC:{timeline_index},{index}"),
            SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index, index, target }) => format!("CT:{timeline_index},{index},{}", enc_target(target)),
            SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index, index, interpolation }) => format!("CI:{timeline_index},{index},{}", enc_interpolation(*interpolation)),
            InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, keyframe }) => format!("IK:{timeline_index},{channel_index},{index},{}", enc_keyframe(keyframe)),
            RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index }) => format!("RK:{timeline_index},{channel_index},{index}"),
            SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, t }) => format!("KT:{timeline_index},{channel_index},{index},{t}"),
            SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, value }) => format!("KV:{timeline_index},{channel_index},{index},{}", enc_value(value)),
        }
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        use crate::standards::v1::subsets::animation::schema::diff::{dec_channel, dec_interpolation, dec_keyframe, dec_str, dec_target, dec_timeline, dec_value};
        use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
        use SemioAnimationMutation::*;
        let fail = |e: String| semio_framework_diagnostic::TextError::new(e, semio_framework_diagnostic::TextSpan::at(1, 1));
        if line.starts_with("patch-snapshot patch=") {
            return semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| Self::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })).map_err(fail);
        }
        let parse_usize = |s: &str| s.parse::<usize>().map_err(|e: std::num::ParseIntError| e.to_string());
        let parse_f64 = |s: &str| s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string());

        let (tag, rest) = line.split_once(':').ok_or_else(|| fail(format!("op: bad shape {line:?}")))?;
        (|| -> Result<Self, String> {
            match tag {
                "S" => Ok(SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_animation_snapshot(rest)? })),
                "IT" => {
                    let (index, timeline) = rest.split_once(',').ok_or_else(|| "IT: missing comma".to_string())?;
                    Ok(InsertTimeline(insert_timeline::InsertTimeline { index: parse_usize(index)?, timeline: dec_timeline(timeline)? }))
                }
                "RT" => Ok(RemoveTimeline(remove_timeline::RemoveTimeline { index: parse_usize(rest)? })),
                "TN" => {
                    let (index, name) = rest.split_once(',').ok_or_else(|| "TN: missing comma".to_string())?;
                    let parts = split_top_level(strip_brackets(name)?, ',');
                    let name = match parts.as_slice() {
                        ["0"] => None,
                        [tag, value] if *tag == "1" => Some(dec_str(value)?),
                        other => return Err(format!("TN: bad option shape {other:?}")),
                    };
                    Ok(SetTimelineName(set_timeline_name::SetTimelineName { index: parse_usize(index)?, name }))
                }
                "IC" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, index, rest_channel @ ..] = parts.as_slice() else { return Err("IC: expected 3+ fields".to_string()) };
                    let channel = rest_channel.join(",");
                    Ok(InsertChannel(insert_channel::InsertChannel { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, channel: dec_channel(&channel)? }))
                }
                "RC" => {
                    let (ti, index) = rest.split_once(',').ok_or_else(|| "RC: missing comma".to_string())?;
                    Ok(RemoveChannel(remove_channel::RemoveChannel { timeline_index: parse_usize(ti)?, index: parse_usize(index)? }))
                }
                "CT" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, index, rest_target @ ..] = parts.as_slice() else { return Err("CT: expected 3+ fields".to_string()) };
                    Ok(SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, target: dec_target(&rest_target.join(","))? }))
                }
                "CI" => {
                    let parts: Vec<&str> = rest.splitn(3, ',').collect();
                    let [ti, index, interp] = parts.as_slice() else { return Err("CI: expected 3 fields".to_string()) };
                    Ok(SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: parse_usize(ti)?, index: parse_usize(index)?, interpolation: dec_interpolation(interp)? }))
                }
                "IK" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, ci, index, rest_kf @ ..] = parts.as_slice() else { return Err("IK: expected 4+ fields".to_string()) };
                    Ok(InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)?, keyframe: dec_keyframe(&rest_kf.join(","))? }))
                }
                "RK" => {
                    let parts: Vec<&str> = rest.splitn(3, ',').collect();
                    let [ti, ci, index] = parts.as_slice() else { return Err("RK: expected 3 fields".to_string()) };
                    Ok(RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)? }))
                }
                "KT" => {
                    let parts: Vec<&str> = rest.splitn(4, ',').collect();
                    let [ti, ci, index, t] = parts.as_slice() else { return Err("KT: expected 4 fields".to_string()) };
                    Ok(SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)?, t: parse_f64(t)? }))
                }
                "KV" => {
                    let parts = split_top_level(rest, ',');
                    let [ti, ci, index, rest_value @ ..] = parts.as_slice() else { return Err("KV: expected 4+ fields".to_string()) };
                    Ok(SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index: parse_usize(ti)?, channel_index: parse_usize(ci)?, index: parse_usize(index)?, value: dec_value(&rest_value.join(","))? }))
                }
                other => Err(format!("op: unknown tag {other:?}")),
            }
        })()
        .map_err(fail)
    }
}

//#region 🏷️WireTags
/// 🏷️ Op tags of `SemioAnimationMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("💾️binary/📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_INSERT_TIMELINE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-timeline");
const TAG_REMOVE_TIMELINE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-timeline");
const TAG_SET_TIMELINE_NAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-timeline-name");
const TAG_INSERT_CHANNEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-channel");
const TAG_REMOVE_CHANNEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-channel");
const TAG_SET_CHANNEL_TARGET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-channel-target");
const TAG_SET_CHANNEL_INTERPOLATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-channel-interpolation");
const TAG_INSERT_KEYFRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-keyframe");
const TAG_REMOVE_KEYFRAME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-keyframe");
const TAG_SET_KEYFRAME_TIME: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-keyframe-time");
const TAG_SET_KEYFRAME_VALUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-keyframe-value");
//#endregion 🏷️WireTags

/// 🧾️ Each record kind's text-grammar tag, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
const TEXT_KEYWORDS: [(&str, &str); 12] = [
    ("set-snapshot", "S"),
    ("insert-timeline", "IT"),
    ("remove-timeline", "RT"),
    ("set-timeline-name", "TN"),
    ("insert-channel", "IC"),
    ("remove-channel", "RC"),
    ("set-channel-target", "CT"),
    ("set-channel-interpolation", "CI"),
    ("insert-keyframe", "IK"),
    ("remove-keyframe", "RK"),
    ("set-keyframe-time", "KT"),
    ("set-keyframe-value", "KV"),
];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wire_tag(m: &SemioAnimationMutation) -> u8 {
    use SemioAnimationMutation::*;
    match m {
        SetSnapshot(_) => TAG_SET_SNAPSHOT,
        PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
        InsertTimeline(_) => TAG_INSERT_TIMELINE,
        RemoveTimeline(_) => TAG_REMOVE_TIMELINE,
        SetTimelineName(_) => TAG_SET_TIMELINE_NAME,
        InsertChannel(_) => TAG_INSERT_CHANNEL,
        RemoveChannel(_) => TAG_REMOVE_CHANNEL,
        SetChannelTarget(_) => TAG_SET_CHANNEL_TARGET,
        SetChannelInterpolation(_) => TAG_SET_CHANNEL_INTERPOLATION,
        InsertKeyframe(_) => TAG_INSERT_KEYFRAME,
        RemoveKeyframe(_) => TAG_REMOVE_KEYFRAME,
        SetKeyframeTime(_) => TAG_SET_KEYFRAME_TIME,
        SetKeyframeValue(_) => TAG_SET_KEYFRAME_VALUE,
    }
}

const OP_BINARY_FORMAT: u8 = 1;

/// 🔢️ Real binary op frame (animation wave — off the old whole-`OpText`-line `.into_bytes()` F6
/// text-as-binary shortcut). `format u8` + `tag u8` (its kind's record tag in `💾️binary/📡️.protocol.semio`) as two real fixed
/// fields, then the variant's own `key=value,...` argument text (i.e. `print_op`'s output with its
/// `TAG:` prefix stripped) as one opaque trailing `bytes` chain — reuses the real, tested
/// `print_op`/`parse_op` text codec (one source of truth), same treatment every prior semio wave's
/// `OpBinary` upgrade uses.
impl OpBinary for SemioAnimationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        if let Self::PatchSnapshot(payload) = self {
            let mut out = vec![1, TAG_PATCH_SNAPSHOT];
            out.extend(protocol::OpBinary::encode_op(&payload.patch)?);
            return Ok(out);
        }
        let printed = <Self as OpText>::print_op(self);
        let args = match printed.split_once(':') {
            Some((_, rest)) => rest,
            None => "",
        };
        let mut out = vec![OP_BINARY_FORMAT, wire_tag(self)];
        out.extend_from_slice(args.as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 0, detail };
        let [format, tag, rest @ ..] = bytes else { return Err(malformed("op header", format!("expected at least 2 bytes, got {}", bytes.len()))) };
        if *format != OP_BINARY_FORMAT {
            return Err(malformed("op format", format!("unsupported op format {format}")));
        }
        if *tag == TAG_PATCH_SNAPSHOT {
            return Ok(Self::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: protocol::OpBinary::decode_op(rest)? }));
        }
        let kind = dsl::protocol_record::kind(WIRE_PROTOCOL, u64::from(*tag)).ok_or_else(|| malformed("op tag", format!("tag {tag} names no record of 📡️.protocol.semio")))?;
        let keyword = TEXT_KEYWORDS.iter().find(|(record, _)| *record == kind).map(|(_, keyword)| *keyword).ok_or_else(|| malformed("op tag", format!("record {kind} has no text keyword")))?;
        let args = std::str::from_utf8(rest).map_err(|e| malformed("op args utf8", e.to_string()))?;
        let line = format!("{keyword}:{args}");
        <Self as OpText>::parse_op(&line).map_err(|e| malformed("op text", e.to_string()))
    }
}
//#endregion OpCodecs

/// 🧱️ Module-scope (not `mod tests`-local) fixture + demo mutation cases — so the `🎹️composer`
/// conformance-law tests can reuse them, same promotion pattern every prior semio wave's report
/// documents (a private item of a child `mod tests` isn't visible to a sibling module).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fixture() -> SemioAnimationSnapshot {
    SemioAnimationSnapshot {
        timelines: vec![AnimTimeline {
            name: Some("walk".into()),
            channels: vec![AnimChannel {
                target: AnimTarget { node: "hip".into(), property: crate::standards::v1::subsets::animation::schema::snapshot::AnimTargetProperty::Translation },
                interpolation: AnimInterpolation::Linear,
                keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } }],
            }],
        }],
        ..SemioAnimationSnapshot::default()
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioAnimationMutation> {
    let base = fixture();
    use SemioAnimationMutation::*;
    vec![
        SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        InsertTimeline(insert_timeline::InsertTimeline { index: 1, timeline: AnimTimeline { name: Some("wave".into()), channels: vec![] } }),
        RemoveTimeline(remove_timeline::RemoveTimeline { index: 0 }),
        SetTimelineName(set_timeline_name::SetTimelineName { index: 0, name: None }),
        InsertChannel(insert_channel::InsertChannel { timeline_index: 0, index: 1, channel: base.timelines[0].channels[0].clone() }),
        RemoveChannel(remove_channel::RemoveChannel { timeline_index: 0, index: 0 }),
        SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index: 0, index: 0, target: AnimTarget { node: "spine".into(), property: crate::standards::v1::subsets::animation::schema::snapshot::AnimTargetProperty::Rotation } }),
        SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: 0, index: 0, interpolation: AnimInterpolation::Step }),
        InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index: 0, channel_index: 0, index: 1, keyframe: AnimKeyframe { t: 2.0, value: AnimValue::Scalar { value: 5.0 } } }),
        RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: 0, channel_index: 0, index: 0 }),
        SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: 0, channel_index: 0, index: 0, t: 3.5 }),
        SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index: 0, channel_index: 0, index: 0, value: AnimValue::Weights { values: vec![0.1, 0.9] } }),
    ]
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🌀️steps/🦀️.rs"]
mod set_snapshot_steps_the_spin_channel_and_appends_a_keyframe;
//#endregion 🧪️FixtureCases

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic text framing and direct-owner registry for the visible PDF mutation aggregate.

use super::PdfMutation;
use protocol::OpText;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertPage", super::insert_page::text::TEXT_OPCODE),
    ("RemovePage", super::remove_page::text::TEXT_OPCODE),
    ("SetPageMediaBox", super::set_page_media_box::text::TEXT_OPCODE),
    ("SetPageCropBox", super::set_page_crop_box::text::TEXT_OPCODE),
    ("AppendPageContent", super::append_page_content::text::TEXT_OPCODE),
    ("SetInfo", super::set_info::text::TEXT_OPCODE),
    ("InsertObject", super::insert_object::text::TEXT_OPCODE),
    ("RemoveObject", super::remove_object::text::TEXT_OPCODE),
    ("SetObjectValue", super::set_object_value::text::TEXT_OPCODE),
    ("SetDictEntry", super::set_dict_entry::text::TEXT_OPCODE),
    ("RemoveDictEntry", super::remove_dict_entry::text::TEXT_OPCODE),
    ("SetTrailerEntry", super::set_trailer_entry::text::TEXT_OPCODE),
    ("RemoveTrailerEntry", super::remove_trailer_entry::text::TEXT_OPCODE),
    ("MovePage", super::move_page::text::TEXT_OPCODE),
    ("SetPageContent", super::set_page_content::text::TEXT_OPCODE),
    ("SetPageRotation", super::set_page_rotation::text::TEXT_OPCODE),
    ("SetPageBox", super::set_page_box::text::TEXT_OPCODE),
    ("SetPageUserUnit", super::set_page_user_unit::text::TEXT_OPCODE),
    ("InsertContent", super::insert_content::text::TEXT_OPCODE),
    ("RemoveContent", super::remove_content::text::TEXT_OPCODE),
    ("ReplaceContent", super::replace_content::text::TEXT_OPCODE),
    ("InsertAnnotation", super::insert_annotation::text::TEXT_OPCODE),
    ("RemoveAnnotation", super::remove_annotation::text::TEXT_OPCODE),
    ("SetAnnotation", super::set_annotation::text::TEXT_OPCODE),
    ("SetFont", super::set_font::text::TEXT_OPCODE),
    ("RemoveFont", super::remove_font::text::TEXT_OPCODE),
    ("SetImage", super::set_image::text::TEXT_OPCODE),
    ("RemoveImage", super::remove_image::text::TEXT_OPCODE),
    ("SetForm", super::set_form::text::TEXT_OPCODE),
    ("RemoveForm", super::remove_form::text::TEXT_OPCODE),
    ("SetExtGState", super::set_ext_g_state::text::TEXT_OPCODE),
    ("RemoveExtGState", super::remove_ext_g_state::text::TEXT_OPCODE),
    ("SetShading", super::set_shading::text::TEXT_OPCODE),
    ("RemoveShading", super::remove_shading::text::TEXT_OPCODE),
    ("SetPattern", super::set_pattern::text::TEXT_OPCODE),
    ("RemovePattern", super::remove_pattern::text::TEXT_OPCODE),
    ("SetColorSpace", super::set_color_space::text::TEXT_OPCODE),
    ("RemoveColorSpace", super::remove_color_space::text::TEXT_OPCODE),
    ("SetProperties", super::set_properties::text::TEXT_OPCODE),
    ("RemoveProperties", super::remove_properties::text::TEXT_OPCODE),
    ("SetEmbeddedFile", super::set_embedded_file::text::TEXT_OPCODE),
    ("RemoveEmbeddedFile", super::remove_embedded_file::text::TEXT_OPCODE),
    ("SetOutlines", super::set_outlines::text::TEXT_OPCODE),
    ("SetNamedDestination", super::set_named_destination::text::TEXT_OPCODE),
    ("RemoveNamedDestination", super::remove_named_destination::text::TEXT_OPCODE),
    ("SetPageLabels", super::set_page_labels::text::TEXT_OPCODE),
    ("SetOutputIntents", super::set_output_intents::text::TEXT_OPCODE),
    ("SetAcroForm", super::set_acro_form::text::TEXT_OPCODE),
    ("SetOptionalContent", super::set_optional_content::text::TEXT_OPCODE),
    ("SetPageLayout", super::set_page_layout::text::TEXT_OPCODE),
    ("SetPageMode", super::set_page_mode::text::TEXT_OPCODE),
    ("SetViewerPreferences", super::set_viewer_preferences::text::TEXT_OPCODE),
    ("SetOpenAction", super::set_open_action::text::TEXT_OPCODE),
    ("SetLanguage", super::set_language::text::TEXT_OPCODE),
    ("SetMarkInfo", super::set_mark_info::text::TEXT_OPCODE),
    ("SetMetadata", super::set_metadata::text::TEXT_OPCODE),
    ("SetDocumentId", super::set_document_id::text::TEXT_OPCODE),
    ("SetEncryption", super::set_encryption::text::TEXT_OPCODE),
    ("SetCatalogEntry", super::set_catalog_entry::text::TEXT_OPCODE),
    ("RemoveCatalogEntry", super::remove_catalog_entry::text::TEXT_OPCODE),
    ("SetSnapshot", super::set_snapshot::text::TEXT_OPCODE),
    ("PatchSnapshot", super::patch_snapshot::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        format!("pdf-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-mutation payload=").ok_or_else(|| text_error("expected canonical PDF mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        dsl::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic text framing and direct-owner registry for the visible PDF/X mutation aggregate.

use super::PdfXMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
    ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
    ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
    ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
    ("SetTrimBox", super::set_trim_box::text::TEXT_OPCODE),
    ("RemoveTrimBox", super::remove_trim_box::text::TEXT_OPCODE),
    ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
    ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
    ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
    ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
    ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
    ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
    ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
    ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF/X mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        if (b'a'..=b'f').contains(&value) {
            Some(value - b'a' + 10)
        } else {
            None
        }
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF/X mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF/X mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfXMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&dsl::ToValue::to_value(self))).into_bytes();
        format!("pdf-x-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-x-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/X mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        <PdfXMutation as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic text framing and direct-owner registry for the visible PDF/H mutation aggregate.

use super::PdfHMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("SetInfoTitle", super::set_info_title::text::TEXT_OPCODE),
    ("SetInfoAuthor", super::set_info_author::text::TEXT_OPCODE),
    ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
    ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
    ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
    ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
    ("InsertSignatureField", super::insert_signature_field::text::TEXT_OPCODE),
    ("RemoveSignatureField", super::remove_signature_field::text::TEXT_OPCODE),
    ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
    ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF/H mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF/H mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF/H mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfHMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        format!("pdf-h-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-h-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/H mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        <Self as dsl::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

```

## ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📕️xlsx/🔖️ecma-376/✳️any/🦀️.rs

```rust
//! program -> xlsx — one worksheet per program table, header row 1 and one row per record from column A; stdio's minimal
//! package builder turns the workbook into the authoritative OPC XML parts (`build_minimal_xlsx`).
use crate::ProgramSnapshot;
pub use semio_s_artifact_stdio_xlsx::schema::snapshot::{XlsxCell, XlsxCellValue, XlsxSheet, XlsxWorkbook};
pub use semio_s_artifact_stdio_xlsx::XlsxSnapshot;
use semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx;
use std::collections::BTreeSet;

pub fn register() {}

fn export_error(message: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(message.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

fn cell_value(value: &dsl::DslValue) -> Result<XlsxCellValue, semio_framework_diagnostic::TextError> {
    match value {
        dsl::DslValue::Null => Ok(XlsxCellValue::Empty),
        dsl::DslValue::Bool(flag) => Ok(XlsxCellValue::Boolean(*flag)),
        dsl::DslValue::Number(_) => value.as_f64().map(XlsxCellValue::Number).ok_or_else(|| export_error(format!("program->xlsx: number {value:?} is not representable as f64"))),
        dsl::DslValue::String(text) => Ok(XlsxCellValue::InlineString(text.clone())),
        dsl::DslValue::Bytes(_) | dsl::DslValue::Array(_) | dsl::DslValue::Object(_) => Ok(XlsxCellValue::InlineString(dsl::json::to_json_string(value))),
    }
}

pub fn serialize(snapshot: &ProgramSnapshot) -> Result<XlsxSnapshot, semio_framework_diagnostic::TextError> {
    let tables = crate::io::program_export_tables(snapshot).map_err(export_error)?;
    let mut sheets = Vec::with_capacity(tables.len());
    for table in tables {
        let columns: Vec<String> = table.rows.iter().flat_map(|row| row.iter().map(|(key, _)| key.clone())).collect::<BTreeSet<_>>().into_iter().collect();
        let mut cells = Vec::with_capacity(columns.len().saturating_mul(table.rows.len().saturating_add(1)));
        for (col, name) in columns.iter().enumerate() {
            cells.push(XlsxCell { row: 1, col: u32::try_from(col).map_err(|_| export_error("program->xlsx: too many columns"))?, value: XlsxCellValue::InlineString(name.clone()) });
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            let row_number = u32::try_from(row_index + 2).map_err(|_| export_error("program->xlsx: too many rows"))?;
            for (col, name) in columns.iter().enumerate() {
                let value = row.iter().find(|(key, _)| key == name).map(|(_, value)| cell_value(value)).transpose()?.unwrap_or(XlsxCellValue::Empty);
                cells.push(XlsxCell { row: row_number, col: u32::try_from(col).map_err(|_| export_error("program->xlsx: too many columns"))?, value });
            }
        }
        sheets.push(XlsxSheet { name: table.name.into(), cells });
    }
    Ok(build_minimal_xlsx(XlsxWorkbook { sheets, shared_strings: Vec::new() }))
}

pub fn serialize_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(<XlsxSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}

pub fn serialize_raw_bytes(snapshot: &ProgramSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    let workbook = serialize(snapshot)?;
    semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx(&workbook).map_err(|error| export_error(format!("program->xlsx: {error}")))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

```

## ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs

```rust
//! 🏛️ program ← csv — a register table (`register,id,name,status,priority,tags,source`, any RFC 4180
//! producer) read by stdio's own csv codec into a fresh program through the editor's
//! `import_registers_csv` with `MergeStrategy::Replace`; a table with any other header is refused.
//!
//! 🔖 `IoFidelity::Lossy`: the inverse of the sibling export — register rows only.
use crate::editor::architect::behavior::{import_registers_csv, MergeStrategy};
use crate::schema::snapshot::ProgramSnapshot;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<ProgramSnapshot, semio_framework_diagnostic::TextError> {
    let error = |message: String| semio_framework_diagnostic::TextError::new(format!("program←csv: {message}"), semio_framework_diagnostic::TextSpan::at(1, 1));
    let text = std::str::from_utf8(bytes).map_err(|e| error(e.to_string()))?;
    let mut program = ProgramSnapshot::default();
    import_registers_csv(&mut program, text, MergeStrategy::Replace).map_err(|e| error(e.to_string()))?;
    Ok(program)
}

```

## ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🧬️schema/🧬️mutations/📝️text/🦀️.rs

```rust
//! 📝️ Generic text framing and direct-owner registry for the visible PDF/E mutation aggregate.

use super::PdfEMutation;
use protocol::OpText;

//#region 🧾️DerivedRegistry
/// 🧾️ Direct-owner text opcodes in aggregate declaration order.
pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[
    ("InsertEncryptionDictionary", super::insert_encryption_dictionary::text::TEXT_OPCODE),
    ("RemoveEncryptionDictionary", super::remove_encryption_dictionary::text::TEXT_OPCODE),
    ("InsertJavascriptAction", super::insert_javascript_action::text::TEXT_OPCODE),
    ("RemoveJavascriptAction", super::remove_javascript_action::text::TEXT_OPCODE),
    ("InsertLaunchAction", super::insert_launch_action::text::TEXT_OPCODE),
    ("RemoveLaunchAction", super::remove_launch_action::text::TEXT_OPCODE),
    ("InsertMediaAnnotation", super::insert_media_annotation::text::TEXT_OPCODE),
    ("RemoveMediaAnnotation", super::remove_media_annotation::text::TEXT_OPCODE),
    ("SetOutputIntent", super::set_output_intent::text::TEXT_OPCODE),
    ("RemoveOutputIntent", super::remove_output_intent::text::TEXT_OPCODE),
    ("EmbedFontFile", super::embed_font_file::text::TEXT_OPCODE),
    ("RemoveFontFile", super::remove_font_file::text::TEXT_OPCODE),
];
//#endregion 🧾️DerivedRegistry

//#region 🧱️Framing
const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn decode_hex(value: &str) -> Result<Vec<u8>, String> {
    if !value.len().is_multiple_of(2) || value.len() > MAX_PAYLOAD_BYTES * 2 {
        return Err("PDF/E mutation text payload exceeds its budget".into());
    }
    fn nibble(value: u8) -> Option<u8> {
        if value.is_ascii_digit() {
            return Some(value - b'0');
        }
        (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10)
    }
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let high = nibble(pair[0]).ok_or_else(|| "PDF/E mutation payload must be lowercase hexadecimal".to_string())?;
            let low = nibble(pair[1]).ok_or_else(|| "PDF/E mutation payload must be lowercase hexadecimal".to_string())?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn text_error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl OpText for PdfEMutation {
    fn print_op(&self) -> String {
        let payload = semio_framework_pack_json::to_json_string(self).into_bytes();
        format!("pdf-e-mutation payload={}", encode_hex(&payload))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix("pdf-e-mutation payload=").ok_or_else(|| text_error("expected canonical PDF/E mutation aggregate"))?;
        let bytes = decode_hex(payload).map_err(text_error)?;
        let parsed = semio_framework_pack_json::parse_bytes(&bytes, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| text_error(error.to_string()))?;
        dsl::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| text_error(error.to_string()))
    }
}
//#endregion 🧱️Framing

```


## Direct TextError Forwarding Correction

The two imported JSON text parsers now return their existing TextError directly. The initial authored forwarding expression retained a callee qualification and was corrected before any native compiler attempt by removing the projection entirely. Full immediate original/inverse/authored sources are captured in generated/value-refusal/text-error-higher-direct-text-authored-1.json; independent Rust grammar2/2 is clean and exact post hashes have zero gaps.
