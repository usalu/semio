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
