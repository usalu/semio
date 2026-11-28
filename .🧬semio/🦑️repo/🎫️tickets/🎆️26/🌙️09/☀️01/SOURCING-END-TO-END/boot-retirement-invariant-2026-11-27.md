# Boot retirement-receipt invariant (2026-11-27)

## Invariant
`collect_original_publication_receipts` (`⚛️reactor/🔄️turn/🩹️reconcile-drive-and-publication-receipts/🦀️.rs`) refuses when the receipt of a family (here Retirement) is smaller than what the publication window already posted on the recipient (`OriginalPublicationReceiving.posted`, bound by `PluginExchangeOutput::bind_window_posting` in `⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs`). Observed: receipt 64 items / 12569 B, received 111 items / 26011 B.

## Root cause
Working-tree edit (mtime 14:47, diff against HEAD fa8a184 of 11:17) of `🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs` and `🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs`:
- `retire_typed_operation_run` dropped the `if pump != 0 { return Ok(()) }` exit (present since at least 11-26 15:22) and `continue`d after a granted retirement run.
- `retire_granted_typed_operation_run` took the pending `original_retirement_receipt` back and accumulated into it, and the frontier guard `original_retirement_receipt.is_some()` in the granted step was removed.
Effect: direct retirement units (pump > 0) and the granted run shared one publication window, but only the granted run's progress is reported as the Retirement receipt, so the window posting exceeded the receipt. At boot `setActiveExample` retires the previous example's typed operations, hence `error:sourcing`.
Other peer changes since 11:30 (job outcome budget `witness_copy_bytes`, actor close_native_step, sqlite decode_framed, natural-file prints) do not touch receipt accounting.

## Fix (revised 16:3x, shared-window design kept; peer's version not reverted again)
A peer deliberately re-edited both files (15:51/15:55): one publication window continues across direct units and granted runs, the granted runs extend one original receipt (`original_retirement_receipt` taken back, loop `continue`s, no overwrite assert, frontier guard removed). That design is kept. Its defect: the receipt only counted granted-run progress while the reconcile compares it with everything the window posted, direct units included.
Change in `🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs`: `retire_typed_operation_run` now tracks `direct` (progress of direct units settled while no receipt exists). A direct unit settling while a receipt already exists is folded straight into that receipt (`widen_plugin_grant` + `checked_add`). When a granted run opens a new receipt it starts from `direct` (grant widened, spent += direct), so the receipt always covers every effect the window posted. Frontier file untouched (guard removal is consistent: the receipt is extended, never overwritten).

## Test
`🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts` ("mounted retirement preserves its original Close receipt...") asserts direct progress is accumulated/folded and passed into the granted run, which widens the grant and adds it to spent (37 pass; 3 other tests there fail on stale source-text expectations, unrelated). A native Rust test of direct-then-granted in one window was not feasible: no existing native fixture drives `retire_typed_operation_run`; the end-to-end check is the rebuilt hub boot.

## Verification
Rebuild: `NATIVE=1 sh rebuild-sourcing-retry.sh semio-hub-sourcing-component 2` (log `🗑️generated/boot-inv-rebuild.out`); result recorded below.

Result (16:53 hub wasm, NATIVE=1 rebuild with the direct-progress accounting; the peer's 16:46 variant of the same file carries the same accounting): boots to ready, no "smaller than" in any journey log; core journey exit 0 (en and de PASS). Editing/history/surfaces journeys were still running when this was written; see 🗑️generated/all-journeys/summary.txt.

Final journeys on the 16:53 hub: journey exit 0, editing exit 0, no "smaller than" anywhere. history exit 1 only for "revert to command takes back only the newer commands" (en, de: revert kept the newer command; the revert agent's scope), surfaces exit 1 only for "module toggles ..." (en, de; known cascade from the surfaces agent's scope). Neither is the retirement-receipt invariant.
