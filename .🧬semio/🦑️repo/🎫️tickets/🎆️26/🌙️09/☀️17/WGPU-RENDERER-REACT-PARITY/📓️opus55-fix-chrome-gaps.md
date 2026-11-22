# Chrome parity fixes (opus55): intro, panel drop, toasts, focus ring, hotkeys

Nothing was compiled or run (build freeze). Every file below is written to compile and carries laws; the integrator owns the run.

## Shared schema (new)
`🧰️framework/🔨️modules/🖱️ui/🧬️schema/📐️chrome-parity-constants/`: `🔣️.json` (JSON schema), `🧮️values.json` (values), `🧫️introduction-docking.json` (dock and near-miss rows).
- TS: `📐️introduction-geometry/🟦️.ts` exports `INTRODUCTION_INFO_BOX_GAP_PX` (now 0 from the JSON), `INTRODUCTION_DOCK_REACH_PX` (40), `INTRODUCTION_CLEAR_RANGE_PX` (120); `🎓️introduction/🟦️.tsx` imports them; `🧲️panel-dock/🟦️.tsx` nest band reads the JSON.
- Rust: `chrome_parity_constants()` appended to `staging/🎚️chrome-state/🎨️theme-file-import-and-accessibility/🦀️.rs` (`include_str!` + serde, `OnceLock`), re-exported by `shell-chrome-state`.

## 1 Intro info box
Gap 0 (was 16), centered branch now clamps like React. Ported `dock_introduction_info_box`, `clear_introduction_info_box_near_misses`, `settle_introduction_box`, `clamp_introduction_info_box` (view-overlays `🗨️chrome-overlays-and-tour`), called from `chrome_tour_layout` with lines from `window_content_rects` (dock) plus registered element rects (chips). `INTRODUCTION_INFO_BOX_GAP` const became `introduction_info_box_gap()`; the two test sites and the chrome-4 import were updated. Laws read the shared fixture on both sides: `docking.test.ts`, `the_introduction_dock_and_near_miss_clearing_answer_the_shared_fixture_rows`.
Divergence: wgpu has no window silhouette line list, so dock lines are window content rects.

## 2 Panel drop feedback
- Row passive fill (`theme.celebrate[1]` = `--accent-secondary`) while a tab drag is live, nest highlight (`accent` at 0.2) over a branch chip in the 30 to 70 percent band, insert bar suppressed while nesting (`dock_tab_nest_target`, shell-dock).
- Empty anchors paint a 10rem by control-height zone with the grip, accent when hovered (`paint_empty_anchor_drop_zones`, called in view-panels phase 6; `empty_anchor_drop_zone_rect`).
- Drop on a branch nests as last child (`dock_tab_nest_branch`, `finish_dock_tab_drag`).
Divergence: zone placement is the anchor's corner or edge of the dock body (React renders it inside the anchor row); nesting handles the root row only; a same-anchor drop stays a no-op as before.

## 3 Toasts and status stacks
- `TRANSIENT_NOTICE_MAX_WIDTH` removed; toast is `width * toast.maxWidthFraction` (0.5, React `left-1/2`), wraps onto lines (`status_band_layout`, `transient_notice_layout`); text inset fixed to `px-double`.
- Plugin-install cap now 90 percent of the viewport (React `max-w-[90vw]`).
- New `StatusStacks` frame phase (view-panels) -> `render_status_stacks_step` (view-chrome navbar file) paints bootstrap, execution-target, artifact-creation and route-admission as shared-hairline `NoticeStack`s (`status_stack_layout`, `status_stack_ops`). Sources: bootstrap from `sync.sync_reseed` (rebootstrap text), execution-target from the document open while `Resolving` (the old band now skips that phase), artifact-creation from the hub creation door (catalog + operation, cancel/open actions dispatch the existing hub actions), route-admission from `shell_route_admission` (port of `shellRouteAdmissionV1`/`shellIdentityResolutionV1`, fixture `🧭️session-lane`). En+de keys added to `📖️shell-vocab/🧵️task-manager-and-route-vocab`.
Divergence: wgpu has no bootstrap byte-progress, execution-target progress meter or creation elapsed line, so those cells show text only.

## 4 Focus ring
Shell chrome ring: outset accent-alpha 4 edges replaced by an on-border hairline `push_shared_border(..., theme.selected /*--active-base*/, Focus)`. React uses the same compositor stroke for form controls too, so the retained painters were switched from `theme.accent` to `theme.selected` (control-nodes, reference, widget painters, Stepper, tree row ring width now hairline); the paint-law test and stale docs were updated. Not done: accent-surface inversion (`--border-emphasized-color`) and the selected-underline rule (needs a per-control accent flag the ring rect does not carry).

## 5 Hotkeys
- `key_event_matches_chord`: `mod` is meta on Apple hosts, ctrl elsewhere (`host_platform_uses_meta`); chord needs exact ctrl/meta match.
- Overrides reach any id: `keybinding_override` (parse gate), `command_active_chords` (OS and plugin commands), `match_app_keybinding` uses the override of the action id; `shell_shortcut_table` shares the gate. Persistence already projects arbitrary ids.
- Shared fixture `🧪️tests/🧫️fixtures/⌨️mod-platform-and-override-reach/🔣️.json`; TS `⌨️mod-platform-and-override-reach`, Rust `⌨️wgpu-mod-chord-and-override-reach` (mounted in shell-tests-input-3).

## 6 React bug
`os.toggleFullscreen: "mod+shift+f"` removed from `SHELL_KEYBINDINGS`; the command registry (`f11`, `control+meta+f`) is the single source (ui cannot import the OS registry, so the table carries no row). Law in `⌨️os-command-shortcuts/🟦️.ts`.

## Run list
- `cargo test` on the test bins `shell-tests-input-3` (new laws: mod, overrides, chrome parity constants, route admission, status stacks, toast), `shell-tests-chrome-4` (intro placement + dock fixture), `shell-tests-chrome-6`, `shell-tests-docs-1/2`, `shell-view-chrome`, `shell-view-overlays`, the ui wgpu paint crate (`🧪️tests/🔬️targets-wgpu-paint-unit` focus ring laws), `winit-app-p3c`.
- `repo` vitest: `🧪️tests/📐️chrome-parity-constants`, `⌨️mod-platform-and-override-reach`, `⌨️os-command-shortcuts`, `📐️introduction-geometry/docking.test.ts`, panel-dock/PanelTabBar suites.
- Expected fallout of the `mod` rule on a macOS host, tests that press ctrl for a `mod` chord (and on Linux the reverse for meta, e.g. `⌨️wgpu-chord-example-role-parity` `probe_chord` presses meta): `🔬️wgpu-shell-input/{⌨️retained-pane-focus-and-keyboard,🔎️quick-search-palette-keyboard,🧭️dock-order-key-events-and-drag}`, `🎡️wgpu-wheel-and-escape-routing`, `🔬️wgpu-shell-chrome-parity/🛑️chords-controls-and-compute-status`, `🔬️wgpu-panel-anchor-model/⚙️settings-keybindings-hug-and-mobile`, `🔬️wgpu-winit-app-p3c`, Interpreter `wgpu-ui-command-wiring`. Fix by setting the host platform in the test or pressing the platform's modifier.
- Possible compile nits to check first: `ActionDescriptor`/`StatusAction` derive (`Clone` only), `cursor.item`/`cursor.scalar` integer types in the new steps, `PanelAnchor::ALL` use, `HubArtifact*` import paths in `status_stack_cells`.
