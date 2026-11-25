# WGPU blank paint (2026-11-24) - in progress

## Root cause of the blank canvas
- Not a stuck dispatch loop and not a 1x1 surface (canvas is 1280x800 in a real-GPU Chromium). The frame Worker FAULTED at frame 32 and tried to close; the close never completes (so the fault never reaches the page and no banner/console error appears), leaving a hot `closeOriginalFrame` loop (~50k `original-stock-request`/s, CPU profile `wgpu-paint-worker-profile.mjs`).
- The fault (decoded from the retained failure pack): `invariantViolated: resize lost its empty-outcome worker` from `MountedSurfaceResizeLane::drive_one` (`📐️surface-lane/🦀️.rs`). After `pump_one_for_interactive_turn` the checked-out worker owns a pending physical step receipt; every other caller takes it (`take_checked_out_retained_step_receipt`) but the resize lane did not, so `resume()` was refused.
- Fix: take and validate (`fits(issued)`) the receipt right after the pump; source pin added to `📐️surface-lane/🧪️tests/🟦️.ts`.

## Console errors
- (a) module MIME error: the unbundled `new Worker(new URL("../../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url))` of `🗂️local-catalog` resolves to `/🏪️store/👷️worker/🟦️.ts`, which the wgpu serve answered with the SPA HTML. Fix: `🌐️server/🟦️.ts` aliases that route to the workspace worker source (Vite transforms it) and allows the workspace in `fs.allow`.
- (b) `/🧩️extension-modules/install` 404 (swallowed by the route boundary): `🌐️server/🟦️.ts` now mounts `semioExtensionStoreVitePlugin` like the React/play serves.
- (c) `load-document` dropped: `🎭️actor/🖼️wire-turn/🟦️.ts` maps it to `{loadDocument:{pack,spr}}` (same as React); fixture row added in `🧪️tests/🧫️fixtures/📨️effect-wire-routes`.
- Also: wasm-bindgen init now called with `{ module_or_path }` (deprecation warning).

## Tests
- vitest in-source wire-turn (35 pass), config `wgpu-paint-vitest.config.ts`.
- `🧊️browser-serving` test extended (extension inventory 200 `[]`, store worker route serves JS).

## Second and third root causes (frame build), and what is still open
- `WorkerJobSessionAdmission::advance` (`🧰️framework/🔨️modules/🧵️job/👷️worker/🚪️session-admission/🦀️.rs`) stayed at terminal stage 9 after the first session birth/close, so the reusable `FrameBuildHandle` never admitted a second frame build: exactly one build ever ran and the rest of the session was silent. Fixed: a terminal, output-free admission restarts at stage 0 (verified: builds now cycle, 60+ admitted per 35 s).
- STILL OPEN (why nothing paints): every build ends after its first step. `ActiveFrameBuild::step_turn` (`🧵️frame-job/🦀️.rs`) answers a pending phase with `Ok(None)` (no outcome descriptor), but `FrameBuildHandle::poll_runtime_on_original_caller` reads descriptor kind `None` as non-Yield and closes the session (`frame-build outcome kind=None failure=None continuation=None`, 54/54 builds). Renderer viewport stays 0x0, frame-latency observations 0, no window registered. Two experiments: (a) treating `None` as Yield, (b) emitting `admit_yield` for pending phases plus `borrow_outcome` for Yield, both end in `original-frame.native-authority-refused` (native receipt refusal) on the first resume, so both were reverted. The yield/resume receipt accounting of the frame job is the next thing to fix (compare the layout/preview jobs that use `admit_yield` and pay the receipt before `resume`).
- Evidence tooling: `wgpu-paint-probe.mjs` (`FAULT_TRACE=1` decodes the hidden worker fault), `wgpu-paint-worker-profile.mjs` (CDP CPU profile), `wgpu-paint-stats.mjs` (per-window dumps), `wgpu-paint-vitest.config.ts`.
- Not run: native Rust tests (disk), no Rust unit test for the admission restart (source-pin only for the surface lane).

## Yield/resume accounting of the frame job (2026-11-24/25)

Root cause chain, each found by running the real-GPU Chromium and fixed in source:
1. `ActiveFrameBuild::step_turn` answered a pending phase with `Ok(None)`; the handle read "no outcome" as not-a-yield and closed the session. Fix (`🧵️frame-job/🦀️.rs`): pending and completed phases `admit_yield`, `borrow_outcome` accepts Yield, the handle resumes on Yield (and on a missing outcome, like the deadline phase).
2. The caller's "already posted" receipt check ignored the return span an outcome reserves on the caller (`reserved_return_progress`): receipt + reserved delta == posted. Shared helper `posted_receipt` (`🧾️frame-build/📋️frame-preparation/🦀️.rs`), used by the handle, the deadline phase and the preparation step.
3. `FrameTransaction::step` compared the lent business generation with the presentation generation: every build was superseded at its first transaction step. New `business_generation` + `with_business_generation` (`🧮️frame-transaction`).
4. The scene camera storage refuses a receiver wider than its installed bootstrap authority; the frame turn grant is wider. `narrow_to_scene_camera_authority` (`📷️scene-camera-dispatch`) narrows it in the SceneCamera phase.
5. `PreparedRenderJob` compared its input's presentation generation with the business generation: `bind_business_generation` / `generation_is_stale`; the preparation reports `fault_detail()` (new `BatchJobSession::checked_out_job`).
6. One unit per host tick made prepare take >11k ticks: `poll_runtime_on_original_caller` now loops over `poll_runtime_unit` within the caller's time slice (half the interactive ceiling) and 256 units, with no per-iteration allocation (borrowed runtime and inputs).

