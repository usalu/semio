# Receiving Scope Refund

## Cause

A decoder owner holds one pooled Source escrow row for all its receiving scopes (`OriginalReturnScope`, `📦️stock/🎟️scope`). The row is claimed with the whole offered share (`unquoted_receiving_grant`: half the turn quantum, capped at 16 MiB copy, 1 Mi items, 64 MiB release, 1 Mi work; capacity uncapped). `OriginalJobDecodeState::run_scoped_on_caller` (`🛬️receiving/🎟️state/🦀️.rs`) settled every call with `finish_scope(.., release_unused = false, ..)`. Only the owner's final retirement (`release_original_scope`) released the unspent hold, so every live owner kept its whole share out of the Source total (copy, capacity, release, items and fuel). With the host stock (504 MiB copy / 512 MiB release) a few resident owners exhaust the Source although the actual spend is tiny: the measured 16,786,730 B copy / 134,208,200 B capacity / 67,108,864 B release / 1,048,602 items against 83,948 B spent is exactly one unreleased share.

## Fix

`run_scoped_on_caller` now passes `release_unused = idle`, where `idle = !plan.recovering_pending && !has_pending_receiving() && pending_work() == 0`. An idle scope refunds its unspent items, copy, capacity, release and fuel in the same settle (`settle_stock` already moves held to released and drops the claim); the next scope re-claims and pays only its control fees. Scopes with pending native custody (recovery or unresolved work reservations) keep their hold, and rows that must be retired (`retains_output` or retained capacity) keep it until their credit settles, as before.

## Law

`⏱️context/🧪️tests/📊️native-progress/receiving-refund.rs` (`idle_unquoted_receiving_scopes_leave_no_held_share_on_the_source`), fixture `🧫️fixtures/receiving-refund.json` plus schema, TS twin in `🟦️.ts` (ajv corpus check plus SQLite cross-check; bun is not installed on this machine, so the twin was not executed here). It runs 8 unquoted scopes with a checkpoint-only body on one live Source and asserts, after every settle, that the Source budget snapshot holds nothing (items, copy, capacity, release, fuel) and no live claim. Red without the fix (`scope 1 kept its unspent share`), green with it.

## Cleanup

Removed the `[DEBUG]`/`crate_dbg` prints from `🛬️owned/🦀️.rs` (headroom, admit_loan, nine `crate_dbg` tags and the helper) and from `🧪️testing/🎟️caller/🦀️.rs` (refreshed, table_birth, borrow, service, run, drop). Another agent re-added `semio_framework_trace::debug_tag`/`debug_turn` calls during this work (also in `operation-wire/🧳️replacement`); I removed them from the two files above where they broke the build and left the rest to their owner.

## Verification

- `semio-framework-job --features testkit --no-fail-fast`: every target green (`original-native-progress` 10 of 10 with the new law).
