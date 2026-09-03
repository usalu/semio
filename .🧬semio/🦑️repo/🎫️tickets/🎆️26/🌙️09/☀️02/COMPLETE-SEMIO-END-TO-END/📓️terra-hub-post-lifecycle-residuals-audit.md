# Hub Post-Lifecycle Residuals Audit

Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`
Goal: `🎯r2603`
Scope: read-only audit after the DB I/O page-lifecycle and SQLite BLOB fixes, 2026-09-03.

## Evidence and execution limits

This audit inspected the current tree and the two preceding reports:

- `📓️sol-db-page-lifecycle.md` proves the recent page lifecycle repair moved retained output publication to an atomic terminal transition and records the original two hub residuals.
- `📓️sol-sqlite-blob-storage.md` proves the SQLite `BLOB` concatenation repair and records that the SQLite BLOB boundary cases pass until the generic missing-`get` oracle.

I attempted the narrow hub case through the sanctioned Nx route, using a ticket-local target directory:

```text
RUST_MIN_STACK=16777216 CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=<ticket>/🗑️generated/terra-hub-post-lifecycle/target \
  bun nx run os-hub:test -- blob_put_get_head_round_trip -- --exact --nocapture
```

It reached the project test runner but its fundamental-level assertion budget terminated the `nextest` execution after 15 seconds. A quick-level retry was stopped during its `nextest list` test-binary build while concurrent workspace Cargo work was consuming the shared build graph; it did not reach an assertion. Thus neither command is evidence that the product test passed or failed. The current source trace below and the prior reports are the evidence for the observations. The source itself declares the sanctioned budgets as 15 s fundamental and 30 s quick in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`.

## 1. Missing payload GET returns 500

### Proven causal path

The exact hub test is an FS-backed integration test: `test_state` opens `Database::open_at(..., Profile::Test)` in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:1945-1948`. It PUTs and reads bytes successfully, observes missing HEAD as 404, then requires missing GET as 404 at lines 2280-2307.

The backend does produce the intended taxonomy before the I/O task boundary:

| Backend/mode | Exact missing `PayloadGet` behavior | Facade that lowers a `DbIoFault` |
| --- | --- | --- |
| Memory / blocking | `MemoryDbIoExecutor::execute_step` returns `DbError::NotFound("memory payload not found")` (`…/🗄️storage/🦀️.rs:5837-5859`). | `memory_execute` / `MemoryStorage::get` use the common task operation path. |
| FS / blocking | `open_err` maps OS `NotFound` to `DbError::NotFound`; `PayloadGet` calls `read_pages_step` (`…/🗄️storage/🦀️.rs:6341-6350, 6482-6503, 6755`). | `fs::execute` awaits the task and calls `DbIoFault::into_db_error` (`…/🗄️storage/🦀️.rs:6917-6919`). |
| SQLite / blocking | `PayloadGet`'s `stage_read` supplies `DbError::NotFound(format!("payload {hash} not found"))` (`…/🗄️storage/🪶️sqlite/🦀️.rs:350-357`). | SQLite `execute` has the identical lowering at lines 546-548; `PayloadStorage::get` uses it at 699-710. |
| PostgreSQL / async-native | Async task `PayloadGet` delegates to `payload_read_into` (`…/🗄️storage/🐘️postgres/🦀️.rs:739`), whose absent-payload branch is typed `NotFound`. | `PostgresStorage::execute` starts the async native driver, then calls `DbIoFault::into_db_error` at 861-864. |
| Neo4j / async-native | The equivalent async `PayloadGet` delegates to `payload_read_into` (`…/🗄️storage/🌐️neo4j/🦀️.rs:903`). | `Neo4jStorage::execute` has the same lowering at 1014-1017. |

At the common boundary, the exact `DbError` tag is irreversibly discarded:

1. `db_io_task_fault(kind, error)` renders the error into bounded `DbIoText` and returns only `DbIoFault { kind, detail, result_handback }` (`…/🗄️storage/🦀️.rs:3141-3148`). `DbIoFaultKind::Backend` denotes the runner provenance, not the backend error category.
2. The blocking driver catches an executor `Err(error)`, publishes `DbIoTerminal::Fault(db_io_task_fault(Backend, &error))`, and wakes its waiter (`…:3425-3479`). The async-native terminal driver does the same after returning the async executor (`…:3515-3546`).
3. `DbIoTaskOperation::take` gives that fault one result-handback owner and moves the task to `Closing` (`…:4039-4085`).
4. `DbIoFault::into_db_error` maps `Backend` and `Panic` to `DbError::Internal(detail)`, rather than restoring the backend category (`…:2984-3014`). The FS and SQLite facades then apply that conversion.
5. Hub `get_blob` recognizes only a surviving `DbError::NotFound` as 404 and maps every other storage error to 500 (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:538-550`). It therefore receives `Internal("…not found…")` and returns 500. This also directly explains the generic `exercise_payload_storage` missing-`get` oracle (`…/🗄️storage/🦀️.rs:8447-8476`).

