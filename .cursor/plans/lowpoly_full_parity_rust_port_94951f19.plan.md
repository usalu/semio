---
name: Lowpoly Full Parity Rust Port
overview: Restore lowpoly's full mesh-editing experience (real Rock example, working toolbar, real edit operations, vertex/edge/face picking, transform gizmo, and paint mode) by porting the existing kernel_3d_mesh-backed logic into the Rust plugin, and by adding a generic AppTools/toolbar concept to the Rust framework so other technologies can adopt it later.
todos: []
isProject: false
---


# Restore Lowpoly Full Feature Parity on the Rust Framework

## Root causes (confirmed by investigation)

1. **Empty default example**: [lowpoly/example/default.lowpoly.json](lowpoly/example/default.lowpoly.json) ships `"objects":[]`, while the real mesh kernel wrapper [lowpoly/core/rs/lib.rs](lowpoly/core/rs/lib.rs) `default_fixture()` already builds a genuine "Rock" object (ico-sphere with an extruded face, backed by a real `HalfedgeMesh`). The shipped example was hand-authored empty during the migration and never regenerated.
2. **No toolbar concept exists in the Rust framework at all.** `AppBuilder` ([framework/plugin/rs/app.rs](framework/plugin/rs/app.rs)) has no `.tools(...)`, `ModeDefinition` ([framework/core/rs/ui.rs](framework/core/rs/ui.rs)) is `{id, label}` only, and the React shell's `Footer.toolbar` slot ([ui/js/react/index.tsx](ui/js/react/index.tsx)) is never populated. The old TS `AppTools`/`ToolLeaf`/`toolCollection` system that drove lowpoly's model/paint toolbars was deleted wholesale in the migration and never re-implemented in Rust.
3. **The new plugin's mesh model is a fake.** [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs) stores `mesh_json: "{kind:\"box\"}"` (a primitive-kind selector regenerated from scratch every render) instead of a real editable topology. Its edit commands (`extrude`, `inset`, `bevel`, `subdivide`, `triangulate`, `mirror`) are literal no-ops (`return Vec::new()`), even though the real kernel operations already exist and work in `kernel/3d/mesh/rs` + `lowpoly/core/rs`.
4. **No sub-element (vertex/edge/face) picking or paint mode exists in the shared `world-3d` renderer** ([framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx)) — it only supports whole-instance selection, whereas the old lowpoly did real raycasting against dedicated point/line/face pick geometries plus brush-based texture painting.

Per your answers: build full old parity, and build the toolbar concept as **generic framework infrastructure** (not lowpoly-specific), since ~17 other technologies still carry the same dead `AppTools` types in their orphaned TS cores.

## Key reusable assets discovered

- `kernel_3d_mesh` ([kernel/3d/mesh/rs/lib.rs](kernel/3d/mesh/rs/lib.rs)) is a pure-Rust, wasm-bindgen-free crate (`serde`/`serde_json` only) with a complete `HalfedgeMesh` API: primitives (`box_prim`, `plane_prim`, `cylinder_prim`, `cone_prim`, `ico_sphere_prim`), edit ops (`extrude_faces`, `inset_faces`, `bevel_edges`, `loop_cut`, `subdivide_faces`, `triangulate`, `mirror`, `decimate`, `flip_faces`, `merge_vertices`, `dissolve_edges/vertices`, `snap_vertices_to_grid`), transforms (`move_vertices`, `rotate_vertices`, `scale_vertices`), UV (`unwrap_uv`, `mark_uv_seam`), and export (`tessellate() -> MeshTransfer`, `to_json`/`from_json`). It's safe to depend on directly from `lowpoly/plugin/rs` (wasm32-compatible, no JS interop).
- [lowpoly/core/rs/lib.rs](lowpoly/core/rs/lib.rs) already wraps this into a `LowpolyDocument`/session with selection resolution, paint pixel storage (`paint_stroke`, `fill_bucket`, `sample_pixel`), and fixture sync — but its useful parts are module-private and only exposed today via a `#[cfg(target_arch = "wasm32")]` wasm-bindgen JS bridge that nothing in the running app calls anymore (the old TS consumers were deleted; only its own orphaned `wasm`/`test` nx targets still touch it). This crate is built `rlib`+`cdylib`, so it can become a normal Rust dependency of `lowpoly/plugin/rs`.
- `UnifiedGumball` already exists in [ui/js/react/index.tsx](ui/js/react/index.tsx) (used elsewhere for spatial transforms) and can be reused as the transform gizmo — no need to build a new gizmo from scratch.
- `MeshData`/`World3dScene` ([framework/core/rs/mesh.rs](framework/core/rs/mesh.rs), [framework/core/rs/ui.rs](framework/core/rs/ui.rs)) already carry generic `positions`/`normals`/`indices` per mesh, so real per-object tessellated geometry can flow through the existing declarative schema — it just lacks picking id arrays and paint-texture fields, which are additive extensions usable by any world-3d consumer, not lowpoly-specific hacks.

