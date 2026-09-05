# Sync Retained WAL Opportunity-Resume Audit

Read-only audit on 2026-09-05. No product source was changed and no build was run. This reviews the current source after the committed cursor landed and explains the reported retained sync failure at `sync:2590` without treating the eight-millisecond opportunity as a larger operation deadline.

## Decision

Keep the existing 8 ms `WalCursorControl` deadline as a **single caller-owned opportunity**. A cursor result of exact `"wal cursor deadline reached"` or `"wal cursor fuel"` means: retain the cursor/borrowed transaction state, yield once through the sync scheduler, re-check the real sync cancellation/deadline flags, replenish a fresh 8 ms/fuel grant, and retry the same step.

Do **not** retry `"wal cursor cancelled"`, corruption, storage failures, or any other `LimitExceeded`. The former is a real shared cancellation flag; the latter errors are not an opportunity boundary. The sync-wide 30-second deadline remains owned by `DatabaseSyncHelloState.expired`, which is checked before and after every scheduler opportunity.

This directly fixes the observed no-command/aborted-transaction replay case. Two adjacent cursor issues must still be fixed for that contract to be truthful under `fuel = 1`: the current outer cursor step double-charges before opening a segment, and post-validation physical commit skipping has no external fairness boundary.

## Current contract and failure

| Current seam | Verified current behaviour | Consequence |
| --- | --- | --- |
| [`db/📝️wal/🦀️.rs:187`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:187>) | `WalCursorControl::grant` first rejects the shared cancellation flag, then its instant deadline, then spends one fuel unit. `replenish` replaces only deadline/fuel. | Its deadline/fuel are not the sync request deadline and must not become a terminal sync error. A real cancellation remains distinguishable. |
| [`db/📝️wal/🦀️.rs:1612`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1612>) and [`:1774`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1774>) | Raw and committed cursors deliberately leave fuel/cancel/deadline errors non-fatal (`wal_cursor_interrupted` at [`:1803`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1803>)). Their retained decoder, segment pages, grammar spans, and offset therefore remain resumable. | Callers must renew and retry the same cursor—not drop it or start another cursor. |
| [`db/🔄️sync/🦀️.rs:951`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:951>) | `replay_sync_state_retained` already creates a fresh 8 ms opportunity before each outer transaction step and inner record step. But it propagates `records.next_transaction_step().await?` and `transaction.next_record_step()?` directly at [`:977`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:977>) and [`:990`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:990>). | If a backend segment read or bounded retained scan crosses the 8 ms instant, the next grant returns `Unavailable("wal cursor deadline reached")`; it bypasses the existing `Yield` branch and reaches the test’s `unwrap` at [`:2590`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:2590>). |
| [`db/🔄️sync/🦀️.rs:713`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:713>) and [`:719`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:719>) | `database_sync_hello_opportunity` checks `expired` then `cancelled`, yields once, then checks both again. `expired` reports `Timeout`; cancel reports `Closed`. | It is the correct handoff point. It prevents a retry from masking the externally enforced 30-second deadline or a client cancellation. |
| [`db/📝️wal/🦀️.rs:1863`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1863>) | Dropping an unfinished `WalCommittedTransaction` poisons its raw cursor. `close_owner_step` is intentionally ungated. | An inner-step retry must retain the existing borrowed transaction through the opportunity. Returning from the inner loop, or recreating the parent cursor, turns a benign deadline into a fail-stop corruption error. |

The existing native law at [`db/📝️wal/🦀️.rs:2610`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2610>) already establishes the intended distinction: an instant deadline is expected to return the exact cursor deadline error; after replenishment, the same cursor must continue to the neutral transaction result. I did not run it.

## Smallest sync correction

Split the present private `wal_cursor_interrupted` classification into two named predicates in `db_wal`:

```text
opportunity exhausted := exact `wal cursor fuel`
                    or exact `wal cursor deadline reached`
interrupted          := opportunity exhausted or exact `wal cursor cancelled`
```

