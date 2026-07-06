---
name: Lowpoly Full Parity Restoration
overview: "Restore remaining large-scale parity gaps between the old TypeScript lowpoly editor and the current Rust/React port: no camera navigation in the React 3D viewport, a non-functional UV paint window, missing edge/component picking and marquee, and missing mesh-edit undo — plus several smaller polish gaps."
todos:
  - id: orbit-camera
    content: Add WorldOrbitGated + OrbitControls (pan/zoom/rotate) to World3dHost in framework/renderer/react/components/world-3d-host.tsx, wired to setCamera command
    status: completed
  - id: uv-canvas-image-polyline
    content: Extend JsonLayersCanvasSession in canvas-2d-host.tsx to render image (dataUrl) and polyline (points+seams) layers, plus add pan/zoom to the UV canvas
    status: completed
  - id: edge-picking
    content: Enable edge picking (remove raycast=null) and dispatch worldPick with granularity edge in world-3d-host.tsx
    status: completed
  - id: component-marquee
    content: Extend marquee selection in world-3d-host.tsx (and world3d.rs if needed) to project vertex/edge/face hits per current granularity with live add/remove preview during drag
    status: completed
  - id: edit-undo
    content: Add edit-mode (mesh operation) undo/redo history in lowpoly/plugin/rs/lib.rs mirroring paint undo but snapshotting doc/fixture state
    status: completed
  - id: paint-undo-per-stroke
    content: Fix paint undo to snapshot once per stroke (on stroke begin) instead of per sample in paint_at_uv
    status: completed
  - id: live-engagement
    content: Recompute lowpoly_window_engagement() from current envelope state on each panel refresh instead of once at app build
    status: completed
  - id: catalogue-polish
    content: Rename Box primitive label back to Cube and restore per-item description in catalogue tree
    status: completed
  - id: inspector-selection-summary
    content: Restore combined readonly Selection summary field (targets label + selected count) in inspector
    status: completed
  - id: layers-polish
    content: Add selected_ids highlight and opacity/blendMode description to build_layers_tree
    status: completed
  - id: document-object-select
    content: Allow whole-mesh selection via toggleSelectionTarget on document object rows plus default_open for the active object
    status: completed
  - id: flat-shading
    content: Apply flatShading based on smoothShading flag in React mesh material
    status: completed
  - id: engagement-input-wiring
    content: Wire on_change/engagementInput handler for the window engagement command-bar text field
    status: completed
  - id: verify-full-parity
    content: Extend Rust tests, rebuild wasm, run React E2E sweep, manually verify all fixes live in browser, update ticket
    status: completed
isProject: false
---

# Lowpoly Full Parity Restoration

A follow-up audit (beyond the already-completed document/selection/colors/gumball/footer work) found several **large, previously unaddressed** gaps. These explain the user's "large areas are still missing" feedback — the most severe ones make the React-mode 3D/UV viewports barely usable, independent of the footer/document work already done.

## Tier 1 — Viewport is effectively broken without these

### 1. No camera navigation in the shared React 3D viewport
[framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx) renders `WorldCanvas` with a **fixed** `cameraPosition`/`cameraFov` from the scene JSON (line 966) and never mounts `OrbitControls`/`WorldOrbitGated`. There is no wheel/drag/pan handling anywhere in the file. This means **orbit, pan, and zoom do not work at all** in React mode for every plugin that goes through this shared host (lowpoly, puzzle3d, puzzle5d, shooting, procedural3d), while the wgpu renderer already has working orbit camera (`framework/renderer/wgpu/rs/world3d.rs:1410-1420`).

Fix: compose `WorldOrbitGated` + `OrbitControls` (from `@semio-tech/infinite-world-r3f`) into `World3dHost`, following the existing pattern in [cad/renderer/js/index.tsx](cad/renderer/js/index.tsx) (`InteractionCanvas`, lines ~3563-3619, and the `WorldOrbitCameraViewRig`/mouse-button helpers around lines 1804-1936). Persist camera state via the existing `setCamera` command (`lowpoly/plugin/rs/lib.rs:1746`), which is defined but never invoked from the host today.

### 2. UV paint window renders nothing useful
Rust emits two layer kinds for the UV canvas (`uv_canvas_layers_json`, [lowpoly/plugin/rs/lib.rs:534-580](lowpoly/plugin/rs/lib.rs)):
- `"kind": "image"` with a base64 `dataUrl` (the paint texture)
- `"kind": "polyline"` with a `points` array and a `seams` bitmask (the UV wireframe, dashed on seam edges)

But [framework/renderer/react/components/canvas-2d-host.tsx](framework/renderer/react/components/canvas-2d-host.tsx)'s `JsonLayersCanvasSession.renderFrame()` (lines 63-121) only understands bounding-box rectangles and two-point `x0/y0/x1/y1` lines — it has no handling for `image`/`dataUrl` or `polyline`/`points`/`seams`. The paint texture and UV wireframe are **completely invisible**, and there's no pan/zoom for the UV canvas.

Fix: extend `JsonLayersCanvasSession` to draw `image` layers (decode/draw the `dataUrl` via an `<img>`/`ImageBitmap`) and `polyline` layers (stroke each point pair, dashed when the corresponding `seams` entry is set), and add pan (drag) / zoom (wheel) camera controls to the UV canvas host, matching the old `LowpolyUvCanvas` behavior (`f8376e848:lowpoly/react/index.tsx:1451-1624`).

