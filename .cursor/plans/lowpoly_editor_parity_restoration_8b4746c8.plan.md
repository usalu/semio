---
name: Lowpoly Editor Parity Restoration
overview: Restore lowpoly's document tree, selection/hover system, semantic theme colors, and gumball behavior to match the pre-migration TypeScript editor, fixing several bugs introduced in the Rust port along the way.
todos: []
isProject: false
---

# Lowpoly Editor Parity Restoration

## Context

The Rust port of lowpoly (`lowpoly/program/rs/lib.rs` + `framework/renderer/react/components/world-3d-host.tsx`) is functionally scaffolded but diverges significantly from the old TypeScript editor (`git show 32693795d:lowpoly/core/js/index.ts`, `git show f8376e848:lowpoly/react/index.tsx`) in exactly the areas reported: document tree, gumball, selection, hover, colors. Investigation confirmed concrete gaps and bugs (see below); this plan restores parity and fixes the bugs found along the way.

## Confirmed gaps and bugs

- **Document tree** ([lowpoly/program/rs/lib.rs:414-439](lowpoly/program/rs/lib.rs)) is a flat object list with hardcoded `"box"` icons and no selection state. The old tree nested `Vertices`/`Edges`/`Faces` groups per object (icons `circle`/`minus`/`square`), synced `selectedIds`/`highlightedIds` with the live selection/hover, and exposed a hover-reveal "flip normal" action on faces.
- **Colors** are hardcoded hex (`#60a5fa` selected / `#94a3b8` default) in Rust ([lowpoly/program/rs/lib.rs:333](lowpoly/program/rs/lib.rs)), and hover is never visually distinct because Rust's explicit `color` always wins over the React hover ternary. The old editor resolved live theme tokens (`--active-base`, `--hover-base`, `--border-normal-color`, `--panel`) via `resolveSemanticColorHex`, with distinct opacity/linewidth/size per selected vs hovered state for mesh, edges, vertices, and faces.
- **Gumball** is attached to the object's transform group ([framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx)), not the selection centroid. The old gumball attached to a synthetic pivot positioned at the centroid of selected vertex/edge/face geometry (`lowpoly_core::selection_transform_pivot` already exists and matches this but is unused by the renderer). Drag-end dispatch also never sends the current `mode`/`ids`, so `apply_translate`/`apply_rotate`/`apply_scale` call `doc.apply_selection("mesh", [])` and silently wipe any component selection before transforming.
- **Selection merge** for components is missing: `worldPick` ([lowpoly/program/rs/lib.rs:1225-1233](lowpoly/program/rs/lib.rs)) always replaces with a single id — no shift/ctrl multi-select for vertices/edges/faces, unlike the object-level `worldSelect` which already has `merge_world_selection_ids`.
- **Marquee selection** is drawn but never dispatched (`handlePointerUp` just clears the path) — old editor supported live marquee add/remove.
- **Vertex pick id bug**: `buildVertexPickGeometry` deduplicates positions into pick geometry, but the click handler indexes the original `meshData.vertexIds` array by the click event's point index, causing wrong ids on shared vertices.
- **Face highlight bug**: the "selected face" overlay renders the entire mesh as one translucent overlay instead of only the selected face triangles.

## Phase 1 — Component-level selection system (Rust)

In [lowpoly/program/rs/lib.rs](lowpoly/program/rs/lib.rs):

- Add `merge_selection_ids(existing: &[u32], incoming: &[u32], merge: &str) -> Vec<u32>` mirroring `merge_world_selection_ids` in [framework/program/rs/world3d_host.rs](framework/program/rs/world3d_host.rs) (`add`/`toggle`/`replace`/`invertive`).
- Rework `worldPick` to accept a `merge` arg and use `merge_selection_ids`, auto-enabling the matching `targets.*` flag (like `toggleSelectionKind` does) and setting `selection.mode`.
- Add `toggleSelectionTarget` command (object id + granularity + component id) for document-tree row clicks, using invertive merge by default, setting `active_object_id`, matching old `lowpolyDocumentTargetRowId` semantics.
- Maintain `selection.keys` (`lowpoly:{objectId}:{index}:{mode}:{id}`) whenever ids change, so both the document tree and 3D picking agree on stable row identifiers.
- Add a `setHover` command carrying `{objectId, mode, id}` or `null`, stored on `LowpolyPlayRuntime` as a generalized `hovered_target` (superseding the mesh-only `hovered_object_id` for component-level hover), consumed by both the 3D scene and the document tree's `highlightedIds`.
- Fix `apply_translate`/`apply_rotate`/`apply_scale` to only call `doc.apply_selection(mode, ids)` when `ids` is non-empty (or the renderer now always sends the live selection — see Phase 3), preventing the destructive selection reset.

