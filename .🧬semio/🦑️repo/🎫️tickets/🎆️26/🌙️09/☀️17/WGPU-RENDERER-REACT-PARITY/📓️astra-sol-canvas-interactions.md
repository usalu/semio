# Astra–Sol Canvas Interactions

## Result

The available React service on `127.0.0.1:6313` is the fixed `puzzle3d` React development target. Its process environment names `serve-puzzle3d-react-dev`, `SEMIO_PLUGIN=puzzle3d`, and `VITE_SEMIO_PLUGIN=puzzle3d`. Supplying `?plugin=draw` does not change that baked variant. The measured page exposes two `.semio-world-3d-host` surfaces, `window:puzzle3d-main-top` and `window:puzzle3d-main-perspective`; it exposes no Draw `Canvas2dHost` with controller `drawing-play` and surface `drawing.play.composite`.

Storybook on `127.0.0.1:6327` returned an index with 230 entries and zero entries matching `draw` or `Canvas2dHost`. It therefore could not provide a real Draw host without an unauthorized rebuild or relaunch.

The ticket now owns a bounded physical-probe contract at `🔬️canvas-interactions/📜️script.ts` and its neutral fixture at `🔬️canvas-interactions/🧫️fixtures/🔣️.json`. It requires the exact Draw host selector and records observed surfaces before any interaction. A wrong host fails explicitly; Puzzle3D is never labelled as Draw.

## Interaction contract

The fixture records these cases for a later dedicated Draw React activation:

1. Trusted primary drag with `shapeRect`: require ordered `canvasPointerDown`, `canvasPointerMove`, and `canvasPointerUp`, one added layer, and a changed canvas image.
2. Trusted primary click with `selectDirect`: require pointer down/up, one selected layer, and a changed selection overlay.
3. Trusted Shift-click and platform-modifier click: require additive selection followed by subtractive selection. The real Draw law maps Shift to `additive`, Control/Meta to `subtractive`, and their combination to `invertive`.
4. Trusted pointer input followed by physical Escape: require `canvasEscape`, no committed layer, and removal of the draft preview.
5. Trusted Playwright double-click: require `canvasDoubleClick` and one observable draft commit.

These action names and utilities come from the Draw publication contract and `Canvas2dHost`; the default utility is `selectDirect`, and the real shape gesture returns to it after commit. No production defect was isolated because no matching host was available.

Desktop Playwright has no trusted physical `pointercancel` primitive. Synthesizing `dispatchEvent` would bypass the browser input path, so the fixture marks that case unsupported rather than fabricating evidence. Escape remains a genuine keyboard cancellation case.

## Validation

- `bun .../🔬️canvas-interactions/📜️script.ts test`: PASS, five cases and one explicit unsupported case.
- `bun .../🔬️canvas-interactions/📜️script.ts run`: expected explicit failure, exit 1. It measured only the two Puzzle3D World3D surfaces and wrote the unavailable receipt.
- `GET http://127.0.0.1:6327/index.json`: HTTP 200, 230 indexed stories, zero Draw/Canvas2dHost matches.

The canonical dedicated target exists as `@semio-tech/framework-os-dev:activate-draw-react-dev`; the play project also references `@semio-tech/framework-os-dev:prepare-draw-react-dev`. A later integration pass must activate that Draw React variant on its own port, point `SEMIO_CANVAS_REACT_URL` at it, and then calibrate the physical cases against the exact host. This packet did not activate, build, generate artifacts, run native code, or exercise WGPU. It makes no WGPU runtime claim.

## 2026-09-20 probe revision

The neutral fixture is now version 3 and names the canonical Draw, Layout, and Note surface specimens instead of treating clipboard or catalogue work as a Draw concern:

- Draw remains the five-case Canvas2d journey at React/WGPU ports 6064/6164.
- Layout owns the four-case catalogue drag journey at React/WGPU ports 6079/6179.
- Note owns the six-case Ink clipboard/editor/cancellation journey at React/WGPU ports 6080/6180.

