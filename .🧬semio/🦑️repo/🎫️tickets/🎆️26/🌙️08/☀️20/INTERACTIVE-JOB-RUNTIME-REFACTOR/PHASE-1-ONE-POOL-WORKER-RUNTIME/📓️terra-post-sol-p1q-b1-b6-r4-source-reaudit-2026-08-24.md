# P1q Post-Sol B1–B6 and R4 Source Re-Audit

Date: 2026-08-24  
Auditor: Terra, independent read-only acceptance re-audit  
Disposition: **RED — do not advance P1q to P1w**

## Scope and method

I read the master plan, P1q repair contract, R4 packet/census, prior Terra RED audit, and `📓️sol-high-integrated-p1q-b1-b6-r4-remediation-2026-08-24.md`. I then re-inspected the live integrated source, including the storage core, PostgreSQL/Neo4j facades, snapshot/index/WAL/state/artifact/query/engine/compact/CLI/pack paths, and the isolated verifier implementation. This report is not based on the remediation claims.

No Cargo, Nx, build, runtime, network, or ticket command was run. This report is the only mutation made by this audit.

## Checks actually run

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | Pass: `live-source and hostile mutations clean` |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` on the 15 remediated Rust packets | Pass; no output |
| Scoped current and cached `git diff --check` | Pass; no output |
| Scoped current/cached name-status for remediated P1q paths | Current changes are modifications only; no `D`. No cached change was reported. |
| Independent source predicate / ownership trace | **Fail**; live production counterexamples below |

The verifier pass is real but insufficient: its R4 bulk-close predicates only match `while … close_step`, so they do not see the production `loop { match close_step … }` or `for … { close_step }` forms below. It also does not inspect the compactor's retained `Vec<Page>` working set.

## B1–B6 verdict

| Gate | Verdict | Re-audit evidence |
| --- | --- | --- |
| B1 — actual facade driver polling uses shared `Lane::Io` | PASS, source-only | PostgreSQL and Neo4j `execute` submit to the injected `WorkerPool`, call `start_async_native_on_lane_io`, then await the terminal owner. Storage retains `DbIoAsyncDriverFuture` in the task slot and `db_io_poll_async_driver` is submitted through `Lane::Io`. The old facade-side `drive_task(...).await` is absent. |
| B2 — actual artifact/lease/driver working-set ownership is ledgered | PASS, source-only | `DbIoArtifactId::try_from_text` pre-reserves and observes the constructed `ArtifactId` capacity. `LeaseInfo` is now the retained `DbIoLeaseResult` alias, with fixed `DbIoText` fields and result handback. PostgreSQL/Neo4j use `with_admitted_artifact!`; the old raw `ArtifactId(document.as_str().to_string())` and raw `LeaseInfo` reconstruction are absent. |
| B3 — core result/cancel/drop/close handback | PASS, source-only | `DbIoLeaseResult::close_step` advances one holder/resource/handback opportunity and its populated Drop parks the exact retained owner. The task result lease attaches the same result handback before transfer. |
| B4 — saturation/loss/panic containment | PASS, source-only | Fixed lost-owner ring, permanent retained-fault witness on saturation, typed cancellation/fault state, and named hostile coverage remain present. |
| B5 — real backend close reachability | PASS, source-only | Registered backend executor ownership and mounted close witnesses remain. PostgreSQL's close future reaches `PgPool::is_closed`; the PostgreSQL/Neo4j lost-facade laws remain. |
| B6 — hostile laws and verifier prove every exact live law | **RED** | The named new laws exist, but the verifier accepts live production terminal sweeps and a retained dynamic page-owner graph. Therefore coverage is not one-to-one with the R4 contract. |

The B1–B5 entries are source-only results, not runtime claims. The overall disposition is RED because B6/R4 are required gates.

## R4 verdict

| Packet | Verdict | Evidence |
| --- | --- | --- |
| Pack / snapshot page read | PASS, source-only | Production `SnapshotChainCursor::read_page` uses `PackIdentityChunkCursor::read_fragment` into an admitted page writer. The `PackFile::read_chunk -> Vec<u8>` snapshot helper is `#[cfg(test)]`. |
| Index / WAL close phase | PASS, source-only | `decode_run_pages` advances one page close then losslessly hands the remainder to retained page retirement; `WalReplayCursor::next_step` returns `WalReplayStep::Yield` after each segment-close opportunity. |
| State module | PASS, source-only | `StateEntry` retains a fixed text key plus page owner and closes one nested opportunity at a time. |
| Query / engine ordinary Drop | PASS, source-only | `QueryRows` and engine `QueryStream` now move unfinished fixed owners into fixed mounted retirement rings; their Drops do not loop. |
| Artifact state failure close | **RED** | `DocumentState::apply_entries` closes every already-staged `StateEntry` in one ordinary rejection path. |
| Compaction | **RED** | Snapshot consolidation accumulates all retained `db_state::Page` owners in `Vec<db_state::Page>` before publication. `Page` embeds `DbIoPages`; this is an uncensused whole working-set graph, not a frozen schema field or a single exact output writer. |
| CLI | **RED** | Normal CLI command paths synchronously loop each `WalRecord`/`WalReplayCursor`/snapshot cursor/record batch to terminal instead of retaining/mounting one close opportunity. |

