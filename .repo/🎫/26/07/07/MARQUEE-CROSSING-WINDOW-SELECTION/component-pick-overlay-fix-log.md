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

## Follow-up (face hover + rectangle preview regression)

### Symptoms
- Face hover under cursor not showing
- Rectangle / group selection preview not showing for faces

### Root causes
1. Face hover pick used per-triangle `ray_triangle` loop instead of `ray_pick_mesh_detail` (same path as mesh paint hit) — less reliable under scene camera.
2. `sync_world3d_state` cleared `hovered_component_*` whenever `hoveredComponent` was absent from selection JSON, wiping renderer-local hover before plugin echo.
3. Face overlays gated only on `granularity == "face"`, not `selection_targets.face`.
4. `screen_select_components` skipped face marquee when `face_ids` was empty.
5. `setSelection` marquee commit did not update local `component_ids` via `apply_world_command_preview`.

### Fixes
- Face pick → `ray_pick_mesh_detail` + `mesh_face_id`
- `apply_hovered_component_from_selection`: only touch hover when JSON key present/null
- Face line + translucent overlays → `face_component_mode_active`
- Kernel face marquee branch without `face_ids.is_empty()` guard
- `apply_world_command_preview` handles `setSelection`
- Tests: face marquee preview, hover preservation, overlay lines

- `cargo test -p infinite_world` — 23/23 pass
- `cargo test -p kernel_3d_scene` — pass
- `cargo build --target=wasm32-unknown-unknown --manifest-path framework/renderer/wgpu/rs/Cargo.toml` — pass
- `bun framework/renderer/wgpu/script.ts wasm` — trunk CLI env error (`--no-color`); unrelated to code changes
