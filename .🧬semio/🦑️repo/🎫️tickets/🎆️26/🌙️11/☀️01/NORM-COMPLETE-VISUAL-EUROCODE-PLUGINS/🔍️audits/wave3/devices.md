# W3-DEVICES audit (2026-11-03)

Read-only audit of the ten norm editors and the shared app-surface for mobile and tablet support of tonight's features. Code was checked, not the reports. Shorthand used below:

- `APP` = `/home/deck/semio/✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface`
- `WGPU` = `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu`
- `TSW3D` = `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost`

"Production caller" means a non-test call site. A test-only call site does not make a feature reachable.

## Device classes and window kinds

| Device | Window kinds (layout) | Evidence |
|---|---|---|
| Desktop (`≥1024 px` or no touch) | Five windows in a docked dock: outliner, viewport, properties, checks, diagrams. Min target 24 px, hover on. | `APP/🪟️visual-windows/📱️device-profiles/🦀️.rs` (`ROLES`, `device_profile`) |
| Mobile (`<600 px`) | One tabbed stack of four tabs: viewport, properties, checks, diagrams. No outliner. Properties is a bottom sheet over the model tab while something is selected. Min target 44 px, tap-to-nudge handles, no hover. | `APP/🪟️visual-windows/📱️device-profiles/📐️layout/🦀️.rs` (`MOBILE_ROLES`, `bottom_sheet`) |
| Tablet (`<1024 px` with touch) | Two panes: viewport plus side tabs (properties, checks, diagrams). No outliner. Min target 44 px, free drag on handles, nudge buttons on. | `APP/🪟️visual-windows/📱️device-profiles/📐️layout/🦀️.rs` (`SIDE_ROLES`, `TabletPrefs`) |

All ten editors share one shell hook (`NormShellHooks::choose_layout`), so device behaviour is the same in en1990 to en1999. Only the command tables differ per editor.

## Per-feature table

Status: ok = reachable and wired in production on that device class; partial = reachable with a gap or only through a non-norm path; missing = no production path on that device class. Evidence paths are absolute.