Expose only the first predicate to sync, rather than re-matching private error strings in another crate. Cancellation must remain in the cursor’s internal fail-stop exemption but **not** in sync’s renewal predicate.

At [`sync:971-1008`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:971>), use this exact control flow:

1. Check `database_sync_hello_control`, replenish the cursor and decoder control with a new `now + 8 ms` opportunity.
2. Call one `next_transaction_step` / `next_record_step`.
3. On `Yield`, call `database_sync_hello_opportunity` and begin the next opportunity as today.
4. On **only** `opportunity exhausted`, call `database_sync_hello_opportunity`, then retry the same outer or inner step. Do not close, drop, finish, or advance an offset/record index.
5. On cancellation/overall expiry/other error, retain the present error-cleanup path. It drains owners without a grant, then returns `Closed` or `Timeout` when `database_sync_hello_control` observes the real flag.

The inner retry must be lexically inside the lifetime of `transaction`. That preserves its gate, `record_index`, raw decoder, and source segment. The sync code can make the retry explicit without a new future type.

### Held `Command` decode is a second retry boundary

After `WalCommittedRecordStep::Record(Command(bytes))`, `transaction` owns a live record until `close_record_step` ([`wal:1817`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1817>) and [`:1844`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1844>)). `database_sync_hello_decode_envelope` can itself exhaust `decode_control`: every retained fragment spends a grant ([`sync:820`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:820>), [`:854`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:854>)).

That routine already closes its transient envelope builder and returns its ledger credit on error ([`sync:881`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:881>)). Therefore the narrow safe rule is a retry loop **around the envelope decode while retaining the same delivered `bytes` borrow**:

```text
decode Command(bytes)
  success                 -> push envelope, hash, then close record normally
  opportunity exhausted   -> scheduler opportunity, replenish decode_control, re-decode same bytes
  other error             -> leave normal record/cursor cleanup path
```

It must not call `next_record_step` again before the delivered record closes: that returns `wal committed body must be closed`; it must not call `close_record_step` on an incomplete decode because that would advance the body index and silently lose the command. Retrying from byte zero is safe here because the builder is unpublished and is explicitly retired before the error leaves `database_sync_hello_decode_envelope`.

## Fuel-one and fairness defects

### P1: first segment cannot open with one unit of fuel

Both raw and committed public steps spend fuel twice before a successful open has an externally observable result:

- `WalReplayCursor::next_validated_step` grants at [`wal:1623`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1623>), then `open_segment` grants again at [`:1581`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1581>).
- `WalCommittedCursor::prepare_transaction_step` does the same at [`wal:1748`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1748>) and then [`:1751`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1751>).

With `replenish(..., 1)`, the first outer grant consumes the only unit; `open_segment` fails before setting pages. The next opportunity repeats the exact same state, so it cannot progress. This contradicts the existing neutral progress law’s `turns < 4096` expectation at [`wal:2617-2646`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2617>).

The minimum coherent repair is phase-owned charging:

1. Remove the preparatory top-level grant.
2. Let `open_segment`, one `WalSegmentChain::step`, one physical-frame parsing step, one retained-record decoder step, or one segment-close step consume its own single grant.
3. Return public `Yield` immediately after a successful phase, especially after `open_segment`; do not spend again to validate in the same public call.

Do not merely catch fuel in sync: it would create a tight schedule/yield/retry loop around a cursor whose state has not changed.

### P1: logical-frame parsing can consume an unbounded number of phase grants

`wal_next_page_frame` grants once before parsing, then once for every CRC fragment ([`wal:1276`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1276>) and [`wal:1094`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1094>)). A 496 KiB legal frame therefore needs multiple fuel units but keeps the source offset unchanged until it returns. It also loops over arbitrary contiguous physical `REC_COMMIT` frames at [`wal:1304-1311`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1304>) without an external `Yield`.

