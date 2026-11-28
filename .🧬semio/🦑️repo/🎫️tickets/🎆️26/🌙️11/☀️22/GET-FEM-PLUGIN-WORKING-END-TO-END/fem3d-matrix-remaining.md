# 🧊️ FEM 3D matrix: remaining failures and two follow-ups

Ticket `26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END`. Own headless Chromium on http://127.0.0.1:6087/ (published wasm of 06:11), results under `matrix-v3` via `run.mjs 3d --out <dir>` (never the coordinator's `matrix-v3/3d`). Debug drivers: `matrix-v3/rf-driver.mjs`, `matrix-v3/rf-profile.mjs`, `matrix-v3/rf-vitest.config.mjs`.

## Verdict

All nine checks PASS in my own Chromium (`--out rf/r6`) against the coordinator's wasm of 10:29 (it contains the Rust change below); the TS fixes are live through vite. Measured profile numbers in follow-up 2 were taken before that publish.

| Check | Root cause | Kind | Fix | State |
|---|---|---|---|---|
| `3d.shell.fullscreen` | With the Inspection dock open (top-right) the dock's panel root (`z-panel`) paints and hit-tests over the navbar's trailing controls. The dock already reserves that strip in its cap row (`chromeHostedPanelCapRowStyle`), but nothing cut it out of the dock, so the Fullscreen button sat under the dock (`elementFromPoint` returned `panel-content`). The stale pass was a run in which Inspection happened to be folded | product (framework TS, 2D and 3D alike) | `chromeHostedPanelTrailingClearStyle`: a `clip-path` on the open top-right chrome-hosted dock that leaves the reserved strip out of paint and hit area | live in vite, PASS |
| `3d.del.inspection-material`, `3d.del.inspection-section` | Intended refusal (a material/section something still names is protected), but the refusal was SILENT. The guest emitted `delete-material`, the diff builder answered `mutation.target-referenced`, the publication was rejected (`interactive-job.publication-rejected`) and no notice existed | product (Rust) | `removeSelection` now checks referrers up front and refuses with a Fault carrying `{id}` and `{referrers}`; EN and DE notices in the editor's `fault_notices` table (`fem3d.remove-selection.material-referenced`, `.section-referenced`, `.load-case-referenced`, 2D twins `fem2d.…`); the checks assert the role=alert notice | native law green (9/9 `remove_selection` laws in `editor_app_2`); live on the 10:29 wasm: PASS, notice "Nothing was removed: the material 'steel' is still used by e1, col_00_u, ... Remove or change those first." German is asserted by the native law only (a browser DE row was tried and dropped: switching the language from the check did not take effect) |
| `3d.del.inspection-loadNodal` | Harness: removing one load does not change the summary counts (`Load Cases 2` counts cases), so "entities 46 -> 46" was the assertion, not the product | harness | asserts the load row leaves the tree | PASS (2 rows -> 0 in 2.6 s) |
| `3d.del.inspection-support` | Harness (wrong tree, fixed by the previous agent's scoped `inspTree`) | harness | none needed | PASS in isolation (47 -> 46) |
| `3d.del.palette-remove-selection` | State leak: an earlier delete check had already removed the element row `col_20_u`, the check then waited 70 s on a missing row; the product removes the selection in about 1.4 s | harness | check is `fresh` | PASS |
| `3d.del.combination-term-zero` | State leak (`del.inspection-combination` had deleted the only combination, "no term factor field"). The old assertion also passed vacuously (it searched the Artifact tree for "dead", which is never there) | harness | `fresh`; asserts the term input leaves the Inspection (3 -> 2) and the other term stays | PASS (4.9 s). Product law already exists: `patch_combination_reweights_adds_and_removes_terms_3d` |
| `3d.inspect.solid-readonly` | State leak (`del.inspection-solid` removed `sol1` before) | harness | `fresh` | PASS |
| `3d.ckp.switch-alternative` | Harness: the `switchAlternative` form needs an `alternativeId` and its Execute stays disabled without one, so `runAction` timed out on a disabled button. The id is not shown as text, it is the History row's DOM id (`framework.history.alternative.<id>`) | harness | creates an alternative when there is none, reads both ids from the History rows, switches to the non-current one, asserts the `Current` label moves | PASS (switched to the other alternative, the `Current` label moved in 4 ms) |

## Follow-up 1: Transform options hidden under the Artifact dock

Root cause (framework TS): `Pane` yields to a docked chrome panel only when the panel covers the pane's HEADER (`headPx`, the F4 rule). The Model window's Utilities pane is anchored bottom-left, so its header sits under the footer band, below the Artifact dock (dock bottom y 942, header y 968 to 1000) and never clears it, while its open BODY (the Transform option checkboxes, y 844 to 933) lies under the dock. `elementFromPoint` at the checkboxes returned the dock's tree label.

Fix: `chromePanelOpenSafeArea` (`🔨️modules/🛟️chrome-safe-area`) grades an OPEN pane's header against every panel (as before) and its whole body against the docks that cover at least `windowViewportCoverMinShare` (half) of the host height, the same docks that already inset the viewport, and takes the wider shift. `Pane` passes `openBodyShare` when it is not folded. 2D and 3D share `Window`/`Pane`, so both get it. Live: the pane moved from x 0 to x 300 (`data-safe-area-inline="300"`), all five checkboxes are hit-testable (screenshot `rf/opts-after.png` before cleanup).

Law: `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🛟️chrome-panel-safe-area/🟦️.ts`, `F5` (new): the exact live geometry; fails on the old rule (header alone: no shift), shifts by 300, checked against an independent Three `Box2` intersection; short chip rows and bands never push an open pane aside; F4 stays green. `F6` (new): the fullscreen cut-out polygon, checked with `polygon-clipping` (the dock keeps `300*365 - 88*29` px, the trailing strip is outside). `vitest run` of that file: 7 tests green, plus the existing `aria-pressed` law. The mounted Window test in `🟦️.tsx` needs the real suite's `ResizeObserver` setup and was not runnable with my minimal config; its pane rows (`folded-*`) were not touched.

## Follow-up 2: selection round trip 3 to 7 s under load

Measured with `matrix-v3/rf-profile.mjs` (a `Worker` wrapper plus `longtask` observer injected before boot, 3D Demo, 100 % CPU, about 36 cargo/rustc processes running). Per canvas click, `interactionSelect` to the new id in the Inspection:

| | click 0 | click 1 | click 2 |
|---|---|---|---|
| wall time to the new selection | 7.9 s | 3.7 s | 4.8 s |
| guest time inside wasm (sum over `turn` replies) | 6.0 s | 2.9 s | 3.8 s |
| host to worker post latency (sum) | 0.27 s | 0.02 s | 0.08 s |
| worker reply to main thread delivery (sum) | 1.0 s | 0.6 s | 0.5 s |
| main-thread long tasks | 5 to 7, 55 to 150 ms, all in the last second | | |

Crossings per click: about 150 `turn` requests, about 130 `stepJob`, 1 `startJob`, about 290 results and about 670 `original-turn-receipt` messages (idle: 0 messages in 4 s, so nothing polls). The `stepJob` count comes from the retained tool job of the command; the `turn` requests alternate an empty drain turn with a re-offered command page the guest answers with backpressure while its job is running.

Findings:
- The main thread is not the bottleneck: it is idle until the last second, then renders (60 to 150 ms tasks). The post and delivery hops cost 10 to 20 % of the time.
- 65 to 80 % of the time is guest compute inside the `turn` calls: about 40 ms per turn times 77 to 132 turns. At idle that is the 1.1 to 1.5 s measured earlier; under CPU contention the same work takes 3 to 5 times as long, so the "3 to 7 s" is the machine load multiplied by a fixed amount of work (about 290 crossings, about 670 receipts, about 3 to 6 s of wasm).
- TS experiment, reverted: giving `stepJob` a worker-owned drive (like the existing MoreWork drive for `turn`) halves the worker messages (1294 -> 696, 1242 -> 610, patched worker injected with `page.route`), but the click latency stays inside the noise (3.9 to 5.3 s vs 3.7 to 5.7 s) because the guest turns dominate. No clear host defect, so nothing was shipped.
- The defect, if any, is in the guest/protocol: one selection costs 77 to 150 guest turns because the command page is re-offered and refused with backpressure while the job runs, and every turn pays the origin refresh and receipt traffic. Recommended Rust-side follow-ups (need the coordinator's publish and a profile of the wasm): coarsen the retained job step to use its 100 ms wall budget (it is stepped 130 times for one select), stop re-offering a page while the actor's own job is running, and fold the 4.4 receipts per crossing into one.

## Native laws run

`cargo test -p semio-s-artifact-fem-3d-editor --test editor_app_2 remove_selection`: 9 passed (new `…referenced_target_is_refused_with_a_localized_notice_3d`; the mounted atomicity law now pins `fem3d.remove-selection.material-referenced`). Same for `semio-s-artifact-fem-2d-editor`: 8 passed (new `…_2d`, referrers read from the shared JSON fixture with serde_json). `vitest` (`matrix-v3/rf-vitest.config.mjs`): chrome-panel-safe-area 7 tests and PanelTabBar aria-pressed green.

## Files

Product TS: `🔨️modules/🖱️ui/🔨️modules/🛟️chrome-safe-area/🟦️.tsx` (`chromePanelOpenSafeArea`, `chromeHostedPanelTrailingClearStyle`, `openBodyShare`), `🧱️elements/📌️WindowPane/🟦️.tsx`, `🧱️elements/🖼️Panel/🟦️.tsx`, law `🧪️tests/🛟️chrome-panel-safe-area/🟦️.ts`.
Product Rust (not yet compiled into the served wasm): 3D `✏️editor/🎮️commands/🗂️remove-selection/🦀️.rs`, `✏️editor/🖥️play-app/🦀️.rs` (notices), law `…/🧪️tests/🔬️unit/🦀️.rs` (`remove_selection_of_a_referenced_target_is_refused_with_a_localized_notice_3d`, mounted law now pins the fault code); 2D twins in `◻️2d/…/🗂️remove-selection` and `🎛️play-app-editor`.
Harness: `matrix-v3/checks-core.mjs` (referenced material/section assert the notice, new `del.refusal-de`, loadNodal, fresh flags, switch-alternative), `checks-model.mjs`, `lib.mjs` (`alerts`, `waitAlert`).