## Phase 2 — Document tree with vertex/edge/face nesting

In [lowpoly/program/rs/lib.rs](lowpoly/program/rs/lib.rs) `build_document_tree`:

- For each object, nest three child groups: `Vertices` (icon `circle`), `Edges` (icon `minus`), `Faces` (icon `square`), each populated from the tessellated mesh's counts, mirroring `buildLowpolyPlayDocumentTree` (`git show 32693795d:lowpoly/core/js/index.ts` lines ~218-299).
- Each leaf row's `command` dispatches `toggleSelectionTarget`; populate `UiTreeNode.selected_ids`/`highlighted_ids` from `envelope.fixture.selection` + `runtime.hovered_target`.
- Face rows need a hover-reveal "Flip normal" action dispatching `flipFaces` scoped to that single face id. Since [framework/core/rs/ui.rs](framework/core/rs/ui.rs) `UiTreeItemNode` has no `actions` field today (verified — `ui/js/react/index.tsx` `TreeDataItem.actions`/`TreeHeaderAction` already supports hover-reveal buttons, just not threaded through the declarative schema), add:
  - `actions: Option<Vec<UiTreeItemAction>>` to `UiTreeItemNode` in `framework/core/rs/ui.rs` (icon_id + command, `reveal_on_hover: bool`).
  - Map it through in `framework/renderer/react/types.ts` (`UiTreeItemNode.actions`) and `uiTreeItemsToTreeData` in [framework/renderer/react/ui-interpreter.tsx](framework/renderer/react/ui-interpreter.tsx) to `TreeDataItem.actions`.
- Wire hover sync: document tree needs pointer-enter/leave dispatch. Since declarative UI is command-based (no live callbacks across the WASM boundary), add `onPointerEnter`/`onPointerLeave` equivalents as `hover_command`/`unhover_command` fields on `UiTreeItemNode`, mapped to `TreeDataItem.onPointerEnter`/`onPointerLeave` in the interpreter, dispatching `setHover`.

## Phase 3 — Gumball on selection centroid

In [lowpoly/program/rs/lib.rs](lowpoly/program/rs/lib.rs):

- Compute and emit a `gumballTarget: [x,y,z]` (world space) in `world_selection_json_for`, using `lowpoly_core::LowpolyDocument::selection_transform_pivot()` (already implemented, currently unused) plus the active object's transform offset.
- Include the current `selection.mode` and `selection.ids` (or the object-level `selected_object_ids`) in the selection JSON so the renderer can pass them back on transform commands.

In [framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx):

- Replace the per-instance transform-group gumball target with a synthetic `Object3D` positioned at `selection.gumballTarget`, shown once for the whole scene (not per-instance) when there is a non-empty selection, matching old `active={interactionMode === "model" && (selectedTargets.length > 0 || (targets.mesh && activeObject))}`.
- On drag end, include the current `granularity`/`componentIds` (or object ids) in the dispatched `translateSelection`/`rotateSelection`/`scaleSelection` args so Rust's `apply_selection` receives the real selection instead of defaults.
- Keep rotation as Y-axis-only euler delta (matches old behavior — not a bug to fix).

## Phase 4 — Semantic theme colors

In [framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx):

