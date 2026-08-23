# P1q Actual Database I/O Page Ownership Repair Contract

Date: 2026-08-24  
Owner: next Sol High foundation executor after the current P6g packet  
Status: prepared, reopened source packet not accepted  

## Packet Boundary

This packet repairs reopened P1q-R1 through P1q-R3: actual page ownership, typed operation
admission, and bounded terminal close. It must replace the deficient shared boundary used by every
database backend and caller. P1w/P1x catalog CAS work stays blocked until this packet is
independently accepted.

P1q-R4 remains a separately audited platform packet: platform syscalls may be one explicitly
indivisible I/O opportunity, but all in-memory copy/hash/encode/decode work on either side must use
the page cursors introduced here. This packet must not add a database-owned pool, scheduler,
thread, blocking fallback, unbounded queue, permanent script, external runtime dependency, legacy
API, compatibility adapter, or generic opaque retained graph.

## Exact Current Defects

### Logical pages are not storage pages

`DbIoPages` currently owns one standard `Vec<u8>`, a logical start offset, and a logical page count.
`try_new` and `try_range` validate `len`, not `capacity`; `page()` returns borrowed slices into that
same allocation. A one-byte vector with a process-cap-sized capacity is admitted as one page.
Allocator rounding/control ownership is invisible.

`into_vec` returns the original vector only for start zero. A ranged owner allocates and copies the
suffix after admission. Ordinary `DbIoPages`/rejection Drop recursively releases the whole backing
without an exact page-close/credit transition.

### Requests are estimates, not owner admission

`DbIoRequest::admitted_bytes` trusts caller-supplied input/output byte estimates plus a scalar list
estimate and a decorative base page. It does not own or census captured paths, document IDs, keys,
backend/control handles, input capacity, nested list/result strings, or output backing.

`run_blocking_op<T, F>` accepts arbitrary `FnOnce` and arbitrary result `T`. Neither type carries the
claimed reservation. A caller may understate captured input; a backend may allocate a larger or
capacity-heavy result; the bridge cannot inspect either before ownership transfer.

### Terminal close is opaque and recursive

`DbIoState::close_one` takes and drops a generic WorkerPool job, retry job, closure, terminal
closure, or `Result<T, DbError>` in one grant. Those values can hide multiple pages, strings,
paths, keys, Arcs, vectors, maps, and backend owners. `DbIoAdmission` itself releases all operation
credit from `Drop`, independent of per-owner retirement.

`DbIoOperation::Drop` calls one close step and may leave the remaining state dependent on incidental
Arc destruction. The empty witness observes fields after opaque destruction; it does not prove one
owner/page per grant or exact aggregate credit return.

## Required Greenfield Boundary

### Actual page type and fixed arena

Replace the logical wrapper with an actual owned page authority. One page is an owned, fixed-size
`DB_IO_PAGE_BYTES` backing plus an explicit used length. It must not be a view into a larger
allocation. Use one process-owned fixed page arena initialized zero-touch with a fixed slot array,
generation/ABA tag, free ring, checked operation/process counters, and actual page backings.

`DbIoPages` must be a fixed-capacity ordered collection of page leases plus first/last offsets and
total length; it may not contain an ordinary dynamic collection. Page MAX succeeds. Page MAX+1
returns the exact input or writer authority before transfer. Page lease Drop must fail closed unless
the lease was explicitly handed back or installed in a durable reclaimer.

The process arena must distinguish free, checked-out input, checked-out writer, queued work,
executing, terminal result, rejected, and closing states. A page credit is charged before allocation
or checkout and returned only after the exact page backing/lease is terminal-empty. Generation zero
and counter exhaustion are non-admissible; reused slots reject stale handles.

### Retained page writer and reader

All new input/output construction uses a retained `DbIoPageWriter` with explicit operation and
aggregate page reservations. Each write/copy/hash/encode opportunity consumes at most one bounded
fragment and checks cancellation/deadline. Full-page handoff is constant-time; final partial-page
handoff is explicit and atomic.

