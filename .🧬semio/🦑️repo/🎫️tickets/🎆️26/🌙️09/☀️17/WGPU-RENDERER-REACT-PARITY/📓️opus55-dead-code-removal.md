# Dead wgpu code removal (opus55)

## Re-verification of the audit
- CONFIRMED dead and deleted: immediate widgets crate, per-element wgpu files, hosting `shell`, winit half of hosting `host`, light `tabs`.
- REFUTED (live, kept): hosting `scene_slots` (ScenePaintStep/SceneHost used by Scenes, Interpreter, engine), hosting `accessibility` (accessibility_projection used by Interpreter), hosting `host` clipboard part (ClipboardContent/ClipboardIoJob used by retained-ui-engine), light `presence_bar` (Shell staging production), light `minimap` (dag board host/graph/model).
- Input element: no live call, deleted.

## Deleted
- `UI/🪀️widgets/🦀️.rs`, `UI/🪀️widgets/📦️packages` (crate semio-ui-wgpu-widgets), orphan `UI/🪀️widgets/↗️affine` (never wired; its glyph-catalog advance law is uncompiled, re-home if wanted). Kept `🪀️widgets/🧰️kit`.
- Element wgpu files: Button, Toggle, KeyValue, Ring, IconSelector, Input, Tree (`🎯️targets/🧊️wgpu/🦀️.rs`), Tabs (+ `Tabs/🧪️tests/🔬️wgpu-unit`).
- `UI/🐚️shell`, `UI/🧪️tests/🔬️targets-wgpu-shell-unit`, `UI/🧪️tests/🔬️targets-wgpu-widget-metrics`.
- hosting `host`: removed winit window-event bridge (pointer_coords, modifiers_from_winit, WindowInputState, dispatch_window_event); clipboard kept.
- PR `🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🔽️immediate-select-wheel` (+ include).
- Engine `Ui`: removed `shell` field, `set_window_kind_icons`, `set_window_layout`, `set_navbar`, `shell()`, `dispatch_shell_event`, Shell imports; engine-unit law `set_window_layout_wires_into_the_facades_shell`.
- engine-unit tests: removed all immediate-path laws (render_widget/measure_widget goldens, widget-interaction render laws, measure laws, known-gap parity, WidgetHarness, to_widget_node converters). Kept retained-only laws and kit laws (wrap_text, render_scroll_region, draw_text_on, interaction maps).

## Wiring edited
- `🧰️framework/Cargo.toml` member, controls `🦀️.rs` (6 mods removed; Select/Slider/Stepper kept) + description, light `🦀️.rs`/Cargo description, hosting `🦀️.rs`/Cargo (locale moved to dev-deps), engine companion Cargo (widgets dep), scene-tables (unused import + dep), shell-tests-input-1 dep, `ui/📜️script.ts` UI_WGPU_PACKAGES, `📋️build-owners.json`, `🔒️dependencies.json`, wasm compiler fixture json. Cargo.lock files NOT regenerated (cargo will do it; if run `--locked` it needs a refresh).

## Coverage notes (behavior the live path should have)
- Deleted `a_published_immediate_select_owns_wheel_without_parsing_its_option_id`. The retained equivalent ALREADY EXISTS and is stronger: `wheel_over_an_escaped_select_popup_scrolls_its_accepted_viewport_only` in `🔬️targets-wgpu-events-unit/🎯️event-fixtures-hit-test-capture-and-focus-laws` (same fixture `retained-select-overlay-raster` `select.wheel`; asserts only the select scrolls, unrelated root unchanged, no commands, NaN/inf ignored, clamp to an independent taffy-oracle maximum both directions). The behaviour is implemented in events `dispatch` (Scroll over `state.select_popup`). Follow-up added to that law: closing the popup drops `select_popup`, resets scroll, and a closed select owns no wheel (the immediate-only "accepted popup stays authoritative after open_selects cleared" has no retained analogue: `finish_close` clears the accepted viewport). No new behaviour needed; code not compiled.
- Deleted immediate-vs-retained golden parity (no longer meaningful). Tree row action overpaint, checkbox outline and selected-ring laws retained.

## Engine cell finding (NOT a defect, no change)
`install_ui_engine_locale` has four production callers: `ShellState::new` (session-and-topology:27, run by all boot paths: browser-worker, frame-transaction, native-socket-probe), chrome preferences load (chrome-frame-and-main-window:82), settings locale switch (dispatch-action/settings:84) and `load_ui_prefs_once` (test/testkit). `UI_ENGINE` is a process-wide OnceLock<Mutex<_>> in production, so one install serves all workers; accessors only run after documents exist, i.e. after shell boot. The `Ui` locale is genuinely used (RetainedPaintPresentation.locale), so Option cell stays. No fix needed.

## Verification status: NOT COMPILED
Cargo was run for semio-ui-wgpu-hosting only; two attempts aborted with `Source boundary exceeded: semio-ui-wgpu-events (active lines 4032/4000)` (another agent's edit, not mine); a third attempt with `--manifest-path 🧰️framework/Cargo.toml` was left running and unread. Per build policy no further cargo was started. Remaining for the integration agent, all with `--manifest-path 🧰️framework/Cargo.toml`: check+test `semio-ui-wgpu-hosting`, `semio-ui-wgpu-light`, `semio-ui-wgpu-controls`, `semio-ui-wgpu-engine` (+ engine companions, `semio-ui-wgpu-engine-laws`-style companion crates incl. `🧪️tests/🥣️companion` for engine-unit), `semio-ui-wgpu-widget-kit`, `semio-renderer-wgpu`, scene-tables, shell-tests-input-1, ui-engine-tests. Expect possible fallout: unused imports in engine-unit includes (e.g. StdHashMap, InputState, WidgetContext, SliderMeta etc. in `🔬️targets-wgpu-engine-unit/🦀️.rs` and `use` lists in the two engine companions), any remaining reference to `VIEWPORT`, hosting deps now unused (winit is still used by cursor), Cargo.lock refresh.

## Run list (integration agent, `--manifest-path 🧰️framework/Cargo.toml`)
check+test: semio-ui-wgpu-events (incl. events-unit companion for the extended select law; note events source-boundary 4032/4000 must be fixed by its owner first), semio-ui-wgpu-hosting, semio-ui-wgpu-light, semio-ui-wgpu-controls, semio-ui-wgpu-engine + both engine companions, semio-ui-wgpu-widget-kit, semio-ui-wgpu-paint, semio-renderer-wgpu, scene-tables, shell-tests-input-1, ui-engine-tests. Then refresh Cargo.lock.