## Phase A — Generic AppTools/toolbar framework plumbing

Add to `framework/core/rs` (new `tools.rs` region, following the `WindowMeasure` precedent in `layout.rs`):
- `ToolLeaf` enum (`Separator | Button | Toggle`, mirroring TS `ToolLeaf` in [framework/core/js/index.ts](framework/core/js/index.ts) lines 89-131) and `ToolNode` (`Leaf | Collection`), using `CommandDescriptor` on interactive leaves.
- Add `tools: Vec<ToolNode>` to `ModeDefinition` (serde default-empty, so existing plugins are unaffected).

In [framework/plugin/rs/app.rs](framework/plugin/rs/app.rs):
- Add `ModeSpec.tools` + `AppBuilder::mode_tools(id, tools)` (or extend `.mode(...)` with a builder chain) so plugins can declare per-mode static tools; add `PluginApp::tools(&self, document_json, view_state) -> Vec<ToolNode>` (default empty) for dynamic toggle state (pressed/disabled) that static manifest data can't express.

In [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx):
- Add a `renderToolNode`/`ToolTree` renderer using the existing `ToolbarZone`/`ToolbarGroup`/`ToolbarItem`/`ToolbarDivider` primitives and `Toggle`/`ButtonGroup` from `ui/react`.
- Resolve active-mode tools (static `modes[].tools` merged with dynamic `plugin.tools()`), and pass them into the already-existing but unused `Footer.toolbar` slot, refreshed on the same cadence as `refreshUi`.

Scope note: wire this into the React renderer only (now the default renderer); the WGPU shell's `render_footer` keeps its current hardcoded footer for now — full WGPU tool-tree parity is a separate follow-up, not blocking this ticket.

## Phase B — Real mesh model + edit operations in the plugin

- Make the useful parts of `LowpolyDocument`/session logic in `lowpoly/core/rs/lib.rs` `pub`, and feature-gate the wasm-bindgen JS bridge (currently `#[cfg(target_arch = "wasm32")]`) behind an opt-in Cargo feature so depending crates don't transitively pull in `wasm-bindgen`/`js-sys` glue.
- Add `lowpoly_core` (which already depends on `kernel_3d_mesh`) as a plain path dependency of [lowpoly/plugin/rs/Cargo.toml](lowpoly/plugin/rs/Cargo.toml).
- Delete `lowpoly/plugin/rs/lib.rs`'s duplicated local `LowpolyTransform`/`LowpolySelection`/`LowpolyObject`/`LowpolyFixture` structs in favor of importing the canonical types from `lowpoly_core`, eliminating the schema split between the two crates.
- Replace the primitive-`kind` mesh model with real `HalfedgeMesh`-backed `mesh_json` (`to_json()`/`from_json()`), and implement `addPrimitive` via the real kernel primitives (`box_prim`, `plane_prim`, `cylinder_prim`, `cone_prim`, `ico_sphere_prim`) instead of a placeholder kind string.
- Wire the edit commands that are currently no-ops to real kernel operations, resolving the active object's selection (`selection.mode` + `selection.ids`) into `FaceId`/`EdgeId`/`VertexId` and calling: `extrude` → `extrude_faces(distance)`, `inset` → `inset_faces(amount)`, `bevel` → `bevel_edges(amount, segments)`, `loopCut` → `loop_cut(cuts)`, `subdivide` → `subdivide_faces()`, `triangulate` → `triangulate()`, `mirror` → `mirror(axis, weld_threshold)`, `decimate` → `decimate(ratio)`, plus `flipFaces`, `merge` (`merge_vertices`), `dissolve` (`dissolve_edges`), `snap` (`snap_vertices_to_grid`), `toggleSmooth` (`set_shading`). Tool parameters (`extrudeDistance`, `insetAmount`, `bevelAmount`, `bevelSegments`, `loopCuts`, `decimateRatio`, `snapGrid`, `mirrorAxis`) live in `LowpolyPlayRuntime` and are edited via the inspector, matching the old defaults.

