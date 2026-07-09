# Puzzle 3D React Parity — Verify Log

## Changes

- Part A: `GLB_MESH_FRAME_ROTATION_X` wrapper on `GlbInstanceMesh` (glTF Y-up → CAD Z-up)
- Part B: `setHover`, `worldPick`, `setTransformTool` handlers + enriched `world_selection_json` (gumball, selectionMode, targets)
- Part C: Extended `World3dScene` type + vortex/attraction/volume/reference/brush-preview rendering in `World3dHost`
- Part D: `window_engagements` with Select/Brush/Fill toolbar + brush candidate toggle + fill slider

## Tests run

- `bun nx run @semio-tech/framework-renderer-react:test` — 21 passed
- `cargo test -p puzzle-plugin` — blocked on native host by pre-existing `plugin_exports!` wasm-only macro in `puzzle/plugin/rs/lib.rs` (same as `gis-plugin`); d3 module compiles after import fix

## Manual browser check (pending dev server)

Load Concrete Forest in Puzzle 3D and confirm:
- GLB meshes upright
- Hover highlight on objects
- Click/marquee selection + gumball
- Vortex markers and attraction cables visible
- Select/Brush/Fill toolbar in window engagement rail
