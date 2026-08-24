# P1z Deadline/Cap Final Source/Static Audit

Date: 2026-08-24  
Auditor: Codex, fresh independent read-only source/static audit  
Verdict: **RED — do not accept P1z.**

## Scope And Method

Read completely: repository `AGENTS.md`; the earlier final P1z RED report; the P1z caller census/contract; Sol's updated P1z report; the live retained sync source, engine facade, hub caller, WAL cursor, and root P1z verifier. Inspected the preserved P1y/P1x/P1w/P1q gates. No production source or verifier was edited.

## Positive Findings

- `DatabaseSyncHelloFollowUp::drive_one` creates a fresh checked monotonic `Instant::now().checked_add(Duration::from_millis(8))` grant at `sync/component.rs:977-1002`.
- The production snapshot branch caps the requested chunk at `DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES`; it has control checks immediately before allocation, page access, copy, and the sequence/owner publication transfer at `:1033-1062`.
- `database_sync_hello_control` returns deadline `Timeout` before testing cancellation, so `expired = true, cancelled = false` is independently terminal at `:656-663`. `deadline_callback` stores `expired`, then `cancelled`, then schedules at `:1694-1700`.
- The hub takes and acknowledges exactly one welcome/frame lease after its matching send at `hub/bin.rs:775-823`. Returned-frame close retains the generation and releases its stored debit only after the mounted close is empty at `sync/component.rs:1475-1507`.
- The maximum-request law enters the public `DatabaseSyncHelloFuture::try_submit` route and its forced-expiry law calls the shared production stream body `drive_one_with_grant`; both have the expected source evidence at `sync/component.rs:3047-3106`.

## Blocking Counterexample: The Snapshot Allocation Is Not Pre-Debited To The Fixed Unit

The fixed-unit claim is false in the live production allocator. The snapshot path passes a 4 KiB-or-smaller `len` to the shared allocator (`sync/component.rs:1033-1038`), but that allocator does **not** debit `len` before allocating:

1. `DatabaseSyncHelloBackingLedger::reserve_allocation` computes `reserved` as **all remaining** P1z byte credit: `DATABASE_SYNC_HELLO_MAX_BYTES - self.bytes` (`:423-429`).
2. `database_sync_hello_allocate_vec` passes the snapshot request to that routine (`:456-463`), so it pre-debits that entire remaining amount before `try_reserve_exact`.
3. It only settles down to the allocator-observed capacity after allocation (`:467-473`). Its sole cap is `actual <= reserved`, not `actual <= DATABASE_SYNC_HELLO_FRAME_UNIT_BYTES`.

Thus a first snapshot frame produced for a caller request of 256 MiB has a 4 KiB logical copy limit, but it transiently consumes almost the full 256 MiB ledger rather than the claimed fixed 4 KiB debit. Moreover, the returned `Vec` capacity is statically permitted to exceed 4 KiB: `try_reserve_exact(4096)` is checked only against the remaining 256 MiB reservation. The source therefore does not establish the required actual 4 KiB allocation/credit/returned-frame unit on every supported allocator.

This is not a benign accounting detail. It contradicts the P1z contract's specific claim that “Only that fixed unit is pre-debited,” can spuriously exhaust the authority during the allocation window, and makes the lease debit depend on an allocator capacity that has no fixed-unit upper bound.

## Static-Verifier False Green

`bun ./📜️script.ts verify interactivity p1z` reports its live source and hostile corpus clean, including the final fresh-clock, monotonic-deadline, deadline-check, 4 KiB cap, unit-predebit, initial/allocation/page/copy/publication checkpoints, independent-expiry, callback-dataflow, law, and contract mutations.

That corpus does not inspect the implementation of `database_sync_hello_allocate_vec` or assert that its reservation/observed capacity is bounded by the snapshot unit. The relevant predicate only requires the snapshot-local text `let len = unit_bytes.min(...)` and the checkpoint tokens (`📜️script.ts:11241-11245`); its “snapshot-unit-predebit-removed” mutation changes that local `len` expression alone (`:11317-11324`). The max-request law's source evidence requires a runtime capacity assertion, but the static verifier merely checks the law's tokens (`:11265-11287`).

Consequently, the verifier cannot distinguish the present all-remaining-credit reservation from a fixed-unit reservation and is false-green for this concrete requirement. This is independent of the earlier missing-grant defect, which the present source does correct.

## Checks Executed

| Check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1z` | PASS — static corpus clean; false-green counterexample above |
| `bun ./📜️script.ts verify interactivity p1y` | PASS |
| `bun ./📜️script.ts verify interactivity p1x` | PASS |
| `bun ./📜️script.ts verify interactivity p1w` | PASS |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS |
| Scoped `rustfmt --edition 2021 --check --config skip_children=true` on sync/engine/WAL/hub | PASS |
| Scoped `git diff --check` on sync/hub/root verifier | PASS |

No Cargo, Nx, build, Wasm, browser, or runtime Rust test was run.

## Required Closure

Use a snapshot-specific allocation/credit primitive that reserves the maximum actual capacity allowed for this frame unit (and rejects/retains on any larger allocation) before allocation; do not reserve the whole remaining P1z budget. Keep the exact observed unit debit with the returned-frame lease. Add a runtime law that asserts the ledger debit before allocation is no greater than the unit and that acknowledgement/terminal close returns it to zero. Extend the P1z verifier with mutations of the shared allocator reservation and its actual-capacity bound, not only the snapshot-local `len` expression.
