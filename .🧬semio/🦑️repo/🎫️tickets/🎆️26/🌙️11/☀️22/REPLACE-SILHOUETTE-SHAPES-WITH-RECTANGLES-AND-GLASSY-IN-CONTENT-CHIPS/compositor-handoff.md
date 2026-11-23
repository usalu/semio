# Lane `compositor` handoff

The React compositor paints the rectangular frame model. Verified with jsdom tests and the styling tests; not looked at in a browser (see "Open").

## Final DOM of a `WindowChrome` stack

```
div[data-slot=stackSlot "window-chrome-stack"][data-window-frame][data-frame-state=pending|ready][data-level][data-active]
    class: relative flex min-h-0 min-w-0 flex-col overflow-clip bg-transparent   (frame rectangle = the only clip)
    style: --window-frame-top-clearance, --window-frame-bottom-clearance (px, = chip row depth per edge)
  svg|div[data-slot=frameBorderSlot "window-chrome-frame-border"][data-window-frame-border][data-kind]   absolute inset-0 z-40, pointer-events none
  div[data-slot="window-chrome-body-surface"][data-level=content level]                                   absolute inset-0 z-0, glass of the content level
  div[data-slot=bodySlot "window-chrome-body"][data-window-frame-content][data-frame-state][data-level=content level]
      class: window-frame-content-plane relative min-h-0 flex-1   (fills the frame; padding-block = clearances)
  div[data-slot=capSlot "window-chrome-cap"][data-dock=capDock]                                            absolute inset-x-0 top-0|bottom-0, pointer-events none, z 2
    div[data-slot=chipSlot "window-chrome-chip-cap"][data-window-frame-chip][data-dock][data-chip-reach=next][data-level=chip level]   glass, pointer-events auto
    div[data-slot=controlsSlot "window-chrome-controls"][data-window-frame-chip][data-dock][data-level=chip level]                    ms-auto
  div[data-slot="window-chrome-footer"][data-dock=bottom]                                                  absolute inset-x-0 bottom-0, pointer-events none, z 2
    div[data-slot="window-chrome-footer-left"|"-center"|"-right"][data-window-frame-chip][data-dock=bottom][data-level=chip level]
```

- chip level = `windowFrameChipLevel(content level)`, one entry up in `LEVELS` (topmost stays); the content level is `bodySurfaceLevel ?? level ?? useLevel()`.
- The cap row always renders (empty rows have zero height); the footer row only when a footer chip exists. With a footer centre chip the footer is a 3 column grid with chips on `col-start-1|2|3`, otherwise a `justify-between` flex row.
- No gap elements, no `*-gap` slots, no wrappers around footer chips, no clip-path, no negative margins.
- `chipOnly`: the stack is `inline-flex` and holds the border overlay plus the one in-flow chip (`data-window-frame-chip`, `data-dock`, `data-level`); there is no content plane.
- Nested frames: chips whose closest `[data-window-frame]` is another frame are ignored by the outer measurement.
- Chip abutting (`data-chip-reach`, `data-chip-abut`) is kept only for chips in one row: a marked chip gets `flex: 1 1 auto; justify-self: stretch` and takes the free row space.

## What changed

Renames follow design.md: `WindowChromeFrameBorder`, `WINDOW_FRAME_BORDER_KINDS`, `WindowFrameBorderKind`, `resolveWindowFrameBorderKind`, `windowFrameBorderPaint`, `measureWindowFrameMetrics`, `useWindowFrameGeometry`, prop `frameBorderSlot`. Removed: `gapProps`, `footerGapProps`, `silhouetteSlot`, `windowGapFrameClass`, context menu fusion measurement, gap selectors and gap measurement, the polygon content clip. Added: `windowFrameChipLevel` (exported from `🗂️WindowChrome/🟦️.tsx`), `windowFrameOutline` and `windowFrameLines` (exported from `shared-border.ts`).

Files (all under `🧰️framework/🔨️modules/🖱️ui`):

