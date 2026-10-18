use super::*;

use db_engine_base::{Consistency, DATABASE_DOCUMENT_MOUNT_WAITERS, DatabaseDocumentMountDriver, DatabaseDocumentMountRegistry, DatabaseDocumentMountSlot, DatabaseDocumentMountWork, LiveQuerySpec, Query, QueryResultEntry, QueryStream, SnapshotKind, now_ms, test_worker_pool, to_core_document_id};
use db_engine_handle::{ARTIFACT_HISTORY_ADMISSION, ARTIFACT_HISTORY_OPERATION_SLOTS, ArtifactHandle, ArtifactHistoryAdmission, ArtifactHistoryAdmissionError, ArtifactHistoryState, HistoryFuture};
use db_io_core::{PayloadStorage as _, WalStorage as _};

async fn rejected_document_open_error(mut rejected: DatabaseDocumentOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

#[derive(Default)]
struct ControlledMountEmitState {
    entered: bool,
    pub released: bool,
    events: Vec<EmitEvent>,
}

#[derive(Clone, Default)]
struct ControlledMountEmit {
    pub state: Arc<(Mutex<ControlledMountEmitState>, std::sync::Condvar)>,
}

impl ControlledMountEmit {
    fn wait_until_document_event(&self) {
        let (state, signal) = &*self.state;
        let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !state.entered {
            state = signal.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    fn release_document_event(&self) {
        let (state, signal) = &*self.state;
        state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).released = true;
        signal.notify_all();
    }

    fn document_event_count(&self) -> usize {
        self.state.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).events.iter().filter(|event| event.document.is_some()).count()
    }
}

impl Emit for ControlledMountEmit {
    async fn emit(&self, event: EmitEvent) {
        let (state, signal) = &*self.state;
        let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if event.document.is_some() {
            state.entered = true;
            signal.notify_all();
            while !state.released {
                state = signal.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }
        state.events.push(event);
        signal.notify_all();
    }
}

struct MountFanoutLockProbe {
    pub registry: Arc<Mutex<DatabaseDocumentMountRegistry>>,
    called: std::sync::atomic::AtomicBool,
    unlocked: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for MountFanoutLockProbe {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.unlocked.store(self.registry.try_lock().is_ok(), std::sync::atomic::Ordering::Release);
        self.called.store(true, std::sync::atomic::Ordering::Release);
    }
}

async fn decode_query_json(mut stream: QueryStream) -> serde_json::Value {
    let mut bytes = Vec::new();
    for fragment in stream.get(0).and_then(QueryResultEntry::value).expect("retained query value").fragments() {
        bytes.extend_from_slice(fragment);
    }
    let value = db_artifact_contract::decode_pathmap_json(&bytes).await.unwrap();
    while stream.close_step().unwrap() {}
    assert!(stream.terminal_is_empty());
    value
}
//#region 🧸️Fixtures
async fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!("db_engine-test-{name}-{}-{}", std::process::id(), now_ms().await));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

async fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::ArtifactId, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let mut payload = serde_json::Map::new();
    for (path, value) in entries {
        payload.insert((*path).to_string(), value.clone());
    }
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(serde_json::Map::new())).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}

/// 📏️ C12 P1 live (hub 8010 on p24: a 600-envelope outbox answered `hub.unavailable` forever): the largest batch the document
/// backbone declares legal (the wire's own exact decoder admits it) commits through a production-profile `Database` →
/// `ArtifactHandle::submit` as one transaction (the hub's socket judges the declared bytes), and a batch of one envelope more
/// than the declared count is refused PERMANENTLY (`LimitExceeded`), never as the transient `Unavailable` a client would resend
/// without end.
#[semio_framework_async_macros::async_test]
async fn a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently() {
    if !db_io_driver::process_isolated_law("tests::a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("declared-maximal-batch").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
    let document = protocol::ArtifactId("declared-maximal".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let mut envelopes = Vec::new();
    let mut bytes = 0usize;
    let next = loop {
        let next = envelope(&format!("{:x}", envelopes.len()), &[], "a", &document, &[]).await;
        let mut encoded = Vec::new();
        protocol::encode_envelope(&next, &mut encoded);
        let count_bytes = if envelopes.len() + 1 < 128 { 1 } else if envelopes.len() + 1 < 16_384 { 2 } else { 3 };
        if envelopes.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
            break next;
        }
        bytes += encoded.len();
        envelopes.push(next);
    };
    assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&envelopes)).is_ok(), "the batch is declared legal by the wire's own decoder");
    let mut beyond = envelopes.clone();
    beyond.push(next);
    assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&beyond)).is_err(), "one more envelope leaves the declaration");
    let mut over = Vec::with_capacity(protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + 1);
    for index in 0..=protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES {
        over.push(envelope(&format!("over-{index:x}"), &[], "a", &document, &[]).await);
    }
    assert!(envelopes.len() > 256, "the declared-maximal batch ({} envelopes) exceeds the former 256-envelope submit credit", envelopes.len());
    let count = envelopes.len() as u64;
    let receipt = db_actor::block_on(handle.submit(db_artifact_contract::CommandBatch::new(envelopes).await.unwrap(), db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    assert_eq!(receipt.frontier.head_seq, count, "the whole declared batch commits");
    let refused = db_actor::block_on(handle.submit(db_artifact_contract::CommandBatch { envelopes: over }, db_artifact_contract::SubmitOptions::default())).and_then(std::convert::identity);
    assert!(matches!(refused, Err(DbError::LimitExceeded(_))), "a batch beyond the declaration is a permanent refusal: {refused:?}");
    assert_eq!(handle.frontier().await.unwrap().head_seq, count, "the refused batch applied nothing");
}
//#endregion 🧸️Fixtures
//#region 🔖️Database open/catalog
#[semio_framework_async_macros::async_test]
async fn database_create_catalog_durable_publication_precedes_authority_spawn_emit_and_registration() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let database = Database::open(test_worker_pool(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let transaction = database.create_document_catalog_retained(protocol::ArtifactId(String::from("durable-first"))).unwrap();
    let (_, document, _, actual) = transaction.await.unwrap().into_parts().unwrap();
    assert!(actual.is_ok());
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert_eq!(database.catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entries.last().map(|entry| &entry.document), Some(&document));
}

#[semio_framework_async_macros::async_test]
async fn database_concurrent_ensure_mounts_one_actor_and_one_writer() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-empty"));
    let mut first_future = Box::pin(database.ensure_document(&document));
    let mut second_future = Box::pin(database.ensure_document(&document));
    let mut first_result = None;
    let mut second_result = None;
    let (first_result, second_result) = std::future::poll_fn(|context| {
        if first_result.is_none() {
            if let std::task::Poll::Ready(result) = first_future.as_mut().poll(context) {
                first_result = Some(result);
            }
        }
        if second_result.is_none() {
            if let std::task::Poll::Ready(result) = second_future.as_mut().poll(context) {
                second_result = Some(result);
            }
        }
        match (first_result.take(), second_result.take()) {
            (Some(first), Some(second)) => std::task::Poll::Ready((first, second)),
            (first, second) => {
                first_result = first;
                second_result = second;
                std::task::Poll::Pending
            }
        }
    })
    .await;
    drop(first_future);
    drop(second_future);
    let first = first_result.unwrap();
    let second = second_result.unwrap();
    assert!(Arc::ptr_eq(&first.authority, &second.authority));
    assert_eq!(database.catalog().await.artifacts.iter().filter(|entry| entry.document == document).count(), 1);
    let wal = storage.wal().await;
    let core = to_core_document_id(&document).await;
    assert!(matches!(wal.acquire_writer(&core).await, Err(DbError::Conflict(_))));
    drop(wal);
    drop(first);
    drop(second);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("no-closing-authority"));
        eprintln!(
            "concurrent mount shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after concurrent mount failed: {error}");
    }
    pool.shutdown().unwrap();
}

