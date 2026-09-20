# Surface Completeness Audit

Read-only source audit on 2026-09-20. This inventories the live 15-variant `SurfaceKind` contract against the React reference and the WGPU production route. It deliberately does not reopen the repaired World3d grid-snap path, Table/BlockList transfer/cancellation, agent cancellation, Sync/Settings, or the root-owned native/runtime gates.

No build, Cargo command, browser journey, or pixel comparison ran for this audit. `✓` below means that a current source route exists, not that runtime parity has been accepted.

## Result

All 15 contract variants have a React producer, a WGPU `ComponentScene` payload field, and a non-placeholder WGPU paint route. The remaining confirmed source gaps are concentrated in the Canvas2d and InkCanvas interactive producers; they are not missing scene renderers.

The canonical contract is the 15-entry enum in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81-129`; the WGPU copy has the same wire tags in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2937-3003` and one typed payload field per kind at `:3259-3301`.

## Source Flow

React decodes `SurfaceProps.doc.bytes`, uses the complete `SURFACE_KIND_SCENE_FIELD` map, and selects the host in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:359-424,483-565`. `SurfaceView` wraps every surface in the shared accessibility shell; World3d, Canvas2d, Board2d, and TiledMap additionally use the paged lane (`:2055-2092`).

WGPU reaches the same scene node through `FrameworkSceneHost::paint_slot_step` in `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1291-1332`, then `render_component_scene_step` in `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1921-2064`:

- direct: World3d, Canvas2d, InkCanvas, IconRender;
- retained list paint: Table, VirtualFileSystem, GraphTimeline, BlockList, DiffView, EventFeed (`:2075-2104`);
- engine texture: NodeGraph, TiledMap, Board2d, Paint2d, TextEditor (`🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2972-3012`).

Every kind has the React-compatible context-menu vocabulary mapping (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1726-1756,1875-1892`). WGPU queues generic scene pointer/move/wheel input in `🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:896-903,1088-1265`; World3d, NodeGraph, TiledMap, and Board2d own their direct event-loop route (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2294-2297`).

## Per-Surface Inventory

| Surface kind | React producer | WGPU paint and pointer route | Context menu, keyboard, cancellation | Current finding |
| --- | --- | --- | --- | --- |
| Canvas2d | `📐️Canvas2dHost/🟦️.tsx` | direct canvas paint; dedicated pointer/move/wheel bridge | whole-surface menu in both; React has gesture cancel, double-click, and catalogue DnD | **gap 1 and gap 3** |
| World3d | `🌐️World3dHost/🟦️.tsx` | direct 3-D state/pass; bespoke OS route | shell state target; relocate Escape/cancel route | ✓ current route; repaired snap work excluded |
| NodeGraph | `🕸️NodeGraph/🟦️.tsx` | engine texture; bespoke OS route | engine pick target; note editor key route | ✓ |
| TextEditor | `✏️TextEditor/🟦️.tsx` | engine texture; focused pointer/wheel route | text target; focused keys, completion and rename cancellation | ✓ |
| Table | `📊️Table/🟦️.tsx` | retained list rows | row target; shared list transfer is Escape/window/stale-generation cancellable | ✓ |
| Paint2d | `🖌️Paint2dHost/🟦️.tsx` | engine texture; dedicated pointer/wheel route | engine pick target; pointer modifiers reach its marquee path | ✓ |
| VirtualFileSystem | `🗣️Interpreter/🟦️.tsx:785-855` | retained list rows | row target, selection and VFS double-click route | ✓ |
| TiledMap | `🧭️TiledMapHost/🟦️.tsx` | engine texture; bespoke OS route | map target and host-owned cancellation | ✓ |
| Board2d | `🖥️Board2dHost/🟦️.tsx` | engine texture; bespoke OS route | engine target; Escape/Tab and pointer cancellation route | ✓ |
| IconRender | `🖼️IconRenderHost/🟦️.tsx` | direct icon renderer | whole-surface menu vocabulary; no scene-specific key producer in either host | ✓ |
| InkCanvas | `🖋️InkCanvasHost/🟦️.tsx` | direct ink renderer; resumable pointer job and wheel route | block target; React owns text/table editor keys and clipboard | **gap 2** |
| GraphTimeline | `🌳️GraphTimelineHost/🟦️.tsx` | retained list rows | deliberate whole-surface menu; scroll/click only | ✓ |
| BlockList | `🧩️BlockListHost/🟦️.tsx` | retained list rows | deliberate whole-surface menu; shared sort/transfer cancellation | ✓ |
| DiffView | `🔺️DiffViewHost/🟦️.tsx` | retained list rows | deliberate whole-surface menu; scroll/click only | ✓ |
| EventFeed | `📡️EventFeedHost/🟦️.tsx` | retained list rows | entry target; click and scroll only | ✓ |

The noninteractive/whole-surface context targets are intentional parity, not no-ops: Canvas2d, GraphTimeline, BlockList, and DiffView report empty hits/selection in the React convention and WGPU documents the same convention at `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1688-1725`. Likewise, the generic list hosts have no host-specific keyboard behavior in the React surface boundary; the absence of a separate WGPU scene key handler is not a gap. Shared retained-transfer cancellation is implemented at `:1397-1437` and invoked for Escape/window closure by Shell, so it is not included below.

## Top User-Visible Gaps

### 1. Canvas2d loses Ctrl, Meta, and Alt and cannot emit a terminal cancellation

React's gesture lane carries all four modifiers on pointer down/up and turns a cancelled gesture into `canvasPointerUp { cancelled: true }` at `🧱️elements/📐️Canvas2dHost/🟦️.tsx:425-450,690-717`. The WGPU scene ingress preserves all modifiers in its type (`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:293-303`), but the Canvas branch forwards only `modifiers.shift` (`:1182-1192`). Its wire writer then serializes `ctrl`, `meta`, and `alt` as literal `false` (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1135-1211`). A release outside the bounds merely clears local drag state and returns without a `canvasPointerUp` (`:1262-1271`). There is no `PointerCancel` variant in `SceneIntentEvent`.

