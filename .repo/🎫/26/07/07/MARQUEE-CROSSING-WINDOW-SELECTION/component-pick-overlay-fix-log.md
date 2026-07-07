# Component pick / overlay fix log

## Issues
- Edges: hover/select highlight on wrong edge (not under cursor)
- Faces: no hover/select/highlight visible
- Vertices: already correct

## Root causes
1. **Pick viewport mismatch** — `pointer_in_pick_rect` used `pick_bounds` (clip) for projection local coords while rendering uses `bounds`; when clip offset differs from render viewport, screen picks drift (edges worse than vertices).
2. **Edge depth ambiguity** — screen-distance-only pick chose wrong edge when multiple edges overlap in screen space; fixed with depth + ray-distance tie-break among screen hits.
3. **Face overlays invisible** — translucent pass uses back-face culling; single-sided overlay triangles often culled. Added reversed winding duplicate + triangle edge line overlays.

## Changes
- `pointer_in_pick_rect`: clip test on `pick_bounds`, projection on `bounds`
- Edge pick: screen segment distance + closest-depth / ray-distance tie-break
- Face pick: `ray_triangle` (published in kernel)
- Face display: double-sided translucent triangles + highlighted triangle edge lines
- Marquee preview/commit: use render viewport for projection
- Tests: viewport offset, face overlay lines, ray face/edge pick

## Verification
- `cargo test -p infinite_world` — 23/23 pass
- `cargo test -p kernel_3d_scene` — pass
- `cargo build --target=wasm32-unknown-unknown --manifest-path framework/renderer/wgpu/rs/Cargo.toml` — pass
- `bun framework/renderer/wgpu/script.ts wasm` — trunk CLI env error (`--no-color`); unrelated to code changes
