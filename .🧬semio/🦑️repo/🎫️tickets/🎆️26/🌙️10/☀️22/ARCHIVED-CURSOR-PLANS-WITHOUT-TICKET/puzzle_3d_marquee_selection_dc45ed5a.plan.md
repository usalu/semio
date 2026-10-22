---
name: Puzzle 3D marquee selection
overview: Add left-drag marquee selection (Rectangle + Lasso) to the puzzle 3D viewport with modifier-driven modes (default/subtractive/additive/invertive), CAD-style crossing-vs-window direction semantics, three window-header kind toggles (objects/vortices/attractions) wired to existing selectableKinds, and remapped camera controls (orbit/pan/zoom on the right button).
todos:
 - id: ticket
   content: Open repo MCP ticket associated to the best matching goal (read repo://goals first).
   status: completed
 - id: camera
   content: "Remap OrbitGated camera controls: orbit/pan/zoom on right button (shift=pan, alt=zoom), free the left button, conditional contextmenu preventDefault."
   status: completed
 - id: modes
   content: Rename SelectionMode to default/additive/subtractive/invertive across react+play; add marqueeModeFromModifiers helper and update mergeSelection/mergeIdList, settings select, defaults.
   status: completed
 - id: marquee-helpers
   content: Add pure marquee geometry helpers (rect/polygon predicates, marqueeIsCrossing, marqueeSelectionFromCandidates) in a new Marquee subregion.
   status: completed
 - id: marquee-runtime
   content: Add registry marquee state/store + begin/update/commit methods (project objects AABB, vortices, attraction endpoints), MarqueeBridge gesture component, and SVG overlay in Canvas3D.
   status: completed
 - id: play-toggles
   content: Add three window-header kind toggles wired to selectableKinds plus selectionMethod (Rectangle/Lasso) control and command in the play controller; extend snapshot.
   status: completed
 - id: wire-props
   content: Thread selectionMethod + marqueeSelectableKinds through CanvasProps/PlayCanvasProps/Inner/RegistryProvider and the host Puzzle3dPlayViewportHost.
   status: completed
 - id: tests
   content: Extend existing vitest blocks in react/index.tsx and play/index.ts for the new helpers, measures, and commands; run and confirm green.
   status: completed
 - id: close
   content: Close the repo MCP ticket with a summary and the list of touched files.
   status: completed
isProject: false
---

# Puzzle 3D Marquee Selection

## Context

The viewport lives in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) (region `🎬️Viewport`). It already has a full selection store (`SelectionSnapshot`, `mergeSelection`, `RegistryProvider`), per-mesh click picking, hover, and a `SelectionMissBridge`. Camera uses `OrbitControls` with `LEFT: MOUSE.ROTATE` at line 3789. The play controller in [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) already holds `selectableKinds` (objects/vortices/attractions), `selectionMode`, and a window-header measure builder (`lodMeasures()`), and filters every emitted selection via `filterSelectionByPlaygroundKinds`. The host wiring is in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) (`Puzzle3dPlayViewportHost` → `PlayCanvas`).

## 1. Remap camera controls to the right button

In `OrbitGated` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) line ~3776):

- Set `mouseButtons={{ LEFT: undefined, MIDDLE: MOUSE.DOLLY, RIGHT: MOUSE.ROTATE }}` so left is free for selection.
- Add a capture-phase `pointerdown` listener on `gl.domElement` that, for `button === 2`, sets `controls.mouseButtons.RIGHT` before OrbitControls reads it: `shiftKey → MOUSE.PAN`, `altKey → MOUSE.DOLLY`, else `MOUSE.ROTATE`.
- Track whether the right pointer moved past a small threshold; on `contextmenu`, only `preventDefault()` when a drag occurred (so a plain right-click leaves the native menu available; a real context-menu UI is out of scope and noted as a follow-up hook). The existing `onContextMenu` in `Canvas3D` (line ~4828) becomes conditional via a shared ref.

## 2. Selection modes: rename + modifier mapping

- Rename `SelectionMode` (line 77) union to `"default" | "additive" | "subtractive" | "invertive"`, updating `mergeSelection`/`mergeIdList` (lines 299-341), the settings select and `setSelectionMode` validation in play (lines ~887, ~1353), and `PUZZLE_3D_PLAY_IDLE_SNAPSHOT`/defaults. `invertive` keeps the symmetric-difference (current `toggle`) behavior; `default` keeps replace (current `single`).
- Add pure helper `marqueeModeFromModifiers({shiftKey, ctrlKey, metaKey})` → mode, encoding the requested order: none=`default`, ctrl=`subtractive`, shift=`additive`, ctrl+shift=`invertive`.

## 3. Marquee gesture + hit testing

Add a `🔖️Marquee` subregion in `🎬️Viewport`.

