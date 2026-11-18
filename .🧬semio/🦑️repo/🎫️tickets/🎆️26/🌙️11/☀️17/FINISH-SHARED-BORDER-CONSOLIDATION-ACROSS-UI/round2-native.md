# Round 2: native twins (wgpu, TUI) under the radical rule

All paths below are under `🧰️framework/🔨️modules/🖱️ui/` unless absolute.

## 1. Field: shared-border row cell model (no gap)

Chromium re-measurement: React `Field` (vertical `SharedBorderRow`, label/description/error as `SharedBorderRowCell`, control in a nested row) mounted in my own tab on the course dev server (5391, `touch` class removed so the compact density applies: `--spacing-single` 3.2 px, Anta loaded), width 80 px, label "Threshold value", description "Shown before control", error "Fix this value now", Button control. Raw rects (px, relative to the field):

| band | measured | fixture value |
|---|---|---|
| label text | x 3.19, y 4, w 73.63, h 19.19 | labelTop 4.0, textInline 3.2, textWidth 73.6 |
| label cell (4 + 19.2 + 4 + 1 divider) | 0 .. 28.19 | labelCellHeight 28.2 |
| description text | y 32.19, h 32 (2 lines) | descriptionTop 32.2 |
| description cell | h 41 | descriptionCellHeight 41.0 |
| control row (button 22.39 + 1 divider) | y 69.19, h 23.39 | controlTop 69.2, controlRowHeight 23.4, controlHeight 22.4 |
| error text | y 96.58, h 32 | errorTop 96.6 |
| error cell (last: no divider) | h 40 | errorCellHeight 40.0 |
| field | h 132.578 | totalHeight 132.6 (Chromium layout units 1/64) |

Changes:
- `🧪️tests/🧫️fixtures/📐️section-field-presentation/🔣️.json`: new `density.cellPaddingBlock` 4, `density.cellDivider` 1 and the field bands above (replaces the gap-based `controlTop` 57.6 / `errorTop` 83.2 / `totalHeight` 115.2). Section numbers unchanged.
- `🎯️targets/🧊️wgpu/📐️flex/🎨️style-mapping-and-chrome-metrics/🦀️.rs`: `field_chrome_metrics` models cells (`FIELD_CELL_PADDING_BLOCK` = 0.25 rem, `FIELD_CELL_DIVIDER` = hairline, no gap); the `gap` parameter is now the inline cell padding (`ChromeBand.inline`, new field, 0 on Section/control bands); new `field_description_height(control_top)`; re-exported from `📐️flex/🦀️.rs`.
- `🎯️targets/🧊️wgpu/🖌️paint/📝️retained-text-steps/🦀️.rs`: `chrome_band_rect` insets by `band.inline`; description/error wrap width is `width - 2 * gap_standard`; description height derives from `field_description_height`.
- `🎯️targets/🧊️wgpu/📌️mounted_layout/🔤️layout-input-and-text-measurement/🦀️.rs`: detail text measured at `width - 2 * gap`.
- Tests: `🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs` (label/description/error bands, total, control row, textWidth, description height), `🧪️tests/🔬️targets-wgpu-paint-unit/🦴️skeleton-celebrate-and-chrome-paint-laws/🦀️.rs` (glyph rows against `descriptionTop`/`errorTop`), TS `🛍️products/💻️os/.../🧪️tests/⚙️puzzle3d-settings-document/🟦️.tsx` (same formulas from the fixture; 7/7 pass with `vitest-os.config.ts`).
- `🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs` still calls `field_chrome_metrics(.., gap)` unchanged (signature kept).

## 2. PresenceBar Rust twin

`🧱️elements/👥️PresenceBar/🎯️targets/🧊️wgpu/🦀️.rs`: every stack (root roster, peer rows, badges, overflow) carries `gap: Some("none")` instead of `"tight"` (a `None` gap would resolve to the theme gap, so the explicit `none` token is required). New test `the_roster_has_no_gap_between_avatars` in `🧪️tests/🔬️wgpu-unit/🦀️.rs`.

## 3. Shared JSON cases

