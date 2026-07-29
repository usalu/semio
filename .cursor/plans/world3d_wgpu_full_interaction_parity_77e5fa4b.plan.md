---
name: World3d WGPU Full Interaction Parity
overview: "Fix the actually-still-broken frustum plane extraction that empties every 3D viewport, then bring all world3d plugins (puzzle3d, puzzle5d, procedural3d, lowpoly, cad, shooting) to premigration interaction parity: orbit/pan/zoom with correct button mapping and persistence, hover, select, marquee, and a translate/rotate/scale gumball."
todos:
 - id: fix-frustum-planes
   content: Correct frustum_planes rows 2-5 to use w-row base; add concrete-forest-camera regression tests
   status: completed
 - id: fix-orbit-reset
   content: Only reset orbit in sync_world3d_state when camera_json itself changed
   status: completed
 - id: e2e-body-check
   content: Wire assertBodyContent into smokePlugin so empty 3D bodies fail e2e
   status: completed
 - id: input-left-drag
   content: Forward left-button drags and full modifiers to world3d handlers in lib.rs
   status: completed
 - id: camera-buttons
   content: "Premigration button mapping: middle/shift+right pan, alt+right/right orbit, wheel zoom"
   status: completed
 - id: camera-persist
   content: Emit setCamera on nav end; add/normalize setCamera handlers in all six 3d plugins
   status: completed
 - id: gumball-render
   content: Render translate axes/planes + rotate rings gizmo at selection centroid via line/translucent draws
   status: completed
 - id: gumball-interact
   content: Gizmo handle hit-testing, axis/plane drag projection, live preview, commit on release
   status: completed
 - id: plugin-transform-handlers
   content: Add translateSelection/rotateSelection/scaleSelection handlers to all six 3d plugins
   status: completed
 - id: verify-all
   content: cargo tests, WASM rebuild, body-checked e2e for all six plugins, manual browser check
   status: completed
isProject: false
---

# World3d WGPU Full Interaction Parity

## Root cause of the still-empty viewport (verified)

The previous `frustum_planes()` fix in [ui/wgpu/rs/scene3d.rs](ui/wgpu/rs/scene3d.rs) (lines 436-445) is still wrong. For column-major `cols[c][r]`, Gribb-Hartmann needs the w-row `m[c][3]` as base for every plane, but rows 2-5 currently use the x-row `m[c][0]`:

```439:444:ui/wgpu/rs/scene3d.rs
        [m[0][0] + m[0][3], m[1][0] + m[1][3], m[2][0] + m[2][3], m[3][0] + m[3][3]],
        [m[0][3] - m[0][0], m[1][3] - m[1][0], m[2][3] - m[2][0], m[3][3] - m[3][0]],
        [m[0][0] + m[0][1], m[1][0] + m[1][1], m[2][0] + m[2][1], m[3][0] + m[3][1]],
        [m[0][1] - m[0][0], m[1][1] - m[1][0], m[2][1] - m[2][0], m[3][1] - m[3][0]],
        [m[0][0] + m[0][2], m[1][0] + m[1][2], m[2][0] + m[2][2], m[3][0] + m[3][2]],
        [m[0][2] - m[0][0], m[1][2] - m[1][0], m[2][2] - m[2][0], m[3][2] - m[3][0]],
```

Rows 0-1 (left/right) are correct; rows 2-5 must be `m[c][3] ± m[c][1]` (bottom/top), `m[c][2]` (near, WGPU z in [0,1]), `m[c][3] - m[c][2]` (far). The existing tests pass by coincidence (symmetric box; behind-camera box culled by the correct left/right planes). The e2e also cannot catch this: `assertBodyContent` exists in the test file but is never called, so chrome pixels alone pass the check.

## Phase A - Fix rendering correctness

1. **`frustum_planes`** ([ui/wgpu/rs/scene3d.rs](ui/wgpu/rs/scene3d.rs)) - correct rows 2-5 as above. Add a regression test using the concrete-forest camera (position `[30,-30,20]`, target `[7,0,3]`, fov 45): a unit box at the target must be visible; boxes far above/below/behind must be culled. Also assert each of the 6 planes contains the camera look-at point.
2. **Orbit reset bug** ([framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs), `sync_world3d_state`) - the orbit controller is re-seeded from `camera_json` whenever any scene JSON changes (hover/selection), snapping the camera back mid-interaction. Diff `camera_json` separately and only reset `state.orbit` when it actually changed.
3. **E2E body check** ([.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts](.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts)) - call the existing `assertBodyContent(page, pluginId)` inside `smokePlugin` so an empty 3D body fails the test instead of passing on chrome pixels.

## Phase B - Camera navigation parity (premigration mapping)

Premigration bindings (from the archived PlayCanvas shell / OrbitGated): left drag = pick/marquee/gumball, middle drag = pan, shift+right = pan, alt+right = orbit, wheel = zoom. Current wgpu: right = orbit, shift+right/middle = pan, and left drags are never delivered.

