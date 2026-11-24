# FEM Browser End to End (2026-11-23)

## Route

- Native component build: `native-fem-dev.sh wasm` with the generated `prototype-component` profile (`native-fem-component-profile.py` → `native-fem-profile.toml`). 144,672,798-byte component, sha256 `f58876c2…`, linked in about 35 minutes cold.
- Browser module: `native-fem-materialize.ts <component.wasm>` runs the staged native transpile (`.cache/b/p/m/browser/prototype/tools/native-transpile/win32/semio-native-transpile.exe`), the jco async-result and asset-url rewrites, the preview2 import rewrite against the published module directory, and regenerates `🌉️bridge.js` (`pluginComponentBridgeSource`), `🟨️.js` (`hostShimSource`), the finalized descriptor pair (`finalizePluginDescriptor` of the committed semantic pack) and the receipt digests.
- Dev servers: `native-fem-dev.sh serve2d|serve3d` (vite on 6086/6087).

## Findings

1. The Nov 20 published module refused the first window poll with `Window owner must declare its original disposer birth`. Current source implements `store_disposer_birth` for all four FEM window owners; the stale binary was the cause. Fixed by the fresh build.
2. Publishing only the transpiled component with the Nov 20 bridge failed with `original-turn.source-not-ready`; the bridge and host shim are generated from current host source and must be regenerated with the component. Fixed in the materialize script.
3. With the fresh module both editors boot (`data-semio-os-ready=fem2d` / `fem3d`), paint model and results canvases, and spawn the mounted analysis job. The results stay `Pending` (2D) / `Preparing analysis` (3D):
   - The first job (initial document) runs about 1,300 host steps and then fails admission with `fem2d.session-stale-factory` / `fem3d.visual-stale-factory`, because the shell's initial example reload replaced the document meanwhile. This is the intended refusal of a superseded job.
   - The job for the Demo document fails after 87 (2D) / 114 (3D) steps: 2D with an empty fault payload, 3D with `fem3d.numerical-child-fault`. Both are the numerical child job faulting; 3D's `child_end` discards the child's fault detail and 2D forwards an empty child detail.
4. Temporary `[DEBUG]` prints were added to capture the child fault detail (FEM 2D `MountedState::fail` and job bridge, FEM 3D `child_end`, host `driveSpawnedJob`). A rebuild with these prints is running.
5. Native control (`a3d-session-demo.sh`): `fem3d_production_numerical_child_solves_the_demo_with_paged_owner_backing`, `…_concrete_forest` and `…_house` pass (3/3, 392 s). The numerical child solves the Demo natively, so the browser fault is specific to the wasm32 guest or the browser job driver.
6. The canonical describe loop (`pub-describe7.sh`) spent about two hours in the graph inventory on attempt 14; it was stopped in favour of the native publication route.

## Root Cause: Child Operation Identity

The debug build printed `[DEBUG] mesh job fail at=🕹️mesh-job-run/🦀️.rs:13 message="stale-mesh-operation"`. `MeshJob::run` (and every bounded child) refuses a step whose `StepContext::operation()`/`generation()` differ from the `Operation` it was built with. The browser binds an isolated job's StepContext to the host-issued operation and generation (`original_job_binding` in `⚛️reactor/🎟️context/🌉️call/🎭️exports`), but both FEM sessions built their child operations from the job id and the document generation:

- 2D `MountedIdentity::operation()` → `Operation::new(identity.operation, base_revision, identity.generation, seed)`.
- 3D `MountedState::step` → `Operation::new(identity.operation, base_revision, identity.generation, job)`.

The native tests admit their fixture with that same pair, which hid the mismatch. `MeshJob::fail` also drops its message, so 2D surfaced an empty fault and 3D only `fem3d.numerical-child-fault`.

Fix: the child operation takes the host-bound `cx.operation()`/`cx.generation()`; the document identity only seeds revision and seed (2D `MountedIdentity::operation(operation, generation)` at all five call sites, 3D `MountedState::step`). Regression: `mounted_demo_static_publishes_live_load_reaction` now admits its fixture under a host operation and generation that differ from the document identity, which is the browser's situation.

## Results After the Fix

