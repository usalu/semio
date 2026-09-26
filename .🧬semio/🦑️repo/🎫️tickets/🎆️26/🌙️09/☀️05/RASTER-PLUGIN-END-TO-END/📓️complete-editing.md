# Complete Raster Editing

## Task and Tracking

Continue the existing open Raster Plugin End To End ticket for the user's 2026-09-26 goal: a complete, usable image editor with layers, selections, and algorithms. Repository MCP `repo://goals` was read before work. `ticket_reopen` reports the matching ticket is already open; work continues in that ticket. The applicable repository goal is Running Raster.

## Initial Inspection

The raster plugin exposes a typed Rust document, layer mutations, shared history, a semantic UI, and React/wgpu paint surfaces. Existing layer operations include grouping, visibility, opacity, blend modes, duplication, ordering, and adjustment metadata. The current paint surface uses a fixed 512×512 scratch buffer and hardcoded blue paint. Stroke completion discards its inverse scratch without publishing an artifact mutation. Marquee selects layers rather than image pixels. This makes persistence and the pixel editing contract essential prerequisites for further UI controls.

The existing ticket records incomplete adjustment-map codecs, child-image materialization, and previous native test failures. Those notes are historical evidence, not current test results; current behavior must be checked again.

## Intended Acceptance

- Authored schemas and language-neutral fixtures cover editing commands, validation, selections, layers, algorithms, transforms, and persistence.
- Paint and erase respect real layer dimensions, transforms, color, opacity, locks, and pixel selections; gestures cancel without partial writes and commit through history.
- Layer creation, duplication, visibility, locks, opacity, blending, ordering, merging, and deletion have accessible controls.
- Rectangle, ellipse, lasso, and contiguous color selections support replace/add/subtract/intersect and inversion.
- Image adjustments, convolution filters, flood fill, crop, resize, rotation, and flips have validated parameters and cancellable progress.
- File import, editable document save/load, flattened image export, undo/redo, zoom/pan, shortcuts, and English/German labels form a coherent end-user workflow.
- Existing framework primitives remain the integration points; no new external runtime dependency or parallel document authority.
- Run relevant Nx targets, third-party algorithm oracles, and browser runtime verification. Record exact outcomes here.

## Validation

- `@semio-tech/pixels:test-typescript`: 25 passed, 0 failed, 69 assertions (first core suite). Uses shared JSON cases, Ajv schema validation, and Sharp/libvips transform oracles.
- `@semio-tech/pixels:test-rust`: 6 passed, 0 failed, 14 filtered (first native core suite). Uses the same JSON cases and the independent `image` crate as a transform oracle.
- `@semio-tech/raster-raster-rs:check`: passed, including four prerequisite generation tasks. Native command and retained job compile.
- Later UI-model, stroke, and command tests are still pending. No browser runtime result is claimed.
- Expanded TypeScript run: 37 passed, 0 failed, 94 assertions, covering stroke schema/selection coverage, asynchronous selection cancellation, independent SVG selection oracle, and native-compositor coordinate fixtures. Additional convolution cases were added afterward and are being checked.
- Current TypeScript run: strict type checking of the pure editing/model modules passed; 42 tests passed, 0 failed, 108 assertions, including the independent libvips box-convolution oracle.
- Current native pixel run: all 23 tests passed, 0 skipped. This includes seven editing tests and the existing PNG codec suite plus two new incremental PNG tests. The new encoder is checked against the independent `png` decoder across scanline/block boundaries and rejects incomplete/cancelled output.
- Initial React development startup failed in repository bootstrap while starting the Nx daemon. An isolated-cache direct Nx invocation is being investigated.

## Implementation in Progress

Added a framework pixel-editing module in Rust and TypeScript with schema, common fixtures, bounded unpublished image candidates, validation, selection masks, image adjustments, transforms, and alpha-aware compositing. Added an authoritative raster `editPixels` retained command that rejects stale image keys and emits image/layer mutations into existing history. Added accessible English/German React controls and a pixel gesture overlay. These are integration work in progress, not a claim of feature completion.

Outstanding acceptance includes runtime publication and undo/redo, performance of large strokes/selections/encoding, native renderer parity, layer locking/merging, actual selection outlines, save/reload/export, complete UI validation, and cleanup of this session's generated outputs.