## Blocking counterexamples

### R4-A: artifact rejection bulk-closes up to 64 live retained entries

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs:446` handles a failed `StateEntry::try_admit` as follows:

```rust
control.grant()?;
let _ = rejected.close_step()?;
for entry in staged.iter_mut().flatten() {
    let _ = entry.close_step()?;
}
return Err(...);
```

`staged` is a 64-slot retained owner array. `StateEntry::close_step` can retire one page owner or its key; executing it for every staged entry makes one ordinary error call drain many separate owners. There is one grant before the whole sweep, no persistent close cursor, no one-owner return/yield, and no mounted handoff for the unprocessed staging set. This violates the exact fault/cancellation/close rule.

### R4-B: CLI still completes retained close synchronously

Production code, not the test module (which begins at line 1181), contains terminal drain loops:

- `⌨️cli/🦀️component.rs:431–449`: `cmd_wal_inspect` loops every record's `close_step` and then the replay cursor's `close_step` until false.
- `⌨️cli/🦀️component.rs:529–536`: snapshot inspection loops `SnapshotChainCursor::close_step` to terminal.
- `⌨️cli/🦀️component.rs:730–743`: replay command drains each record and the retained replay cursor.
- `⌨️cli/🦀️component.rs:1004–1010`: migration drains `WalRecordBatch` to terminal after submit.

The repair contract prohibits production callers synchronously draining terminal state. Calling these `loop` forms rather than `while` does not make them one-grant state machines; ordinary, success, and error exits remain non-resumable full drains.

### R4-C: compaction materializes an uncensused retained page graph

`🗜️compact/🦀️component.rs:254–283` returns `Result<(SnapshotDescriptor, Vec<db_state::Page>), DbError>`. It owns a `HashSet` plus a growing `Vec`, iterates every generation and `descriptor.new_pages`, calls `cursor.read_page`, and pushes each retained `db_state::Page`. `db_state::Page` contains a `DbIoPages` owner (`🔘️state/🦀️component.rs:83–85`). `SnapshotConsolidator::consolidate` passes the full vector to `publish` at compact line 311.

This is not the permitted frozen `SnapshotDescriptor` metadata vector and not a pre-admitted single `DbIoPageWriter`: it is a dynamic, complete repository page-owner working set with no fixed owner array, aggregate credit, close cursor, or lossless mounted partial handback. It violates retained streaming/working-set admission independently of the CLI and artifact sweeps.

### R4-D: the verifier has a semantic blind spot

`📜️script.ts` `interactivityP1qR4Failures` only rejects `/while[^\\n{]*close_step/` in artifact, compact, and CLI source. Consequently the CLI `loop { match …close_step… }` drains and artifact `for entry … entry.close_step()` sweep pass the verifier. The function also does not receive the state or compactor page-owner type as a predicate and has no mutation that restores `Vec<db_state::Page>`.

The successful isolated verifier therefore confirms its present string/mutation set, but not the exact ordinary/fault close and retained-working-set law it is intended to witness.

## Preserved and missing law coverage

No remediated in-scope test/module deletion was observed in current or cached status. The new async-worker, artifact/lease, identity-cursor, WAL-yield, query-drop, and engine-drop fixture symbols are present, as are prior hostile B1–B6 fixtures.

Required laws still missing from the effective acceptance map are:

1. Artifact state-admission rejection with N staged entries consumes at most one close opportunity and yields/mounts the remainder.
2. Each CLI ordinary-success/fault/cancellation close path hands off one retained record/cursor/batch opportunity rather than looping to terminal.
3. Snapshot consolidation rejects or streams a chain exceeding its fixed retained owner capacity, including partial close/drop/cancel and exact ledger return.
4. The root verifier must kill `loop` and `for` close sweeps and dynamic `Vec<Page>` restoration, rather than only matching `while` text.

## Bounded repair packets

1. Replace artifact's staged-error sweep with a retained fixed staging-close cursor. On the first rejected entry, persist the staged array and return/yield after one `StateEntry::close_step`; ordinary Drop must mount the same cursor.
2. Change CLI commands to explicitly advance one close step per scheduled/batch close opportunity or transfer unfinished owner state to the existing retained maintenance authority. Do not use `loop`/`while` terminal drains in production CLI paths.
3. Replace compaction's `Vec<db_state::Page>`/`HashSet` collector with a fixed, admitted streaming consolidator that writes page fragments/pages into an exact output authority and retains a resumable dedup/traversal cursor. If full-chain consolidation needs more fixed capacity than one operation, make that capacity/operation split explicit rather than using a dynamic graph.
4. Extend verifier source predicates and self-mutations for all three forms above, then add corresponding hostile laws for ordinary, success, fault, cancel, stale, and Drop handback.

## Final conclusion

Sol's remediation resolves the prior core facade/ArtifactId/LeaseInfo and principal snapshot/index/WAL/query/engine findings at source level. The live artifact, compactor, and CLI paths nevertheless violate the required one-owner-at-a-time terminal and retained-streaming contract, and the passing verifier does not detect those forms. **P1q remains RED.**
