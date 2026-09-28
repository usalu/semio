# WGPU Browser Media Host Parity Audit 39

Read-only source audit on 2026-09-28. No production or test source was changed and no WGPU build was run.

## Finding

The smallest honest parity design is a browser-page DOM overlay whose slot list is published with the same GPU-accepted frame witness as WGPU input geometry, plus a request/reply lane to the frame worker for the existing resumable media export protocol. Native WGPU remains a bounded, localized unsupported presentation and never starts an export because the repository has no native decoder/player boundary.

One current blocker precedes that design: WGPU rejects the canonical ready packet. The canonical resource now has seven exact fields—`kind`, `controllerId`, `appInstanceId`, `parentDocumentId`, `outputPort`, `revision`, and `generation`—in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts:31-39,66,97-125` and its JSON Schema. The independent Rust validator still admits only the old four fields at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:988-997`. The shared reservation fixture is stale in the same way: `ready-source-remains-unsupported-en` has only `kind`, `controllerId`, `outputPort`, and `revision` in `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎬️media-transport-reservation/🔣️.json`. Consequently, a newly canonical ready packet reaches `media_transport_host_status` as invalid and produces “Invalid media transport contract” rather than the localized unsupported status or a future live browser host.

The first patch must update the fixture and Rust validator to the schema-owned seven-field resource and add missing/extra/invalid-owner vectors. Longer term, the JSON Schema and language-agnostic fixture should be the one authority; the TS parser and Rust validator should both be held against them so this drift cannot recur.

## Existing contract and reusable export seam

The contract already carries every media-host datum required by the proposed route:

- Exact resource authority is `(controllerId, appInstanceId, parentDocumentId, outputPort, revision, generation)`. `revision` and `generation` are decimal u64 strings, avoiding loss through JSON numbers.
- `kind`, `mediaType`, duration/position/selection, explicit `en|de` labels, capability status/reason, and bounded `hostContentHeight` are validated by `parseMediaTransportProps` at `🎬️media/🟦️.ts:97-125`.
- Wire authority is `MediaExportHandle { app_instance_id, parent_document_id, operation_id, base_revision, generation }`; status includes exact `mime_type` and `total_bytes` at `🧰️framework/🛍️products/💻️os/🟦️.ts:2635-2644`. `AppChannelClient` already supplies submit, poll, cancel, and take-chunk at `:4176-4194`.
- The MIME defect is resolved: `ArtifactMediaExportResult` owns a separate bounded `mime_type: String` and completion publishes it (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15219-15235,41286-41295`). `media_type` remains the semantic media class/form.
- The producer contract is 4,096 bytes per chunk and 33,554,432 bytes total (`🔌️plugin/🦀️.rs:15034-15047`).

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎬️MediaTransportHost/🚚️lifecycle/🟦️.ts` is already React-free despite living under the React element. `MediaTransportPort` (`:11-16`) and `collectMediaTransportBytes` (`:58-107`) serialize submit/poll/cancel/take, validate every returned handle, require exact MIME and length, enforce segmented-download caps, yield between bounded steps, and drain a completed operation even when its bytes are refused. Move this file to a renderer-neutral media transport module and import it from both React and WGPU. Do not copy this state machine.

`PluginRuntime` currently repeats the `AppChannelClient` adapter in its large component at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:3850-3862,3966-3985`. Extract its `mediaExportReply`, authority checks, and four methods into the same neutral module. The WGPU worker can then wrap its live `AppChannelClient` with the identical adapter. Cancel must continue to require one `Done` frame; submit/poll/chunk must each require one matching typed reply.

## Native WGPU boundary

The current native reservation is structurally useful:

- Reconcile preserves an extension as `UiNode::ExternalSlot` with plugin/controller address, raw params, and host status at `🔀️reconcile/🦀️.rs:1168-1173`. `UiExternalSlotNode` lives at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:3283-3297`.
- `host_content_height` reads the bounded contract value, falls back to six control rows, and clamps to `[0,4096]` at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs:38-51`; the node becomes `LayoutNodeKind::HostContent` at `:715`.
- Native paint draws the reserved panel and `host_status` at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:1770-1789`.

Keep this native capability boundary explicit: validate the canonical packet, reserve the same height, paint the packet's localized unsupported label, expose the same truthful label to native accessibility, and make zero submit/poll/cancel/take calls. A capability request is not proof that a decoder exists. Adding a native decoder would require a separate host interface and implementation; it should not be simulated by the browser route.

Accessibility needs one correction. The shared projection maps every Extension to `region` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:78-98`), but labels only authored accessibility or a few built-in components (`:244-270`). `host_status` therefore is painted yet absent from the projected tree. The WGPU projection stamps mounted bounds but does not consult `UiExternalSlotNode.host_status` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:101-154`). Give the exact reserved media extension a target-neutral host accessibility override: localized status label, noninteractive state, and an appropriate `status`/labeled-region role. Native AccessKit should consume that projection. In the browser, the live audio/video DOM subtree is the accessibility authority and the hidden generic ARIA mirror must omit the corresponding Extension node to prevent duplicate announcements.

