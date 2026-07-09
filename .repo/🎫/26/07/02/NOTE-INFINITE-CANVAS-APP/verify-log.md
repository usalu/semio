# Verify Log — Note Infinite Canvas App (E2E)

## Grid & window options (2026-07-03)

### Grid

- Replaced finite 8000×8000 world-space SVG with viewport-aligned `NoteViewportGrid` (screen space, camera-offset pattern)
- Grid fills entire canvas at any pan/zoom (`hasViewportGrid: true`, canvas SVG rect matches viewport)
- Document fields: `gridSpacing`, `gridSubdivisions`, `gridOpacity`, `snapGridSpacing`
- Snap on block place/paste and on move release when `snapEnabled`

### Window options (measures rail)

- Grouped sections: **Camera** (zoom), **Grid** (show, major spacing, subdivisions, opacity), **Snap** (enable, spacing), **Drawing** (pencil, eraser)
- Toggles use `iconId` + descriptive `text` labels
- Engagement status shows grid/snap summary

### Tests

- `bun run test:note` — PASSED (note-core 11 tests, note-react 2 tests)

### Runtime verify

- http://127.0.0.1:6080/ — Window Options rail shows CAMERA / GRID / SNAP / DRAWING groups with labeled sliders

## Dev host E2E

- `NOTE_PLAY_PORT=6080 bun run dev:note` — Vite ready at http://127.0.0.1:6080/
- `PUZZLE_PLAY_ENTRY=note` confirmed in served bundle
- Browser boot: Note shell with Canvas + Navigator windows, toolbar tools (Direct, Marquee, Text, Image, Table, Math, Pencil, Stroke Eraser, Point Eraser, Pan), Import/Export Note
- Canvas renders after `UiRenderer` note case fix (no "Unsupported UiNode")

## Feature completion (2026-07-02)

### Core model (`note/core/internal.ts`)

- Rich text: `NoteTextParagraph` / `NoteTextRun` with bold/italic/underline/link marks
- Eraser tools: `eraserStroke`, `eraserPoint` + `eraserRadius` document field
- Geometry: `noteSelectionBounds`, `noteScaleBlockWithinGroup`, `noteResizeBounds`
- Ink erase: `noteEraseInkStrokeAtPoint`, `noteEraseInkPointsNearPoint` (splits strokes)
- Clipboard: `noteClipboardPayload`, `noteBlocksFromClipboardPayload`, `noteCloneBlocksWithOffset`
- Image assets: `noteImageAssetDataUrl`, `createNoteImageAssetFromDataUrl`, `createNoteImageAssetKey`

### Controller (`note/core/index.ts`)

- Commands: `deleteSelection`, `duplicateSelection`, `clearSelection`, `nudgeSelection`, `undo`, `redo`
- Table inspector: Add/Remove Row/Column buttons
- Eraser radius slider + stroke/point eraser tool toggles

### Keybindings (`note/core/playground.ts` + renderer)

- `ctrl+a`, `delete/backspace`, `ctrl+d`, `ctrl+z`, `ctrl+shift+z/ctrl+y`, `escape`, arrow nudge (shift = 10px)
- `playgroundKeybindings={playground.keybindings}` wired in `NotePlayInner`

### React canvas (`note/react/index.tsx`)

- Double-click empty canvas → create text block + inline editor
- Double-click text block → rich text overlay (B/I/U/Link toolbar, contenteditable)
- Double-click table cell → inline cell editor (Enter/Tab advance)
- Image blocks render from `doc.assets` via `<img>`
- 8-handle resize chrome for selection (single + multi-select group scale)
- Multi-block drag move when selection includes pointer-downed block
- Stroke eraser: drag deletes whole ink strokes under cursor
- Point eraser: drag removes points within `eraserRadius`, splits strokes
- Copy/paste: `cmd+c`/`cmd+v` block clipboard JSON; OS image/SVG paste creates image blocks; plain text paste creates text block

### Fixtures

- `note/fixture/semio.note.json` updated to `paragraphs` schema
- `note/manifest/blocks.manifest.json` text property → `paragraphs`

## Fixes applied for E2E

1. `runViteBunxDev` now passes playground env (`PUZZLE_PLAY_ENTRY`) and validates port reuse by play entry
2. `note` added to `PACKAGE_ROOT_BY_ENTRY` in playground dev script
3. `@semio-tech/note-core` added to framework-playground-core deps
4. Vite aliases for `@semio-tech/note-core` and `@semio-tech/note-react` in `vite-elements-assets.ts`
5. Renderer: duplicate import cleanup, procedural/trinity import fixes, `UiRenderer` `case "note"`
6. `trinity/react`: removed browser-incompatible `node:fs` vitest wasm init from module top level
7. `s/core/playground.ts`: note fixture glob for S play

## Runtime (controller)

```
[DEBUG] note block added text text-1
[DEBUG] note block added table table-2
[DEBUG] note block added math math-3
[DEBUG] note block added image image-4
[DEBUG] note block added ink ink-5
[DEBUG] note selection [ "text-1" ]
```