1. **Input routing** ([framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) line ~239) - forward drag deltas for all buttons including left (needed for object drag + gumball), and pass full `PointerModifiers` (alt/meta, not just shift/ctrl) down to `handle_world3d_pointer_drag`/`_button`.
2. **Button mapping** ([framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs), `handle_world3d_pointer_drag`) - middle = pan, shift+right = pan, alt+right = orbit, plain right = orbit (kept, since wgpu has no 3D context menu), wheel = zoom (already).
3. **Camera persistence** - on nav gesture end (right/middle button up), emit `setCamera` with `{camera: {position, target, zoom/fov}}` from the orbit state so the camera survives re-renders and reloads. Add `setCamera` handlers to plugins missing them: [procedural/3d/plugin/rs/lib.rs](procedural/3d/plugin/rs/lib.rs), [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs), [puzzle/3d/plugin/rs/lib.rs](puzzle/3d/plugin/rs/lib.rs) (has it), [puzzle/5d/plugin/rs/lib.rs](puzzle/5d/plugin/rs/lib.rs) (`setCamera3d`); cad/shooting already handle it. Plugins that persist camera must send position+target (shooting/cad/puzzle5d currently send only x/y/z without target - normalize to full `{position, target, fov}` via `world3d_camera_json`).

## Phase C - Gumball (translate/rotate/scale gizmo)

Premigration used `UnifiedGumball`: translate axes + planes, rotate rings (scale for lowpoly), committing on drag end via `translateSelection` / `rotateSelection` / `scaleSelection` with `{mode, ids, dx/dy/dz | ax/ay/az/angle | sx/sy/sz}`.

1. **Rendering** ([framework/renderer/wgpu/rs/world3d.rs](framework/renderer/wgpu/rs/world3d.rs) + [ui/wgpu/rs/scene3d.rs](ui/wgpu/rs/scene3d.rs)) - when selection is non-empty, draw a gizmo at the selection centroid: 3 colored axis shafts (line draws + small cone meshes via the existing world pipeline), 3 translucent plane quads (translucent pipeline from the puzzle3d work), 3 rotation rings (line-strip circles). All reuse the `ScenePass3d` `line_draws`/`translucent_draws` added previously; no new GPU pipelines needed.
2. **Hit-testing + drag** - ray-test gizmo handles before instance picking on left-down (`pick_gumball_handle_at`): axis handles use ray-to-segment distance, plane quads use ray-plane, rings use ray-torus approximation. During left drag project the ray onto the constraint (axis line / plane) exactly like premigration `gumballProjectRayOntoAxis` / `gumballRayPlanePoint`; update a local preview transform on the dragged instances.
3. **Commit** - on left-up emit `translateSelection {mode:"mesh", ids, dx, dy, dz}` (or rotate/scale equivalents). Keep puzzle3d's existing free-drag `worldRelocate` path only when no gumball handle was hit (body drag), matching premigration relocate semantics.
4. **Plugin handlers** - add `translateSelection`/`rotateSelection`/`scaleSelection` to: [procedural/3d/plugin/rs/lib.rs](procedural/3d/plugin/rs/lib.rs), [puzzle/3d/plugin/rs/lib.rs](puzzle/3d/plugin/rs/lib.rs) (translate updates object origins + proximity connect, rotate updates orientation), [cad/plugin/rs/lib.rs](cad/plugin/rs/lib.rs), [shooting/plugin/rs/lib.rs](shooting/plugin/rs/lib.rs), [puzzle/5d/plugin/rs/lib.rs](puzzle/5d/plugin/rs/lib.rs). [lowpoly/plugin/rs/lib.rs](lowpoly/plugin/rs/lib.rs) already handles `translateSelection`; add rotate/scale.

## Phase D - Hover/select consistency check

Hover (`worldHover`), click select with shift=add / ctrl=toggle, and rect/lasso marquee already exist in the wgpu path and all six plugins handle `worldSelect`/`worldHover`. Verify each works after the frustum fix; fix ray-picking if the NDC change broke `ray_from_screen` (it now uses z=0/1 endpoints - validate against a known camera in a unit test).

## Deferred (explicit, not silently dropped)

- Face/vertex sub-object picking (`worldPick` granularity) and paint mode for lowpoly - large separate subsystem (per-face ids in the wgpu mesh path).
- Context menu on plain right-click (premigration reserved right for it; wgpu shell has no 3D context menu yet).
- Touch gestures.

## Verification

- `cargo test -p ui_wgpu` (new frustum + ray tests must fail before / pass after the plane fix).
- WASM rebuild `bun ./framework/renderer/wgpu/script.ts wasm`, then e2e with the now-active body check for puzzle3d, puzzle5d, lowpoly, procedural3d, cad, shooting (dev server on port 7202, `SKIP_DEV=1`).
- Manual browser verification (cursor-ide-browser): geometry visible, orbit/pan/zoom with premigration bindings, camera persists across hover, click/marquee select, gumball translate moves an object and puzzle3d proximity-connect draws an attraction line.
