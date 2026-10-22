---
name: Note Infinite Canvas App
overview: Create a new fully-featured S technology app "note" — an infinite canvas with text, image, table, math, and pencil-ink blocks — fully integrated with hover/selection/document/window options, VCS persistence, playground dev host, and S plugin registration.
todos:
 - id: ticket
   content: Read repo://goals and open ticket via ticket_open
   status: completed
 - id: core-model
   content: Create note/core with NoteDocument model, blocks, edit operations, VCS handler, tests
   status: completed
 - id: core-playground
   content: Implement NotePlayController, windows, tools, measures, engagement, panel trees, notePlayAppDefinition
   status: completed
 - id: react-canvas
   content: Create note/react NoteCanvas with pan/zoom, block editing, pencil, selection/hover, math renderer interface
   status: completed
 - id: platform
   content: Add note ComponentKind, UiNoteHostSurfaceNode, buildNoteWindowBody to platform core
   status: completed
 - id: renderer-boot
   content: Add bootNotePlay, surface host, side panels to playground renderer react
   status: completed
 - id: registry-ports
   content: Register note in app-registry, dev script, PLAYGROUND_PORTS
   status: completed
 - id: s-integration
   content: Add S resource kind, program definition merge, VCS handler registration
   status: completed
 - id: commands
   content: Add dev:note/test:note scripts and launch.json entry
   status: completed
 - id: verify
   content: Run tests, boot dev:note and dev:s, verify runtime with debug logs, close ticket
   status: completed
isProject: false
---

# Note Infinite Canvas App

## Context

"S apps" are technology packages (like `draw`, `writer`, `raster`) that ship a playground app definition and register into S as a program. Draw ([draw/core/playground.ts](draw/core/playground.ts), [draw/react/index.tsx](draw/react/index.tsx)) is the closest full template: infinite pan/zoom camera, pen tool, layer document, hover/selection sync via `AppPointerFocusStore`, window measures/engagement, VCS-backed document. There is no existing table-block or math typesetting on canvas — those are new.

Work happens inside a repo ticket (open via `ticket_open` after reading `repo://goals`, associate with the most fitting goal).

## New package: `note/`

Mirror draw's layout: `note/core/` (`index.ts`, `internal.ts`, `playground.ts`, `fixture-slugs.ts`, `package.json`, `project.json`, `script.ts`, `vitest.config.ts`), `note/react/` (`index.tsx` + config files), `note/fixture/semio.note.json` demo fixture, `note/manifest/blocks.manifest.json`.

### Document model (`note/core/internal.ts`)

- `NoteDocument`: `schema: "note.document"`, `id`, `camera: { x, y, zoom }`, `blocks: NoteBlockNode[]`, `assets`, `activeTool`.
- Block kinds (each with `x/y/width/height`, rotation optional):
  - `text` — rich text content with basic styling (size, weight, align)
  - `image` — asset reference like `DrawImageLayer`
  - `table` — structured `{ columns, rows, cells }`, editable per cell
  - `math` — TeX source string, display/inline mode
  - `ink` — freehand pencil strokes (points + width + color), from pen-drag like draw's `pen` → path
  - `group` — nesting for document
- Edit operations (`NoteEditOp`) + VCS envelope + `createNoteAppVcsHandler()` following `createDrawAppVcsHandler` ([draw/core/internal.ts](draw/core/internal.ts) ~line 1386).
- Pointer-focus key encoding `note:${kind}:${id}`, `NoteHoverPayload`, in-file vitest tests via `import.meta.vitest`.

### Playground (`note/core/playground.ts`)

Follow `PlaygroundDraw` / `DrawPlayController`:

- `NotePlayController` with `pointerFocus`, `DocumentVcsStore`, commands: `setSelection`, `setHover`, block CRUD, camera, tool switching, undo/redo checkpoints.
- Windows: **Canvas** (composite) + **Navigator** (minimap/outline), each `WindowKindRuntime` with:
  - Measures: zoom slider, grid toggle, pencil width slider, snap toggle
  - Engagement: command input (rename block, quick-add), status (selection count, zoom)
