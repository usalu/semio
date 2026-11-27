# Execute footer covered by the docked chip row

## Root cause
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx` (the three top-anchored `Pane`s: Actions/engagement, Options/measures, Search) and `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📌️WindowPane/🟦️.tsx` (`positionStyle`, via `anchorPositionStyle` in `⚓️panel-anchor/🟦️.ts:44`). Each pane is an absolute overlay with `max-height: 100%` of the window body. The Utilities pane (bottom-left, `anchor="bottom-left"`) is a later sibling overlay, so its chip row paints over the bottom of a tall Actions pane. Longer labels (de-DE) wrap the staged form taller, which pushes the Execute/Reset footer under the chip row.

## Fix
- `WindowPane`: new prop `blockEndReserve` (CSS length kept free at the host's opposite block edge); when set, `max-height = calc(100% - (0px * 2) - <reserve>)`, so the pane body scrolls instead of extending under that chrome.
- `Window`: top-anchored panes (engagement, measures, search) pass `windowBottomChipReserve = var(--chip-row-height)`. Generic for every plugin and locale.

## Tests
- `🪟️Window/🧪️tests/🧩️component/🟦️.tsx` (existing pane-edges test) now asserts per pane `maxHeight` from `🧫️fixtures/pane-edges.json` (new `maxHeight` field): utility bar unchanged, the three top panes reserve the chip row, ltr/rtl, folded/unfolded. Vitest: 17/17 pass; typecheck of both changed sources passes (0 errors). Layout itself cannot be measured in jsdom; the journey `hit` evidence below covers it.

## Journey (strict)
`pdf-pane-journey.ts` `act`: always a real click (`clickUncovered`), no DOM-click fallback; returns `covered` plus `hit`; the ops verdict fails if any of insert-page, insert-rectangle, set-page-rotation reports `covered`.
- `run-round4-journey.sh execute-cover/r1 ops-de ops-en`: all 6 PASS; Execute `hit` de `31,910,72x22 top=self`, en `76,910,64x22 top=self` (was y=934, covered, in de).
- `execute-cover/r2 a11y-de details-de`: all PASS (no regression).
Logs/evidence under `🗑️generated/execute-cover/`.

## Limitations
If the Utilities pane is expanded at the same time as a tall Actions pane they can still overlap (only the folded chip row height is reserved).