- With the child-operation fix, the mounted analysis completes in the browser: 2D `semio.fem2d.mounted-analysis` done after 766 host steps, 3D `semio.fem3d.mounted-live-visual` done after 2,022 steps.
- After an interaction, the 2D results window paints the converged solution (`Converged · 36 / 512`, residual and reaction label).
- Native session suites with the fix: 2D `semio-s-artifact-fem-2d-editor-session` 21/21 (`a2d-session-demo.sh`), 3D numerical child 3/3 (`a3d-session-demo.sh`).
- Windows native test note: a split `CARGO_BUILD_BUILD_DIR` adds one `-L dependency=` per unit and overflows the command line (os error 206). The repo `.cargo/config.toml` routes rustc through the ingress script, so the native scripts override it with `--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""'`.

## Open: No Repaint on Job Completion

The host delivers `job-completed` as a Background turn (`deliverJobCompletionTurn`). For the finished job, that turn settles with 0 UI patches and status `idle`, so the results window keeps showing `Pending` (2D) / `Preparing analysis` (3D) until a user interaction re-renders it.

On the guest side, `job_completed_event` dirties the instance's background surfaces only when `JobRenderBindingRegistry::complete(job)` returns the binding. It returns the binding only while the job is still the instance's single `current_by_instance` binding. A guest `[DEBUG]` build printed no binding for the FEM jobs at all.

### Root Cause: Pending Effects Bypassed Admission

The FEM editors emit their `SpawnJob` from render-time reconcile, so it reaches the turn through `plugin_drive_pending_effects`. That function appended the app's pending effects with `effects.extend(polled)`. It skipped `push_admitted_effect`, the only place that binds a job to its instance (`JOB_RENDER_BINDINGS`) and admits timers (`ARMED_TIMERS`). An unbound job's completion dirties no surface, so the Background `job-completed` turn renders nothing.

Fix: `plugin_drive_pending_effects` takes an `admit(instance, effect)` callback and hands every published effect to it with its owning cell id. The turn passes `push_admitted_effect`. Regression law: `pending_effects_are_admitted_under_their_owning_instance` (`⚛️reactor/🧪️tests/🔬️command-ingress-terminal`).

### Other Observations

- The blank 3D model viewport in the hidden app browser pane is not a bug. `requestAnimationFrame` never fires while the pane is hidden, so three.js never draws; the scene's mesh data is present. Verify with headless Playwright instead.
- The rebuild at 21:36 picked up a peer change in `🌱️value/🛬️decode/🧳️owned/🧩️record/🌳️source`. With it, `setActiveExample` and the mounted job are refused with `original-turn: scalar close changed Source or Work`. This belongs to the peer's work in progress, not to FEM.

### Verification of the Admission Fix (Headless, `job-repaint-probe.mjs`)

- 2D: the completion turn of the finished `mounted-analysis` job now publishes 1 UI patch (it was 0 before the fix).
- 3D: the completion turn of the finished `mounted-live-visual` job publishes 2 UI patches.
- Headless Chromium renders the 3D model and results viewports correctly (`🗑️generated/job-repaint-probe-6087.png`).
- The peer refusal `scalar close changed Source or Work` no longer occurs.

### Open: Results View Not Bound at Completion

Both results windows still show their waiting caption after the repaint: 2D `Pending` without progress, 3D `Preparing analysis`. 2D `with_live_fields` returns no view, so one of three conditions fails: the window's live request binding, the current slot, or the revision match. A user interaction later makes the same window show the converged result. An instrumented build (`[DEBUG] fem2d live fields`) is running to identify the failing condition.

### Root Cause: Unaddressed Windows Had No Config Partition

The instrumented build printed `fem2d live fields: no window authority` on every results render, before and after completion. `WindowConfigOwnerRegistry::capture` returns `None` until the Window's native config partition exists. Partitions were born only by `ensure_for_view`, which runs when a command addresses that Window (the inline-interaction path).

An untouched results window therefore had no authority. Every render passed `cfg.window = None`, and `prepare_snapshot_read` skipped the window because `windows.borrow` returned `None`. The window never became a result consumer and showed `Pending` until the first click created its partition.

