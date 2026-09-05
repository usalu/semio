# Writer Release Fault Wakes And WAL Open Owner Frontier

Status: read-only source audit on 2026-09-05. No build, process, or product source was changed.

## Controller: two remaining P0 paths

### Refusal faults a pre-polled close without waking it

`WalWriterRelease::poll` saves exactly one waiter and returns its retained failure as soon as its cell has a fault ([`storage/🔐️writer/🔔️release/🦀️.rs:124–143`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:124)). `request_controller` correctly latches a refusal into every requested exact cell, but it neither takes nor wakes these existing waiters ([227–237](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:227)). A close that was polled before a shutdown/stale/closed maintenance request therefore remains asleep indefinitely even though its next poll would return the retained error. An unpolled close happens to observe the fault, so this is a true wake race rather than absence of error state.

The recently added post-unlock notification sites are correctly placed: the page-admission rollback drops the task arena before notification ([`storage/🦀️.rs:3409–3417`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3409)), async executor return drops the backend registry first ([2908–2920](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2908)), and task-close drops both task owner and close-turn before notification ([4518–4525](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4518)). They do not cover a refusal originating directly from permit drop/retry.

**Bounded fix:** make `request_controller` copy the exact `(pool, ticket)` while holding its controller-row mutex, drop that mutex, call `request_maintenance`, then on refusal latch the backend’s currently requested, nonfaulted cells and call `notify_faults(backend)`. `notify_faults` already takes a waiter under its cell lock and wakes it after releasing that lock ([185–193](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:185)). It must not be invoked while the controller-row mutex is held: a synchronous waiter can immediately retry and reenter `request_controller`.

The latch should remain backend-wide for a genuine scheduler refusal: no requested guard can be run on that closed/stale hook, and all must remain fail-closed. Do not clear `requested`, drop the table guard, or claim completion. `WalWriterReleaseFailure::into_parts` and `retry` preserve the exact closer ([105–120](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:105)).

### An outer executor panic loops `More` with no fault owner

The table-owned path catches its own guard failure, latches it to one exact key, and returns `Err` ([`storage/🔐️writer/🦀️.rs:171–206`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:171)). But an arbitrary `DbIoTaskExecutor::writer_release_step` may itself panic. `db_io_writer_release_lane_step` catches that only outside the executor ([`storage/🦀️.rs:2751–2757`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2751)); the controller then converts every `Err` into `WorkerMaintenanceStep::More` without latching a key or waking a waiter ([`release/🦀️.rs:239–253`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs:239)). The next callback sees the same request and panics/spins forever. Its guards stay retained but the exact close authority sees neither success nor failure.

**Bounded fix:** distinguish an executor-reported, key-latched release fault from an outer executor fault. Add `DbIoWriterReleaseStep::Faulted` and have `WalWriterTable::release_requested_step` return that after it latches the selected key. The controller maps `Faulted` to `More`, so a later healthy requested key gets its bounded turn. For an actual `Err` from `db_io_writer_release_lane_step` (including the outer panic), call a private `fault_all_requested(backend, error)`, wake those waiters after all locks are dropped, and return `Idle`. A backend-wide fault is deliberately conservative: the executor panicked before it identified a safe selected entry; it must not keep attempting any of its guards.

This also preserves the intended meaning of current `Err => More`: **only a proven, per-key-latched table error** may continue toward a healthy peer. The current untyped `Result<DbIoWriterReleaseStep, DbError>` cannot express that distinction safely.

## Native law additions

1. **Coalesced fault plus healthy fairness.** On one mounted controller, acquire A with a one-time guard-close `Io` error and B with a healthy guard. Poll both releases to pending before the same hook turn. A becomes a retained `WalWriterReleaseFailure`; B reaches terminal without waiting for A’s retry; A remains closed/fenced. Record one bounded fault turn, one B turn and an idle stop—no repeated A callbacks. The existing `wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer` only proves B remains *valid*; it releases B after A retry and does not prove coalesced progress.
2. **First refusal / waiter already parked.** Poll a release with a counting waker, then make its installed maintenance ticket refuse (pool shutdown or exact hook closure) before `permit.release()`/`retry` calls `request_controller`. Assert exactly one wake after the cell fault, `Poll::Ready(Err(WalWriterReleaseFailure))`, unchanged requested cell/table guard, and a fresh contender still gets `Conflict`. Repeat with an unpolled release and assert its first poll returns the same retained failure. This test specifically fails current lines 227–237.
3. **Unexpected outer executor panic.** A test executor supports writer authority, owns two requested table entries, and panics from `writer_release_step` before delegating to its table. Both already-polled release waiters wake once with retained failures, their guards/table entries remain, the hook becomes idle instead of continually `More`, and backend close still retains/drains its executor according to its normal close protocol. This must not reuse the one-time `WriterControllerLawGuard` error: that error is already caught inside the table and is the healthy-fairness case above.

