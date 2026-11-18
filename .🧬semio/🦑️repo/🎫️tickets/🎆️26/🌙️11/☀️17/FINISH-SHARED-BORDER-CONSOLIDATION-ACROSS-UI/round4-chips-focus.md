# Round 4: cap-row chip gap rule and seam-recolour focus

Resolves Open 3 and Open 4 of `round4-final-reprobe.md`.

## 1. Chips closer than CARD_BAND_MIN_GAP abut

Rule (one constant, `CARD_BAND_MIN_GAP` = 40 px in `🃏️CardStack/bands.ts` and `bands.rs`): a gap between two chips of one row that is larger than 0.25 px and smaller than 40 px is a sliver. Slivers are closed (the chips abut and share one seam); gaps of 40 px or more stay open; abutting or overlapping chips are untouched.

- Schema first: `bands.schema.json` / `bands.cases.json` gained `gaps` (`{gap, sliver}`), new `chipGapReadsAsSliver` (TS) and `chip_gap_reads_as_sliver` (Rust); the card-band `touch` now reuses it. Tests: `bands.test.ts`, `bands.test.rs` (language-neutral vectors).
- React: new `🗂️WindowChrome/chip-abut.ts` (`resolveChipRowAbuts` pure, `settleChipRowAbuts`/`observeChipRowAbuts` DOM) with `chip-abut.schema.json`, `chip-abut.cases.json`, `chip-abut.test.ts` (pure cases + a painted-row test with mocked rects, 39.79 px vs 40 px). `WindowChrome` observes its cap row and footer row; chips declare `data-chip-reach` (title chip and footer-left `next`, footer-right in the centred grid `prev`). A sliver marks the reaching chip `data-chip-abut`; CSS in `🎨️styling/🖌️ui/🎨️.css` stretches it over the gap (`flex: 1 1 auto`, the gap collapses, grid footer wrapper stretches). Decisions use natural widths, so a stretched chip reopens the gap as soon as 40 px are free (no flapping). The silhouette merges abutting chip spans, so the pair shares one line.
- Native TUI: `🖥️chrome/🦀️.rs` `chip_gap_cells_read_as_sliver` (1 to 4 cells = under 40 px at `CELL_WIDTH_PX` 8) and `abut_chip_sliver`, applied in `window_chip_layout` (top and bottom row, left group to right group/controls) and `compact_layout`. Test `window_cap_row_chips_abut_instead_of_leaving_a_sliver_gap` (`⌨️tui/🧪️tests/🔬️unit/🪟️tui-layout-and-window-laws`): cross-checks the cell rule against `bands.cases.json` (include_str) and sweeps window widths 14..90 for no sliver while at least one width abuts. `serde_json` added as dev-dependency of `semio-ui-tui-unit-laws`.
- Native wgpu card stacks already stack chips by `resolve_card_bands`; unchanged. The OS native dock tab bar (`🛰️Dock/.../🔲️window-silhouette-and-stack-geometry`) lays out its chips elsewhere and was not changed (see Open).

## 2. Focus is the seam recolour, not a ring

- Generic rule in `@layer shared-border-priority` (`🎨️styling/🖌️ui/🎨️.css`): a shared-border cell/frame that is itself `:focus-visible` (or a frame wrapping a focused input/textarea/select-trigger) drops `outline` and `box-shadow` (`!important`, beats utilities and product CSS) and a frame recolours its own border to `--active-base` (forced colours: `Highlight`, focus segments `Highlight`). Cells keep the existing seam recolour (`:focus-within` and `:has(+ ... :focus-within)`); frame edges keep the compositor's `focus` segments. `segments.test.ts` now forbids any non-`none` box-shadow on cell rules (was: any box-shadow).
- Window chips (the primary dialog action was `[data-window-silhouette-chip] > :focus-visible { box-shadow: inset 2px }`): the inset ring is gone. New `🗂️WindowChrome/chip-focus.ts` (`focusedWindowChipPath`) outlines the chip that holds keyboard focus on the silhouette centreline; `WindowChromeSilhouetteBorder` paints it in the focus segment state (`--active-base`) over its own line, so the chip's seams and the window edge it reaches recolour. Only `:focus-visible` counts (pointer focus draws nothing). Tests: `chip-focus.test.ts`, updated `🪟️Window/.../🟦️.tsx` "Window chrome focus indicator".
- Removed ring/outline focus styles: `chrome-class-names` and `chrome-control-presentation` (`focus-visible:ring-1 ring-inset`, panel tab buttons and pane toggles), `Tabs` trigger (now `focus-visible:border-active-base`), `OverviewCard` whole-card button (`ring-2 ring-offset-2`; CSS recolours the card's silhouette stroke on `:focus-visible`), course `.course-input`/`select` (outline, now border recolour), `.course-cell` (the `select.course-target`: seam recolour of its divider and of the next cell's), `.course-card-link` outline (chip path recolours); forced-colours variants use `Highlight`.
- Language-neutral guard: `🗂️WindowChrome/focus-seam.cases.json` + `focus-seam.schema.json` + `focus-seam.test.ts` (files must contain the recolour rules and must not contain the ring classes).

## Tests
- `vitest-ui-all`: all new/changed pass (bands, chip-abut, chip-focus, focus-seam, Window, segments except a Playwright browser launch that needs `npx playwright install`; unrelated failures in the same run: Tree text-flow, Layout panel overlay, WebGPU readback, bun:test files and others, none touch these files).
- `vitest-course`: 1135/1135. `vitest-os-final`: 158/158.
- Rust: `semio-ui-tui-unit-laws` (new test passes), `bands.test.rs` via the wgpu-light crate (see end of file for the final run).

## Open / not done
- Elements not changed on purpose: `Scrollable` focus ring (focusable scroll region, not a bordered cell), `Slider` thumb, `Resizable` handle (0 px separators), dialog close button ring (`ring-offset`, in a Dialog without shared frame), Diagram keyboard outline (canvas content), `window-measures-tree` select trigger (accent border, no ring). The course forced-colours `.course-app :focus-visible` outline remains for unbordered controls.
- Native OS dock tab bar chips (wgpu `🛰️Dock`) have no sliver rule yet.
- The contrast of `--active-base` against the surface was not re-measured here.

## Live re-probe needed (keyboard focus-visible, then base probe count 0 at 1440 and 375)
1. OS `window-rich-many` at 768 (and 1440): Search chip / Window Options chip: gap now 0 or at least 40 px; silhouette continuous; also resize the window across the 40 px threshold.
2. OS windows with footer chips (`window-chrome-footer`, left/right and centred grid): sliver closes without clipping.
3. Course: submit dialog (primary `CardAction`), `select.course-target` in rows (grasshopper-data/hard step4, excel-workbook/hard step3, bim-concepts/hard step3), `.course-input` guess field with unit select, `.course-card-link` on the overview: Tab focus shows seam/chip recolour, no ring.
4. Elements: `LayeredOverview/*` card buttons (Tab focus recolours the card outline), `Tabs`, panel tab bars (`chrome-class-names`), window pane chrome toggles.
5. Demonstrator opened pane and Play landing at 768 for the chip rows (no regression in the footer 768 pairs of Open 1).
6. Forced-colours emulation on one course view and one OS window (Highlight on focus).

## Final Rust runs

- `semio-ui-tui-unit-laws`: 100 passed, 0 failed (incl. `window_cap_row_chips_abut_instead_of_leaving_a_sliver_gap`).
- `semio-ui-wgpu-light` `card_`: 7/7 (incl. `chip_gap_language_neutral_vectors`).
