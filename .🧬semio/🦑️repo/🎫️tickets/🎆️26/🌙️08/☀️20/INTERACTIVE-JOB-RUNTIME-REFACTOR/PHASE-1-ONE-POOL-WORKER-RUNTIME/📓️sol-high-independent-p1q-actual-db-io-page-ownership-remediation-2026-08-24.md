# P1q Actual Database I/O Page Ownership Remediation

Date: 2026-08-24  
Packet: reopened P1q fresh-audit B1–B6 remediation  
Disposition: **B1–B6 core is source-audit-ready; full P1q acceptance remains deferred until the separately active R4 snapshot/index/WAL/caller packet integrates.**

Inputs read completely:

- `📓️coordinator-independent-p1q-retained-byte-credit-reopen-audit-2026-08-23.md`
- `📓️p1q-actual-db-io-page-ownership-repair-contract-2026-08-24.md`
- `📓️terra-p1q-actual-db-io-page-ownership-acceptance-audit-2026-08-24.md`
- `📓️terra-fresh-independent-p1q-b1-b6-source-audit-2026-08-24.md`

The prior RED audits remain in the ticket unchanged. This report does not claim that R4 or the global P1q gate is green.

## B1–B6 Source Closure

### B1 — one injected pool and typed driver authority

- `MemoryStorage::new` requires the caller's `Arc<WorkerPool>`; production contains no memory-owned `process_worker_pool` construction. The only DB pool constructor is `#[cfg(test)]` fixture support.
- Memory, filesystem, SQLite, PostgreSQL, and Neo4j all register owned `DbIoTaskExecutor` controls and submit typed `DbIoTask` values through `Lane::Io`.
- PostgreSQL and Neo4j hold a generation-qualified `DbIoAsyncTaskLease` across the actual external future. `enter_lane_io_driver_turn`, typed `parts_mut`, `drive_task(...).await`, `leave_lane_io_driver_turn`, and `complete` are ordered and checked against the same `{slot,generation,operation}` authority.
- PostgreSQL reserves `MAX_READ_BYTES` before every raw driver `Vec` result, observes actual allocator capacity, and writes directly into the task-supplied `DbIoPageWriter`; the former temporary result-page graph is gone.
- Neo4j uses native `BoltBytes`, not base64 `String`/decoded `Vec`; the precharged driver reservation observes the immutable Bolt owner before writing into the supplied task writer. Append/truncate use fixed prepared-platform owners with explicit close.

### B2 — real aggregate owners

- `DbIoOperationLedger` is a fixed process authority over page, byte, item, and control credit. One operation identity binds task shell, input/output pages, result lease, async executor lease, driver reservation, retry state, backend control, and terminal close.
- Result and async leases charge their actual shell sizes and controls. Page credit moves atomically between task and backend owner operations; terminal-empty releases a slot only after live credit, task attachment, and result leases are all empty.
- Memory replaces the audited `HashMap`/`BTreeMap` graphs with fixed arrays or one inspected boxed fixed-length backing. `owner_backing_bytes` is charged before backend ownership; WAL/snapshot/payload/catalog/index/lease owners and operation cursors are fixed, and one entry/page/scalar moves per grant.

### B3 — explicit result handback

- Populated `DbIoPageWriter`, `DbIoPages`, `DbIoU64List`, `DbIoLeaseResult`, `DbIoFault`, and `DbIoResultLease` destructors do not return ledger credit or result handback. They move the exact owner into mounted lost-owner maintenance.
- `take` adds a generation-qualified result lease before detaching the terminal. The task slot cannot be reused until explicit nested result close returns that lease.
- Cancel-after-async-take keeps the task detached and close-enqueued. Completion publishes `Cancelled(Some(result))`; mounted close retires that exact result before the cancellation fault can transfer. The hostile fixture inspects this pre-completion state and proves the close does not spin or free the slot.

### B4 — lossless mounted close and exhaustion

- DB close/retry owns fixed typed rings and mounted maintenance; production DB core contains no recursive `callback_at`, opaque retry job, `expect`, `assert`, `unwrap`, or saturating owner-loss path.
- Page/platform ring saturation retains the owner in its slot and raises the permanent retained-fault witness. The generic lost-owner ring exposes a fallible exact handback primitive; production permanently retains the exact full-ring candidate rather than dropping it.
- Retry-generation exhaustion publishes a typed saturated terminal fault. Artifact runner, artifact submit, and artifact history retry generation/deadline overflow terminalize their exact retained authority instead of panicking.

### B5 — mandatory backend close witness

- The registry owns `Option<Box<dyn DbIoTaskExecutor>>`, not an `Arc` last-drop authority. Every backend must complete `close_backend_step` and then prove `backend_terminal_is_empty` before its control/ledger slot returns.
- Lost PostgreSQL facades install and poll the real `PgPool::close()` future from mounted backend-close authority until `PgPool::is_closed()` is true. The no-network lazy-pool law checks the actual pool witness.
- Memory, filesystem, SQLite, Neo4j, and PostgreSQL have explicit lost-facade/control retirement laws; a five-backend law rejects false/stale terminal witnesses.

