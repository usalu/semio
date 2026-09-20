# Terra Hub and Tree Production Seam Audit

**Audit date:** 2026-09-20  
**Scope:** current production Hub and retained Tree routes against the reachable React source, plus the requested narrow first-run and browser accessibility transport checks. This was static source tracing only: no source edits, Git mutations, Cargo build, or browser run.

## Result

| Priority | Classification | Finding |
| --- | --- | --- |
| P1 | **Proven** | A retained `TreeItem` or `TreeDragHandle` never receives the pointer press/release that arms its `EventRouter` drag session in the production Shell route. Both Handle and Surface drivers therefore cannot start a retained Tree drag. |
| P1 | **Proven** | A delayed React Hub sign-out, and a delayed Hub command, can mutate the newly selected connection because connection switching does not retire those operations or guard their completions. |
| P1 | **Proven** | The current browser accessibility mirror exposes projection data but has no focus, click, input, or activation route back to the canvas/worker. An AT activation on a mirrored actionable control cannot invoke its retained control. |
| P2 | **Unverified coverage gap** | Hub projection tests establish pure map/source-string laws, not the browser transport → worker admission/retry/retirement route. |
| P2 | **Unverified runtime reach** | The new first-run contract is declared as a frame-worker source/input, but no current React shell or WGPU rendering host consumes the walkthrough component/contract. Source ownership alone does not demonstrate a user-reachable flow. |

## P1 — retained Tree drag is registered but not routed

### Evidence

The retained engine registers both row and semantic-handle targets. The row receives drag data under `Surface`; the trailing handle receives drag data under `Handle`:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:725-740`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:769-801`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:312-342`

The document interpreter copies those registrations into the Shell input registry, and applies the active driver's drag mode before rendering:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:421-430`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1420-1450`

The Shell correctly recognizes a retained-body hit and calls `route_retained_pointer_press` for both down and up:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11424-11429`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11452-11458`

That routing function dispatches a retained `PointerDown`/`PointerUp` only for `Input`, `Select`, `Toggle`, `Slider`, `NumberStepper`, `Ring`, and `IconSelect` (`:11916-11923`). It excludes `TreeItem` and `TreeDragHandle`. The other branch runs only on release and can only dispatch an explicit action (`:11924-11942`). A semantic tree handle has `action: None` (`📥️input/🦀️.rs:796-799`). Pointer moves are dispatched (`Shell/…/🦀️.rs:11876-11880`), but the router requires the earlier `PointerDown` to arm its capture/payload (`⚡️events/🦀️.rs:1125-1137`, `:1781-1785`).

### Trigger

1. Render a retained Tree item with `draggable` and payload under the **Handle** driver.
2. Press the visible `tree.drag.sort.*` or `tree.drag.transfer.*` target, then move beyond the drag threshold.
3. The Shell focuses the window but does not send `PointerDown` to the retained router. Move has no armed capture, and release has no handle action. No `DragSession`, `DropCommitted`, or `DropCancelled` results.

The same loss occurs under **Surface**: the `TreeItem` row target has drag data, but never receives the retained pointer press that begins a session.

React's Tree explicitly attaches pointer arming to a `DragHandle` when surface drag is disabled (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:587-616`) and resolves whole-row initiation for the surface driver (`:1967-1974`). The WGPU registration and paint policy mirror this division; the host event bridge breaks it.

### Why current tests do not cover production

`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs:687-765` constructs a `UiTree`, sets `NodeFlags::DRAG_SOURCE` directly, and calls `EventRouter::dispatch` directly. `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-input-unit/🦀️.rs:82-122` calls the target-registration helpers directly. Neither test reaches the production Shell hit → `route_retained_pointer_press` → `dispatch_ui_event` seam.

### Minimum acceptance case

Use the actual `render_ui_document_step`, retained registration, and Shell pointer entry path for a document containing a draggable Tree and a compatible drop target.

1. Under **Handle**, move from the label band: assert no retained drag session. Move from the semantic handle: assert a session, then assert the expected `DropCommitted` action and payload on release.
2. Under **Surface**, move from the row: assert the equivalent session/commit.
3. Change driver while that real session is active: assert exactly one `DropCancelled`, no capture, and no later stale commit.
4. Include an actionable Tree row and assert its ordinary release action still fires once; its EventRouter-owned selection/focus behavior is currently unverified because the host also skips its press event.

## P1 — React Hub operation completion is not scoped to the selected connection

### Evidence and trigger

`selectConnection` aborts sign-in, authority, and members work, but neither a command nor an outstanding sign-out; it then immediately clears local Hub state and selects the next connection:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🟦️.tsx:511-524`

`signOut` creates an untracked local `AbortController`. Its completion has no aborted/current-connection check and unconditionally dispatches `signed-out`, clears rows/invite/redemption, and sets the phase to loading:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🟦️.tsx:576-590`