## ArtifactWal: failed-open cleanup currently drops the only retry owner

`ArtifactWal::release_failed_open` consumes a `WalWriterPermit`, awaits its close, and reduces a `WalWriterReleaseFailure`—which owns the retryable `WalWriterRelease`—to a formatted `DbError` ([`db/📝️wal/🦀️.rs:2313–2317`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2313)). The failure object is then dropped. The table/OS guard correctly remains retained, but no caller can retry or terminally close it; reopening the same document only conflicts. This occurs in both `create` ([2322–2328](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2322)) and all `open_acquired` failure exits ([2346–2352](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2346), [2447–2451](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2447)).

There is a second early-loss branch: `while indices.close_step()?` at [2448](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2448) can return before the following `match`, dropping `writer: Option<WalWriterPermit>` by ordinary destructor. The signal is requested, but its explicit retained close future is not returned. All recovery exits, including retained-index cleanup failure, must pass one owner-preserving finalizer.

### Minimal retained error shape

Split the current flattening helper into two explicit layers.

```text
ArtifactWalAcquiredRejected {
  cause: DbError,
  writer: WalWriterPermit,
}

ArtifactWalOpenRejected {
  cause: DbError,
  release: WalWriterRelease,
}
```

`open_acquired` returns `Result<(ArtifactWal, WalRecoveryReport), ArtifactWalAcquiredRejected>` and **never releases the caller-supplied writer on error**. It must unify every recovery/result/index-close error before returning that owner. This is essential for the planned cluster flow: the caller deliberately obtains a follower writer and uses `open_acquired` before planning; that caller, not a nested recovery helper, owns the decision to keep or close it.

The self-acquiring `create`, `open`, and `open_with_control` map `AcquiredRejected` to `OpenRejected` by moving the writer into `writer.release()` and returning it without awaiting/discarding it. If an error happens before acquisition, use a no-release variant in the same public enum rather than faking a close owner. `OpenRejected::into_parts()` exposes the original open cause plus exact release owner; `retry_close(self)`/its returned failure must retain the same owner exactly like `WalWriterReleaseFailure::into_parts`. No destructor may turn this into a string error.

### Engine and façade propagation

The only production engine constructors are `ArtifactEngine::create_retained` ([`db/🗿️artifact/🦀️.rs:1183–1186`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1183)) and `open_retained` ([1198–1244](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1198)). They currently use `?` into `DbError`, so must change to an `ArtifactEngineOpenRejected` enum that preserves `ArtifactWalOpenRejected` (and has an ordinary pre-WAL `DbError` variant for snapshot/materialization errors). Then propagate it through the construction `ArtifactAuthority::spawn` result and the two `Database::spawn_authority_{create,open}` paths; `Database::create_document`/`document` must expose or mount the retained rejection rather than call `?` and lose it.

Direct non-engine owners must consume the same type: CLI repair/append at [`db/⌨️cli/🦀️.rs:1051`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs:1051) and [1257](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs:1257), and cluster follower open at [`db/🌐️cluster/🦀️.rs:217`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️.rs:217). Cluster’s pending `acquire_writer → open_acquired` ordering should retain `ArtifactWalAcquiredRejected.writer` across plan/recovery failure, not close/reacquire a different authority.

### First executable failure law

Create a test-only mounted backend whose `SegmentWriter::begin` operation fails after acquisition and whose writer guard’s first close returns `Io`. Assert the new rejected result retains (1) the original create/open error and (2) the exact `WalWriterRelease`; a same-document acquisition conflicts before retry; `retry_close` returns the same owner on another fault and only a terminal close permits a new acquire. Cover both direct `ArtifactWal::create/open` and `ArtifactEngine::{create_retained,open_retained}` propagation. Add a retained-index-close fault variant to verify the [2448](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2448) finalizer instead of only the initial list failure.

No native result is claimed by this audit.