## Phase C — Real default "Rock" example

- Regenerate [lowpoly/example/default.lowpoly.json](lowpoly/example/default.lowpoly.json) from the plugin's now-real fixture schema (ico-sphere + one extruded face, named "Rock", matching `lowpoly_core::default_fixture()`), so the plugin's own `.example("default", "Default", DEFAULT_FIXTURE_JSON)` call ships genuine geometry instead of an empty object list. This must happen after Phase B since it depends on the shared fixture schema.

## Phase D — 3D picking overlays (generic declarative extension)

- Extend `MeshData`/`World3dScene` in [framework/core/rs/mesh.rs](framework/core/rs/mesh.rs)/[framework/core/rs/ui.rs](framework/core/rs/ui.rs) with optional picking arrays sourced from `HalfedgeMesh::tessellate()`'s `MeshTransfer` (`face_ids`, `vertex_ids`, `edge_positions`, `edge_ids`, `edge_uvs`, `edge_is_seam`), and extend `selection_json` with a selection-granularity field (`mesh`/`vertex`/`edge`/`face`) alongside the existing merge `mode`. These are additive, serde-default-empty fields — any other world-3d consumer (cad, puzzle3d, puzzle5d, shooting, procedural3d) is unaffected unless it populates them.
- Extend `WorldInstancesLayer`/`World3dHost` ([framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx)) to build pick/hover/select overlay geometries (points for vertices, thick line segments for edges, translucent triangles for faces) whenever a mesh declares picking data, with raycasting/onClick handlers dispatching a generic `worldSelect`-style command carrying `{granularity, id}`.
- Wire lowpoly's `LOWPOLY_PLAY_BODY_MAIN` scene to emit real per-object tessellated geometry (not shared "kind" meshes) plus the new picking arrays, and to consume the selection-target toggles (mesh/vertex/edge/face) from the new toolbar.

## Phase E — Transform gizmo

- Add a generic gumball overlay to `World3dHost` using the existing `UnifiedGumball` component, active when there's a lowpoly selection; on drag-end, dispatch `translateSelection`/`rotateSelection`/`scaleSelection`-equivalent commands that the plugin resolves to vertex ids and applies via `kernel_3d_mesh`'s `move_vertices`/`rotate_vertices`/`scale_vertices` (selected face/edge selections expand to their corner/endpoint vertices, matching the old `lowpolyResolveTransformVertexIds` logic).

## Phase F — Paint mode

- Add a second `"paint"` app mode to lowpoly's `App::builder` (model layout: single window; paint layout: main 60% + UV canvas 40%, matching the old `ModeRuntime` split), with a paint toolbar (brush/eraser/fill/eyedropper, UV unwrap + mark/clear seam, undo/redo) built with the new generic `ToolNode` system.
- Port paint pixel storage and stroke/fill/sample logic from `lowpoly_core` (`LOWPOLY_PAINT_TEXTURE_SIZE = 1024`, per-object-per-layer RGBA buffers, `paint_stroke`/`fill_bucket`/`sample_pixel`) into the plugin's internal state (kept out of `document_json`, same as the old design).
- Transfer the composited paint texture to the renderer efficiently (PNG-encode + base64, resend only when the texture actually changes) rather than re-embedding raw 4MB buffers in every declarative render pass — a real risk given the prior "Lowpoly Performance Fix" history in this codebase.
- Add a UV 2D canvas panel (edge wireframe from `edgeUvs`/`edgeIsSeam`, composited texture, brush painting in UV space) as the second paint-mode window.
- Add `paintUndo`/`paintRedo` as an internal per-app-instance history stack in `LowpolyPlayApp` (before/after layer-pixel snapshots), matching the old paint-only undo scope — mesh edit operations remain without undo, consistent with the original implementation.