- Tools: `selectDirect`, `selectMarquee`, `pan`, `text`, `image`, `table`, `math`, `pencil`, `eraser`.
- Side-panel trees: `buildNotePlayDocumentTree()` (blocks with hover sync via `CANVAS_HOVER_SOURCE_DOCUMENT`, selection, reorder/delete actions), `buildNotePlayCatalogueTree()` (drag block templates), `buildNotePlayInspectorTree()` (per-kind property groups).
- `notePlayAppDefinition: PlaygroundAppDefinition` with `devHost.playEntryKind: "note"`, `bootRenderer` importing `@semio-tech/framework-playground-renderer-react/note`.

### React renderer (`note/react/index.tsx`)

`NoteCanvas` component modeled on `DrawCanvas`: SVG/HTML infinite canvas with pan/zoom, block rendering per kind, hover highlight, selection handles + move/resize, marquee, pencil drag → ink strokes, in-place text/table/math editing. External libs wrapped behind interfaces per repo rule:

- `NoteMathRenderer` interface with a KaTeX-backed default (dep only in `note/react`), settable like `MarkdownHtmlCompiler` in [framework/product/presentation/renderer/react/markdown.ts](framework/product/presentation/renderer/react/markdown.ts).

## Framework integration (fully integrated)

- [framework/product/platform/core/index.ts](framework/product/platform/core/index.ts): add `ComponentKind` `"note"`, `UiNoteHostSurfaceNode`, `buildNoteWindowBody()` (mirror `buildDrawWindowBody`, line ~931).
- [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx): `bootNotePlay`, `NotePlayPaneSurfaceHost` wiring `NoteCanvas`, side panel tabs (document/catalogue/inspection) — mirror the draw sections; add `"./note"` export + deps in its `package.json`.
- [framework/product/playground/core/app-registry.ts](framework/product/playground/core/app-registry.ts): add `note` loader.
- [framework/product/playground/dev/script.ts](framework/product/playground/dev/script.ts): `PACKAGE_ROOT_BY_ENTRY.note = "note"`.
- [repo/lib/js/index.ts](repo/lib/js/index.ts): `PLAYGROUND_PORTS.note = { dev: 6080, test: 6081, env: "NOTE_PLAY_PORT" }` + `PlaygroundHostKind`.

## S integration

- `note/core/index.ts`: `buildNoteWorkflowDefinition()` like `buildDrawWorkflowDefinition` ([draw/core/index.ts](draw/core/index.ts)).
- [s/manifest/artifacts.manifest.json](s/manifest/artifacts.manifest.json): add `{ "id": "2d.note", "sourceFormat": "note.document", "componentKind": "note", "dimension": "2d" }`.
- [s/core/internal.ts](s/core/internal.ts): `TECHNOLOGY_APP_RESOURCE_BY_PROGRAM.note` entry (~line 184).
- [s/core/program-extensions.ts](s/core/program-extensions.ts): lazy `mergeSWorkflowDefinition("note", ...)` + `registerAppVcsHandler(createNoteAppVcsHandler())`.
- [s/core/playground.ts](s/core/playground.ts): register note in the S play test extensions/fixture loading; add `@semio-tech/note-core` devDep to `s/core/package.json`.

## Tooling and commands

- Root [package.json](package.json): `dev:note`, `test:note` scripts (nx-based, matching `dev:draw`/`test:draw`).
- [.vscode/launch.json](.vscode/launch.json): `🛠️dev📝️note` entry (`bun run dev:note`, `NOTE_PLAY_PORT=6080`, group `3_dev`, ordered next to draw/writer).
- Verify workspace globs pick up `note/*` packages; add if needed.

## Verification

- Run in-file vitest for `note/core` and `note/react`.
- Boot `dev:note` and confirm runtime behavior with `[DEBUG]`-prefixed logs: block creation for all kinds, pencil drawing, hover/selection sync canvas <-> document, window measures, undo.
- Boot `dev:s`, spawn a note app on the media graph, confirm the VCS handler persists edits.
- Close the ticket with summary and file list.