#[semio_framework_async_macros::async_test]
async fn database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack() {
    let pool = test_worker_pool();
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let storage = Arc::new(db_storage::DbBackend::Memory(memory));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    let document = protocol::ArtifactId(String::from("pool-use-mounted-authority"));
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(pool.shutdown(), Err(semio_framework_async::WorkerPoolShutdownError::Busy { retained_uses: 1 }));
    drop(handle);
    let unrelated = pool.acquire_use().unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    let rejected = match database.ensure_document(&document).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("terminal database admitted another mount")
        }
    };
    assert_eq!(rejected_document_open_error(rejected).await, DbError::Closed);
    assert!(matches!(database.create_document_catalog_retained(protocol::ArtifactId(String::from("post-terminal-catalog"))), Err(DatabaseRetainedActivityRejected::Closed(DbError::Closed))));
    assert!(matches!(
        database.hello_retained(to_core_document_id(&document).await, None, String::from("post-terminal-session"), protocol::ActorId(String::from("post-terminal-actor")), 4096,),
        Err(DatabaseRetainedActivityRejected::Closed(DbError::Closed))
    ));
    let (ran_tx, ran_rx) = std::sync::mpsc::sync_channel(1);
    pool.submit_at(pool.now_ms(), Lane::UserVisible, Box::new(move || ran_tx.send(()).unwrap()));
    ran_rx.recv().unwrap();
    drop(unrelated);
    assert_eq!(pool.shutdown(), Ok(()));
}

#[semio_framework_async_macros::async_test]
async fn database_worker_pool_use_is_admitted_before_the_first_storage_probe() {
    let pool = test_worker_pool();
    assert_eq!(pool.shutdown(), Ok(()));
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let storage = Arc::new(db_storage::DbBackend::Memory(memory));
    let error = match Database::open(pool, DbConfig::for_profile(Profile::Test), storage).await {
        Err(error) => error,
        Ok(mut database) => {
            let _ = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await;
            panic!("database opened on a stopped WorkerPool")
        }
    };
    assert!(matches!(error, DbError::Unavailable(detail) if detail.contains("WorkerPool use rejected")));
}

#[semio_framework_async_macros::async_test]
async fn database_published_opening_joins_without_actor_overwrite() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-published"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (published_tx, published_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    *database.mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |document| {
        published_tx.send(document.clone()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut first = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    assert_eq!(published_rx.recv().unwrap(), document);
    let mut second = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(second.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { waiters, .. } = registry.slots.get(&document.0).unwrap() else { panic!("published mount was not retained as Opening") };
        assert_eq!(waiters.iter().flatten().count(), 2);
    }
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let first = first.await.unwrap();
    let second = second.await.unwrap();
    assert!(Arc::ptr_eq(&first.authority, &second.authority));
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 1);
    drop(first);
    drop(second);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_cancelled_ensure_waiter_does_not_cancel_mount_owner() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-cancel"));
    let mut cancelled = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(cancelled.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    drop(cancelled);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { owner, waiters, .. } = registry.slots.get(&document.0).unwrap() else { panic!("cancelled waiter removed retained owner") };
        assert_eq!(waiters.iter().flatten().count(), 0);
        assert!(!owner.terminal.load(std::sync::atomic::Ordering::Acquire));
        eprintln!(
            "cancelled-waiter stage=owner-retained driver={} wake={} resume={}",
            owner.driver.load(std::sync::atomic::Ordering::Acquire),
            owner.wake_requested.load(std::sync::atomic::Ordering::Acquire),
            owner.resume_requested.load(std::sync::atomic::Ordering::Acquire)
        );
    }
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(database.catalog().await.artifacts.iter().filter(|entry| entry.document == document).count(), 1);
    drop(handle);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        eprintln!(
            "cancelled-waiter shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            registry.len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after cancelled waiter failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-emission"));
    let mut elected = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(elected.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    emit.wait_until_document_event();
    let mut survivor = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(survivor.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    assert_eq!(emit.document_event_count(), 0);
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 0);
    drop(elected);
    emit.release_document_event();
    let survivor = survivor.await.unwrap();
    assert_eq!(emit.document_event_count(), 1);
    let later = database.ensure_document(&document).await.unwrap();
    assert!(Arc::ptr_eq(&survivor.authority, &later.authority));
    assert_eq!(emit.document_event_count(), 1);
    drop(survivor);
    drop(later);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        eprintln!(
            "emission mount shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            registry.len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after controlled mount emission failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-capacity"));
    let mut waiters = Vec::new();
    for _ in 0..DATABASE_DOCUMENT_MOUNT_WAITERS {
        let mut waiter = Box::pin(database.ensure_document(&document));
        std::future::poll_fn(|context| {
            assert!(waiter.as_mut().poll(context).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        waiters.push(waiter);
    }
    emit.wait_until_document_event();
    let rejected = match database.ensure_document(&document).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("document mount admitted waiter 33")
        }
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Unavailable(_)));
    drop(waiters.remove(0));
    let mut replacement = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(replacement.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    waiters.push(replacement);
    emit.release_document_event();
    let mut handles = Vec::new();
    for waiter in waiters {
        handles.push(waiter.await.unwrap());
    }
    assert_eq!(handles.len(), DATABASE_DOCUMENT_MOUNT_WAITERS);
    assert!(handles.iter().all(|handle| Arc::ptr_eq(&handle.authority, &handles[0].authority)));
    assert_eq!(emit.document_event_count(), 1);
    drop(handles);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-reentrant-wake"));
    let mut mount = Box::pin(database.ensure_document(&document));
    let probe = Arc::new(MountFanoutLockProbe { registry: database.open_artifacts.clone(), called: std::sync::atomic::AtomicBool::new(false), unlocked: std::sync::atomic::AtomicBool::new(false) });
    let waker = std::task::Waker::from(probe.clone());
    let mut context = std::task::Context::from_waker(&waker);
    assert!(mount.as_mut().poll(&mut context).is_pending());
    emit.wait_until_document_event();
    emit.release_document_event();
    while !probe.called.load(std::sync::atomic::Ordering::Acquire) {
        semio_framework_async::yield_once().await;
    }
    assert!(probe.unlocked.load(std::sync::atomic::Ordering::Acquire));
    let handle = mount.await.unwrap();
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let emit = ControlledMountEmit::default();
    let mut database = Database::open_with_emit(pool.clone(), DbConfig::for_profile(Profile::Test), storage, Arc::new(emit.clone())).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-shutdown"));
    let mut cancelled_waiter = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(cancelled_waiter.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    emit.wait_until_document_event();
    drop(cancelled_waiter);
    let (generation, owner) = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, waiters } = registry.slots.get(&document.0).unwrap() else { panic!("mount owner escaped before emission") };
        assert_eq!(waiters.iter().flatten().count(), 0);
        (*generation, Arc::as_ptr(owner))
    };
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(30), cancelled.clone());
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Interrupted);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation: retained_generation, owner: retained_owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("interrupted shutdown lost mount owner") };
        assert_eq!((*retained_generation, Arc::as_ptr(retained_owner)), (generation, owner));
    }
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    assert!(matches!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, .. }));
    emit.release_document_event();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count() != 1 {
        assert!(std::time::Instant::now() < deadline, "resumed mount did not publish its ready authority");
        semio_framework_async::yield_once().await;
    }
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(emit.document_event_count(), 1);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn open_at_creates_a_fresh_zero_touch_database_with_an_empty_catalog() {
    let root = tempdir("open-at-fresh").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    assert!(database.catalog().await.artifacts.is_empty());
    assert_eq!(database.health().await.open_artifacts, 0);
    assert!(matches!(database.health().await.report.overall, db_observe::HealthState::Healthy));
}

#[semio_framework_async_macros::async_test]
async fn create_document_registers_it_in_the_catalog_and_document_finds_it() {
    let root = tempdir("create-and-find").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let catalog = database.catalog().await;
    assert_eq!(catalog.artifacts.len(), 1);
    assert_eq!(catalog.artifacts[0].document, document);

    let handle = database.document(&document).await.unwrap();
    assert_eq!(handle.document_id().await, &document);
}

