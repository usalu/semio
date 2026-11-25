# wgpu laws handoff

Laws 1, 4, 10/11, 12 and audit2-native lanes 1 to 4 on the Rust/wgpu side. Schema and cases first, tests next, code last.

## Schema and cases (new)
- `🧱️elements/🔽️Select/🧭️behavior/popup-rows.{schema,cases}.json` (`select-popup-rows/v1`)
- `🧱️elements/🖱️ContextMenu/rows.{schema,cases}.json` (`context-menu-rows/v1`)
- `🧱️elements/🔝️Navbar/bands.{schema,cases}.json` (`shell-bands/v1`, shared by navbar and footer)
- `🧪️tests/🧫️fixtures/🔽️select-popup-geometry/🔣️.json` updated by hand: no viewport padding, `rowHeight` 24 (chip row), new popup and scroll numbers.

## Law 1 / lane 1 (chip glass level)
Sites: card stack, Dock tab groups, pane chips, time-travel chips. Verified against React: the web card is an `OverviewCard` at `level="dialog"`, so the card stack now uses `chip_glass(Level::Dialog)` (was Base). Dock groups, pane chips and time-travel chips cover `canvas_clear` (the Base canvas), so Base is the right covered level and stays. `Theme::chip_glass`/`Level::above` math test exists (draw-execution unit test) and passes.

## Law 4 / 10 / 11 (bands)
- `Theme::band_glass()` = `chip_glass(Level::Pane)` (panel glass plus chip tint). `Theme::chip_row_height()` = `max(1.5 rem, 7 ui-spacing)`.
- Shell navbar and footer: glass band painted into the overlay list (above the canvas), content inside `begin_glass_content`, opened and closed per walk opportunity (`ShellChromeChildCursor::band_glass`). The opaque `theme.navbar` solids are gone. The bands paint their own shared line (`shell_navbar_line_rect`, `shell_footer_line_rect`).
- `shell_band_layout`, `shell_band_hit`, `SessionCtx::{band_layout, canvas_rect, body_rect}` (session crate, `pub use` added). Main-window floor now fills the whole client (`canvas_rect`); docks, windows and panels keep `body_rect` (band height as clearance).
- Hit testing: windows and docks are laid out in the body, so a band wins by construction; `shell_band_hit` is band first, tested against the cases.
- Law 10/11: panels are painted before the navbar in the same overlay list, bands last; no canvas paint is above a band.

## Law 12 / lane 4 (Select and context menu rows)
- Select: `select_row_height` = `chip_row_height`, no viewport padding, rows span the viewport, single hairlines between rows, chosen row `accent` fill with `active_foreground` ink, hovered or highlighted row `row_hover` fill with `border_emphasized` (emphasised) ink, no tick, chevron from the icon catalog (trigger and scroll bands). Three painters updated (immediate kit, retained control node, reference). Contract chosen: the React Select (fill plus emphasised text), documented in `select_row_colors`.
- Context menu: rows chip-row high, full width, no inset, lines between rows, an unlabelled separator has no height, active row hover fill plus emphasised text, submenus abut (no gap), `Rounded` paint op removed.

## Lane 3 (square controls)
Colour swatch (two painters), media overlay button, track and thumb, slider track rail, range and thumb (dial and ring stay round: drawn content), progress track and fill. Radius arguments dropped from all chrome `push_glass` sites and from `open_chrome_overlay_glass_content` (signature lost the radius). Presence avatars and node content untouched.

## Verification
All through `lockf -k /private/tmp/claude-501/semio-cargo.lock repo cargo --store=shared --configuration=prototype -- test -j 3 ...`.
- `-p semio-ui-theme -p semio-ui-wgpu-light --lib`: 0 + 28 passed (card stack, chip glass inside frame). Logs: `/private/tmp/claude-501/wgpu-law1.txt`.
- `-p semio-ui-select-behavior -p semio-ui-wgpu-draw-execution --lib`: 21 + 23 passed (popup rows cases, row colours, chip glass math, band glass, chip row height). First run failed two stale expectations (row pitch epsilon, fixture row height); fixed, rerun green (`wgpu-law12b.txt`).
- `-p semio-renderer-wgpu-shell-tests-chrome-5 -p semio-renderer-wgpu-shell-tests-input-3 --lib -- shell_bands context_menu tour_overlay navbar_footer`: chrome-5 18 passed (incl. the three new `shell_bands` tests), input-3 first run 25 passed 1 failed (scroll test used `control_height`; fixed to `chip_row_height`), rerun of input-3 26 passed (`wgpu-os-a.txt`, `wgpu-os-b.txt`).

## Open, stated honestly
- Not compiled or run: `semio-ui-wgpu-paint` and the other paint crates (control-nodes row paint, reference painter, slider/progress/media squares, glass radius drops in Select target) were only built as dependencies of the passing runs where reachable; their own test suites were not run. Other shell crates (view-chrome/view-panels tests beyond chrome-5/input-3) not run.
- The main-window floor spans the client, but docks, windows and panels still lay out in the body (band height as clearance). Edgeless full-bleed windows beneath the bands are not done.
- Select rows are hairline pixels, not compositor cells (`push_shared_border_edges`), same for context menu lines.
- Dial slider and ring keep round shapes (drawn content). Switch rail and radio dot painters were not found in this pass; not changed.
- Tree rows and some `push_rounded` calls with `border_radius` (token 0) remain.
- Lanes 5, 6, 7 untouched (other agent). No React/CSS/TS file touched except the Select popup fixture JSON.
