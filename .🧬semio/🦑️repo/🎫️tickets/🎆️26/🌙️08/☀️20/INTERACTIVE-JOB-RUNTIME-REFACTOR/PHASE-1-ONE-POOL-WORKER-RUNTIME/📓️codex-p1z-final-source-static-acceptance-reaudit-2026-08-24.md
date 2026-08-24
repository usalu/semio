# P1z Final Sync-Hello Source/Static Acceptance Re-Audit

Date: 2026-08-24  
Auditor: Codex, independent read-only source/static audit  
Verdict: **RED — do not accept P1z.**

## Scope

Read completely: repository `AGENTS.md`; the P1z caller census; the preceding P1z RED report; Sol's remediation report; live sync, engine, WAL, hub, async worker-loop, root verifier, and preserved P1q/P1w/P1x/P1y boundaries. No production source or verifier was changed. This report is the sole audit artifact created.

## Positive Findings

- The selected facade is `Database::hello`: it mounts `DatabaseSyncHelloFuture`, awaits its terminal witness, and has no selected `db_actor::block_on` ([engine](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:7390)).
- The hub takes one returned `Welcome`, sends it, acknowledges it, then awaits, sends, and acknowledges one returned follow-up frame at a time ([hub](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:775)).
- Returned-frame leases are generation-qualified, retain their exact chunk debit through acknowledgement/Drop close, reject a next demand while live, and retain a fallback owner for failed/stale mounting ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1829)).
- The production decoder pre-debits before `try_reserve_exact`, and the production retained region rejects admitted input-field clones while requiring the three `mem::take` transfers. No ignored `database_sync_hello_control` result remains.
- Snapshot page-close errors are returned as `Result`, retained as `DatabaseSyncHelloFollowUpCloseFault`, and terminal registry/admission release follows execution teardown ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1275), [sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1553)).
- The refusal path installs a generation-qualified public registry synchronously, atomically caps submissions at eight, retries through `Lane::Io`, retires released registry heads one callback at a time, and makes the held sole-worker case ownership/discoverability-only ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:2055), [sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:2112)).

## Blocking Counterexample: Snapshot Opportunities Have No Actual Eight-Millisecond Deadline

P1z claims an actual eight-millisecond grant. The only `DATABASE_SYNC_HELLO_TURN_MS` deadlines are installed in the retained WAL replay cursor ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:893), [sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:906)). `DatabaseSyncHelloFollowUp::drive_one` has no `Instant` or 8-ms control. It accepts a caller-controlled `snapshot_chunk_bytes` with only a zero check ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1099)); the public retained facade exposes that value unchanged ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1672)).

On the first snapshot demand, the one worker opportunity reserves and allocates `min(snapshot_chunk_bytes, remaining)` bytes before copying a page fragment ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:989)). A client may therefore request up to the remaining 256-MiB ledger capacity in one allocation. There is no wall-clock check before or after that allocation/copy/transfer, and no cap tying the allocation to a page-sized bounded unit. The supplied `cancelled`/`expired` helper only tests `cancelled`; `expired` merely chooses the error variant after cancellation has already been observed ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:655)).

The later 30-second pool timer does not provide an 8-ms per-opportunity deadline ([sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1696), [sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs:1719)). Thus cancellation/deadline is not actually enforced immediately before the full pre-allocation/transfer opportunity as required. This is a live production-path counterexample, independent of the repaired returned-frame accounting.

## Verifier Assessment

`bun ./📜️script.ts verify interactivity p1z` passed and reports its complete listed hostile-mutation corpus clean. That reproduces all permanent P1z mutations, including lease-install/next-demand/credit-return/fallback/Drop/hub-ack, additive admitted-origin clone, predebit, post-yield and stream control, page-close fault, terminal ordering, retry/no-ninth-submit/registry-drain, and contract mutations.

It is nevertheless false-green for the blocker. The P1z predicate only requires the literal `const DATABASE_SYNC_HELLO_TURN_MS: u64 = 8`, WAL `replenish` tokens, and three stream calls to `database_sync_hello_control`; it does not require an `Instant` deadline in `drive_one`, a bounded snapshot allocation, or a deadline-callback-to-control dataflow ([verifier](/Users/ueli/Documents/semio/📜️script.ts:11218), [verifier](/Users/ueli/Documents/semio/📜️script.ts:11238)).

I also bound a single in-memory mutation solely inside `deadline_callback`, replacing its `self.cancelled.store(true, ...)` with `false`. The mutation bound exactly once. All verifier-relevant generic `expired`, `cancelled`, and stream-control tokens remain because other paths contain them, while `database_sync_hello_control` then returns `Ok(())` after a deadline callback. The verifier has no deadline-callback mutation; its deadline law checks only tokens such as `deadline_callback` and `expired.load` ([verifier](/Users/ueli/Documents/semio/📜️script.ts:11273)). This is a distinct static false-green, not a source change.

## Executed Checks

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1z` | PASS — complete listed self-mutation corpus; false-green for the deadline blocker |
| preserved `p1y`, `p1x`, `p1w`, `p1q-b1-b6` static verifiers | PASS |
| scoped `rustfmt --edition 2021 --check --config skip_children=true` on sync/WAL/engine/hub | PASS |
| scoped `git diff --check` | PASS |

No Cargo, Nx, Wasm, browser, or runtime Rust test was run.

## Required Closure

Give every snapshot frame-production grant its own real 8-ms monotonic deadline and bounded allocation unit before `try_reserve_exact`; recheck that control immediately before allocation, page copy/transfer, and publication. Make `database_sync_hello_control` reject an independently expired deadline. Add runtime laws for a maximum-sized requested chunk and deadline firing between allocation and transfer. Extend the P1z verifier with mutations that disconnect the deadline callback from cancellation/control and remove the follow-up's per-grant clock/cap; the old token-only checks are insufficient.
