---
name: Deselect Perf Fix
overview: Make selection/deselection in puzzle 2d play instant by decoupling selection from the Tree rendering context (so only changed rows re-render) and from the WASM world-content geometry cache (so selection changes no longer rebuild all node/edge geometry across 3 panes).
todos:
  - id: tree-selection-store
    content: "ui/react: add TreeSelectionStore/Context, remove selectedIds from treeDataRenderingValue, per-row useSyncExternalStore in TreeDataItemView, update handlers to read store ref"
    status: completed
  - id: wasm-selection-overlay
    content: "puzzle/2d/rs: build world_content_cache without selection styling, add selection/preselect overlay pass, remove bump_content_scene_generation from selection setters"
    status: completed
  - id: framework-canvas-context
    content: "framework playground: split stable applyCanvasSelection context so the 3 pane canvases don't re-render on selection changes"
    status: completed
  - id: validate-deselect
    content: Extend existing vitest files, add [DEBUG] counters, run package tests, runtime-verify instant deselect on Nakagin; track under repo ticket
    status: completed
isProject: false
---

# Deselect Performance Fix

## Root cause
Clicking the background emits one cheap `select {ids: []}`, but every selection change triggers two O(N) operations, and background deselect fans this out to all 3 triptych panes:

- UI Tree: `treeDataRenderingValue` carries `selectedIds`, so any selection change recreates the context and re-renders all ~700 document rows.
- WASM puzzle/2d: each selection setter calls `bump_content_scene_generation()`, invalidating `world_content_cache` (selection style is baked into cached geometry), forcing a full node/handle/edge rebuild. Deselect broadcasts to 3 panes -> 3 full rebuilds.

## Fix 1 - ui/react: per-row selection subscription (removes O(rows) reconcile)
In [ui/react/index.tsx](ui/react/index.tsx):
- Add a small `TreeSelectionStore` (a ref holding the `Set<string>` + a version counter + `subscribe`/`getSnapshot`), created once per `Tree` and updated in an effect when `resolvedSelectedIds` changes. Provide it via a new `TreeSelectionContext` whose value identity is stable across selection changes.
- Remove `selectedIds` from `treeDataRenderingValue` (line ~9140) so the shared `TreeDataRenderingContext` no longer changes on selection.
- In `TreeDataItemView` (line ~8704), replace `selectedIds.includes(item.id)` (line 8746) with a per-row `reactHostPort.useSyncExternalStore` selecting only this row's boolean from the store. Only rows whose selected state flips re-render.
- Update `handleSelectItem`/`handleDoubleClickItem` to read current ids from the store ref instead of the `resolvedSelectedIds` closure, keeping handler identities stable.
- External `selectedIds` / `onSelectionChange` API is unchanged.

## Fix 2 - puzzle/2d/rs: draw selection chrome as an overlay (stops geometry cache busts)
In [puzzle/2d/rs/lib.rs](puzzle/2d/rs/lib.rs):
- Build the cached `world_content_cache` (`append_cached_world_content` ~5522, `append_nodes_and_handles` ~5343) with neutral/original styling only - no selection/preselect-dependent `resolve_node_style_kind` styling baked in.
- Add a selection overlay pass appended after the cached content each frame that iterates only selected + preselect + exit-highlight ids (O(selected)) and redraws those nodes/handles/edges with selected/highlighted fill+stroke (+icon) on top.
- Remove `bump_content_scene_generation()` from the selection/preselect setters (lines 3880, 3918, 3934, 3946, 3967) so selection-only changes keep the geometry cache warm; keep `sync_selection_flags_to_objects` for hit-testing/serialization but it no longer invalidates the cache. Trigger only a normal repaint.
- Net: background deselect (and its 3-pane broadcast) reuses cached geometry; only the cheap overlay redraws.

## Fix 3 - framework playground: stop re-rendering all pane canvases on selection (secondary)
In [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx):
- `Puzzle2dPlayPaneCanvas` consumes `usePuzzle2dPlaySelection()` only for the stable `applyCanvasSelection`, but re-renders because `selectionValue` identity changes with `selectionIds`. Provide `applyCanvasSelection` via a separate stable context (split from `selectionValue`) so the 3 pane canvases don't re-render on selection changes.

## Validation
- Extend existing vitest files only (no new test files): ui/react Tree test (per-row selection isolation: changing selection re-renders only affected rows), puzzle/2d/rs selection setters (no `content_scene_generation` bump; overlay draws selected chrome), puzzle/2d/react (deselect keeps descriptor/geometry cache warm), framework playground (pane canvas not re-rendered on selection).
- Add `[DEBUG]` render/redraw counters, run the affected package tests, and runtime-verify on the Nakagin fixture that select-all then background-click deselect is instant (no multi-second freeze) with document/inspector still correct.
- Track under repo MCP ticket (reopen `26/06/01/PUZZLE3D-SELECTION-PERF` or open a new selection-perf ticket); close with summary and touched files.