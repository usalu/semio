---
name: Puzzle 3D Brush Fill Context Menu Fix
overview: Fix three confirmed root-cause bugs preventing the puzzle 3D brush tool, fill tool, and context menu from working, then add a net-new context menu (which never existed for any world-3d plugin) to reach premigration parity.
todos:
  - id: fix-fill
    content: Fix setFillCount to accept both count and value keys (d3/mod.rs)
    status: completed
  - id: fix-brush-sync
    content: Sync precompute session unconditionally at top of handle_command_patch_ops (d3/mod.rs)
    status: completed
  - id: context-menu-core
    content: Add context_menu_json to World3dScene (core/rs, plugin/rs) and update 3 callers
    status: completed
  - id: context-menu-puzzle3d
    content: Build context menu items + duplicateSelection/selectSameKindSelection handlers in d3/mod.rs
    status: completed
  - id: context-menu-react
    content: Wire ContextMenuController + zoom-to-selection into world-3d-host.tsx and os-shell.tsx types
    status: completed
  - id: validate-close
    content: Run vitest + cargo test, manual verify, update verify-log, reopen/close ticket
    status: completed
isProject: false
---

# Puzzle 3D: Fix Brush, Fill, and Add Context Menu

Root-caused all three complaints by tracing the exact command/data flow. Each has a concrete, confirmed fix.

## 1. Fill is broken — wrong argument key

`framework/renderer/react/os-shell.tsx`'s generic engagement `Slider` dispatcher (`windowEngagementControlToSpec`, ~`os-shell.tsx:396-411`) always sends `{ value: newValue }` on change. But [puzzle/plugin/rs/d3/mod.rs](puzzle/plugin/rs/d3/mod.rs)'s `"setFillCount"` handler (~line 1351-1366) only reads `args.get("count")`, so `count` is always `0` and `envelope.runtime.fill_count` gets reset to `0` on every slider move — fill silently no-ops.

The sibling `puzzle/plugin/rs/d2/mod.rs` already handles this correctly (line 1669-1674): `args.get("count").or_else(|| value.get("value"))`.

**Fix:** apply the same `.or_else` fallback to d3's `"setFillCount"` handler.

## 2. Brush is broken — precompute session never synced on hover

`Puzzle3dPrecomputeSession` (`self.precompute`) is only synced (`sync_precompute_session`) inside specific command branches (`worldVortexSelect`, `addBrushObject`, `setFillCount`, etc.), but **not** in `"worldVortexHover"` (~line 1224) or `"engagementPossibleSelect"` (~line 1310). Since `render()` and `window_engagements()` take `&self` and read `self.precompute.brush_candidates(...)` / `brush_preview_json(...)` without mutating it, `self.precompute.scene` stays `None` until some unrelated command happens to sync it — so hovering a vortex marker in Brush mode never produces a `brushPreview`, and `WorldVortexMarkers`'s click handler (`world-3d-host.tsx:1363-1373`, `handleBrushPlace`) silently returns early (`if (!brushPreview) return;`). The brush candidate `ToggleGroup` in the engagement rail is empty for the same reason.

`puzzle/plugin/rs/d2/mod.rs` avoids this entirely by calling `sync_host_from_envelope(&mut self.host, &envelope)` unconditionally at the top of every command, before the `match`.

**Fix:** mirror that pattern — add `sync_precompute_session(&mut self.precompute, &envelope);` right after `let mut envelope = parse_envelope(document_json);` at the top of `handle_command_patch_ops` (`puzzle/plugin/rs/d3/mod.rs:1008`), so `self.precompute.scene` is always fresh before any render/engagement read. This is cheap (`set_scene` + registering a small fallback mesh under each URL, no `precompute_step`). The existing scattered `sync_precompute_session`/`drive_precompute` calls in specific branches can stay (now redundant but harmless, and `drive_precompute` still adds real precompute warm-up).

## 3. No context menu — feature never existed for any world-3d plugin

