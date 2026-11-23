# Print Lane Handoff

Lane `print`: windows, figures, tables, contents, headings, navbar and footer in
`🧰️framework/🛍️products/📓️print` (`PRINT` below) plus the pets story `.card .tab`.

## Model as implemented

- Every component is one rectangle. The frame is drawn by the box itself (tcolorbox skin rules, the long table
  chrome band, the heading frame row, the page-bar rule), and its top rule is continuous.
- Chips are inside the frame, inset by one hairline, anchored to the top (windows, tables, headings) or to the
  shared rule (navbar chips sit on the bottom rule, footer chips hang from the top rule). A chip draws only its
  inner edges: window title `rb`, window number `lb`, heading title `r`, heading number `l`, navbar `tlr`,
  footer `blr`.
- Chip surface is `semio-chrome-window` (one step above the `semio-chrome-canvas` content surface) at
  `\semio@chip@opacity` (0.82) through `pgfscope` fill opacity. One switch, `\SemioChipTransparency{false}`
  (`\ifsemio@chip@glass`), fills the opaque chip surface instead, for PDF/A-1. Only `\semio@chip@fill` reads it.
- Windows: chips are tcolorbox `finish unbroken|first|middle|last` layers (painted after content, so they stay
  above images). Every page fragment has its own chips; the whole-keep split start still runs and no longer emits a
  header. Continuations draw their own top stroke (`overlay middle|last`), the first fragment uses the skin top
  rule. Document windows get `top = bodypad + chip height`, tables `top = chip height`, `image-fit=cover` windows
  stay full-bleed (chips run over the image). Image and text-fit target heights account for the top rule and the
  clearance.
- Long tables (contents, registers, `\SemioTableLong`): the chrome row is the top band (top rule, side strokes,
  title chip); the live page-part number chip is overlaid inside that band after the stored head is copied.
- Headings: `\semio@heading@frame` is one full-width rectangle in the ladder colour with title and number chips
  inside; solid levels fill the chip with the ladder colour at the glass opacity, outline levels fill the chip
  surface.

## Removed (no leftovers)

`\semio@window@cap*` family (raise, open-bottom, accent/muted vboxes, left overlay, metrics, slot, min titleh),
`\semio@window@tab|ctrl`, `\semio@heading@cap*`, `\semio@heading@chip(@open)`, `\semio@heading@pair*`,
`\semio@heading@title@end*`, `\semio@window@header@gap@open`, `\semio@window@header@muted*`, baseline corner
stubs, `\semio@window@header@row@wrap`, flush/baseline switches, `\semio_window_vskip_stroke_hairline:` with the
`-5.75pt` seam, `\semio_window_header_muted_use:`, whole-keep header defer and emit, `\semio_window_header_overlay_baseline:`,
`\semio_window_column_tab_floor_set:`, `\semio@chrome@chip@rule|body`, table phantom chrome borders,
`\semio@chrome@titlebar@height` (also its token generator line and `semio-tokens.sty`), `\semio@heading@number@suffix`.
`\semio@window@header@number@page1` became `\semio@window@header@number@first` (the digit-delimited macro name was
fragile). `\semio@tab@inset(@v)` became `\semio@chip@inset(@v)`. The assignment fixture key `silhouette` became `frame`.

## Files changed

PRINT:
- `🖋️latex/semio-window.sty`, `🖋️latex/semio-table.sty`, `🖋️latex/semio-tokens.sty`
- `🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts` (titlebar token line removed)
- `🧪️tests/🏷️page-chrome/🟦️.ts`, `🔣️.json`, `🥒️.feature` (chip geometry, edges, glass switch, removed macros)
- `🧪️tests/📝️assignment/🟦️.ts`, `🔣️.json`, `🥒️.feature` (`silhouette` to `frame`)
- `🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts` (source assertions for the new hooks)
- `README.md` (English and German section on frames and chips)

Pets: `🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css` (`.card .tab` is now an inside top-right overlay chip, inner edges
only, translucent `color-mix` fill with backdrop blur, opaque fallback for reduced transparency and forced colours).
Not browser-checked.

Ticket folder: `evidence/*.png`, `print-lane-build.sh`, `print-lane-pdf2png.swift`,
`print-lane-run-tests.ts`, `print-lane-pipeline-asserts.ts`.

## Verified

`repo` was too heavy for this (graph build, capacity queue), so the native fallbacks were used.

- Compiled with the repository's pinned tectonic 0.16.9 and its cached bundle and fonts (the command in
  `print-lane-build.sh`, same flags as the compiler module). Documents: report template (light and dark), a custom
  stress document (breakable window across 2 pages, `\SemioTableLong` across 3 pages), thesis, paper, flyer and a
  minimal assignment. All compile with zero TeX errors.
- PNG inspection (PDFKit via `print-lane-pdf2png.swift`), in `evidence/`:
  - before: `before-p3-heading-window.png`, `before-p5-figure-photo-blockquote.png`, `before-p2-contents-register.png`
  - after: `after-p3-heading-window.png` (heading, window, navbar/footer), `after-p5-figure-photo-blockquote.png`
    (figure, photo over cover image with glass chips, blockquote), `after-p2-contents-register.png` (contents long table),
    `after-split-window-continuation.png` (continued window, chips and top rule on the fragment),
    `after-long-table-continuation.png` (continued long table, live part number), `after-p5-dark.png`.
  - Translucency confirmed on the cover photo (image visible through the chips). A separate probe showed 50 percent
    blue over red rendering purple through xdvipdfmx.
- `bun print-lane-run-tests.ts`: `verifyPageChromeBorders` and `verifyPageChromeLogo` pass (mathjs arithmetic,
  Ajv edge schema, source assertions that removed macros are gone and every `finish` hook carries the chips).
- `bun print-lane-pipeline-asserts.ts`: the 22 source assertions copied from the quick pipeline verification pass.

## Open

- The full `repo test` quick pipeline run and `test-assignment` were not run through `repo` (infrastructure queue);
  only the extracted assertions above ran. Run `repo run @semio-tech/print:test-assignment` and the quick level once.
- The remaining gallery and `viz-api` documents were not recompiled; they only use `\SemioTableLong`, which compiled.
- Which build profile uses PDF/A-1 is not defined in this repo; the switch exists but nothing calls it.
- Chip fill is 0.82 opacity of `semio-chrome-window` over the canvas; it is a subtle step. Raise the surface to
  `semio-chrome-pane` in `\semio@chip@surface` if more contrast is wanted (one line).
- Navbar and footer chips keep their previous side-stroke layout (inner edges plus sides, none on the rule); the bar
  has no outline of its own besides the rule.
- `semio-viz-*` silhouette chart kinds were not touched.
