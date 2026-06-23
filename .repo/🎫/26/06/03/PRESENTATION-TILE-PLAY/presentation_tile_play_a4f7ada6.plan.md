---
name: Presentation Tile Play
overview: "Add a dev-only \"play\" sandbox for the presentation framework, built on the @semio-tech/framework-playground shell, that lets devs visually pick parameters for presentation features — first the one-to-many morph tile selector: load a figure, seed overlapping tiles from a grid engagement, drag/resize/rename them, and copy an LLM prompt (with the resulting parameters) to the clipboard."
todos:
  - id: ticket
    content: Read repo://goals and ticket_open a 'Presentation Tile Play' ticket under the framework goal.
    status: completed
  - id: core
    content: "Add 🔖TilePlay region to framework/product/presentation/core/index.ts: FigureTileDraft/FigureTileSource, populateTileDraftsFromGrid, resizeNormalizedRect/moveNormalizedRect, buildTileMorphPrompt; extend the existing in-file test region."
    status: completed
  - id: renderer-host
    content: "Add 🔖PresentationPlayHost to framework/product/playground/renderer/react/index.tsx: FigureTilesSurfaceHost (img + draggable/resizable overlapping rects + marquee), registerSurfaceBinding, PresentationPlayChrome, bootPresentationPlay; update its package.json deps/exports."
    status: completed
  - id: play-app
    content: "Create framework/product/presentation/play package: index.ts (PresentationPlay + controller, grid engagement, tree panel, copy-prompt), index.html, globals.css, package.json, project.json (PRESENTATION_PLAY_PORT=6051), script.ts, vite.config.ts, vitest.config.ts."
    status: completed
  - id: vite-entry
    content: Extend PlaygroundRendererPuzzleKind in ui/styling/vite-elements-assets.ts to include 'presentation' so the boot gate fires.
    status: completed
  - id: launch
    content: Register 🛠️dev📽️presentationplay and 📦build📽️presentationplay in .vscode/launch.json following existing order/grouping.
    status: completed
  - id: verify
    content: Run @semio-tech/framework-presentation-play:dev and confirm seed→drag→resize→rename→copy-prompt at runtime; run :test for new core tests; ticket_close with summary.
    status: completed
isProject: false
---

## Presentation Tile Play

A standalone Vite sandbox at `framework/product/presentation/play/`, on the `@semio-tech/framework-playground` shell, modeled on [puzzle/2d/play](puzzle/2d/play/index.ts). First (and only initially shipped) feature: the **one-to-many morph tile selector**. Architected to host more presentation-feature pickers later (one Playground app, feature-per-mode/window).

Greenfield: no compat layers. Pure geometry/prompt logic lives in `@semio-tech/framework-presentation-core` (testable, DOM-free); interaction lives in the playground React renderer; clipboard write is the only browser side-effect.

### 1. Where it lives & how it launches
- New package `@semio-tech/framework-presentation-play` in `framework/product/presentation/play/` with: `index.ts`, `index.html`, `globals.css`, `vite.config.ts`, `project.json`, `package.json`, `script.ts`, `vitest.config.ts` — mirroring [puzzle/2d/play](puzzle/2d/play/project.json) and [33.projektetage](mit-bestand/präsentation/33.projektetage/index.html).
- Dev port `PRESENTATION_PLAY_PORT=6051` (next free after projektetage 6050).
- `script.ts`: `ScriptRouter` with `dev`/`build`/`test` using `runViteBunxDev`/`runVitest` from `repo/lib/js` (no wasm/cargo, unlike puzzle).
- Register in [.vscode/launch.json](.vscode/launch.json): a `🛠️dev📽️presentationplay` entry in the `🛠️ Dev` region right after `🛠️dev📽️projektetage`, and a `📦build📽️presentationplay` in the `📦 Build` region, both calling `bun nx run @semio-tech/framework-presentation-play:dev|build` with `serverReadyAction` opening `http://localhost:6051`.

### 2. Core additions — `framework/product/presentation/core/index.ts`
New `//#region 🔖TilePlay` (reusing existing `splitFigureGrid`, `unionSourceCrops`, `DispositionPosition`, `tile`, `split`):
- `FigureTileDraft` = `{ id; name; crop: DispositionPosition }` (normalized 0..1; overlap allowed — no dedup/snap).
- `FigureTileSource` = `{ src; sourceAspect?; frame: DispositionPosition }`.
- `populateTileDraftsFromGrid(source, rows, columns, gap?)` → wraps `splitFigureGrid` to seed initial non-overlapping drafts the user then edits.
- `resizeNormalizedRect(rect, handle, dx, dy, min?)` and `moveNormalizedRect(rect, dx, dy)` — pure, clamped to 0..1 (lift the math currently inlined as `resizeDispositionRect` in the renderer so both share one source of truth).
- `buildTileMorphPrompt(source, drafts)` → the NL LLM prompt string (see §5).
- Extend the existing `//#region 🧪Tests` in the same file (do not add a new test file): cover grid seeding count/crops, resize/move clamping, overlap preserved, and prompt contains every tile name + crop.