| # | Feature | Desktop | Mobile | Tablet | Evidence |
|---|---|---|---|---|---|
| 1 | Window layouts per device class | ok (5 windows) | partial (4 tabs, no outliner) | partial (2 panes, no outliner) | `/home/deck/semio/✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📱️device-profiles/📐️layout/🦀️.rs` (`MOBILE_ROLES`, `SIDE_ROLES`); `/home/deck/semio/✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🔌️shell-hooks/🦀️.rs` (`choose_layout`) |
| 2 | Outliner (entity tree, primary selection surface) | ok | missing | missing | `.../📐️layout/🦀️.rs` (`MOBILE_ROLES` and `SIDE_ROLES` have no `outliner`) |
| 3 | Tap select (single pick) | ok (mouse) | partial: primary press reaches pick through winit `Touch` to `PointerDown`; the norm gesture table is not called | partial: same | `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🪟️winit-app/🚀️native-winit-app/🦀️.rs:279-287` |
| 4 | Long-press: add to selection (`LongPress` → additive pick) | ok (Shift/Ctrl click) | missing: `map_gesture` has no production caller | missing | `.../🪟️visual-windows/📱️device-profiles/👆️gestures/🦀️.rs:46-62` (only test callers) |
| 5 | Long-press drag: rectangle select | ok (marquee) | missing: `LongPressDrag` is produced by nothing | missing | `.../👆️gestures/🦀️.rs:56` |
| 6 | Handle grip drag (3D, free drag) | ok | missing: `TapNudge`, so a one-finger drag on a handle orbits instead of editing | partial: `HandleMode::FreeDrag` exists, but the 44 px hit area is not applied (see 7) | `.../👆️gestures/🦀️.rs:55`; `.../🕹️direct-manipulation/🛠️handle-drag-tool/🦀️.rs` |
| 7 | Handle hit area ≥ 44 px on touch | n/a (24 px desktop minimum, not applied in production either) | missing: `handle_hit_px` has no production caller | missing: same | `.../📱️device-profiles/🦀️.rs:81-82` (only `.../🧪️tests/🦀️.rs:37-38, 271` call it) |
| 8 | Nudge buttons (tap-to-nudge handle) | ok (keyboard arrows in `.../🕹️direct-manipulation/⌨️keyboard/🦀️.rs`) | missing: `Gesture::NudgeButton` and `Intent::HandleNudge` have no production producer or consumer | missing | `.../👆️gestures/🦀️.rs:35, 60` |
| 9 | Marker grip drag (diagram marker bound to a field) | ok | partial: the tool is shared, but the 8 px pick target is too small | partial: same | `.../🕹️direct-manipulation/🪧marker-grips/🦀️.rs`; `.../📈️diagrams2d/🖼️canvas-lowering/🦀️.rs:11, 493` (`MARKER_RADIUS` 6 px + 2 px) |
| 10 | Diagram marker and point pick (hit-test) | ok | partial: `pick(targets, p, tolerance)` takes its tolerance from the caller; no device-aware value; diagram budget 2 000 points on mobile is applied | partial: 6 000 budget, same 8 px targets | `.../📈️diagrams2d/🎯️hit-test/🦀️.rs:100-104`; `.../📈️diagrams2d/🖼️canvas-lowering/🦀️.rs:493, 569` |
| 11 | One-finger orbit (3D) | ok | partial: web shell has the orbit rig; native wgpu has no 3D orbit or touch wiring in `APP/🪟️visual-windows/🧊️viewport3d` | partial: same | `.../🪟️visual-windows/📱️device-profiles/👆️gestures/🦀️.rs:56` (`Orbit`, production-unused); `TSW3D/👆️pointer-and-drop/🔶️active-utility/🟦️.ts` (web orbit rig, see row 12) |
| 12 | Two-finger pan and pinch zoom (3D) | ok (pointer, wheel) | partial: web shell `TSW3D/👆️pointer-and-drop/🔶️active-utility/🟦️.ts:16` yields to pinch (orbit rig owns pan and zoom); native wgpu 3D: missing (`GestureRecognizer` only used by 2D node-graph and tiled-map scenes, see `WGPU` scene-state and node-graph) | partial: same as mobile | `TSW3D/👆️pointer-and-drop/🔶️active-utility/🟦️.ts:16`; `.../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🧭️node-graph-and-tiled-map/🦀️.rs:150-166`; `.../🗃️scene-state/🧯️surface-state-and-retirement/🦀️.rs:45` |
| 13 | Touch-menu long-press (Edit menu) | ok (right-click) | missing: `touch_menu()`, `render_touch_menu()` and `publish_touch_menu_access()` have no production caller; winit maps touch to a primary press only | missing | `APP/🧰️entity-editing/🖐️touch-menu/🦀️.rs:19, 57`; `APP/🧰️entity-editing/🧭️host-routes/🦀️.rs:196`; native winit `.../🚀️native-winit-app/🦀️.rs:283-287` |
| 14 | Context menu (right-click plus Shift+F10 and the context-menu key) | ok | missing: secondary press is `button == 2` only, `open_context_menu` has no touch caller | missing | `WGPU/📥️staging/🍱️view-chrome/🖱️pointer-input/🦀️.rs:157` (`button == 2`); `WGPU/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🔁️deferred-menus/🦀️.rs:254` |
| 15 | Bulk edit (menu row plus Properties bulk inspector) | ok (menu row, Alt+3) | partial: Properties tab and bottom sheet render the bulk inspector, but the multi-selection cannot be made by touch and the menu row is unreachable | partial: same, side pane | `APP/🧰️entity-editing/🍽️context-menu/🦀️.rs:102`; `APP/🪟️visual-windows/📱️device-profiles/📐️layout/🦀️.rs` (`bottom_sheet`) |
| 16 | Multi-entity delete (`deleteEntities`) | ok (Delete key plus context menu) | missing: no touch selection, touch-menu Delete entry unwired | missing: no Delete key, touch menu unwired | `APP/🧰️entity-editing/🗑️delete/` (reducer); `APP/🧰️entity-editing/🖐️touch-menu/🦀️.rs` |
| 17 | Touch menu entries (Duplicate, Copy, Paste, Delete, Select all) | n/a | missing (unwired, see 13) | missing | `APP/🧰️entity-editing/🖐️touch-menu/🦀️.rs:19-35` |
| 18 | Focus window chords and palette (Alt+1..5, `focusWindow`) | ok | partial: no device filter in `handle_key`, so Alt+1 opens an outliner as a root split on mobile | partial: same | `APP/🔌️shell-hooks/🦀️.rs:150-170` (`handle_key`); `.../🔌️shell-hooks/🧪️tests/🦀️.rs` |
| 19 | Properties on phone (bottom sheet) | n/a (docked) | ok: sheet with `min_handle_px` 44 and peek, half, full states | partial: side pane, no sheet | `APP/🪟️visual-windows/📱️device-profiles/📐️layout/🦀️.rs` (`bottom_sheet`, `SheetSpec.min_handle_px`); `APP/🔌️shell-hooks/🦀️.rs:152` |
| 20 | Layout customisation (resize, hide, reorder tabs) | ok | ok (tab order in `MobilePrefs`) | ok (side tab order in `TabletPrefs`) | `APP/🪟️visual-windows/📱️device-profiles/📐️layout/🦀️.rs` (`apply_layout`, `sanitised`) |
| 21 | Keyboard-only paths without a touch equivalent (Shift+F10, Ctrl+D/C/V/A, Delete, Alt+1..5, arrow nudges) | ok | missing for all keyboard-only paths; the touch menu that would cover Ctrl+D/C/V/A and Delete is unwired | missing: same | `APP/🧰️entity-editing/🍽️context-menu/🦀️.rs`; `WGPU/📥️staging/🖐️pointer/⌨️keyboard-and-palette/`; `APP/🕹️direct-manipulation/⌨️keyboard/🦀️.rs` |
| 22 | Latency tests cover touch drag | ok for mouse-style value drag (non-touch input) | missing: no touch input in the drag bench; the device gesture tests only test dead `map_gesture` | missing: same; `DRAG_P95_MS` (12 ms) is one gate for every device, mobile debounce (120 ms) is not measured | `APP/🧪️tests/⏱️latency-bench/🦀️.rs:191-204` (`gesture_json`, synthetic `s` value); `APP/🪟️visual-windows/📱️device-profiles/🧪️tests/🦀️.rs:222-265` |

