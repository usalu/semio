---
name: Puzzle 3D React Parity
overview: Restore premigration parity for puzzle 3D in the React renderer by fixing the GLB coordinate-frame bug, aligning the puzzle3d Rust plugin's command protocol with what `world-3d-host.tsx` actually dispatches (hover/pick/gumball/marquee), rendering the vortex/attraction/target-volume/reference/brush-preview scene layers the plugin already computes but the host drops, and adding the missing Select/Brush/Fill toolbar (`window_engagements`).
todos: []
isProject: false
---

# Puzzle 3D React Parity Restoration

## Root causes (all verified by reading code, not assumed)

1. **Wrong mesh coordinate system**: [framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx) `GlbInstanceMesh` loads GLB scenes and renders `<primitive object={scene} />` with **no** frame correction. The premigration renderer wrapped every GLB in `<group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>` (confirmed at the premigration tag, `puzzle/3d/react/index.tsx:6741-6744`, doc'd as "glTF Y-up mesh geometry → CAD object-local Z-up"). That exact constant **still exists and is exported** today from `@semio-tech/infinite-world-r3f` ([infinite/world/r3f/index.tsx:208](infinite/world/r3f/index.tsx)) but is unused by `world-3d-host.tsx`. This is why concrete-forest's GLB objects render in the wrong orientation.

2. **Hover/select/marquee protocol mismatch**: `world-3d-host.tsx` dispatches `setHover` (args `{objectId, mode, id}`) and `worldPick` (args `{granularity, id, merge}`) for clicks/marquee, and reads `selection.{selectionMode, targets, activeObjectId, transformTool, gumballActive, gumballTarget}` to drive highlighting/gumball. [puzzle/plugin/rs/d3/mod.rs](puzzle/plugin/rs/d3/mod.rs) only implements the older `worldHover`/`worldSelect` commands and its `world_selection_json` only emits `{method, mode, ids, hoveredId}` — none of the fields the host needs. This is confirmed by contrast with [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs), which implements both conventions and is the one plugin `world-3d-host.tsx` actually works against today. Net effect: no hover highlight, clicking an object does nothing, marquee/rectangle-lasso selection dispatches into a void, and the gumball never appears.

3. **Scene layers computed but never rendered**: `puzzle/plugin/rs/d3/mod.rs::render()` already calls `world3d_scene_extended(...)` with real `vortices_json`/`attractions_json`/`target_volumes_json`/`references_json`/`brush_preview_json`/`interaction_json` — but the wire type `World3dScene` in [framework/renderer/react/os-shell.tsx:2117-2122](framework/renderer/react/os-shell.tsx) only declares 4 fields, and `world-3d-host.tsx` only reads `cameraJson/meshesJson/instancesJson/selectionJson`. So vortex markers, attraction/cable lines, target volumes, reference image planes, and the brush ghost preview are silently dropped before they ever reach Three.js.

4. **No Brush/Fill/Select toolbar**: `Puzzle3dPlayApp` never implements `window_engagements()` (default no-op per [framework/plugin/rs/lib.rs:415-420](framework/plugin/rs/lib.rs)), even though `engagementPossibleSelect`/`addBrushObject`/`setFillCount`/`cycleBrushCandidate` command handlers already exist. The sibling [puzzle/plugin/rs/d2/mod.rs::puzzle2d_engagement](puzzle/plugin/rs/d2/mod.rs) is the exact reference pattern (Select/Brush/Fill `options` with `pressed` state, brush candidate `ToggleGroup`, fill-count `Slider`) and is directly portable.

## Part A — Fix GLB coordinate frame

[framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx)

- Import `GLB_MESH_FRAME_ROTATION_X` from `@semio-tech/infinite-world-r3f`.
- In `GlbInstanceMesh`, wrap the cloned scene in a `<group rotation={[GLB_MESH_FRAME_ROTATION_X, 0, 0]}>` before returning, so glTF Y-up mesh data lands correctly in the Z-up fixture frame (matches premigration `GlbMeshFrame`).

## Part B — Fix hover/select/gumball/marquee protocol

[puzzle/plugin/rs/d3/mod.rs](puzzle/plugin/rs/d3/mod.rs)

1. Add `transform_tool: String` (default `"move"`) to `Puzzle3dRuntime`.
2. Add command handlers (alongside existing `worldSelect`/`worldHover`, keep both since `worldHover`/`worldSelect` may still be used elsewhere e.g. wgpu renderer — do not remove):
   - `"setHover"`: if `args.objectId` present, set `runtime.hovered_object_id` from it; else clear it.
   - `"worldPick"`: read `granularity` (always `"mesh"` for puzzle3d) and `merge`; if `id` is `null`, clear `selection.object_ids` when `merge == "replace"`; else resolve the numeric `id` (array index sent by `World3dHost.handleInstancePointerDown`) to `envelope.fixture.objects[id].id` and merge into `selection.object_ids` via the existing `merge_world_selection_ids`.
   - `"setTransformTool"`: `{tool}` → `runtime.transform_tool`.
3. Rewrite `world_selection_json` (mirroring `lowpoly::world_selection_json_for`) to enrich the base `world3d_selection_json(...)` output with: `granularity: "mesh"`, `selectionMode: "mesh"`, `targets: {mesh: true, vertex: false, edge: false, face: false}`, `transformTool: runtime.transform_tool`, `activeObjectId: selection.object_ids.first()`, `gumballActive: !selection.object_ids.is_empty()`, `gumballTarget: <centroid of selected objects' world origins>` (only when non-empty).
4. Extend `#[cfg(test)]` tests: `worldPick` selects by resolved id and clears on `id: null`; `setHover` sets/clears `hovered_object_id`; selection JSON contains `gumballActive`/`gumballTarget`/`selectionMode` once an object is selected.

## Part C — Render the already-computed scene layers

1. [framework/renderer/react/os-shell.tsx](framework/renderer/react/os-shell.tsx) `World3dScene` type (~line 2117): add the optional fields already emitted by Rust: `vorticesJson?`, `attractionsJson?`, `targetVolumesJson?`, `referencesJson?`, `brushPreviewJson?`, `interactionJson?`, `lodJson?`, `chunkingJson?` (camelCase matches `#[serde(rename_all = "camelCase")]` on the Rust `World3dScene`).
2. [framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx):
   - Parse `vorticesJson` (`{fullId, objectId, vortexKind, position, direction, radius, color}[]`) and render each as a small colored sphere/marker; hover dispatches `worldVortexHover` (`{fullId}`), click dispatches `worldVortexSelect` (`{fullId}`) when `interaction.activeTool !== "brush"`, or `addBrushObject` with a `BrushPlacePayload`-shaped `{targetVortexFullId, objectKindId, sourceVortexIndex, origin, orientation}` when the active tool is `"brush"` (candidate resolved from `interaction.brushCandidateIndex`).
   - Parse `attractionsJson` (`{id, from, to, color}[]`) and render as `lineSegments` between endpoints (cables).
   - Parse `targetVolumesJson`/`referencesJson` and render via the **existing** `WorldVolumeLayer`/`WorldReferenceLayer` components already exported by `@semio-tech/infinite-world-r3f` ([infinite/world/r3f/index.tsx:2781](infinite/world/r3f/index.tsx), [:2541](infinite/world/r3f/index.tsx)) — do not reimplement, just map the parsed JSON into their `WorldVolumeProps`/`WorldReferenceProps` shapes.
   - Parse `brushPreviewJson` (present only while `activeTool === "brush"`) and render a translucent ghost instance of the candidate mesh (reuse the existing translucent-material pattern already used for face/edge preview overlays in this file).
   - Parse `interactionJson` (`{activeTool, brushCandidateIndex, hoveredVortexFullId}`) to gate the above gestures and, when `activeTool === "fill"`, disable normal mesh pick so fill-count changes (driven by the new toolbar, Part D) aren't fought by canvas clicks.

## Part D — Brush/Select/Fill toolbar (`window_engagements`)

[puzzle/plugin/rs/d3/mod.rs](puzzle/plugin/rs/d3/mod.rs) — port `puzzle2d_engagement`/`puzzle2d_brush_placement_control`/`puzzle2d_fill_count_control` from [puzzle/plugin/rs/d2/mod.rs:536-665](puzzle/plugin/rs/d2/mod.rs):

1. `fn puzzle3d_brush_placement_control(envelope) -> Option<WindowEngagementControl>`: build a `ToggleGroup` from `self.precompute.brush_candidates(target_vortex_full_id)` (target = `runtime.selection.vortex_ids.first()` or `runtime.hovered_object_id`-derived vortex), options labelled by candidate object kind, `on_select: puzzle3d_cmd("engagementControlSelect", None)`.
2. `fn puzzle3d_fill_count_control(envelope) -> WindowEngagementControl`: `Slider` bound to `runtime.fill_count`, `on_change: puzzle3d_cmd("setFillCount", None)`, max 1000 (matches `FILL_COUNT_MAX` in `puzzle/3d/rs/lib.rs`).
3. `fn puzzle3d_engagement(envelope) -> WindowEngagement`: `options` = Select/Brush/Fill (`PUZZLE3D_ENGAGEMENT_TOOL_SELECT/BRUSH/FILL`, `pressed` from `runtime.active_tool`, `command: puzzle3d_cmd("engagementPossibleSelect", Some({possibleId}))`), `control` = brush/fill control when active, `session_active: Some(runtime.active_tool != "select")`.
4. Add `"engagementControlSelect"` command handler: parse `puzzle3d.brush.candidate.<index>` id into `runtime.brush_candidate_index`.
5. Implement `fn window_engagements(&self, document_json, _view_state) -> HashMap<String, Vec<WindowEngagement>>`-equivalent (actual trait signature per `framework/plugin/rs/lib.rs:415`, returns `HashMap<String, WindowEngagement>` keyed by window id) returning `{PUZZLE3D_PLAY_WINDOW_MAIN: puzzle3d_engagement(&envelope)}`.
6. In `create_puzzle3d_app()`, switch `.window_kind(...)` to `.window_kind_with_engagement(PUZZLE3D_PLAY_WINDOW_MAIN, "Puzzle 3D", PUZZLE3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d, puzzle3d_engagement(&default_envelope()))`.
7. Extend `#[cfg(test)]` tests: `window_engagements` includes Select/Brush/Fill options; brush tool with candidates shows a `ToggleGroup`; fill tool shows a `Slider`.

## Validation

- Open/reopen the appropriate repo ticket (check `repo://goals` first) before editing; work inside it per workspace rules.
- `cargo test -p puzzle-plugin` (or the crate's actual package name) for all new/changed handlers and engagement tests.
- `bun nx test framework-renderer-react` (or repo equivalent) for the `os-shell.tsx` type change and any `world-3d-host.tsx` unit-testable helpers (parsing functions).
- Rebuild the puzzle3d plugin WASM + rerun the React dev server; manually load the Concrete Forest example and, with `[DEBUG]`-prefixed temporary console logs (removed before finishing), confirm: GLB objects stand upright, hovering highlights an object, clicking selects it (gumball appears), rectangle/lasso marquee selects multiple objects, vortex markers and attraction cables are visible, the Select/Brush/Fill toolbar appears and brush/fill controls work end-to-end (placing an object, filling a count).
- Close the ticket with a summary of all files touched.
