---
name: Full S Studio and Shell Parity
overview: Fix two shell-wide regressions affecting every program (redo-shadowed-by-undo, dead desktop panel tab clicks), restore missing keyboard/context-menu/drag-drop interactions in the shared node-graph host and S studio, restore cross-program media export coverage (SVG/PNG for 9 plugins, OBJ/GLB regression in puzzle/5d), and add a minimal error boundary around the shell's render tree.
todos: []
isProject: false
---

# Full S Studio and Shell Parity — Round 3

A fresh, deeper audit (beyond the previously-completed `s_studio_parity_restoration_3b748184.plan.md`) found that the prior pass introduced two **new shell-wide regressions** affecting every program, plus surfaced several real gaps that were missed before. This plan fixes all of them.

## Phase 0 — Critical shell-wide regressions (affect every program)

### 0.1 Keybinding modifier-exactness bug (redo shadowed by undo)

[framework/renderer/react/os-shell.tsx:1156-1166](framework/renderer/react/os-shell.tsx) only checks that _required_ modifiers are present, never that unwanted ones are absent:

```1156:1166:framework/renderer/react/os-shell.tsx
const matches = (event: KeyboardEvent, binding: string) => {
    const parts = binding.split("+").map((part) => part.trim());
    const key = parts[parts.length - 1] ?? "";
    const needsCtrl = parts.includes("ctrl") || parts.includes("meta") || parts.includes("mod");
    const needsShift = parts.includes("shift");
    const needsAlt = parts.includes("alt");
    if (needsCtrl && !(event.ctrlKey || event.metaKey)) return false;
    if (needsShift && !event.shiftKey) return false;
    if (needsAlt && !event.altKey) return false;
    return event.key.toLowerCase() === key;
};
```

Since every program declares `mod+z` (undo) before `mod+shift+z` (redo), and the loop returns on first match, **Cmd/Ctrl+Shift+Z currently fires undo instead of redo for every plugin** (draw, writer, note, raster, layout, s, and 10+ more). Fix: require exact modifier match (`needsCtrl === hasCtrl`, `needsShift === event.shiftKey`, `needsAlt === event.altKey`).

### 0.2 Desktop side-panel tab clicks are no-operations for every program except S Studio

[framework/renderer/react/os-shell.tsx:1629-1653](framework/renderer/react/os-shell.tsx) always supplies a controlled `onActiveTabChange`/`activeTabId` pair to the left/right `SidePanel`s, but the callback body only does something `if (studioMode && session?.app.id === S_PLAY_APP_ID)`. Because [ui/js/react/index.tsx:14072-14078](ui/js/react/index.tsx)'s `SidePanel` only falls back to its own working internal tab state `if (!onActiveTabChange)`, and `activeTabId` is always forced back to `workbenchLeftTabs[0]?.id`/`detailsRightTabs[0]?.id` for non-S sessions, **clicking any panel tab besides the first is dead for every other plugin** (draw, note, raster, layout, forms, puzzle/2d/3d/5d, cad, procedural/2d/3d, writer, gis/2d, and more — nearly all of which declare 2+ tabs per side).

Fix: only pass a controlled `activeTabId`/`onActiveTabChange` pair when S Studio's own command state needs to drive it; otherwise pass `undefined` for both so `SidePanel`/`MobilePanel` self-manage via their internal state. Since `activeLeftPanelKind`/`activeRightPanelKind` (workbench/display, details/settings) toggle the entire `tabs` array and `internalActiveTab` only initializes once via `useState`, key the `SidePanel` instances (e.g. `key={activeLeftPanelKind}` / `key={activeRightPanelKind}`) so switching kind still resets to the first tab of the newly-selected group.

## Phase 1 — Keyboard interactions in the shared node-graph host

[framework/renderer/react/components/node-graph-host.tsx](framework/renderer/react/components/node-graph-host.tsx) registers no `onKeyDown` handler at all, so Delete/Backspace/Mod+A/Escape do nothing in S's media graph (old `FlowCanvas` wired these). Fix, generically for any `NodeGraphScene` consumer:

- Wire a keydown handler (reusing the same `isEditableTarget` guard pattern from `os-shell.tsx`) that dispatches:
  - `Delete`/`Backspace` → a new `"deleteSelection"` command
  - `Mod+A` → dispatch existing `"setMediaNodeSelection"` with all node ids
  - `Escape` → dispatch existing `"setMediaNodeSelection"` with an empty array
- In [s/program/rs/lib.rs:1608-1627](s/program/rs/lib.rs), the current `removeAppInstance` fallback (no `instanceId` given) only resolves `primary_selected_instance_id` (singular) — it does not delete a full multi-selection. Add a `"deleteSelection"` command arm that iterates `selected_instance_ids(...)` (already used by copy/duplicate) and removes all of them, clearing `active_instance_id`/`focused_instance_id`/selection state as needed.

## Phase 2 — Restore dropped media-graph context menu items

[s/program/rs/lib.rs:385-395](s/program/rs/lib.rs)'s `media_graph_context_menu_json()` replaced the old menu's **Select all / Clear selection / Reorganize** with a different (also useful) set. Restore the three dropped items alongside the existing copy/paste/duplicate/rename/remove:

- "Select all" / "Clear selection" → same `setMediaNodeSelection` commands as Phase 1's keyboard wiring.
- "Reorganize" → new `"reorganizeMediaGraph"` command that re-flows all (or selected) instance `MediaGraphPosition`s into a simple grid layout — a clean, from-scratch implementation (not porting the old force-directed algorithm) consistent with a greenfield codebase.

## Phase 3 — Generic window-template drag-and-drop

`Mode` still supports `onTemplateDrop` ([ui/js/react/index.tsx:19661,21093-21175](ui/js/react/index.tsx)) but [os-shell.tsx:1592-1606](framework/renderer/react/os-shell.tsx) never passes it, and the "Windows" catalogue tab ([os-chrome-panels.tsx:79-90](framework/renderer/react/os-chrome-panels.tsx)) renders plain non-draggable leaves. Restore generically: make window-kind catalogue rows draggable, and wire `onTemplateDrop` on `<Mode>` to spawn the dropped window kind at the drop position — this benefits every plugin's "Windows" tab, not just S.

## Phase 4 — Cross-program media export coverage

`framework/product/os/core/rs/media_graph.rs`'s own test (`export_coverage_reports_missing_handlers`) documents that most resource kinds have no export handler. `s/program/rs/lib.rs`'s `exportMedia` command silently no-operations for any technology without one ([s/program/rs/lib.rs:1739-1767](s/program/rs/lib.rs)).

1. **puzzle/5d regression (quick, true regression):** restore the OBJ/GLB `register_os_media_export_handler` calls that exist for the other 3D techs (`cad/program/rs/lib.rs:1062-1083`, `procedural/3d/program/rs/lib.rs`, `puzzle/3d/program/rs/lib.rs`, `lowpoly/program/rs/lib.rs`) but are entirely absent from `puzzle/5d/program/rs/lib.rs`, mirroring the same pattern.
2. **Shared PNG rasterization helper:** add a small `web_sys`-based canvas rasterizer (same pattern already used for `LocalStorageBackbonePort` in `vcs/rs/lib.rs`) in a shared crate, taking an SVG string + dimensions and returning PNG bytes, so every 2D program can reuse it instead of duplicating canvas glue.
3. **SVG (+PNG) export for the 9 2D plugins never wired:** draw, note, gis/2d, layout, procedural/2d, puzzle/2d, raster, shooting, presentation. For each, write a `<plugin>_document_to_svg()` serializer against that plugin's own document schema (porting the old TypeScript algorithm from `git show f8376e848:<plugin>/core/js/index.ts` as the reference, e.g. `drawDocumentToSvg` at `draw/core/js/index.ts` — already located during the audit) and register `Svg`/`Png` handlers via `register_os_media_export_handler`, following the exact call-site pattern already used in `cad/program/rs/lib.rs:1062-1083`.
4. Confirm `assert_os_media_export_coverage()` passes (or update its test expectation) once all resource kinds have handlers.

