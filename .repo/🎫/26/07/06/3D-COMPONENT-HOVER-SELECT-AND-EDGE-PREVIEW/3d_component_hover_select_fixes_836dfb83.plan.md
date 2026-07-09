---
name: 3D component hover select fixes
overview: Fix the vertex/edge/face hover and selection pipeline between the WGPU world engine and the lowpoly plugin (both WGPU and React renderers), and add an independent "Show Edges" window-option toggle.
todos:
 - id: fix-wire-format
   content: "Make component ids numeric end-to-end: fix WorldSelectionRecord deserialization, pick_select_command id encoding, and marquee_select_command to emit setSelection"
   status: completed
 - id: add-component-hover
   content: Add component hover picking to WGPU pick_hover_command via setHover command
   status: completed
 - id: add-vertex-face-overlays
   content: Add vertex and face highlight overlay branches to append_component_overlays in infinite/world
   status: completed
 - id: add-show-edges-toggle
   content: Add show_edges runtime flag, toggleShowEdges command, and Show Edges WindowEngagementOption to lowpoly plugin
   status: completed
 - id: wire-show-edges-renderers
   content: Sync show_edges into WGPU World3dState/overlay condition and React WorldSelectionRecord/edge visibility gate
   status: completed
 - id: add-tests
   content: Add regression tests in lowpoly and infinite/world for numeric id round-trip, overlays, and showEdges
   status: completed
 - id: verify
   content: cargo test, rebuild WASM, rerun lowpoly e2e, manual browser verification of hover/select/edge-preview
   status: completed
isProject: false
---

# 3D Vertex/Edge/Face Hover, Select, and Edge-Preview Fixes

## Root causes found

Tracing the pipeline (`lowpoly` plugin -> selection JSON -> `infinite/world` WGPU engine, and separately -> React `world-3d-host.tsx`) turned up several concrete bugs, not just "missing polish":

1. **Selection JSON silently fails to parse on the WGPU side whenever component ids are present.** [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs) emits `componentIds` as JSON numbers (`envelope.fixture.selection.ids: Vec<u32>`, line 470). [infinite/world/rs/lib.rs](infinite/world/rs/lib.rs) declares `WorldSelectionRecord.component_ids: Option<Vec<String>>` (line 68) and does `serde_json::from_str(&world.selection_json).unwrap_or_default()` (line 958). A type mismatch on one field fails the **whole struct**, silently resetting granularity/selected ids/everything back to defaults. This is the primary reason vertex/edge/face selection "doesn't show" in the WGPU view.
2. **`hoveredComponent.id` type mismatch.** lowpoly emits `id: u32` (`LowpolyHoverTarget.id`, line 141); WGPU only extracts it via `.as_str()` (`sync_world3d_state`, lines 982-987) so hover id is always `None`.
3. **Click-pick sends a string id; lowpoly expects a number.** `pick_component_at` (lines 1832-1930) stringifies numeric mesh ids; `pick_select_command` (1636-1682) puts that string in `worldPick.id`; lowpoly's handler (1873-1885) does `.as_u64()`, which fails on a JSON string and defaults to `0` -- WGPU click-select of a component in lowpoly always resolves to component 0.
4. **Marquee sends `ids` (array); lowpoly's `worldPick` handler only reads singular `id`.** `marquee_select_command` (1684-1750) builds `worldPick` with `"ids": [...]`; lowpoly ignores it -- marquee component selection is silently dropped.
5. **No component hover picking in WGPU.** `pick_hover_command` (1611-1634) only does object-level `pick_instance_at` / `worldHover`. There is no `setHover` equivalent for vertex/edge/face, so `hoveredComponent` can never get set from WGPU interaction (React already does this via `onComponentHover` -> `setHover`, `world-3d-host.tsx:990-999`).
6. **Vertex and face overlays are unimplemented in WGPU.** `append_component_overlays` (369-454) only draws highlight lines when `state.granularity == "edge"`. Selected/hovered/preview vertices and faces get zero visual feedback.
7. **No independent "always show edges" mode.** The gray baseline wireframe (376-404) only renders during paint mode or when component granularity is active. In React, edge visibility is gated by the same `targets.edge` flag that also controls whether edges are pickable (`world-3d-host.tsx:521`), so there's no way to just look at the wireframe without entering edge-select mode.

## Fix plan

### A. Make component ids numeric end-to-end (fixes 1-4)

