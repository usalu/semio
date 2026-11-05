# Native Selection Gestures

The current native surface publishes parametric `paintStroke` and `fillRegion` commands. The editor derives their selection from the shared config. The old note about direct `selection:null` paint publication does not describe this current route.

Added a neutral intrinsic/display-coordinate fixture and a native host test before implementation. Its expected 8×8 coverage is independently materialized through the existing image crate. The candidate owns the shared PixelSelectionJob, advances one raster scanline per grant, combines and prepares RLE in 32,768-pixel grants, and exposes no publication until completion. Candidate matching includes selected layer, target, image key, world placement and intrinsic extent. Cancellation discards the candidate. Locked visible targets are selectable while painting still refuses locked targets.

Native surface gesture/frame integration is in progress. Test verdicts are pending; no runtime acceptance is claimed.

MCP exec probe g18:13 and native suite g18:24 both terminated with `timed out awaiting tools/call after 300s`. Neither returned test diagnostics or a suite verdict. No replacement native run was launched. Source inspection uses the supported direct repo script ingress with ticket-local scripts, without sandbox escalation.

## Integrated Source

Wand now owns a bounded shared Rust flood job, initialized and visited in grants of at most 32,768 pixels. Its shared JSON schema was authored before the job and existing neutral flood fixtures now assert job equivalence, monotonic progress, cancellation and result withholding. Native wand targets the intrinsic pixels or mask and combines coverage through the same shared selection combination job as shapes. Missing image content uses the corresponding uniform transparent-pixel or opaque-mask source without allocating a whole fallback image.

Previous RLE transport is restored in 32,768-byte grants, including zero coverage spans, field-order changes and ordered-span validation. A direct serde_json oracle test reconstructs the same large descriptor across grant boundaries. Candidate publication borrows a first-party descriptor and spans; it does not build or serialize a complete JSON result. The bounded native action builder measures its key-byte requirement before writing. The previous carrier is copied once into the owned candidate at initiation; span parsing and coverage construction are incremental. Large carrier ownership-copy latency is not independently benchmarked.

Native scene phase 5 advances once and continues rendering the frame. Phase 8 paints numeric progress, a bar, Escape hint and an animated loading border. The accepted animated primitive reaches the existing AcceptedAnimationClock. Parent wired the browser `has_pending_raster_selection_work` predicate into request_frame. The production source audit traced the real frame_into_step route and found no dirty-paint bypass that would prevent this work from advancing on a newly requested frame; the unrelated frame_step test wrapper has a dirty gate and must not be used to infer production behavior.

Escape in the real asynchronous shell keyboard handler resolves the exact focused retained Paint2d host and cancels selection work before utility deactivation or generic retained consumers. The bool cancellation API only consumes Escape when work existed. Pointer cancellation and exact retired Paint2d component cleanup also discard candidates. Completed config is left intact on candidate cancellation.

## Authored Regression Coverage

- Intrinsic/display coordinate mapping against the neutral 8×8 fixture and the image crate's expected grayscale mask.
- Wand replace/add/subtract/intersect against image::GrayImage coverage, plus image-key revision invalidation.
- Shared flood fixtures through one-pixel grants, progress, withheld results and cancellation.
- Large prior RLE restoration against serde_json, including zero-coverage spans and grant boundaries.
- Native gesture submission only after completion, preserved completed config on cancellation.
- A production retained-document regression uses unchanged scene/document content across frame_into_step frames, checks progress increases, observes exactly the completed selection action, tests pending-work transitions, then cancels another candidate after release using the exact retained host.

These tests are authored and registered in existing suites. None has an execution verdict in this continuation because the native MCP call timed out and the parent's direct supported target route is blocked by FD3 EPERM before suites execute.

## Modified Files

Shared pixels under `🧰️framework/🔨️modules/🔲️pixels/✍️editing`: `🦀️.rs`, `🧬️schema/🔣️.json`, `🧪️tests/🦀️.rs`.

Surface paint under `🧰️framework/🔨️modules/🗺️surface/🎨️paint`: `🦀️.rs`, `✍️gesture/🦀️.rs`, new `🎯️selection/🦀️.rs`, new `🎯️selection/previous.rs`, `🧪️tests/🔬️unit/🦀️.rs`, new `🧪️tests/🧫️fixtures/🎯️selection/🔣️.json`.

Renderer engine under `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`:

- `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🖌️paint2d/🦀️.rs`.
- `🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🧲️marquee-and-navigator-laws/🦀️.rs`.
- `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🎨️scene-render-entry/🦀️.rs`.
- `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/⌨️keyboard/🦀️.rs`.
- `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📕️close-scene-pointer-and-dispatch/🦀️.rs`.
- `🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs`.
- `🎯️targets/🧊️wgpu/🧿️ui-engine-tests/🧰️surface-fixtures/🦀️.rs`.

Ticket input script `native-inspect.ts` is retained. Parent independently modified browser-worker scheduling and native_controls independently authored tool declarations.

## Open Verification and Limits

Compile/type checking, all native test execution and an actual native end-user walkthrough are unverified. The focused Escape hook is authored and source audited; the production regression resolves the same host and directly calls its cancellation function, rather than driving a complete Shell keyboard event. Selection coverage contour rendering and brush preview clipping are not implemented by this change. Actual committed paint commands use the config-derived selected-edit route already present in the editor; this continuation adds no second mutation pipeline. Very large RLE publication is subject to existing bounded action item limits and may refuse; a streamed selection publication protocol is not authored.
