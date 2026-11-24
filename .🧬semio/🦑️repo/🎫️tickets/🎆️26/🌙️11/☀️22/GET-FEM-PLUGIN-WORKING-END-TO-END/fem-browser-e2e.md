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
- An instrumented build (`[DEBUG] live cleanup zero progress`) is pending to name the blocking step.

## Changed Files

- `🧰️framework/…/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🧾️typed-operations/🦀️.rs`: `plugin_drive_pending_effects` admits each pending effect with its owning instance.
- `🧰️framework/…/🔌️plugin/⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs`: passes `push_admitted_effect`.
- `🧰️framework/…/🔌️plugin/⚛️reactor/🧪️tests/🔬️command-ingress-terminal/🦀️.rs`: law `pending_effects_are_admitted_under_their_owning_instance`.
- `🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🧭️owner-registry/🦀️.rs`: `ensure_for_roster`.
- `🧰️framework/…/🔌️plugin/🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs`: `pending_effects` ensures roster partitions before preparation.
- `🧰️framework/…/🔌️plugin/🪟️window/🎚️config/🎟️capture/🧪️tests/🦀️.rs`: law `roster_partitions_open_for_every_unaddressed_window`.
- FEM 2D session (`🏗️model-build-job`, `🎼️eigen-stages`, `🔁️mounted-state-step`, `🧪️tests/🎼️eigen`, `🧪️tests/🔬️unit`) and FEM 3D `💼️mounted-state`: child operation from the host-bound operation and generation.

## Tests

- 2D session suite 21/21 and 3D numerical child 3/3 natively, after the child-operation fix.
- `semio-plugin-window-config-original --lib`: 10 of 11 fail in the current tree, including the pre-existing capture law, with `native Window partition did not initialize`. Blocked by the peer value-protocol work.
- The reactor source law runs in the `semio-plugin` facade test host and was not built natively in this pass.
- The published browser module was rebuilt clean on 2026-11-24 (wasm sha256 `19d1b253…`, without the temporary prints).