## Integration Findings

- Paint surfaces center pixel layers on their transforms. The new interaction model now includes this center offset and intrinsic-image-to-layer scaling. A shared fixture is consumed by both TypeScript gesture tests and the native scene test.
- Native surface image commands previously used the identity transform, while layer clips and picking used the world transform. The image command now uses the world transform and image scaling; runtime verification is pending.
- Selection previews now display the actual mask, constructed in cancellable batches. Pixel selections survive edits that keep the layer dimensions.
- The retained command catalog requires an exact proof/manifest/route join. `editPixels` is now included in the proof list and exhaustive command cases; native route, publication, undo, and redo tests are pending.
- Native test and WASM builds are waiting on shared Cargo compilation units while other workspace work proceeds. No shared build processes or cache state were removed.
- The native pixel rerun used a ticket-scoped Cargo build directory after the shared compilation units waited for over 19 minutes. Only this chat's superseded pixel-test process was stopped. The renderer WASM build subsequently completed.
- Image encoding now progresses in fixed 4096-byte blocks using the existing first-party fixed-Huffman compressor. The pixel command reports prepare/compute/encode stages and can cancel during encoding; publication integration is still under test.
- Raster export now centers layers like the editor and fits the complete bounds including negative coordinates. A native regression test covers an odd-width image translated into negative world coordinates; this new export regression has not run yet.
- Remaining architecture issues include replacing delete/create layer publication with a sparse pixel-content mutation, incremental image encoding/materialization, editable pixel masks, adjustment/group export parity, and moving React-only controls into shared semantic UI for native parity.

## Sparse Publication and Navigation

- Replaced the pixel command's delete/create layer sequence with a `ChangeLayerPixels` event. The event checks the expected image revision and updates only the attachment/extent, with a transform change only for geometric edits. Names, hierarchy, opacity, blend modes, and other settings are preserved. Exact nullable extent undo, wire roundtrips, retained candidate/digest/retirement handling, a JSON schema, and a language-neutral fixture are added; native validation remains pending.
- The independent Python mutation reference and vocabulary now include pixel-content replacement. Its fixture and differential scenarios are registered alongside the existing layer mutations.
- React controls now include hand navigation, middle-button/Space panning, shared pinch-camera math, keyboard tool shortcuts, and device-pixel-ratio-aware selection chrome. These new interactions have not been confirmed in the browser yet.
- Full React typechecking ran and reported a tuple assertion type error in the new coordinate fixture test plus unrelated existing errors in interpreter stories and artifact-creation tests. The tuple assertion is corrected. All 42 focused TypeScript/model tests pass again. The newly added navigation JSX still needs another typecheck.
- The native artifact check failed on eight missing `UiTreeItemNode.detail` initializers in the shared UI crate. A subsequent read showed those initializers had been repaired by concurrent work; the artifact check was restarted. No edits to that UI file were made by this task.
- The native gesture host still paints hard-coded 512×512 blue scratch buffers without publishing a pixel event. This must be replaced with stroke intent dispatch and shared tool settings for native parity. The surface parser also interprets persisted degree rotations as radians; a shared rotated-layer fixture is needed.

## Native Gesture Integration and Verification

- The six filtered native `editPixels` tests passed, including retained publication, undo, redo, malformed selections, stale revisions, and hidden ancestor refusal. The whole artifact suite is now running to cover the new semantic mutation and export regression.
- All 46 focused TypeScript tests pass (118 assertions), including three new Sharp/libvips SVG coordinate oracles. Rotated fixtures use a numeric tolerance after exposing expected floating-point roundoff at 90 degrees.
- Full React typechecking was rerun after the navigation edits. It reports no diagnostics in Paint2dHost/editing; the target still fails on concurrent shared UI, worker replication, and interpreter story errors elsewhere.
- Native RasterHost now collects bounded stroke intents in intrinsic image coordinates and produces an `editPixels` payload on release. Cancellation discards the intent without changing image buffers. The native EngineCanvas dispatches that payload and retains it until action publication succeeds. Tests cover coordinate fixtures, cancellation, eraser/style carriage, and missing target behavior. Native runtime confirmation remains pending.
- The native surface parser now converts persisted degree rotations to radians. Native stroke previews use the same layer matrix as images. Foreground/hardness setters exist on the native host but still need shared semantic tool configuration/UI carriage.
- Fixed the surface test router to await its asynchronous native test runner so failures propagate through Nx. This script fix has not completed runtime verification yet.
- The first browser development build failed after 48 minutes because the shared mutation envelope was temporarily missing `observed`/`target`. Those fields are present in current source; the dev pipeline has been restarted. No browser server or end-user runtime has been confirmed yet.

