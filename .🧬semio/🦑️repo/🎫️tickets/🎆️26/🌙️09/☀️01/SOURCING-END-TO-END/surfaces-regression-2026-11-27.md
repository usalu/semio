# Surfaces journey regression (2026-11-27)

## Ran
- Full `journey-surfaces.ts en` twice against 6081 (13:55-14:15): both 25 PASS / 9 FAIL (module toggles, typology, stock names, curated sorts, curate action, both Empty example steps, preview empty state, grid GLB). Logs: `🗑️generated/surf-reg1.log`, `surf-reg2.log`.
- Run must start from the repo root (journey-kit resolves `process.cwd()`).

## Findings
- Not a product UI regression in sourcing/framework TS, and not a label or journey-expectation issue. Fresh load shows all 12 rows and stays at 12 for 90 s (`probe-row-decay.ts`).
- Cross-session interference is excluded: every journey uses a fresh Playwright context (own storage/OPFS); no shared folder is attached by these steps.
- The cascade has ONE origin: the module-toggle step. Final screenshot shows Beams+Windows still pressed, 7 rows (beams+windows), i.e. the toggle-off commands were not applied; every later step then starts from a filtered pool (7 rows, missing slabs/reuse) and cascades (typology, sorts, curate, Empty example "kept rows" 7!==0, etc.).
- It is timing dependent: step subset `module toggles` alone PASSES; `pool sorts + ... + grid` subset (10 steps) all PASS; `breadcrumb + module toggles` FAILS; `every control + module toggles` and `window labels + module toggles` PASS. The breadcrumb step is read-only, so only the start time of the toggle step changed. => lost/refused input under load (the host had cargo/rustc builds running), not deterministic product logic. `set-filter-module` handler (`✏️editor/🎮️commands/🧱️set-filter-module`) and the declarative Toggle dispatch (`pressed` from the click) are correct.
- Suspected mechanism: framework input refusals ("input #N ... refused: dispatch-failed", `original-turn.native-return-refused`). At 14:09+ the served build emits at boot `original publication full receipt of Retirement is smaller than its already received effects` (kernel, `🧰️framework/.../🔌️plugin/⚛️reactor/🔄️turn/🩹️reconcile-drive-and-publication-receipts/🦀️.rs`, retirement receipt accounting, being edited by another agent) and `setActiveExample` is refused; the app then boots to `error:sourcing`. Also the wasm emits many `[DEBUG]` archive/replacement lines (not mine).

## Fixes
None made: no product defect found in sourcing UI/framework TS; no Rust change from me; journey unchanged (an `eventually` around the click-off in the module-toggle step would mask, not fix, a dropped input).

## Open
- 6081 serves `error:sourcing` since ~14:09 (kernel retirement invariant above) through at least 14:43; a rebuild (rebuild-sourcing-retry.sh) is still in flight. surfaces en/de could not be rerun on a healthy build.
- Next step once ready: run surfaces en with the console captured for the `module toggles` step (`STEPS="breadcrumb|module toggles Beams"`) and look for `refused` warnings between the click-off and the stuck state; if confirmed, the owner of the retirement/archive turn work must fix refused user inputs while an archive/retirement turn is in flight (and add a Rust unit test there).
- Added input script: `probe-row-decay.ts` (row/toggle state probe).

## Update 18:30 (6081 green again, product fix landed)

### Cause of the lost toggle, in two layers
1. Product (fixed): `set-filter-module` rewrote the whole `moduleIds` list from the config snapshot it was rendered against (read-modify-write), so a toggle issued from a stale view overwrote the previous one. Replaced by the event `ToggleFilterModule {module_id, enabled}` (config mutation `toggle-filter-module`, same ordinal as the old `filter-modules`): `diff()`, `inverse()` and `prepare_sourcing_curation_config` compute the new list against the STORED base at apply time (`ToggleFilterModuleEdit::applied_to`). The command `setFilterModule {moduleId, enabled}` was already per-module and stays; its handler no longer reads the snapshot.
   Files (all under `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`): `🎚️config/🦀️.rs`, `📬️config-store-preparation/🦀️.rs`, `🎮️commands/🧱️set-filter-module/🦀️.rs`, `🦀️.rs` (import), tests `🎚️config/🧪️tests/🔬️unit/🦀️.rs` (new `back_to_back_module_toggles_both_apply_and_undo_in_order`, round-trip and op text/binary cases), `🧪️tests/🧬️app/🦀️.rs`, `🧪️tests/🌉️actions/🦀️.rs`. No JSON schema/descriptor mentions the old mutation; hub descriptor after rebuild: committed MATCH.
   Verified: `cargo test --workspace` in 🗂️curation exit 0, 44 test binaries ok, the new test passes (`🗑️generated/curation-native-test-toggle2.txt`).
2. Host/kernel (open, not mine): after the rebuild (hub wasm staged on 6081) surfaces en+de = 66 pass, 2 fail (only "module toggles", the journey keeps rapid back-to-back clicks). Evidence with `globalThis.__semioInputLedger`: the first `setFilterModule` is `applied`, the second, issued immediately after, stays `outcome: null` (pending) for 20+ s, no refusal, no console warning, no pageerror, ledger `superseded 0`. Singles with a pause between clicks pass; waiting for the first toggle to render (`aria-pressed`) before the second click passes the "on" phase (journey-only variant gave surfaces en 34/34 earlier). So a second input arriving while the previous command's turn/refresh is still in flight is never dispatched to completion. The queue point is the worker action lane (`🏪️store/👷️worker/🧷️actor-reservation/🎯️action-dispatch`: `awaitUiQuiescence` then `enqueueTurn`) or the guest command-ingress turn; the Rust kernel files `⚛️reactor/🔄️turn/⌨️command-ingress-turn`, `🧵️retained-command/🔄️full-operation`, retirement receipts and `reconcile-drive-and-publication-receipts` were all modified by other agents since the 05:25 pass (the `settleYieldsToQueuedCommandV1` host fix dates from 05:06, i.e. before the pass). Not changed by me.
   Suggested next owner: whoever owns the command-ingress/retirement work; reproduce with `STEPS="breadcrumb|module toggles Beams" JOURNEY=surfaces bun journey-surfaces.ts en` (fails reproducibly; 12th ledger entry pending).

### Other results in the same rebuild run (`🗑️generated/all-journeys`)
core exit 0, editing exit 0, surfaces 66 pass / 2 fail (en, de module toggles), history exit 1 (12 FAIL lines, e.g. "revert to command takes back only the newer commands"; not investigated, outside scope).

### Journey
Reverted to the original rapid back-to-back clicks (guards the bug). Temporary debug edits removed.
