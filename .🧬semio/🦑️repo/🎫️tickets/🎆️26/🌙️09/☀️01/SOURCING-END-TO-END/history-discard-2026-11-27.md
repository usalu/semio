# History discard receipt (2026-11-27)

## Root cause
`apply_time_travel_event` (`🔌️plugin/⏪️time-travel/🧵️instance/🎬️action-dispatch-and-effects/🦀️.rs`) staged the Discard command receipt under `mounted_policy.maintenance` (64 items) regardless of the receiving turn. The reactor check in `collect_original_publication_receipts` (`⚛️reactor/🔄️turn/🩹️reconcile-drive-and-publication-receipts/🦀️.rs`) compares the History receipt grant to `cx.retained_grant()` (remaining authority), so a turn with less than the policy left was refused: "exceeds its producer or remaining caller authority". Every other drive path uses `original_plugin_turn_grant(maintenance, remaining)`.

## Fixes
- Discard staging grant = `original_plugin_turn_grant(maintenance, cx.retained_grant())` when a receiving context exists.
- The refusal message now names family, grant, remaining and progress (permanent diagnostic).
- Journey (`journey-history.ts`): `beginEdit` no longer toggles an already expanded mutation row (the second Edit after discard); "back" presses Escape because the band Back button sits under the finish dialog veil (dialog cancel is bound to historyEditBack).

## Native test
`a_discard_staging_receipt_never_exceeds_the_remaining_caller_authority` in `🧪️tests/🧪️time-travel-part-2/🗑️retirement-custody-and-transaction-rows/🦀️.rs` (maintenance policy richer than the caller). NOT RUN: `semio-plugin-tests-14-2` does not compile today (86 errors in peers' in-flight modules: builder-contract-part-4, reactor-driver, API drift in turn/app); none in this file.

## Results (live 6081, hub wasm rebuilt with the fix, staged, server not restarted by me)
history edit steps, 10 per language: en 10/10, de 10/10 in the last full run. One earlier en-then-de full run had de accept/discard fail with `selected preview retains its original base cleanup issuers` (UnsupportedOwner, `⏪️time-travel/♻️retirement/🦀️.rs:154`, issuers None while preview_base is Some); not reproduced in 3 later runs, so open and flaky.
Other history failure (revert) belongs to the revert agent.
Target dir deleted. Script added: `discard-wasm-build.sh` (build only, no restart).