Readers expose page/offset cursors and bounded fragments. Range construction must retain a view of
the same page leases or move exact pages plus offsets; it must never allocate/copy a suffix. If a
caller truly needs a new contiguous platform buffer, that conversion is an explicit P1q-R4
prepared-buffer cursor with its own admitted pages, not `into_vec`.

Remove `into_vec` and all APIs whose only implementation requires post-admission whole-copy
materialization. Greenfield trait outputs that currently return raw `Vec<u8>` must return the owned
page authority or another schema-owned fixed result. No legacy raw-vector facade remains.

### Schema-first task taxonomy

Remove `DbIoRequest` estimates and the generic `run_blocking_op<T, F>` production API. Define an
owned, schema-first task taxonomy for the database storage operations, including metadata/open,
WAL create/append/sync/seal/read/length/list/truncate/delete, snapshot write/read/latest/list/delete,
payload put/get/exists/delete, catalog read/CAS, index write/read/list/delete, leases, and backend
close.

Each task variant must own only repository types and exact page/string/control authorities. Dynamic
paths, document IDs, hashes, keys, list limits, expected epochs, and input pages are admitted before
the task enters the shared I/O lane. Backend-specific or external-driver values remain behind an
owned repository interface and cannot appear in exported task/result types.

Use corresponding typed result variants whose output writers/pages are allocated from the same
reservation. A backend cannot return arbitrary `T`; it must fill the provided bounded writer or a
fixed scalar/list authority. Attempting to exceed reserved items/pages/bytes yields a retained
terminal fault with the exact partial writer, never an outside allocation.

### One shared WorkerPool I/O lane

Every blocking backend submits the typed task through the existing process WorkerPool `Lane::Io`.
There is no inline/pool-less fallback and no backend-owned scheduler. Queue admission must reserve
the exact task/control/input/output/retry/terminal/close working set before transfer. Saturation or
contention returns the same task owner or retains it in a fixed retry slot; it cannot drop/rebuild the
task.

Async-native backends must still use the same owned task/result page protocol at their repository
boundary. External futures/drivers do not gain permission to allocate uncensused repository result
graphs.

### Explicit execution and terminal state

Replace generic closure state with a fixed task slot whose phases include `Admitted`, `Queued`,
`Executing`, `Completed`, `Cancelled`, `Faulted`, and `Closing`. All phases are tagged by slot
generation and operation identity. Retry callbacks validate both before moving a task.

Execution consumes the exact task owner once. The backend writes through the admitted result
authority. Completion atomically publishes one typed terminal handle without freeing input/output
credit. Cancellation before execution moves the intact task to terminal ownership; cancellation
during a syscall retains the exact returned/partial result for close and publishes `Cancelled`
without losing the backend outcome.

Provide exact `take`, `resume` where retry is valid, and `close_step` methods for terminal task,
retry job, backend control, input strings/pages, output strings/pages/list slots, fault detail,
operation shell, and process credit. Production callers must not synchronously drain terminal
state.

## Bounded Close and Lost-handle Recovery

Every dynamic or page-backed field needs an explicit close cursor. One close grant retires at most
one semantic owner or one page/control backing and returns only that exact real credit. Ordinary
`Drop`, whole `clear`, `truncate`, vector/map replacement, closure destruction, generic result
destruction, and implicit Arc last-drop are not terminal retirement mechanisms.

Dropping the future, receiver, retry handle, terminal handle, backend, database, or pool during
partial close must durably enqueue the same slot/generation/cursor in a fixed process retirement
arena. Maintenance and application/database close must rediscover it without resetting or
duplicating retirement. A false terminal shell or duplicate credit return is fail-closed.

Aggregate operation/page counters return to their exact prior values only after the exhaustive
empty witness proves that task, retries, execution owner, results, pages, strings, backend controls,
fault, waker, and terminal handles are all gone.

