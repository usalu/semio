# Component World Close Versus Asset Ownership

## Decision

The current component-retirement bridge has a reachable deadlock when the exact World component has an asset owner checked out by the browser transport, the frame-owned decoder, or the native reference decoder.  This is a production ownership fault, not a test-harness concern.  The close bridge stops every ordinary frame transaction before the only current decode pump runs, while World retirement requires the same checked-out asset claim to return before it can become terminal.

The close repair therefore needs the previously identified asset-owner split now.  It must be an exact-component asset-close phase before the World state leaves `world3d_states`; invoking the existing global `RuntimeMailbox::close_renderer_asset_step` is not safe because it closes shared assets and cancels the single native HTTP cancellation root.

No commands, builds, or tests were run for this audit.

## Confirmed Browser Cycle

The sequence below is established by current source.

1. `BrowserRendererWorker::poll_asset_request` takes a World request with `RuntimeMailbox::take_renderer_asset_step`.  That method calls `take_next_world3d_asset`, which marks the exact `WorldAssetIoAuthority` claim `in_flight` and transfers its sole owner to `BrowserRendererWorker.asset_fetch`.
   
   Sources: [browser worker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:130), [runtime take](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11124), [authority take](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:15022).

2. A scene retirement requests the component bridge.  `OsHost::build_and_publish_snapshot` calls `advance_component_surface_close` first; when it returns `true`, the host invalidates `RESOURCE_READY` and returns without draining events, admitting a frame, or calling `AppFrameTransaction::step`.

   Source: [winit close gate](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:230).

3. In its first `World` phase, `ComponentSurfaceCloseOwner` calls `Shell::close_component_world_step`.  The latter removes the state from the live `world3d_states` map with `take_exact_to_retirement`, starts dynamic retirement, and retains it privately in `component_world3d_retirement`.

   Sources: [component close owner](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:355), [state transfer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23363).

4. Dynamic retirement starts `asset_io` closing.  Its first asset step cannot advance an in-flight claim: `WorldAssetIoAuthority::close_step` returns `false` before it can close or release the owner.  `world3d_dynamic_retirement_terminal_is_empty` requires `asset_io.terminal_is_empty`, so the World phase cannot complete.

   Sources: [dynamic begin](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:2255), [asset gate](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:2426), [in-flight refusal](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:15159), [terminal witness](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:2450).

5. The browser transport can observe that the request is stale and call `abortAssetResponse`, but it is now too late: `RuntimeMailbox::return_renderer_asset_owner` only finds a World authority through the live `world3d_states` token.  It returns the owner as an error, and the browser worker moves it into `asset_blocked`.  Nothing returns that owner to the component-retired authority.

   Sources: [current check](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:148), [abort failure retention](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:251), [live-only return route](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11432).

6. Browser continuation is not a remedy.  The worker deliberately requests more frames while `component_surface_close_occupied`, but each tick reaches the same early close gate before the frame transaction.  The only call to `pump_renderer_asset_decode_step` is in that transaction.

   Sources: [browser continuation predicate](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:371), [frame-owned decode pump](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12862).

This cycle needs no external latency: it starts whenever a caller has taken a World request and a component unmount arrives before handback.

## Confirmed Decoder and Native Variants

### Renderer probe

After a completed World response is taken by `take_completed_renderer_asset_step`, `RuntimeMailbox.asset_probe` owns the asset claim in flight.  `pump_renderer_asset_decode_step` is the only normal path that moves the probe through parsing/materialization, closing, and `finish_renderer_asset_owner`.  The component close phase does not call it.  Thus a close between probe admission and terminal handback has the same cycle as the browser transport.

Sources: [probe admission and progress](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11234), [probe close and handback](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11403), [component phases](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:355).

### Native reference decoder

The native reference decoder is driven from that same probe and keeps a `NativeReferenceDecodeJob` in `RuntimeMailbox.native_reference_decode`.  The full-host asset close cancels the job and waits for phase `2`, but component close never invokes that full-host method.  A scheduled job can wake the host, but wake merely retries the component close gate; it does not pump the probe, so its terminal result cannot release the World claim.

Sources: [reference decode ownership](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11592), [full-host cancellation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11472), [wake after asynchronous native fetch](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11712).

### Native fetch

Native local-file streaming checks `renderer_asset_cancelled` at every page boundary.  The per-World authority becomes cancelled only after its close begins.  However, after the component state has been removed, the async task returns through the same live-map-only `return_renderer_asset_owner` and parks failure in `native_asset_blocked`; that blocked owner is only progressed by `pump_native_asset`, another frame-transaction call.  This is a confirmed structural variant, although the native populated page handoff currently fails closed before a successful real page is installed.