## Phase G — Cleanup dead code

- Delete the now-fully-superseded legacy TS files that nothing in the running app consumes: `lowpoly/core/js/index.ts`, `lowpoly/core/js/internal.ts`, and retire `lowpoly/core/rs`'s wasm-pack build pipeline (`lowpoly/core/script.ts` "wasm" command, `lowpoly/core/rs/pkg/*`) once `lowpoly/plugin/rs` depends on it as a plain crate — keep its `cargo test` targets since the pure-Rust logic still needs coverage.

## Phase H — Verification

- Extend `lowpoly/plugin/rs`'s existing `#[cfg(test)]` module with real coverage for each edit op, the Rock default fixture, picking/selection resolution, and paint stroke round-trips.
- Update the ticket `.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS` (reopen), run the existing React E2E sweep, and manually verify in a live browser: Rock renders by default, toolbar tools reshape the mesh, vertex/edge/face picking + highlighting works, the transform gizmo moves the mesh, paint mode brush/eraser/fill/UV-unwrap work, and undo/redo works for paint strokes.
- Close the ticket with a summary of every file touched.
</plan>
<todos>[{"id":"apptools-core-types","content":"Add generic ToolLeaf/ToolNode types + ModeDefinition.tools to framework/core/rs (mirroring WindowMeasure pattern)"},{"id":"apptools-builder","content":"Add AppBuilder mode-tools API + PluginApp::tools() dynamic hook in framework/plugin/rs/app.rs"},{"id":"apptools-react-render","content":"Render ToolNode tree into the existing Footer.toolbar slot in framework/renderer/react/os-shell.tsx using ui/react Toolbar primitives"},{"id":"lowpoly-core-pub","content":"Make lowpoly_core's document/session logic pub and feature-gate its wasm-bindgen bridge behind an opt-in Cargo feature"},{"id":"lowpoly-plugin-depend-core","content":"Add lowpoly_core as a plain dependency of lowpoly/plugin/rs; delete plugin's duplicated fixture structs in favor of lowpoly_core's canonical types"},{"id":"lowpoly-real-mesh-ops","content":"Wire addPrimitive and all edit commands (extrude/inset/bevel/loopCut/subdivide/triangulate/mirror/decimate/flipFaces/merge/dissolve/snap/toggleSmooth) to real kernel_3d_mesh operations via selection resolution"},{"id":"lowpoly-default-rock","content":"Regenerate lowpoly/example/default.lowpoly.json with a genuine Rock HalfedgeMesh fixture matching the shared schema"},{"id":"lowpoly-toolbar-wiring","content":"Build lowpoly's model-mode toolbar (selection/transform/edit collections) and tool-param inspector fields using the new generic AppTools system"},{"id":"world3d-picking-schema","content":"Extend MeshData/World3dScene with optional per-mesh picking arrays (faceIds/vertexIds/edge data) and selection granularity, sourced from HalfedgeMesh::tessellate()"},{"id":"world3d-picking-render","content":"Extend WorldInstancesLayer/World3dHost to render vertex/edge/face pick+hover+select overlays and dispatch granular worldSelect commands"},{"id":"lowpoly-selection-wiring","content":"Wire lowpoly's selection-target toggles end-to-end from toolbar through plugin state to scene overlays and picking dispatch"},{"id":"lowpoly-gumball","content":"Add UnifiedGumball-based transform gizmo to World3dHost, dispatching translate/rotate/scale resolved to vertex ids via kernel_3d_mesh"},{"id":"lowpoly-paint-mode","content":"Add lowpoly's paint App mode, paint toolbar, ported paint pixel storage/stroke/fill/sample logic, efficient texture transfer, and UV canvas panel"},{"id":"lowpoly-paint-undo","content":"Add internal paintUndo/paintRedo history stack to LowpolyPlayApp"},{"id":"lowpoly-cleanup-dead-code","content":"Delete orphaned lowpoly/core/js/index.ts, internal.ts, and retire lowpoly_core's wasm-pack build pipeline"},{"id":"lowpoly-tests-verify","content":"Extend Rust unit tests, run React E2E sweep, manually verify Rock/toolbar/picking/gizmo/paint in a live browser, then update and close the ticket"}]