The segment has already passed page-bounded CRC/hash-chain admission through `WalSegmentChain::step` before either raw or committed code reaches this parser ([`wal:1630-1636`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1630>) and [`:1752-1758`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1752>)). The durable repair is a private post-validation physical-frame state that consumes exactly one physical frame per cursor call and emits either `Logical(frame)`, `PhysicalCommitSkipped`, or `SegmentEnd`. The latter two map to public `Yield`; only `Logical(frame)` enters the raw decoder or committed gate. It should validate local bounds/shape but not rescan an already authenticated payload CRC.

This eliminates both the multi-grant frame scan and the unbounded physical-commit loop without weakening frame admission. It also gives one-fuel progress a meaningful definition: every successful call either returns `Yield` after an observable phase/state advance, returns a transaction/record, or returns `Done`.

## Minimal executable-law matrix

1. **Retained aborted sync opportunity retry.** Use the existing neutral `aborted-commands-snapshot-cas-have-no-effects` row. Force the first committed-cursor opportunity to expire after segment I/O but before its next grant; retained replay must yield/retry and return the already asserted zero-command/zero-frontier state. `expired == false`, `cancelled == false`, and ledger returns to zero. This is the reported seventh-law regression.
2. **Overall expiry and client cancellation are terminal, not renewal.** Trigger `expired` respectively `cancelled` between a scheduler yield and the next cursor attempt. Assert `Timeout` respectively `Closed`, no additional storage operation, and terminal owner cleanup. The existing scheduler law at [`sync:3291`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:3291>) covers part of the cancellation ordering; extend it through a committed cursor.
3. **Command decode retry keeps its live record.** A committed Command spans multiple retained pages. Force `decode_control` exhaustion after a builder field and again after a payload fragment. Each retry starts from the retained bytes, ledger never exceeds its bound, exactly one envelope is published, and only then may `close_record_step` increment the index.
4. **Fuel one opens, validates, skips physical commit, and closes.** Replenish only one unit per turn from an unopened cursor through a segment header, an empty committed transaction, a physical SPR commit, end-of-segment, and terminal close. Every non-error call must advance a named cursor phase or return `Yield`/transaction/`Done`; cap turns. Run this against both raw and committed cursors.
5. **Physical-frame fairness.** A legal segment with many consecutive `REC_COMMIT` frames between logical records returns a public `Yield` for each bounded physical-frame step even with ample fuel. It must not monopolize one 8 ms worker opportunity, and final logical output/offset must still match the neutral fixture.

## Nonclaims

This audit did not run the reported native law or any build. It does not claim a production replay success. It does not change WAL recovery semantics, the committed grammar, or the actual 30-second hello deadline; it identifies the narrow sync caller boundary needed to use the already-resumable cursor correctly.

## Update — landed opportunity repair review

Read-only review of the current source after the frame-scanner and sync retry changes. No build was run here; the parent-reported native run remains the only execution evidence.

### What is now correct

The two earlier structural objections are addressed without weakening chain admission:

