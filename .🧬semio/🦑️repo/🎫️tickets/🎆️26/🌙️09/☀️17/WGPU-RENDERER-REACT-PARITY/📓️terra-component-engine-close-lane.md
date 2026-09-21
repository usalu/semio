# Per-Component Engine CPU/GPU Close Lane

Read-only source audit for the retained component-host rollout and external close lane. No build or runtime probe was run.

## Decision

Use one exact, token-bearing external close owner per presented retired component. It must complete four owned resource classes before UI and scene retirement acknowledge that host:

1. an exact staged EngineCanvas packet, if present;
2. the EngineCanvas CPU token;
3. the EngineCanvasPresenter token; and
4. the shared GPU raster-table entry keyed by that component host ID.

Do not reuse the whole-host PairedEngineSurfaceClose scan as the component implementation. Its scan is appropriate only while destroying the entire OsHost.

The renderer already derives its production engine raster identity from host_id, rather than public document surface_id. The parameter name of engine_raster_key is misleading, but retained scene paint calls it with scene.host_id. This report corrects an earlier contrary suspicion.

## Current owner boundary

### Local scene retirement cannot close external engine state

SceneSurfaceRetirement owns its SCENE_STATE token, Canvas gesture, hover, list-transfer and pending-raster state. Its pending-raster path waits for an outstanding checkout to return, takes the exact map record, drains it incrementally, then releases its admission credit.

- [SceneSurfaceRetirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:442)
- [exact pending-raster sequence](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:475)
- [current local acknowledgement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:630)

It has no EngineSurfaceToken, RuntimeMailbox, AppPresenter, or GpuContext. Local completion therefore cannot be the final acknowledgement for an EngineCanvas host.

This matters because the ordinary retained document path stops normal rendering while close remains pending:

- [Interpreter close gate](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2449)
- [Shell CloseDocument gate](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22909)

GPU progress inside either gate would wait behind a frame build that cannot advance.

### Existing paired CPU/presenter protocol is reusable

The whole-host close owner implements the exact per-token phases needed here:

BeginCpu, BeginGpu, Cpu, Gpu, Witness, Terminal.

It distinguishes temporarily unavailable RuntimeMailbox access from a fault, checks CPU/GPU token equality, and demands both terminal witnesses:

- [phase and owner definition](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:205)
- [begin and stepping logic](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:255)
- [terminal witnesses](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:310)

Factor only those phases into PairedEngineSurfaceCloseOne. Retain operation and sequence plus the admitted token. Leave Scan and Advance in the existing whole-host owner. A component lane must never find the current host after admission, because that could select a successor.

The capability wrappers already exist:

- [RuntimeMailbox CPU begin, step, witness](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11074)
- [AppPresenter GPU begin, step, witness](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:14693)

The CPU close is nontrivial: it retires NodeGraph, Map, Editor, Paint2d raster, Board, their sync caches, and owned scalars before terminal release ([EngineCanvas CPU owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:630)). The presenter closes a candidate admission, renderer, view, texture and slot identity ([EngineCanvas presenter close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1471)).

### Host identity is already in the production GPU key

ensure_engine_surface reserves the CPU slot by host_id, and the identity returned by that slot is used by the EngineGpuCandidate. The parsed public surface_id is stored as a wire/action field:

- [host reservation and identity](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1949)
- [candidate raster-key construction](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1359)
- [scene draw uses host_id](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2975)

Two siblings with one public document address therefore do not share the production EngineCanvas GPU texture key. Rename helper arguments and direct fixture use to host_id, but retain public surface_id in emitted actions.

## Missing exact owners

### Staged EngineCanvas packet

STAGED_ENGINE_SCENES retains painted vector packets before the frame-build transfer. It is host keyed and a remove function exists only under test configuration:

- [staging registry](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:841)
- [test-only removal](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:883)

Add take_for_retirement(host_id, token), accepting only a full matching snapshot identity. Retain the taken EngineCanvasPacket in the external owner and drive packet.close_step to terminal before CPU Begin. Removing by host only can discard a successor packet.

EngineSurfaceRegistration currently does not retain the token that must cross this boundary:

- [registration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2072)

Add the exact snapshot token at original mount/paint time, or provide an admission API which takes host ID and begins the matching CPU close before any successor can mount. Do not reconstruct a token after the local state has been released.

### Shared GPU raster table

After EngineGpuBuildPhase::Stage, a candidate moves the texture and view into GpuContext through stage_engine_texture:

- [GPU staging transfer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1405)

