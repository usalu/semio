# Pointer-up publication stall (2026-11-23)

## Status
The root cause is not yet pinned to one line. The stall is localised to a silent no-op branch of the typed-operation publication ladder that runs AFTER the store has committed the edit. All silent branches are now instrumented (guest change, needs a rebuild); the next browser run names the exact cause in the fault message.

## Reproduction (about 15 s, no folder)
`probe-rect-lean.mjs <out> nodiag|diag <wait-ms> [switch]` (Playwright; `PLAYWRIGHT_BROWSERS_PATH=$PWD/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`) boots Draw, presses `shapeRect`, drags and releases.
- Layers before the drag: `1 layer`; after the release: `2 layers`. The rectangle IS committed to the document.
- Yet `canvasPointerUp advanced 4096 publication units without moving its ladder` is raised and all later inputs are refused.
- `probe-addlayer.mjs` (click "Add Path") stalls identically: `typed operation 'addLayer' advanced 4096 publication units`. So it is not gesture specific: every artifact-committing typed operation stalls in the browser guest. `setActiveUtility` (window config) passes; with `SEMIO_RUNTIME_DIAGNOSTICS=1` the boot `setCamera` (lane WindowConfig) stalls too (timing dependent).
- The persistence folder is irrelevant (the lean probe never attaches one).

## Evidence (diagnostics armed through `localStorage SEMIO_RUNTIME_DIAGNOSTICS=1`)
```
[TRACE] typed-operation canvasPointerUp advanced 4096 units with no change: operation=512 stage=Publishing flags=0b01011100
  lane=Some(Artifact) checkpoint={cursor:1, completed_items:3, completed_bytes:688} returned_reads=0 fuel=47.8M
  grant={items 7.1M, copy 2.0G, capacity 325M, release 424M, depth 64}
```
- Fuel and grants are plentiful: not an unfunded loop.
- Flags: completion, publication, pending artifact publication, terminal seen; no result page yet.
- Fuel drops by about 1 per unit: the units do nothing (the pump's `consume_fuel(1)` fallback).
- The fault text has no ` : waiting on ...` suffix, and a scan of the actor's linear memory (`probe-rect-wait-scan.mjs`) finds no `LAST_PUBLICATION_WAIT` cause string: no unit ever recorded a wait. The stalled units leave through branches returning `Ok(())` without `note_publication_wait`.

## Silent branches (return `Ok(())`, no note, witness unchanged)
In `typed-operation-unit/🦀️.rs`: publication-failure reservation; `advance_pending_result_page` answering true; `reserve_store_result` returning false on the accepted-receipt branch (the state the committed rectangle is in); `close_streamed_transaction_unit` false; `admit_apply_batch_with_pages` returning `Ok(None)`; effect, event and ui outbox pages (`retained_work_can_enter` false); child emit preparation `Pending`; window transient refresh `Pending` (two sites). In `🔁️stall-witness-and-unit-advance/🦀️.rs`: window-config refresh (another agent added notes there concurrently).

Strongest suspect (unproven): the post-commit accepted-receipt branch or the result-page ladder. `close_original_admission` (`🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🦀️.rs`) returns `Ok(None)` ("retry on a fresh turn") whenever the remaining loan is below a demand that still fits the Source total, so a loan permanently smaller than its demand spins forever with no fault. The native `addLayer` test passes with `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` (depth 4096, no clock); the browser guest has depth 64 and a real turn deadline.

Second defect: the stall witness (`publication_progress`, stage, flags) does not see progress made by the result-page ladder or the window-config refresh, so a slow healthy ladder is indistinguishable from a stall.

## Change (guest, needs a rebuild)
`typed-operation-unit/🦀️.rs`: each silent branch above calls `note_publication_wait(op, cause, [..])`, so `fault_stalled_typed_operation_publication` appends the cause and its counters. No other behaviour change.

## Empty console errors
The five empty `console.error("")` come from `preview2-shim/cli.js:84` (`consoleStream((line) => console.error(line))`, patched in `🌐️browser-bundle/🏗️materialization/🩹️preview2-shim-patches`): the guest writes empty stderr lines. They appear right after the `typed-operation slots live=0/64` trace in the full journey and are not reproduced by the lean probe, so they come from the example/blank switch or attach. The guest source line is not located; `probe-pointer-up.mjs` logs `[PROBE] empty console.error <stack>` for the next full-journey run. Host-side hardening regardless of source: classify a blank stderr line as `debug` in `classifyGuestLogLine` (`🌐️wasi/🟦️.ts`) and in the patched Preview2 shim, because a blank line is never a fault.

## Next
1. Rebuild Draw (not done here).
2. Run `probe-rect-lean.mjs <out> nodiag 20000`; the stall message now ends with the cause.
3. Fix that cause; add a native test with the browser grant (depth 64, real clock).

## Verification
Compile check of `semio-plugin-instance-app` was started after `repo cache gc` (the build image did not mount before); see the result in the reply. No native test was run.

## Files
- changed: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs`
- probes: `probe-rect-lean.mjs`, `probe-addlayer.mjs`, `probe-rect-wait-scan.mjs`, `probe-pointer-up.mjs`; outputs in `🗑️generated/`.