Pure helpers (testable, mirroring existing `pickNearestScreenVortex` style):

- `pointInScreenRect`, `screenRectContainsRect` (window), `screenRectIntersectsRect` (crossing), `pointInPolygon`, `polygonContainsRect`, `segmentIntersectsPolygon`.
- `marqueeIsCrossing(startX, endX)` → `endX < startX` means crossing (partial); otherwise window (full-enclosure).
- `marqueeSelectionFromCandidates({ rectOrPolygon, crossing, method, kinds, candidates })` returning a `SelectionSnapshot` of objects/vortices/attractions whose projected footprints satisfy crossing/window.

Registry additions in `RegistryProvider` (reusing `attractionThreeRef` camera/gl, `raycasterRef`, `ndcRef`, and `objectGroupMap`/`vortexGettersRef`):

- `collectObjectEntries()` → `[id, group][]` (project group `Box3` corners to screen for object footprints).
- Marquee state (`active`, `method`, `start`, `current`, `path` for lasso) stored in a small external store (like `createSelectionSnapshotStore`) so the DOM overlay can subscribe without re-rendering the scene.
- `beginMarquee/updateMarquee/commitMarquee(clientX, clientY, modifiers)`: project objects (AABB→screen rect), vortices (`getVortexWorld`→point), and attraction endpoints (from object-state `store.getAttractions()`, available via the bridged context) to screen, build the new snapshot with `marqueeSelectionFromCandidates`, then merge with current selection via `mergeSelection`-style logic using `marqueeModeFromModifiers`, and `publishSelection`.

`MarqueeBridge` component (inside `Inner`, mirroring `AttractionWindowBridge` at line ~3871):

- On left `pointerdown` (button 0) with no attraction gesture active, record start; on `pointermove` past threshold mark active and `updateMarquee`; on `pointerup` `commitMarquee`. A left click below threshold falls through to existing per-mesh click picking (unchanged).
- Gate by `selectableKinds` and skip when `attractionDragActive`/`attractionIndirectPickAwait`.

Overlay: a DOM element in `Canvas3D`'s outer div (line ~4824) subscribing to the marquee store, drawing an SVG `<rect>` (Rectangle) or `<polyline>`/`<path>` (Lasso) with `pointer-events: none`.

## 4. Window-header kind toggles + method control (play)

In [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts):

- Extend `lodMeasures()` / `rebuildShellMode()` (lines ~658-705) to also emit three `kind: "toggle"` `WindowMeasure`s labeled Objects/Vortices/Attractions, `pressed: this.selectableKinds[kind]`, `onChange` → existing `toggleSelectableKind` command with `args: { kind }` (single source of truth with the toolbar `buildPlaygroundBrowseSelectionTools`). Add a `selectionMethod: "rectangle" | "lasso"` field plus a `setSelectionMethod` command and a fourth toggle/select measure for the method (default Rectangle).
- Add `selectionMethod` and `selectableKinds` to `Puzzle3dPlaySnapshot` and `rebuildSnapshotCache`.

## 5. Wire props through to the canvas

- Add to `CanvasProps` and `PlayCanvasProps` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) lines ~462, ~4719): `selectionMethod?: "rectangle" | "lasso"` and `marqueeSelectableKinds?: { object: boolean; vortex: boolean; attraction: boolean }`, threading them through `Inner` → `RegistryProvider` and into the `MarqueeBridge`.
- In `Puzzle3dPlayViewportHost` ([framework/.../react/index.tsx](framework/product/playground/renderer/react/index.tsx) line ~995) pass `snap.selectionMethod` and `snap.selectableKinds` to `PlayCanvas`.

## 6. Tests

Extend the existing `import.meta.vitest` blocks only (no new files):

- In [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) (region near line 4880): add `describe` blocks for `marqueeModeFromModifiers`, `marqueeIsCrossing`, the rect/polygon predicates, and `marqueeSelectionFromCandidates` (crossing selects partially-overlapping items; window requires full enclosure; kinds gating respected).
- In [puzzle/3d/play/index.ts](puzzle/3d/play/index.ts) (region near line 1451): assert window-header measures include the three kind toggles + method, and that `setSelectionMethod`/`toggleSelectableKind` update the snapshot.
- Run via `bun`/`nx` (`vitest.config.ts` present in both projects) and confirm green before closing the ticket.

## Process

Open a repo MCP ticket first (associate with the most appropriate goal from `repo://goals`), keep all temp artifacts in the ticket folder, structure new code with regions/subregions, and close the ticket with a summary when done.

## Open follow-up (non-blocking)

A true right-click context-menu UI does not exist in puzzle 3D today; this plan only stops orbit from swallowing a plain right-click and leaves a hook. Building the menu itself can be a separate ticket unless you want it included.
