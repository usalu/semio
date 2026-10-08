# Lane V2 — wgpu parity for the world3d modelling primitives (2026-10-08)

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, continues `📓️wsx-lane-v-viewport-2026-10-07.md` section 6. No ticket opened/closed/reopened, no git state modified.
Scope: the `🧊️wgpu` target (`🖍️draw` pipelines used by native and wasm). The parallel `🖌️render/🎯️targets/{webgpu,metal,d3d12,vulkan}` backends were not touched (their `World3dGlobals` is a separate 240-byte copy).

## 1. What now works on wgpu

| Primitive | wgpu behaviour |
|---|---|
| Annotations | `render_world_3d` projects the layer with the shared pure `project_world3d_annotations` and paints lines, arrowheads, arcs, markers and EN/DE labels as screen-constant overlay primitives (arrow 9 px, marker 10 px, stroke 1.5 px, label gap 14 px), tone colours from the live theme, clipped to the viewport by a scissor layer. |
| Legend | Applied scalar field: panel at the viewport's lower left, 32-slice ramp (range max on top), evenly spaced tick labels, no-data swatch + caption, title with unit. Mismatch: a status message instead of a wrong legend (as the React host). |
| Section | Per-instance clip bit (policy bit 32 of `World3dGpuInstance.flags.x`) + `clip_plane` in `World3dGlobals` (now exactly 256 bytes = the slot size), `discard` in the lit/painted/authored fragment shaders. The cap: two new pipelines (`fs_section_stencil`: stencil-only, inverts stencil bit 7; `fs_section_cap`: flat tone plane drawn only where bit 7 is set). Sequence in the pass's translucent list: solids toggle, cap plane, solids toggle (restores the bit to 0). |
| Highlight tokens | `modelling_options.highlight` resolved through the shared `world3d_sub_element_paint` over the theme (`neutral`=text, primary/secondary/tertiary=celebrate triad, success/warning/danger=outcome palette, info=`colors::INFO`); drives face fill overlays, face/edge outline lines and vertex markers (`widthPx` of the vertex style = mark size). No token = the previous built-in palette, bit for bit. |
| Accessibility | The paint stages a text alternative (`world3d_modelling_accessibility`) through the existing scene-accessibility presentation (stage/seal/acknowledge/discard with the frame witness) and the Interpreter appends it to the mirror: `list` + `listitem` per annotation (`Bemaßung: Breite 40 mm`), `group` + scale `list` + tick `listitem`s + no-data `paragraph` for the legend, and a polite live `status` for a mismatch. Language follows the shell locale (`SceneChromeLabels.german` -> `World3dState::set_modelling_locale`). |
| React GLB ids | `world3dGlbSubElementIds` / `world3dGlbSubElementMeshData` / `world3dGlbHitFaceId` read `_FACE_ID` (first corner per triangle) and `_VERTEX_ID` (also GLTFLoader's lower-cased names; an id table exists only if every primitive carries it, as in the Rust materializer). `GlbInstanceMesh` renders `GlbSubElementLayer` (face hover/selection overlays, vertex pick points and overlays) and the GLB group picks faces by id; both renderers resolve ids from the same attributes. |

## 2. Shared language-agnostic fixtures (`🧫️fixtures/📏️world3d-modelling/🔣️.json`, new keys)

- `annotationProjection`: camera, size, layer, projected screen geometry. Points come from three.js `Vector3.project` through the React twin; Rust reproduces them within 0.05 px / 1e-3 rad, including invisible (behind-camera) cases.
- `legendValues` (108 rows): `Intl.NumberFormat(locale, {maximumSignificantDigits: 4})` outputs (ICU oracle) for en/en-US/de/de-DE; Rust `format_world3d_legend_value` matches all, including half-expand rounding on the shortest decimal.
- `legendLines` (title/ticks/no-data in en and de), `subElementPaint` (token over default).
- Reused: `ramps`/`legends` (ramp and tick colours), `sectionPlanes` (clip plane), `pickTargets`, `glbSubElementIds`.
- Caveat: the 2D layout rules (arrow/label placement) are ours in both languages; only the point projection, plane, ramps and number text have a third-party oracle.

## 3. Tests executed (all run by me)

| Suite | Result |
|---|---|
| `cargo test -p semio-framework-ui-scene --lib world3d_modelling` | 20 passed (new: projection fixture, legend values, legend lines, accessible names, paint table, unit normal) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- a_native_gpu_context_creates_every_pipeline every_painted_shader_edit world_mesh_instance_packs world_globals_slot_size` | 4 passed (real Metal device builds all pipelines incl. the two section pipelines; WGSL anchors intact; instance layout fixture untouched) |
| `cargo test -p semio-framework-os-infinite --lib -- world::` | 273 passed, incl. 12 new: capped section roles/order/plane/cap pose/theme tone, open and absent section, annotation overlay positions vs the shared projection, screen-constant arrowhead at two distances, EN/DE glyph counts, legend ramp colours top-down, mismatch message, highlight tokens through two themes, tone colours, text alternative EN/DE/mismatch |
| Headless GPU render `on_a_real_gpu_a_section_removes_the_far_half_and_the_cap_closes_the_cut_without_leaking_stencil` (real prepared packet -> offscreen device -> readback) | passed: a z=0.5 section removes exactly the upper half in a side view; with cap the top view changes only the cut area (576 px) in the danger tone; all probes outside the cube equal the open render (stencil bit restored, nothing leaks) |
| `bun ./📜️script.ts world3d-modelling-check` | 46 passed (4 fixture-row + 4 GLB-id tests new) |
| scene package `bun ./📜️script.ts test` | 91 passed |
| `tsc --noEmit` (react package) | 0 errors in `📏️modelling`, `World3dHost/🟦️.tsx`, the modelling test; remaining errors are peers' files |

## 4. Not verified / limits

- Blocked: `cargo test -p semio-framework-os-renderer-wgpu --lib glb_sub_element_id` and the new Interpreter test `accepted_world3d_overlay_controls_publish_read_only_nodes_and_a_polite_mismatch_status`. The renderer crate does not build because of peers' in-flight changes (`flow/sqlite/snapshot/reconstruction`, `semio-framework-artifact-infinite-dag`: `RetainedCloneGrant` vs `Grant`), unrelated to world3d. The renderer edits (Scenes, Shell, Interpreter, its test) follow the event-feed accessibility pattern but are NOT compiled; re-run both commands when the peer break lands. Lane V's earlier "1 passed" for the GLB ids predates the world edits.
- Labels are verified as overlay instances (counts/positions); the headless packet carries no glyph atlas, so text and legend are not pixel-checked on a GPU.
- Edge `widthPx` is not honoured (1 px `LineList` pipeline); colours, face fills and vertex mark size are.
- Number text: en/de families only (`de-CH`, `fr` fall back by language family).
- The cap appears once the shared `gumball-plane` placeholder is built (~70 frames cold in a headless run); the section clips immediately.
- Clipping covers inline, painted and authored (GLB) instances; celebrated instances are not clipped (celebration shader has none). The cap stencils only opaque inline solids (React's `world3dSolid` contract); open meshes give parity noise as in React. Hover/selection overlays are not clipped (also true in React).

## 5. Files changed

Fixture: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/📏️world3d-modelling/🔣️.json` (new keys only).
Scene crate: `…/🎬️scene/📏️world3d-modelling/🦀️.rs` (overlay region), `…/🧪️tests/🔬️world3d-modelling-unit/🦀️.rs`, `…/📐️math/🦀️.rs` (`SceneSectionRole3d`, `SceneInstanceMaterial3d.section`, `ScenePass3d.section_clip`).
wgpu target: `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`, `…/🎨️shaders/🦀️.rs`, `…/🧩️component/🦀️.rs`, `…/🦀️.rs` (re-exports).
World: `♾️infinite/🌍️world/🦀️.rs`, `…/🧪️tests/🔬️unit/🦀️.rs`.
Renderer: `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, `🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`.
React: `🌐️World3dHost/📏️modelling/🟦️.tsx`, `🌐️World3dHost/🟦️.tsx`, `🌐️World3dHost/🧪️tests/📏️modelling/🟦️.tsx`.
Temporary generator and logs lived only in `🗑️generated/lane-v2` (removed at the end).