The exact Ink journey and its validation boundary are recorded in `📓️astra-sol-ink-clipboard.md`. The Draw cases remain preserved unchanged and are reported as `ready-for-physical-calibration` until root runs the freshly activated dedicated Draw target. The fixture test now also executes Chromium's browser-level CDP cancellation producer and proves its events are trusted and contain no forged pointer-up.

## Layout catalogue physical adapter

`🔬️canvas-interactions/🧬️schema/📐️layout-catalogue/🔣️.json` fixes the Layout journey to the real `demo` document and the exact four-item catalogue roster: `page`, `rect`, `text`, and `image`. It also fixes the base MIME `application/x-semio-catalogue-item`, each kind MIME, each exact JSON payload, the Catalogue and Artifact tab identities, and the seed counts of two pages and three total frames. The schema permits no extra cases or fields.

The ticket-owned `runLayout` adapter uses genuine Playwright mouse input. For each case it performs this complete sequence:

1. Reload the `demo` fixture with the physical example picker and observe the seed count in the Artifact tree.
2. Open the Catalogue panel and start a browser drag from the real transfer handle. React requires the `data-slot="drag-handle"` and `data-drag-role="transfer"` witness; WGPU requires the exact `tree.drag.transfer.<sourceId>` hit.
3. Move onto the exact Layout Canvas2d surface, require `canvasDragOver`, verify the editor controller plus base and kind MIME values, and capture the preview pixels.
4. Move outside the surface and release. Require `canvasDragLeave`, no synthetic `canvasDrop`, restored pixels, and an unchanged Artifact count.
5. Repeat the physical drag and drop. Require terminal action order `canvasDragLeave` then `canvasDrop`, the canonical JSON payload, and exactly one new page or frame in the Artifact tree.

Page preview is deliberately pixel-stable because the page operation has no frame preview. Rectangle, text, and image previews must change the canvas pixels. All four cases require leave retirement before the committed mutation.

The neutral fixture test includes an independent Chromium HTML Drag and Drop oracle. It uses a real draggable DOM node and Playwright mouse movement, observes trusted `dragstart`, `dragover`, `dragleave`, and `drop` events, checks both MIME types, and checks that the exact JSON payload is readable at drop. Chromium deliberately withholds `getData` during dragover, so the adapter uses the MIME roster for preview and the payload for the terminal drop, matching browser policy.

The contract is grounded in the production sources:

- Layout catalogue publication supplies the exact roster, MIME types, and payloads in `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`.
- The Artifact panel publishes the page and frame section identities consumed for mutation counts.
- React Tree exposes the physical transfer handle and mirrors its MIME/payload attributes.
- `Canvas2dHost` admits the base MIME, publishes the kind roster during preview, publishes leave, and publishes leave before the exact raw payload on drop.
- WGPU input exposes the same transfer owner as `tree.drag.transfer.<item.id>`.

### Runtime boundary

The canonical targets are:

- `@semio-tech/framework-os-dev:prepare-layout-react-dev`
- `@semio-tech/framework-os-dev:activate-layout-react-dev`
- `@semio-tech/framework-os-dev:serve-layout-react-dev`
- `@semio-tech/framework-os-dev:serve-layout-wgpu-dev`

This packet did not activate or serve them. The Layout adapter therefore has a strict execution-ready contract and an explicit unavailable runtime boundary; it has no passing physical-app claim yet. Root can run `📜️script.ts run layout react` against `http://127.0.0.1:6079/?plugin=layout` and the WGPU counterpart against port 6179 after the corresponding canonical targets are active.

### Layout validation receipt

- `bun .../🔬️canvas-interactions/📜️script.ts test`: PASS. The fixture reports version 3, the exact four Layout cases, and the trusted Chromium drag event sequence with the exact terminal payload.
- `bun x tsc .../🔬️canvas-interactions/📜️script.ts --noEmit --module esnext --moduleResolution bundler --target es2022 --allowImportingTsExtensions --skipLibCheck`: PASS.
- No native build, WGPU activation, artifact generation, or Layout application serve was started by this packet.

