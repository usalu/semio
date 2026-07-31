---
name: Flip VCS History And Wire Play Shell
overview: "Rewire `vcs/play` onto the standard Platform/Controller/Window playground shell (instead of a bare `createRoot` page), and flip `HistoryTable` from a horizontal 3-row-per-column grid into a vertical commit-graph list: one minimal-height row per checkpoint (newest on top), a graph/lane column with vertical parent-connector lines, and labels hung off their node with a horizontal connector."
todos: []
isProject: false
---

## Current state

- `vcs/play` bootstraps with a bare `createRoot(...).render(<VcsPlayApp />)` ([vcs/play/index.tsx:69-71](vcs/play/index.tsx)) — no `Platform`/`Controller`/`WindowKindRuntime`, unlike every other `*/play` (e.g. `draw/play`, `forms/play`, `semios/play`) which boots through `Playground` + `bootXPlay`.
- `HistoryTable` ([vcs/react/index.tsx](vcs/react/index.tsx)) is a horizontal CSS grid: one **column** per checkpoint (oldest→newest, left→right), 3 fixed **rows** (`labels` / `parent` avatars+lane-SVG / `description`). `HistoryLaneSvg` (lines 24-59) draws parent→child connectors as horizontal polylines _inside_ the shared "parent" row; `column.lane` only offsets where a line starts vertically within that single row.
- `buildHistoryColumns()` / `HistoryColumn` ([vcs/core/index.ts:79-88, 151-187](vcs/core/index.ts)) already carry everything needed for a graph view: `parentCheckpointId`, `lane`, `labels`, `authors`, `description`, sorted oldest-first.
- The codebase already has a vertical "gutter" line-drawing primitive for exactly this kind of parent/child connector: `TreeDocumentGutter` in `ui/react/index.tsx:9522-9552` draws a vertical `tree-branch-stem` line plus a horizontal `tree-branch-elbow` line using the shared `treeGuideLineStrokeClassName` token, and tree rows use a fixed compact height (`h-workbench` / `min-h-workbench`, token `treeRowUiSpacing`, `ui/react/index.tsx:9420-9422`). This is the reference for "minimal row height" and "horizontal line to parent."
- Precedent for wiring a new technology into the play shell already exists for `draw` (see `buildDrawWindowBody` at `framework/product/platform/core/index.ts:889-904` and `registerDrawPlaySurfaceHosts` / `DrawPlayChrome` / `bootDrawPlay` at `framework/product/playground/renderer/react/index.tsx:9793-9812`, plus the `"./draw"` export + deps in `framework/product/playground/renderer/react/package.json`).

## Part 1 — Wire `vcs/play` onto the play shell

Mirror the `draw`/`forms` pattern exactly:

1. **`framework/product/platform/core/index.ts`**: add `"vcs"` to `ComponentKind` and `CANVAS_COMPONENT_KINDS` (lines 307, 309); define `UiVcsHostSurfaceNode` (`type: "vcs"`, `componentKind: "vcs"`, `surfaceId`, `controllerId`, `view: "editor" | "history"`, optional `paneId`/`bindingId`); add `buildVcsWindowBody(surfaceId, controllerId, view, paneId?, bindingId?)` mirroring `buildDrawWindowBody`.
2. **`framework/product/playground/renderer/react/index.tsx`**: new `//#region 🔖️VcsPlayHost` with:
   - `registerUiVcsSurfaceHost(surfaceId, Component)`
   - `VcsPlayEditorSurfaceHost` — the toolbar (Counter / Commit checkpoint / Undo / Redo / New alternative) and status line currently inlined in `vcs/play/index.tsx`
   - `VcsPlayHistorySurfaceHost` — renders the redesigned `HistoryTable` bound to the controller
   - `registerVcsPlaySurfaceHosts()`, `VcsPlayChrome`, `mountVcsPlayChrome`, `vcsPlayChromeBoot`, `bootVcsPlay(playground)` — mirrors `registerDrawPlaySurfaceHosts`/`DrawPlayChrome`/`mountDrawPlayChrome`/`bootDrawPlay`
3. **`framework/product/playground/renderer/react/package.json`**: add `"./vcs": "./index.tsx"` export; add `@semio-tech/vcs-play` and `@semio-tech/vcs-react` (`workspace:*`) to `dependencies`.
4. **Rewrite `vcs/play/index.tsx` → `vcs/play/index.ts`** (drop JSX, same split as `draw/play/index.ts`):
   - `VcsPlayController extends Controller` — wraps the existing `createVcsDemoStore()` / `seedVcsDemoHistory()` from `vcs/play/demo.ts`; exposes `projection()` / `historyColumns()` / `getEnvelope()`; implements `run(command, args)` dispatching `apply` / `undo` / `redo` / `commitCheckpoint` / `createAlternative` onto the store; subscribes to the store so every mutation calls `notifyPlatform()` (this replaces the ad hoc `useSyncExternalStore(store.getGeneration())` hook that lived in the deleted `VcsPlayApp`).
   - IDs + layout: `VCS_PLAY_APP_ID`, `VCS_PLAY_CONTROLLER_ID`, `VCS_PLAY_WINDOW_EDITOR`, `VCS_PLAY_WINDOW_HISTORY`, matching surface/body-key constants, `VCS_PLAY_LAYOUT = createDefaultLayout([EDITOR, HISTORY], "row", [30, 70], ["Editor", "History"])`.
   - `buildVcsPlayAppRuntime(ctrl)`, `registerVcsPlayDeclarativeBodies()`, `class PlaygroundVcs extends Playground` (`createRuntime()` / `registerBodies()` mirroring `PlaygroundDraw`).
   - Entry point: `const { bootVcsPlay } = await import("@semio-tech/framework-playground-renderer-react/vcs"); bootVcsPlay(new PlaygroundVcs());` (same dynamic-import shape as `draw/play/index.ts:1448-1449`).
