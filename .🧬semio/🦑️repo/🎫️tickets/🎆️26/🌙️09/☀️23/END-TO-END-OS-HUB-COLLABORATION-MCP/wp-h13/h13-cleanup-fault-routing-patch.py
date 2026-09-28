#!/usr/bin/env python3
"""🧯️ H13 session 14c: DB I/O operation-cleanup faults are routed to their own backend (kernel-db, hub-closure-only).

Root cause: the process-global close ring (`db_io_task_close_step`, maintenance class 5) returned a backend's operation-cleanup
error (`close_operation_step` / `db_io_backend_return_operation`) to whichever caller happened to drive the maintenance step and
kept the failing task at the ring head — so an unrelated backend's `finish()` (task-retirement wait) and `close_db_io_backend`
failed with a foreign fault (observed: in-memory SQLite `close()`/`len()` failing with another file database's
`database is locked`) and could not retire behind it. Now the ring records the fault on the failing backend's registry slot
(`cleanup_fault`, fixed-authority `DbIoFault`, kind preserved), rotates the task and retries it; a successful cleanup of that
backend clears it; only that backend's own task waiters and its close take and report it. Law:
`a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close`. Idempotent; `--dry-run` writes nothing.
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
STORAGE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs"
FIXTURES = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"

HELPERS = r'''
/// 🧯 Records one operation-cleanup fault on the backend it belongs to — never on the caller that happened to drive the close
/// ring — so only that backend's own task waiters and its close report it; the task stays in the ring and is retried. A
/// successful cleanup of the backend (`None`) clears it.
fn db_io_note_backend_cleanup(control: DbIoBackendControl, outcome: Option<&DbError>) {
    let (slot, generation) = db_io_backend_parts(control);
    let stale = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return;
        }
        std::mem::replace(&mut owner.cleanup_fault, outcome.map(|error| db_io_task_fault(DbIoFaultKind::Backend, error)))
    };
    if let Some(mut stale) = stale {
        while stale.close_step() {}
    }
}

/// 🧯 Takes the operation-cleanup fault last recorded on `control`, if any, as the error its own waiter or close reports.
fn db_io_take_backend_cleanup_fault(control: DbIoBackendControl) -> Option<DbError> {
    let (slot, generation) = db_io_backend_parts(control);
    let fault = {
        let mut registry = db_io_backend_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = &mut registry.slots[slot as usize];
        if owner.generation != generation || db_io_backend_control(owner.kind, slot, generation) != control {
            return None;
        }
        owner.cleanup_fault.take()
    };
    fault.map(DbIoFault::into_db_error)
}
'''

EXECUTOR = r'''
struct CleanupFaultLawExecutor {
    fail: Arc<std::sync::atomic::AtomicBool>,
    terminal: bool,
}

impl DbIoTaskExecutor for CleanupFaultLawExecutor {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Ok((DbIoExecutionStep::Complete, Some(DbIoResult::Unit)))
    }
    fn drive_async(self: Box<Self>, _operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let executor: Box<dyn DbIoTaskExecutor> = self;
            (executor, task, Err(DbError::Internal("cleanup fault fixture has no async driver".to_string())))
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        if self.fail.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Io("injected operation cleanup fault".to_string()));
        }
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        self.terminal = true;
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.terminal
    }
}
'''

LAW = r'''
/// 🧯️ An operation-cleanup fault stays with its backend: the close ring records it on the failing backend and moves on, so a
/// healthy backend's operation finishes, drains and closes `Ok` beside it, while the failing backend's own waiter and close
/// report the fault (its kind preserved) until its cleanup succeeds.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close() {
    let _owner = fixture_owner();
    let before = ledger_witness();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
    let fail = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let faulty = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(CleanupFaultLawExecutor { fail: fail.clone(), terminal: false }), pool.clone()).unwrap();
    let healthy = register_db_io_backend(DbIoBackendKind::Filesystem, Box::new(BlockingCompleteLawExecutor { terminal: false }), pool.clone()).unwrap();
    let injected = DbError::Io("injected operation cleanup fault".to_string());
    let faulted = submit_db_io_task(DbIoTask::BackendOpen { backend: faulty, path: DbIoText::try_from_str("fixture://cleanup-fault").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert_eq!(faulted.finish().await.err(), Some(injected.clone()), "the failing backend's own waiter reports its cleanup fault");
    let neighbour = submit_db_io_task(DbIoTask::BackendOpen { backend: healthy, path: DbIoText::try_from_str("fixture://cleanup-fault-neighbour").unwrap() }).unwrap_or_else(|(error, _)| panic!("{error}"));
    assert!(matches!(neighbour.finish().await, Ok(DbIoResult::Unit)), "a neighbour's operation never inherits another backend's cleanup fault");
    drain_control_tasks(healthy).await;
    close_db_io_backend(healthy).await.unwrap();
    assert_eq!(close_db_io_backend(faulty).await, Err(injected), "the failing backend's close reports its own cleanup fault");
    fail.store(false, std::sync::atomic::Ordering::Release);
    drain_control_tasks(faulty).await;
    close_db_io_backend(faulty).await.unwrap();
    assert_eq!(pool.shutdown(), Ok(()));
    assert_eq!(ledger_witness(), before);
}
'''

CLEANUP_OLD = """        let result = db_io_executor_close_operation(backend, handle.operation, task);
        if matches!(result, Ok(true)) {
            owner.backend_cleanup_done = true;
        }
        drop(owner);
        let rotated = if matches!(result, Ok(false)) { db_io_rotate_close_head(handle) } else { Ok(()) };
        drop(_turn);
        writer::release::defer_fault_notifications(backend);
        result?;
        rotated?;
        return Ok(Some(0));
