# Independent P1z Sync-Hello Source/Static Acceptance Audit

Date: 2026-08-24  
Auditor: Codex, independent read-only source/static review  
Verdict: **RED — P1z must not be accepted.**

## Scope

Read completely: repository `AGENTS.md`; the governing Phase-1 ticket/attachment; the P1z caller census and implementation handoff; P1q/P1w/P1x/P1y boundary reports; the live engine, sync, WAL, hub, async-pool and root P1z verifier sources. No production source or verifier was edited. No Cargo, Nx, build, Wasm, browser, or runtime Rust test was run.

The selected cutover is real: `Database::hello` has no `db_actor::block_on`, mounts `DatabaseSyncHelloFuture`, uses `Lane::Io`, and the hub takes `Welcome` once then sends one `next_frame` result at a time. Those facts do not establish the retained-owner contract below.

## Blocking Source Counterexamples

### Z1 — Decode And Clone Backings Are Allocated Before Credit, And Several Are Never Charged

The input ledger records the owners once at sync `component.rs:810-812`. It then creates a second dynamic document owner before any debit at `:814`, a second session owner at `:878`, and tail origin/frontier clones at `:850`. None has a matching `ledger.observe`.

More directly, each WAL command decoder allocates every text, dependency vector, and two payload vectors at sync `component.rs:57-97`. The lower cursor uses `Vec::with_capacity(remaining)` for every text field at WAL `component.rs:368-376`. Only after all of those allocations does P1z calculate the envelope capacity and debit the ledger at sync `component.rs:639-646`.

Thus a command with two individually permitted large payload fields can allocate both real backings before cumulative refusal happens; a high-capacity accepted input can additionally be duplicated by the uncharged clones. This is the forbidden after-allocation accounting, not pre-admission of every backing capacity.

### Z2 — Cancellation Can Be Accepted Between The Yield Check And First WAL/Backend Work

`database_sync_hello_opportunity` checks `cancelled` only before its yield and returns success without a post-yield recheck at sync `component.rs:584-590`. The first execution opportunity yields at `:813`, and the resumed future immediately mounts retained WAL replay at `:814`; replay opens the WAL backend at `:616`.

Permitted trace: the first driver poll observes `cancelled == false` and returns `Pending` from `yield_once`; another thread calls `DatabaseSyncHelloFuture::cancel` (`:1227-1231`); the next I/O poll completes `yield_once` and begins `replay_sync_state_retained` without rechecking the atomic. A cancellation accepted before WAL replay therefore does not prevent the backend/WAL path. The existing handoff law only freezes cancellation before the first driver release and does not cover this interval.

### Z3 — Panic Quarantine And Close Still Permit An Implicit Dynamic Future Drop

The driver catches an execution panic and retains the unwound boxed future in `quarantined` at sync `component.rs:1030-1037`. The governed close cursor contains no quarantine close step; when ordinary owners happen to be exhausted it assigns `core.quarantined = None` at `:1131-1135`. That drops the dynamic boxed future in one unretained operation, bypassing one-backing-at-a-time retirement. The same close path only reads `prepared.ledger.items` (`:1117-1119`); it does not prove that the byte ledger is zero or return all ledger debit before releasing admission.

### Z4 — Refusal Close Retry Is Infinite And Has No Exhaustion Terminal

The ordinary driver carries a retry attempt and bounds it. `DatabaseSyncHelloRejectedClose` does not: a saturated refusal stores its job then schedules `callback_at` at sync `component.rs:1426-1433`, and every failed retry repeats the same unbounded callback recursion at `:1437-1447`. There is no retry counter, deadline, cancellation branch, quarantine, or terminal close witness. This fails the specified refusal/retry-exhaustion close obligation.

## Verifier Assessment And Faithful Mutation Reproduction

`bun ./📜️script.ts verify interactivity p1z` did execute the root verifier's actual baseline and its bound source-string mutation corpus; it reported green. It is nevertheless false-green for Z1-Z4. The P1z predicate checks only generic ledger tokens and the existence of `database_sync_hello_envelope_credit` at `📜️script.ts:11144-11159`; it never connects a debit to the allocation site.

I independently performed the exact in-memory, single-binding mutation replacing only `ledger.observe(items, bytes_used, "database sync hello cumulative envelope backing")?;` with `let _ = (items, bytes_used);`. It bound once, left every predicate-required generic ledger and envelope-credit token intact, and neither the P1z predicate nor its self-mutation list mentions the removed expression. Therefore its test would still accept a live decoder that never charges completed envelopes; this is source-faithful mutation evidence rather than a count claim.

The verifier likewise has no mutation for the post-yield cancellation recheck, uncharged clones, quarantine cursor, ledger-zero release, or refusal retry bound. Its existing panic and pre-poll-cancel replacements cannot prove any of these transitions.

## Gates Executed

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1z` | PASS — false-green against Z1-Z4 |
| preserved `p1y`, `p1x`, `p1w`, `p1q-b1-b6` static verifiers | PASS |
| scoped `rustfmt --edition 2021 --check --config skip_children=true` on sync, engine, WAL, hub | PASS |
| scoped `git diff --check` | PASS |
| engine/sync/WAL/hub `block_on`, eager-output, lane, callback and caller census | selected production cutover present; no new selected blocking bridge found |

## Required Closure

Debit/reserve each dynamic allocation before it happens and move rather than clone owned input where possible; account every remaining clone against the same ledger. Recheck cancellation and deadline after every cooperative yield before the next backend/WAL operation. Give panic quarantine its own incremental close cursor and prove the item and byte ledger reaches zero before registry/admission release. Replace refusal self-rescheduling with a bounded retained retry owner that closes exactly on cancellation, deadline, or exhaustion. Add branch-local hostile mutations and deterministic interleaving laws for each path.