**Conclusion (proven): taxonomy flattening alone explains the reported missing-GET mismatch.** It affects the generic storage test and the FS hub route even though the individual backend executors return `NotFound` exactly. It also affects SQLite and both async-native backends through the same common conversion.

### Why this is not another page-publication or close failure

The repaired lifecycle is visible in the active driver:

- A blocking yield transitions the whole retained page set `Executing -> Queued`; a terminal result or fault transitions it as a whole `Executing -> TerminalResult` (`…/🗄️storage/🦀️.rs:3425-3442`).
- The async driver likewise makes the whole terminal transition before publication (`…:3515-3539`).
- A missing FS payload faults before a successful `DbIoPages` result is created; SQLite fails in `stage_read` before `read_stage_step` can seal pages. Memory creates its fixed cursor before looking up the payload, but it also produces no output page result on a missing lookup.
- Fault handback is retained rather than dropped. `DbIoFault::close_step` closes bounded detail before returning its task handback (`…:3016-3039`); `db_io_task_close_step` waits for executor cleanup, drains terminal/task owners, queue and async admission credits, backend close, aggregate credit, and finally requires `db_io_operation_terminal_is_empty` before freeing the slot (`…:4167-4275`).

Thus close/retirement is an essential law to preserve in the fix, but it does not change a `NotFound` into `Internal`; that conversion occurs before the facade observes the fault. A regression must nevertheless prove a typed backend fault fully retires, because Memory's cursor and all output-writer reservations still have owners on this terminal path.

### Minimal Sol implementation packet

**Production scope.**

1. In `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs`, split executor-failure provenance from the bounded database-error category. Extend `DbIoFault` and `db_io_task_fault` with a fixed-size, project-owned `DbIoFaultCause` (or equivalently a category field) derived from `DbError`; keep `DbIoFaultKind::{Backend,Panic,…}` for runner state only.
2. Make `DbIoFault::into_db_error` reconstruct the original reachable database category from that cause plus the existing bounded `DbIoText`. Preserve scalar fields needed by generation/fence variants in fixed-size fields. Do **not** retain a raw `DbError`, a `String`, a `Vec`, or a foreign error object in the terminal task merely to retain taxonomy.
3. Set the cause at every construction site: `db_io_task_fault`, the direct invalid-step fault, cancellation/stale synthetic faults, blocking `db_io_drive_one`, and `db_io_finish_async_driver`. Cancellation must remain `Closed`; a backend panic must remain an internal/panic-class error rather than becoming a semantic `NotFound`.
4. In `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:get_blob`, replace the special `NotFound`/catch-all pair with `Err(error) => Err(db_error_status(&error))`. This is not needed to turn a preserved `NotFound` into 404, but it removes the adjacent divergent HTTP lowering for `Unavailable`, `Timeout`, `InvalidArgument`, and `LimitExceeded`.

No database schema, SQL migration, fixture format, or public wire protocol change is needed. The SQLite BLOB casting repair remains untouched.

**Focused regression suite and oracles.**

1. Add a common task-runner fault law beside the existing retained output/cancel/abandon laws in `…/🗄️storage/🦀️.rs`. A deterministic blocking executor returns `DbError::NotFound` from `PayloadGet`; assert the awaited facade/result is `Err(DbError::NotFound(_))`, then drive bounded maintenance until every task, page, backend, aggregate, and result-handback owner is terminal-empty. Repeat with a forced yielded writer before the typed error if the existing fixture can express it.
2. Add the async-native counterpart using a test executor that completes with `DbError::NotFound`; assert the same tag survives `db_io_finish_async_driver` and that its returned executor/admission credit retire. This covers PostgreSQL and Neo4j lowering without requiring live external services.
3. Keep `exercise_payload_storage` as the cross-backend oracle: its pre-put missing GET and post-delete missing GET at lines 8458-8464 must remain `NotFound`. Run it for Memory and FS. Extend SQLite's existing `assert_payload_roundtrip` / `payload_roundtrip_obeys_neutral_page_boundaries_and_arbitrary_bytes` (`…/🪶️sqlite/🦀️.rs:805-854`) to assert missing `get` both before put and after delete, closing any returned pages as its existing helper does.
4. Keep the hub boundary fixture and `blob_put_get_head_round_trip` unchanged in meaning: PUT hash/size, present GET exact bytes, present HEAD 200, missing HEAD 404, and missing GET 404. Add one hub status-level test only if needed to cover a non-`NotFound` mapped status; do not substitute a string-match oracle for the typed error oracle.

**Required ownership, cancellation, and close laws.**

