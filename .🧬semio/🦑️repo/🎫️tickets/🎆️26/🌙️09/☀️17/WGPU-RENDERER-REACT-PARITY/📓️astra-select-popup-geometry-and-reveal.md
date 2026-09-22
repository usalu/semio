# Select Popup Geometry and Reveal Audit

Source and mounted-browser audit on 2026-09-21. This report precedes the WGPU repair. No native build or Cargo command was run by this executor.

## Mounted React authority

Root measured the real `Window Options` projection Select at a 16 px root size. The trigger is 104 px wide, but `SelectContent` is 128 px wide because its outer content carries `min-w-32`. At 1600×1000, the seven-option popup is 128×232.0625 px: a 1 px border, a 22.375 px up band, a 185.3125 px viewport, a 22.375 px down band, and the second 1 px border. The content position is 149.5 px plus the declared 4 px bottom transform, yielding top 153.5 px. The viewport begins at scrollTop 0 because the selected row already fits. Receipt: `🗑️generated/astra-runtime/checkpoint21-iab/react-normal-select-geometry.json`.

At 1600×300, both 22.375 px bands remain mounted and the viewport clamps to 142.5 px. Opening on the selected row calls `scrollIntoView({block: "nearest"})` and produces scrollTop 29. A wheel delta of 34 changes it to 42, the browser-clamped end. Receipts: `🗑️generated/astra-runtime/checkpoint21-iab/react-constrained-select-wheel.json` and `.png`.

The source matches the mounted result. `SelectContent` declares `min-w-32`, `overflow-hidden`, and a 1 px border at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:617-622`. It always mounts `SelectScrollUpButton`, the padded scrolling viewport, and `SelectScrollDownButton` at `:674-680`. The up/down bands are `size-tiny + 2×py-single` at `:778-820`. The open and keyboard movement paths call `scrollIntoView({block: "nearest"})` at `:529-546` and `:643-670`.

## Current WGPU divergence

The WGPU geometry already defines the correct row height and 22.4 px scroll-band height, but does not compose them into the popup box.

- `select_menu_height` returns rows plus viewport padding only at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:68-72`. For seven rows this is about 185.6 px, matching the React viewport alone rather than the 232.0625 px outer popup.
- `select_popup_geometry` copies the 104 px trigger width into `menu.w` at `:237-252`; it has no 128 px minimum.
- The same function creates up/down rectangles only when rows overflow at `:244-250`. React mounts both bands even when every option fits.
- `select_clamped_scroll`, `select_visible_rows`, and the row clip price the outer painted height as though it were the viewport at `:119-152`; once bands and border are present they must use the inner viewport height.
- The retained synchronize path reads only the existing scroll offset/direction and writes the resulting popup at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:1758-1775`. Opening or moving the highlighted row never asks for nearest-edge reveal.
- `EventRouter::set_select_highlight` changes only `highlighted` and dirties paint at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:2403-2410`. Therefore the selected row opens at offset 0, and Arrow/Page/typeahead can move the highlight outside the visible viewport.

This explains the physical WGPU popup: width 104, roughly 46 px shorter than React at normal height, conditional chevrons, and initial offset 0.

## Required contract

The language-neutral fixture must cover two mounted rows from the same seven-option Select:

1. Normal height: trigger width 104, popup minimum width 128, 1 px border, two always-present 22.375 px bands, viewport 185.3125 px, total 232.0625 px, initial scroll 0.
2. Constrained height: the same width/border/bands, viewport 142.5 px, initial selected-row reveal 29, wheel delta 34, browser-clamped result 42.

The WGPU implementation should derive its values from the existing theme tokens (`SIZE_TINY`, `padding_standard`, row line height) and a 1 px content border rather than hard-code browser rounding. Its native 16 px theme equivalents are 128 px minimum width, 22.4 px bands, and a natural height of 232.4 px for seven rows.

The geometry owner must expose a distinct viewport rectangle. Rows, hit clipping, visible-row counts, maximum scroll, chevron scrolling, and wheel scrolling must all use that viewport. Both chevron bands remain present even when max scroll is zero. Opening and keyboard navigation request a one-shot nearest-edge reveal; ordinary wheel/chevron movement must not be reset by an unchanged highlighted row on the following paint.

No compatibility path or additional unbounded state is needed. The existing bounded per-node Select state and existing two immediate-mode scroll slots remain the owners.

## Validation plan

- Neutral JSON Schema + fixture validate the two mounted React measurements.
- The React component law validates schema, structural bands/min-width token, and the existing selected-row `scrollIntoView`/wheel behavior using the third-party DOM harness.
- Native pure laws validate min width, border/band composition, distinct viewport geometry, clipping, and nearest reveal against theme-derived values.
- A retained native law opens on an offscreen selected row, then moves the highlighted row and proves the accepted row geometry becomes visible without changing selection. Existing wheel and max-scroll laws remain active to catch a reveal implementation that resets user scrolling.

Runtime acceptance remains a fresh WGPU browser measurement after the native and React gates pass.