Sources: [per-page cancellation check](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:14771), [blocked retention](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11678), [native pump](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11686).

For HTTP, the external await has only the process-wide `native_asset_http_cancel` child.  That proves an additional repair constraint: component close cannot call the existing full-host close to interrupt one request, because that would cancel unrelated World and shared requests.  Whether an individual HTTP await can remain blocked until its existing 15-second deadline is source-supported; a particular live HTTP hang was not exercised in this audit.

Source: [global HTTP cancellation root](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:14815).

## Safe Boundary

Add `ComponentSurfaceClosePhase::Asset` before `World`.  It owns the exact current `world3d_states` admission token for `target.host_id`, while that state remains visible to all existing asset handback APIs.

The phase must take one bounded step per host turn and end only when this exact World state has no asset claim, no matching `asset_probe`, no matching browser-owned request, no matching native fetch/block, no matching native reference job, and no staged reference lease for one of its request tokens.  It then enters the existing `World` phase, whose dynamic retirement can safely transfer the state.

The smallest coherent split is:

1. Add target-aware RuntimeMailbox methods that match `RendererAssetFetchOwner::World.surface_token` to the captured token.  They must never select `Shared` or a different World component.
2. First mark only that live World authority closing, so its own `renderer_asset_cancelled` predicate rejects additional fetch/decoder work.
3. Close a matching `asset_probe` in bounded `RendererAssetProbe::close_step` calls, then finish it against the still-live state.  For a matching reference job, cancel it and wait for its existing terminal phase before probe handback.  Discard the exact `StagedReferenceImageAuthority` entry by `WorldAssetRequestToken` before returning the reference owner.
4. Keep a matching browser/native fetch in the live state until it returns.  The browser worker must hand its exact owner back before the component World phase begins; it must not rely on `asset_blocked` as a handback route.  Native HTTP needs an exact per-request cancel child, keyed by the same World admission/request token, because the present cancellation root is global.
5. Once every checked-out owner has rejoined or been finished against the live authority, advance that authority's own close one bounded item at a time to terminal.  Only then call the existing `Shell::close_component_world_step`.

The existing `RuntimeMailbox::close_renderer_asset_step` is intentionally a full-host operation: it cancels the global native HTTP root, drains the sole `asset_probe`, native fetch block, and the shared renderer authority.  It is called by full World shutdown and browser-worker host shutdown only, not by the component bridge.  It must remain unsuitable for a component close.

Sources: [global close call sites](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11022), [browser full-host close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:402).

## Fail-First Laws

Add the primary native law beside the existing renderer async-boundary tests at:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`

Suggested law name:

`a_component_world_close_retires_an_exact_checked_out_asset_before_transferring_its_state`

It should use a real `Component::Surface(World3d)` host and the production bridge, not a source-string assertion:

1. Mount World A and reserve a GLB or reference request in A's actual `World3dState`.
2. Let the real browser worker or runtime asset path take that owner so A's `asset_io` claim is `in_flight`.
3. Request component retirement before handback, then repeatedly advance only the same host close turns that production calls.
4. Assert the close reaches terminal under a finite fixed turn budget; the World authority is terminal empty; no `BrowserRendererWorker.asset_fetch` or `asset_blocked`, runtime `asset_probe`, native block/job, or staged reference entry survives; and World B remains live and its asset request is untouched.

The RED should appear before the existing `World` phase changes: current close transfers A first and never pumps the exact owner.  A sibling World B is essential: it proves the repair does not use global asset shutdown.

Add a browser/React-neutral counterpart to the component retirement fixture used by the component-host lifetime suite:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts`

The browser assertion should model a pending `fetch` or reference decode for A, unmount A, and verify exactly A's request is aborted/released while B's request remains current.  It is an oracle for React's component cleanup semantics, not a reason to emulate React's unbounded cancellation behavior.

## Non-Blocking Cases

These cases were distinguished so the fix stays narrow.

- A World asset still resident in `asset_io` and not in flight can be retired by the existing world dynamic close path.  It does not need the bridge asset phase.
- A sealed response that has not yet been moved into `RuntimeMailbox.asset_probe` also remains in the World authority and can be retired in place.
- `has_pending_asset_decode` and browser `continue_frame` are liveness signals only.  They cannot satisfy the close because they do not bypass the host's early component-close return.

## Confidence

High for the browser checked-out owner and frame-owned probe cycles: their transfer, close ordering, and only-progress call sites are all directly present in source.  High for the native reference and native fetch structural cycles.  The maximum observed duration of a particular native HTTP wait is not established here; the global cancellation scope and lack of a component-targeted interruption are established.
