# Display-Created World Close Drain Audit

## Scope and evidence

Read-only review of Native144's failing assertion in `display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag` at Shell input test line 1596. I did not run a build or test.

The test creates `main-2` through the actual Display transfer path, publishes a retained `World3d` body, paints and seals it, closes the dock window, drains the Shell-local retired World deque, then repeatedly calls only `interpreter::close_ui_document_one()`.

Relevant source:

- [Display integration test](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1526) registers the actual World body; [its close loop](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1573) drains `retired_world3d_states` before the failing loop at [1590](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1590).
- [Shell's local World retirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23137) owns only `retired_world3d_states`. It is distinct from the UI component retirement below.
- [Interpreter's UI close step](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:346) first advances one `SceneSurfaceRetirement`; when `Ui::close_surface_one` emits `RetiredScene`, it creates that retirement at [390](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:390).
- Every removed `World3d`, including a World with no EngineCanvas token, receives an external close target by [the World3d clause](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1401). Its retirement starts a bridge request and cannot continue until terminal and acknowledged at [494](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:494).

## Immediate Native144 cause: incomplete fixture lane

The failing loop drives no owner which can transition the component-close bridge from `Pending` to `Terminal`.

`request_component_surface_close` records a pending bridge request at [OsHost 265–281](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:265). The only production driver is [`OsHost::advance_component_surface_close`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:694), which creates a `ComponentSurfaceCloseOwner`, closes its asset/World/CPU/GPU/raster phases, and publishes terminal at [708](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:708). The ordinary native redraw invokes it before frame admission at [winit 248](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:248).

The test instead calls only the Interpreter at [1594](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1594). Thus the bridge remains pending indefinitely; this directly explains line 1596 without involving the test's `ShellChromeChildCursor` or a retained document lease.

Do **not** insert the generic CPU-only close helper. [That helper](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:38) explicitly accepts only TiledMap, NodeGraph, and Board2d, then fabricates the terminal bridge. It rejects World3d by design because a live World may own an asset/decoder/World retirement. The test has already proven its local World is absent and terminal (`world3d_states` no longer contains `world_host`; local retired deque is empty), but that does not make the un-driven external bridge terminal.

### Narrow fixture repair

Keep the Display test's functional assertions through its real seal/ACK, hit, drag, and old-local-World retirement. Replace the final direct-Interpreter-only drain with a test-only exact World-close handoff, or move this final drain assertion to the existing OsHost World-close suite.

The narrower choice is a dedicated test helper whose contract is limited to an **already locally retired** World:

1. Take only the pending bridge request.
2. Require `owner.kind == World3d`, `owner.host_id == world_host`, `request.engine_token == None`, no local `world3d_states` token for the host, and an empty local `retired_world3d_states` deque.
3. Publish terminal once, then resume `close_ui_document_one` until it consumes the exact terminal acknowledgement.

This is not a substitute for World resource-close testing. The real World close owner already has focused coverage in [the OsHost standalone component close fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:286), which exercises a `ComponentSurfaceCloseOwner` with a live `RuntimeMailbox`, checked-out asset, and sibling World. The Display test should remain an integration proof that its own local World reached terminal before handing over the independently-owned bridge.

## Separate production liveness defect: a closed wait is reachable

The fixture omission is real, but source establishes a second concern that the fixture cannot validate.

1. A worker frame begins document close in [`ShellState::render_chrome_step`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22711). Once the UI close remains pending, that same frame returns `false` at [22714–22715](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22714).
2. The scene close then starts its external request and waits as described above.
3. That unfinished worker is a live `FrameBuildHandle` session; [`has_live_session`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:699) is true while its session exists.
4. The OsHost refuses to *take* a pending external close if either a presented packet **or a live frame session** exists at [694–700](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:694).

For a World retirement whose own Chrome frame is waiting for this bridge, the conditions form a closed wait: the frame needs bridge terminal; the bridge refuses to start while that frame remains live. The browser continuation keeps requesting turns while either owner exists ([browser worker 368–375](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:368)), but no current branch breaks this predicate cycle. I did not run a runtime reproducer; the reachability conclusion is from the cited normal control flow.

The existing `component_close_external_wait_does_not_stop_unrelated_frame_publication` assertion is not sufficient: it only checks source ordering and explicitly requires the blanket live-frame admission fence at [async-boundary test 2064–2084](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs:2064). It does not construct a Chrome frame that has itself requested external World close.

### Bounded owner-level repair

Preserve the presenter fence: a component must never detach while a prepared packet can still present it. Replace the blanket frame-session fence with a **narrow handoff state** owned by `FrameBuildHandle`/the Chrome cursor:

- The cursor may advertise `AwaitingComponentSurfaceClose` only after it has started the exact bridge, has no sealed input candidate or prepared engine packet, and is parked in `CloseDocument` for that same request.
- `OsHost::advance_component_surface_close` may take and advance one close unit while that exact state is live. It still refuses `presenter.has_pending_presentation()` and all other live frame states.
- The Runtime mailbox lock remains the serialization boundary. A temporary `try_lock` failure returns nonterminal work and schedules `RESOURCE_READY`; it must not fault or terminal-ack the bridge.
- Once terminal is published, the existing worker continuation resumes the same Chrome cursor, observes terminal, acknowledges it in `SceneSurfaceRetirement`, and only then finishes UI close. A close must not synthesize input or acknowledge a new generation.

This avoids an unsafe global “close while any frame is live” change while breaking the exact owner cycle.

### Required fail-first behavioral law

Add a native OsHost/FrameBuild integration fixture with two surface hosts:

1. A displayed World A has an accepted component identity and starts document close from the Chrome `CloseDocument` phase.
2. The same frame creates the bridge and is parked in `AwaitingComponentSurfaceClose`; assert there is no sealed input candidate or presentable packet for A.
3. A host turn advances exactly one external World close step while that frame remains live, then schedules its wake. The close is still nonterminal after the first unit.
4. A live unrelated World B can complete/publish while A is closing; A cannot publish or receive pointer input.
5. Once A's external close reaches terminal, the exact Chrome frame consumes its acknowledgement and removes the UI surface. Reusing A's host/document token beforehand must be refused.
6. A control row with `presenter.has_pending_presentation()` must keep the bridge pending until that packet completes or is safely retired.

## Conclusion

Native144 line 1596 is immediately blocked by omitted external-close progression in the test. Fixing that fixture must use a World-specific already-retired handoff or move the terminal assertion to the OsHost World suite; it must not reuse the CPU-only helper. Independently, the current native/browser source has a normal Chrome-close-to-OsHost admission cycle that needs the narrow `AwaitingComponentSurfaceClose` handoff and an actual cross-owner law before this close path can be considered live-safe.

## Follow-up review: World fixture and reusable frame handoff

The fixture was subsequently corrected. [`finish_world_fixture_component_close`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:631) takes the actual bridge request, requires `World3d` with no EngineCanvas token, places the same retained Shell in [`runtime_with_shell`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:91), and drives the real [`ComponentSurfaceCloseOwner::close_asset_world_step`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:356). It asserts the owner terminal before publishing its bridge terminal and returns the same Shell to the Display test. This replaces the prior missing owner step without fabricating a World terminal. Its bounded-loop limit is test harness only; production must continue one owner unit per host opportunity.

Do **not** use [`FrameBuildHandle::close_step`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:703) for the production handoff. That method irreversibly sets `closing`; both native and wasm [`poll_runtime_and_resubmit`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:505) then refuse every future build. The correct owner is a distinct, reusable `retire_for_component_close_step` on `FrameBuildHandle`, patterned after generation supersession:

1. Admit only when a component bridge is occupied and the active transaction is the Chrome `CloseDocument` request before seal. [Frame construction](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:16104) seals an input witness only after Shell returns true; [the close phase](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22719) returns false while close remains pending. A presenter-owned packet remains a refusal.
2. Cancel and retire one session unit using the existing cancellation ladder. [`ActiveFrameBuild::retire_cancelled_phase`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:230) first returns any staged candidate; [the frame cursor close ladder](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13343) retires its resources and requires `input_candidate == None`.
3. Clear only the terminal session. On native, also reset `last_submitted_generation` to `None`: native admission otherwise requires a changed generation ([lines 585–589](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:585)), whereas this close can complete without a new input event. Do not clear the installed completion waker; it belongs to the reusable host.
4. Advance the external close on later host opportunities through the existing fence. The browser already retains a continuation while a bridge is occupied ([browser worker 368–389](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:368)); native already invalidates `RESOURCE_READY` whenever close work remains ([winit 248–250](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:248)). Neither requires a second wake protocol.
5. Keep the existing presenter fence in [`advance_component_surface_close`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:694). No new frame is admitted until the bridge becomes terminal and the normal scene retirement acknowledges it.

The follow-up law should prove: a Chrome frame that requests World close is transiently retired before seal; exactly one external close unit progresses while another presented packet remains protected; the bridge terminal is acknowledged by the normal UI retirement; and native re-admits a same-generation frame after its handoff.
