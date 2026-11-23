# TUI lane handoff

## Model in cells

A window is one rectangle of cells with a plain border. Chips live inside it, on the first (top) or last (bottom) interior row, above the content. A chip covers `tab.x .. tab.x + interior_width + 2` cells of that row; its outer walls coincide with the frame border when it touches the box edge, and an inner wall meets the frame edge in one junction (`┬` top, `┴` bottom). Terminals cannot blend, so the documented capability fallback applies: an opaque chip surface `Surface::Pane`, one elevation step above the content surface `Surface::Window`. Cells of the chip row no chip covers keep the content (`Surface::Window`, blank) and hit the content. There are no gap cells, no floor cutouts and no sliver closing.

Document content (the scene children) is inset by the safe clearance: `window_content_padding = [1 + top, 1, 1 + bottom, 1]` where top/bottom are the chip rows (0 or 1). The content rect itself is the whole interior (`window_content_rect`), and `window_safe_clearances` / `ChromeState::safe_clearances` report the chip rows as `[top, right, bottom, left]`.

## Before and after (one window, 44 x 6, two tabs, focused off)

Before (silhouette: chips in a cap row outside the body, a hairline seam, notch cells showing the floor):

```
┌───────────┬──────────┐
│⠿alpha ⤢ ✕ │⠿beta ⤢ ✕ │
├───────────┴──────────┴───────────────────┐
│ content starts three rows down           │
│                                          │
└──────────────────────────────────────────┘
```

After (one rectangle, chips inside on the first interior row, free cells are content):

```
┌───────────┬──────────┬───────────────────┐
│⠿alpha ⤢ ✕ │⠿beta ⤢ ✕ │                   │
│ content starts two rows down             │
│                                          │
└──────────────────────────────────────────┘
```

Cells under `⠿alpha ⤢ ✕` and `⠿beta ⤢ ✕` use the chip surface; the blank cells right of them on row 2 use the content surface and belong to the content. A bottom chip mirrors this on the last interior row with `┴` junctions.

## Changed files

Code (lane `⌨️tui`):

- `⌨️tui/🖥️chrome/🦀️.rs`: `WindowChipLayout` loses `top_body_y`/`bottom_body_y` and gains `top_clearance`/`bottom_clearance`, `chip_rect`, one-row `text_y`; `abut_chip_sliver`, `chip_gap_cells_read_as_sliver`, `CELL_WIDTH_PX`, `CHIP_SLIVER_MAX_CELLS` and `WindowTab` removed; hit testing covers whole chip cells (walls included) on the chip row; `window_drop_corner` uses the chip row only; `window_tab_rect` is the one-row chip; new `window_safe_clearances`, `window_content_rect`; padding is derived from the clearances; `ChromeState` gains `paint_frame_emphasized`, `paint_chips_emphasized`, `chip_at`, `safe_clearances`; full chrome needs 4 rows (5 with top and bottom chips), compact strips need 3.
- `🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs`: `paint_window` replaced by `paint_window_frame` (plain rectangle) and `paint_window_chips` (chips, walls, junctions, state priority emphasized over active over normal); silhouette fill, corner tabs, seams and body hairlines removed.
- `⌨️tui/⚙️engine/🖌️paint-and-gutters/🦀️.rs`: chrome paints in two layers (`ChromePart`): frame before the children, chips after them (projected drag previews paint both).
- `⌨️tui/🎬️scene/🦀️.rs`: `Scene::hit` returns the window for chip cells before descending into children.

Tests, fixtures, schema (rewritten first to the new laws):

- `⌨️tui/🧪️tests/🔬️unit/🦀️.rs`: include and imports.
- `⌨️tui/🧪️tests/🔬️unit/🎨️tui-widget-paint-and-window-frame-laws/🦀️.rs` (renamed from `…window-silhouette-laws`): box is a plain rectangle, chips inside, chip surface above content, bottom chip, composed window with hit testing (chip cells go to the chip, uncovered cells to the window, below the chip row to the content), wall and junction state colours for top and bottom chips.
- `⌨️tui/🧪️tests/🔬️unit/🪟️tui-layout-and-window-laws/🦀️.rs`: chip-in-box law, flat right side, no-gap sweep over widths 14 to 89 replacing the sliver test.
- `⌨️tui/🧪️tests/🔬️chrome/🦀️.rs`: chrome-emphasis oracle (tiny-skia raster) re-derived for rectangles, compact and hover laws, short-terminal and tiny-window laws.
- `⌨️tui/🧪️tests/🖼️window-contour/🦀️.rs`, `🥒️.feature`, `⌨️tui/🧬️schema/🖼️window-contour/🔣️.json`, `⌨️tui/🧪️tests/🧫️fixtures/🖼️window-contour/🔣️.json`: contract `window-contour-v2`, no `body` field, rectangle plus chip-wall graph, raster oracle, chip and content surface checks, no notch cell.
- `⌨️tui/🧪️tests/🖼️chrome-emphasis/🥒️.feature`: silhouette and seam wording replaced.
- `⌨️tui/🧪️tests/🪟️drag-preview/🦀️.rs`, `🥒️.feature`, fixture and schema: the cap-row case removed, chip preview is one row (Taffy oracle height 1).
- `⌨️tui/🧪️tests/🔬️pointer-routing/🦀️.rs` (unchanged), `🖱️pointer-routing/🟦️.ts`, `🥒️.feature`, fixture: golden rows are the shared edge and the chip row, wizard rows shift one row up.
- `⌨️tui/🧪️tests/🎨️emphasis/🟦️.ts`: contour fixture law for the box.

No `silhouette` identifier remains under `⌨️tui` or `🧱️elements/🪟️Window/🎯️targets/⌨️tui`.

## Verified

Run from `🧰️framework` through the repo cargo ingress (the plain `cargo` shim refuses without the launcher):

```
repo cargo --store=shared --configuration=prototype -- test -p semio-ui-tui-laws -p semio-ui-tui-unit-laws -p semio-ui-tui-frame -p semio-ui-tui-engine --no-fail-fast
```

Result: `semio-ui-tui-laws` 102 passed (chrome anatomy, pointer routing, engine, overlays, render equivalence, window contour), `semio-ui-tui-unit-laws` 100 passed (paint and layout frame laws), `semio-ui-tui-frame` 8 passed, `semio-ui-tui-engine` 0 tests, 0 failures, no warnings from the TUI crates.

```
repo test-file -- ⌨️tui/🧪️tests/🖱️pointer-routing/🟦️.ts      17 pass, 0 fail
repo test-file -- ⌨️tui/🧪️tests/🎨️emphasis/🟦️.ts             3 pass, 2 fail (both in fixtures this lane did not touch)
```

The two failures of `🎨️emphasis/🟦️.ts` are not caused by this lane: Ajv2020 cannot resolve the draft-07 `$schema` of the shared schemas (`no schema with key or ref "http://json-schema.org/draft-07/schema#"`), and the `chrome-emphasis` fixture role expectation (`hover` equal to `active`) was untouched. The updated contour law test in that file passes.

## Open

- `Window` TS/React side, wgpu and the product plugins are other lanes.
- Children are laid out inside the safe clearance (the chip row is not scrolled under) because the TUI children are documents; an edgeless scene child would need `window_content_rect` instead of the padding, which no TUI consumer needs today.
- No schema file for the cell frame exists beyond `window-contour-v2`; the TS `WindowFrame` frame cases are not consumed by the TUI.
