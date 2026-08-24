# Fresh Independent P1q B1–B6 Source Audit

Date: 2026-08-24  
Auditor: Terra (fresh, read-only)  
Disposition: **RED — P1q is not accepted; P1w/P1x/P1y/P1z remain blocked.**

## Evidence Read

- `AGENTS.md`
- `📓️p1q-actual-db-io-page-ownership-repair-contract-2026-08-24.md`
- prior Terra RED audit, Sol remediation report, and P1q-R4 census in this ticket
- live root verifier, hub source references, storage core, memory/filesystem/SQLite/PostgreSQL/Neo4j, engine, snapshot, index, WAL, SPR, pack, sync/cluster/compaction/testkit caller census

Scoped parse/style and whitespace checks were run only on the P1q storage/backends/codecs:

- `rustfmt --edition 2021 --check` on storage, SQLite, PostgreSQL, Neo4j, snapshot, index, and WAL: exit 0.
- `git diff --check --` on the same P1q files and root script: exit 0.

No Cargo, Nx, Wasm, browser, network, database integration, or broad runtime/build gate was run.

## Blocking Source Findings

### B1 — Not Every Backend Executes The Typed Operation On `Lane::Io`

`MemoryStorage::new` creates its own `process_worker_pool(...HeadlessBatch, 2)` at
`🛢️db/🗄️storage/🦀️component.rs:4866-4869`; it does not accept/use the application’s shared pool.
That is a backend-owned scheduler, expressly prohibited by the contract. Its executor also constructs
heap `ArtifactId`/`String` keys during task execution (`:4468-4475`, `:4478-4481`, `:4780-4788`)
outside the typed fixed text/item ownership taxonomy.

For async-native controls, the `Lane::Io` job only transitions the task to `async_ready`
(`🛢️db/🗄️storage/🦀️component.rs:2366-2388`). The caller-facing facades then take the executor and
run external driver I/O themselves: PostgreSQL `:771-780`; Neo4j `🌐️neo4j/🦀️component.rs:1018-1027`.
Thus `submit_db_io_task` precedes the operation, but the actual async operation is not executed by the
registered `Lane::Io` worker.

The external-result path is also not the supplied task output authority. PostgreSQL receives a raw
`Vec<u8>` after the query (`🐘️postgres/🦀️component.rs:259-262`, `:327-329`, `:371-373`,
`:397-400`, `:444-449`). `db_io_copy_observed_bytes` observes it only after allocation, creates a
new temporary page writer at `🛢️db/🗄️storage/🦀️component.rs:709-729`, then the async task later
copies those pages again into the pre-admitted `output` via `db_io_transfer_pages`. This is a
post-driver, post-result-allocation temporary page graph rather than driver output into the supplied
admitted writer.

Neo4j likewise obtains a driver `String`, decodes it using `BASE64.decode` into a raw `Vec<u8>`
(`🌐️neo4j/🦀️component.rs:119-139`), and only then turns it into pages (`:521-533`). Its append path
materializes decoded prior bytes and a whole encoded `String` (`:475-503`). Reserving an upper bound
does not make those values repository-owned page/task authorities before their allocation.

### B2 — The Ledger Does Not Bind All Real Owners Or Give An Exact Terminal Witness

The fixed ledger is real (`🛢️db/🗄️storage/🦀️component.rs:83-259`), but it credits a scalar task
estimate (`DbIoTask::aggregate_credit`, `:1572-1578`) and fixed task-slot size, not every actual
owner created by the implementation. The memory backend has uncensused dynamic `HashMap`, `BTreeMap`,
`ArtifactId`, `String`, and `LeaseInfo` storage (`:3892-3902`, `:4468-4481`, `:4780-4788`), while
the async drivers create external `Vec`/`String` values before source conversion. None has an
operation-qualified fixed owner/close cursor in `DbIoOperationLedger`.

The contract requires one ledger binding task/input/output/result/retry/backend identities and exact
prior counters only after terminal-empty. The observed source instead tracks the typed shell/pages
while permitting uncredited dynamic backend and codec graphs, so that witness cannot prove the
required aggregate claim.

### B3 — Explicit Result Closure Is Still Bypassed By Ordinary `Drop`

The result lease correctly delays operation-ledger release, but actual output/fault handback is still
performed by implicit destructors: `DbIoPages` (`:1116-1125`), `DbIoU64List` (`:1403-1408`),
`DbIoLeaseResult` (`:1510-1514`), `DbIoFault` (`:2058-2064`), and `DbIoResultLease`
(`:2483-2496`). `DbIoPageWriter::Drop` also returns shell credit directly (`:614-620`). Those are
exactly the ordinary-drop terminal paths forbidden by the contract; no explicit close lease is
required before an owner’s credit/handback changes.

There is also a cancellation race not covered by the fixture: `cancel` publishes
`Cancelled(None)` after async readiness (`:2606-2624`) even though `take_async_native` has removed
the task from the slot (`:2627-2653`). Until the detached async lease later completes, close sees no
task and a non-abandoned cancelled terminal, and returns forever at `:2808-2827`. The supplied law
only calls `take` after `lease.complete` (`:6252-6266`), so it does not exercise this retained-result
race.

### B4 — Lost Owners, Saturation, And Generation Exhaustion Are Not Fail-Closed

