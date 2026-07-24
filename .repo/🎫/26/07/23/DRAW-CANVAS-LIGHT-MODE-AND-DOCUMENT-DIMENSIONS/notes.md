# Draw Canvas Light Mode And Document Dimensions

## Cause
React `Canvas2dHost` hardcoded dark clear (`#111318`) and dark checkerboard, so light mode looked like a dark canvas unlike flow/infinite boards.

## Fix
- React: clear + LOD grid from `STYLING_BOARD_PALETTES` (`rasterClear` / `gridMinorStroke`) — same tokens as flow.
- wgpu: always draw theme-aware infinite LOD grid under canvas-2d layers.
- Draw plugin: emit artboard paper + `W × H` dimension label; blank docs default to 1024×1024 artboard.

## Verification
- `cargo test -p draw --lib artboard` → ok
- `cargo test -p draw-plugin --lib artboard|renders_canvas_scene_with_segments` → ok
- vitest `canvas-2d surface colors` → ok
