# 🔁️ Open File livelock: MediaIn decode starved of retained items (2026-11-27)

## Symptom
With the wasm rebuilt from the tree of 2026-11-27 03:30, every Open File in the browser stalls: `command stalled: ... operation: media-in, stage: continuation 25431/16 (guest status more-work, ingress command-pending)`. The browser shows "stdio-pdf has not answered" after about 260 s. The wasm of 2026-11-25 20:32 opened the same files.

## Native reproduction
Law `media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_the_interactive_grant` (editor unit tests, crate `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests`):
`sh native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib reaches_its_mount_through_the_real_runtime_under_the_interactive_grant -- --nocapture`

- **RED:** `MediaIn neither completed nor faulted within 512 turns`. The same law under the shard budget (`..._under_the_shard_budget`) completes.
- **Census:** I extended it with the turn's more-work sources (`last_turn_more_work_sources`, now exported through `semio_plugin::reactor_turn_laws`). Every turn shows `CommandPending / MoreWork / ["command_ingress"]`, with no typed operation pending and no contention.

## Cause (traced with temporary probes, removed)
1. Ingress assembles the one-page MediaIn command (`CommandIngressOwner::Generic`), and `exchange_generic` runs every turn. Output never arrives and no failure is raised.
2. In `plugin_exchange`, after about 50 store-constructor calls, every call goes into `PluginCommandIngress::advance_original`. That returns `Pending` forever, because the command is stuck in `Decoding`: `OriginalCommandDecode::step_original` never moves.
3. At that point the turn's budget reads `fuel=49,998,852`, `deadline_exceeded=false`, `retirement_should_yield=false`, and **`retained_grant().maximum_items == 0`**, on every turn. Every decode move needs one retained item (`copied_items:1`), so it never fits.
4. The interactive per-turn grant (64 items) is used up before the decode runs. The FEM ticket found the same item starvation natively for the store initializer (`26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END/boot-store-replacement-hang.md`, section 2). It traced it to the original-return escrow scope edits made between 01:12 and 03:17 today in `🧵️job/⏱️context/🛬️owned` and `📬️outcome/🪙️budget/📦️stock/🎟️scope`. The return escrow quotes about 49–63 of the 64 items (`OriginalReturnEscrowAmounts { items: 53 }`, `[DEBUG] plan ready=true wanted={items 8} quantum={items 1}`). Those files were still being edited by their owner at 08:49 and 09:04.

## Why it is a livelock, not a fault
Each exchange call alternates between a quiet call and a call that spends exactly 1 fuel unit. That keeps resetting `note_command_turn`'s quiet streak, so `plugin.command-stalled` never fires. A decode starved of items (or of any other per-turn grant component) for N turns should fault rather than spin. That belongs in the stall rule, once the escrow owner settles the budget split.

## State
- **Not fixed by this ticket:** the escrow-scope area is under active development by its owner. Editing it concurrently would race their fix.
- **The law is the acceptance gate.** Once it is green, rebuild and stage the wasm (`round6-chain.sh`, `stage-pdf-component.ts`) and rerun the full journey and the oracle.
- **Kept changes:** the census now names the armed more-work sources and shows the full ingress cursor (240 characters), and the law re-export.

## Refined cause (2026-11-27, 12:00–13:30, temporary probes removed)
Traced with the same law, top down:
1. `drive_command_ingress` resumed the live ingress owner only one turn in three, because `resume` settled the slot's previous return one step per turn. **Fixed (kept):** `resume` now repeats `settle_step` within the turn's slice through the file's own `step_until_slice`. The ingress now resumes every turn (512 of 517 turns).
2. The MediaIn custody then never leaves `prepare_media_descriptor`. Its `OriginalPackReceiving<MediaArtifactDescriptor>` goes through parse (phase 0), admission (2) and native receiving (6), then sits in **phase 8, `close_sources`**, forever. The receiver is not terminal and not closing, so every call asks `retired_native` to begin the close, with demand `{copy 1, depth 1}`, which the grant admits (52 items free).
3. `retired_native` → `StepContext::with_native_retirement_receiving` → `original_receiving_outcome` (`🧵️job/⏱️context/🛬️owned/🦀️.rs`) **returns with a scope but never runs the closure**: `scope=Some`, `ran=None` on every call. None of the function's own `return Ok(false)` exits fire, so the outcome comes back filled without the operation having run. The last suspect is `owner.state.run_scoped_on_caller` (its `ready` closure or the scope plan's quantum).
4. This is the return-escrow scope code from 2026-11-27 01:12–03:17 (FEM ticket, `boot-store-replacement-hang.md` section 2), still being edited by its owner (files changed at 08:49, 11:15–11:32). Fixing it concurrently would race them, so it is handed over with this reproduction.

