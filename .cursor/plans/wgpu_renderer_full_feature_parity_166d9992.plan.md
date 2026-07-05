---
name: Wgpu Renderer Full Feature Parity
overview: "Bring the wgpu renderer to full pre-migration feature parity across all six world3d plugins (cad, puzzle3d, puzzle5d, procedural3d, shooting, lowpoly): live dynamic tools/window-engagement bridging, full UV canvas rendering and interaction for lowpoly, component-level (vertex/edge/face) picking and marquee, component selection visuals, correct selection-JSON parsing, paint-on-mesh-surface in the 3D viewport, and gumball targeting fixes."
todos:
  - id: bridge-tools-engagements
    content: Add tools()/window_engagements() to wgpu PluginBridgeEntry and cache them in ShellState.refresh_ui
    status: completed
  - id: footer-dynamic-tools
    content: Render active_tools ToolNode tree in wgpu render_footer with collection expand/collapse state
    status: completed
  - id: engagement-rail-live
    content: Use cached window_engagements over static kind.engagement in render_window_engagement_rail; wire engagementInput on_change
    status: completed
  - id: uv-canvas-layers
    content: Extend CanvasLayer with dataUrl/points/seams and render paint texture image + dashed-seam UV wireframe + checkerboard in scenes.rs
    status: completed
  - id: uv-canvas-interaction
    content: Fix UV canvas pointer coord mapping, wire paintStrokeBegin/End and canvasWheel, refresh_ui on mode switch
    status: completed
  - id: mesh-component-data
    content: Extend Mesh3d with CPU-side face_ids/vertex_ids/edge_positions/edge_ids/uvs threaded through mesh ingestion
    status: completed
  - id: component-picking
    content: Add pick_component_at + worldPick dispatch for vertex/edge/face granularity in world3d.rs
    status: completed
  - id: component-marquee
    content: Extend marquee selection to project vertex/edge/face hits per granularity with live preview
    status: completed
  - id: component-overlays
    content: Render wireframe/hover/selection overlays for components via line_draws/translucent_draws
    status: completed
  - id: selection-json-parity
    content: Extend WorldSelectionRecord (granularity, componentIds, transformTool, interactionMode, gumballTarget) and fix transform-tool/mode field mixup and gumball centroid/mode commit
    status: completed
  - id: paint-on-mesh
    content: Decode paint texture to wgpu texture, add UV-mapped textured mesh draw path, dispatch paintAt via ray-uv hit test with stroke begin/end
    status: completed
  - id: verify-wgpu-parity
    content: Rust tests, wasm rebuild, wgpu e2e sweep for all six plugins, manual browser verification, update ticket
    status: completed
isProject: false
---

# Wgpu Renderer Full Feature Parity

## Context

The wgpu renderer already has object-level orbit/pan/zoom/hover/select/marquee/gumball parity across all six world3d plugins, completed by the already-closed [world3d_wgpu_full_interaction_parity_77e5fa4b.plan.md](.cursor/plans/world3d_wgpu_full_interaction_parity_77e5fa4b.plan.md). That plan explicitly deferred "face/vertex sub-object picking and paint mode for lowpoly," "context menu on plain right-click," and "touch gestures." Since then, the just-completed React-renderer lowpoly parity work added several plugin-side features (component picking commands, live window engagements, editUndo/Redo, per-stroke paint undo) that the wgpu shell never surfaces, because wgpu talks to plugins over a narrower bridge than React's.

This plan closes the framework-wide bridge gap (tools/engagements) for all six plugins, then closes the lowpoly-specific gaps (UV canvas, component picking/marquee, paint-on-mesh, selection-JSON parity) required for "full parity."

Context menu on right-click and touch gestures remain explicitly deferred, matching the prior plan (not requested in scope).

## Phase 0 — Wire `tools()` / `windowEngagements()` through the wgpu plugin bridge (all 6 plugins)

The JS side is already fully ready: `PluginWasmHandle.tools`/`windowEngagements` and `pluginHandleForBridge()` already expose both (`[framework/core/js/index.ts](framework/core/js/index.ts)` lines 645-649, 727-732), and the WASM exports `semio_plugin_tools`/`semio_plugin_window_engagements` already exist (`[framework/plugin/rs/plugin_runtime.rs](framework/plugin/rs/plugin_runtime.rs)` lines 119-150, 246-253). Only the Rust-side `PluginBridgeEntry` in wgpu and the shell's use of it are missing.