- `🧱️elements/🃏️CardStack/bands.cases.json` (+ schema): new Rust implementation `bands.rs` (`resolve_card_bands`, `CardBand*`) and `bands.test.rs` consuming all cases; mounted as `pub mod card_bands` in the light wgpu tier (`🎯️targets/🧊️wgpu/🪶️light/🦀️.rs`, crate `semio-ui-wgpu-light`; mounting it in `semio-ui-draw-contract` next to the segment resolver was tried and refused: that crate is already over its 4000 active-line source boundary). It also ports the newer `CARD_BAND_MIN_GAP` (40) / abut (0.25) rule of `bands.ts`; the first run caught that drift ("chips a few pixels apart are stacked").
- `🔨️modules/📏️border-presentation/cases.json` (consumed by `segments.test.rs`): unchanged by this ticket (only `segments.test.ts` got new browser fixtures), so Rust already passes it.
- `🧱️elements/🗂️WindowChrome/compositor.cases.json` `cellFrames` / `interlock`: NOT consumed by Rust. They are DOM/compositor fixtures (jsdom element rects, overlay counts, CSS seam colours, window silhouettes with chip geometry from `WindowSilhouette`); the Rust side has no silhouette geometry or DOM, only the segment resolver, which has its own `cases.json`. Porting the silhouette geometry is out of scope; the band decision it depends on (`bands.cases.json`) is now covered natively.

## 4. Other twins of changed elements

- `📑️Tabs` wgpu twin: React `Tabs` became `gap-0` with a `p-0` list; the Rust `stack()` used `gap: None` (theme gap) -> now `Some("none")`; test `the_tablist_and_its_panel_have_no_gap`.
- TUI twins (`Navbar`, `Footer`, `Dialog`, `Table`, `Tabs`, `Tree`, `Window`): the terminal borders were already consolidated in the previous ticket (shared boundary cells, `TUI-BORDERS.md`). The remaining 1-cell gaps (`BAND_GAP`, `BUTTON_GAP`, `STATUS_GAP`, table column gaps) separate text, not bordered boxes (the TUI has no bordered inset elements); not changed. No TUI cargo run.
- `🌳️Tree` wgpu twin (`TREE_SECTION_GAP` 8): this is the retained widget tree, not the React `Tree.Files` card that changed (card margin removed); no equivalent card in Rust, left unchanged.
- `🎴️IconSelector` wgpu twin is a single bordered chip control, not the React vertical row; unchanged.
- No Rust twins exist for `OverviewCard`, `HistoryTable`, `Popover`, `Canvas`, `Layout`, `PanelTabBar`, `LanguageChoice`, `StatTiles`, `SharedBorderRow`, `CardStack` components (only the pure band logic above).


## Verification

Commands: `repo cargo --store=isolated --configuration=prototype -- test --offline --manifest-path 🖱️ui/📦️packages/🦀️rust/Cargo.toml --lib -p <crate>` (plain `cargo` is refused by the compiler ingress; `-p` outside that manifest is "not a member").

- PASS `semio-ui-wgpu-light --lib`: 22/22 (PresenceBar incl. `the_roster_has_no_gap_between_avatars`, Tabs incl. `the_tablist_and_its_panel_have_no_gap`, `card_band_language_neutral_vectors` over all 11 `bands.cases.json` cases).
- PASS TS `puzzle3d-settings-document` (os, `vitest-os.config.ts`): 7/7 with the new fixture formulas.
- Field arithmetic checked independently (python replica of `field_chrome_metrics`): 28.2 / 32.2 / 69.2 / 96.6 / 132.6, equal to the Chromium-measured fixture.
- NOT RUN (blocked by other agents' concurrent API refactor, not by this work): the lib tests of `semio-ui-wgpu-layout` (`targets-wgpu-flex-unit`, errors `RetainedCloneGrant`/`close_granted(&mut RetainedAllocationEffect)` at lines 7, 22, 24, 524, 534-539) and `semio-ui-wgpu-mounted-layout` (`targets-wgpu-mounted-layout-unit`, 9 errors at lines 13, 198, 658, 668, 782: `InteractiveJobStep`/`try_admit_owned` signature drift). The non-test libs of both compile (the light and laws builds depend on them). So the mounted-layout Field assertions (text wrap of the 73.6 px column in Rust's atlas: 2 lines expected) and the paint-unit glyph-row assertions are written but unexecuted; `targets-wgpu-paint-unit` was not found in `semio-ui-wgpu-laws --lib` (0 of 45 matched), its host crate was not located in the time box. Rerun once the job API refactor lands.
- No TUI test run.

Browser tab `tab-18` (mine) used for the measurement; its page had the `touch` class removed from `<html>` for compact density (reload restores it).