## Caller Migration

Migrate every exact current `DbIoPages`, `DbIoRequest`, and `run_blocking_op` caller in one packet,
including memory, filesystem, SQLite, PostgreSQL, Neo4j, testkit, WAL, snapshot, payload, catalog,
index, compaction, cluster snapshot, and engine bootstrap/persist routes. Delete the old raw/generic
APIs; do not leave adapters.

P1w/P1x may begin only after their catalog encoders write directly into this accepted writer and
catalog CAS consumes/returns exact page authorities. P1y/P1z must use the same boundary rather than
inventing compaction/sync-specific retained byte containers.

## P1q-R4 Platform Boundary Rules

This packet must cursorize all repository-visible preparation and aftermath: path/key construction,
page copying, hashing, checksum, framing, encoding/decoding, list parsing/sorting, and result
construction. The remaining indivisible platform call must accept already prepared buffers and
return into already admitted buffers wherever the OS API allows.

Record each unavoidable syscall in a follow-on P1q-R4 census with platform, maximum buffer/page
count, cancellation semantics, and observed max/p99 duration. A syscall label cannot conceal a
whole repository buffer allocation/copy before or after the call.

## Hostile Permanent Fixtures and Mutations

Add permanent fixtures proving:

1. a one-byte source whose original vector capacity exceeds operation/process caps is rejected with
   exact owner identity before page conversion;
2. page MAX succeeds and page/operation/process MAX+1 rejects without partial transfer;
3. ranged owners preserve the same page leases and perform zero suffix allocation/copy;
4. captured path/key/document/backend control and nested list/result capacities are all represented
   by exact item/page/byte credit;
5. a backend cannot write one byte/item/page above its result reservation;
6. saturation/contended retry retains the exact task and ABA-stale callbacks cannot consume a
   reused slot;
7. cancellation before/during execution, panic, backend fault, future/receiver Drop, and shutdown
   preserve the exact task/result owners and reach terminal-empty close;
8. interrupting close after every owner/page resumes the same cursor once and returns aggregate
   credit exactly once;
9. memory/filesystem/SQLite and async-native backend law suites return the same typed semantics;
10. no raw `Vec<u8>` storage result, logical page view, `into_vec`, generic retained closure/result,
    recursive close, inline fallback, backend pool, unbounded queue, or whole repository buffer copy
    remains reachable.

Mutations must restore logical-length credit, decorative page slices, suffix `to_vec`, estimated
requests, arbitrary generic closure/result ownership, outside-writer output allocation, bulk
terminal Drop, eager admission release, stale retry consumption, lost-handle orphaning, and whole
in-worker copy/hash/encode. The focused mutation target must kill every mutation.

## Acceptance Evidence

Source acceptance requires an independent Terra read-only audit of the final diff, exact caller
census, retained-owner/page inventory, close-state inventory, and mutation inventory. No broad
Cargo/Nx/Wasm/browser command may run while overlapping Rust packets are active. The later
serialized immutable-tree owner must capture:

- focused database storage/engine debug and release tests plus strict warnings through Bun/Nx;
- real WorkerPool 1/2/4/default replay and all backend law suites available locally;
- page/operation/process MAX/MAX+1, saturation, retry, cancellation, panic, stale generation,
  lost-handle, and close-drain evidence;
- native, `wasm32-unknown-unknown`, and `wasm32-wasip2` compile/behavior gates for applicable
  backends;
- exact allocation/page counters before admission and after interrupted terminal close;
- worker-step max/p99 below 8 ms for all repository-side cursors, with separate P1q-R4 syscall
  latency evidence;
- deterministic page/result bytes and catalog/WAL/snapshot/payload/index parity.

Passing P1q re-enables P1w/P1x source work but does not close Phase 1. The final Phase 1 and master
runtime matrices remain required.