## Canvas2d camera gesture parity laws

The current implementations expose two concrete Draw-visible mismatches before any new production repair. React wheel zoom is cursor anchored, applies exactly `1.1` for zoom-in and `0.9` for zoom-out, and clamps to `0.05..32`. WGPU currently changes zoom around the viewport centre with the continuous factor `(1 - delta * .001)` and clamps to `.125..8`. React pans on a primary drag while `transformMove` is active or on a middle-button drag; it leaves a right-button drag in the document gesture lane. WGPU currently starts pan only for middle or right buttons and publishes pointer actions for its pan gestures.

The shared schema and fixture are now:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🧭️canvas2d-camera-gestures/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🧭️canvas2d-camera-gestures/🔣️.json`

They pin an off-centre 400×300 wheel specimen, both factors and both limits, and three exact drag cases: primary active Pan, middle-button Pan, and right-button document gesture. The expected camera coordinates are exact and no numeric tolerance was added.

The mounted React Canvas2d suite consumes the schema through Ajv, verifies the pure cursor anchor and limits, observes one settled `setCamera` from the real `JsonLayersCanvasSession`, and verifies the middle/right button split. Focused Nx receipt `🗑️generated/astra-runtime/canvas2d-camera-react-oracle-5.log`: five selected passed, 23 outside the filter, Nx 10.6 seconds. The WGPU real-function laws are registered in `Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs`; their production code is intentionally unchanged pending root's fail-first run.

Exact native filters:

- `canvas2d_wheel_uses_the_react_factors_limits_and_exact_cursor_anchor`
- `canvas2d_primary_drag_with_the_active_pan_utility_changes_only_the_camera`
- `canvas2d_middle_drag_changes_only_the_camera`
- `canvas2d_right_drag_remains_a_document_gesture_and_does_not_pan`

The Draw undo chord is declared and already has shell routing laws, but this audit found no Canvas-specific undo divergence. No speculative undo case was added.

## Camera event granularity and pan parity

The shared `canvas2d-camera-gestures` schema/fixture now includes two exact two-event wheel bursts at one cursor point: two zoom-in events have ordered factors `[1.1, 1.1]`, while zoom-in then zoom-out has ordered factors `[1.1, 0.9]` and therefore ends at factor `0.99`, not the initial zoom. Both debounce to one final `setCamera` publication. This pins event granularity without replay loops or numeric tolerances.

The mounted React oracle dispatches both actual DOM `WheelEvent`s through `JsonLayersCanvasSession`, derives the final camera by applying `wheelCameraAtScreen` once per row in order, and observes one settled action. Focused Nx receipt `🗑️generated/astra-runtime/canvas2d-camera-react-bursts-1.log`: three selected tests passed (strict Ajv plus both mounted bursts), 27 outside the filter, Vitest 5.08 seconds, Nx 5.8 seconds.

The WGPU Canvas2d helper now reads the same `role: meta` utility carrier React reads. `transformMove` primary drags and middle-button drags own a local `PanViewport`, publish no document pointer commands, and settle one camera action. A right-button drag under `selectDirect` remains a document down/move/up sequence and does not pan. Sol's concurrent per-pointer `PointerId` ownership remains on every gesture comparison and mutation.

A new actual App/Shell law, `renderer_canvas_secondary_drag_reaches_the_document_gesture_before_opening_its_context_menu`, drives right down/move/up through `AppInteractionState`, requires the three Canvas document commands, and independently requires the context menu to open. Production Shell routing is held until root records its intended RED because the current `button == 2` branch opens the menu and returns before retained Canvas dispatch.


## Pointer-slot reconciliation and actual secondary route

The WGPU pan implementation now uses the shared 16-slot `CanvasGestureSlots` owner introduced by the pointer-identity packet. Pan admission is exactly middle button, or primary button while `transformMove` is active; a right button therefore remains a document gesture for every utility. Down/move/up for primary active-pan and middle pan mutate only the local camera and schedule one settled `setCamera`; right continues to publish `canvasPointerDown`, `canvasPointerMove`, and `canvasPointerUp`.

A new native law, `canvas2d_middle_pan_cancellation_retires_locally_without_a_document_action`, pins React's local cancel behavior: the exact pointer slot and surface drag retire while the document action queue stays empty. SolLocale owns the coordinated production cancellation branch so the existing document-gesture cancellation laws continue to publish one cancelled terminal up and foreign pointers remain inert.

The actual App/Shell secondary-button law now consumes the same neutral `right-button-gesture` row rather than duplicating its coordinates, utility, and expected action sequence. It snapshots the result and closes gesture capture plus the published document before asserting, so a failure cannot leak an owner into later tests. The native actual-route law remains fail-first until root records the right-button Shell admission result. All touched Rust sources parse with rustfmt; no Cargo command was run.

## Actual secondary-button repair

Native86 recorded the intended App/Shell failure: `renderer_canvas_secondary_drag_reaches_the_document_gesture_before_opening_its_context_menu` received no document actions, while the neutral row requires `canvasPointerDown`, `canvasPointerMove`, and `canvasPointerUp`. The full receipt was 1,287 run, 1,281 passed, six failed, zero skipped in 31.862 seconds; the focused assertion is in `🗑️generated/astra-runtime/renderer-native86/run.log`.

Shell now resolves a secondary press through the exact retained Canvas2d scene registration before opening the context menu. Admission requires the published hit rectangle, window/body owner, Canvas2d kind, and live scene target to agree. The existing capture route owns subsequent move and release. Text-editor Alt completions retain precedence, and other secondary hits keep the existing context-menu path.

Both the production source and the actual App/Shell law parse with `rustfmt`; the check reports unrelated repository formatting differences, including pre-existing differences elsewhere in the Shell module tree. No Cargo or activation command was run here. The exact native filter remains `test(renderer_canvas_secondary_drag_reaches_the_document_gesture_before_opening_its_context_menu)`.

## Camera mount lifetime

The shared camera fixture and schema now carry a three-step `mountLifetime`: camera A authored at zoom 2 reaches 2.2; a same-key refresh authored at 8 keeps A's local session and reaches 2.42; replacement camera B authored at 3 starts fresh and reaches 3.3. A pending A deadline after B replaces it and a pending B deadline after removal must each publish zero camera actions.

The mounted React law uses the real `Canvas2dHost`, its `GraphWasmCanvas` session seam, real DOM `WheelEvent`s, keyed reconciliation, fake time, and separate A/B dispatch journals. Its intended RED was exact: the replaced A deadline still published one action where the fixture requires zero (`canvas-camera-lifetime-red/run-3.log`).

`Canvas2dHost` now owns a cleanup effect keyed to the exact `dispatch` identity. Replacement, changed action ownership, and unmount clear and null the pending camera timeout; a same-key authored-camera refresh with unchanged dispatch leaves the live local session and its deadline intact. The focused law passes 1/1 with 30 outside the filter, and the complete mounted Canvas input suite passes 31/31. Receipts: `🗑️generated/astra-runtime/canvas-camera-lifetime-green/run.log` and `🗑️generated/astra-runtime/canvas-camera-input-full/run.log`.

## Exact Canvas hit provenance

Native91 disproved the first secondary-button repair without weakening its identity rule. The helper's temporary census recorded `scene=false`, `owner=true`, `body=true`, and `current=true` for `canvas2d-camera-gestures`: the generic Canvas hit belonged to the correct published body and a live exact Canvas scene existed at the point, but `RetainedHitRegistration.scene` intentionally omitted every component kind except World3d, NodeGraph, TiledMap, and Board2d. The actual route therefore produced no document actions again. The full Native91 receipt is `🗑️generated/astra-runtime/renderer-native91/run.log` (1,286/1,290 passed; the other failures were separately owned retirement laws).

Every `ComponentScene` hit registration now carries its canonical surface kind and ID. Shell continues to classify only its existing dedicated scene kinds as `PointerHitOwner::Surface`; Canvas2d remains on the established Shell retained route. The exact generation-qualified `ScenePointerTarget` is now staged and published beside the Canvas hit, which lets the secondary helper require equality with the live scene at the press point. The temporary diagnostic was removed.

`a_published_canvas_hit_cannot_retarget_a_same_key_successor` publishes and paints a real Canvas document, preserves its old hit registry, then publishes and paints a successor with the same surface ID and retained key. It proves the successor has a distinct generation-qualified identity and the old published hit is refused. This keeps key replacement fail-closed while enabling the current exact Canvas owner.

Exact focused filters: `test(renderer_canvas_secondary_drag_reaches_the_document_gesture_before_opening_its_context_menu) | test(a_published_canvas_hit_cannot_retarget_a_same_key_successor)`.

## Same-key component remount identity

Native93 proved that a retained Canvas published as `Canvas2d → Container → Canvas2d` under the same document node, key, surface ID, and window generation recreated the same `ScenePointerTarget`. The focused law failed at the distinct-owner assertion even though all three document generations reconciled. An old published Canvas hit could therefore target the later component mount.

Each retained node now carries a checked component-mount generation. An exact key/kind/surface scene refresh preserves it. A key, kind, surface, or component-type transition advances it before the new scene becomes visible. The old generation is copied into `UiRetiredComponentScene`; the current generation crosses `SceneSlot` into paint-owned Canvas camera state and crosses retained lookup into published hit provenance. Liveness compares it with the current node. Retirement backpressure returns `Pending` before either the spec or generation changes, and counter exhaustion faults the reconcile without retiring or mutating the live owner.

The language-neutral camera fixture adds `sameKeySceneRemount`: a same-key refresh retains zoom `2.42`, an intervening `container` retires the pending deadline with zero actions, and the same-key Canvas remount starts from authored zoom `4` and publishes exactly one zoom action at `4.4`. Its schema is closed and validates through Ajv in the mounted input suite. The mounted React oracle uses the real `Canvas2dHost`, keyed React reconciliation, a real component-type unmount/remount, DOM `WheelEvent`s, and fake time. The complete input-contract file passed 32/32 tests; Vitest took 5.27 seconds and Nx 6.1 seconds. Receipt: `🗑️generated/astra-runtime/canvas-same-key-remount-react/full-2.log`.

Native fail-first receipt: `🗑️generated/astra-runtime/renderer-native93/run.log`, one selected failure, 1,291 outside the filter, 44 milliseconds, Nx 2 minutes 28 seconds. Green native validation is owned by root. Exact relevant filters are `a_published_canvas_hit_cannot_retarget_a_same_key_successor`, `same_key_scene_kind_replacement_emits_the_exact_old_identity_once`, and `a_full_scene_retirement_ledger_yields_before_mutating_the_old_node`.

### Native95 component-epoch receipt

The full Native95 renderer census ran 1,294 laws: 1,293 passed and one exact `EngineSurfaceRegistry` size census failed after an independently owned eight-byte change. The same-key component remount, exact old-hit refusal, scene-kind retirement identity, retirement-backpressure, and checked-out camera deadline laws all passed. Root owns the remaining size census update.

### Complete window-close audit

The component-removal path is exact, but complete Dock-window closure currently bypasses it. `close_dock_window` removes the Dock entry and schedules a Full refresh. `refresh_ui` removes the closed surface's `window_ui` lease through `retire_documents_outside`, while `sync_engine_surface_states` retires the bespoke World3d, NodeGraph, TiledMap, and Board2d mirrors. Neither path closes the matching retained `UI_ENGINE` surface. `Ui::close_document_step` is unused by production and, if called, frees document-bound arena nodes without first enqueueing their `UiRetiredComponentScene` identities.

That leaves two stale Canvas authorities after a complete close: `retained_scene_target_at` still finds the old node because the UI engine window survives, and `SceneCameraDeadline::owns_surface` accepts the old deadline because it compares only `SCENE_STATE.canvas_camera_owner`. It does not ask `scene_pointer_target_is_live`. Owner liveness must fail closed when its retained window has gone away, while ownerless Paint2d deadlines keep their separate path; that predicate alone is insufficient until the Shell close ladder also steps the retained UI document to terminal and consumes its exact scene retirements.

The fail-first law `renderer_canvas_closed_window_retires_its_pending_camera_owner` uses the actual Dock close and `window_ui` retirement lanes after a real retained Canvas paint and Scroll. It requires zero settled camera actions and no retained scene target, then republishes a Container before asserting so a RED result leaks no global owner. Native96 confirmed the causal failure: one selected law failed with one camera action instead of zero, 1,294 outside the filter, 62 milliseconds test time and Nx 4 minutes 10 seconds. Nextest run `20902770-c79c-4535-8ba5-5931f6b4f067`; receipt: `🗑️generated/astra-runtime/renderer-native96/run.log`.

The repair retains a bounded close owner for the exact `(SurfaceId, accessibility generation)` at the successful Dock mutation. The Shell settle predicate keeps the runtime awake while that owner exists, and the Chrome `CloseDocument` phase advances one close unit before the ordinary document-arena retirement unit. A same-ID successor with a different generation makes the old request stale instead of letting it close the successor. Repeated close requests for one ID replace the older queued generation, and the queue cannot exceed the UI engine's 64 retained surface slots.

`UiTree::close_document_binding_step` now offers each component scene to the existing retirement ledger before freeing its arena node. Ledger backpressure preserves the live node for a later step. The Interpreter consumes one exact `UiRetiredComponentScene` outside the UI engine borrow and uses the same Canvas/Map retirement functions as ordinary reconcile. Owner-bearing camera deadlines also require a live retained target; ownerless Paint2d deadlines retain their existing path. The native law now checks the wake, drives the bounded production close step to terminal, and requires both the camera action and retained target to be absent. The existing closed neutral `sameKeySceneRemount` row and mounted React Canvas→Container type-unmount law supply the language-neutral and independent third-party oracle for this lifecycle; the full React receipt remains 32/32.

UI full3 remained green before this bounded close edit: 656/656, zero skipped, 2.445 seconds test time and Nx 29.7 seconds, Nextest run `50cf9332-6c1f-4e16-8709-d83416678bf8`. The touched Rust sources parse with `rustfmt`; its repository-wide check reports unrelated formatting drift. Root owns the focused and full native reruns.

### Immediate Dock-close fence

Native97 passed the initial complete-window close law and the independent engine census; its only failure was the separately owned presentation law. The first repair still left an ordering gap: `close_dock_window` mutated the Dock before the asynchronous layout refresh called `retire_documents_outside`, while the next frame runs the camera settlement phase before its Build/refresh phase. A due Canvas camera could therefore publish in the interval after the window disappeared and before the retained close was admitted.

`close_dock_window` now first proves the Dock contains the requested window, then admits the exact bounded UI document-close owner, and only then mutates the Dock. Admission refusal leaves the Dock intact. `retire_documents_outside` keeps its generic admission for mode, layout, and panel removals; repeated admission for the same window/generation consumes no extra credit. The native camera law now sweeps expired cameras immediately after the Dock close and before `retire_documents_outside`, requires zero actions in that interval, and only then drives the retained document and scene retirement to terminal.

A later bounded packet still needs to reclaim complete `UiSurfaceRegistry` slots and empty `SCENE_STATE` entries. Closing the current document frees document nodes but retains the occupied surface slot, so more than 64 sequential distinct surface IDs can exhaust the registry even when every document was closed. This is a separate full-surface lifecycle law and does not weaken the immediate close fence.