In [infinite/world/rs/lib.rs](infinite/world/rs/lib.rs):

- Make `WorldSelectionRecord.component_ids` tolerant of numeric JSON (custom `deserialize_with`, coercing each element to `String` so the rest of the file's `Vec<String>` id-matching code is untouched).
- Fix `hovered_component_id` extraction (982-987) to accept `.as_u64()` as well as `.as_str()`.
- In `pick_select_command` (1636-1682), when in component mode, emit `worldPick.id` as a JSON **number** (parse the picked string id back with `.parse::<u64>()`), matching what lowpoly's `.as_u64()` expects.
- In `marquee_select_command` (1684-1750), stop emitting `worldPick` with an `ids` array for component mode (lowpoly ignores it). Instead, compute the final merged numeric id set locally (existing `state.component_ids` + new hits, respecting add/toggle/replace) and emit `setSelection` (`{mode: granularity, ids: [numbers]}`) -- the command lowpoly already implements and tests (`lowpoly/plugin/rs/lib.rs:1547-1557`).

### B. Add component hover picking to WGPU (fixes 5)

- Extend `pick_component_at` (1832-1930) to also return the owning `instance.id`.
- In `pick_hover_command` (1611-1634), when `component_mode_active(state)`, call it and emit `setHover` (`{objectId, mode: granularity, id}`) instead of `worldHover`, mirroring the React path and lowpoly's existing tested handler (`lowpoly/plugin/rs/lib.rs:1817-1839`, test `set_hover_round_trips_through_runtime`).

### C. Add vertex + face highlight overlays in WGPU (fixes 6)

In `append_component_overlays` ([infinite/world/rs/lib.rs:369-454](infinite/world/rs/lib.rs)), add two branches parallel to the existing `granularity == "edge"` branch:

- **Vertex**: for each selected/hovered/preview vertex id, draw a small 3-segment "jack" marker at the vertex's transformed position using the existing highlight/preview colors.
- **Face**: for each selected/hovered/preview face id, resolve its triangle via `mesh.indices`/`face_ids` and draw its 3 edges in the highlight/preview color (same technique as the edge branch, sourced from the triangle instead of `mesh.edge_positions`).

### D. Add a "Show Edges" window-option toggle (fixes 7)

- [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs): add `show_edges: bool` (default `false`) to `LowpolyPlayRuntime` (108-131); add a `WindowEngagementOption` ("Show Edges", `pressed: Some(envelope.runtime.show_edges)`, `command: lowpoly_cmd("toggleShowEdges", None)`) to `lowpoly_window_engagement` (954-1000) next to Snap/Smooth; add a `"toggleShowEdges"` command handler that flips the flag and returns `set_document_op`; include `showEdges` in `world_selection_json_for` (450-484).
- [infinite/world/rs/lib.rs](infinite/world/rs/lib.rs): add `show_edges: bool` to `World3dState`/`WorldSelectionRecord`, sync it in `sync_world3d_state`, and OR it into the baseline-wireframe condition (line 376): `state.interaction_mode == "paint" || component_mode_active(state) || state.show_edges`.
- [framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx): add `showEdges?: boolean` to `WorldSelectionRecord` (99-112); change the edge-visibility gate (line 521) from `targets.edge && edgeGeometry` to `(targets.edge || selection.showEdges) && edgeGeometry`.

## Verification

- Extend existing test files (no new test files, per repo convention):
  - `lowpoly/plugin/rs/lib.rs`: add a `toggleShowEdges` round-trip test and confirm `showEdges` appears in `world_selection_json_for` output.
  - `infinite/world/rs/lib.rs`: add regression tests for `sync_world3d_state` parsing numeric `componentIds`/`hoveredComponent.id` without losing state; test that `append_component_overlays` produces non-empty lines for vertex and face granularity; test that `pick_select_command`/`marquee_select_command` emit numeric ids / `setSelection` in component mode.
- `cargo test` for `infinite_world` and `lowpoly_plugin`.
- Rebuild the WASM renderer + lowpoly plugin, rerun the lowpoly e2e smoke test (body-content check), and manually verify in-browser: click and marquee select a vertex/edge/face in the WGPU 3D view and confirm highlight + status bar `selected_count` update; hover a component and confirm hover highlight; toggle "Show Edges" and confirm the wireframe appears without entering edge-select mode.