## Browser presentation route

### Accepted slot publication

Do not derive DOM bounds from current/candidate renderer state on the page and do not overload `SceneHost`. `scene_slots` intentionally handles only `ComponentScene` and `Image` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📨️scene_slots/🦀️.rs:19-37,199-242`). Add a bounded read-only external-slot collector modeled on its absolute-layout walk.

The slot must be composed with the shell body rect and published only after GPU acceptance:

- `Ui` already exposes the presented tree, document node identity, tree revision, and surface generation at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:3553-3614`.
- `ShellState::register_retained_body_hits` is called for dock windows and panels but only remembers a body rect when a child control minted a hit (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14894-14907`). An ExternalSlot may mint no hit. Add a bounded `retained_body_rects` map to `PresentedInputGeometry` (`:3505-3516`), populate it unconditionally in this method, and stage/promote it with the existing candidate witness.
- The authority promotion is already atomic at `ShellState::acknowledge_presented_input` (`:14970-14995`). Derive `PresentedMediaSlot` values from the just-accepted Shell geometry and presented UI tree at this point or immediately afterwards while the same accepted state is held.
- Carry the immutable list through `AppPresentStep::Complete` (`🧊️renderer/🦀️.rs:16458-16472,17406`), `RenderSnapshot` (`📸️render-snapshot/🦀️.rs:11-26`), `BrowserRedrawOutcome`, `BrowserTickOutput` (`🌐️browser-worker/🦀️.rs:48-60,340-395`), the frame worker's `frame` post, and `BrowserFrameDirectives` (`🚚️browser-frame-transport/🟦️.ts:331-389`). The page updates the overlay only for the accepted lifecycle and generation, the same check currently applied at `:896-1009`.

A minimal `PresentedMediaSlot` contains:

- a stable slot token: window id, surface generation, document node id/key, tree revision, and resource generation;
- page-CSS logical rect and clip after shell body composition, plus stable paint order;
- plugin id and the shell-owned app instance/controller/document owner;
- parsed bounded media props/resource, not arbitrary unbounded JSON;
- whether the browser host may replace the fallback.

Set an explicit slot count and descriptor-byte cap. The existing frame transport has 64 lossless items, 256 KiB aggregate bytes, and a 4 KiB message item limit (`🚚️browser-frame-transport/🟦️.ts:12-31`); media descriptors need their own bounded budget rather than silently expanding the frame message.

The canvas fallback and DOM overlay must be governed by the same accepted snapshot. Native always paints the localized unsupported status. Browser WGPU should paint only the reserved background for a slot marked DOM-presented, or make the DOM host opaque over the exact accepted rect. If the browser host faults or is unavailable, clear the presented flag and retain the canvas unsupported fallback.

### Page DOM host

`browser-boot` currently replaces the root with one canvas (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:264-317`) and consumes only cursor/fullscreen/AX directives at `:329-333`. Add one absolute overlay sibling under a positioned root. Reconcile keyed media-host elements from accepted directives; do not remount unchanged slot tokens.