Smallest neutral runtime fixture: a 100×100 interactive `canvas-2d` surface receives `PointerDown(20,20,left,{ctrl:true})`, then a captured `PointerCancel(22,22)`. The action trace must be exactly `canvasPointerDown` with `ctrl:true`, followed by `canvasPointerUp` with `cancelled:true`; it must not commit a completed drag. Current WGPU cannot represent the second input and writes `ctrl:false` for the first action.

### 2. InkCanvas has no native text/table editing or clipboard producer

React double-clicks a text or table block to open a real editing overlay (`🧱️elements/🖋️InkCanvasHost/🟦️.tsx:1320-1350`), commits text on blur and cancels it on Escape (`:810-892,1390-1410`), and commits/advances table cells on Enter or Tab while Escape cancels (`:897-935,1413-1435`). It also copies selected blocks and pastes blocks, images, or text through the canvas root (`:1454-1515,1540-1545`).

WGPU's `InkInteractionEvent` has only down/up/move and `InkInteractionJob` implements selection, move, resize, stroke, and eraser work (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:5705-5757,5868-5913,5963-6034`). The production key dispatcher offers NodeGraph, TextEditor, World3d, and Board2d before shell keys, with no Ink owner (`🧊️renderer/🦀️.rs:15023-15069`); its scene event ingress contains neither double-click nor clipboard input (`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:298-303,1197-1235`). Existing `updateBlock` output is move/resize/stroke output, not text/table editing.

Smallest neutral runtime fixture: an interactive Ink document with one text block `"a"` and one 1×1 table cell `"b"`. Double-click the text, replace it with `"x"`, then blur; double-click the cell, enter `"y"`, then press Enter. It must emit two exact `inkApplyEvents` `updateBlock` commits. Repeat text edit with Escape and require no document action. A separate two-block Copy/Paste assertion can reuse the same document: the paste must add re-identified blocks at the viewport centre. None of these routes has a WGPU producer today.

### 3. Canvas2d cannot accept React's catalogue drag-and-drop input

The React host accepts only the catalogue MIME, throttles `canvasDragOver`, clears it with `canvasDragLeave`, and dispatches `canvasDrop` with the raw payload and surface coordinates (`🧱️elements/📐️Canvas2dHost/🟦️.tsx:872-905,947-959`). It also emits `canvasDoubleClick` (`:719-720`). No production WGPU source contains any of those four Canvas action producers; its only Canvas input cases are pointer down/up/move and wheel (`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1169-1195`). This is separate from the repaired retained Table/BlockList transfer lane.

Smallest neutral runtime fixture: one 100×100 `canvas-2d` surface and one catalogue item with a valid catalogue MIME/payload. Drag it to `(24,36)`, then drop. The reference action trace is `canvasDragOver {x:24,y:36,width:100,height:100,types:[mime]}` followed by `canvasDrop` carrying the same coordinates and payload. Add two left click/release pairs at `(24,36)` inside the same double-click interval and require `canvasDoubleClick`. WGPU currently emits no corresponding action.

## Lower-Priority Observation

EventFeed formats a nonzero timestamp as local time in React (`📡️EventFeedHost/🟦️.tsx:30-35,133`) but WGPU explicitly formats UTC wall-clock time (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3434-3441,3537-3542`). A controlled non-UTC locale fixture should decide whether the component contract intends local time; this is not promoted above the missing interaction producers because the payload has no explicit locale/time-zone field.

## Verification Boundary

This is source evidence only. The next implementation packet should add one language-neutral fixture for each ranked gap, a React oracle that drives the actual host, and a WGPU law that crosses the production input route. Runtime acceptance remains root-owned.
