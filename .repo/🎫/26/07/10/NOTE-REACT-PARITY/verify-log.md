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

## Browser (`browser-verify.ts` + manual dev server)
Started `bun run dev:note` (NOTE_PLAY_PORT=6080, SEMIO_RENDERER=react) directly; it built the note wasm plugin (`framework/product/os/dev/plugin-modules/note/note_plugin.js`/`.wasm`, confirmed fresh via file mtimes) and Vite served the shell at `http://127.0.0.1:6080/`. `note_plugin.js`/`.wasm` loaded over the network with 200 OK.

**Known limitation hit during this session, not a defect in this change:** several other concurrent sessions in this same checkout were running their own scoped dev servers (`dev:raster`, `dev:puzzle`, …) at the same time. `framework/plugin/registry/generated/plugins.ts` is a single shared generated file, and each `bun run dev:<tech>` regenerates it scoped to only its own plugin. The two dev processes kept clobbering each other's registry file, so the OS shell intermittently booted with `Error: No plugins loaded` whenever a rival session's regeneration landed between my page load and plugin-registry read — this reproduced with the registry file containing only `raster` or only `puzzle`, never a note-specific bug. This is a pre-existing multi-writer race in the shared dev tooling (unrelated to any code in this ticket) that only surfaces when multiple scoped dev servers run concurrently against the same working tree; a single-session `dev:note` run is unaffected.

Given that race, the full OS-shell live-browser pass (`browser-verify.ts`, and the click/drag/pencil/table manual checklist) could not be captured as a clean screenshot in this session — it depends on `plugins.ts` staying note-scoped for the lifetime of the page load, which a concurrent session's dev server was repeatedly invalidating. `wasm-verify.ts` covers the equivalent behavior against the same compiled artifact the browser loads (real `note_plugin.js`/`.wasm`, real command/render/undo round trips) and passed in full — see above. `browser-verify.ts` is left in place, ready to run standalone (`bun .repo/🎫/26/07/10/NOTE-REACT-PARITY/browser-verify.ts`) once only one scoped dev server is active in the checkout.
