# Dynamics wiring: scene tracks, viewport toolbar, ten Eurocode windows

Status: code complete, NOT compiled (integrator owns `@semio-tech/norm-artifact-contract-rs`; artifact `-rs` builds belong to the norm agents; "ready for rebuild: dynamics wiring" appended to every `build-enXXXX/latest.md`, request filed in `requests.md`). Twins: `wp01-gen-twins.py --check` 99 types green, `wp02-gen-twins.py --check` green.

SC = `✏️s/🔌️plugins/📕️norm/🎬️structural-scene`, SURFACE = `…/📇️registry/🧬️contract/🖥️app-surface`, VP = `SURFACE/🪟️visual-windows/🧊️viewport3d`.

## 1. Schema
- `StructuralScene.tracks: Vec<ParameterTrack>` (validated: tracks, unique ids `unique-track-ids`, keyframe layers exist `track-layer-exists`); `SceneDelta.tracks: Option<Vec<..>>` with diff/apply; `Fragment.tracks` merged by `assemble`. Only 4 real literals existed in SC (+ viewport test, + 10 kit `finish`); the "42" were mostly signatures. Each artifact `kit::Out` gained `pub tracks` and `finish` forwards it (`Out::new` of EN 1996 too).
- Projection kit `🎞️tracks`: `key track indexed timed phase` (+ `mode_track` by the steel fork). Test `track_constructors_emit_valid_tracks`.
- `Dynamics` removed: `lower_dynamic(scene, inputs, &PlaybackState, tess)` reads `scene.tracks`. Tracks not selected rest on their first keyframe (`playback::sample(t, t.min)`), so mode/fire layers do not pile up in the static view. Test `tracks_come_from_the_scene_and_unselected_track_layers_rest_on_their_first_keyframe`.
- Shell mode samples: `SurfaceSample.at` for shells is now physical (arc length in metres along columns and rows from the first node, averaged over the other index): `parts::surface_coordinates`; schema docstring updated; test `shell_mode_samples_use_physical_surface_coordinates`.

## 2. Windows
- New `VP/🧰️toolbar`: `ViewportView { config, playback }`, `render_toolbar` (field section: Utilisation plus every scene field; playback section: static view, tracks, play/pause with readout, step, keyframe seeks, speeds; all EN/DE, rows dispatch the six existing actions via `norm_action`), `render_viewport_panel` (lower_dynamic + toolbar above the `world-3d` surface).
- All 20 windows (`✏️editor`, `👁️viewer` of EN 1990..1999): `render(..)` unchanged signature, forwards to new `render_with(.., &ViewportView, locale)`. Editors use `crate::CONTROLLER_ID`, viewers define `CONTROLLER = "s.norm.enXXXX@1/*#viewer"`.
- Not done: the host reducer that applies the toolbar actions to a persisted `ViewportView` (callers pass `ViewportView::default()` today); needs the host/config wiring of `command_of`.

## 3. Tracks per artifact
| EN | Track(s) | Binding |
|---|---|---|
| 1990 | `combination` (Combination, 1..n columns) | `Instance` fields `combination:<column>` (gamma*psi) on every load entity |
| 1991 | `fire-time` (Time, 0..duration, 9 stops); `traffic-position` (Combination, 5 stops) | `temperature@<min>` fields (ISO 834) on fire entities; tandem copies on layers `traffic-position-1..5` |
| 1992 | `fire` (Time 30..120 min), `creep`, `shrinkage` (Age 28 d..50 a) | grid surfaces with Vertex fields `temperature@`, `creep@`, `shrinkage@` (Annex B closed forms) |
| 1993 | `mode` (Mode, flexural, LTB) | sibling mode entities on layers `mode-flexural`, `mode-ltb` |
| 1994 | `mode` (buckling, LTB) | new mode entities (kernel Euler / LTB), emitted only with two modes |
| 1995 | `fire` (Time 30..120 min) | char/zero-strength layers `fire-30..120` (beta_n t) |
| 1996 | `fire` (Time R30..R240) | grid surface field `fire-demand@R` |
| 1997 | `consolidation` (Time, Terzaghi Tv 0..1.5) | `settlement@<d>d` fields, last = `settlement` |
| 1998 | `mode` (x1..y3) + `phase` | mode layers `mode-*`, overlay moved off the slabs/columns |
| 1999 | `mode` (flexural, LTB) | as EN 1993 |

Tracks appear only when there is something to scrub (indexed needs two stops, timed ascending stops).

## 4. Tests
Scene crates: per-artifact track test (presence, `validate`, keyframe bindings exist, closed-form golden: EN 1991 841.8 C at 30 min, EN 1992 phi(365 d)=1.80509, EN 1996 R90 0.13875 m, EN 1997 U(Tv 0.197)=0.5, EN 1998 mode 1 roof 1, EN 1993/1994 Euler) and a `.feature` scenario each. Viewer window tests: `render_with` at a fixed track value (assert Ok only; the readout text is covered by `the_toolbar_lists_fields_tracks_and_the_readout_in_both_languages` in VP, "Fire duration: 30 min" / "Branddauer: 30 min"). VP scenarios added to `🧪️tests/🥒️.feature`.

## 5. Open points
- Frame goldens are structural (readout, layers, closed forms), not mesh hashes: none could be generated without a compile. Re-bless scene JSON fixtures (EN 1992, 1995, 1996 and others with fire/mode) after the first green run.
- Mode siblings (1993, 1999) duplicate member geometry on their layer: check z-fighting; mute with `Ghost` if needed.
- EN 1992/1996 fields are flat per face; only the EN 1992 fire field varies spatially. EN 1991 traffic: stop 1 keeps old ids, new ids `bridge:lm1-position-n-tandem-*`.
- Assumptions to verify at compile: glob imports in `toolbar` (`norm_action`, `chrome`, `ui_error`), `Option::is_none_or` (Rust 1.82), EN 1998 test assumes a fixture with two mode layers.
- `viewport-dynamics.md` "NOT done" items on `tracks` and `SurfaceSample` are now resolved here.

## 6. Host reducer (follow-up)
- Lane: `ViewLane { active_field, track, value, playing, clock, speed }` in `NormTransient` (ephemeral local-only; `is_clear` ignores it). Events `ViewField/ViewTrack/ViewSeek/ViewToggle/ViewSpeed` fold in `NormTransient::apply`; `view_event(action, text, number)` maps the six viewport actions (Step carries the target value computed by the toolbar, which also covers wrap/clamp). Selecting a track resets value and playback.
- Dispatch: one editor tool `viewportAct {action, text, number}` (Transient lane only, no document, no CRUD) routed by `norm_live_commands!` / `norm_live_editor!` into `live_ephemeral`; ten editors carry the enum row, tool id, action definition.
- Render: editor windows pass `frame.transient.view` as `ViewportView.lane`; `render_viewport_panel` calls `resolved(scene)` (lane field and speed over config, lane track snapped through `playback::reduce` Seek). Toolbar rows now dispatch `viewportAct`.
- Tests: `viewport_actions_fold_into_the_view_lane_and_round_trip` (lane runtime), `a_viewport_action_changes_the_rendered_state` (readout, playing, active field and legend change), feature scenario added.
- Open: the 10 viewers have no ephemeral facet (`NoTransient`), so their windows still render the default view; no host `Tick` yet, so Play toggles the flag but the clock does not advance; reduced-motion preference is not read yet.
