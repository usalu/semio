# Puzzle 3D React Parity — Verify Log

## Root cause (brush/fill)

- **Brush candidates:** `brush_candidates()` returns `{ free, unknownPending }` but d3 parsed the payload as a bare array — always empty. Fixed via `parse_brush_candidates_free()`.
- **Brush preview:** `world_brush_preview_json` now uses `puzzle3d_brush_target_vortex` (selection + hovered-object fallback).
- **Precompute:** `drive_precompute` increased; runs on example switch, brush/fill tool select, vortex hover, and `cycleBrushCandidate`.
- **Runtime round trip:** `parse_envelope` now deserializes `Puzzle3dEnvelope` first so `runtime` (active tool, fill count, selection) is not dropped.
- **Fill:** `setFillCount` accepts `{ value }` / `{ count }`; fill tool activates when count > 0.

## React / shell

- **Example dedupe:** `exampleOptions` filters strictly on `example.appId === session.app.id` plus id de-duplication (manifest shows one Empty + one Concrete Forest for `puzzle3d-play`).
- **Camera:** `parseCameraState` respects fixture `zoom`; autofit fallback when camera JSON has no `position`.
- **Brush meshes:** `BrushMeshRegistrar` in `world-3d-host.tsx` dispatches `registerBrushMesh` when GLBs load (needed for collision-free brush candidates in the live app).

## Tests run

- `bun nx run @semio-tech/framework-renderer-react:test` — 24 passed
- `cargo build -p puzzle-plugin --target wasm32-wasip2 --release` — ok
- `bun ./.repo/🎫/26/07/09/PUZZLE-3D-REACT-PARITY/wasm-verify.ts` — fill slider round trip (select fill → value 4); manifest example scoping ok
- `cargo test -p puzzle_3d brush` — ok
- `cargo test -p puzzle-plugin` — blocked on native host by pre-existing `plugin_exports!` wasm-only macro in `puzzle/plugin/rs/lib.rs`

## Live verification notes

- Puzzle 3D dev server: `http://127.0.0.1:6013/` (isolated from other `dev:puzzle:*` builds to avoid shared wasm-pack races).
- Wasm API verification confirms fill command round trip; brush placement candidates require GLB collision meshes (`registerBrushMesh`) after assets load — headless wasm script without mesh fetch may still show zero brush candidates even when the parse/precompute path is correct.
- Browser automation: fill tool button click works; engagement chrome may need the window focused/expanded before the slider appears in the overlay (wasm path does not depend on React engagement chrome).

## Ticket scripts (temporary)

- `wasm-verify.ts` — automated wasm fill + manifest dedupe check
- `browser-verify.ts` — playwright smoke (run after dev server + wasm rebuild)
- `manifest-check.ts`, `brush-debug.ts` — investigation helpers