## Handoff
- **Reproduction:** `sh native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib reaches_its_mount_through_the_real_runtime_under_the_interactive_grant -- --nocapture`. Pass `--nocapture`: without it the abort at thread exit swallows the assertion message.
- **Expected after the fix:** the law reaches `CommandComplete` within 512 turns. The browser then opens files again, which needs a wasm rebuild and the `round6-chain.sh` + journey run.
- **Separate native-only abort:** the persistent `COMMAND_INGRESS` slot panics on drop at thread exit (`actor inline return requires actual same-Source observation`). It is a side effect of the unfinished command, not a cause.

## Further findings (2026-11-27, 12:40–14:00)
- **Cause of the stall:** the scope opens, but `with_original_effect` on the recovery handle declines with `None` ("work quote not admitted"). `retired_native` quotes `receiving_units(fuel)`, about 1,048,576 work units, for a `{copy 1, depth 1}` close, more than the pooled retirement scope grants. The body grant itself was fine (1 item, 81 bytes, depth 1).
- **Experiment:** capping that quote at 4096 units removed the stall. The import then hit the next error: `original publication full receipt of Retirement is smaller than its already received effects` (receipt 20 items, received 21). The second refusal then tripped `original Reactor refusal slot cannot replace an earlier original output`. That is a double-count between the escrow scope's posted receiving and the producer's retirement receipt, in the same owner's area.
- **Reverted the experiment:** with it, the shard-budget law also failed on the receipt mismatch.
- **Baseline at 13:50 (both of my experiments reverted):** the shard-budget law, which completed this morning, now stalls too ("neither completed", never mounted). The open path is broken for every budget by the ongoing job and escrow refactor, independent of this ticket's changes.
- **Kept:** the ingress settle loop (`resume` → `step_until_slice(settle_step)`). The interactive-grant ingress then resumes every turn, and the shard-law regression is present with and without it.

## Third pass (2026-11-27, 15:00–15:30): receipt mismatch localized, then stopped
- **Retirement quote cap re-tried (reverted again):** with it, the import reaches the publication window that mixes families. One `advance_typed_operation_publication` window posted 52 items while the app emitted only a **Retirement** receipt of 5 items. The preceding Preparation windows posted about 50 items each. `collect_original_receipts` picks one family per window by priority, so preparation work and a retirement step in the same window can never match one family receipt. The fix belongs in the typed-operation publication pump: stop the window after a receipt-bearing unit, or emit one receipt per family.
- **Measured and ruled out:** escrow scope claims during these windows are zero, so the escrow admission charge is not the mismatch.
- **Stopped:** `🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs` was edited at 14:47 by another agent (`L3D_TRACE` probes, Puzzle ticket). All measurement probes and counters are removed, and `🛬️owned/🦀️.rs` is byte-identical to its pre-probe copy. The cap is reverted, because the receipt mismatch ends in an assert, and a guest panic is worse than a stall.
- **Required to unblock (owners: job/escrow and typed-operation publication):** (1) `retired_native`'s work quote must fit the pooled retirement scope; (2) a publication window must not mix receipt families.

## Precise location of the receipt mismatch (15:35)
The pump (`advance_typed_operation_publication_turn`) runs one unit per window while operations exist, so the mismatched window was a single **retirement unit**. `retire_typed_operation_run` (`🧵️instance/🧾️typed-operations/📥️staging/♻️retirement/🦀️.rs`) posted 52 items, but `original_retirement_receipt` records only the `spent` of its granted-retirement loop (5 items). The postings of the same unit outside that loop are not in the receipt. That function was edited at 14:47 by the Puzzle ticket's agent (its `L3D_TRACE` "granted retire spent" probe sits exactly at the receipt), so it is left to them.
