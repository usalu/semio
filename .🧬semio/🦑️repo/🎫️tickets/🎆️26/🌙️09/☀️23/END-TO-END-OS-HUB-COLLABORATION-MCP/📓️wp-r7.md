# WP-R7 — Event-Driven Runner Retirement, Faulted-Close Retry Owner, Dropped Engines, Per-Owner Ledger Witnesses

Slice: R7 (session 10). Native only. Private cargo target: `.tmp-ticket/wp-r7/target`. Captures: `wp-r7/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Runner retirement hook spin → event-driven | LANDED (WAL/engine close is a real `poll_close`; hook no longer re-requests itself) |
| 2. Faulted artifact close: product retry owner | LANDED (timer-driven bounded backoff in the runner handoff; `Database::shutdown` re-admits, reports, cancels) |
| 3. Engines dropped without close retire through pre-reserved hook | IN PROGRESS |
| 4. Per-owner ledger witnesses | IN PROGRESS |
| 5a. `wal_recovery_abort_faults_retry_without_duplicate_abort` | ROOT-CAUSED + FIXED (writer-signal reacquire race) |
| 5b. `hello_sessions_retire_…` | ROOT-CAUSED + FIXED (lost timer in `⏳️async` worker parking) |
| 5c. extra flakes found | FIXED: `db_io_output_task_yield_cancel_abandon…` (7/30 → 0/40), bootstrap fixture waker race |
| 6. Kernel `--features sync,ureq` os_store::sync | 2 lost wakes fixed (channel backbone send, closing outbox drain); see Evidence |
| Gates | see Evidence |

(Filled in below as items land.)
