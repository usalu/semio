# 🔀️ W9b — the projection pane's body, the Top pane's framing, the grid's reach, the mesh paint table, and the live camera

Packet W9b of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Closes `📓️w8b-orthographic-camera-and-3d-parity.md`
§7.2 (P1), §7.1, §7.3, §7.4 and §7.5, and with them `📓️w7b-presenter-one-frame-per-boot.md` §9.5/§9.6
and `📓️w6a-navbar-footer-polish.md`'s open "confirm LIVE" item 4.

Read first: `📓️w8b` §7 (all five), `📓️w7b` §9, `📓️w6a` §1, `📓️w4b`. Line numbers are post-edit.
Logs under `🗑️generated/w9b-*.txt`; probe captures under `🗑️generated/w9b-*/`.

---

## 0 — What changed, in one table

| # | site | change |
| --- | --- | --- |
| 1 | `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9545` | `pointer_press_belongs_to_shell_chrome` also claims the `shell.` id NAMESPACE, so a pane chip painted over an engine surface reaches `handle_shell_hit` at all |
| 1 | `…:13684` | `paint_window_projection_step` measures its column ONCE per body and CLAMPS a short pane to its own inset instead of abandoning the body on row 0 |
| 1 | `…:17737` | the window walk's phase 11 resets the column memo per pane |
| 2 | `🎬️scene/📐️math/🦀️.rs:453`, `:499`, `:519` | new `WorldCardinalView` / `WorldProjectionOrientation`, `world_projection_view_half_extent` (React's verbatim) and `frame_projection_orbit_to_bounds` |
| 2 | `♾️infinite/🌍️world/🦀️.rs:10355` | the projection content frame reads the CARDINAL half-extent, not the projected box one |
| 2 | `…:10267`, `:10629` | the wire camera's `projection.orientation` and `mode.variant` are parsed and carried on `World3dState` |
| 2 | `…:9586` | new `apply_world3d_projection_spec` — a template press re-arms the framing, React's `seedPendingWorldProjectionCamera` |
| 2 | `🐚️Shell/…:13356` | `WorldProjectionTemplate` gains React's `orientation` / `oblique_off_axis` columns; the hit arm applies the whole spec |
| 2 | `♾️infinite/🌍️world/🦀️.rs:9892`, `:10629` | a locally selected spec is held against the wire echo — the family in the snapshot apply, the orientation in the bridge parse |
| 3 | `♾️infinite/🌍️world/🦀️.rs:6964` | measured, reverted and DOCUMENTED: the division ceiling is invisible because the fade is clamped with it; the shader-plane port is scoped, not started |
| 4 | `♾️infinite/🌍️world/🦀️.rs:6999`, `:7048` | React's whole `MESH_STYLE_PAINT` as one table + `resolve_mesh_style`'s priority ladder; `provisional`'s second rule deleted |
| 4 | `🎨️styling/🌗️mixing/🦀️.rs:49` | Rust twin of `oklabMix`, for React's `color-mix(in oklab, …)` disabled fill |
| 5 | `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2678` | new `WORLD_CAMERA_LEDGER` + `note_world3d_live_camera`; `dumpMeshStats` gains `liveCamera` |
| 5 | `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1923` | the one point per frame that holds both surface id and live orbit records it |
| 5 | `♾️infinite/🌍️world/🦀️.rs:9617` | new `world3d_live_camera_json` |

---

## 1 — P1: the unfolded `Projection` pane painted no body live

### Root cause — the press never reached the shell

Not the chrome walk's budget, and not the pane's geometry. `📓️w8b` §7.2 suspected
`world_projection_column_width`'s 15 measured labels against the per-frame budget; the measurement
says otherwise.

`🧊️renderer/🦀️.rs:14699` gates a PRESS that lands inside an engine surface's rect:

```rust
if ShellState::pointer_press_belongs_to_shell_chrome(self.input.hit_at(x, y)) {
    … self.shell.handle_pointer_button(…).await …
    return;
}
// otherwise: enqueue_world3d_event(state, …pointer_button…) and return
```

and the predicate it calls was KINDS only:

```rust
hit.is_some_and(|hit| matches!(hit.kind, HitKind::NavbarItem | HitKind::DropdownItem | HitKind::ContextMenu | HitKind::Select)
    || hit.control_id.as_deref().is_some_and(|id| id.starts_with("ui.introduction.")))
```

A pane chip is `HitKind::Toggle` (`paint_window_pane_chips_step`'s `ChromeGroupItem { … kind:
HitKind::Toggle }`), and every world pane paints its chips INSIDE the world surface's own rect. So
the press failed the predicate, the world3d lane consumed it, and `handle_shell_hit` never ran. The
fold never flipped, `projection_pane_folded` stayed `true`, and `paint_window_projection_step`
returned on its first line — which is exactly "runs but paints nothing".

**Live evidence, reproduced before the fix** (`🗑️generated/w9b-projection-live/`, 1440×900 dpr 1,
tour suppressed, against the serve already running):

- the chip resolves: `os_host pointer hit x=1395.48 y=853.6 targets=43 staged=43 hit=Some((Toggle,
  Some("shell.projection.fold.puzzle3d-main-perspective")))`
- `grep -c "wgpu-shell pointer button" console.txt` → **1**, and that one is `down=false`. The
  `PointerDown` never reached `handle_pointer_button` at all; the release the shell ignores did.
- the hit ledger stays at **43 rows** through ten 1.5 s retries, with `error: null` — no projection
  template row ever appears (`chrome.json`'s `samples`).

The same gap made `Actions`, `Search`, `Window Options` and `Utilities` unpressable on any world
pane — a whole control family, not one chip. `📓️w6a` §1 fixed the chip's hit REGISTRATION (the
audit's P0) and its "confirm LIVE" item 4 was never run, so the second half stayed open.

### Fix

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9545` — the predicate also claims the two id NAMESPACES the shell
alone mints:

```rust
hit.is_some_and(|hit| matches!(hit.kind, HitKind::NavbarItem | HitKind::DropdownItem | HitKind::ContextMenu | HitKind::Select)
    || hit.control_id.as_deref().is_some_and(|id| id.starts_with("shell.") || id.starts_with("ui.introduction.")))
```

This is still not a control-id LIST — the docstring's own rule. `shell.` is a prefix no engine
surface can produce: every scene keys its targets by its own `surface_id`
(`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`'s `scroll_key(&scene.surface_id, …)`, `format!("{}.vfs.chevron.{}",
scene.surface_id, …)`, …), and a grep for a `shell.`-prefixed `control_id` under `🎞️Scenes` and
`⚙️EngineCanvas` returns nothing. Inverting the predicate instead ("anything that is not the
surface's own `World3d`/`ScrollRegion` region") was rejected: scenes register per-element targets as
`HitKind::Generic` and `HitKind::Input`, so an inversion would have stolen every board, graph and
table row press.

React has no such gap to close: a `Pane` chip is a DOM button in a layer ABOVE the `<canvas>`, so
the canvas never sees the press.

### Fix — the body's own two defects

`paint_window_projection_step` (`…:13684`) had two more, both fatal for a short pane and wasteful
for a tall one:

1. **`world_projection_column_width` was re-derived per ROW.** It walks all 15 labels, so one
   unfolded body spent 225 `FontAtlas::measure_text` calls to answer the same number 15 times — the
   cost `📓️w6a`'s own "Boot cost" note warned about. It is now measured once per body and carried in
   `cursor.x` (reset by the walk's phase 9→10 and phase 11 exits).
2. **A pane too short for 15 rows abandoned the WHOLE body.** The old guard was
   `if y < window_rect.y + theme.panel_inset { return true }` evaluated per row — and `cursor.scalar
   == 0` is the TOPMOST row, so the very first opportunity returned "complete" and nothing painted.
   The body now derives HOW MANY rows fit up front —
   `rows = floor((chip_top - window_rect.y - panel_inset) / row_h).min(15)` — and lays row `n` at
   `chip_top - (rows - n) · row_h`, so a short pane paints the rows that fit and a tall one is
   unchanged. React's `Pane` gives its tree `overflow-auto` and still paints. (The first attempt
   compared floats — `y + row_h > chip_top` — and dropped the LAST row to a one-ulp difference
   between two ways of computing 574.4; the row count has no such edge.)

At 1440×900 the puzzle3d panes are 478×814 and 956×814, and 15 × 22.4 px of rows clears the inset by
449 px, so the geometry was never the live cause — but it is the cause on any pane under ~350 px.

### Selection → camera

`handle_shell_hit`'s `shell.projection.template.<windowId>::<id>` arm now applies the whole spec
through the new `apply_world3d_projection_spec` (§2), so a row press moves the pane's camera FAMILY,
its framing ORIENTATION, and re-arms the content frame. It still dispatches nothing of its own,
which is React's rule verbatim: `handleProjectionKindChange` (`🌐️World3dHost/🟦️.tsx:6580`) only sets
`externalPendingProjectionSpec`, because the single `setProjection` consumer takes granular
`{field, value}` pairs and has no lossless mapping from a whole spec. The pose the remount settles on
rides the ordinary `setCamera` — here the zero-delta wheel settle the arm already queues.

### Live proof, after the fix

`🗑️generated/w9b-final/` (1440×900 dpr 1, tour suppressed, `🐍️w9b-projection-and-framing-probe.mjs`):

- the press lands and the body publishes: the hit ledger goes **43 → 58**, exactly the 15 template
  rows, `error: null`;
- `projection.json`'s `rows` are the whole taxonomy in React's declared order at 22.4 px pitch, each
  level indented 10 px off ONE column trailing edge at `x = 1333.3`, the last row one row above the
  chip: `parallel [1333.3, 506.4]`, `orthographic [1343.3, 528.8]`, … `curvilinear [1343.3, 820.0]`,
  chip `[1357.4, 842.4]`;
- `🗑️generated/w9b-final/final.png` shows it painted — `Parallel · Orthographic · Axonometric
  (Isometric/Dimetric/Trimetric) · Oblique (Cabinet/Cavalier/Military) · Perspective
  (1-Point/2-Point/3-Point/Curvilinear)`, `Orthographic` reading ACTIVE, and the folded chip's own
  icon switched to the orthographic glyph — React's `WorldProjectionKindSwitch` row for row.

The press also reaches the camera, which needed two more fixes (§2's *Holding the selection*):
`liveCamera` on `puzzle3d-main-perspective` goes
`{perspective, orientation: Free, zoom: 1, target: [3.5, 0, 0.005]}` →
`{orthographic, orientation: Cardinal(Top), zoom: 12.053, target: [7, 0, 0.01]}`.

### Laws

| test | what it pins |
| --- | --- |
| `a_pane_chip_press_over_an_engine_surface_belongs_to_the_shell` | every id `window_pane_chips` mounts is claimed by the press gate, and so is a template row; the surface's own `World3d`/`ScrollRegion` region and a surface-minted `Generic` target are NOT |
| `an_unfolded_projection_pane_publishes_its_rows_within_one_frame_budget` | the body terminates inside `WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES`, publishes all 15 rows top-down, every row hangs off ONE measured column trailing edge indented per level, and a 6-row-tall pane still paints the rows that fit instead of nothing |
| `the_projection_chip_folds_its_own_pane_and_switches_its_template` (W6a's, still green) | one id both sides of the fold, per-pane fold, React's order, the icon follows the selection, the camera family moves |

---

## 2 — P2: the `Top` pane's framing

### Root cause

React frames a projection pane on the box's CARDINAL half-extent; wgpu framed it on the box's
projected screen half-extent.

- React: `frameWorldProjectionPose` (`🎨️r3f/🟦️.tsx:2523`) → `worldProjectionViewHalfExtent(spec,
  bounds)` (`:2492`) → `(hx, hy)` for top/bottom, `(hx, hz)` for front/back, `(hy, hz)` for
  left/right, `(hx, hz)` for a free oblique that is not `military`, and the ISOTROPIC span
  `max(hx, hy, hz)` on both axes for everything else → `worldProjectionOrthoZoom(…, 1.35)` (`:2514`).
- wgpu: `sync_world3d_projection_content_frame` → `frame_orbit_to_bounds` →
  `screen_half_extent(&camera, minimum, maximum)` (`📐️math/🦀️.rs`), the eight box corners projected
  onto the LIVE camera's screen basis.

The two agree for an exactly cardinal camera — which is why the boot `Top` pane's numbers did not
move — and diverge for every other orientation. That set is no longer hypothetical: §1 makes
`Isometric`, `Cabinet`, `Cavalier`, `1-Point`… selectable, and each one is a spec React frames
isotropically and this renderer would have framed on the true projected box.

The wire also dropped the orientation entirely: `World3dSceneProjectionRecord` carried only
`mode.kind`, with the docstring "the orientation is already baked into the delivered `position`/`up`".
True for the POSE, false for the FRAMING.

### Fix

- `📐️math/🦀️.rs:452` — `WorldCardinalView` (with React's `plan` → `top` alias),
  `WorldProjectionOrientation`, `world_projection_view_half_extent` (React's five branches verbatim)
  and `frame_projection_orbit_to_bounds` (React's `frameWorldProjectionPose`, parallel half).
  `frame_orbit_to_bounds` and `screen_half_extent` are untouched — they serve the producer's fit
  lane (`WorldAutoFit`), which carries no spec.
- `🌍️world/🦀️.rs:10267` — `projection.orientation` (`{type, view}`) and `mode.variant` parsed;
  `:10629` `World3dState::{projection_orientation, projection_oblique_off_axis}` set when the camera
  record parses; `:10355` the content frame calls the new framing.
- `🌍️world/🦀️.rs:9586` — `apply_world3d_projection_spec` sets both, then RELEASES
  `camera_user_moved` / `projection_frame_key` / `projection_frame_zoom`. Without that release a
  press was self-defeating: `apply_world3d_projection` arms `camera_user_moved`, which is the very
  gate `sync_world3d_projection_content_frame` refuses on, so a pane switched to `Orthographic` took
  the parallel frustum and kept the perspective pane's zoom forever. React re-frames on a spec change
  through `seedPendingWorldProjectionCamera` (`🌐️World3dHost/🟦️.tsx:1139`).
- `🐚️Shell/…:13356` — every `WORLD_PROJECTION_TEMPLATES` row carries React's
  `worldProjectionDefaults(kind).orientation`: only `Orthographic` is cardinal (`Top`); the `Oblique`
  subtree minus `Military` is `oblique_off_axis`; everything else is free.

### Laws

`a_projection_frame_reads_reacts_cardinal_half_extent` (`🔬️math-unit`) — the six cardinal branches,
the free-oblique branch, the isotropic branch, and **React's own chunkkey fixture ported to Rust**:
a 50-wide reference at `[7, 0, 0.01]` → `halfExtent [25, 25, 0.5]` → a 400×800 top pane frames at
`zoom = 200 / (25 · 1.35)`, to the digit. There was no Rust mirror of that fixture before, which is
why the divergence went unpinned. It closes with the corner case: for a thin box at yaw 0.7 / pitch
0.6, React's isotropic span frames strictly WIDER than the projected extent.

`a_projection_selection_rearms_the_panes_framing` (`🔬️unit`) — the latch release, idempotence on the
same row, and that `Cabinet` after `Orthographic` still counts as a change (same family, other plane).

### Holding the selection against the wire

Two more fixes were needed before a press survived one round trip, both found LIVE (`liveCamera` is
what made them visible at all):

1. **`camera_user_moved` armed itself against its own framing.** The arm queues a zero-delta wheel
   settle to publish the new pose, and that settle runs through `WorldFlatActionKind::CameraPlan`,
   which sets `camera_user_moved = true` — the very gate the content frame refuses on. So the family
   changed and the framing never ran. `World3dState::projection_frame_owed` is now the exemption: a
   selection owes exactly ONE framing, immune to the latch, cleared when it lands
   (`🌍️world/🦀️.rs:10380`). A press is not a gesture the latch exists to protect.
2. **The guest's echo took the selection back.** `setCamera`'s payload has no `projection` member at
   all (`orbit_camera_action`), so the camera the guest echoes always arrives in the DELIVERED family
   — and `step_world3d_snapshot` assigned `state.orbit = orbit` whole. Measured: the press moved the
   pane's target onto the content centre and `liveCamera` still read `perspective` 8 s later
   (`🗑️generated/w9b-verify/projection.json`). `World3dState::projection_selected` now holds the
   local family through the snapshot apply (`:9892`) and the local orientation through the bridge
   parse (`:10629`) — React's own `externalPendingProjectionSpec` precedence.

### Live proof

`🗑️generated/w9b-final/projection.json`, `liveCamera` on `puzzle3d-main-perspective`:

| | projection | orientation | zoom | target |
| --- | --- | --- | --- | --- |
| before the press | `perspective` | `Free` | 1 | `[3.5, 0, 0.005]` |
| after `Orthographic` | `orthographic` | `Cardinal(Top)` | **12.053** | `[7, 0, 0.01]` |

And the number is React's, to five digits: the pane is 955.7 × 813.6 and the content half-extent is
`[25, 25, 0.5]`, so React's `worldProjectionOrthoZoom(25, 25, 955.7, 813.6, 1.35)` is
`min(477.9, 406.8) / 33.75 = 12.0533`. The live orbit reads `12.053333`.

### Honest remainder — and what the ≈285 px vs ≈350 px actually was

`📓️w8b` §7.1 read React's `Top` sheet at ≈285 px against wgpu's ≈350 px. With `liveCamera` the wgpu
half of that is now exact rather than eyeballed, and it is RIGHT:

- `liveCamera` on `puzzle3d-main-top` at boot reads `zoom: 7.079506`, `target: [7, 0, 0.01]`,
  `orientation: Cardinal(Top)` (parsed straight off the wire spec's
  `{"type":"cardinal","view":"top"}`), while the WIRE camera on the same surface reads `zoom: 1` —
  the two halves §5 exists to separate.
- The pane is 477.9 × 813.6, so React's own formula gives
  `worldProjectionOrthoZoom(25, 25, 477.9, 813.6, 1.35) = 238.95 / 33.75 = 7.0800`. The live orbit
  reads `7.079506`, a 0.007 % difference — i.e. the framing already IS React's, on React's own
  content bounds (`halfExtent [25, 25, 0.5]` from the 50-wide reference at `[7, 0, 0.01]`, which is
  the very fixture React's `🧪️chunkkey/🟦️.tsx:691` asserts).
- The sheet therefore spans `50 × 7.0795 = 354 px` of the 478 px pane, which is `478 / 1.35` — an
  X-bound fit with exactly React's padding.

The ≈285 px was measured off `🗑️generated/react-6313/final.png`, which `📓️w6a` itself flags as
TOUR-BLURRED and "usable for band positions, not for glyphs". A gaussian veil moves a soft edge by
tens of pixels; re-measuring the same capture here gave ≈320 px, and a clean React capture could not
be taken (the 6313 serve boots blank inside a 60 s probe window, `🗑️generated/w9b-react-clean/`).
**So this packet does not claim a pixel-for-pixel Top match — it claims the framing MATH is React's,
proved against React's own fixture numbers, and that the earlier delta is unreproducible from a
blurred capture.** A clean 6313 capture is the one thing still owed on this item.

## 3 — P2: the grid

### What was measured

`WORLD_GRID_MAX_DIVISIONS: i32 = 512` is spent on the RADIUS: `divisions = min(fade · 2 / step, 512)`,
`half = step · divisions · 0.5`. `📓️w8b` §3.2/§7.3 recorded the consequence as a plan pane's grid
clamping "where React's infinite plane would keep going".

**Spending the ceiling on the STEP instead was implemented, measured live, and REVERTED.** Walking the
band up React's own `lodGridStepWorld` quantum ladder until the whole `camera_grid_fade_distance` fits
inside the budget looks right on paper and is wrong in fact: that distance is React's
`cameraGridVisibleRadius × WORLD_LOD_GRID_COVERAGE_MARGIN (32)` — a shader FADE parameter, not a draw
extent. Covering it with geometry walks the puzzle3d boot band from 10 world units to 10 000 and
paints four giant cells across a pane where React paints a 10-unit grid. Both builds were captured
(`🗑️generated/w9b-verify-3/boot.png` with the coarsening, `🗑️generated/w9b-final/boot.png` without)
and are pixel-identical at the boot cameras — the coarsening never triggers there — so it bought
nothing at the cameras that matter and cost the band's spacing at the ones that do.

The clamp is therefore kept, and the reason it is INVISIBLE is now pinned rather than assumed: the
fade curve is clamped with the radius (`lod_grid_fade_alpha(offset.abs(), half)`), so the outermost
line is alpha 0 and there is no edge to see.

### Law

`the_grid_never_ends_on_a_hard_edge_however_far_the_camera_is` — at camera distances 12, 400, 20 000
and 500 000: a grid exists, the vertex count stays inside `8 · (512 + 1)`, the rim reaches alpha 0,
and **the band keeps React's own `lodGridStepWorld` spacing at every one of them**, which is the half
the reverted coarsening would have broken. W7b's existing band-spacing law is unchanged and green.

### NOT done, and what it costs

**The technique is still line geometry, not a shader plane.** React's grid is a screen-space-derived
shader that never aliases; at a grazing angle this one will. The port was deliberately not started,
because it cannot be verified here — only two of the four backends are reachable on this machine, and
one of the other two has no world3d lane at all. The plan, with every site located:

1. new `WORLD3D_GRID_SHADER` + `WORLD3D_GRID_PIPELINE` in `🖱️ui/🖌️render/✨️shader-contract/🦀️.rs`
   (beside `WORLD3D_LINES_SHADER` `:680` and `WORLD3D_TEXTURED_SHADER` `:736`), a camera-following
   quad whose fragment shader derives the lines from world-space derivatives and fades by
   `WORLD_LOD_GRID_FADE_STRENGTH` — React's `cellSize = sectionSize = stepWorld`,
   `sectionThickness = 0`, both colours `--color-element`, `followCamera`, `infiniteGrid`
   (`🎨️r3f/🟦️.tsx:966-982`);
2. the byte-identical twin in `🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs`, and the twin law at
   `🖌️render/🧪️tests/🔬️shader-contract-unit/🦀️.rs:176` extended to cover it — note that law does NOT
   currently cover `WORLD3D_TEXTURED_SHADER`, so those two copies can already drift silently;
3. a `grid_draws` lane on `ScenePass3d` mirroring `textured_draws`, its pipeline at
   `🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:2689` and its replay beside `:2994`;
4. the four backends: webgpu `🧊️webgpu/🧵️pipelines/🦀️.rs:120` + `🎞️frame/🦀️.rs:252`; metal
   `🍎️metal/✨️msl/🦀️.rs:221` + `🏗️pipelines/🦀️.rs:370`; d3d12 `🪟️d3d12/✨️hlsl/🦀️.rs:221` +
   `🏗️pipelines/🦀️.rs:641`. **Vulkan has no world3d module at all** (`🌋️vulkan/🌋️backend/🦀️.rs:406`
   only validates the passes; `render_inner` is still milestone 1's clear-and-present), and neither
   metal nor d3d12 has grown the TEXTURED pipeline yet, so "all four backends" is a standing debt far
   larger than this one primitive.

## 4 — P2: the mesh style table

### Root cause

`world3d_style_paint` was an inline `if/else if` covering two of React's seven rows, with no table
behind it:

```rust
let fill = if instance.selected { theme.celebrate[0] } else if instance.hovered { theme.row_hover } else { return instance };
```

`provisional` was painted by a SECOND, unrelated rule (`tool_run_provisional_color`, which
substituted `theme.accent` for React's `--color-secondary` on the belief that this theme carries no
secondary tone — it does, as `theme.celebrate[1]`). `celebrated`, `highlighted` and `disabled` had no
wgpu paint at all, so a disabled instance painted fully opaque where React paints it at 0.45.

### Fix

`🌍️world/🦀️.rs:6999` — `MeshStyleKind`, `MeshStyleState`, `MeshStylePaint`, `mesh_style_paint` and
`resolve_mesh_style`: React's whole table with React's values, and React's own priority ladder
`disabled → provisional → celebrated → selected → highlighted → hovered → neutral`.

| row | fill | line | emissive | opacity |
| --- | --- | --- | --- | --- |
| `neutral` | `theme.panel` | `border_normal` | 0 | 1 |
| `hovered` | `row_hover` | `border_emphasized` | 0.08 | 1 |
| `selected` | `celebrate[0]` (primary) | primary | 0.35 | 1 |
| `highlighted` | `celebrate[1]` (secondary) | secondary | 0.2 | 1 |
| `provisional` | secondary | secondary | 0.2 | `PROVISIONAL_OPACITY` (1.0) |
| `celebrated` | primary, **+ conic `[primary, secondary, tertiary]`** | primary | 0.55 | 1 |
| `disabled` | `oklab_mix(text_muted, panel, 0.45)` | `text_muted` | 0 | **0.45** |

The `disabled` fill needed React's `color-mix(in oklab, …)`, which had a TypeScript implementation
(`🎨️styling/🌗️mixing/🟦️.ts`'s `oklabMix`) and no Rust twin. `🌗️mixing/🦀️.rs:25` now carries
`linear_to_oklab` / `oklab_to_linear` / `oklab_mix`, the same reference matrices without the byte
round-trip, because every Rust paint is already linear.

`render_world_3d`'s draw lane resolves one `MeshStyleState` per instance and the provisional partition
no longer recolours: the colour is already the `provisional` row, which outranks `selected`/`hovered`
exactly as React's ladder does.

### Law

`the_mesh_style_table_is_reacts_whole_paint_table` — all seven rows' fill/line/emissive/opacity, the
disabled mix being a REAL mix (neither of its ends), `celebrated` being the only row with a conic
triad, the whole priority ladder, and that a neutral instance keeps its producer's authored colour
while a styled one takes the row's opacity into its alpha.

### NOT done

`CelebratingConicMaterial`'s SPIN. The table carries the triad and
`CELEBRATE_CONIC_SPIN_SECONDS = 1.2`, but `WORLD3D_SHADER` has no conic term and no per-instance
"celebrated" flag, so the row paints React's own solid fallback (`tokenVar("primary")`, which React
itself uses for celebrated LINES). Animating it needs a flag bit in the instance `flags: vec4<f32>`
and a conic branch in the WGSL — and then in the MSL and HLSL mirrors, which the
`world3d_lighting_constants_are_identical_in_every_backend_mirror` law would have to grow to cover.
Nothing publishes `celebrating`, `highlighted` or `disabled` on the wgpu wire yet either
(`World3dSceneInstanceEntry` carries only `provisional`), so those three rows are reachable by the
table and not yet by the producer. That is one wire field, not a renderer change.

---

## 5 — `dumpMeshStats` reported the wire camera

### Root cause

`mesh_stats_for_scene` filled the surface's `camera` from `World3dScene.camera_json` — what the GUEST
published — and nothing else. Every local camera move is therefore invisible to a probe: an orbit, a
wheel, a `Projection` switch. `🐍️w8b-projection-switch-probe.mjs` reads exactly that field, which is
why §1's dead control and a working one were indistinguishable to it.

The plumbing gap is real: `dump_mesh_stats` reaches only `UI_ENGINE`, while the orbit lives on the
shell's `world3d_states`.

### Fix

The chrome ledger's own shape, which exists for this exact reason:

- `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2660` — `static WORLD_CAMERA_LEDGER: WorkerCell<BTreeMap<String,
  Value>>`, `note_world3d_live_camera(surface_id, camera)` gated on
  `semio_framework_trace::runtime_diagnostics_enabled()` exactly as `note_chrome_hit_registry` is, so
  a production frame pays nothing.
- `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1923` — recorded once per surface per painted frame, at the one
  point that holds both the surface id and `&World3dState`.
- `🌍️world/🦀️.rs:9617` — `world3d_live_camera_json`: `position`/`target`/`up`/`zoom` in the same
  `setCamera` shape `camera` carries so the two are directly diffable, plus the two members the wire
  shape has no room for and a local move turns on — the projection FAMILY and the framing
  ORIENTATION — plus `userMoved`.
- `DumpMeshSurface.live_camera` → `liveCamera`, beside `camera`.

### Law

`the_live_camera_row_reports_the_orbit_and_not_the_wire` — an untouched pane reports perspective and
`userMoved: false`; after `apply_world3d_projection_spec` the row reads `orthographic`,
`Cardinal(Top)` and a real parallel frustum scale; moving the orbit target moves the row.

---

## 6 — Gates

Every one RUN, in the foreground, on this tree. Logs under `🗑️generated/w9b-*.txt`.

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | ✅ `w9b-check-ui.txt` |
| `cargo check -p semio-framework-os-infinite --lib -j 4` | ✅ `w9b-check-infinite.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ✅ `w9b-check-renderer.txt` |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --target wasm32-unknown-unknown -j 4` | ✅ `w9b-check-ui-wasm.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | ✅ `w9b-check-renderer-wasm.txt` |
| `cargo test -p semio-framework-os-infinite --lib world:: -- --test-threads=1` | ✅ 156 passed, **6 failed — all pre-existing** (`w9b-tests-world.txt`; the packet declared 7, a peer fixed `prepared_world_resources_are_send_and_deduplicate_uploads` mid-run) |
| `cargo test -p semio-framework-ui-render --lib` (twin law) | ✅ 132 passed (`w9b-tests-render.txt`) |
| `cargo test -p semio-framework-ui-scene --lib math::` | ✅ 89 passed (`w9b-tests-scene-math.txt`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- navbar_footer_parity_tests window_pane_chrome_tests` | ✅ 21 passed (`w9b-tests-shell.txt`) |
| `framework-renderer-wgpu:wasm` + `activate-puzzle3d-wgpu-dev` | ✅ ×4 (`w9b-wasm*.txt`, `w9b-activate*.txt`) |
| probes | ✅ `w9b-projection-live/` (the P1 reproduction), `w9b-verify*/`, `w9b-final/` |

The six pre-existing world failures, unchanged by this packet:
`live_renderer_retains_generation_wake_…`, `world_authority_retains_front_plan_…`,
`world_component_marquee_cursor_…`, `world_component_marquee_publish_…`,
`world_object_registry_enforces_capacity_…`, `world_saturation_owner_blocks_…`.

New laws, all green: `a_pane_chip_press_over_an_engine_surface_belongs_to_the_shell`,
`an_unfolded_projection_pane_publishes_its_rows_within_one_frame_budget`,
`a_projection_frame_reads_reacts_cardinal_half_extent`,
`a_projection_selection_rearms_the_panes_framing`,
`the_mesh_style_table_is_reacts_whole_paint_table`,
`the_live_camera_row_reports_the_orbit_and_not_the_wire`,
`the_grid_never_ends_on_a_hard_edge_however_far_the_camera_is`.

---

## 7 — Remaining

**Owned by this lane, still open**

1. **P2 — the grid is line geometry, not a shader plane** (§3). Scoped site by site, not started;
   Vulkan has no world3d lane at all, so the "four backends" framing is a larger debt than the
   primitive.
2. **P2 — `celebrated`'s conic spin** (§4). The table carries React's triad and
   `CELEBRATE_CONIC_SPIN_SECONDS`; animating it needs a flag bit in `WORLD3D_SHADER`'s instance
   `flags: vec4<f32>` and the same in the MSL and HLSL mirrors.
3. **P2 — nothing publishes `celebrating`, `highlighted` or `disabled`** on the wgpu wire
   (`World3dSceneInstanceEntry` carries only `provisional`), so three of the seven rows are reachable
   by the table and not yet by a producer. One wire field, not a renderer change.
4. **P3 — a clean React `Top` capture.** The 6313 serve boots blank inside a 60 s probe window
   (`🗑️generated/w9b-react-clean/`), so §2's pixel comparison rests on React's own fixture numbers
   rather than on a capture. Worth one probe with a longer settle.

**Observed here, NOT this lane, and worth someone's attention**

5. **The reference underlay stopped painting on trunk.** Both world panes now render the grid and the
   concrete-forest mesh but NO plan sheet, where `🗑️generated/w8b-boot-2/final.png` shows it in both
   (`🗑️generated/w9b-final/boot.png`). It is not this packet's: the same frame appears with and
   without §3's reverted grid change, the surface still submits the draw (`textured=1`), and the
   asset still decodes (`asset ready kind=ReferenceImage … bytes=483496`, `reference image decode
   done`). The suspect is the uncommitted `take_reference_underlay_upload`
   (`🌍️world/🦀️.rs:9878`), which `std::mem::take`s the pixel buffer on first upload — a second
   `ensure_world_plane_texture` after that finds an empty buffer. Whoever owns the raster-ledger
   refactor should re-check it against a boot.
6. **W9c renamed the pane chip ids mid-packet** to React's own
   `framework.worldOrbit.projection.<camelCaseWindowId>.pane.fold` and extended this packet's press
   gate with `dock.`, `panel.resize.`, `WINDOW_PANE_CHIP_PARENT` and `WORLD_PROJECTION_PANE_PARENT`
   plus four more `HitKind`s. Both changes are live and green together; the probes accept either id
   spelling.
