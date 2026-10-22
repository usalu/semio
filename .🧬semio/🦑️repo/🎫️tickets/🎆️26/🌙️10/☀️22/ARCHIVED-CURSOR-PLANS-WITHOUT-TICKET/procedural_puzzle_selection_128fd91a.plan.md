---
name: Procedural Puzzle Selection
overview: Replace procedural's single-pick selection with puzzle-grade partial/full/group marquee selection on both the flow graph (sharing puzzle-2d's Rust logic) and the 3D preview (mirroring puzzle-3d), using the same hotkeys, merge modes, and SelectionMarquee visuals.
todos:
 - id: rust-shared
   content: Extract pure marquee primitives (modifier→mode, merge, drag-shape, partial/full hit, preselect) into mathematical/graph/lib.rs and refactor board_host to use them
   status: completed
 - id: engine-dag
   content: Add SelectionPending/AreaSelect/group-DragNodes + preselect + screen-preview + select-all/delete/cancel to GraphEngine and wire DAG host (pointer modifiers, id mapping, preselect paint)
   status: completed
 - id: flow-core
   content: "flow/core: new pointer signature (button+modifiers, drop shift-pan), selection options + preselect/preview/select-all/delete/cancel pass-throughs and WASM bindings; rebuild WASM"
   status: completed
 - id: flow-react
   content: "flow/react: pass modifiers/button, space/middle pan, SelectionMarquee DOM overlay from preview points, ctrl+a/delete/esc hotkeys, controlled selectionMethod/mode, emit preselect"
   status: completed
 - id: ts-shared
   content: Lift generic marquee helpers into @semio-tech/ui-react next to SelectionMarquee; refactor puzzle-3d to import them
   status: completed
 - id: preview-marquee
   content: "procedural/react ProceduralPreview: rect/lasso marquee with screen-projected partial/full hit testing, modifier merge modes, overlay, orbit gating, preselect visuals; onSelectionChange(ids, mode)"
   status: completed
 - id: play-renderer
   content: "procedural/play + renderer: selectionMode/method state+tools+keybindings, merge-aware shared setSelection, wire both panes (flow + preview) with controlled props"
   status: completed
 - id: tests
   content: Add Rust + vitest coverage; rebuild WASM; validate persistence/partial/full/group/modifiers via [DEBUG] logs; close ticket
   status: completed
isProject: false
---

# Procedural Puzzle-Style Selection (Flow graph + 3D preview)

Work inside the repo ticket: reopen `2026/06/07/PROCEDURAL-BREP-PLAYGROUND` via repo MCP `ticket_reopen` (or open a new ticket after reading `repo://goals`). Extend existing files only, using `//#region` / `pub mod` sub-regions. Rebuild flow WASM after Rust edits; run the vitest + Rust suites before closing.

## Target behavior (parity with puzzle 2d/3d)

- Marquee coverage: drag left→right = **full** (enclosing/contained), right→left = **partial** (crossing/intersecting).
- Methods: `rectangle` + `lasso`; modes: `default | additive | subtractive | invertive`.
- Hotkeys: `shift`=additive, `ctrl/cmd`=subtractive, `shift+ctrl`=invertive; `ctrl/cmd+a`=select all; `Delete`/`Backspace`=delete selection; `Esc`=cancel marquee restoring pre-drag selection; background click clears.
- Group selection: multi-selected nodes drag together; dragging a member moves the whole group.
- Visuals: dashed-when-partial `SelectionMarquee` overlay on both panes; selected = primary outline/fill, preselect-exit = secondary highlight.
- Selection **persists** across pane switches and is shared/merge-aware between flow graph and 3D preview.

## Phase 1 — Shared Rust marquee primitives

Goal: single source of truth for the selection algorithms currently embedded in `BoardHost`.

- In [mathematical/graph/lib.rs](mathematical/graph/lib.rs) add a `//#region` of pure free functions (generic over node id/rect):
  - `pick_merge_mode_for_modifiers(ctrl_or_meta, shift, option_mode)` (lift from [board_host.rs](mathematical/graph/port/directed/normal/board_host.rs:3748)).
  - `merge_pick_into_selection` / `merge_ids(mode, current, incoming)`.
  - `selection_drag_shape(start, points) -> (rect, enclosing, polygon)` (lift from `selection_drag_shape_world`, [board_host.rs](mathematical/graph/port/directed/normal/board_host.rs:5576)).
  - `rect/polygon contains/overlaps` hit tests for partial vs full.
  - `apply_area_preselect(...) -> (ids, removed_ids)` core (from [board_host.rs](mathematical/graph/port/directed/normal/board_host.rs:2886)).
- Refactor `BoardHost` to call these shared functions (remove its private duplicates) so puzzle-2d and flow share identical behavior.

## Phase 2 — Marquee/group/preselect in the shared engine + DAG host

- [mathematical/graph/lib.rs](mathematical/graph/lib.rs): extend `InteractionMode` with `SelectionPending { initial, start }`, `AreaSelect { method, points, initial }`, and group `DragNodes { primary, offset, start_positions }`. Add `selection_options` (method/mode/select kinds), `preselect`/`preselect_removed` sets, and a screen-preview points buffer. Add `BoardEvent::PreselectChanged`. Rework `pointer_down/move/up` to accept modifiers+button and route to pick (with merge mode), area-select (preselect during move, commit on up), or group drag — mirroring [board_host.rs](mathematical/graph/port/directed/normal/board_host.rs:5113) `pointer_down_screen`/`pointer_move_screen`/`pointer_up_screen`. Add `cancel_area_select`, `delete_selection`, `select_all`.
- [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs): update `pointer_down/move/up` (lines ~1163-1201) to pass modifiers+button; map engine preselect/selection ids via `node_id_map`; expose `selection_preview_points`, `preselect_node_ids`, `set_selection_options`, `cancel_area_select`, `delete_selected`, `select_all_node_ids`. Paint preselect (secondary highlight) in the node-paint loop and keep selected outline.