Tests (`🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs`): `a_frame_build_survives_its_first_yield_and_resumes_the_same_session_for_further_steps` (red with the receipt refusal before fix 2, green after; run gated with `wgpu-frame-job-test.sh`) and `a_pending_frame_phase_admits_a_yield_that_the_job_borrows_back_and_acknowledges` (not yet run). The full 23-test suite was not re-run.

State at stop: with all six fixes the capped 60 s probe (`wgpu-paint-capped-probe.mjs`, peak RSS ~1.5 GB) reaches dock plan, `ui-doc ingress` for both windows and `prepared world mesh residency restored`, then the worker ends with `original cleanup retains its whole child failure` (page error `original-frame.native-authority-refused`). The primary child failure that this cleanup refusal hides is still unidentified; the instrumented build (tick semantic failure print in `🌐️browser-worker`) was killed (SIGTERM) twice before finishing, so the temporary logs were removed. Panels are not painted yet (`🗑️generated/wgpu-paint/capped4.png`, trace14 tags). The served wasm (`.cache/g3d-native/wasm-dev`, 23:06) still contains `[DEBUG]` authority-refusal logs from the browser worker; rebuild via `wgpu-native-wasm-build.sh` (now `repo cargo`).

## Seventh root cause and paint verified (2026-11-25)

- The hidden primary behind `original cleanup retains its whole child failure` (temporary `[DEBUG] close primary` print of `original_cleanup_refusal()` in `closeOriginalFrame`, removed again): `InvalidValue: presented roster initiating caller changed`.
- Cause: `OriginalPresentedRoster` (`🎬️media-slots/🎟️collect/🦀️.rs`) pinned its caller as `(operation, generation, clock, deadline_us)`; the roster is retained across yielded turns and every turn has a new deadline, so its first resume failed. The sibling media-slot caller check pins only operation, generation and clock. Fix: the identity drops `deadline_us` (type, set, check).
- Regression: source pin in `🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts` ("presented roster pins its initiating caller ... but not by the per-turn deadline"), green via `bun test`.
- Result with the clean wasm (02:16): capped 60 s probe (`🗑️generated/wgpu-paint/final.png`, `final-console.txt`, peak RSS 1212 MB) paints the chrome, the Flow editor panel with its nodes and the 3D preview grid; no `pageerror`, no `original-frame.native-authority-refused`.
- Remaining console noise: `console.error` lines `[DEBUG] overlay step / follow phase / settle op` (11 lines) plus 4 `wireEffectToFriendly: unmapped effect "publish-event"` warnings. The `[DEBUG]` lines come from STALE plugin component wasm (`.cache/b/p/m/browser/prototype/{modules,components}`, flow 20:23, procedural 17:04) built from sources whose logs were removed since; the renderer wasm contains none of them. A component rebuild (`repo run ...:component`, heavy) is needed to drop them; not run because of the memory rules. Remaining `[DEBUG]` strings that exist in source (return ledger, `close_native`, `ptr window`, node-graph fit, `unsettled clause`) belong to other tickets and were not touched.

## Lazy activation wiring and component restage (2026-11-25, later)

- `wireEffectToFriendly` (`🎭️actor/🖼️wire-turn/🟦️.ts`) maps `publish-event` to `{publishEvent:{topic,payload:number[]}}`; fixture row `publish-event-maps-to-the-host-event-publication`; wire-turn vitest 36/36.
- After the React lane restaged flow (03:35) and procedural (04:09) and the `:support` shard-worker rebuild, `wgpu-native-activate.ts` published the receipt (flow, brep, math, procedural, raster). Capped probe `final3` (screenshot `🗑️generated/wgpu-paint/final3.png`): the frame Worker panics at boot, `OwnershipLimit: original return escrow table has no free slot for a new receiving scope` (procedural actor), `shell-boot 86%`. Owned by the dedicated escrow agent; geometry probe on hold until the coordinator announces the rebuilt components.
- Remaining `[DEBUG]` console lines belong to their owners: `follow unit`, `receiving quote`, `gen-preview` in `🌊️flow/🖥️host/⏱️eval-session/🔭️preview-tessellation-cancellation-and-status/🦀️.rs`, `🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs` and `🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs` (all under `🧰️framework/🛍️products/💻️os/🔨️modules/`).
- Frame-job suite status: 14/23 green; diagnosing the 9 failures one at a time (gated); my two new tests used `turn()` (one work unit) where `step_turn` needs `turn_funded(1<<16)`.
