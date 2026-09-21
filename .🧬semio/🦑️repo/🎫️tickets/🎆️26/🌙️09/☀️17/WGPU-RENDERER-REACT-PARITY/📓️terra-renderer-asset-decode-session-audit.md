# Renderer Asset Decode Session Audit

## Scope and evidence

This is a read-only source audit. No build or test was run.

The current decoder is still frame-owned. `RuntimeMailbox::pump_renderer_asset_decode_step` takes a completed response into `asset_probe: Mutex<Option<RendererAssetProbe>>` and advances it at [renderer/🦀️.rs:11307](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11307). `AppFrameTransaction::step` calls it and then drives the policy-violating deadline loop at [renderer/🦀️.rs:12939](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12939).

## Reusable retained-job protocol

Use `WorkerJobSession<RendererAssetDecodeJob>`, directly, rather than `MountedWorkerJobSession`. The latter is a UI convenience wrapper whose caller has to poll/requeue it. The portable session contract is:

1. `InteractiveJob::step` performs exactly one decoder unit and returns `StepOutcome::Yield`; Yield is explicitly non-terminal at [job/🦀️.rs:1092](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs:1092).
2. Native submits exactly one unit through `try_submit_step(pool, Lane::Maintenance)`. That method is where `J: Send` is required at [job/🦀️.rs:2879](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs:2879).
3. A returned outcome is taken by its ticket, its job-owned typed result is inspected, then its owner resumes after a pending result or begins close after a boundary. The production frame analogue is [frame-job/🦀️.rs:516](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:516).
4. Register a `Waker` before submission. `WorkerJobSubmission::run` returns authority with `put_authority`, which raises that wake, so completion does not rely on a later input or frame. The frame bridge wraps its existing host callback in `FrameCompletionWake` at [frame-job/🦀️.rs:153](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:153).
5. Cancellation calls the session `begin_close`; each host turn calls its `close_step(1, JOB_PAYLOAD_PAGE_BYTES)` until `terminal_is_empty`. `InteractiveJob` provides an optional exact blocked-close wake at [job/🦀️.rs:1159](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs:1159).

The closest complete native owner is `RendererIoNode`: fixed slot/generation, session/ticket/rejection, Waker, exact pool submission, cancellation, one-step pumping, and close at [renderer/🦀️.rs:3069](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3069). Its `pump_one` covers `Idle → Submitted → Outcome/Terminal → Closing` at [renderer/🦀️.rs:3109](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3109), and the pool result is retried only for Saturated/Contended at [renderer/🦀️.rs:3136](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:3136).

## Admission and transfer safety

A completed `WorldAssetFetchOwner` cannot be dropped or silently displaced. It owns pages and asserts terminal emptiness on drop at [world/🦀️.rs:14776](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/♾️infinite/🌍️world/🦀️.rs:14776). The existing frame pump takes that owner immediately at [renderer/🦀️.rs:11307](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11307).

`WorkerJobSessionAdmissionRejected` exposes `job()`, `begin_close`, and incremental `close_step`, but no consuming `into_job` path at [job/🦀️.rs:2478](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs:2478). A generic `job_mut` API is unnecessary for this one renderer job. Make the private job own `RefCell<Option<RendererAssetProbe>>` and give it a private `take_probe(&self)`. An admission guard owns the job exclusively because no submission occurred; a pool-rejection guard owns it exclusively after `take_rejected`. In either state the private method may take the probe, then the now-empty job and its retained parameters/fault backing close through the existing guard. `RefCell<T>` is `Send` when `T: Send`.

Required job invariant: `close_step` treats an absent probe as complete, and `terminal_is_empty` requires it absent. `step` must not manufacture a publication after the probe was taken; a defensive terminal cancellation/fault is appropriate for that unreachable call. Do not call `take_probe` while the session is Submitted or Idle.

The existing generic admission law is [retained-ownership/🦀️.rs:487](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs:487). Add a renderer law that fills `WORKER_JOB_SESSION_SLOTS`, rejects a decode job containing a sealed response, takes that exact probe from the rejected guard, verifies its response token/byte count still match, then retires both recovered probe and rejection guard one credit at a time. It proves no loss on the only admission-rejection route.

## Transfer bounds

`Mesh3dLease` and `Mesh3dWriteToken` contain only `u16/u64` fields and are `Copy` at [scene/math/🦀️.rs:657](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs:657); both are automatically `Send + Sync`. The materializer retains those tokens in `GlbMaterializeCursor` at [renderer/🦀️.rs:2195](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:2195).