5. **`vcs/play/index.html`**: point `<script src>` at `./index.ts`.
6. **`vcs/play/package.json`**: add `@semio-tech/framework-core`, `@semio-tech/framework-playground-core`, `@semio-tech/framework-playground-renderer-react` workspace deps (mirror `draw/play/package.json`).
7. Delete the retired `VcsPlayApp` component / `useVcsDemoStore` hook; their logic is absorbed into `VcsPlayController` and the two surface hosts.

This keeps the History content itself a normal scrollable DOM panel (no WASM/WebGPU pan-zoom canvas from `infinite/cavas`) — just hosted inside a real playground window instead of a standalone page, per your answer ("looks like a regular UI, not zoomable/pannable, just window options").

## Part 2 — Flip `HistoryTable` into a vertical commit-graph list

**2a. `vcs/core/index.ts`** — `buildHistoryColumns()` (lines 151-187): sort **newest-first** (reverse the current ascending sort) instead of oldest-first; keep `HistoryColumn`'s fields (`parentCheckpointId`, `lane`, `labels`, `authors`, `description`) unchanged — they already fit a "one entry per row" model.

**2b. `vcs/react/index.tsx`** — rewrite `HistoryTable`:

- Replace the "3 fixed rows × N checkpoint columns" grid with "N fixed-height rows × [graph lane | content]" — one row per checkpoint, newest on top.
- **Row height**: use the same compact row-height token the rest of the app uses for list/tree rows (`h-workbench` / `min-h-workbench`, `ui/react/index.tsx:9420-9422`) instead of the old `rowHeight=56` 3-row block, so every History row is exactly as tall as any other list row in the app.
- **Graph column** (replaces `HistoryLaneSvg`'s horizontal-in-one-row polylines): per row, draw a node dot at the row's lane x-position, and a **vertical** stroke connecting each checkpoint's node down to its parent's node (spanning intermediate rows for lanes that persist across them) — same visual language as `TreeDocumentGutter`'s vertical stem (`ui/react/index.tsx:9546-9551`), reusing `treeGuideLineStrokeClassName` / the hairline stroke token rather than inventing new colors/strokes.
- **Label connector**: a short horizontal stroke from each row's lane node to its label chip(s) — same elbow pattern as `TreeDocumentGutter`'s branch elbow (`ui/react/index.tsx:9539-9545`), so labels read as callouts hanging off the graph node.
- Keep `TableAvatar` for authors, placed inline in the row instead of stacked/absolutely positioned.
- Update `HistoryTableProps` (currently `rowLabelWidth` / `columnWidth` / `rowHeight`, lines 9-15): replace with `laneWidth` and a single compact `rowHeight` default tied to the shared UI token.

**2c. `vcs/play`** — no changes to `demo.ts`'s fixture/data; `VcsPlayHistorySurfaceHost` (added in Part 1) renders `<HistoryTable columns={ctrl.historyColumns()} />` in a normal scrollable window pane.

## Verification

- `bun nx run @semio-tech/vcs-core:test`, `@semio-tech/vcs-react:test`, `@semio-tech/vcs-play:test`.
- `bun run dev:vcs` — confirm two tiled windows (Editor / History) render through the real shell; toolbar still drives the store; History shows newest-on-top rows with vertical parent-connector lines and horizontal label connectors at minimal, uniform row height. Verify visually with a screenshot, not just by reading code.
- Per workspace policy this work happens inside a repo ticket (`ticket_open` at start, `ticket_close` with full file list at the end); reuse or reopen `26/07/01/EXTRACT-VCS-TECHNOLOGY` if still relevant, otherwise open a new ticket for this follow-up (it's a distinct scope: shell wiring + graph redesign, not the original entity-model extraction).
  </plan>
  <todos>[{"id": "platform-core-vcs-kind", "content": "Add \"vcs\" ComponentKind, UiVcsHostSurfaceNode, buildVcsWindowBody in framework/product/platform/core/index.ts"}, {"id": "playground-renderer-vcs-host", "content": "Add VcsPlayHost region (surface hosts, chrome, bootVcsPlay) in framework/product/playground/renderer/react/index.tsx"}, {"id": "renderer-package-vcs-export", "content": "Add \"./vcs\" export + vcs-play/vcs-react deps to framework/product/playground/renderer/react/package.json"}, {"id": "vcs-play-controller", "content": "Rewrite vcs/play/index.tsx into vcs/play/index.ts: VcsPlayController + Playground subclass + layout/IDs, delete ad hoc VcsPlayApp"}, {"id": "vcs-play-entry-deps", "content": "Update vcs/play/index.html script src and package.json workspace deps"}, {"id": "vcs-core-sort-order", "content": "Flip buildHistoryColumns() to sort newest-first in vcs/core/index.ts"}, {"id": "vcs-react-vertical-table", "content": "Rewrite HistoryTable in vcs/react/index.tsx as a vertical per-checkpoint row list with a graph/lane column (vertical parent lines + horizontal label connectors), minimal shared row height"}, {"id": "verify-vcs", "content": "Run vcs-core/vcs-react/vcs-play tests and bun run dev:vcs, verify visually via screenshot"}]