## Top 10 gaps, ranked by impact

1. **The long-press Edit menu is not wired in production (row 13).** `touch_menu`, `render_touch_menu` and `publish_touch_menu_access` only have test callers, and the native winit host maps every touch to a primary press. Duplicate, copy, paste, delete, select all and bulk edit have no touch path at all. Evidence: `APP/🧰️entity-editing/🖐️touch-menu/🦀️.rs:19`, `APP/🧰️entity-editing/🧭️host-routes/🦀️.rs:196`, native winit `.../🚀️native-winit-app/🦀️.rs:283-287`.
2. **Handles cannot be edited on a phone (rows 6, 8).** Mobile uses `TapNudge`, so a one-finger drag on a handle orbits, and the nudge buttons (`Gesture::NudgeButton` to `Intent::HandleNudge`) are produced by nothing. Evidence: `APP/🪟️visual-windows/📱️device-profiles/👆️gestures/🦀️.rs:55-60`.
3. **Multi-selection is not reachable by touch (rows 4, 5).** Long-press add (`Additive`) and long-press rectangle select are dead code, and there is no outliner on mobile or tablet. Without selection, bulk edit (15) and multi-delete (16) cannot start. Evidence: `.../👆️gestures/🦀️.rs:46-62`, `.../📐️layout/🦀️.rs`.
4. **The outliner is missing on mobile and tablet (rows 2, 1).** It is the only list-based selection surface on desktop. `MOBILE_ROLES` and `SIDE_ROLES` leave it out, so touch users have only 3D picking. Evidence: `.../📐️layout/🦀️.rs` (`MOBILE_ROLES`, `SIDE_ROLES`).
5. **The 44 px touch hit area is not applied (row 7).** `handle_hit_px` only has test callers, so handle and marker targets stay at their desktop size on touch devices. Evidence: `.../📱️device-profiles/🦀️.rs:81-82`.
6. **Native wgpu 3D has no two-finger pan or pinch, and no one-finger orbit (rows 11, 12).** The pinch recognizer only feeds 2D node-graph and tiled-map scenes. Only the web shell has the orbit rig. Evidence: `.../🧭️node-graph-and-tiled-map/🦀️.rs:150-166`, `TSW3D/👆️pointer-and-drop/🔶️active-utility/🟦️.ts:16`.
7. **Context-menu parity is absent on touch (row 14).** Secondary press is `button == 2` only, and `open_context_menu` has no touch caller. Evidence: `WGPU/📥️staging/🍱️view-chrome/🖱️pointer-input/🦀️.rs:157`.
8. **Diagram marker targets are 8 px on every device (rows 9, 10).** `MARKER_RADIUS` (6 px) plus 2 px, and the pick tolerance is supplied by the caller with no device value. Evidence: `APP/📈️diagrams2d/🖼️canvas-lowering/🦀️.rs:11, 493`.
9. **Focus-window chords ignore the device class (row 18).** `handle_key` has no device input, so Alt+1 on mobile opens an outliner as a root split. Evidence: `APP/🔌️shell-hooks/🦀️.rs:150-170`.
10. **The latency gates do not cover touch input (row 22).** The drag bench drives `HandleDragTool` with synthetic values, the device gesture tests cover only dead `map_gesture`, and the 120 ms mobile debounce and 33 ms frame budget are not gated. Evidence: `APP/🧪️tests/⏱️latency-bench/🦀️.rs:191-204`.

## Notes

- Keyboard-only paths with no touch equivalent (row 21) are covered by gaps 1, 2 and 3 once those are fixed. Fixing 1 alone gives the Edit menu entries for copy, paste, delete and select all.
- `map_gesture` is called from tests only, so its unit tests prove the intent table, not the device behaviour.
- Web shell (React/TS) long-press: no handler found in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, so long-press is missing on web too.
- Unverified: whether the web shell's touch pinch works on the norm 3D viewport end to end. Only the source of the yield to the orbit rig was read, not a run.