The target-neutral DOM host should create explicit `<audio>` or `<video>` plus host controls: play/pause, seek, current position, duration/unknown duration, and two separately labeled selection ranges when duration is known. Keep autoplay off. Use the contract's explicit locale labels. Set exact accepted bounds/clip/z-order. Stop pointer, click, wheel, context-menu, and keyboard propagation inside the overlay so the canvas input lane cannot also own a media interaction; the overlay physically prevents pointer events from reaching the canvas.

Use the shared `collectMediaTransportBytes` runner. Publish a Blob URL only if its slot token is still current after the terminal drain. Teardown is exactly once: mark the run cancelled, pause the element, remove `src`, call `load()`, revoke the URL, and remove the node. A descriptor replacement or disappearance removes stale UI synchronously while its serial runner finishes cancel-or-drain asynchronously.

### Worker command route and authority

The current browser protocol has no media messages (`BrowserFrameUiMessage`/`BrowserFrameWorkerMessage` at `🚚️browser-frame-transport/🟦️.ts:318-374`). Add a correlated bounded lane:

- UI → worker: `media-command { lifecycle, requestId, slotToken, operation, args/handle }` for submit, poll, cancel, and take.
- worker → UI: `media-result { lifecycle, requestId, slotToken, result|fault }` with chunk buffers transferred, not copied.
- Keep `bigint` through structured clone for revision/generation/operation/length; never downcast to number or serialize through JSON.
- Permit at most one in-flight command for each transport runner. Reject every pending request on transport close, worker fault, lifecycle change, or slot retirement.

The worker is the authority. Before every command it must resolve the currently accepted slot token and verify plugin id, app instance, controller, parent document, revision, generation, and output port. The page-provided descriptor or handle is never authority. The guest remains the final authority through the existing expected-document/revision and exact-handle checks.