`runCommand` has the same selection race: after an old command completes, it invokes its receipt callback then calls `loadSpaces()` and `loadMembers()` through current refs, which now refer to the newly selected Hub:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🟦️.tsx:440-460`

Concrete sequence: start sign-out at Hub A; delay its network response; select Hub B and begin browsing/signing in; release A's response. A's completion applies a terminal state and clears B's visible Hub state. A delayed create/archive/invite command similarly invokes its stale receipt callback and refreshes B through mutable refs.

This is a React reference correctness issue, not a claim that WGPU should replicate the unsafe result. It matters to parity because the Hub authority/UI route cannot be accepted as a reliable baseline until completion is scoped to the initiating connection/session.

### Minimum acceptance case

Use deferred Hub port promises. Start an A sign-out or command, select B, establish B rows/authority/invite state, then resolve A. Assert that A cannot change B's session phase, rows, invite/redemption state, or issue B refreshes. Repeat for an aborted A sign-in returning a normalized failure.

## Hub authority and document-status seams that are currently present

No missing route was proven in these two places:

- The React document event handler publishes a remote status into `semioWgpuHubProjection` before updating local state (`🏛️ShellHost/🟦️.tsx:3240-3248`), and `closeDocument` publishes `null` before retiring the attachment (`:6381-6450`). The browser boot host installs and removes that global bridge (`🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:106-114`).
- Hub capability changes are announced by the port (`🔗️HubConnection/🟦️.tsx:755-764`), received by `ShellHost` as `onCapability` (`🏛️ShellHost/🟦️.tsx:2653-2679`), and cause its authority effect to create/close the private session port and refresh subscription (`:3450-3563`). The footer derives presence from the verified shell authority rather than the workspace overlay (`:9042-9051`).

These paths are source-proven present. Their race and backpressure behavior is not exercised end-to-end by the current fixture tests.

## P2 — Hub projection fixture coverage stops before the transport seam

The TypeScript Hub projection test uses a fixture fold and source substring checks, including the `event.remote` and `null` bridge text:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🔗️hub-projection/🟦️.ts:25-71`

The Rust workspace test checks map replacement, retirement, and a 64-entry capacity in the Shell directly:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs:39-87`

Neither proves that `publishDocumentStatus` enters the browser transport, survives an unavailable worker admission, is coalesced/retried in order, and eventually retires the exact key.

### Minimum acceptance case

Exercise the real browser-frame transport and worker admission boundary with a controlled host result: publish an update, refuse admission once, publish a newer update and then a close, tick admission, and assert only the newest state is delivered and the close cannot be followed by a stale status resurrection. Cover capacity with 65 distinct keys and a later release/retry. This is an **unverified coverage gap**; no production failure is asserted from static review.

## First-run walkthrough reachability boundary

The new shared contract defines state-to-stage selection and an `IntroductionDefinition` (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🎓️first-run/🟦️.ts:36-58`, `:126-188`). The React component renders that definition when directly mounted (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎓️HubFirstRun/🟦️.tsx:38-52`). The current React target barrel exports it (`🎯️targets/⚛️react/🟦️.tsx:1417-1423`), but no `HubFirstRun`, `hubFirstRunShouldStartV1`, or `markHubFirstRunSeenV1` consumer exists in the current `ShellHost` or `HubWorkspace` source.

The canonical contract now appears in the WGPU browser source/input declaration (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:28673`, `:28792`). That makes source ownership and regeneration observable; it does not make the UI definition render or make its anchors/actions reachable. This audit intentionally makes no claim about the concurrently running generated artifact or native build.

**Minimal acceptance case:** mount the walkthrough from the Hub workspace once per unseen profile, prove the four live anchors resolve after each action, prove skip/done writes the seen flag, and run the same flow through the selected renderer's actual interaction route. Until a host consumes the contract, it is not a parity baseline for either renderer.

## P1 — accessibility projection is descriptive only

The current browser host is not the prior empty mirror: it pulls the worker's current accessibility projection on directives and renders role/label/value/live properties (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:128-263`, `:452-475`). The interpreter and Shell do publish retained and chrome accessibility nodes.

The concrete remaining seam is activation. Every mirrored node is a flat `div` with `tabIndex = -1` (`browser-boot/🟦️.ts:186-214`), and `accessibilityMirror` registers no focus, click, keyboard, input, or pointer listener (`:174-262`). The only DOM input listeners are on the canvas (`:328-402`), and the transport wire contains only canvas pointer/key/text events (`🚚️browser-frame-transport/🟦️.ts:95-120`, `:151-173`). There is no message carrying a mirror node/window id back to a retained `UiEvent` or an action.

Therefore a screen reader can inspect the current projection, but focus or virtual-click activation of a mirrored actionable node cannot invoke that control. Canvas keyboard input remains a separate route; it does not identify the projected mirror node.

### Minimum acceptance case

Render a retained action control and a text/value control. Obtain their mirror nodes, then exercise focus plus assistive-technology-equivalent click/Enter activation. Assert the host sends a node-addressed event, the retained router changes focus/value or emits the declared action exactly once, and the mirror refreshes the new focused/value state. Also assert ordinary canvas keyboard routing still works and does not double-dispatch.

## Validation boundary

The audit used direct source traces and existing test bodies only. No test command was run, so this document does not claim passing runtime behavior. The active source/activation work was treated as concurrent and no files outside this report were changed.