1. `WalCursorControl::check` at [`db/📝️wal/🦀️.rs:210`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:210>) checks cancellation/deadline without spending fuel. Both raw and committed entrypoints use it before selecting exactly one phase; opening a segment, one chain step, one verified-frame step, one retained decoder step, or one close step supplies the sole grant. Successful segment open returns public `Yield` immediately ([`wal:1628-1664`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1628>) and [`:1754-1778`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1754>)). The previous fuel-one unopened-segment livelock is removed.
2. `WalVerifiedFrameStep` parses one post-admission physical frame and returns `Frame`, `PhysicalCommit`, or `Done` ([`wal:1281-1319`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1281>)). It intentionally omits a second CRC scan only after `WalSegmentChain` has authenticated the immutable `DbIoPages` in full. Raw replay cannot call it until validation cleared ([`wal:1636-1656`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1636>)); committed replay has the equivalent fence ([`:1759-1777`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1759>)); recovery and the neutral oracle first run the same chain scanner ([`:1898-1916`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1898>). No source path found calls this post-admission parser on unverified pages.
3. A logical frame still leaves the raw offset at its start until its consumer accepts it: raw replay first retains a decoder and advances only after it returns a record; committed replay advances only after `WalTransactionGate::push` succeeds ([`wal:1651-1663`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1651>) and [`:1774-1778`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1774>)). A skipped physical commit is the only frame that moves the offset before public `Yield`, and it has already passed exact chain/CRC/commit validation.
4. The new sync helper recognizes only exact fuel/deadline exhaustion ([`db/🔄️sync/🦀️.rs:729-744`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:729>)). It yields and checks the true `expired`/`cancelled` flags before replenishing. The outer replay and live borrowed transaction retry exactly those outcomes but continue to surface cancellation and all structural/storage errors ([`sync:992-1032`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:992>)). This is the correct no-widen/no-mask boundary.
5. `WalBytesCursor::varint` now rolls its offset back on every error, while a field-fragment read grants before it mutates its cursor/remaining count. The helper is used only for those atomic operations in the current sync decoder ([`sync:837-945`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:837>)). Thus a deadline/fuel retry does not duplicate an output fragment or lose a byte. If overall cancel/expiry wins during its scheduler yield, the existing builder-close path releases every credited owner before error return.

I found no new chain, transaction-grammar, or retained-owner regression in this current path. In particular, `WalTransactionGate` still receives a body span only after full chain admission and still defers decoded bodies to the committed borrow; `TxAbort` only clears address spans ([`wal:1341-1395`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:1341>)).

### Remaining test gaps (actionable, not a request to widen the turn)

1. **P1 — the new sync varint law does not force a partial, fuel-spent read.** [`sync:2598-2622`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:2598>) begins each neutral varint attempt with an already-expired instant. That proves renewal before byte zero, but it does not prove the helper handles the important `fuel = 1` failure after the first byte of a two-or-more-byte varint. Add one known multi-byte fixture row with a future deadline/fuel one, assert exactly two read attempts, byte-offset rollback before retry, exact final value/consumed count, and a finite attempt cap. The present `attempts >= 2` has no cap, so this native law can hang rather than diagnose a replay-livelock regression.
2. **P1 — no deterministic retained-sync cursor-expiry integration seam.** The repaired abort law at [`sync:2624-2648`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs:2624>) exercises real timing, which previously exposed the bug but cannot deterministically prove the outer `Err(turn_exhausted)` branch. Add a test-only turn source or a small private drive helper that expires the committed cursor after segment open/one chain step, then prove retained aborted replay resumes with zero effects and zero ledger credit. Do not make production timing longer.
3. **P2 — physical-frame fairness is indirectly, not directly, asserted.** The neutral fixture contains many physical commits, but `wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions` only checks final output and a broad turn cap ([`wal:2629-2665`](</Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:2629>)). A future change could scan several `REC_COMMIT`s in one call and still pass. Add an exact row with N consecutive physical commits and assert N externally observable `Yield`s / monotonically advanced source offsets before the next logical span. Mirror it for raw replay so the raw `PhysicalCommit` branch is covered too.
4. **P2 — cancellation/expiry test closures do not prove no operation occurs.** The new overall-flag checks call `database_sync_hello_read(..., |_| Ok(()))`; they establish returned error type but not that the closure was not invoked. Give that closure a counter and assert zero calls. Add the stronger variant that flips `expired` or `cancelled` between the first exact turn-exhausted result and the resume poll, then assert the helper neither replenishes nor makes its second source read.
5. **P2 — the exact exhaustion classifier is duplicated privately.** `sync:729-731` re-matches `db_wal`’s private control text rather than reusing a named WAL predicate. Current messages match, so this is not a runtime bug. A small exported `is_opportunity_exhausted(&DbError)` would prevent future drift where cursor fail-stop logic and sync renewal disagree; it must exclude cancellation exactly as this implementation does.

The private `database_sync_hello_read` is safe for its current three atomic closures only. Its contract should stay narrow: do not reuse it for an operation that mutates an owner before returning a turn-exhausted error, or its generic retry would livelock/duplicate that mutation. The current calls satisfy the condition.