Confirmed via search: no `lowpoly`, `cad`, `shooting`, `procedural`, or `puzzle` world-3d plugin implements a context menu, and `World3dScene` has no `contextMenuJson` field at all. This is net-new, following the exact pattern already used by `gis-map-host.tsx` / `node-graph-host.tsx` (`contextMenuJson` on the scene type, local `{x,y}` React state opened via a DOM `contextmenu` event, `ContextMenuController` from `@semio-tech/ui-react`).

Premigration reference (`buildPuzzle3dSelectionMenuItems`) had: Hide/Show, Lock/Unlock, Duplicate, Select-same-kind, Zoom-to-selection, Delete, plus an AI "Suggest objects" entry. Scoping this pass to what maps onto **existing** backend capability plus small additions, skipping Hide/Lock (would require new `hidden`/`locked` fields threaded through `Puzzle3dObject`, instances JSON, and renderer visibility/interaction filtering — a separate feature) and skipping "Suggest objects" (AI feature, unrelated to this bug fix):

- Delete Selection (reuse existing `"deleteSelection"` handler, `d3/mod.rs:1088`)
- Duplicate Selection (new `"duplicateSelection"` command: clone each selected `Puzzle3dObject` with `next_object_id()`, small origin offset, select the new copies — mirrors the existing `"addObjectKind"` handler shape at `d3/mod.rs:1055-1086`)
- Select Same Kind (new `"selectSameKindSelection"` command: from the first selected object's `object_kind`, set `selection.object_ids` to all objects sharing that kind)
- Zoom to Selection (client-side only in `world-3d-host.tsx`: compute a bounding sphere from selected objects' positions and dispatch the existing `"setCamera"` command with a new camera position/target — no new Rust command needed)

**Implementation:**

- `framework/core/rs/lib.rs`: add `context_menu_json: Option<String>` field to `World3dScene` (~line 2189-2214), `#[serde(default, skip_serializing_if = "Option::is_none")]`.
- `framework/plugin/rs/lib.rs`: add `context_menu_json: Option<String>` parameter to `world3d_scene_extended(...)` (~line 2013-2043).
- Update the 3 callers of `world3d_scene_extended`: `puzzle/plugin/rs/d3/mod.rs` (pass real menu json), `puzzle/plugin/rs/d5/mod.rs` and `cad/plugin/rs/lib.rs` (pass `None`, no behavior change).
- `puzzle/plugin/rs/d3/mod.rs`: add `fn puzzle3d_context_menu_json(envelope: &Puzzle3dEnvelope) -> Option<String>` returning `None` when `selection.object_ids` is empty, else the 4 items above with `command`/`args` shaped like other menu-driven UI nodes (`{id, label, command, args}`); wire into the `render()` call for `PUZZLE3D_PLAY_BODY_COMPOSITE`. Add the two new command handlers (`duplicateSelection`, `selectSameKindSelection`).
- `framework/renderer/react/os-shell.tsx`: add `contextMenuJson?: string` to the `World3dScene` type.
- `framework/renderer/react/components/world-3d-host.tsx`: import `ContextMenuController` from `@semio-tech/ui-react`; parse `contextMenuJson` via the existing `parseJsonArray` helper; add local `const [contextMenu, setContextMenu] = useState<{x,y} | null>(null)`; add `onContextMenu` to the existing outer host `<div>` (`world-3d-host.tsx:1603-1611`) that calls `event.preventDefault()` and opens the menu when items are non-empty; render `<ContextMenuController open=... position=... items=... onOpenChange=... />` mapping each item's `command`/`args` through `dispatch`.
- `framework/renderer/react/index.test.ts`: extend the existing `World3dScene` test case with `contextMenuJson`.

## Validation

- `bun nx run @semio-tech/framework-renderer-react:test`
- `cargo test -p puzzle-plugin` (d3 module compiles; native test run remains blocked by the pre-existing wasm-only `plugin_exports!` macro, as before)
- Manual browser check in the ticket's verify-log: switch to Brush, hover a vortex marker (candidates + ghost preview appear), click to place; switch to Fill, drag slider (objects actually populate); right-click a selected object (menu appears with Duplicate/Select Same Kind/Zoom/Delete, each working)

All work continues inside the existing `PUZZLE-3D-REACT-PARITY` ticket (reopen it) — no new ticket needed.
