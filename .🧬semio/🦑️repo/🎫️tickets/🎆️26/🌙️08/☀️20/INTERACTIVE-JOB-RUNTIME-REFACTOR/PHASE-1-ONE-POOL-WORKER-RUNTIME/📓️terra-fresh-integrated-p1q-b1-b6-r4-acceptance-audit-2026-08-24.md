# P1q Fresh Integrated B1–B6 and R4 Acceptance Audit

Date: 2026-08-24  
Auditor: Terra, independent source-level acceptance pass  
Disposition: **RED — P1q is not acceptable for P1w**

## Scope and method

This is an adversarial audit of the integrated live source, not a restatement of the remediation reports. I read the governing master plan, the P1q repair contract, both earlier/fresh Terra RED reports, the core remediation report, the R4 packet and syscall census, and the coordinator residual checkpoint. I then inspected the current core, PostgreSQL, Neo4j, snapshot, index, WAL, state, artifact, query, engine, compaction, CLI, and pack sources.

Only source/static checks were run because overlapping Rust/UI work is active. No Cargo, Nx, build, runtime, ticket API, source, or test edit was performed. The sole mutation from this audit is this report.

## Checks actually run

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | Pass: `live-source and hostile mutations clean` |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` for the audited Rust packets | Pass; no output |
| Scoped `git diff --check` and cached diff check | Pass; no whitespace errors |
| Scoped current and cached `git diff --name-status` over P1q paths | No `D` entries; changed files are modifications only |
| Adversarial source predicates / ownership trace | **Fail**: findings below |

The passing verifier establishes only its present source/mutation predicates. It does not establish actual executor affinity, every repo-owned allocation, or exact terminal closure, so it cannot override the failures below.

## B1–B6 verdict

| Gate | Verdict | Live-source evidence |
| --- | --- | --- |
| B1 — actual driver I/O is one shared `Lane::Io` execution | **RED** | `db_io_drive_one` marks an async-native task ready, but does not drive its future. `PostgresStorage::execute` and `Neo4jStorage::execute` call `take_async_native`, then directly await `executor.drive_task(...).await` in the facade caller future. `enter_lane_io_driver_turn` only flips bookkeeping; it does not place the future on a `WorkerPool` IO worker. |
| B2 — exact ledger covers the complete working set | **RED** | PostgreSQL and Neo4j task dispatch repeatedly creates `ArtifactId(document.as_str().to_string())` after admission. `DbIoLeaseResult::into_lease_info` and both facade `current` paths create raw heap `String` values for `LeaseInfo`. These are repo-owned operation/output values with no operation credit or retained close owner, and are not a named frozen protocol schema exception. |
| B3 — every terminal path uses exact incremental close/handback | **RED** | `DbIoLeaseResult::into_lease_info` synchronously drains `holder` and `resource` with `while ... close_step()`, then handbacks. Facade `current` constructs raw `LeaseInfo` instead of retaining and explicitly closing the result owner. This is a normal result boundary, not a mounted one-grant close state machine. |
| B4 — loss containment / no panic-on-capacity path | **PASS, source-only** | Fixed `DB_IO_LOST_OWNERS` ring, saturation faulting, preserved owner retention, bounded retry generation, and no production core `assert!`/`expect` were found. The exact max-plus-one lost-owner law remains present. |
| B5 — backend close remains reachable without service calls | **PASS, source-only** | Backend registry retains typed executor ownership. PostgreSQL polls `PgPool::close()` to `is_closed`; facade drop retires the backend into mounted maintenance. PostgreSQL/Neo4j no-service lost-facade laws remain present. |
| B6 — hostile laws provide one-to-one proof | **RED** | Many useful core fixtures remain, but they do not exercise the actual facade driver future on `Lane::Io`, the `ArtifactId`/`LeaseInfo` allocation graph, or the raw result-boundary close path. The verifier passing is therefore insufficient. |

## Core blocking evidence

### B1: the actual async driver runs on the facade caller executor

`🗄️storage/🦀️component.rs` `db_io_drive_one` advances an `AsyncNative` task to ready state. `DbIoTaskOperation::take_async_native` then gives the caller a lease containing the typed executor and task. Both `🐘️postgres/🦀️component.rs` `PostgresStorage::execute` and `🌐️neo4j/🦀️component.rs` `Neo4jStorage::execute` perform the actual `executor.drive_task(...).await` after that extraction.

Calling `enter_lane_io_driver_turn` around that await is not worker affinity: its state is an `async_lane_turn` bookkeeping flag. The actual SQLx/Neo4j future therefore is not driven by the shared worker pool's `Lane::Io` worker. The mock async-native law does not cover this facade path.

### B2: uncaptured operation/output heap is still created after admission

The PostgreSQL and Neo4j `drive_task` match arms convert captured document text into new `ArtifactId(document.as_str().to_string())` values for database trait calls. The operation ledger has no owner/credit/close cursor for those strings.

`DbIoLeaseResult::into_lease_info` likewise emits a raw `LeaseInfo` containing two newly allocated strings. PostgreSQL and Neo4j `current` construct the same raw shape directly. These values are neither exact pre-admitted external driver buffers written directly into a charged writer nor frozen `protocol::MutationEnvelope` wire fields. The pre-reserved, observed external SQLx `Vec` and Neo4j `BoltBytes` lanes are not independently rejected here; the uncredited repo-owned identifier/result allocation is.

### B3: result conversion bulk-drains owners

`DbIoLeaseResult::into_lease_info` contains a synchronous `while self.holder.close_step() || self.resource.close_step() {}`. That drains arbitrary retained work in an ordinary production accessor without a worker context, control grant, or persisted resume cursor. On the facade `current` paths, raw result construction then relies on ordinary result dropping rather than an explicit terminal close boundary. The newer cancellation-state handling appears improved; this finding is specifically the success/result conversion and close mechanism.

## R4 retained-streaming verdict

| Packet | Verdict | Blocking evidence |
| --- | --- | --- |
| Snapshot | **RED** | `SnapshotChainCursor::read_page` reserves then calls production `PackFile::read_chunk`, which returns a full `Vec<u8>` before observation/copy. Exact admission after the fact does not turn this generic full payload materialization into streaming. Frozen `SnapshotDescriptor` metadata remains an acceptable schema exception. |
| Index | **RED** | `decode_run_pages` executes `while pages.close_step()?.is_some() {}` after decode, with no grant/cursor/mounted close state. Ordinary, malformed, cancellation, and fault paths reach the same hidden full drain. |
| WAL | **RED** | `WalReplayCursor::next` loops `while self.close_segment_step().await? {}` when a segment ends. One `next` call can drain the entire retained segment without yielding a close phase. `WalBytesCursor::text -> String` is treated as its documented frozen boundary, not this failure. |
| State | PASS, source-only | Fixed retained map, explicit entry closure, and admission gates are present. This does not cure callers that bulk-close returned graph owners elsewhere. |
| Artifact | **RED** | Snapshot/WAL decode and failure paths contain direct `while ... close_step` graph drains. |
| Query | **RED** | `impl Drop for QueryRows` loops `while matches!(self.close_step(), Ok(true)) {}`. `QueryStream` has no corresponding explicit mounted close/terminal witness for ordinary partial consumption. |
| Engine | **RED** | `ArtifactHandle::query` performs nested `while results.close_step()?` / `while value.close_step()?` sweeps on error, rejection, and stale-frontier paths. |
| Pack | Conditional pass | The old test-only `PackWriter::write_chunk(&[u8])` is not the blocking production path; identity fragment APIs exist. Production `PackFile::read_chunk -> Vec<u8>` nevertheless blocks snapshot's reader until a fragment reader replaces that use. |

Compaction and CLI contain the same direct production close-loop pattern. A close step may itself be bounded, but repeatedly running it to terminal inside an ordinary call is still a whole retained graph drain and has no durable externally schedulable resume point.

## Law coverage and preservation audit

No in-scope legacy test/module deletion was observed in either current or cached diff status. Named core hostile laws remain for page caps, high-capacity input, aggregate ledger witness, range/process caps, exact lost-owner ring overflow, interrupted close, queued-task ABA, retry cap, mock drivers, all five controls, actual memory I/O, lost backend, cancellation/drop, and panic/shutdown.

That is preservation, not one-to-one acceptance coverage. The following required laws are absent or do not reach the live production path:

| Required law | Coverage result |
| --- | --- |
| Actual PostgreSQL/Neo4j driver future is polled by a `WorkerPool` `Lane::Io` worker | Missing; mock executor is not the facade/driver path |
| Every post-admission identifier/result heap owner has a ledger credit and close witness | Missing for `ArtifactId(...to_string())` and `LeaseInfo` |
| Lease-info ordinary success, fault, cancellation, and drop uses explicit incremental close/handback | Missing; raw result plus bulk close/drop path |
| Query partial-consumption ordinary drop does not bulk-drain | Missing; live `Drop` does exactly that |
| Index decode success/fault/cancel closes one admitted page owner per step | Missing; live unconditional loop |
| WAL segment-boundary close is resumable across `next` calls | Missing; live loop |
| Snapshot pack read never materializes a generic full payload before fragment admission | Missing; live `read_chunk -> Vec<u8>` path |
| Engine/artifact stale/error cleanup is mounted/resumable | Missing; direct nested close loops |

## Bounded repair packets

1. **Core async-driver placement:** make the actual PostgreSQL/Neo4j driver future a pool-owned `Lane::Io` task/poller. The facade may submit/observe but must not await `drive_task` itself. Add a law that records the worker lane/thread at every actual mock-driver poll through the real facade path.
2. **Task/result ownership:** remove post-admission `ArtifactId(String)` conversion by accepting retained/borrowed captured text at the DB boundary, or admit that owner before construction and retain/close it in the operation ledger. Replace raw `LeaseInfo` allocation with a retained output owner and incremental terminal conversion, unless its exact fields are formally made a frozen wire schema with lifetime/handback proof.
3. **Close scheduler:** delete production whole-drain `while close_step` loops, including `QueryRows::Drop`. Put close progress in mounted persistent cursors that consume one ownership grant per poll, and give ordinary drop a lossless retained-owner handoff rather than completion-by-drop.
4. **R4 readers:** add a production fragment reader to pack and make snapshot consume it directly. Convert index decode cleanup and WAL segment transition to resumable close phases, with ordinary/success/cancel/stale/fault laws.
5. **Acceptance laws/verifier:** extend the P1q verifier and hostile suite with the eight missing laws above. A static string check alone must reject facade-side `drive_task(...).await`, uncredited identifier/result allocations, and unbounded production close drains.

## Final acceptance conclusion

The integrated implementation has meaningful improvements: injected shared memory pool ownership, ledger-backed retained core structures, loss-ring containment, backend retirement, fixed retained state structures, and many preserved hostile fixtures. Those improvements do not meet the exact P1q contract while the real async database operations can execute outside `Lane::Io`, repo-owned live heap remains uncaptured, and normal source paths materialize or synchronously drain retained graphs. **Do not advance P1q to P1w until the bounded packets and their exact laws pass.**