- `🧱️elements/🗂️WindowChrome/🟦️.tsx` (rewritten: measurement, geometry hook, border overlay, `WindowChrome`)
- `🧱️elements/🗂️WindowChrome/shared-border.ts` (window entry registers `sharedOutlineSegments(..., regions)`: frame rectangle plus chip inner edges in the normal state; stated chips snap onto frame and chip inner lines; the degenerate-outline branch and `chipOutput` are gone; the single `borderPath` fast path applies only when the frame has no chip region; `hoverWithin` reads `data-shared-border-hover="frame"` or `"declared"`)
- `🧱️elements/🗂️WindowChrome/chip-abut.ts`, `chip-abut.test.ts`, `focus-seam.cases.json`, `compositor.cases.json`, `shared-border.test.ts`, plus new `frame-structure.test.tsx`
- `🎨️styling/🖌️ui/🎨️.css` (`.window-frame-*`, `data-window-frame*`, gap rules deleted, plane negative margins deleted, chip row and chip pointer rules, glass fallbacks without gap rules), `🌐️globals/🎨️.css`
- `🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts`, `🎨️styling/🧪️tests/🧫️fixtures/🔁️animation-scope/🔣️.json`
- `🔨️modules/🎨️chrome-class-names/🟦️.ts` (comments only)
- `🧱️elements/🌈️Surface/🟦️.tsx`: already changed by another lane when I got there (gap hit test and `isSurfaceActiveBackgroundPointer` gone), I did not touch it.
- `🎨️styling/🏗️builder/🌐️vite/🪝️playground-stubs/🟦️.ts`: untouched, its "silhouette" is the minimap meaning.

CSS: the chip glass reuses `ui-glass` and the `[data-level]` token formula by stamping the chip level on the chip itself, so the existing reduced-transparency, no-backdrop-filter and forced-colors fallbacks give the opaque chip surface. The content plane keeps `padding-block` from the clearance variables and gets `scroll-padding-block`; the edgeless and dead-line opt-out (full bleed) is unchanged. Chip rows are `pointer-events: none`, chips `auto`.

## Verified

Native fallback (`repo` has no `ui-react` project on this machine; the whole graph read does not complete, so `repo run/test` could not be used). Scripts in this folder:

- `./compositor-vitest.sh` with `COMPOSITOR_CONFIG=./compositor-vitest.config.ts`: vitest on all tests of the WindowChrome folder: 6 files, 111 tests, all pass (compositor, chip abut, focus seam/boxes, state boxes, new frame structure and chip inner edge tests).
- `./compositor-vitest.sh WindowChrome/shared-border` with the stock `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (the config the suite uses): passes.
- `repo test-file -- 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts`: 14 pass, 2 fail. Both failures exist at HEAD and are not from this lane: the `panel-tabs` divider rule (no such rule in the CSS) and the "chips borderless" test (it expects `[chip], [chip] > * { ... !important }`, the CSS only has `[chip] { border-width: 0 }`).
- `repo test-file` for `🔁️animation-scope/🟦️.ts`: 3 pass.
- `tsc --noEmit` over `🟦️.tsx`, `shared-border.ts`, `chip-abut.ts`, `frame-structure.test.tsx`: no errors in the lane files.

Third-party cross-check: the chip region and border path geometry is checked against `frame.cases.json` by lane `contract`; the lane tests assert the compositor edges against the literal rectangle and chip edge coordinates of those cases. No independent rectangle library is installed, so no third-party diff was added.

## Open

- Browser check not done: storybook/preview is not cheap here and the other lanes are still moving files. Needs a look at one window chrome story (chip glass over content, scrolling under the chips, forced colours).
- `data-shared-border-hover` value `"silhouette"` became `"frame"`; OverviewCard already sets `"frame"`.
- Footer row with `capDock="bottom"` plus footer chips overlap (both anchor to the bottom); no consumer does this today.
- Card stack/grid CSS still lifts and min-heights the cap/footer rows (`--card-stack-cap|footer`); under overlay rows this needs the elements lane to decide the card interlock model.
- `modeDockInactiveTabBeforeGapClass` in `chrome-class-names` keeps its name (used by the Canvas dock, lane `elements`); its comment now speaks of free row space.
- The `ui-react` vitest include list does not list the new `frame-structure.test.tsx` or the other WindowChrome tests (lane `uitests` owns the config).