The paired presenter close no longer owns this staged/live table entry. It only clears its candidate and slot identity. The existing raster table retires live entries via a presented keep-set and an unowned scan; its explicit modes are commit, abort, and unowned:

- [raster retirement modes](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:2398)

I found no production exact-key RasterTextureTable retirement entry point. A paired Engine token terminal witness therefore does not prove an already staged component texture is gone; it relies on a later presented keep-set omitting it.

Add GpuContext::retire_exact_engine_raster_step(host_id, expected_witness), backed by an exact-key RasterTextureTable cursor. It must:

1. match host-derived raster key and exact witness/token before taking state;
2. drain matching staged or live RasterTextureEntry through RasterTextureRetirementOwner, including texture.destroy and CPU/GPU release witnesses;
3. retire matching reservation/upload state before its entry;
4. refuse a different exact witness without touching a successor; and
5. terminally report absence from staged, live, reservation and retirement state.

The existing exact Mesh GPU retirement API shows the required bounded shape ([draw](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:687)). This avoids a whole raster-table scan and avoids unbounded clones.

## Phase integration

1. Candidate tree reconciliation keeps removed UiRetiredComponentScene records in a bounded credited candidate-retirement queue.
2. Only a presentation acknowledgement moves a removed record into the external close lane. A rejected candidate discards its candidate retirement record and leaves the presented host untouched.
3. The accepted record carries host ID, EngineSurfaceToken, raster witness, document generation and component generation. It retains the SceneSurface/UI close credit.
4. The external lane drains the exact staged packet, local SceneSurfaceRetirement, exact CPU/presenter paired owner, and exact raster-table owner.
5. The SceneSurface/UI credit releases only after all four witnesses are terminal.

This preserves the visible and input baseline until the accepted replacement is physically presented. It also prevents a delayed old close acknowledgement from releasing a new same-document successor.

## Exact host seam and wake policy

The correct cross-platform owner is OsHost::build_and_publish_snapshot. It owns RuntimeMailbox and AppPresenter, and still runs while the Shell close path blocks regular frame construction:

- [shared coordinator](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:230)
- [runtime completion share](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:248)
- [normal frame admission](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:255)

Add a fixed ComponentEngineCloseLane to OsHost. Step one owner phase after pump_pending_applies and before admit_next_frame. While it is pending, invalidate RESOURCE_READY, preserve the last accepted presentation, and do not admit a new frame which could reintroduce the retiring packet.

Native redraw already responds to RESOURCE_READY:

- [native redraw request](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:979)

Browser worker must include host.component_engine_close_pending in its existing runnable continuation predicate:

- [browser continuation predicate](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:371)

This uses the retained MessageChannel continuation without creating an input generation.

## Fail-first test packet

Add a crate-private native law through the existing Winit module test seam:

- [Winit test module seam](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:1035)

Suggested name: wgpu-component-engine-close. Use a neutral fixture with one document surface, a NodeGraph sibling A and a TiledMap sibling B, shared public surface ID, distinct component host IDs, accepted first presentation, removal of A, and a later successor for A.

Its real bridge and coordinator assertions are:

1. A leaves input/scene admission but document close remains pending.
2. Every host tick executes at most one external-owner phase.
3. B remains paintable and interactive.
4. CPU and presenter terminal witnesses precede Scene/UI acknowledgement.
5. The old exact raster key is absent before acknowledgement.
6. An old acknowledgement cannot retire A's successor.
7. RESOURCE_READY yields native redraw and browser continue_frame without another input event.

Existing EngineCanvas test helpers are CPU-only setup, not the integration oracle:

- [CPU-only helper with test-only staged removal](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:67)
- [one-fuel populated CPU retirement law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs:257)

For real GPU destruction, reuse the headless RasterTableGpuHarness pattern:

- [headless GPU harness](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-residency/🦀️.rs:63)

The close state-machine law must not skip. The headless test is a separate real-device witness for exact texture destruction. The React fixture should prove sibling preservation, accepted removal timing, distinct remount identity, and absence of stale pointer/focus delivery; it cannot prove native CPU/GPU terminality.

## Confidence

- High: SceneSurfaceRetirement cannot close EngineCanvas CPU/GPU and should retain its credit until external acknowledgement.
- High: the existing paired whole-host token protocol is the correct reusable owner.
- High: Engine CPU and production GPU raster identities already use component host ID.
- High: a staged/live GPU raster entry lacks an exact per-component retirement witness.
- Not executed: no native, browser, React, or headless-GPU test was run.