## Latest Verification Checkpoint

- `@semio-tech/raster-raster-rs:check` now passes with the sparse pixel mutation and centered export changes.
- The first native surface test compile failed because the repository's first-party `Affine` intentionally does not expose `determinant`/`inverse`. The gesture now computes its checked inverse from the six public coefficients and retains its forward matrix for previews. The paint surface suite is running again with the corrected API use.
- Full artifact tests, native paint tests, and the restarted browser development build are still running. The browser runtime is not yet available.
- Subsequent work must connect foreground/hardness and pixel tool state to shared semantic UI/config, finish native selection/filter parity, address real pixel masks and layer locks/merge, fix group/adjustment export parity, replace complex-selection payload limits, validate rapid consecutive strokes and native-job completion/error UI, and confirm save/reload/export in the real application.

### Shared Brush Style and Selection Follow-Through

- Added schema-first foreground RGB and hardness to RasterConfig, corresponding typed config commands, retained publication contracts, and both language mirrors. Paint scenes carry these fields to React and native hosts. The native brush options expose the current color plus a palette and hardness, with resolved English/German labels; React color picker and eyedropper now dispatch shared session commands.
- Added language-neutral brush vectors, Rust config/codec/JSON-oracle tests, and TypeScript JSON Schema + Sharp/libvips tests. `@semio-tech/raster-js:test` passed: 13 tests, 42 assertions. New native style tests remain pending with the broader native suite.
- Preview retry caught an incorrect new Rust module path; corrected it. The active retry is `raster-dev-style.log`; no real browser runtime is confirmed yet.
- Empty native pixel layers previously materialized opaque checkerboard pixels, hiding lower artwork. They now use a one-pixel transparent display placeholder and allocate no authoritative paint buffer. Added a shared empty-layer fixture and image-crate oracle test; native execution pending. A dedicated transparency checker background still needs finishing.
- Found the former renderer interaction gap has an available `render_with_request_context` hook. Wired live framework selection and pointer hover into composite/navigator scenes. Layer tree rows now use exact layer IDs, matching canvas selection IDs and preserving IDs containing periods. Updated tree-window assertions and added a shared selection fixture/native scene test. These changes await native and runtime verification.
- Latest React typecheck completed with no diagnostics in Paint2dHost or the editing module, but the overall target remains blocked by shared framework diagnostics outside this change. Full raster test, native surface paint test, and preview builds remain heavily contended in shared Cargo build queues.

The goal remains active. No feature-complete claim is justified: real pixel masks, non-destructive adjustment rendering/export parity, grouping compositing, comprehensive user-facing operation completion/cancellation, and runtime workflows still need work.

### Selection Inspector

Replaced the schema-only inspector with localized semantic name, visibility, opacity, blend, position, display width/height and foreground color inputs. Multi-selection shows mixed values and dispatches one `patchLayers` command. Shared selection fixtures and native UI projection tests cover empty, single, and multiple selection in English/German. Fixed numeric/boolean UI values crossing the text-valued patch command boundary and preserved numeric-looking names as text. Removed duplicate patch implementation from `patch-layers`; it uses the adjacent single-layer implementation. React no longer keeps a competing chosen-layer state. These changes are implemented but native execution and browser runtime remain pending.

Verification update: the new inspector initially failed compile because `ui_node_list` requires result items and the JSON writer accepts JsonValue rather than DslValue. Corrected both bindings. `@semio-tech/raster-raster-rs:check` now passes (`raster-inspector-check-2.log`). Targeted diff whitespace checks pass. Native test suites still running; new preview retry is `raster-dev-inspector.log`. No browser runtime claim yet.