Fix: `WindowConfigOwnerRegistry::ensure_for_roster` pumps the partition birth of every mounted Window in the roster. `RenderCtx::pending_effects` runs it before the mounted-job preparation and stays armed while births are pending. Regression: `roster_partitions_open_for_every_unaddressed_window` (window config capture laws, `window-roster-test.sh`).
- Native status 2026-11-24: `semio-plugin-window-config-original --lib` fails 10 of 11 tests in the current shared tree. The existing `original_window_authority_capture_…` fails with `native Window partition did not initialize` before any roster code runs, so the regression test cannot pass natively until the peer value/retirement work settles. The browser run is the verification for now.

### Verification of the Roster Fix (Headless)

- 2D: with both fixes, the results window binds and paints the deformed shape with live progress, without any interaction. The completion turn repaints `fem2d-model` and `fem2d-results`.
- 3D: the completion turn repaints `fem3d-model` and `fem3d-results`, but the results window still shows `Preparing analysis`.

## Final Frame at Completion (2026-11-24)

### 2D: Resolved by the Admission and Roster Fixes

- Instrumented host prints (`isolated job terminal`, `job completion turn`) plus a canvas `fillText` recorder show 3 of 3 runs drawing `Pending · 34 / 512` (the last progress repaint) and then `Converged · 36 / 512 · Residual 7.460e-9` in the same instant as the completion turn of job 962.
- `Pending · 34 / 512` is not an unconverged solve. It is the last progress repaint before the final frame.
- Job 961 (the initial document) does start and is pumped for 1313 host steps. It ends with `fem2d.session-stale-factory` because the Demo reload reused its shell. That wastes about 1 s of drive time but does not change the result.

### 3D Root Cause: Job Completion Never Asked the Shell to Repaint

- Instrumented `result_stage` / `with_result_fields` show the 3D job finishing (`done=true`, `Complete`, matching request binding). At completion the guest renders the results window with `Case: dead` and 44 node values, and the host installs `fem3d-results`.
- The DOM still showed `Preparing analysis`. A host-side refresh trace showed no `refreshUi` request after that install.
- The shell paints a surface only on a `refreshUi` pass. A job's completion turn routes no shell frame, and the spawned-job progress lane notified the shell only for throttled mid-drive progress. The final install therefore waited for an unrelated later refresh. It came by chance in 2D, and never in 3D.
- The orphaned `surfacePublicationListeners` door in `🧺️handle-ui-intake` has no subscriber at HEAD either. It was left untouched: wiring every install to a full refresh can loop, because a refresh may cross to the guest and install again.
- Fix: `createSpawnedJobProgressLane` gains `publishSpawnedJobTerminal`. `driveSpawnedJob` calls it after `deliverJobCompletion`, clearing the throttle and notifying the shell's spawned-job listeners, which run a full refresh. Verified headless: the 3D results window switches to `Case: dead` and paints the stress-coloured solid about 1.5 s after completion, without interaction.

### 2D Inspection: Unnamed Number Inputs

- The 2D browser smoke failed after selecting `n1`: `plugin-ui.intake-rejected:validation:Owned UI graph invariants violated (unnamedControl)`. The rejected records were `fem2d-play-inspection.node.x.input` / `.y.input`, inputs inside a `treeItem` row with no accessible name of their own.
- Fix: 2D `control_row` gives the control its row label as its accessible name, as 3D's `control_row` already does. Regression assertion in `a_node_section_carries_a_number_input_bound_to_patch_node_2d`.
- Verified in the browser smoke: the `X` spinbutton is found and accepts `1.5`.

### Open: Live Cleanup Zero Progress on Edit

