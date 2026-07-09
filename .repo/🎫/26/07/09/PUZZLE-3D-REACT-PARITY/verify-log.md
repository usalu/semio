# Puzzle 3D React Parity — Verify Log

## Changes (round 1)

- Part A: `GLB_MESH_FRAME_ROTATION_X` wrapper on `GlbInstanceMesh` (glTF Y-up → CAD Z-up)
- Part B: `setHover`, `worldPick`, `setTransformTool` handlers + enriched `world_selection_json` (gumball, selectionMode, targets)
- Part C: Extended `World3dScene` type + vortex/attraction/volume/reference/brush-preview rendering in `World3dHost`
- Part D: `window_engagements` with Select/Brush/Fill toolbar + brush candidate toggle + fill slider

## Changes (round 2 — brush/fill/context menu)

- Fill: `setFillCount` now accepts engagement slider `{ value }` as well as `{ count }` (matches puzzle 2d)
- Brush: `sync_precompute_session` runs at the top of every `handle_command_patch_ops` so brush candidates/preview are available on hover without an unrelated command first
- Context menu: `contextMenuJson` on `World3dScene` (core + plugin + React); puzzle 3d emits Duplicate / Select same kind / Zoom to selection / Delete when selection is non-empty; `duplicateSelection` and `selectSameKindSelection` commands added; `ContextMenuController` wired in `world-3d-host.tsx` with client-side zoom

## Tests run

- `bun nx run @semio-tech/framework-renderer-react:test` — 21 passed
- `cargo check -p semio-framework-core -p semio-framework-plugin` — ok
- `cargo test -p puzzle-plugin` — blocked on native host by pre-existing `plugin_exports!` wasm-only macro in `puzzle/plugin/rs/lib.rs`; d3 logic compiles after borrow fix

## Manual browser check (pending dev server)

Load Concrete Forest in Puzzle 3D and confirm:
- Brush mode: hover vortex → candidate toggle + ghost preview; click places object
- Fill mode: slider adds objects (count > 0)
- Right-click with selection → context menu (Duplicate, Select same kind, Zoom, Delete)
- GLB meshes upright, hover/select/marquee+gumball, vortex/cable overlays, Select/Brush/Fill toolbar