1. Add `tools()` and `window_engagements()` async methods to `PluginBridgeEntry` in [framework/renderer/wgpu/rs/plugin_bridge.rs](framework/renderer/wgpu/rs/plugin_bridge.rs), mirroring the existing `render()` method (lines 104-129): call the JS `tools`/`windowEngagements` functions on `self.handle`, await the promise, and deserialize into `Vec<ToolNode>` / `HashMap<String, WindowEngagement>`.
2. In `ShellState::refresh_ui` ([framework/renderer/wgpu/rs/shell.rs](framework/renderer/wgpu/rs/shell.rs) lines 522-585), after the existing per-window `plugin.render(...)` loop, call `plugin.tools(instance_id, view_state)` and `plugin.window_engagements(instance_id, view_state)` once per refresh; cache results in new `ShellState` fields (`active_tools: Vec<ToolNode>`, `window_engagements: HashMap<String, WindowEngagement>`).
3. Replace the static `kind.engagement` read in `render_window_engagement_rail` ([shell.rs](framework/renderer/wgpu/rs/shell.rs) line 4006) with `self.window_engagements.get(&kind.id).or(kind.engagement.as_ref())`, matching React's `windowEngagementsByKind[kind.id] ?? kind.engagement` fallback pattern (`os-shell.tsx` line 1643).
4. Wire the `engagementInput` on-change command: `render_engagement_input` ([shell.rs](framework/renderer/wgpu/rs/shell.rs) lines 4171-4177) currently sets `on_change: None`; dispatch `spec.on_change` through the input state like other text fields.
5. Render `self.active_tools` in `render_footer` ([shell.rs](framework/renderer/wgpu/rs/shell.rs) lines 2897-2971): recurse the `ToolNode` tree into `ChromeGroupItem`s — `Separator` → visual divider, `Button`/`Toggle` → `ChromeGroupItem` (icon/label/pressed), `Collection` → a toggle `ChromeGroupItem` that expands its leaf children inline when open, tracked via a new `tool_collection_expanded: HashMap<String, bool>` field (follow the existing `engagement_expanded` precedent at [shell.rs](framework/renderer/wgpu/rs/shell.rs) line 212). Keep the existing app-label chip; keep the studio-mode generic undo/redo/checkpoint chip only when `session.app.controller_id == S_PLAY_CONTROLLER_ID` (unchanged), and render `active_tools` alongside it for all plugins.
6. This automatically surfaces lowpoly's `editUndo`/`editRedo`/paint-undo tool buttons (already implemented in `edit_tools()`/`paint_tools()`, `lowpoly/plugin/rs/lib.rs` lines 1143-1144, 1213-1214) in the wgpu footer — no plugin-side change needed. Verify this explicitly.

## Phase 1 — UV canvas full rendering + interaction parity (lowpoly)

`CanvasLayer` in [framework/renderer/wgpu/rs/scenes.rs](framework/renderer/wgpu/rs/scenes.rs) (lines 807-834) only has generic rect/line fields and drops `dataUrl`/`points`/`seams`, so `render_canvas_2d` (lines 836-893) renders colored placeholder boxes instead of the paint texture and UV wireframe.