`frame-worker` currently throws away the typed plugin handle when mounting: `PluginHandleMount` stores only `pluginHandleForBridge(module)` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:540-563`. Retain the typed `WgpuPluginHandle` beside the Rust bridge. Its `loadPluginModule` closure already owns `channelByInstance: Map<number, AppChannelClient>` (`🐚️plugin-bridge/🟦️.ts:1219-1245`). Add the four neutral adapter methods to `WgpuPluginHandle` (`:1154-1204,1849-1874`) and call those from the worker registry. There is no need to push binary media chunks through Rust's string/Reflect `WgpuJsBridge` (`:1905-1934,1970-1995`).

Do not derive app ownership solely from `UiExternalSlotNode.app_id`; it is the controller string and the node has no instance/document owner. Shell has the real owners: `ActiveSession` (`Shell WGPU:1410-1415`), `ShellSyncChannel` (`:1422-1431`), and `SpawnedAppEntry` (`:1083-1090`). Mint the descriptor from the matching shell session/spawned entry and publish it only when every resource owner field agrees. A mismatch means no live browser slot and no export.

## Smallest implementation sequence

1. Repair the schema drift in the WGPU validator and shared fixture; pin both TS/Ajv and Rust to all seven exact resource fields.
2. Move the React-free lifecycle and AppChannel adapter to a renderer-neutral media transport module. Keep the React host as one consumer.
3. Add the presented body-rect map and bounded external-slot collector. Publish parsed `PresentedMediaSlot` descriptors only with the accepted presentation witness.
4. Thread slots through `AppPresentStep`, `RenderSnapshot`, browser tick, frame worker, and `BrowserFrameDirectives` with explicit caps and accepted-generation checks.
5. Retain typed WGPU plugin handles and add the correlated worker media-command lane with exact owner checks, transferable chunks, and deterministic pending-request retirement.
6. Add the page overlay and target-neutral imperative DOM media host. Tie fallback suppression and ARIA-mirror suppression to the same accepted descriptor.
7. Add the native accessibility override and prove native makes no media export calls.

## Focused test packet

1. **Contract fixture:** update `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎬️media-transport-reservation/🔣️.json`; TS/Ajv and Rust must accept exact owner fields and reject missing, extra, mismatched-revision, out-of-range instance/generation, and null-resource-ready cases. Existing test owners are `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎬️media-transport-reservation/🟦️.ts` and `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs:645-671`.
2. **Layout/collector:** exact bounded host height; absolute nested slot rect and clip; panel and dock body composition; item/byte caps; generic extensions excluded.
3. **Presented authority:** extend `🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs`: a candidate slot is invisible, GPU acknowledgement publishes it atomically, abort preserves the prior list, and removal/replacement retires the old token and body rect.
4. **Snapshot/bridge:** prove slot lists move unchanged through `RenderSnapshot`/`BrowserTickOutput`; stale frame generation cannot update the overlay.
5. **Transport:** extend `engine/🧪️tests/📨️browser-frame-transport/🟦️.ts`: correlated result, exact bigint survival, transferred chunk buffer, item/byte caps, foreign/stale token refusal, and pending rejection on close/fault/lifecycle change.
6. **Plugin adapter:** submit/poll/cancel/take use the same AppCommand/AppFrame truth as React; cancel requires `Done`; every foreign handle field is rejected; completed MIME and total length remain exact.
7. **Shared lifecycle:** replay the same happy, cancellation, stale-owner, fault, cap, MIME, length, and drain vectors against both React and WGPU port adapters.
8. **Browser journey:** run a live WGPU page in Chromium, not copied markup. Use a small valid WAV fixture and assert accepted bounds, `<audio>`/`<video>`, no autoplay, play/pause/seek, two labeled selection ends, host AX ownership without a duplicate mirror region, canvas input isolation, descriptor replacement, cancellation, and one URL revoke.
9. **Native:** Rust fixture asserts exact reserved height, localized painted and AccessKit status, and zero submit/poll/cancel/take calls.

Relevant Nx lanes after implementation:

```sh
bun nx run @semio-tech/ui-rs:test-wgpu-engine
bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker
bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit
```

The current React real-browser oracle was run separately with the user Playwright cache and passed: one selected Chromium case passed, seven were skipped, case time 3.506 s, Nx time 50.8 s. It launches Chrome for Testing 151.0.7922.34 and verifies the copied static DOM (`AUDIO`, paused/no-autoplay, seek and selection labels/ranges, window identity). It does **not** run the React host, handlers, Blob URL, or AppChannel lifecycle in Chromium because the test copies JSDOM `innerHTML`; it cannot serve as the WGPU live-host acceptance test.

Focused command that passed:

```sh
PLAYWRIGHT_BROWSERS_PATH="$HOME/Library/Caches/ms-playwright" NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun node_modules/nx/dist/bin/nx.js run '@semio-tech/framework-renderer-react:test' --excludeTaskDependencies --skip-nx-cache -- long '../../../../🧱️elements/🎬️MediaTransportHost/🧪️tests/♻️lifecycle/🟦️.tsx' --run --silent=false --reporter=verbose --maxWorkers=1 --testNamePattern='exposes the mounted native media and two-ended selection controls in Chromium'
```

## Lifecycle hazards to keep explicit

- **Candidate versus presented:** never expose slots from a candidate UI tree or unacknowledged shell geometry.
- **Owner composition:** `controllerId` alone is insufficient. Bind plugin, instance, document, revision, and generation before issuing any command.
- **Stale completion:** an old run may complete after replacement. It must drain/discard and must never publish a URL.
- **Cancel versus take:** keep one serial runner. Do not race cancel with poll/take. A completed operation must be drained to its terminal chunk even when refused.
- **Close/fault:** reject all page pending promises and retire worker runs before disposing the typed plugin handle.
- **Object URL:** create only after exact terminal validation; revoke once on replace, decode/play failure, close, and unmount.
- **Accessibility ownership:** browser DOM owns live media semantics; hidden canvas mirror excludes that slot. Native announces the localized fallback.
- **Input ownership:** DOM media events must not also enter the canvas transport.
- **Bounds and scale:** published rects are page CSS logical pixels after shell composition; do not apply DPR twice.