### B6 — hostile laws

Source fixtures now cover:

- zero, operation/process/page/list/platform maxima, max+1, and exact lost-ring max+1 handback;
- one-byte/high-capacity driver candidate rejection with pointer and capacity intact;
- aggregate before/after witnesses and actual memory submit/take/result/close on one injected pool;
- supplied PG/Neo writer use with observed high-capacity mock driver ownership;
- real queued callback after actual task-slot reuse, including generation/operation ABA rejection;
- retry generation at `u64::MAX`, queued saturation, and lossless typed fault publication;
- cancel before execution, detached cancel before lease completion, receiver drop, panic, backend fault, interrupted close, shutdown, and all-five backend close witnesses;
- actual no-service PostgreSQL lazy-pool and Neo4j configured-control lost-facade retirement.

The root verifier now has an isolated permanent command, `bun ./📜️script.ts verify interactivity p1q-b1-b6`. Its mutations alter executable ordering/ownership shapes for shared-memory pool construction, direct Drop handback, cancellation ordering, async credit, external-driver turn bypass, post-allocation admission, Neo base64 restoration, fake PostgreSQL close, queued ABA, retry exhaustion, fixed-ring capacity, and backend terminal witnesses. The root caller census now also reads state, query, and projection in addition to every previously named DB source.

## Retained Owner Inventory

| Owner | Backing/admission | Exact close witness |
| --- | --- | --- |
| `DbIoPageLease` | one static 16 KiB page; operation-qualified before checkout | one page per explicit/mounted step |
| `DbIoPageWriter` / `DbIoPages` | 64 fixed lease slots; same leases move through range/result | shell plus pages empty; result handback returned once |
| `DbIoOperationLedger` | fixed operation slots and checked process totals | zero live credit, zero result leases, no attached task |
| `DbIoTaskSlot` | fixed task slot with generation and operation | task/result/retry/backend/close all empty before reuse |
| `DbIoResultLease` / `DbIoFault` | charged generation-qualified terminal shell | explicit result/fault close returns handback once |
| `DbIoAsyncTaskLease` | charged task/executor/control shell | driver turn left, executor/task restored, credit returned |
| `DbIoDriverReservation` | maximum pre-reserved, then observed driver backing | rejected or consumed owner retired before credit return |
| `DbIoPlatformBuffer` | 16 static contiguous slots plus page credit | page credit and ABA slot returned explicitly/mounted |
| `DbIoLostOwner` ring | fixed page/control-sized slots | one typed owner per maintenance opportunity; exact full-ring handback |
| `DbIoBackendControl` | 64 fixed owned executor slots | close cursor complete and terminal witness true |
| `DbIoText` / `DbIoU64List` | inline fixed 1,024-byte text / 4,096 scalars | one text/scalar/result handback opportunity |
| Memory backend | fixed arrays and inspected boxed fixed-length WAL backing | one owner/page/cursor entry per executor/close grant |
| PostgreSQL/Neo4j driver result | precharged external owner plus supplied task writer | observed owner dropped, reservation closed, supplied pages transferred |

## R4 Boundary

Snapshot chain/state pages, index keys/runs/sort/merge/scan/compaction, WAL record/replay, and their CLI/sync/cluster/compact/artifact/query/projection callers are owned by the separately active `p1q_r4_snapshot_index_wal` lane. Its packet report is `📓️codex-p1q-r4-snapshot-index-wal-retained-codec-packet-2026-08-24.md`. This B1–B6 handoff deliberately does not edit or claim those regions. A fresh global P1q audit must wait until both packets are integrated.

## Scoped Evidence

- `rustfmt --edition 2021` parsed and formatted storage core, PostgreSQL, and Neo4j after the final owner/ring changes.
- `bun ./📜️script.ts verify interactivity p1q-b1-b6`: clean live-source predicates and hostile mutations.
- Full `bun ./📜️script.ts verify interactivity` progressed past the P1q B1–B6 mutations, then stopped at the unrelated concurrent Puzzle FillBuilder baseline. A global verifier pass is not claimed.
- Production-only static scan of storage core, PostgreSQL, and Neo4j found zero `expect`, `assert`, `unwrap`, `unreachable`, saturating owner arithmetic, or checked-add-then-expect patterns.
- Final `rustfmt --check` is clean for storage core, PostgreSQL, and Neo4j; artifact/engine retry regions also parse through `rustfmt --emit stdout` without mutation of the separately active caller regions.
- Exact-file `git diff --check` is clean for the B1–B6 source, manifest, verifier, and both retained ticket reports.

No Cargo, Nx, Wasm, browser, database runtime, network, broad build, or broad test was run. Compilation, runtime services, target matrices, allocation instrumentation, WorkerPool 1/2/4/default execution, and latency remain serialized global gates.

## B1–B6 Owned Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs` (retry overflow/terminal authority only)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs` (artifact submit/history retry overflow only)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml` (Neo4j base64 dependency removal)
- `📜️script.ts`
- this report and `📓️p1q-r4-platform-syscall-census-2026-08-24.md`