- Exactly one terminal publish owns the complete retained task; no page is terminally published on an ordinary yield.
- A fault returned to the caller owns one result handback. Its bounded detail/category closes before that handback; the task slot is reused only after backend cleanup, page/task close, queue/async credit return, aggregate detach, and `terminal_is_empty`.
- Cancellation and abandonment win over an unfinished operation, drain retained output terminally, and must never expose a backend `NotFound` as a normal result. A completed semantic `NotFound` must never be reclassified as `Closed` merely because close follows delivery.
- Stale-generation and panic semantics remain distinct from ordinary backend categories.

This preserves fixed-authority rules: the new taxonomy is a finite enum plus scalar values, while error display remains the existing bounded `DbIoText`; it adds neither an unbounded error owner nor an additional driver/queue/terminal owner.

## 2. Private directory websocket: non-Welcome first frame

### Proven order and what is not yet observable

`handle_ws` serializes its pre-session work in this order (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs:759-981`): decode Hello; `resolve_auth`; descriptor lookup; schema/hash check; color/policy construction; `ensure_document`; `Database::hello`; take/encode/ack Welcome; drain bootstrap frames; send Session; only then insert presence, record a directory sync session, and publish `DirectoryStreamMessage::Connection`.

The existing test at lines 2553-2597 establishes these successful observations before the second document:

- anonymous directory stream is closed/no text;
- the authorized observer receives Heartbeat;
- the other private member receives `Welcome`, `Session`, and presence on the first document;
- a private-space membership mutation does not leak into the observer's directory stream;
- the observer is a spectator in `mine`, and `mine/shared` has been durably announced before opening its valid socket.

At the failing second-document assertion, `next_server_frame` decodes a frame and immediately discards it inside `matches!` (`…/📦️bin.rs:2053-2067, 2586-2589`). There is no existing log/tracing output of that decoded `ServerFrame`. Consequently the *actual* non-Welcome variant and its message are **not observable from the current test output or source alone**. The attempted exact runtime command did not reach assertions (execution limits above), so this audit cannot honestly invent the message.

The source does establish the finite error sites before Welcome:

| Stage | Failure frame/code |
| --- | --- |
| auth | `unauthorized` |
| descriptor missing | `document-not-announced` |
| descriptor backend error | `directory` |
| schema/hash mismatch | `schema-hash-mismatch` |
| `ensure_document`, `Database::hello`, `take_welcome`, or Welcome encoding | `storage`, with `error.to_string()` |

Therefore an observed `ServerFrame::Error { code: "storage", message }` would narrow the cause to the last four storage/hello stages; it is not legitimate to assert that it is already proven without retaining the decoded frame. A `directory`/auth/descriptor frame remains theoretically possible at the earlier gates, even though the fixture prepares those inputs.

### Relation to the blob fault

**The two failures are not yet proven to share a root cause.** The blob result is independently and completely explained by the common `DbIoFault` lowering. The private-stream path does not issue `PayloadGet` as its externally visible operation; it creates/opens a document and runs retained hello/bootstrap work. It can pass through DB task faults for catalog/WAL/snapshot work, so a flattened `NotFound`, CAS/fence, admission saturation, or incomplete retirement remains a plausible downstream storage detail, but it is an inference until the frame message is captured.

The hello mechanism has relevant bounded owners but the two-document scenario is below its declared eight-slot limit: `DATABASE_SYNC_HELLO_SLOTS = 8` (`…/🔄️sync/🦀️.rs:290-400`). `Database::hello` pre-admits a retained future, waits it, and turns it into a session (`…/⚙️engine/🦀️.rs:7445-7472`). `DatabaseSyncHelloSession::next_frame` arms close after `Ok(None)` and `Drop` also cancels/arms close (`…/🔄️sync/🦀️.rs:2030-2123`); the close is scheduled in a later bounded callback (`…:1661-1672`). This makes leaked or delayed retirement a worthwhile hypothesis, not evidence of saturation in this two-session test.

### Minimal Sol implementation packet

1. First make the failing assertion diagnostic without weakening it. In the existing test, bind the first `mine_doc` frame and accept only `ServerFrame::Welcome`; panic with the complete unexpected debug representation, including `ServerFrame::Error { code, message }`. This is a strict oracle, not a fallback or an ignored frame. Rerun the exact test and record the resulting code/message in the ticket before selecting a production edit.
2. If the message identifies a DB task error, trace that exact task through `ensure_document`/catalog creation or `Database::hello`/bootstrap and apply the common typed-fault preservation from packet 1. Add a real two-valid-private-document regression proving first document bootstrap reaches `Session`, its hello owner is driven through bounded close, and the second valid document returns `Welcome` then `Session` without relaxing directory isolation.
3. If it identifies CAS/fence, repair the specific create/get race in `HubState::ensure_document` (`…/📦️bin.rs:274-287`) with the existing `AlreadyExists -> document` recovery only; do not turn a genuine fenced/stale generation result into success.
4. If it identifies hello admission or retirement, repair the retained hello close/owner state in `…/🔄️sync/🦀️.rs`, not the directory privacy filter. The regression must saturate exactly eight hello owners, show plus-one refusal, close one owner at a time, and then prove the valid two-document websocket case recovers capacity.
5. In all branches retain the existing protocol order: no presence or directory `Connection` publication before an acknowledged `Welcome` and `Session`; a failed pre-Welcome connection must release its color and not leak private identity/activity. No directory schema, global identity model, or unbounded connection registry is needed.

## Adjacent medium residuals

- **Uniform hub error lowering:** `get_blob`'s explicit `NotFound` plus `_ => 500` bypasses the existing `db_error_status` mapping at lines 294-302. After typed preservation is fixed, a genuine `Unavailable`/`Timeout` will still be incorrectly sent as 500 instead of 503, and input/limit errors as 500 instead of 400. The one-branch lowering above fixes this without widening authority.
- **Response-copy interactivity:** `db_io_pages_into_http_bytes` enforces `HUB_BLOB_MAX_BYTES` and yields while closing pages, but it copies every fragment into one `Vec` with no cancellation observation or yield in the copy loop (`…/📦️bin.rs:499-517`). It is byte-bounded, not unbounded; nevertheless a maximum-size response can monopolize one handler turn. Add a bounded request-cancellation/close path and yield/check point per owned page or fixed chunk, while retaining deterministic close on success, limit rejection, cancellation, and copy failure.

## Current DbIo list heap move — source RED

The `DbIoU64List` heap move removes the prior 4,096-element inline array from
task/result stack frames: its `Option<Box<[u64]>>` retains fixed capacity and
the result/handback/drop paths continue to close the list incrementally
(`🛢️db/🗄️storage/🦀️.rs:1838-1969,2216-2220,2276-2315,3795-3798`). The static
layout law now correctly keeps `DbIoTask`, `DbIoResult`, `DbIoTaskSlot`, and
`DbIoLostOwner` below their stack bounds (`:8139-8147`). The coordinator's
session `9492` additionally observed the real hub pass the former default-stack
abort through readiness and HTTP bind; that runtime evidence does not prove the
new byte accounting.

The heap allocation is no longer represented in the byte ledger. The operation
and process byte caps derive from `size_of::<DbIoTaskSlot>()`
(`:78-84,142-146`), which used to include `[u64; DB_IO_LIST_ITEMS]`. A list
task now reports only `DB_IO_TASK_SLOT_BYTES` while charging logical items
(`DbIoTask::aggregate_credit`, `:2157-2163`), and its 32 KiB backing is
allocated by `DbIoU64List::new` before `db_io_allocate_task` reserves that
aggregate credit (`:1845-1851,3247-3265`). A result transfer can transiently
own replacement and result boxes as well. Thus multiple admitted list tasks can
exceed the former fixed byte ceiling without the ledger seeing their backing.

This remains RED until list-task admission preflights and charges the fixed
heap bytes (and a law proves the operation/process byte caps reject the next
list before allocating or retaining uncharged backing), while preserving the
existing close/handback semantics.

### Heap-credit repair — source-closed; runtime pending

The preceding uncharged-heap RED is superseded in current source. Lists now
begin without a backing allocation; list-task aggregate credit reserves a
conservative two-backing transient byte/item allowance before allocation
(`🛢️db/🗄️storage/🦀️.rs:76-84,1838-1873,2170-2189`).
`db_io_allocate_task` reserves that aggregate credit first, then calls
`admit_list_backing`; an allocation error detaches the exact operation credit
before returning ownership to the caller (`:3268-3300`). This eliminates the
former post-admission uncharged allocation.

The new direct law checks the charged operation ceiling, plus-one rejection,
heap allocation, stack layout, and post-close ledger-baseline restoration
(`:8154-8200`). It is a source-qualified proof only: this audit did not run
the Cargo law or the real hub again, so session `9492` remains evidence only
for the earlier stack-abort repair.

These are adjacent blob lowering/boundedness items only. They do not alter canonical artifact authority, document descriptors, or directory privacy policy.

## Verification to perform after implementation

Run the exact Nx tests at a level whose sanctioned assertion budget accommodates this workspace's current build graph, then record the actual outputs:

```text
bun nx run os-hub:test -- blob_put_get_head_round_trip -- --exact --nocapture
bun nx run os-hub:test -- directory_ws_isolates_private_realtime_activity_and_global_identity -- --exact --nocapture
```

Also run the focused common storage/SQLite fault and page-lifecycle tests added above. A pass claim requires their completed output, not merely successful compilation or test selection.
