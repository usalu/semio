# WAL Writer Permit and DB I/O Integration Frontier

Status: read-only review on 2026-09-05. No build was started and no native claim is made here. The five registered writer-core laws may be running elsewhere, but they do **not** yet prove a storage or `ArtifactWal` integration.

## Current core assessment

The new private module is a sound narrow starting point for a single in-process table: a permit is non-`Clone`, carries its document and a `(backend, slot, generation)` key, and `validate` fences a wrong backend, document, retired slot, or recycled slot. See [writer core](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:5). The source has no `WalStorage` or task integration yet; its parent exposes it only crate-wide at [storage module declaration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4577).

The core has since gained the bounded slot assertion, retained active-operation/releasing state, explicit guard release, and an independently attested child-process sentinel. The current exact assessment is in [Current writer-table revision](#current-writer-table-revision--terminal-and-integration-review). What remains persistent is that no backend derives/acquires the filesystem guard before a WAL mutation, and the neutral table fixture cannot prove effects through any of the six actual WAL task constructors. Integration must add hostile exact-stamp task laws before declaring durable writer authority.

The fixture/schema and script are registered as a narrow source/native gate: [schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🧪️fixtures/🧬️.schema.json:4) and [five exact laws](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:44). This proves registration, not a successful native receipt.

## Current result-retirement status

The prior `DbIoLostOwner::ResultLease` terminal inversion is fixed in the current source: a nonterminal `Some` returns false, a terminal `None` clears the result but still returns false, and only the following handback returns true ([storage:3904](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3904)). The registered `db_io_lost_result_lease_retains_every_page_and_final_handback` law is the applicable narrow regression. The separate rejected-`Backend` false-terminal case remains P0 below.

## Minimal coherent DB I/O ownership slice

Do not expose a free-standing `WalWriterPermit` to arbitrary callers. The existing [WalStorage trait](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4586) should become the sole owner-facing port:

```text
WalStorage::acquire_writer(document) -> WalWriterPermit
WalStorage::{create_segment, append, sync, seal, truncate_tail, delete_segment}(..., &WalWriterPermit)
WalStorage::release_writer(WalWriterPermit) -> ()
```

`read`, `segment_len`, `segment_state`, and `list_segments` remain permit-free. The permit stays inside `ArtifactWal` from successful open/create through final retirement; it is never copied into a caller-supplied task. Each mutable trait method converts `&WalWriterPermit` into a crate-private, nonconstructible task stamp carrying the exact key/document. Put the stamp in every mutating `DbIoTask` variant, not in a separate thread-local map:

```text
WalWriterAcquire { backend, document }
WalWriterRelease { backend, permit }
WalCreate/WalAppend/WalSync/WalSeal/WalTruncate/WalDelete { ..., writer_stamp }
DbIoResult::WalWriterPermit(WalWriterPermit)
```

Every exhaustive implementation in [the task owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2021), including `backend`, task cleanup, terminal witness, result cleanup/handback, and all driver matches, must be changed in one atomic compiler slice. Current trait implementations are `MemoryStorage`, `FsStorage`, SQLite, Postgres, Neo4j, `db_wal::AbortCancellationStorage`, and `db_testkit::FaultStorage`; a memory/FS-only signature change will not be coherent. The database-backed implementations require their own storage-owned cross-process guard, not `WalWriterTable<()>` (which only serializes this process). The filesystem implementation uses the sidecar guard; Memory can use `()`. A DB implementation must use its own transactional/advisory ownership row keyed by document with the same generation fence, or fail closed until it has one.

### Binding and table credit

`WalWriterTable::new` demands an exact `DbIoBackendControl`, but constructors do not have that control: registration constructs it only after it reserves the exact `(slot,generation)` ([storage:2675](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2675)). Do not seed it with a fake control or delay authority to `BackendOpen`. Add a synchronous repository-only `bind_writer_control(control)` hook before the slot is published and restore the free-ring reservation on its failure, as detailed in the current revision below.

Reserve the fixed table in the backend's owner credit, not per document and not per permit. `register_db_io_backend` already reserves `executor.owner_backing_bytes()` before it admits the executor ([storage:2648](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2648)); add the table/entry/release-state structural bytes to every executor's exact calculation. Memory's explicit byte calculation needs updating ([storage:5851](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:5851)); the default `size_of_val` path covers an inline field but not an allocation added separately. The fixed 32 slots own their guards for the backend lifetime. Task/result footprint remains accounted by existing `DB_IO_TASK_SLOT_BYTES` and result-lease credit because those are derived from the updated enum sizes. File descriptors are not byte credit, but must be retained and explicitly closed in the fixed table state.

For each mutation, executor dispatch must: (1) validate the stamp and exact backend/document before opening/mutating the segment; (2) pin the exact operation in the entry; (3) execute/resume only that operation; and (4) clear the pin in `close_operation_step`, including fault/cancellation. No mutation may rely on an `ArtifactId`/path equality check alone. This covers stale backend, slot, document, and generation at the actual effect boundary.

### Result, drop, and close state machine

The acquire task returns `DbIoResult::WalWriterPermit`. This result must **not** attach the short-lived acquisition task's result handback: a permit can live for the WAL lifetime, and retaining its task slot/result-lease credit would consume the fixed operation arena for every open writer. Instead, when a dropped task or `DbIoResultLease` closes this result, `DbIoResult::close_step` must move the permit into a dedicated lost-owner release state before returning terminal. The next `ResultLease` opportunity then handbacks the acquisition task normally. That state needs two owned alternatives:

```text
WriterPermit { permit, release_operation: None }
WriterPermit { permit: None, release_operation: Some(DbIoTaskOperation) }
```

On an opportunity, atomically submit `WalWriterRelease { permit }` to the permit's exact backend. If submission rejects, recover the permit from the rejected task and keep it in the same lost owner. If accepted, retain the operation and poll/finish it to terminal; only then remove the lost owner. A stale or fully retired backend may terminalize only after its own backend-close drain has explicitly closed the matching guard; it must never apply the key to a newly recycled backend. The current loss owner array/overflow/quarantine already gives the fixed retention locations ([storage:3779](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3779)); repair the `ResultLease` terminal inversion first. If the loss-owner arrays are saturated and parking fails, leave the permit in the result/task owner and report the existing retirement-pressure fault; never let a failed park fall through to `Drop`.

`WalWriterRelease` must not return `Unit` until its guard's `close_step` has reached terminal; retaining only `File::Drop` is not a release protocol. Model the table entry as `Live`, `Releasing`, then empty. `Releasing` fences all mutation stamps and remains credit-accounted; the release task may yield between its explicit unlock step and terminal witness. Backend close likewise iterates those retained entries on `Lane::Io` and its `backend_terminal_is_empty` includes the table. This fits the existing close scheduler, which refuses to retire an executor unless `close_backend_step` returns terminal and `backend_terminal_is_empty` agrees ([storage:2880](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2880)).

Keep `ArtifactWal::close_step` exactly its synchronous active-segment/buffer retirement contract ([wal:2501](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2501)). It must neither discard nor release the document permit. Add a separate retained async `ArtifactWal`/`ArtifactEngine` close future with phases `CloseActive -> SubmitWriterRelease -> AwaitWriterRelease -> Done`; only it may take the stored permit. Rotation must retain the same permit across seal/active close/successor creation ([wal:2493](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2493)). If that future is dropped, the permit moves to the lost-owner state above. This prevents premature release while preserving the existing synchronous close API.

## First acceptance laws after the integration slice

1. Acquire an FS permit in one backend, submit each of the six mutable operations with its valid stamp, then attempt each with wrong document, wrong backend, stale pre-recycle permit, and a slot-recycled stale permit. Each hostile operation is fenced before the segment bytes/state change.
2. Hold a multi-fragment append at its yielded boundary; request release/drop the `ArtifactWal`; prove the release waits for the operation cleanup, the file remains locked until its final explicit close, and a second process cannot acquire meanwhile.
3. Take an acquire result then drop its `DbIoResultLease`; run maintenance through more than one turn and prove the permit is released exactly once, all owner/result credits return, and a new permit can acquire the same document. Repeat with full lost-owner primary slots to exercise overflow/quarantine and rejected release submission.
4. Begin backend close with several live file guards, including one whose first `unlock` faults. Prove the executor remains nonterminal and the matching entry/credit remains; retry closes it explicitly, then backend retirement returns credit. No `retire_one` drop path is permitted.
5. Parent/child FS lock law uses a child-only sentinel, verifies it after child exit, then proves reacquisition only after explicit terminal close. This is the actual cross-process acceptance, not an inferred native compilation result.

## Current writer-table revision — terminal and integration review

This section supersedes the earlier `retire_one` and missing-active-operation observations above. It is a read-only review of the current source; no native run was started here. The registered five selectors each have exactly one test definition and one script registration. The new fifth law may still be running elsewhere and is not claimed as passed here.

### Core state machine

The current [writer table](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:27) has fixed the two important primitive gaps: its 32-slot-to-`u8` relation is asserted at [line 6](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:6), and each entry retains `active_operation` plus `releasing`. `release_step` first flips `releasing`, refuses all new work, still permits only the already-pinned exact operation to resume, and retains the guard across both an active operation and an unlock error ([lines 83–108](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:83)). The new fault-and-pin law exercises that exact sequence ([lines 253–280](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:253)). No unsafe raw guard-retirement path remains in this module.

There are three distinct booleans; integration must not conflate them:

1. `WalWriterGuard::close_step` means **made one release-progress turn**, not terminal. The real file guard returns `true` after a successful unlock but becomes empty in that same turn ([lines 139–146](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:139)).
2. `WalWriterTable::release_step` means **entry still retained** when `true`, and actually removes the entry only on `false` plus `guard.terminal_is_empty()` ([lines 101–108](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:101)). Thus a successful filesystem release deliberately consumes two table turns.
3. `WalWriterTable::close_step` means only **an entry was selected for an opportunity** when `true`; it is not a terminal witness ([lines 111–119](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:111)). A backend may report terminal only after `table.terminal_is_empty()`.

The current primitive is safe on pinned work, but has one P1 fairness hole: `close_step` always selects the first occupied array entry. A permanently/resumably pinned entry in the lowest slot returns retained indefinitely and prevents later already-releasing file guards from being given an unlock opportunity. This is not premature release, but it leaks lock/capacity availability under a stalled first operation. Add a bounded rotating close cursor (or scan each entry once per backend-close turn). Native law: acquire documents A and B; pin A; begin release of B; one bounded backend close round must explicitly advance/release B while A stays retained, and no new A/B operation is admitted.

The primitive intentionally has no `Drop` release for `WalWriterPermit` ([lines 8–25](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs:8)). That is correct for failed filesystem `unlock`, but makes the pending integration requirement non-optional: before any permit reaches `ArtifactWal`, a dropped owner/result must move it into a retained release state. Dropping a permit directly would otherwise leave its table entry and OS lock until full backend shutdown.

### Independent P0: rejected-backend parking reports terminal while retaining the backend

The sibling in [storage lines 3887–3901](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3887) remains a concrete premature-terminal bug, independent of `DbIoResultLease` (whose prior inversion is now corrected at [lines 3904–3913](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3904)). When `db_io_park_rejected_backend` finds all 64 slots occupied, it restores the exact `owner` and `pool` to `DbIoLostOwner::Backend`, sets retirement pressure, then returns `Ok(true)`. `db_io_lost_owner_close_step` treats true as terminal and clears that lost-owner slot ([lines 3822–3859](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:3822)), dropping the executor and its unreleased operation/credit. The failure arm must return `Ok(false)`, leaving the restored owner for a future opportunity; the outer maintenance function still returns its normal `Ok(true)` opportunity signal.

Use the existing serial [fixed-ring pressure law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:8432) for a deterministic native regression rather than worker timing:

1. Drain lost owners, assert the rejected registry is empty, and fill all `DB_IO_BACKEND_CONTROLS` rejected slots with test-only nonzero sentinel generations and `None` executors/pools. A small RAII test helper must clear only those sentinels at test exit.
2. Park one `DbIoLostOwner::Backend { owner: Some(Box::new(BlockingCompleteLawExecutor { terminal: false })), operation: 0, credit: DbIoCredit::default(), pool: Some(db_io_test_pool()) }` in the primary lost-owner ring. Call one `db_io_lost_owner_close_step`.
3. Assert retirement pressure is set and that the exact primary slot still contains `Backend` with both `owner` and `pool` present. Current source fails this assertion because it clears the slot. This witnesses retention directly; it does not depend on a race-prone background executor.
4. Clear the sentinel rejected slots, drive the ordinary lost-owner/rejected-backend close path to terminal, then assert all rings and `ledger_witness()` return to their entry values. This proves both no premature `Drop` and eventual progress once capacity exists.

### Smallest coherent first integration slice

The writer core is still isolated: `WalStorage` exposes naked mutators at [lines 4586–4627](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4586), its `DbIoTask::{WalCreate,WalAppend,WalSync,WalSeal,WalTruncate,WalDelete}` carry only a backend/document ([lines 2021–2032](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2021)), and both Memory and FS still submit these unguarded variants ([Memory lines 6380–6403](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:6380), [FS lines 7301–7308](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:7301)). There is no claim of storage/WAL writer exclusion yet.

The first compiler-coherent slice is:

1. Add `WalWriterAcquire { backend, document }` and `WalWriterRelease { backend, permit }` task/result ownership. Add a crate-private copyable stamp (`WalWriterKey` plus document only) to all six mutating WAL task variants. Public `WalStorage` becomes `acquire_writer(document) -> WalWriterPermit`, permits on every mutable method, and a retained `release_writer(permit)`; reads/inventory remain permit-free. This must update all current implementations in Memory, FS, SQLite, Postgres, Neo4j, `db_wal::AbortCancellationStorage`, and `db_testkit::FaultStorage` together—there is no safe compatibility overload.
2. Bind the table only after the true backend `(kind, slot, generation)` exists. `register_db_io_backend_reserved` obtains those values only at [lines 2675–2705](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:2675). Add a synchronous repository-only `bind_writer_control(control)` hook and invoke it before publishing the registry slot; on failure restore that slot/free-ring reservation. Do not synthesize a control or require an asynchronous `BackendOpen` to bind authority. Each executor keeps a fixed `Mutex<WalWriterTable<G>>` in its already-reserved owner backing; Memory uses `()`, FS uses `WalFileWriterGuard`, and remote/SQL implementations need their own real backend guard before their mutation paths can opt in.
3. At executor dispatch, validate and pin the exact stamp before the first mutable effect; retain that operation id across every `Yield`, error, cancellation, and `close_operation_step`, then call `finish_operation` exactly once. The `&G` returned by `pin_operation` must not be retained across a task yield or a table mutex; it is only a scoped admission witness. `release_writer` drives `release_step` on the I/O lane and does not complete until the table entry is removed.
4. Store the acquired permit in `ArtifactWal` before `SegmentWriter::begin` calls `create_segment` ([wal lines 2312–2314](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2312), and preserve it through rotation/open/recovery. A separately retained async close/release state owns the permit after `ArtifactWal::close_step`; dropping that future must use the repaired lost-owner path, not `Drop` the permit. The backend's `close_backend_step` and `backend_terminal_is_empty` must drive/check the table after normal WAL resources, never retire while a pinned/releasing entry remains.

The first integration law should be real FS, not just the table: hold an append at a forced yield, request release, prove a second process stays `Conflict` until that exact operation is finished and explicit guard release reaches table-terminal; repeat with an injected unlock fault and a dropped result/retirement-pressure handoff. Test all six hostile task stamps before bytes or segment state change, then add Memory only as the deterministic non-filesystem companion.