Next verification handoff: native artifact suite remains in session 46016 (`raster-full-suite.log`); native paint suite remains in session 82555 (`raster-surface-paint-suite.log`); newest React preview is session 60547 (`raster-dev-inspector.log`). Session 64977 completed the native artifact check successfully. Style TypeScript tests passed in `raster-brush-style-ts-2.log`; React full typecheck completed with unrelated framework failures in `raster-style-typecheck.log`. Newly added Rust tests have not yet been executed. The tree-row identity change also required updating the formatted group drop target in the existing mounted move-layer test.

Remaining layer audit: existing `moveLayer` still treats a group target as insertion into that group and places ordinary before/after targets at stack ends. Proper sibling-relative before/after and explicit inside semantics need shared fixtures and a clean command implementation. Native checker background, actual masks/adjustment compositing, and operation completion/rapid-stroke behavior remain unverified or incomplete.

Preview infrastructure follow-up: `raster-dev-inspector.log` failed because `.vscode/🧩️launch.seed.jsonc` had a security launcher inserted inside the preceding German user-path launcher. Preserved both entries and repaired their structural boundary; Bun JSONC parsing now succeeds. Registered the raster and pixel test launchers in the authoritative seed (the generated `.vscode/launch.json` is regenerated by the registry task). Latest preview retry: session 11892, log `raster-dev-launch-fixed.log`. The older failed preview session 60547 may still be draining dependent tasks; it should not be used as the active preview. Native suites 46016 and 82555 still have no completion result.

### Layer Drop Semantics and Browser Validation — 2026-09-26

- Replaced ad hoc root-index drag handling with sibling-relative before/after/inside destinations. Same-container indices account for source removal; no-op/self drops emit nothing; missing targets, non-group inside drops, invalid positions and ancestor cycles refuse the command.
- Added command JSON Schema, language-neutral nested-tree vectors and native tests using independent serde JSON traversal. The mounted group-drop case now requests `inside` explicitly. Nx native `move_layer` run passed 15 tests (227 excluded); full native rerun is pending.
- The earlier full suite exposed integer-versus-floating JSON representation in the pixel-content preservation oracle. Numeric normalization now compares equivalent JSON values. This is a test-only fix; full rerun is pending.
- Native paint suite failed the old mask geometry assertion (10 passed, 1 failed, 49 not run). The mask implementation currently draws a fabricated white image instead of masking pixels. This requires a substantive implementation and export-parity fix; it is not considered complete.
- React preview launched at port 6060. Through actual browser controls, selecting Backdrop populated the new inspector; a blue brush stroke committed and appeared in both composite and navigator. Browser console inspection showed no paint exception, while the dev server reports an unrelated trusted-catalog proxy connection refusal.
- Repeated preview document reloads reset the example state between interactions. The reload source remains unconfirmed. Consequently persistence, filter results and undo are NOT yet verified.
- UI exercise identified unconditional blur cancellation of in-flight selection preparation when moving focus to tools. Added a mounted React regression with a shared rectangular-selection fixture and Sharp SVG alpha oracle; harness setup and red/green execution are in progress.

- The mounted selection-focus regression now passes (1 test, Sharp SVG alpha comparison); it failed before the blur fix. The fixture uses 64×64 pixels, enough to span multiple async selection grants without making the test depend on large-image timing. No temporary debug logs remain.
- Tool blur now cancels only an active gesture, preserving completed-gesture selection preparation. Web brush size limits now agree with shared config (1–2048).
- Browser follow-up confirmed a 50×50 rectangular invert operation confined to its selection, visible in composite and navigator after the task completed. The overlay retains its selection while focus moves to adjustments. A later history attempt was interrupted by an example reset/empty projection; undo correctness remains unverified in the browser.
- Updated the semantic/round-trip mutation catalog to include ChangeLayerPixels. Full native Nx run passed **242 tests, 0 skipped**, native execution 5.811 seconds (`raster-full-suite-3.log`). Earlier numeric normalization and all current inspector/config/selection/patch/drop changes are covered by that successful run.
- Restarting only this task's preview processes with `SEMIO_VITE_HMR=0`; shared builds and unrelated processes are preserved. Goal and ticket remain open. Mask compositing, native advanced tools, command completion/cancellation, rapid strokes, preview, save/reload/export and comprehensive UI parity remain unfinished.