### 3. Playground renderer host — `framework/product/playground/renderer/react/index.tsx`
New `//#region 🔖PresentationPlayHost` (modeled on the minimal `🔖MapPlayHost`, lines ~4554-4615):
- `FigureTilesSurfaceHost({ node })`: renders the `<img>` cover-fit + an absolutely-positioned overlay of normalized tile rectangles. Supports: pointer-drag move, 8-handle resize (via core `resizeNormalizedRect`/`moveNormalizedRect`), marquee to add a new tile (overlap allowed), click-to-select, syncing selection/edits back to the controller. Reuses existing renderer drag helpers where pure.
- Register via the generic `registerSurfaceBinding(PRESENTATION_PLAY_SURFACE_ID, FigureTilesSurfaceHost)` (the `panel` host path) — no new `componentKind` in `@semio-tech/framework-platform-core`.
- `PresentationPlayChrome` using `<PlaygroundView runtime=… defaultAppId=… initialPanelVisibility={{ leftSidePanel:false, rightSidePanel:true }} />`, `mountPresentationPlayChrome`, a `PlaygroundChromeBoot`, and exported `bootPresentationPlay(playground, rootId)`.
- Add the `@semio-tech/framework-presentation-play` export path to [framework/product/playground/renderer/react/package.json](framework/product/playground/renderer/react/package.json) deps + `exports["./presentation"]`.

### 4. Play definition — `framework/product/presentation/play/index.ts`
- `PresentationPlayController extends Controller`: owns `source` (image src/aspect/frame) + `tiles: FigureTileDraft[]` + `selectedIds`. Commands: `seedGrid` (from engagement), `addTile`, `deleteTile`, `renameTile`, `setTileCrop` (from canvas drag/resize), `setFrame`, `setSource`, `clearTiles`, `copyPrompt`.
- Window body = `buildPanelWindowBody(PRESENTATION_PLAY_SURFACE_ID, controllerId)`.
- **Grid engagement** (the requested "grid engagement"): a `WindowEngagement.input` command line (per puzzle pattern) where typing e.g. `3x5` runs `seedGrid`, plus `possibleEngagements` (`3x5`, `add`, `clear`, `copy prompt`). Toolbar tools mirror these as buttons; a **Copy prompt** button runs `copyPrompt`.
- Right **details panel**: a `tree` listing tiles (editable semantic name, shows crop x/y/w/h); selecting a row highlights the rectangle on the canvas and vice-versa.
- `copyPrompt` builds the string via core `buildTileMorphPrompt` and writes it through `navigator.clipboard.writeText` in the host (only the renderer touches the clipboard; core stays DOM-free).
- Image loading: text input/drag-drop of an image path or URL (dev-only); default frame = full image `{0,0,1,1}`.
- Boot gate at bottom: `import.meta.env.PUZZLE_PLAY_ENTRY === "presentation"` → `bootPresentationPlay(new PresentationPlay())`.

### 5. The clipboard prompt (NL with embedded params)
`buildTileMorphPrompt` emits a natural-language instruction for an LLM that embeds the structured parameters and points at the real authoring pattern:
- Source image path/URL + pixel `sourceAspect`, and the normalized `frame`.
- For each tile: semantic `name` + normalized `crop {x,y,width,height}` (6-dp).
- Instructions: wire these as a one-to-many morph in a deck like [33.projektetage](mit-bestand/präsentation/33.projektetage/spec.ts) — define participants/`tile(...)` figure embodiments, the full-figure disposition with `morphTo: MorphToSlot[]` at the tile grid positions, referencing `@semio-tech/framework-presentation-core` (`tile`, `split`, `MorphToSlot`, `DispositionPosition`).

### 6. Vite entry-kind plumbing — `ui/styling/vite-elements-assets.ts`
- Extend `PlaygroundRendererPuzzleKind` to include `"presentation"` so `playEntryKind: "presentation"` sets `import.meta.env.PUZZLE_PLAY_ENTRY` and the boot gate fires (lines ~321-426).
- `framework/product/presentation/play/vite.config.ts`: `createPlaygroundPlayViteConfig({ playDir, repoRoot, playEntryKind: "presentation", extraAliases: [@semio-tech/framework-presentation-core, /react renderer] })`.

### 7. Process (repo MCP / ticket)
Before coding: read `repo://goals`, then `ticket_open` a ticket (associate with the `framework` goal, matching the prior presentation-framework ticket) titled e.g. "Presentation Tile Play". Keep any scratch artifacts inside the ticket folder; `ticket_close` with a summary + touched files when done. Validate by running the play (`:dev`, confirm seed→drag→resize→rename→copy works via console/runtime) and `:test` for the new core tests.