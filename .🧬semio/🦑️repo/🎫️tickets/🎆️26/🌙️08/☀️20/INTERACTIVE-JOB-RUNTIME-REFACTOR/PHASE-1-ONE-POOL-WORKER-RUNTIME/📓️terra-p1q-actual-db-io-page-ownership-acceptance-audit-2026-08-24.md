# P1q Actual Database I/O Page Ownership Acceptance Audit

Date: 2026-08-24  
Auditor: Terra (independent, read-only)  
Disposition: **RED — P1q is not accepted; P1w/P1x remain blocked.**

## Scope And Evidence Read

- `AGENTS.md`
- `PHASE-1-ONE-POOL-WORKER-RUNTIME/📓️p1q-actual-db-io-page-ownership-repair-contract-2026-08-24.md`
- Sol's `📓️sol-high-independent-p1q-actual-db-io-page-ownership-remediation-2026-08-24.md`
- `📓️p1q-r4-platform-syscall-census-2026-08-24.md`
- live storage, SQLite, PostgreSQL, and Neo4j implementation; actual caller and verifier census

The core storage module does contain a real zero-initialized `[u8; 16 KiB]` page backing arena and
fixed lease arrays. That limited fact does not repair the actual repository boundary or prove the
required owner/credit lifecycle.

## Acceptance Blockers

### B1 — Async-native Backends Bypass The Typed Task/Result/Lane Boundary

PostgreSQL and Neo4j do not register a `DbIoTaskExecutor`, own a `DbIoBackendControl`, or submit
their work through `submit_db_io_task(... Lane::Io ...)`. Their public storage trait methods invoke
their drivers directly. PostgreSQL receives raw `Vec<u8>` driver results and only then starts a
page copy cursor at `🐘️postgres/🦀️component.rs:246-248,313-314,356-357,380-382,430-431`.
Neo4j decodes whole driver strings into raw `Vec<u8>` at `🌐️neo4j/🦀️component.rs:62-64` and then
copies them into pages at `:440-448,502-509,546-552,575-584,621-628`.

This contradicts the contract's explicit requirement that async-native backends use the same owned
task/result page protocol at the repository boundary and that external drivers cannot allocate an
uncensused repository result graph. The R4 census's claimed "admitted page-copy output" is not
what the source does: the uncensused driver `Vec` exists first. A logical length ceiling does not
census its allocator capacity, result owner, cancellation, or terminal retirement.

### B2 — Fixed Arena Is Not Coupled To Exact Aggregate Operation Credit

The only process-wide DB I/O counter is `BLOCKING_QUEUE`, and every task calls
`enqueued(0)`/`dequeued(0)` (`🗄️storage/🦀️component.rs:1471,1853-1856`). There is no production
operation/page/item/byte credit ledger. `DB_IO_OPERATION_BYTES` is test-only (`:1882`). Page
leases and task slots independently obtain a new `db_io_next_operation()` identity
(`DbIoPageWriter::try_reserve` at `:255-257`; `db_io_allocate_task` at `:1422-1427`), so a task
operation is not the operation identity of its input/output pages.

Consequently there is no aggregate operation witness that task, input, writer, result, retry,
control, and close have returned to an exact prior value. This fails the contract's required
operation/process credit ownership and exact terminal-empty aggregate proof.

### B3 — `take` Detaches Result Owners Before Close

`DbIoTaskOperation::take` enqueues close, takes `owner.terminal`, and returns its typed result to
the caller (`🗄️storage/🦀️component.rs:1728-1752`). The close cursor subsequently sees no terminal
result and can free the task slot (`:1828-1875`) while the caller still owns `DbIoResult::Pages`.
That result can only be explicitly closed separately or be sent into the page Drop retirement ring.

Thus the operation shell may become terminal-empty and its slot reusable before its output pages
are closed. This is the exact forbidden independent/implicit result retirement path; it also makes
the claimed aggregate credit restoration impossible.

### B4 — Close Is Unbounded Self-Scheduling With Opaque Callback Jobs

`db_io_submit_close_job` retries every submission failure through `WorkerPool::callback_at` with
no attempt limit or retained typed retry authority (`🗄️storage/🦀️component.rs:1500-1508`).
`db_io_schedule_close` then recursively submits a closure which calls the global close step and
reschedules itself while the slot is active (`:1511-1535`). The closure and timer callback are not
represented in the fixed task slot's `retry_job` or a close-owner cursor.

This violates the fixed retained retry/terminal close contract and the specific audit requirement
to reject self-scheduling close. It is also not a durable lost-handle recovery mechanism: it
depends on the WorkerPool callback queue remaining live.

### B5 — Backend Control Is Handed Back By Opaque Arc Drop

The backend registry stores `Option<Arc<dyn DbIoTaskExecutor>>` (`🗄️storage/🦀️component.rs:1234-
1237`), and `unregister_db_io_backend` replaces it with `None` (`:1302-1313`). No typed
`close_backend_step` exists; the only hook is operation cleanup. Therefore the final backend
control, external connection/driver state, and any retained executor graph are retired by implicit
Arc last-drop, contrary to the required one-owner backend close handback. SQLite's implementation
confirms its hook is only per-operation stage/hash cleanup (`🪶️sqlite/🦀️component.rs:481-495`).

### B6 — Required Capacity And Hostile-Lifecycle Fixtures Are Missing

The permanent P1q fixtures cover logical page count, not source allocation capacity. The helper
accepts `&[u8]` and calculates only `bytes.len()` (`🗄️storage/🦀️component.rs:4010-4015`); no P1q
storage/backend source calls `capacity()`. There is no hostile one-byte/high-capacity source test,
no actual async-native backend law fixture, and no mutation that kills the direct driver `Vec`
paths above. The existing ABA fixture only compares locally constructed fields (`:4098-4105`) and
does not exercise a stale callback against a reused slot.

The live verifier misses these defects by design: it reads storage/SQLite/engine/testkit/hub but
not the PostgreSQL or Neo4j source files (`📜️script.ts:5863,6634-6740`) and its P1q acceptance
checks are substring predicates. Its green output is therefore not acceptance evidence for the
contract's actual-caller requirements.

## R4 Assessment

The R4 census accurately defers timings, but its async-native assertion is not source-faithful:
the PostgreSQL/Neo4j whole driver result allocation and Neo4j base64 decode precede every retained
page cursor. The listed syscall boundary is therefore not "prepared syscall only" in the sense
required by P1q-R4.

## Scoped Checks Run

| Check | Result |
| --- | --- |
| `rustfmt --edition 2021 --check …/db/🗄️storage/🦀️component.rs` | pass |
| `git diff --check -- …/db/🗄️storage/🦀️component.rs` | pass |
| `bun ./📜️script.ts verify interactivity` | process exit 0 / reports deny-mode clean; insufficient, as B6 explains |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | `self-tests=328 clean`; unrelated to actual async DB boundary coverage |

No Cargo, Nx, Wasm, browser, runtime/backend integration, or network command was run.

## Required Disposition

Do not start P1w/P1x/P1y/P1z. Repair all B1–B6 in the shared P1q boundary, expand the verifier to
scan every named backend and caller, then request a new independent source audit.