- With the above fixes the smoke now gets to the node edit. `patchNode` fails with `plugin.internal.zero-progress: runtime live cleanup faulted for instance 1`. The app maintenance answered zero progress or `Blocked` for the whole stall budget.
- Instrumented build 1 (wasm `cd289fdf…`) reproduces the fault after `X = 1.5`. All 256 stalled steps are `Progress(0)` and none is `Blocked`.
- None of the FEM 2D session prints fire (`maintenance_step`, `retire_one`, `close_request_step`). The zero receipt therefore comes from the generic `maintenance_ladder_unit_step`, not from the FEM owner.
- That ladder answers `Progress(0)` whenever the terminal witness is not empty and every present rung is either empty or unfunded. It skips an unfunded rung with `continue`. Candidate: stage 15, the mounted FEM job's close or maintenance quote, is never funded by the live grant, so the FEM owner is never reached.
- Instrumented build 2 adds a `[DEBUG] ladder zero` trace of every rung's demand against its grant.
- Instrumented build 2 (wasm `34481dd6…`, built after peers' maintenance and retirement changes landed) no longer reproduces the stall. The node edit `X = 1.5` lands (`n1 · (1.50, …)`). No `ladder zero` or `live cleanup` line was printed, and no fault.
- The same build's 3D page shows `Preparing analysis`, then `Solving loads`, then `Case: dead`, with the stress-coloured solid, about 48 s after load and without interaction.

### Open: Press Settlement Stalls When a Click Follows the Edit

- The browser smoke clicks `Artifact` right after pressing Enter in the X field. `patchNode` then faults with `interactive-job.publication-stalled: … waiting on the tool settlement is not complete (stage, retained items, fuel) [2, …]`. Stage 2 is `Scrub`, the number field's press settlement.
- Edits without an immediate follow-up click land (probe above). The stall is therefore an interaction between the press settlement in `settle_tool_operation` / `settle_press` and a concurrent input turn.
- Build 3 instruments `settle_press` (context, accepts, phase, yield, grant) to name the guard that never opens.
- Build 3 (wasm `81f7d367…`) result: the edit's Tick press (operation 512) settles. Its Commit press (operation 576, mode 1) holds `settle_press` at phase 3, `OriginalScrubWrite::admit`, for all 4096 units.
- Context and accepts both match, and nothing yields (`yield=false`, full 64-item grant). Turn binding and funding are therefore ruled out.
- `admit` only declines on `ledger.writing` or a read lease on the window's press slot. The slot reader is the tool-overlay follow (`🔁️follow`), which only the maintenance ladder's `ToolOverlay` rung advances.
- Build 4 prints which of the two guards holds and what the follow rung answers.

### Root Cause: Commit Waits on the Follow's Read; Only Maintenance Moves the Follow

- Build 4 (wasm `f1eb03df…`) prints `scrub write admit writing=false read=true window=fem2d-model` for all 4096 units. The Commit's scrub writer waits for the read lease that the tool-overlay follow holds on the window's press slot.
- After the Tick press settled, the follow advanced on maintenance turns (`follow unit … Progress`). Once the Commit's publication ran, maintenance stopped reaching it.
- Live cleanup runs only in a turn's leftover budget: `plugin_step_live_cleanup` after `retirement_should_yield`. Every turn spent its budget re-polling the blocked publication unit, so the follow never finished, its read never dropped, and the Commit never admitted. That is a deadlock, and the stall ceiling turned it into the fault.
- Without a click right after Enter, the follow finished on idle maintenance before the Commit arrived, so the race only showed in the smoke.
- Fix: `OriginalPressSettlement::awaits_writer` (phase 3, no writer). When it holds and a follow exists, `settle_press` quotes the follow (`tool_overlay_retirement_demand`), funds it with `plugin_grant_funds`, and drives one `tool_overlay_retirement_step(cx, grant, false)`, the same step the maintenance ladder's `ToolOverlay` rung takes. The waiting publication thereby releases its own blocker.
- Law: `a_press_commit_blocked_by_the_follow_read_drives_the_follow` in `🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧾️source-rejection-anchors-and-host-lane`.
- Not changed: the typing settlement (`OriginalTypingSettlement`) has the same read-lease guard (`retained_read_pending`) and may need the same treatment. FEM's number inputs use the press path.
- Also fixed in passing: the ticket's `native-fem-dev.sh` pinned a removed cached Vite (`.cache/b/d/m/j/420f…`) and now picks the newest cached Vite. Its `wasm` mode exits 0 on failure, so `wasm-retry.sh` reads the logged `rc=`. `RUST_MIN_STACK=128 MiB` is set against the repeated `STATUS_ACCESS_VIOLATION` / `STATUS_STACK_BUFFER_OVERRUN` rustc crashes under `-Z threads=0`.

## Final Evidence (2026-11-24, wasm `e68c95d3…`, no temporary prints)

- 2D, full `🧪️browser-smoke.mjs` rc=0: `Initial FEM 2D Converged · 36 / 512 · Residual 7.460e-9`, then after editing n1 X to 1.5 and clicking away, `Updated FEM 2D Converged · 49 / 512 · Residual 9.515e-9`, and the Live Load reaction is `Ty: 12000 N`.
- 3D, headless, no interaction: Preparing analysis (24.6 s), Solving loads (37.5 s), results `Case: dead` (44.3 s), 0 faults.

## Changed Files

- `🧰️framework/…/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs`: `plugin_drive_pending_effects` admits each pending effect with its owning instance.
- `🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs`: passes `push_admitted_effect`.
- `🧰️framework/…/🔌️plugin/⚛️reactor/🧪️tests/🔬️command-ingress-terminal/🦀️.rs`: law `pending_effects_are_admitted_under_their_owning_instance`.
- `🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🧭️owner-registry/🦀️.rs`: `ensure_for_roster`.
- `🧰️framework/…/🔌️plugin/🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs`: `pending_effects` ensures roster partitions before preparation.
- `🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🎟️capture/🧪️tests/🦀️.rs`: law `roster_partitions_open_for_every_unaddressed_window`.
- `🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts`: `publishSpawnedJobTerminal` repaints a job's final frame after its completion turn.
- FEM 2D `✏️editor/📌️panels/🔍️inspection/🦀️.rs` and its `🧪️tests/🔬️unit/🦀️.rs`: `control_row` names the inline control after its row label.
- FEM 2D session (`🏗️model-build-job`, `🎼️eigen-stages`, `🔁️mounted-state-step`, `🧪️tests/🎼️eigen`, `🧪️tests/🔬️unit`) and FEM 3D `💼️mounted-state`: child operation from the host-bound operation and generation.
- `🧰️framework/…/🔌️plugin/🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs`: `settle_press` drives the tool follow when the Commit waits for its writer (fix C; a peer later folded it into `ToolSettlementUnit` with shortfall reporting).
- `🧰️framework/…/🔌️plugin/🛠️tool-machine/🧵️settlement/🦀️.rs`: `OriginalPressSettlement::awaits_writer`.
- `🧰️framework/…/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧾️source-rejection-anchors-and-host-lane/🦀️.rs`: law `a_press_commit_blocked_by_the_follow_read_drives_the_follow`.
- Temporary `[DEBUG]` prints removed again from the 2D job bridge, cleanup turns, maintenance ladder, 3D session API/mounted job/results, TS turn drive/UI intake/validation graph and the scrub writer.
- `☀️20/GET-DEV-FEM-2D-WORKING-END-TO-END/🧪️browser-smoke.mjs`: closes the Artifact panel only if it opened, then reopens it after the edit; the Results click moved off the new Actions overlay.
- FEM 2D `✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml`: dev-dependency `semio-plugin` with `artifact-app-testing`.
- Ticket scripts: `a2d-inspection-test.sh` (now targets `editor_app`), `native-fem-dev.sh` (dynamic Vite, `RUST_MIN_STACK`), `wasm-retry.sh` (new), `law-test.sh` (new), `inspect-node-probe.mjs` (waits for the Demo combobox, `--click`, 120 s edit poll).

## Tests

- Law `a_press_commit_blocked_by_the_follow_read_drives_the_follow` in `semio-plugin-tests-01` natively (`law-test.sh`): 1 passed.
- FEM 2D inspection laws, including the `control_row` accessible name, in the editor crate's `editor_app` host (`a2d-inspection-test.sh`): 16 passed. The panels crate's own results unit tests could not compile, because its dev-dependencies lacked `semio-plugin`'s `artifact-app-testing` feature (`artifact_app_laws`). Added.

- 2D session suite 21/21 and 3D numerical child 3/3 natively, after the child-operation fix.
- `semio-plugin-window-config-original --lib`: 10 of 11 fail in the current tree, including the pre-existing capture law, with `native Window partition did not initialize`. Blocked by the peer value-protocol work.
- The reactor source law runs in the `semio-plugin` facade test host and was not built natively in this pass.
- The published browser module was rebuilt clean on 2026-11-24 (wasm sha256 `19d1b253…`, without the temporary prints).