## Phase 5 — Minimal error boundary around the shell render tree

Add a small React error boundary component in `framework/renderer/react/os-shell.tsx` (or a new co-located file if a region-based split is cleaner) wrapping the windows/panels render tree, so one spawned program instance's render exception shows a contained fallback instead of blanking the entire OS shell (all windows/panels of all instances).

## Verification

- `cargo test` for every touched crate (`s-program`, `cad`, `procedural-3d`, `puzzle-3d`, `puzzle-5d`, `lowpoly`, `draw`, `note`, `gis-2d`, `layout`, `procedural-2d`, `puzzle-2d`, `raster`, `shooting`, `presentation`, `framework-product-os-core`, `vcs`).
- `cargo check --target wasm32-unknown-unknown` for all touched plugins.
- `framework-renderer-react` vitest suite.
- S studio E2E script (`s-studio-e2e-verify.mjs`).
- Manual smoke checks: redo keybinding, clicking a second panel tab on a non-S program (e.g. draw), Delete/Mod+A/Escape in S's media graph, "Reorganize" context menu item, dragging a window template onto canvas, exporting SVG/PNG from at least 2 of the newly-wired 2D plugins and OBJ from puzzle/5d.

## Todos

</plan>
<parameter name="todos">[{"id": "phase0-redo-keybinding", "content": "Fix matches() in os-shell.tsx to require exact modifier match so redo (mod+shift+z) is no longer shadowed by undo (mod+z) across all plugins"}, {"id": "phase0-panel-tab-clicks", "content": "Fix desktop SidePanel tab clicks being dead for non-S plugins: only pass controlled activeTabId/onActiveTabChange when S Studio needs it, else let SidePanel self-manage with a kind-keyed remount"}, {"id": "phase1-graph-keyboard", "content": "Wire Delete/Backspace/Mod+A/Escape keydown handling into node-graph-host.tsx, dispatching a new deleteSelection command and the existing setMediaNodeSelection command"}, {"id": "phase1-delete-selection-command", "content": "Add deleteSelection command in s/program/rs/lib.rs that removes the full multi-selection (not just the primary selected id)"}, {"id": "phase2-context-menu-restore", "content": "Restore Select all / Clear selection / Reorganize items in media_graph_context_menu_json alongside existing copy/paste/duplicate/rename/remove; implement reorganizeMediaGraph as a grid re-flow of MediaGraphPosition"}, {"id": "phase3-template-drag-drop", "content": "Wire onTemplateDrop on Mode in os-shell.tsx and make Windows catalogue rows draggable so dragging a window template onto the canvas spawns it, for all plugins"}, {"id": "phase4-puzzle5d-export", "content": "Restore puzzle/5d OBJ/GLB register_os_media_export_handler calls (regression vs cad/procedural3d/puzzle3d/lowpoly pattern)"}, {"id": "phase4-png-helper", "content": "Add shared web_sys canvas-based SVG-to-PNG rasterization helper crate/module reusable by all 2D plugins"}, {"id": "phase4-2d-export-handlers", "content": "Implement document-to-SVG serializers and register Svg/Png export handlers for draw, note, gis/2d, layout, procedural/2d, puzzle/2d, raster, shooting, and presentation plugins"}, {"id": "phase4-coverage-assert", "content": "Verify assert_os_media_export_coverage() passes once all resource kinds have handlers"}, {"id": "phase5-error-boundary", "content": "Add a minimal React error boundary around the windows/panels render tree in os-shell.tsx"}, {"id": "verify-all-round3", "content": "Run cargo test/check for all touched crates, framework-renderer-react vitest, S studio E2E, and manual smoke checks for keybindings, panel tabs, graph keyboard shortcuts, drag-drop, and exports"}]