"""
CLEANUP_NEW = """        let result = db_io_executor_close_operation(backend, handle.operation, task);
        if matches!(result, Ok(true)) {
            owner.backend_cleanup_done = true;
        }
        drop(owner);
        let rotated = if matches!(result, Ok(true)) { Ok(()) } else { db_io_rotate_close_head(handle) };
        drop(_turn);
        writer::release::defer_fault_notifications(backend);
        match &result {
            Ok(true) => db_io_note_backend_cleanup(backend, None),
            Ok(false) => {}
            Err(error) => db_io_note_backend_cleanup(backend, Some(error)),
        }
        rotated?;
        return Ok(Some(0));
"""

RETURN_OLD = """        let result = db_io_backend_return_operation(backend, handle.operation);
        if result.is_ok() {
            owner.backend_admitted = false;
        }
        drop(owner);
        drop(_turn);
        writer::release::defer_fault_notifications(backend);
        result?;
        return Ok(Some(0));
"""
RETURN_NEW = """        let result = db_io_backend_return_operation(backend, handle.operation);
        if result.is_ok() {
            owner.backend_admitted = false;
        }
        drop(owner);
        let rotated = if result.is_ok() { Ok(()) } else { db_io_rotate_close_head(handle) };
        drop(_turn);
        writer::release::defer_fault_notifications(backend);
        db_io_note_backend_cleanup(backend, result.as_ref().err());
        rotated?;
        return Ok(Some(0));
"""

EDITS = [
    (STORAGE, "    close_fault: Option<DbIoText>,\n}\n", "    close_fault: Option<DbIoText>,\n    cleanup_fault: Option<DbIoFault>,\n}\n", 1),
    (STORAGE, "                close_fault: None,\n            }),\n", "                close_fault: None,\n                cleanup_fault: None,\n            }),\n", 1),
    (STORAGE, "        close_fault: None,\n    };\n", "        close_fault: None,\n        cleanup_fault: None,\n    };\n", 2),
    (STORAGE, CLEANUP_OLD, CLEANUP_NEW, 1),
    (STORAGE, RETURN_OLD, RETURN_NEW, 1),
    (
        STORAGE,
        "        if let Err(error) = db_io_maintenance_step() {\n            return std::task::Poll::Ready(Err(error));\n        }\n        context.waker().wake_by_ref();\n",
        "        if let Err(error) = db_io_maintenance_step() {\n            return std::task::Poll::Ready(Err(error));\n        }\n        if let Some(fault) = db_io_take_backend_cleanup_fault(control) {\n            return std::task::Poll::Ready(Err(fault));\n        }\n        context.waker().wake_by_ref();\n",
        1,
    ),
    (
        STORAGE,
        "        db_io_maintenance_step()?;\n        let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n        if !db_io_slot_matches(&owner, handle) {\n            return std::task::Poll::Ready(Ok(()));\n        }\n        context.waker().wake_by_ref();\n",
        "        db_io_maintenance_step()?;\n        let backend = {\n            let owner = DB_IO_TASK_SLOTS[handle.slot as usize].lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n            if !db_io_slot_matches(&owner, handle) {\n                return std::task::Poll::Ready(Ok(()));\n            }\n            owner.backend\n        };\n        if let Some(fault) = backend.and_then(db_io_take_backend_cleanup_fault) {\n            return std::task::Poll::Ready(Err(fault));\n        }\n        context.waker().wake_by_ref();\n",
        1,
    ),
    (
        STORAGE,
        "fn db_io_error_text(error: &DbError) -> DbIoText {\n    DbIoText::try_from_str(&error.to_string()).unwrap_or_else(|_| db_io_text_literal(\"DB I/O error detail exceeded fixed authority\"))\n}\n",
        "fn db_io_error_text(error: &DbError) -> DbIoText {\n    DbIoText::try_from_str(&error.to_string()).unwrap_or_else(|_| db_io_text_literal(\"DB I/O error detail exceeded fixed authority\"))\n}\n" + HELPERS,
        1,
    ),
    (
        FIXTURES,
        "struct DropRegisteredBackend(DbIoBackendControl);\n",
        EXECUTOR.lstrip("\n") + "\nstruct DropRegisteredBackend(DbIoBackendControl);\n",
        1,
    ),
]


def main() -> int:
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {STORAGE, FIXTURES}}
    pending = problems = 0
    for path, old, new, count in EDITS:
        text = texts[path]
        if text.count(new) == count:
            print(f"applied already: {path.name} :: {old.strip()[:70]}")
            continue
        if text.count(old) != count:
            print(f"PROBLEM ({text.count(old)} matches, want {count}): {path.name} :: {old.strip()[:70]}")
            problems += 1
            continue
        pending += 1
        print(f"{'would apply' if dry else 'apply'}: {path.name} :: {old.strip()[:70]}")
        texts[path] = text.replace(old, new)
    law_text = texts[FIXTURES]
    if "fn a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close" not in law_text:
        pending += 1
        print(f"{'would append' if dry else 'append'}: law a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close")
        texts[FIXTURES] = law_text.rstrip("\n") + "\n" + LAW
    if problems:
        print(f"{problems} problems — nothing written")
        return 1
    if not dry:
        for path, text in texts.items():
            if text != path.read_text(encoding="utf-8"):
                path.write_text(text, encoding="utf-8")
    print(f"{'dry run: ' if dry else ''}{pending} pending")
    return 0


if __name__ == "__main__":
    sys.exit(main())