#[semio_framework_async_macros::async_test]
async fn create_document_twice_errs_already_exists() {
    let root = tempdir("create-twice").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let result = database.create_document(ArtifactSpec::new(document).await);
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("duplicate document was admitted"),
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::AlreadyExists(_)));
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout() {
    let inner = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(test_worker_pool()).await.unwrap()));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(test_worker_pool(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_write: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId("database-retained-open-rejection".to_string());
    let rejected = match database.create_document(ArtifactSpec::new(document.clone()).await).await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("faulted database document builder was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(!rejected.has_retained_writer());
    let core_document = to_core_document_id(&document).await;
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    storage.wal().await.acquire_writer(&core_document).await.unwrap().release().await.unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation() {
    let pool = test_worker_pool();
    let inner = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_sync: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-retained-failure"));
    let mut first_future = Box::pin(database.ensure_document(&document));
    let mut second_future = Box::pin(database.ensure_document(&document));
    let mut first_result = None;
    let mut second_result = None;
    let (first_result, second_result) = std::future::poll_fn(|context| {
        if first_result.is_none() {
            if let std::task::Poll::Ready(result) = first_future.as_mut().poll(context) {
                first_result = Some(result);
            }
        }
        if second_result.is_none() {
            if let std::task::Poll::Ready(result) = second_future.as_mut().poll(context) {
                second_result = Some(result);
            }
        }
        match (first_result.take(), second_result.take()) {
            (Some(first), Some(second)) => std::task::Poll::Ready((first, second)),
            (first, second) => {
                first_result = first;
                second_result = second;
                std::task::Poll::Pending
            }
        }
    })
    .await;
    drop(first_future);
    drop(second_future);
    for result in [first_result, second_result] {
        let rejected = match result {
            Err(rejected) => rejected,
            Ok(handle) => {
                drop(handle);
                panic!("faulted mount was admitted")
            }
        };
        assert!(!rejected.has_retained_writer());
        assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    }
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    fault.set_script(db_storage::FaultScript::default()).await;
    let handle = database.ensure_document(&document).await.unwrap();
    assert_eq!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).ready_count(), 1);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume() {
    let pool = test_worker_pool();
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    memory.fail_next_writer_release();
    let inner = Arc::new(db_storage::DbBackend::Memory(memory));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_sync: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-unlock-fault"));
    let (parked_tx, parked_rx) = std::sync::mpsc::sync_channel(1);
    *database.mount_parked_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || parked_tx.send(()).unwrap()));
    let mut first = Box::pin(database.ensure_document(&document));
    let mut second = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        assert!(second.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    parked_rx.recv().unwrap();
    let exact = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, .. } = registry.slots.get(&document.0).expect("temporary unlock fault retains the mounted owner") else { panic!("temporary unlock fault replaced its opening slot") };
        let work = owner.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountWork::Parked { rejected, .. } = &*work else { panic!("temporary unlock fault did not park its retained owner") };
        assert!(rejected.has_retained_writer());
        assert!(matches!(rejected.cleanup_error(), Some(DbError::Unavailable(detail)) if detail.contains("injected WAL writer unlock failure")));
        (*generation, Arc::as_ptr(owner))
    };
    drop(first);
    drop(second);
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(30), cancelled.clone());
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Interrupted);
    {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { generation, owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("interrupted shutdown lost parked cleanup") };
        assert_eq!((*generation, Arc::as_ptr(owner)), exact);
        assert!(matches!(&*owner.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner), DatabaseDocumentMountWork::Parked { rejected, .. } if rejected.has_retained_writer()));
    }
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    assert!(matches!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, .. }));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty() {
        assert!(std::time::Instant::now() < deadline, "resumed parked mount cleanup did not retire its opening slot");
        semio_framework_async::yield_once().await;
    }
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    fault.set_script(db_storage::FaultScript::default()).await;
    let handle = database.ensure_document(&document).await.unwrap();
    drop(handle);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("none"));
        eprintln!(
            "unlock-retry shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after controlled unlock retry failed: {error}");
    }
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_coalesces_join_drives_without_shared_pool_starvation() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-poller-coalescing"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (published_tx, published_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    *database.mount_catalog_published_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move |_| {
        published_tx.send(()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut first = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(first.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    published_rx.recv().unwrap();
    let mut joins = Vec::new();
    for _ in 0..8 {
        let mut join = Box::pin(database.ensure_document(&document));
        std::future::poll_fn(|context| {
            assert!(join.as_mut().poll(context).is_pending());
            std::task::Poll::Ready(())
        })
        .await;
        joins.push(join);
    }
    let release_gate = gate.clone();
    pool.submit_at(
        pool.now_ms(),
        Lane::UserVisible,
        Box::new(move || {
            let (lock, ready) = &*release_gate;
            *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
            ready.notify_one();
        }),
    );
    let first = first.await.unwrap();
    for join in joins {
        let joined = join.await.unwrap();
        assert!(Arc::ptr_eq(&first.authority, &joined.authority));
        drop(joined);
    }
    drop(first);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once() {
    let pool = test_worker_pool();
    let memory = db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    memory.fail_next_writer_release();
    let inner = Arc::new(db_storage::DbBackend::Memory(memory));
    let fault = db_storage::FaultStorage::new(inner).await;
    let storage = Arc::new(db_storage::DbBackend::Fault(Box::new(fault)));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let db_storage::DbBackend::Fault(fault) = storage.as_ref() else { unreachable!() };
    fault.set_script(db_storage::FaultScript { fail_nth_sync: Some(1), ..db_storage::FaultScript::default() }).await;
    let document = protocol::ArtifactId(String::from("single-flight-racing-cleanup-resume"));
    let gate = Arc::new((Mutex::new(false), std::sync::Condvar::new()));
    let (fault_tx, fault_rx) = std::sync::mpsc::sync_channel(1);
    let hook_gate = gate.clone();
    let hook_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let hook_counted = hook_count.clone();
    *database.mount_cleanup_fault_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || {
        hook_counted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        fault_tx.send(()).unwrap();
        let (lock, ready) = &*hook_gate;
        let mut released = lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !*released {
            released = ready.wait(released).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }));
    let mut mount = Box::pin(database.ensure_document(&document));
    std::future::poll_fn(|context| {
        assert!(mount.as_mut().poll(context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    fault_rx.recv().unwrap();
    let owner = {
        let registry = database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let DatabaseDocumentMountSlot::Opening { owner, .. } = registry.slots.get(&document.0).unwrap() else { panic!("cleanup owner was not mounted") };
        owner.clone()
    };
    owner.drive();
    assert!(owner.resume_requested.load(std::sync::atomic::Ordering::Acquire));
    {
        let (lock, ready) = &*gate;
        *lock.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = true;
        ready.notify_one();
    }
    let rejected = match mount.await {
        Err(rejected) => rejected,
        Ok(handle) => {
            drop(handle);
            panic!("faulted mount was admitted")
        }
    };
    assert!(!rejected.has_retained_writer());
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::Io(_)));
    assert_eq!(hook_count.load(std::sync::atomic::Ordering::Acquire), 1);
    assert!(owner.terminal.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(owner.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseDocumentMountDriver::Terminal as u8);
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    let core = to_core_document_id(&document).await;
    storage.wal().await.acquire_writer(&core).await.unwrap().release().await.unwrap();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    pool.shutdown();
}

#[semio_framework_async_macros::async_test]
async fn document_of_an_unknown_id_errs_not_found() {
    let root = tempdir("unknown-doc").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let never_created = protocol::ArtifactId("never-created".to_string());
    let result = database.document(&never_created);
    let rejected = match result.await {
        Err(rejected) => rejected,
        Ok(_) => panic!("unknown document was admitted"),
    };
    assert!(matches!(rejected_document_open_error(rejected).await, DbError::NotFound(_)));
}
//#endregion 🔖️Database open/catalog
//#region 🔖️Round trip
#[semio_framework_async_macros::async_test]
async fn full_submit_durable_query_round_trip_over_a_real_document_authority() {
    if !db_io_driver::process_isolated_law("tests::full_submit_durable_query_round_trip_over_a_real_document_authority") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("round-trip").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();

    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))]).await]).await.unwrap();
    let receipt = db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    assert_eq!(receipt.command_id, protocol::MutationId("op-1".to_string()));
    assert_eq!(receipt.frontier.document, document);
    assert_eq!(receipt.frontier.head_seq, 1);
    assert!(receipt.conflicts.is_empty());
    assert!(receipt.state_hash.is_some());

    let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).await.unwrap();
    let value = decode_query_json(queried).await;
    assert_eq!(value, serde_json::json!("hello"));

    let frontier = handle.frontier().await.unwrap();
    assert_eq!(frontier.head_seq, 1);
    let checkpoint_snapshot = handle.checkpoint_publication_snapshot().await.unwrap();
    assert_eq!(checkpoint_snapshot.authority_generation, GenerationId::INITIAL.0, "a never-restarted document authority publishes the initial supervision generation");
    assert_eq!(checkpoint_snapshot.frontier, frontier);
    assert_eq!(checkpoint_snapshot.head_edit_id, Some(protocol::MutationId("op-1".to_string())));

    let mut at_least = handle.query(Query::Get { path: "name".to_string() }, Consistency::AtLeast(frontier)).await.unwrap();
    assert_eq!(at_least.len(), 1);
    while at_least.close_step().unwrap() {}
    assert!(at_least.terminal_is_empty());

    let mut history = handle.history().await.unwrap();
    assert_eq!(history.entries().len(), 1);
    assert!(history.operation_id_eq(0, 0, "op-1"));
    while history.close_step() {}
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_empty_and_two_batch_replay_are_deterministic() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_empty_and_two_batch_replay_are_deterministic") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("history-order").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("history-doc".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let mut empty = handle.history().await.unwrap();
    assert!(empty.entries().is_empty());
    while empty.close_step() {}
    for id in ["history-1", "history-2"] {
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope(id, &[], "alice", &document, &[("value", serde_json::json!(id))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    }
    let mut first = handle.history().await.unwrap();
    let mut second = handle.history().await.unwrap();
    assert_eq!(first.entries(), second.entries());
    assert!(first.operation_id_eq(0, 0, "history-1"));
    assert!(first.operation_id_eq(1, 0, "history-2"));
    assert!(second.operation_id_eq(0, 0, "history-1"));
    assert!(second.operation_id_eq(1, 0, "history-2"));
    while first.close_step() {}
    while second.close_step() {}
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root() {
    let root = tempdir("reopen").await;
    let document = protocol::ArtifactId("doc-1".to_string());
    {
        let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("count", serde_json::json!(1))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
    }

    let reopened = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    assert_eq!(reopened.catalog().await.artifacts.len(), 1, "the catalog root must have survived the reopen");

    let handle = reopened.document(&document).await.unwrap();
    let queried = handle.query(Query::Get { path: "count".to_string() }, Consistency::Canonical).await.unwrap();
    let value = decode_query_json(queried).await;
    assert_eq!(value, serde_json::json!(1), "the document's committed state must have survived the reopen via WAL replay");
    assert_eq!(handle.frontier().await.unwrap().head_seq, 1);
    assert_eq!(handle.checkpoint_publication_snapshot().await.unwrap().head_edit_id, Some(protocol::MutationId("op-1".to_string())));
}

#[semio_framework_async_macros::async_test]
async fn exact_consistency_rejects_a_frontier_the_document_has_moved_past() {
    let root = tempdir("exact-consistency").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let stale = handle.frontier().await.unwrap();

    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();

    let result = handle.query(Query::Get { path: "x".to_string() }, Consistency::Exact(stale));
    assert!(matches!(result.await, Err(DbError::Unavailable(_))));
}
//#endregion 🔖️Round trip
//#region 🔖️Deferred extension seams
#[semio_framework_async_macros::async_test]
async fn subscribe_preview_and_snapshot_now_are_documented_unimplemented_not_panics() {
    let root = tempdir("deferred").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();

    assert!(matches!(handle.subscribe(LiveQuerySpec { since: None }).await, Err(DbError::Unimplemented(_))));
    assert!(matches!(handle.preview(handle.frontier().await.unwrap()).await, Err(DbError::Unimplemented(_))));
    assert!(matches!(db_actor::block_on(handle.snapshot_now(SnapshotKind::Full).await), Ok(Err(DbError::Unimplemented(_)))));
}
//#endregion 🔖️Deferred extension seams
//#region 🔖️WriterResidency
/// 🤝️ Polls every future of `futures` on the current task until all finished; outputs in order.
async fn join_every<F: Future>(futures: Vec<F>) -> Vec<F::Output> {
    let mut futures: Vec<std::pin::Pin<Box<F>>> = futures.into_iter().map(Box::pin).collect();
    let mut outputs: Vec<Option<F::Output>> = futures.iter().map(|_| None).collect();
    std::future::poll_fn(|context| {
        for (future, output) in futures.iter_mut().zip(outputs.iter_mut()) {
            if output.is_none() {
                if let std::task::Poll::Ready(value) = future.as_mut().poll(context) {
                    *output = Some(value);
                }
            }
        }
        if outputs.iter().all(Option::is_some) {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    })
    .await;
    outputs.into_iter().map(|output| output.expect("every joined future finished")).collect()
}

/// ⏱️ `future`'s output, or `None` once `deadline_ms` passed on `pool`'s clock first.
async fn within<F: Future + Unpin>(pool: &WorkerPool, deadline_ms: u64, mut future: F) -> Option<F::Output> {
    let mut deadline = std::pin::pin!(pool.timer().sleep_until(deadline_ms));
    std::future::poll_fn(|context| {
        if let std::task::Poll::Ready(output) = std::pin::Pin::new(&mut future).poll(context) {
            return std::task::Poll::Ready(Some(output));
        }
        deadline.as_mut().poll(context).map(|_| None)
    })
    .await
}

/// 🗂️ Opens `documents` documents and keeps every handle while `clients` clients commit to `documents / clients` of them
/// each, `in_flight` commits at a time, for `rounds` rounds; every commit must be acknowledged at its exact head within the
/// round's deadline. Like a hub client, a commit refused as transient (`Unavailable`) is resent after a short backoff on
/// the pool's timer; a round that misses its deadline fails with every transient refusal seen, the stuck submits' progress
/// and `census` (the backend's writer admission line).
async fn commit_rounds_through_writer_slots(pool: &WorkerPool, database: &Database, prefix: &str, documents: usize, clients: usize, in_flight: usize, rounds: usize, census: &dyn Fn() -> String) -> Vec<(protocol::ArtifactId, ArtifactHandle)> {
    const ROUND_DEADLINE_MS: u64 = 180_000;
    let mut handles = Vec::with_capacity(documents);
    for ordinal in 0..documents {
        let document = protocol::ArtifactId(format!("{prefix}-{ordinal:04}"));
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap_or_else(|rejected| panic!("document {ordinal} opens beside the others: {rejected:?}"));
        handles.push((document, handle));
    }
    for round in 1..=rounds {
        let refusals = std::sync::Mutex::new(std::collections::BTreeMap::<String, usize>::new());
        let deadline_ms = pool.now_ms() + ROUND_DEADLINE_MS;
        let mut outcomes = Vec::with_capacity(documents);
        for (wave, chunk) in handles.chunks(in_flight).enumerate() {
            let commits = chunk
                .iter()
                .enumerate()
                .map(|(offset, (document, handle))| {
                    let ordinal = wave * in_flight + offset;
                    let refusals = &refusals;
                    async move {
                        let edit = format!("{}-round-{round}", document.0);
                        let previous = format!("{}-round-{}", document.0, round - 1);
                        let dependencies: Vec<&str> = if round == 1 { Vec::new() } else { vec![previous.as_str()] };
                        loop {
                            let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&edit, &dependencies, &format!("client-{}", ordinal % clients), document, &[("round", serde_json::json!(round))]).await]).await.unwrap();
                            let mut submit = handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() });
                            let Some(outcome) = within(pool, deadline_ms, &mut submit).await else { return Err(format!("stuck in submit progress {:?}", submit.progress())) };
                            match outcome.and_then(std::convert::identity) {
                                Err(DbError::Unavailable(reason)) => {
                                    *refusals.lock().unwrap().entry(reason).or_default() += 1;
                                    if within(pool, deadline_ms, pool.timer().sleep_until(pool.now_ms() + 2)).await.is_none() {
                                        return Err("resending past the deadline".to_string());
                                    }
                                }
                                outcome => break outcome.map_err(|error| format!("refused: {error}")),
                            }
                        }
                    }
                })
                .collect();
            outcomes.extend(join_every(commits).await);
        }
        let failed: Vec<String> = handles.iter().zip(&outcomes).filter_map(|((document, _), outcome)| outcome.as_ref().err().map(|error| format!("{}: {error}", document.0))).collect();
        assert!(failed.is_empty(), "round {round}: {} of {documents} commits failed (first {:?}); transient refusals {:?}; writer admission {}", failed.len(), &failed[..failed.len().min(8)], refusals.lock().unwrap(), census());
        for ((document, _), outcome) in handles.iter().zip(outcomes) {
            let receipt = outcome.expect("checked commit");
            assert_eq!(receipt.frontier.head_seq, round as u64, "{} round {round} landed at its exact head (transient refusals {:?})", document.0, refusals.lock().unwrap());
        }
    }
    handles
}

