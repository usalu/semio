# Terra Surface Runtime Routes

## Scope and boundary

This is a read-only source audit completed while the current listeners on 6313, 6213, and 6327 were owned by other work. No build, activation, server, browser, native executable, or generated artifact was run or changed. A source route below is not runtime acceptance. In particular, pre-existing listener output cannot evidence the current source tree.

The contract has exactly fifteen surface kinds at:

- 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81-129
- React host selection: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:359-424,483-565
- WGPU scene paint dispatch: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs

The prior source inventories used for cross-checking are 📓️astra-terra-surface-completeness.md, 📓️astra-sol-canvas-input.md, 📓️astra-sol-ink-editing.md, and 📋️astra-parity-matrix.md in this ticket.

## Concrete playgrounds and zero-touch targets

| Physical specimen | Playground metadata | App surface | Default React / WGPU port | Launch entry |
| --- | --- | --- | --- | --- |
| Draw | ✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml:18-20 | s.draw.drawing@1/*#editor; Canvas window drawing-composite, surface drawing.play.composite | 6064 / 6164 | .vscode/launch.json:3656-3708 |
| Layout | ✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/Cargo.toml:18-20 | s.layout.layout@1/*#editor; Blueprint window layout-blueprint, surface layout.play.blueprint | 6079 / 6179 | generated workspace dev route; use its metadata-selected variant |
| Note | ✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/Cargo.toml:18-20 | s.note.note@1/*#editor; composite window note-composite, surface note.play.composite | 6080 / 6180 | .vscode/launch.json:3711-3762 |

Draw produces a real Canvas2d scene at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs:9-11,114-117. Layout produces its editable Canvas2d Blueprint scene at ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🦀️.rs:12-14,41-47. Note produces its interactive InkCanvas composite scene at ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs:9-11,20-23,91-95. Its navigator is the separate non-interactive specimen.

The repository derives the target names rather than storing one static project.json per app:

- 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1024-1053 creates serve and activate targets named serve-<variant>-<renderer>-<profile> and activate-<variant>-<renderer>-<profile>.
- Every serve target depends on its corresponding activation target. WGPU prepare also depends on the plugin session, support, fonts, WGPU wasm, browser boot, frame worker, and selected component materialization.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts:84-105 validates activation and writes the renderer/profile/variant receipt. Its helper at :126-136 delegates the complete closure to @semio-tech/framework-os-dev:activate-<variant>-<renderer>-<profile>.
- Root 📜️script.ts:253-282 makes bun nx run workspace:dev -- <variant> the zero-touch entry point. Environment selection resolves the dynamic dev or serve target; it does not make a server script silently switch renderers.

After the owner has completed fresh activation, the exact serving targets are:

    S_OS_PORT=6313 bun nx run @semio-tech/framework-os-dev:serve-draw-react-dev
    S_OS_PORT=6213 bun nx run @semio-tech/framework-os-dev:serve-draw-wgpu-dev

Replace draw with layout or note for those specimens. The equivalent zero-touch commands, also represented by the Draw and Note launch entries, are:

    S_OS_PORT=<chosen-port> SEMIO_PLUGIN=<variant> SEMIO_RENDERER=react SEMIO_APP=<metadata-app> bun nx run workspace:dev -- <variant>
    S_OS_PORT=<chosen-port> SEMIO_PLUGIN=<variant> SEMIO_RENDERER=wgpu SEMIO_APP=<metadata-app> bun nx run workspace:dev -- <variant>

The WGPU browser server accepts the selected port and prints the URL with ?plugin=<variant> at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts:12-35. React serves the receipt-backed runtime through 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:21-35.

Storybook is a component-only control surface. The registered launch default is 6010 at .vscode/launch.json:2767-2783; 6327 is an owned runtime override. The Canvas2d, InkCanvas, World3d, NodeGraph, TextEditor, Table, Paint2d, IconRender, TiledMap, GraphTimeline, BlockList, and Interpreter stories exist below their respective engine element directories. They can diagnose host rendering but cannot prove plugin activation, action dispatch, or a current app projection.

## Canvas2d event routes and one real integration gap

### Routes that are implemented at the renderer boundary

React Canvas2d preserves Shift, Ctrl, Meta, and Alt on press/release and turns cancellation into one terminal canvasPointerUp action at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx:425-450,657-705. The session’s pointerCancel at :708-717 emits cancelled:true. Its doubleClick at :719-720 emits canvasDoubleClick. Drag-over, leave, and drop are at :872-905; drop preserves the canonical MIME payload as dragData.

The WGPU route now has a full Canvas pointer wire, including all modifier fields, cancelled, and samples, at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1215-1285. It emits canvasDoubleClick at :1302-1316 and turns release outside the scene into cancelled:true at :1519-1544. Retained-catalogue drag ingress accepts the canonical MIME and routes drag-over/drop at 🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:473-542, then serializes dragData at 🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1609-1729.

The language-neutral Canvas oracle is:

- fixture: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧫️fixtures/🖱️input-contract/🔣️.json
- mounted React host test: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx:83-194
- native shell law: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs

Those are good source and host-harness coverage. They are not fresh browser app acceptance.

### Real DnD gap: Layout cannot consume the renderer’s current arguments

Layout is the correct physical DnD specimen: its catalogue presents Page, Rectangle, Text, and Image rows as draggable TreeItems, using application/x-semio-catalogue-item with JSON such as {"kind":"rect"}, at ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs:20-26,42-60. Its CanvasDrop handler genuinely adds a frame or page, and CanvasDragOver sets its preview, at 🎮️commands/📥️canvas-drop/🦀️.rs:42-65 and 🎮️commands/🛬️canvas-drag-over/🦀️.rs:49-67.

But the integration is not compatible:

| Producer | Actual args |
| --- | --- |
| React Canvas2d drag-over | surfaceId, x, y, width, height, types; no kind — Canvas2dHost/🟦️.tsx:884 |
| React Canvas2d drop | surfaceId, x, y, width, height, dragData containing the raw JSON — Canvas2dHost/🟦️.tsx:902 |
| WGPU Canvas2d drag-over/drop | the same types and raw dragData shapes — Scenes/WGPU:1609-1729 |
| Layout action decoder | aliases only drop_kind to required kind, never dragData to kind — Layout editor/🦀️.rs:492-502 |
| Layout command payload | CanvasDragOver and CanvasDrop both require kind — commands/🛬️canvas-drag-over/🦀️.rs:49-58 and commands/📥️canvas-drop/🦀️.rs:42-53 |

No normalizer appears on this renderer-to-Layout action path. A real catalogue drag therefore lacks Layout’s required kind on drag-over and drop. This is a current implementation gap, not merely missing browser coverage. The existing Layout command tests prove only direct typed commands with kind already supplied: 🎮️commands/👇️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs:183-211.

Draw is suitable for modifier, cancellation, and double-click dispatch. It consumes selection modifiers in its drawing gesture state machine at ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs:60-74,252-256,310-333. It does not declare the Canvas DnD actions, so it must not be used to claim DnD acceptance.

## InkCanvas route and clipboard boundary

React opens text/table editors after double-click, commits text on blur, advances table cells on Enter or Tab, and cancels on Escape at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx:1320-1435. Its root handles Copy and Paste at :1454-1515,1542-1545, including selected-block payloads, image assets, and plain text.

WGPU now has InkEditState and block/cell detection at 🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:366-404,5610-5666; it reserves atomic inkApplyEvents at :6052-6057 and commits/advances table input at :6119-6188. The direct scene input opens edits at :6799 and active editors are genuine retained Input hits at :7214-7219. The native law is 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs; the React host law is 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖋️ink-canvas-editing/🟦️.tsx:20-69. Both consume the shared fixture:

    🧰️framework/🔨️modules/🖱️ui/🧪️fixtures/🖋️ink-canvas-editing/🔣️.json

That fixture deliberately marks clipboard as separate-follow-up. It provides text-a and table-b to test editing, but no clipboard acceptance. Note also has a larger text/table document fixture at ✏️s/🔌️plugins/🗒️note/🧫️fixtures/🪪️document-contract/🔣️.json:4-95.

Generic WGPU UI code has OS clipboard plumbing for generic ClipboardCopy, ClipboardCut, and ClipboardPasteRequested commands at 🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:566-610. That does not implement the InkCanvas semantic producer: the Ink scene’s editing path has no Ink clipboard payload emitter or parser matching React’s ink.clipboard payload. Ink clipboard remains a genuine missing implementation; it cannot be downgraded to “browser coverage missing.”

For a visible Note seed, the app exposes Add Block with Text and Table kinds at ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:735-756. It also supports loading a test document through loadRequest → setFixtureJson at 🎮️commands/📥️load-request/🦀️.rs:12-13 and 🎮️commands/🧫️set-fixture-json/🦀️.rs:11-20.

## All-surface execution matrix

The source inventory confirms a React producer and non-placeholder WGPU route for all fifteen kinds. The final column is deliberately strict: no row becomes fresh physical acceptance from a story, unit test, stale server, or old activation artifact.

| Kind | React producer | WGPU route family | Current practical specimen | Fresh physical status |
| --- | --- | --- | --- | --- |
| Canvas2d | 📐️Canvas2dHost | direct canvas input/paint | Draw for modifiers/cancel/double-click; Layout for DnD | Renderer routes present; Layout DnD argument mismatch blocks app acceptance |
| World3d | 🌐️World3dHost | direct 3-D pass | World3d story or root’s owned puzzle3d run | no fresh acceptance recorded |
| NodeGraph | 🕸️NodeGraph | engine texture/direct event loop | NodeGraph story | no fresh acceptance recorded |
| TextEditor | ✏️TextEditor | engine texture/focused key route | TextEditor story | no fresh acceptance recorded |
| Table | 📊️Table | retained rows | Table story | no fresh acceptance recorded |
| Paint2d | 🖌️Paint2dHost | engine texture/direct pointer route | Paint2d story | no fresh acceptance recorded |
| VirtualFileSystem | 🗣️Interpreter VFS branch | retained rows | Interpreter story fixture | no fresh acceptance recorded |
| TiledMap | 🧭️TiledMapHost | engine texture/direct event loop | TiledMap story | no fresh acceptance recorded |
| Board2d | 🖥️Board2dHost | engine texture/direct event loop | Interpreter route; no dedicated story located | no fresh acceptance recorded |
| IconRender | 🖼️IconRenderHost | direct icon renderer | IconRender story | no fresh acceptance recorded |
| InkCanvas | 🖋️InkCanvasHost | direct/resumable Ink input | Note composite, shared text/table fixture | text/table source laws; clipboard missing; no fresh acceptance recorded |
| GraphTimeline | 🌳️GraphTimelineHost | retained rows | GraphTimeline story | no fresh acceptance recorded |
| BlockList | 🧩️BlockListHost | retained rows | BlockList story | no fresh acceptance recorded |
| DiffView | 🔺️DiffViewHost | retained rows | Interpreter route; no dedicated story located | no fresh acceptance recorded |
| EventFeed | 📡️EventFeedHost | retained rows | Interpreter route; no dedicated story located | no fresh acceptance recorded |

The React producer and WGPU-family classifications above are corroborated by this ticket’s source inventory at 📓️astra-terra-surface-completeness.md:25-45. This audit did not invent a playground route for rows whose source only establishes a host or story.

## Bounded next execution packet

Run each renderer sequentially, only after the owner confirms a fresh activation receipt for that variant. Use a new port or the owner-designated 6313/6213 slot; do not reuse a listener merely because it answers HTTP.

1. Draw Canvas2d: load the drawing composite at /?plugin=draw. Use a selection gesture with each modifier combination needed by the shared fixture. Send press, move, release with Shift/Ctrl/Meta/Alt and inspect the action trace for those same booleans. Start another gesture, dispatch a real browser pointercancel or release outside the Canvas bounds, and require exactly one canvasPointerUp with cancelled:true. Perform two left clicks at the same point within the double-click interval and require canvasDoubleClick. Execute once against React and once against WGPU.
2. Layout Canvas2d DnD: load /?plugin=layout and drag the real Rectangle catalogue item onto layout.play.blueprint. The source intent is a drop preview followed by a new frame. Current expected outcome is an actionable failure: the preview/frame does not appear because kind is absent from the renderer action. Capture the exact emitted action and app decode fault for both renderers; this confirms the source mismatch and supplies the implementation regression case.
3. Note InkCanvas: load /?plugin=note. Use Add Block to create one Text and one Table, or load the Note document-contract fixture through its supported loadRequest/setFixtureJson route. Double-click the text, replace content, and blur; double-click a table cell, type, press Enter, then Tab in the advanced cell; double-click text again, type, and Escape. Require the expected atomic inkApplyEvents updateBlock action(s) and no action for Escape. Execute once against React and once against WGPU.
4. Clipboard split: in React, select a Note block, invoke Ctrl/Cmd+C and Ctrl/Cmd+V, and require a re-identified added block. In WGPU, record this as the expected current failure: generic clipboard transport must not be mistaken for Ink clipboard semantics; no Ink clone/addBlock outcome exists yet.
5. Record renderer, fresh activation receipt identity, URL, selected app/window/surface id, exact input event, action trace, visual outcome, and console output. A green result requires the app consequence as well as the renderer event. A source test, component story, or old listener is only diagnostic evidence.

## Trustworthy coverage versus open evidence

| Area | Trustworthy current evidence | Still unknown or missing |
| --- | --- | --- |
| Canvas modifiers, cancellation, double-click | mounted React host oracle; WGPU concrete wire/action source; native shell law | fresh Draw React/WGPU app trace and visible selection consequence |
| Canvas DnD | real Layout catalogue, real Blueprint Canvas, WGPU retained drag route | broken kind normalization prevents an app consequence today |
| Ink text/table | shared fixture, React host oracle, WGPU native scene law | fresh Note React/WGPU browser pass |
| Ink clipboard | React implementation | WGPU Ink producer/parser is missing, then both renderers need fresh app acceptance |
| Remaining thirteen kinds | source host/paint inventory and several component stories | a current activation-backed browser journey for each app-level representative |