- Import `resolveSemanticColorHex` from `@semio-tech/ui-styling` and resolve `meshColor` (`--panel`), `edgeColor` (`--border-normal-color`), `selectColor` (`--active-base`), `hoverColor` (`--hover-base`) with a `MutationObserver` re-resolve on theme change, matching the old hook in `git show f8376e848:lowpoly/react/index.tsx` lines ~1087-1136.
- Stop Rust from sending a pre-resolved `color` per instance; instead send only `selected`/`hovered` booleans (already present) and let React pick `selectColor > hoverColor > meshColor` — this also fixes the "hover invisible" bug since selected will correctly take priority over hovered instead of hardcoding.
- Apply the old per-element style table: mesh selected/hovered tint; face overlay opacity 0.62 selected / 0.48 hovered with `polygonOffset`; edge overlay linewidth 3; vertex overlay size 9 with `depthTest: false`; base edge wireframe always visible in `edgeColor`; base vertex points in `edgeColor` size 5.

## Phase 5 — Fix picking/highlight bugs

In [framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx):

- Fix vertex pick id bug: `buildVertexPickGeometry` must return both the deduplicated position buffer and a parallel `displayVertexIds` array; click/hover handlers index into that parallel array instead of the raw `meshData.vertexIds`.
- Fix face highlight: build a filtered highlight overlay geometry containing only the triangles for `componentIds` (selected) and the hovered face id (separate hover overlay), instead of overlaying the whole mesh.
- Add matching selected/hovered overlay geometries for edges and vertices (distinct from the always-visible base wireframe/points), using the color/size/opacity table from Phase 4.
- Wire marquee selection: on `handlePointerUp`, compute which instances (or, when a component granularity is active, which picked ids) fall inside the marquee polygon/rect and dispatch `worldSelect`/`worldPick` with the appropriate merge mode instead of only clearing the path.

## Phase 6 — Verification

- Extend `lowpoly-program`'s `#[cfg(test)]` module: component selection merge (add/toggle/invertive), `toggleSelectionTarget`, hover target round-trip, gumball target/pivot computation, document tree selected/highlighted ids and nested vertex/edge/face counts.
- Run `cargo test -p lowpoly-program` and `cargo test -p lowpoly_core`.
- Run the framework renderer vitest suite (`bun nx run @semio-tech/framework-renderer-react:test`).
- Run the React E2E sweep (`.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS/verify-react-playgrounds-e2e.ts --program lowpoly`) plus a manual live-browser pass: document multi-select + hover sync with canvas, face flip action, light/dark theme color correctness, gumball at centroid transforming the correct vertices without losing selection, marquee selecting components, and per-element (not whole-mesh) highlight overlays.
- Reopen/update the ticket `.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS` with a summary of every file touched, then close it.
  </plan>
  <todos>[{"id": "selection-merge-rust", "content": "Add merge_selection_ids + rework worldPick/toggleSelectionTarget/setHover in lowpoly/program/rs/lib.rs"}, {"id": "selection-transform-fix", "content": "Fix apply_translate/rotate/scale to not destructively reset selection"}, {"id": "document-tree-nesting", "content": "Add nested Vertices/Edges/Faces groups with selected/highlighted ids to build_document_tree"}, {"id": "tree-actions-schema", "content": "Add actions + hover_command/unhover_command to UiTreeItemNode (Rust + TS types + ui-interpreter mapping)"}, {"id": "gumball-centroid", "content": "Emit gumballTarget from selection_transform_pivot; attach gumball to synthetic Object3D at centroid instead of instance transform group"}, {"id": "gumball-selection-passthrough", "content": "Pass current granularity/componentIds through gumball drag-end dispatch"}, {"id": "semantic-colors", "content": "Resolve theme colors via resolveSemanticColorHex in world-3d-host.tsx; remove hardcoded Rust color field; fix hover visibility"}, {"id": "style-table-overlays", "content": "Apply old opacity/linewidth/size table for mesh/face/edge/vertex selected vs hovered overlays"}, {"id": "fix-vertex-pick-bug", "content": "Fix vertex pick id indexing to use deduplicated geometry's parallel id array"}, {"id": "fix-face-highlight-bug", "content": "Fix face highlight overlay to render only selected/hovered face triangles, not the whole mesh"}, {"id": "wire-marquee-selection", "content": "Dispatch worldSelect/worldPick from marquee pointer-up instead of only clearing the path"}, {"id": "parity-tests-verify", "content": "Extend Rust tests, run full test/E2E suite, manually verify in browser, update and close ticket"}]</todos>
  </CreatePlan>
