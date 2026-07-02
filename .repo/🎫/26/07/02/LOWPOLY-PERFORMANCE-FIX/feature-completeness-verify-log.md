# Lowpoly Feature Completeness — Verification Log

## Automated

- `bun nx run lowpoly-core:wasm` — OK
- `bun nx run lowpoly-core:test` — Rust 10/10 passed (`default_fixture_mesh_has_unwrapped_uvs`, `empty_paint_pixels_are_opaque_white`, …)
- `cargo test` in `kernel/3d/mesh` — 12/12 passed (tessellate edge_uvs/edge_is_seam assertions)
- `bun run dev:lowpoly` — dev server at http://127.0.0.1:6078/

## Vitest (pre-existing import failure)

- `lowpoly-core:test`, `lowpoly-react:test`, `lowpoly-play:test` fail at import with `createDefaultLayout is not a function` from `vcs/core/playground.ts` (unrelated to lowpoly changes)

## Manual checklist (browser)

- [ ] Paint mode: default mesh shows opaque white paint surface in 3D (not black/invisible)
- [ ] Paint stroke in 3D updates UV window live and vice versa (shared session + paintTextureRevision)
- [ ] UV window: checker/grid backdrop, 0–1 unit border, topological edge wireframe, seam edges dashed/highlighted
- [ ] Hover vs selection colors use `--hover-base` / `--active-base` (match marquee primary)
- [ ] Marquee drag previews pending selection before mouse-up