### 3. Edge picking disabled, component-level marquee missing
In `World3dHost`, edges are rendered with `raycast={() => null}` ([world-3d-host.tsx:504](framework/renderer/react/components/world-3d-host.tsx)) so they can never be clicked/picked, while vertices and faces are pickable. Marquee-drag selection only computes whole-object hits (`instanceCenterInMarquee`, lines ~748-767) — there's no per-vertex/edge/face marquee projection like the old `LowpolyMarqueeBridge` (`f8376e848:lowpoly/react/index.tsx:900-1070`), and no live add/remove preview while dragging.

Fix: give edges a real (thickened) raycastable proxy and dispatch `worldPick` with `granularity: "edge"` on click; extend the marquee resolver to project vertex/edge/face positions to screen space per the current `selectionMode`/granularity and support live preview highlighting during drag.

## Tier 2 — Correctness bugs

### 4. Mesh-edit operations have no undo
Old lowpoly registered `createLowpolyAppVcsHandler()` (`internal.ts:423-432`, wired at `index.ts:1152`) giving document-level undo for edit-mode mesh ops (extrude/bevel/inset/etc). The new Rust plugin has no VCS registration for edit mode at all — only paint has an (ad hoc) undo stack. Fix: add an edit-mode undo/redo history in `lowpoly/plugin/rs/lib.rs` (snapshot `doc`/fixture state before each mutating mesh command) and expose `undo`/`redo` the same way paint does, or register the existing app-level VCS handler if the framework now exposes one for other ported plugins (check `writer/rs/document_vcs.rs` for the current idiom to mirror).

### 5. Paint undo pushes on every paint sample instead of per stroke
`paint_at_uv` calls `push_paint_undo` before every stroke point ([lowpoly/plugin/rs/lib.rs:1360-1361](lowpoly/plugin/rs/lib.rs)) instead of once per stroke (old behavior snapshotted only on stroke begin/end, `f8376e848:lowpoly/react/index.tsx:1888-1914`). Fix: only push an undo snapshot on `paintStrokeBegin` (or first sample of a stroke) so one undo = one stroke.

### 6. Window engagement rail is frozen, not live
`lowpoly_window_engagement()` is only invoked once in `create_lowpoly_app()` at app-build time ([lowpoly/plugin/rs/lib.rs:1928](lowpoly/plugin/rs/lib.rs)), whereas the old editor rebuilt `windowEngagement()` on every relevant state change (`index.ts:706-748`) so the status text (e.g. "3 selected", active transform tool) stayed current. Fix: recompute the engagement payload from current `envelope` state whenever the panel/body refreshes, not just at manifest build time.

## Tier 3 — Smaller polish gaps

- **Catalogue**: old primitive label was "Cube" (`index.ts:304`), new shows "Box" ([lowpoly/plugin/rs/lib.rs:46](lowpoly/plugin/rs/lib.rs)); old also had a per-item `description` (`index.ts:319`) that's dropped in `tree_item_with_command` (lines 740-745).
- **Inspector**: old had a combined readonly "Selection" field showing `"{targets label} · {N} selected"` (`index.ts:341-342`); new only shows `"Selection Mode"` ([lib.rs:850-854](lowpoly/plugin/rs/lib.rs)) — restore the combined summary.
- **Layers panel**: old highlighted the active layer via `selectedIds` and showed `opacity · blendMode` description (`index.ts:490-496`); new `build_layers_tree` (lib.rs:761-786) has neither.
- **Document**: old object rows were selectable as a whole-mesh target (`toggleSelectionTarget`, `index.ts:250-276`) and auto-expanded the active object (`defaultOpen`, `index.ts:277`); new object rows only dispatch `setActiveObject` ([lib.rs:706](lowpoly/plugin/rs/lib.rs)) with no `default_open`.
- **Smooth shading**: old mesh material set `flatShading={!object.smoothShading}` (`index.tsx:789`); new `PaintTexturedMesh` never sets `flatShading` (world-3d-host.tsx:~298), so the smooth-shading toggle has no visual effect in React mode.
- **Engagement text input**: `on_change: None` for the command-bar field ([lib.rs:964](lowpoly/plugin/rs/lib.rs)) means typed text never round-trips into local UI state on the wgpu shell — wire an `on_change`/`engagementInput` handler.

## Verification

- Extend Rust unit tests in `lowpoly/plugin/rs/lib.rs` for: edit-mode undo/redo, per-stroke paint undo, engagement live status, catalogue/inspector/layers field content.
- Run `cargo test -p lowpoly-plugin --lib` and the full workspace build.
- Rebuild the lowpoly WASM plugin and re-run the React E2E sweep (`verify-react-playgrounds-e2e.ts --plugin lowpoly`).
- Manually verify in a live browser: orbit/pan/zoom in the 3D viewport, UV window showing the paint texture + wireframe with visible seams and its own pan/zoom, edge picking, component-level marquee with live preview, undo/redo for a mesh edit and for a paint stroke, and a live-updating engagement rail.
- Update the ticket notes and close/reopen as appropriate per the ticket workflow.