The old callback recursion is gone, but its replacement still has prohibited implicit retirement:
`DbIoPageLease::Drop` places the page in the retirement ring (`:397-402`) and
`DbIoPlatformBuffer::Drop` does the same (`:932-945`), while the types above directly return
credits/handbacks from `Drop`.

More seriously, both fixed lost-owner rings panic on saturation rather than retaining a recoverable
typed fault/owner: page `assert!` at `:382-394`, platform `assert!` at `:932-944`. Retry-generation
exhaustion likewise panics through `.expect` at `:2286-2295`. These are concrete panicking-owner-loss
paths, not the required bounded mounted retirement with a terminal fault and exact ABA identity.

### B5 — Lost PostgreSQL Facade Cannot Close Its Backend Control

Normal PostgreSQL close performs the real async `self.pool.close().await` only in the facade-owned
`BackendClose` task (`🐘️postgres/🦀️component.rs:715-718`, `:783-788`). A lost facade merely calls
`retire_db_io_backend` from `Drop` (`:793-798`). Maintenance then invokes
`PostgresDbIoExecutor::close_backend_step`, which only sets an atomic flag (`:740-743`), but its
empty witness also requires `self.pool.is_closed()` (`:745-747`). The registry rejects that as a
false terminal witness (`🛢️db/🗄️storage/🦀️component.rs:1983-1993`) and cannot retire the control.

Therefore the required lost-facade/shutdown one-owner close proof is false for one of the five named
backends. It is not repaired by the otherwise useful `Box<dyn DbIoTaskExecutor>` registry.

### B6 — Required Hostile Laws And Actual-Caller Coverage Are Incomplete

The hostile tests are source-local constructions, not the actual PostgreSQL/Neo4j services; the
"async-native" law registers `AsyncNativeLawExecutor` (`🛢️db/🗄️storage/🦀️component.rs:6217-6233`).
No fixture establishes the real driver `Vec`/`String` capacity/rejection/retirement behavior,
memory’s owned-pool bypass, loss-ring saturation, retry-generation exhaustion, or the cancellation
race above. The stale-ABA test sets a slot struct directly (`:6173-6187`) rather than driving a
queued callback after a real reuse.

## R4 And Direct-Writer Census Is RED

The four named encoders have retained-writing fragments, but the claimed zero production whole-owner
census is not source-faithful:

- Snapshot incremental publish calls `materialize_chain`, which retains every generation in
  `Vec<DbIoPages>`, reverses it, reserves one full `Vec<u8>`, and copies every fragment at
  `📸️snapshot/🦀️component.rs:664-719`. It has no cancellation/deadline authority or explicit close
  of those page owners.
- Index production uses raw public `Vec<u8>` keys/values and whole batch sorting/collection in
  `🔢️index/🦀️component.rs:117-148`, then does whole run decode into `Vec<RunEntry>` at `:317-353`.
  `put_batch`, `scan_prefix`, automatic merge, and compaction collect/merge whole runs at
  `:470-578`, including `sort_by`, `collect`, cloning, and full result vectors.
- WAL still exposes `WalRecord` variants containing raw `Vec<u8>`, `String`, and `Vec<u64>`
  (`📝️wal/🦀️component.rs:170-200`), collects decoded records (`:620-630`) and the whole document
  replay (`:649-662`). These repository-visible paths have no task page reservation or cancellation
  owner.
- Neo4j’s R4 base64 read/decode/materialize path is explicitly whole `String` + whole `Vec` before
  page transfer (`🌐️neo4j/🦀️component.rs:119-139`, `:521-533`), and append constructs one whole
  base64 string (`:475-503`).

Consequently no assertion is made that catalog/index/snapshot/WAL preparation/aftermath is wholly
incremental, deadline/cancellation bounded, or free of hidden whole `Vec`/`String`/sort/collect
work. The R4 census must remain RED.

## Root Verifier Is Not An Acceptance Oracle

The root verifier now reads more named files (`📜️script.ts:6048-6090`), but its P1q predicates are
source substring checks, not semantic source analysis. For example, B1–B6 acceptance asks for names
such as `DbIoOperationLedger`, `take_async_native`, and `DbIoDriverReservation`
(`:7081-7119`), while the mutation suite replaces those same literals (`:7171-7201`). It does not
detect that the facade runs `executor.drive_task(...).await` outside the lane, that memory constructs
its own pool, that `Drop` returns credit, or that the real Postgres lost-facade witness is impossible.

Its direct-writer check only bans a short exact list and requires marker strings
(`:7221-7253`); it never audits production `materialize_chain`, `build_run`, `decode_run`,
`merge_runs`, or WAL replay. Caller migration checks only six legacy substrings (`:7204-7218`).
Thus passing its self-mutations cannot discriminate the live counterexamples above.

## Required Disposition

Do not unblock P1w/P1x/P1y/P1z. A future remediation must remove the memory-owned worker pool,
move actual async execution into the shared protocol/lane boundary, eliminate raw post-driver and
codec graphs or represent them as pre-admitted closeable owners, make all loss paths non-panicking
and explicitly cursor-retired, repair PostgreSQL lost-facade close, and extend source/runtime laws
to the cited real paths before requesting another independent audit.