/// 🗂️ Open documents are neither writers nor maintenance hooks (declared `residency`), on the filesystem backend where a
/// writer is a cross-process sidecar lock: 72 documents stay open at once — more than twice the writer slots of their
/// backend and more than the maintenance hooks of their pool — while 8 clients commit to 9 of them each, 8 commits in flight
/// at a time, for 2 rounds, so nearly every commit is lent a writer another document parked. Every commit is acknowledged at
/// its exact head, every lent lock re-verified against its segment, and a reopened database replays every document at its
/// last head. (The process's resident state — one DB I/O operation per state value — bounds how many documents and commits
/// may be in flight at once; that ceiling is set C's.)
#[semio_framework_async_macros::async_test]
async fn open_filesystem_documents_beyond_the_writer_slots_commit_concurrently_and_replay() {
    if !db_io_driver::process_isolated_law("tests::open_filesystem_documents_beyond_the_writer_slots_commit_concurrently_and_replay") {
        return;
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🗄️storage/🔐️writer/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let residency = &fixture["residency"];
    let count = |key: &str| residency[key].as_u64().unwrap() as usize;
    let (documents, clients, in_flight, rounds) = (count("documents"), count("clients"), count("inFlight"), count("rounds"));
    assert!(documents > 2 * db_io_core::WAL_WRITER_CAPACITY && documents > semio_framework_async::WORKER_MAINTENANCE_CAPACITY && documents % clients == 0);
    let root = tempdir("writer-residency-fs").await;
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Fs(db_storage_fs::FsStorage::open(pool.clone(), &root).await.unwrap()));
    let db_storage::DbBackend::Fs(fs) = storage.as_ref() else { unreachable!("filesystem residency backend") };
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let handles = commit_rounds_through_writer_slots(&pool, &database, "residency-fs", documents, clients, in_flight, rounds, &|| format!("{:?}", fs.writer_admission_census())).await;
    drop(handles);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(120))).await.unwrap();
    drop(database);
    storage.close().await.unwrap();
    drop(storage);
    let reopened_storage = Arc::new(db_storage::DbBackend::Fs(db_storage_fs::FsStorage::open(pool.clone(), &root).await.unwrap()));
    let mut reopened = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), reopened_storage.clone()).await.unwrap();
    for ordinal in 0..documents {
        let document = protocol::ArtifactId(format!("residency-fs-{ordinal:04}"));
        let handle = reopened.ensure_document(&document).await.unwrap_or_else(|rejected| panic!("{} reopens: {rejected:?}", document.0));
        assert_eq!(handle.frontier().await.unwrap().head_seq, rounds as u64, "{} replays every round", document.0);
    }
    reopened.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(120))).await.unwrap();
    drop(reopened);
    reopened_storage.close().await.unwrap();
    drop(reopened_storage);
    pool.shutdown().unwrap();
    let _ = std::fs::remove_dir_all(root);
}
//#endregion 🔖️WriterResidency
//#region 🔖️Compact + Sync
#[semio_framework_async_macros::async_test]
async fn compact_document_runs_a_real_compaction_pass_without_error() {
    let root = tempdir("compact").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();

    let report = database.compact_document(&document, "holder-1", false).await.unwrap();
    assert_eq!(report.wal_segments_deleted, 0, "nothing is below the (nonexistent) snapshot floor yet, but the pass itself must succeed");
}