## Phase 3 — Flow core + WASM bindings

- [flow/core/lib.rs](flow/core/lib.rs): change `pointer_down_screen` (line ~775) signature to `(sx, sy, button, shift, ctrl_or_meta, alt)`; stop treating shift as pan (pan = middle button or a `space`/explicit pan flag). Forward modifiers to `self.dag`. Add pass-throughs: `selection_options`, `selection_preview_points_json()`, `preselect_widget_ids_json()`, `cancel_area_select()`, `delete_selection()`, `select_all()`. Add `#[wasm_bindgen] FlowSession` bindings near existing selection methods (~1258): `setSelectionOptions`, `selectionPreviewPointsJson`, `preselectWidgetIds`, `cancelAreaSelect`, `deleteSelection`, `selectAll`, and updated pointer signatures (~1423).

## Phase 4 — Flow react (canvas overlay + hotkeys)

- [flow/react/index.tsx](flow/react/index.tsx): update pointer handlers (~1111-1150) to pass `e.button`, `e.shiftKey`, `e.metaKey||e.ctrlKey`, `e.altKey`; remove `pan: e.shiftKey` (keep middle-mouse pan; add space-held pan). Emit preselect alongside selection in `emitInteractionState` (~902).
- Add a DOM overlay layer rendering `@semio-tech/ui-react` `SelectionMarquee` from `session.selectionPreviewPointsJson()` (coverage from drag direction), positioned over the canvas (it currently has only `<canvas>`).
- Add keydown handling (focus-scoped): `ctrl/cmd+a` → `selectAll`, `Delete`/`Backspace` → `deleteSelection`, `Esc` → `cancelAreaSelect`. Re-emit interaction state + evaluate after each.
- New `FlowCanvasProps`: `selectionMethod`, `selectionMode` (controlled) → `session.setSelectionOptions`; `onPreselectChange` if needed.

## Phase 5 — Shared TS marquee primitives + 3D preview

- Lift generic marquee helpers next to `SelectionMarquee` in [ui/react/index.tsx](ui/react/index.tsx:369): `marqueeIsCrossing`, `marqueeModeFromModifiers`, `marqueeCoverageFromDrag`, `selectionMergeIds(mode, current, incoming)`, and rect/polygon screen hit-tests (from [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx:584) area). Refactor puzzle-3d to import these (drop local copies of the generic ones; keep snapshot-specific merge).
- [procedural/react/index.tsx](procedural/react/index.tsx) `ProceduralPreview` (~867): add a marquee gesture over `WorldCanvas` — pointer drag (threshold ~4px) builds rect/lasso, projects each handle's world bounds to screen (via camera) for partial/full hit testing against geometry, computes merge mode from modifiers, and calls a new `onSelectionChange(ids, mode)` (additive/subtractive/invertive aware). Render `SelectionMarquee` overlay with coverage. Keep single-mesh click as `default` pick honoring modifiers. Gate `WorldOrbitGated` off during marquee. Reuse existing `worldEntityRenderMode` for selected/hover/preselect visuals; add a secondary "preselect-exit" tint.
- New `ProceduralPreviewProps`: `selectionMethod`, `selectionMode`, `onSelectionChange(ids, mode)`.

## Phase 6 — Procedural play harness + renderer wiring

- [procedural/play/index.ts](procedural/play/index.ts): add controller state `selectionMode: "default"|...`, `selectionMethod: "rectangle"|"lasso"` with getters; make `setSelection` merge-aware (`{ ids, mode }`) using shared `selectionMergeIds` so flow ↔ preview share one persistent selection. Add commands `setSelectionMode`, `setSelectionMethod`, `selectAll`, `deleteSelection`. Add a Select tool group to the flow + preview window engagements (reuse `buildPlaygroundBrowseSelectionTools` from `@semio-tech/framework-playground-core`, as puzzle-3d does). Register `ctrl+a` / `delete` / `backspace` / `esc` keybindings following puzzle ordering/grouping (and `launch.json` if a new command entry is needed).
- [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx:6296): pass `selectionMode`/`selectionMethod` into `ProceduralFlowEditor` and `ProceduralPreview`; change preview `onSelect` (single replace, ~6368) to `onSelectionChange(ids, mode)` → `ctrl.run("setSelection", { ids, mode })`; forward flow preselect/selection. Ensure both panes re-render via the existing interaction revision hook.

## Phase 7 — Tests + validation

- Rust `#[cfg(test)]`: shared primitives (modifier→mode, partial vs full hit, drag-shape, preselect), DAG/engine area-select + group-drag + select-all/delete/cancel round-trips, board_host still green after refactor.
- vitest: `flow/react` (modifier pass-through, marquee overlay points, hotkeys), `procedural/react` (3D marquee hit-test + merge modes, coverage), `procedural/play` (selectionMode/method commands, merge-aware setSelection, select-all/delete), `ui/react` (extracted helpers).
- Rebuild flow WASM; verify runtime with `[DEBUG]` logs that selection persists across panes and partial/full/group + modifiers behave like puzzle. Then `ticket_close` listing all touched files.

## Key decisions

- Pan moves off `shift` to middle-mouse + space-drag so puzzle modifier conventions apply in the flow graph.
- Pure selection algorithms are shared (Rust in graph crate; TS in `@semio-tech/ui-react`); puzzle-2d/3d refactored to the shared source rather than duplicating.
- Selection stays transient host/engine state (not persisted in fixture), shared and merge-aware across both procedural panes.
