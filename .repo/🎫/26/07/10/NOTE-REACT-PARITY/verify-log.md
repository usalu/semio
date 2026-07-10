# Note React Parity — Verify Log

## Rust
- `cargo test -p note-plugin` — 18/18 passed (model round-trips, gesture begin/live/commit undo-boundary semantics, camera/tool commands never push undo, table row/column patch ops with clamping, duplicate-selection offset+re-id, save/load/setFixtureJson ops, setActiveExample, envelope round-trip of new fields, group-child re-id on clone).
- `cargo test -p semio-framework-core -p semio-framework-plugin -p note-plugin` — 18 + 7 + 16 passed. (Doctest step intermittently fails to link `semio_framework_plugin` for the `cdylib` crate type when run as part of a multi-package invocation — confirmed pre-existing/unrelated to this change: reproduces identically in isolation for `puzzle-plugin`, and `-p note-plugin --doc` alone passes with 0 doctests.)

## React / vitest
- `bun ./script.ts test --run` in `framework/renderer/react` — full suite passing (61-62 tests depending on concurrent unrelated raster-host work landing in the same file; the "note canvas host" describe block's 8 tests pass 100% of the time, isolated via `-t "note canvas host"`).
- Covers: semio example renders welcome text + `<table>` + math (KaTeX or fallback, both asserted), grid pattern present in composite / absent in navigator, resize-bounds min-size clamp, group ink-point scaling, ink-stroke point-erase fragment splitting, bold/link paragraph↔HTML round-trip, clipboard payload round-trip, ink block bounds from local points, wheel/pan camera formula symmetry.

## Wasm round-trip (`wasm-verify.ts`)
Loads the built `note_plugin.js`/`.wasm` via `loadPluginModule`, drives the real plugin end-to-end:
- `setActiveExample("semio")` → 3 blocks.
- `applyNoteEvents` begin (addBlock ink) → live (updateBlock with 2 points) → commit (no new events) → 4 blocks.
- `undo` → back to 3 blocks in **one** step (confirms the whole create+draw gesture undoes atomically).
- `windowEngagements()["note-composite"]` has an input and a status line ("3 blocks · 1 selected · zoom 1.00 | grid 32px · snap off").
- `windowMeasures()["note-composite"]` has 4 groups (Camera, Grid, Snap, Drawing).
- `tools()` returns 16 toolbar nodes (buttons + toggles + separators).
- Manifest examples: Empty, Semio.

Ran successfully — see chat transcript for full `[DEBUG]` output, all assertions passed.

## Browser (`browser-verify.ts`)
Playwright script against `http://127.0.0.1:6080/` (note-react-dev, `.claude/launch.json`): loads the semio example, asserts welcome text renders, click-selects a block (ring appears), drags it (single gesture), switches to pencil and draws a stroke, double-clicks the table to open a cell editor, and asserts no React duplicate-key console errors.

## Manual
`bun run dev:note` (NOTE_PLAY_PORT=6080, SEMIO_RENDERER=react) built and served; manual pass covered all 10 tools, marquee, resize handles, both erasers, clipboard (blocks/image/SVG/text), navigator minimap, import/export round-trip, and keybindings including shift-nudge and mod+d.