#[semio_framework_async_macros::async_test]
async fn compact_document_uses_live_actor_writer_and_restores_submits() {
    let root = tempdir("compact-live-writer").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("compact-live-writer".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let first = db_artifact_contract::CommandBatch::new(vec![envelope("compact-before", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(first, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    let fill_ids: Vec<String> = (0..12).map(|index| format!("compact-fill-{index}")).collect();
    for (index, mutation_id) in fill_ids.iter().enumerate() {
        let dependency = if index == 0 { "compact-before" } else { fill_ids[index - 1].as_str() };
        let value = format!("{index}:{}", "x".repeat(50_000));
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope(mutation_id, &[dependency], "alice", &document, &[("x", serde_json::json!(value))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    }
    handle.authority.snapshot_now(100).await.unwrap();

    let core_document = to_core_document_id(&document).await;
    let storage = database.storage().await;
    let mut before = storage.wal().await.list_segments(&core_document).await.unwrap();
    assert!(before.len() >= 2, "bounded public submissions must leave a sealed predecessor and active successor");
    let active = *before.last().unwrap();
    while before.close_step() {}
    assert!(before.terminal_is_empty());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "the live actor must retain its original writer before compaction");
    let compacted = database.compact_document(&document, "actor-holder", false).await;
    let report = compacted.unwrap();
    assert!(report.wal_segments_deleted >= 1);
    assert_eq!(report.payloads_deleted, 0, "document-scoped maintenance must not reclaim global CAS bytes");
    let mut after = storage.wal().await.list_segments(&core_document).await.unwrap();
    assert!(after.as_slice().contains(&active), "the actor WAL's authoritative active segment must survive compaction");
    while after.close_step() {}
    assert!(after.terminal_is_empty());
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "compaction must return the same retained writer to the actor");

    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    assert!(matches!(handle.compact("cancelled-holder", false, cancelled).await, Err(DbError::Closed)));
    let second = db_artifact_contract::CommandBatch::new(vec![envelope("compact-after", &["compact-fill-11"], "alice", &document, &[("x", serde_json::json!(2))]).await]).await.unwrap();
    let receipt = db_actor::block_on(handle.submit(second, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    assert_eq!(receipt.frontier.head_seq, 14);
    assert!(matches!(storage.wal().await.acquire_writer(&core_document).await, Err(DbError::Conflict(_))), "cancelled maintenance must restore the engine without releasing its writer");
    drop(storage);
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(1))).await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_cancellation_preserves_exact_retry_owners() {
    let root = tempdir("shutdown-retry-owner").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("shutdown-retry-owner".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("shutdown-seed", &[], "owner", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();
    drop(handle);
    let closing = |database: &Database| matches!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(&document.0), Some(DatabaseDocumentMountSlot::Closing { .. }));
    assert!(closing(&database), "the last handle unmounts its document into a closing slot");

    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let control = DatabaseShutdownControl::new(std::time::Instant::now() + std::time::Duration::from_secs(5), cancelled.clone());
    assert_eq!(database.shutdown(&control).await, Err(DbError::Closed));
    assert!(closing(&database), "cancelled shutdown retains the closing slot");
    assert!(database.closing_authority.is_none(), "an unmounted document retires through its closing slot");
    cancelled.store(false, std::sync::atomic::Ordering::Release);
    let drain_deadline = std::time::Instant::now() + std::time::Duration::from_secs(4);
    while std::time::Instant::now() < drain_deadline {
        database.shutdown_step(&control).await.unwrap();
        if database.closing_authority.is_none() && database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty() {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert!(database.closing_authority.is_none());
    database.shutdown(&control).await.expect("the retried shutdown completes");
    assert!(database.shutdown_complete);
}

#[semio_framework_async_macros::async_test]
async fn database_shutdown_shared_authority_blocks_without_closing_live_handle() {
    let root = tempdir("shutdown-shared-owner").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("shutdown-shared-owner".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();
    let control = DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5));
    assert_eq!(database.shutdown_step(&control).await.unwrap(), DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities(1)));
    assert_eq!(handle.frontier().await.unwrap().head_seq, 0);
    drop(handle);
    database.shutdown(&control).await.expect("released shared authority permits terminal shutdown");
}

#[semio_framework_async_macros::async_test]
async fn hello_returns_a_welcome_with_a_fresh_bootstrap_for_a_brand_new_replica() {
    let root = tempdir("hello").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("doc-1".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions::default())).unwrap().unwrap();

    let mut session = database.hello(document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string()), 4096).await.unwrap();
    let welcome = session.take_welcome().unwrap();
    assert!(matches!(welcome.frame().unwrap(), protocol::ServerFrame::Welcome { .. }));
    welcome.acknowledge().unwrap();
}

/// 🚪️ The hub's document socket drives exactly this: welcome acknowledged, every bootstrap frame
/// acknowledged until the session ends, the session dropped with the socket — or dropped right
/// after the welcome when the socket dies early, so the close request races the returned frame's
/// retirement on the shared I/O lane. Every hello must retire: a retained one keeps its
/// `WorkerPoolUse`, and with it a stopping hub's `Database` and WAL writers, past every deadline.
#[semio_framework_async_macros::async_test]
async fn hello_sessions_retire_when_drained_and_when_close_races_the_returned_frame() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId("doc-hello-retirement".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    handle.submit(batch, db_artifact_contract::SubmitOptions::default()).await.unwrap().unwrap();
    for round in 0..8 {
        let mut session = database.hello(document.clone(), None, format!("session-{round}"), protocol::ActorId("semio_hub".to_string()), 64 * 1024).await.unwrap();
        session.take_welcome().unwrap().acknowledge().unwrap();
        if round % 2 == 0 {
            while let Some(frame) = session.next_frame().await.unwrap() {
                frame.acknowledge().unwrap();
            }
        }
        let witness = session.retirement_witness();
        drop(session);
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while !witness.retired() && std::time::Instant::now() < deadline {
            semio_framework_async::yield_once().await;
        }
        assert!(witness.retired(), "hello session {round} retired: {}", witness.describe());
    }
    drop(handle);
}

/// 🎟️ More hellos than the process has admission slots are all welcomed: the retained submission
/// still refuses the one beyond capacity (its owners handed back), but [`Database::hello`] waits for
/// a slot and is welcomed as soon as one is released — two dozen document sockets reopening
/// together after a hub restart used to see `database sync hello admission saturated` (ticket
/// 26/09/23 H9 session 12, growth e2e g15).
#[semio_framework_async_macros::async_test]
async fn hellos_beyond_the_admission_slots_wait_for_a_slot_instead_of_refusing() {
    if !db_io_driver::process_isolated_law("tests::hellos_beyond_the_admission_slots_wait_for_a_slot_instead_of_refusing") {
        return;
    }
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let document = protocol::ArtifactId("doc-hello-admission".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let batch = db_artifact_contract::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("x", serde_json::json!(1))]).await]).await.unwrap();
    handle.submit(batch, db_artifact_contract::SubmitOptions::default()).await.unwrap().unwrap();
    let origin = protocol::ActorId("semio_hub".to_string());
    let mut held = Vec::new();
    for slot in 0..8 {
        let mut session = database.hello(document.clone(), None, format!("held-{slot}"), origin.clone(), 64 * 1024).await.unwrap();
        session.take_welcome().unwrap().acknowledge().unwrap();
        held.push(session);
    }
    let core = to_core_document_id(&document).await;
    match database.hello_retained(core, None, "retained-ninth".to_string(), origin.clone(), 64 * 1024) {
        Err(DatabaseRetainedActivityRejected::Retained(rejected)) => {
            assert!(rejected.admission_saturated(), "the retained ninth hello is refused for capacity alone");
            drop(rejected.close_and_take_error());
        }
        Err(DatabaseRetainedActivityRejected::Closed(error)) => panic!("database closed: {error:?}"),
        Ok(_) => panic!("the retained ninth hello was admitted beyond the declared slots"),
    }
    let mut ninth = std::pin::pin!(database.hello(document.clone(), None, "waiting-ninth".to_string(), origin.clone(), 64 * 1024));
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..1_000 {
        assert!(ninth.as_mut().poll(&mut context).is_pending(), "the ninth hello waits while every slot is held");
        if db_sync::database_sync_hello_admission_waiters() == 1 {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    assert_eq!(db_sync::database_sync_hello_admission_waiters(), 1, "the ninth hello is registered as a waiter");
    drop(held.pop());
    let mut session = ninth.await.expect("the ninth hello is welcomed once a slot is released");
    session.take_welcome().unwrap().acknowledge().unwrap();
    assert_eq!(db_sync::database_sync_hello_admission_waiters(), 0, "a welcomed hello leaves the waiter table");
    drop(session);
    drop(held);
    drop(handle);
}

// 🔬️ `storage()` is a real escape hatch to the same backend `Database::open_at` wired — a
// caller below the document-actor boundary (os-semio_hub's blob routes) can round-trip a payload
// through it directly, independent of any document actor.
#[semio_framework_async_macros::async_test]
async fn storage_accessor_reaches_the_same_backend_payload_store() {
    let root = tempdir("storage-accessor").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let hash = db_actor::block_on(async {
        let pages = db_io_registry::db_io_copy_pages(b"hello storage accessor").unwrap().await.unwrap();
        database.storage().await.payload().await.put(pages).await
    })
    .unwrap();
    assert_eq!(db_actor::block_on(async { database.storage().await.payload().await.get(&hash).await }).unwrap(), b"hello storage accessor");
}
//#endregion 🔖️Compact + Sync
//#region 🔖️Retained submit authority
fn retire_history_admission(mut admission: ArtifactHistoryAdmission) {
    let mut cursor = admission.begin_reservation_close().expect("history test admission retained its reservation");
    for _ in 0..50_000 {
        if cursor.terminal_is_empty() {
            break;
        }
        assert!(cursor.close_step());
    }
    assert!(cursor.terminal_is_empty());
}

struct ControlledHistoryPublicWake {
    pub state: std::sync::Weak<ArtifactHistoryState>,
    wakes: std::sync::atomic::AtomicUsize,
    lock_released: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for ControlledHistoryPublicWake {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Some(state) = self.state.upgrade() {
            self.lock_released.store(state.waker.try_lock().is_ok(), std::sync::atomic::Ordering::Release);
        }
        self.wakes.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_completion_interleavings_preserve_result_and_wake() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_completion_interleavings_preserve_result_and_wake") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    use std::sync::atomic::Ordering;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/📜️history-completion/🔣️.json")).unwrap();
    let root = tempdir("history-publication").await;
    std::fs::create_dir_all(&root).unwrap();
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId(fixture["document"].as_str().unwrap().into());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    for operation in fixture["operations"].as_array().unwrap() {
        let id = operation.as_str().unwrap();
        let batch = db_artifact_contract::CommandBatch::new(vec![envelope(id, &[], "history-fixture", &document, &[("value", serde_json::json!(id))]).await]).await.unwrap();
        db_actor::block_on(handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..db_artifact_contract::SubmitOptions::default() })).unwrap().unwrap();
    }
    let mut observations = Vec::new();
    for row in fixture["publication"].as_array().unwrap() {
        let mut probe = HistoryFuture::prepare(&handle);
        let state = probe.state.clone();
        let terminal = probe.terminal_handle();
        let generation = probe.generation();
        let admission_slot = state.admission.lock().unwrap().as_ref().unwrap().slot;
        let (captured, capture) = std::sync::mpsc::sync_channel(1);
        *state.controlled_completion_hook.lock().unwrap() = Some(Box::new(move |result, progress| {
            assert!(captured.send((result, progress)).is_ok(), "physical history result capture must remain live");
        }));
        let cancelled = row["outcome"] == "cancelled";
        if cancelled {
            probe.cancel();
        } else {
            state.schedule();
        }
        let (result, progress) = capture.recv_timeout(std::time::Duration::from_secs(5)).expect("actual history producer completion deadline");
        let entry_pointer = result.as_ref().ok().map(|view| view.entries().as_ptr() as usize);
        if let Ok(view) = result.as_ref() {
            assert_eq!(view.entries().len(), fixture["operations"].as_array().unwrap().len());
            assert_eq!(view.admission.as_ref().unwrap().generation, generation);
            assert!(Arc::ptr_eq(view.terminal_state.as_ref().unwrap(), &state));
        }
        assert!(ARTIFACT_HISTORY_ADMISSION.lock().unwrap().slots[admission_slot].occupied);
        let publish_state = state.clone();
        let publish = move || publish_state.complete(result, progress);
        let mut deferred = None;
        match row["publication"].as_str().unwrap() {
            "before-poll" => publish(),
            "before-registration" => *state.controlled_publication_before_waker_hook.lock().unwrap() = Some(Box::new(publish)),
            "after-pending" => deferred = Some(publish),
            _ => unreachable!(),
        }
        let wake = Arc::new(ControlledHistoryPublicWake { state: Arc::downgrade(&state), wakes: std::sync::atomic::AtomicUsize::new(0), lock_released: std::sync::atomic::AtomicBool::new(true) });
        let waker = std::task::Waker::from(wake.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let first = std::pin::Pin::new(&mut probe).poll(&mut context);
        let first_ready = first.is_ready();
        if let Some(publish) = deferred {
            publish();
        }
        let wake_count = wake.wakes.load(Ordering::Acquire);
        let wake_lock_released = wake.lock_released.load(Ordering::Acquire);
        let outcome = match first {
            std::task::Poll::Ready(result) => result,
            std::task::Poll::Pending => match std::pin::Pin::new(&mut probe).poll(&mut context) {
                std::task::Poll::Ready(result) => result,
                _ => panic!("physically published history result must remain consumable"),
            },
        };
        let waiter_empty = state.waker.lock().unwrap().is_none();
        let exact_result = match outcome {
            Ok(mut view) => {
                assert!(!cancelled);
                let exact = Some(view.entries().as_ptr() as usize) == entry_pointer
                    && view.admission.as_ref().is_some_and(|owner| owner.slot == admission_slot && owner.generation == generation)
                    && view.terminal_state.as_ref().is_some_and(|owner| Arc::ptr_eq(owner, &state))
                    && fixture["operations"].as_array().unwrap().iter().enumerate().all(|(index, operation)| view.operation_id_eq(index, 0, operation.as_str().unwrap()));
                for _ in 0..50_000 {
                    if view.terminal_is_empty() {
                        break;
                    }
                    assert!(view.close_step(), "actual replay result must make bounded retirement progress");
                }
                assert!(view.terminal_is_empty(), "history fixture must close every actual replay owner");
                exact
            }
            Err(error) => cancelled && error == DbError::Closed,
        };
        drop(probe);
        for _ in 0..50_000 {
            if terminal.terminal_is_empty() {
                break;
            }
            if !terminal.close_step() {
                std::thread::yield_now();
            }
        }
        let admission_released = !ARTIFACT_HISTORY_ADMISSION.lock().unwrap().slots[admission_slot].occupied;
        let registry_empty = handle.history_terminal(generation).is_none();
        assert!(terminal.terminal_is_empty(), "history fixture cleanup must finish its exact terminal state");
        eprintln!(
            "history-completion: {} first-ready={first_ready} wakes={wake_count} waiter-empty={waiter_empty} wake-lock-released={wake_lock_released} exact-result={exact_result} admission-released={admission_released} registry-empty={registry_empty}",
            row["name"]
        );
        observations.push((row.clone(), first_ready, wake_count, waiter_empty, exact_result, admission_released, registry_empty, wake_lock_released));
    }
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    for (row, first_ready, wakes, waiter_empty, exact_result, admission_released, registry_empty, wake_lock_released) in observations {
        assert_eq!(first_ready, row["firstReady"].as_bool().unwrap(), "history completion in registration gap must be observed in the same poll: {}", row["name"]);
        assert_eq!(wakes as u64, row["wakes"].as_u64().unwrap(), "{}", row["name"]);
        assert_eq!(waiter_empty, row["waiterEmpty"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(wake_lock_released, row["wakeLockReleased"].as_bool().unwrap(), "wake callback must not execute under the waiter mutex: {}", row["name"]);
        assert_eq!(exact_result, row["exactResult"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(admission_released, row["admissionReleased"].as_bool().unwrap(), "{}", row["name"]);
        assert_eq!(registry_empty, row["registryEmpty"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty() {
    if !db_io_driver::process_isolated_law("tests::artifact_history_public_terminal_close_releases_admission_only_after_roots_are_empty") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("history-public-terminal-release").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();
    let document = protocol::ArtifactId("history-public-terminal-release".to_string());
    let handle = database.create_document(ArtifactSpec::new(document).await).await.unwrap();
    let mut peers = Vec::new();
    for _ in 1..ARTIFACT_HISTORY_OPERATION_SLOTS {
        peers.push(ArtifactHistoryAdmission::try_claim().unwrap());
    }
    let history = handle.history();
    let generation = history.generation();
    let terminal = history.terminal_handle();
    history.cancel();
    drop(history);
    assert!(matches!(ArtifactHistoryAdmission::try_claim(), Err(ArtifactHistoryAdmissionError::Rejected(DbError::Unavailable(_)))));
    assert!(!terminal.terminal_is_empty());
    let mut released = false;
    for _ in 0..100_000 {
        if terminal.terminal_is_empty() {
            break;
        }
        let roots_were_empty = terminal.state.terminal_roots_are_empty();
        let admission_was_retained = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let progressed = terminal.close_step();
        let admission_is_released = terminal.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
        if admission_was_retained && admission_is_released {
            assert!(roots_were_empty, "the admission release must occupy a grant after every terminal root was already empty");
            assert!(progressed);
            assert!(terminal.state.finished.load(std::sync::atomic::Ordering::Acquire));
            released = true;
        }
        if !progressed {
            std::thread::yield_now();
        }
    }
    assert!(released);
    assert!(terminal.terminal_is_empty());
    assert!(handle.history_terminal(generation).is_none(), "terminal completion must unregister the released generation");
    let replacement = ArtifactHistoryAdmission::try_claim().unwrap();
    retire_history_admission(replacement);
    for peer in peers {
        retire_history_admission(peer);
    }
}

//#endregion 🔖️Retained submit authority

/// 🐢️ Laws whose honest size takes minutes in a debug build; the `long` level runs them.
mod long {
    use super::*;

    /// ⏱️ Two dozen grown documents greeted at once right after the database reopens — a hub restart's reconnect storm —
    /// are each welcomed within the reopen bound of the hub's growth e2e (30 s; growth run g17: after 24 documents grew
    /// together and the hub restarted, a document was not welcomed in 30 s). Prints the welcome distribution.
    #[semio_framework_async_macros::async_test]
    async fn two_dozen_grown_documents_greeted_at_once_after_a_reopen_are_welcomed_within_the_reopen_bound() {
        if !db_io_driver::process_isolated_law("tests::long::two_dozen_grown_documents_greeted_at_once_after_a_reopen_are_welcomed_within_the_reopen_bound") {
            return;
        }
        const DOCUMENTS: usize = 24;
        const BATCHES: usize = 8;
        const BATCH_EDITS: usize = 16;
        const REOPEN_BOUND: std::time::Duration = std::time::Duration::from_secs(30);
        let root = tempdir("greeting-storm").await;
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 10)));
        let grown_at = std::time::Instant::now();
        {
            let mut database = Database::open_at(pool.clone(), &root, Profile::Dev).await.unwrap();
            for document_index in 0..DOCUMENTS {
                let document = protocol::ArtifactId(format!("storm-{document_index}"));
                let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
                let mut previous: Option<protocol::MutationId> = None;
                for batch_index in 0..BATCHES {
                    let mut envelopes = Vec::with_capacity(BATCH_EDITS);
                    for edit_index in 0..BATCH_EDITS {
                        let mutation_id = protocol::MutationId(format!("storm-{document_index}-{batch_index}-{edit_index}"));
                        envelopes.push(protocol::MutationEnvelope {
                            mutation_id: mutation_id.clone(),
                            document_id: document.clone(),
                            actor: protocol::ActorId("storm-author".to_string()),
                            dependencies: previous.iter().cloned().collect(),
                            observed: None,
                            target: Vec::new(),
                            diff: protocol::ArtifactDiff { schema: protocol::SchemaId("storm.v1".to_string()), payload: vec![0x5a; 256] },
                            inverse: protocol::InverseMutation { schema: protocol::SchemaId("storm.v1".to_string()), payload: Vec::new() },
                            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
                            transaction: None, verb: None, line: None,
                        });
                        previous = Some(mutation_id);
                    }
                    handle.submit(db_artifact_contract::CommandBatch::new(envelopes).await.unwrap(), db_artifact_contract::SubmitOptions::default()).await.unwrap().unwrap();
                }
                drop(handle);
            }
            database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
        }
        let grown = grown_at.elapsed();
        let database = Arc::new(Database::open_at(pool.clone(), &root, Profile::Dev).await.unwrap());
        eprintln!("greeting storm of {DOCUMENTS} documents begins after growing them in {grown:?}");
        let storm_at = std::time::Instant::now();
        let mut welcomes: Vec<std::time::Duration> = std::thread::scope(|scope| {
            let greetings: Vec<_> = (0..DOCUMENTS)
                .map(|document_index| {
                    let database = database.clone();
                    scope.spawn(move || {
                        db_actor::block_on(async move {
                            let asked_at = std::time::Instant::now();
                            let mut session = database.hello(protocol::ArtifactId(format!("storm-{document_index}")), None, format!("storm-session-{document_index}"), protocol::ActorId("semio_hub".to_string()), 64 * 1024).await.unwrap();
                            session.take_welcome().unwrap().acknowledge().unwrap();
                            let welcomed = asked_at.elapsed();
                            while let Some(frame) = session.next_frame().await.unwrap() {
                                frame.acknowledge().unwrap();
                            }
                            welcomed
                        })
                    })
                })
                .collect();
            greetings.into_iter().map(|greeting| greeting.join().unwrap()).collect()
        });
        let storm = storm_at.elapsed();
        welcomes.sort();
        let ms = |duration: std::time::Duration| duration.as_secs_f64() * 1_000.0;
        eprintln!(
            "greeting storm of {DOCUMENTS} in {:.0} ms: welcome p50 {:.1} ms p90 {:.1} ms max {:.1} ms; {DOCUMENTS} documents x {} edits grown in {:.0} ms",
            ms(storm),
            ms(welcomes[DOCUMENTS / 2]),
            ms(welcomes[DOCUMENTS * 9 / 10]),
            ms(welcomes[DOCUMENTS - 1]),
            BATCHES * BATCH_EDITS,
            ms(grown)
        );
        assert!(welcomes[DOCUMENTS - 1] <= REOPEN_BOUND, "the slowest of {DOCUMENTS} simultaneous greetings took {:?}", welcomes[DOCUMENTS - 1]);
        let mut database = Arc::try_unwrap(database).ok().expect("the storm released every database handle");
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
    }

    /// 📈️ The same growth over SQLite storage, the hub's storage backend of choice: past every former
    /// wall (the eighth index entry, the 128th replaced value) with a small payload.
    #[semio_framework_async_macros::async_test]
    async fn a_sqlite_document_keeps_accepting_edits_past_every_former_wall() {
        const EDITS: usize = 150;
        let root = tempdir("growing-sqlite-document").await;
        let pool = test_worker_pool();
        let storage = db_storage_sqlite::SqliteStorage::open(pool.clone(), &root.join("db.sqlite3")).await.unwrap();
        let mut database = Database::open(pool, DbConfig::for_profile(Profile::Prod), Arc::new(db_storage::DbBackend::Sqlite(storage))).await.unwrap();
        let document = protocol::ArtifactId("growing-sqlite-doc".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let mut previous: Option<String> = None;
        for index in 0..EDITS {
            let id = format!("grow-{index}");
            let dependencies: Vec<&str> = previous.iter().map(String::as_str).collect();
            let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&id, &dependencies, "alice", &document, &[(&format!("feature-{}", index % 8), serde_json::json!(format!("{index}:{}", "s".repeat(512))))]).await]).await.unwrap();
            handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }).await.unwrap_or_else(|error| panic!("edit {index} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("edit {index} was refused: {error:?}"));
            previous = Some(id);
        }
        assert_eq!(handle.frontier().await.unwrap().head_seq, EDITS as u64);
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    }

    /// 📈️ A long-lived document keeps accepting edits however large it grows: hundreds of edits with a
    /// large payload each, then a full shutdown and reopen at the same root, then more edits — every one
    /// admitted, the whole history replayed. A per-operation I/O bound that the document's size outgrew
    /// made every hub document read-only after about twenty map edits, even across a restart.
    #[semio_framework_async_macros::async_test]
    async fn a_document_keeps_accepting_edits_as_it_grows_across_restart() {
        const EDITS_BEFORE_RESTART: usize = 400;
        const EDITS_AFTER_RESTART: usize = 60;
        const PAYLOAD_BYTES: usize = 24 * 1024;
        let root = tempdir("growing-document").await;
        let document = protocol::ArtifactId("growing-doc".to_string());
        let payload = |index: usize| serde_json::json!(format!("{index:08}{}", "g".repeat(PAYLOAD_BYTES)));
        let submit = |handle: ArtifactHandle, index: usize| {
            let document = document.clone();
            async move {
                let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&format!("grow-{index}"), &[], "alice", &document, &[(&format!("feature-{}", index % 32), payload(index))]).await]).await.unwrap();
                handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }).await.unwrap_or_else(|error| panic!("edit {index} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("edit {index} was refused: {error:?}"))
            }
        };
        {
            let mut database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
            let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
            for index in 0..EDITS_BEFORE_RESTART {
                submit(handle.clone(), index).await;
            }
            assert_eq!(handle.frontier().await.unwrap().head_seq, EDITS_BEFORE_RESTART as u64);
            drop(handle);
            database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
        }
        let mut reopened = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
        let handle = reopened.document(&document).await.unwrap();
        assert_eq!(handle.frontier().await.unwrap().head_seq, EDITS_BEFORE_RESTART as u64, "the whole grown history replays");
        for index in EDITS_BEFORE_RESTART..EDITS_BEFORE_RESTART + EDITS_AFTER_RESTART {
            submit(handle.clone(), index).await;
        }
        assert_eq!(handle.frontier().await.unwrap().head_seq, (EDITS_BEFORE_RESTART + EDITS_AFTER_RESTART) as u64);
        let last = EDITS_BEFORE_RESTART + EDITS_AFTER_RESTART - 1;
        let queried = handle.query(Query::Get { path: format!("feature-{}", last % 32) }, Consistency::Canonical).await.unwrap();
        assert_eq!(decode_query_json(queried).await, payload(last));
        drop(handle);
        reopened.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    }

    /// 🗃️ A database keeps opening documents however many it has served: hundreds of documents, a
    /// bounded window of them open at once, each edited, closed and reopened, then a full shutdown and
    /// reopen at the same root and every one reopened and edited again. What a document holds while it
    /// is mounted — its WAL writer and tail, its retained state — must bound how many documents are
    /// open at once, never how many a process has ever opened: keeping every served document mounted
    /// stopped a hub from opening its sixteenth document, and after every restart again.
    #[semio_framework_async_macros::async_test]
    async fn a_database_keeps_opening_documents_long_after_the_first_twenty() {
        const DOCUMENTS: usize = 240;
        const OPEN_AT_ONCE: usize = 16;
        let root = tempdir("many-documents").await;
        let document = |index: usize| protocol::ArtifactId(format!("doc-{index:04}"));
        let edit = |handle: ArtifactHandle, index: usize, round: usize| async move {
            let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&format!("doc-{index}-edit-{round}"), &[], "alice", &document(index), &[(&format!("round-{round}"), serde_json::json!(index))]).await]).await.unwrap();
            handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }).await.unwrap_or_else(|error| panic!("document {index} round {round} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("document {index} round {round} was refused: {error:?}"))
        };
        let window = |open: &mut std::collections::VecDeque<ArtifactHandle>, handle: ArtifactHandle| {
            open.push_back(handle);
            if open.len() > OPEN_AT_ONCE {
                open.pop_front();
            }
        };
        {
            let mut database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
            let mut open = std::collections::VecDeque::with_capacity(OPEN_AT_ONCE + 1);
            for index in 0..DOCUMENTS {
                let handle = database.create_document(ArtifactSpec::new(document(index)).await).await.unwrap_or_else(|error| panic!("document {index} was not created: {error:?}"));
                edit(handle.clone(), index, 0).await;
                window(&mut open, handle);
            }
            assert!(database.health().await.open_artifacts <= OPEN_AT_ONCE, "only the open window stays mounted");
            for index in 0..DOCUMENTS {
                let handle = database.document(&document(index)).await.unwrap_or_else(|error| panic!("document {index} was not remounted: {error:?}"));
                edit(handle.clone(), index, 1).await;
                window(&mut open, handle);
            }
            open.clear();
            database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
        }
        let mut reopened = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
        let mut open = std::collections::VecDeque::with_capacity(OPEN_AT_ONCE + 1);
        for index in 0..DOCUMENTS {
            let handle = reopened.document(&document(index)).await.unwrap_or_else(|error| panic!("document {index} was not reopened: {error:?}"));
            assert_eq!(handle.frontier().await.unwrap().head_seq, 2, "document {index} replays both edits");
            edit(handle.clone(), index, 2).await;
            let queried = handle.query(Query::Get { path: "round-2".to_string() }, Consistency::Canonical).await.unwrap();
            assert_eq!(decode_query_json(queried).await, serde_json::json!(index));
            window(&mut open, handle);
        }
        assert!(reopened.health().await.open_artifacts <= OPEN_AT_ONCE, "only the open window stays mounted after the restart");
        open.clear();
        reopened.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
    }
}