1. Extend `CanvasLayer` with `data_url: Option<String>`, `points: Option<Vec<f64>>` (flat x,y pairs), `seams: Option<Vec<bool>>`.
2. Add image decode + GPU texture upload for `dataUrl` layers (base64 PNG from lowpoly's paint texture) — decode once per unique `dataUrl` and cache by a hash/id (mirror the mesh GPU cache pattern in `MeshGpuStore` at [ui/wgpu/rs/draw.rs](ui/wgpu/rs/draw.rs) lines 405-424), then draw via a textured quad in `render_canvas_2d`.
3. Add `kind == "polyline"` rendering: connect `points` pairs with `ctx.draw.push_line`, using dashed segments where the parallel `seams[i]` is true (mirror React's dash logic in `canvas-2d-host.tsx` lines 146-166) plus a checkerboard background under the UV wireframe.
4. Fix pointer coordinate mapping: `render_canvas_2d`'s hit target (lines 885-892) currently passes raw screen pixels to `canvasPointerDown`; convert through the existing `Viewport::world_to_screen`/inverse so lowpoly's UV-space fallback mapping (`lowpoly/plugin/rs/lib.rs` lines 1922-1927) receives canvas-world coordinates like React's `toCanvasCoords` (`canvas-2d-host.tsx` lines 200-220).
5. Dispatch `paintStrokeBegin`/`paintStrokeEnd` around the canvas pointer-down/up drag lifecycle (mirror React `canvas-2d-host.tsx` lines 297-302), and add a `canvasWheel` handler path if none exists for zoom, matching React's zoom-on-wheel.
6. Ensure a mode switch (edit ↔ paint) triggers `refresh_ui()` so `interactionMode`/UV window content updates — currently the navbar mode click only mutates local view state (`shell.rs` lines 1383-1388); route it through the same op-apply path used by command dispatch (`shell.rs` line 815).

## Phase 2 — Component-level picking data pipeline

Currently `Mesh3d` ([ui/wgpu/rs/scene3d.rs](ui/wgpu/rs/scene3d.rs) lines 328-335) and mesh ingestion (`framework/renderer/wgpu/rs/world3d.rs` lines 876-882, 1425-1428) only carry positions/normals/indices — `MeshData`'s `face_ids`, `vertex_ids`, `edge_positions`, `edge_ids` ([framework/core/rs/mesh.rs](framework/core/rs/mesh.rs) lines 20-28) are dropped.

1. Extend `Mesh3d` with optional CPU-side component arrays: `face_ids: Vec<u32>` (parallel to triangles), `vertex_ids: Vec<u32>`, `edge_positions: Vec<f32>`, `edge_ids: Vec<u32>`, `uvs: Vec<f32>` (needed for Phase 5 paint-on-mesh). Thread them through `store_mesh`/`ingest_glb_mesh` in `world3d.rs`.
2. This stays CPU-side only (no new GPU buffers/pick render target needed) — component hit-testing reuses the existing ray-triangle intersection already used for object picking (`ray_pick_instance`), extended to: face hit → triangle index → `face_ids[triangle_index]`; vertex hit → screen-space proximity test against projected vertex positions (mirror `pick_gumball_handle_at`'s ray-to-point style tests); edge hit → ray-to-segment distance against `edge_positions` pairs (mirror the gumball axis ray-to-segment code already in `world3d.rs` per the prior plan's Phase C).

## Phase 3 — `worldPick` dispatch + component marquee (lowpoly)

1. Add a `pick_component_at` function in [framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs) analogous to `pick_instance_at` (lines 1612-1631) that, when `interactionMode` (from `WorldSelectionRecord`, see Phase 5) indicates component editing, hit-tests per current `granularity` (vertex/edge/face) using Phase 2's data and returns a component id.
2. Add `pick_component_command`/extend `pick_select_command` (lines 1457-1475) to dispatch `worldPick` with `{granularity, id, merge}` instead of `worldSelect` when in component mode, mirroring React's dispatch at `world-3d-host.tsx` line 1158.
3. Extend marquee: `marquee_select_command` (lines 1478-1517) → `screen_select_instances` (`scene3d.rs` lines 689-729) currently only tests whole-instance AABBs/triangle centroids. Add a component-granularity path that projects each vertex/edge-midpoint/face-centroid to screen space per current instance and tests against the marquee rect (mirror React's `resolveMarqueeComponentIds`, `world-3d-host.tsx` lines 828+), with a live preview set updated during drag (mirror `updateMarqueePreview`, lines 1031-1054) before committing on release.

## Phase 4 — Component selection visuals

React overlays hovered/selected components (`buildFaceOverlayGeometry`, edge line proxies, `world-3d-host.tsx` lines 214-254); wgpu's `line_draws`/`translucent_draws` (used for gumball rings/attraction lines, `world3d.rs` lines 1011-1033) currently has no equivalent.

1. When `interactionMode == "paint"` or the active mode's granularity is not "object", build a wireframe line-draw list from each visible mesh's `edge_positions` (transformed by instance model matrix) each frame.
2. When a face/vertex/edge is hovered or selected (from the component selection state), push a translucent triangle draw (face) or thicker/highlighted line draw (edge) or small point marker (vertex) using the theme's selection/hover colors, matching React's highlight styling.

## Phase 5 — Selection-JSON parity fix

`WorldSelectionRecord` in wgpu ([framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs) lines 53-60) only parses `method`/`mode`/`ids`/`hoveredId`, and wgpu misreads `selection.mode` as the transform tool (lines 896-898) when lowpoly's `mode` field is actually the merge mode ("replace"/"add"/"toggle") — the transform tool lives in `transformTool`.

1. Extend `WorldSelectionRecord` to also parse `granularity`, `component_ids`, `transform_tool`, `interaction_mode`, `gumball_target` (matching the fields lowpoly emits in `world_selection_json_for`, `lowpoly/plugin/rs/lib.rs` lines 450-483, and what React reads at `world-3d-host.tsx` line 713).
2. Fix the transform-tool read to use `selection.transform_tool` instead of `selection.mode` (lines 896-898).
3. Use `gumball_target` (when present) instead of averaging all selected instance origins in `selection_centroid` (`world3d.rs` lines 278-297), so component-level gumball positioning matches React.
4. Use `interaction_mode` to drive whether picking dispatches `worldSelect` (object) vs `worldPick` (component) in Phase 3, and whether Phase 4's overlays render.
5. Fix the hardcoded `"mode": "mesh"` in gumball commit commands (lines 667-675) to derive from the current selection's granularity/target instead.

## Phase 6 — Paint-on-mesh-surface in the 3D viewport

Painting directly on the model (not just the UV window) needs a UV-mapped textured mesh render path plus a `paintAt` ray-hit dispatch.

1. Decode `paint_texture_base64` (`MeshData`, [framework/core/rs/mesh.rs](framework/core/rs/mesh.rs) line 32) into a wgpu texture, cached/invalidated similarly to Phase 1's UV image cache; upload per-vertex UVs (Phase 2) into the mesh vertex buffer alongside position/normal.
2. Add a textured-mesh draw path in `ui/wgpu/rs/draw.rs`/`scene3d.rs` — extend the existing textured-draw pipeline used for reference-plane images (`world3d.rs` lines 1127-1164) to also accept mesh vertex/index buffers with UVs, sampling the paint texture instead of a flat color when present.
3. On pointer-down/drag with `interactionMode == "paint"`, ray-hit the mesh (reuse the object ray-triangle intersection, returning barycentric weights), interpolate UV at the hit point via `MeshData.uvs`, and dispatch `paintAt {objectId, u, v}` (mirror React `world-3d-host.tsx` line 1013), plus `paintStrokeBegin`/`paintStrokeEnd` around the drag gesture (mirror lines 1104, 1167) so undo snapshots per-stroke (already implemented plugin-side) work correctly from the wgpu path too.

## Verification

1. `cargo test -p ui_wgpu -p semio-framework-renderer-wgpu -p lowpoly-plugin` for new frustum/pick/UV-layer/selection-parsing unit tests.
2. Rebuild wgpu WASM (`bun ./framework/renderer/wgpu/script.ts wasm`).
3. Run the existing wgpu playground e2e sweep (`.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts`) with body-content assertions for all six world3d plugins plus lowpoly's UV window.
4. Manual browser verification (cursor-ide-browser) for lowpoly under the wgpu renderer: footer shows dynamic edit/paint tool trees with working undo/redo, window engagement rail updates live, UV window renders the paint texture + dashed-seam wireframe with pan/zoom and paints correctly, 3D viewport supports vertex/edge/face click-select and marquee with visible overlays, painting directly on the mesh surface works, and gumball operates at the correct component-level centroid.
5. Spot-check the other five plugins (cad, puzzle3d, puzzle5d, procedural3d, shooting) still orbit/pan/zoom/select/marquee/gumball correctly and now show their dynamic tool trees/live engagement rails (Phase 0 is shared infrastructure).
6. Update the ticket (`.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS/important.md`) with a summary of wgpu parity work and files touched.