The terminal mesh lease is a copyable capability, not the heap owner. The mesh pool remains authoritative; publication must still use the existing take/restore handshake (`take_ready_mesh_lease` and `restore_ready_mesh_lease`) at [renderer/🦀️.rs:2832](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:2832).

`RendererAssetProbe` itself has no current compile-time `Send` assertion. Its directly visible fields are value/Box/String/token types: page indexes at [renderer/🦀️.rs:374](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:374), GLB cursors at [renderer/🦀️.rs:1437](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:1437), and the probe at [renderer/🦀️.rs:2643](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:2643). That supports an expectation but is not a proof. Add a native compile law, for example `assert_send::<RendererAssetDecodeJob>()`; `try_submit_step` then enforces the same requirement in production. No `unsafe impl Send` is justified.

## Minimal portable lane

Use a bounded `RendererAssetDecodeLane` in `RuntimeMailbox`, separate from `asset_probe`:

- It retains `session`, `ticket`, `rejected`, and the decode cancellation token. Each job retains only its `RefCell<Option<RendererAssetProbe>>` and a typed boundary marker.
- `step` calls exactly one `RendererAssetProbe::step`; `Pending` becomes `Yield`. `Ready`, `Reject`, and `Fault` set a typed job marker and also return `Yield`; the host takes the probe while it holds `WorkerJobOutcome`, then applies/returns/closes it through the existing publish code. The job is then closed. This avoids cloning a probe or using a `StepOutcome` payload as an asset transport.
- On native, mirror `RendererIoNode::pump_one` but call `renderer_worker_pool().try_submit_step(... Lane::Maintenance)`. Register a Waker built from the existing `RuntimeMailbox.0.waker` host callback; NativeReferenceDecodeJob already wakes the same callback after a Maintenance task at [renderer/🦀️.rs:10795](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10795).
- Pump one lane transition from the native host before it returns for component close, or inside the close-progress phase. Otherwise a cancelled/checked-out probe can recreate the close deadlock recorded in [terra-component-close-asset-deadlock.md](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📓️terra-component-close-asset-deadlock.md). A close action must only call probe `begin_close/close_step`; it must not call frame transaction or acquire `AppRuntime`.
- Keep response fetch ownership in the existing browser request ABI. The private decode ABI only reports a small structured state, e.g. `{ kind: "idle" | "pending" | "published" | "cancelled" | "fault", pending: boolean, detail?: string }`; it never exposes the probe, response bytes, tokens, or session state to TypeScript.
- Browser calls one Rust session turn through `try_step_on_worker` from the already independent `runAssetDecodeTurn`; it should not go through `tick`. The current TypeScript bridge already expects `runtime.assetDecodeStep()` at [frame-worker/🟦️.ts:401](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:401), and `FrameTurnScheduler` alternates one `assetDecode` turn with a frame turn through its MessageChannel queue at [frame-turn-scheduler/🟦️.ts:96](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-turn-scheduler/🟦️.ts:96). Add the Rust `#[wasm_bindgen(js_name = assetDecodeStep)]` implementation to `BrowserRendererWorker`; it must only call the lane and JSON-encode that result.
- The current Rust browser worker has no such export in this source snapshot. The TypeScript call site is therefore an incomplete bridge until the Rust method lands. This is source evidence, not a build result.

## Fail-first laws

1. A sealed GLB probe with more than one decoder unit, no frame transaction, and an independently pumped native lane reaches Ready then publishes exactly one existing mesh lease. The frame-isolation witness remains zero.
2. With all worker-session slots retained, decode admission returns the exact sealed response to the renderer lane; token, received bytes, and page count survive; both owners retire under one-item grants.
3. Native cancellation while the session is Submitted receives its worker wake, starts probe close, and reaches terminal without a frame transaction or synthetic World completion.
4. Browser: one `assetDecode` MessageChannel turn makes one decoder unit; a concurrently pending frame alternates owner on the following turn. No input generation is emitted by either turn.
5. A Ready mesh publication that cannot acquire `AppRuntime` restores the lease to the probe and remains pending; it does not drop the lease or advance another decode unit.

## Confidence

High on the session/rejection/wake protocol and on Mesh3d token transfer. Medium on full `RendererAssetProbe: Send` until the recommended native compile assertion is compiled; no existing bound proves it.