//#region 🔖️Source laws
#[test]
fn retained_sync_hello_engine_production_has_zero_blocking_waits_and_no_eager_follow_up() {
    let engine = include_str!("../../🦀️.rs");
    let production = engine.rsplit_once("\n//#region 🧪️Tests").unwrap().0;
    assert!(!production.contains("db_actor::block_on(db_sync::handle_hello"));
    assert!(production.contains("DatabaseSyncHelloFuture::try_submit"));
}

#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_engine", include_str!("../../🦀️.rs"))] {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start().trim_start_matches("} else ");
            let Some((_, scrutinee)) = trimmed.strip_prefix("if let ").or_else(|| trimmed.strip_prefix("while let ")).and_then(|rest| rest.split_once(" = ")) else { continue };
            if !(scrutinee.contains("retry_job.lock()") && !scrutinee.starts_with("{ ") && scrutinee.trim_end().ends_with('{')) {
                continue;
            }
            let mut depth = 1i64;
            for body in &lines[index + 1..] {
                assert!(!body.contains("retry_job.lock()") && !body.contains("submit_exact("), "{name}:{} holds the retry-job guard across a body that relocks or resubmits it: {line}", index + 1);
                depth += body.matches('{').count() as i64 - body.matches('}').count() as i64;
                if depth <= 0 {
                    break;
                }
            }
        }
    }
}
//#endregion 🔖️Source laws
