# Round 2 verification fixes

Probe `border-gap-probe.js` (`?t=`, `pending:false`) in own tab `tab-26`; servers 5392 (staffage), 5396 (play harness), 5395 (hub admin) started and killed by this run. All results below: `count` 0.

| Defect | View | 1440x900 | 375x812 |
|---|---|---|---|
| 1 Staffage playground figure rows / markers / 1px edge | `#/playground` (4 placements) | 0 (was 7) | 0 |
| 2 Play landing: CAD chip vs navbar line, card vs Open chip edge | swipe, intro skipped | - | 0, hint/card lines now straight (see below) |
| 2 Play landing: 26px Eurocode 2 / Math pair | tiled grid, pointer away and pointer on a card | 0 / 0, `translate: none` on the hovered card | - |
| 3 Hub admin tab bar and spaces table | overview, users, observability, spaces (+ expanded members) | 0 | 0, document width 375 |

## 1. Staffage playground (`🧪️playground/🎞️timeline/🖼️panel/🟦️.tsx`)
The window bar and the length handle were `absolute top-1 h-6` boxes inside the `h-8` track cell: 5 px gaps to the row's `border-t` above and below. Both are now `absolute inset-y-0`, i.e. they fill the row cell, so their ends are the row lines. The bar draws only `border-l` (its right end is the filled 8 px length handle); a right border at the track end sat 1 px beside the column's own right border, which was the 1 px edge. The selected state is a stronger fill (`bg-[var(--accent)]/55`) instead of an offset outline.

## 2. Play landing
Cause of the 375 near-double and the 1-2 px edge mismatch: not play CSS. The swipe hints are compositor frames next to a silhouette card. Where the card's cap/footer gap leaves the hint's edge unshared, the compositor painted that piece on the hint's own 1 px inset (y 27) while the shared pieces sat on the common centerline (y 28): a 1 px jog at the chip corner (visible as the notch box around the title gap and the footer gap, left edge 1.5 px beyond the card). Measured path before: `M 85.5 28 L 85.5 27 L 348 27 L 348 28`; after: `M 85.5 28 L 348 28` (and the footer hint `M 27 0 L 279.5 0`).
- Fix (framework, generic): `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`, `BorderCompositor.commit`: an unshared piece of a logical edge follows the centerline of the shared pieces of the same edge when that centerline lies on the logical box line (`lineOf`). Edges whose shared line is offset from the box line (unequally scaled overlaps) keep the old behaviour; the existing test for that still passes.
- Test: new `interlock` case "a bar over a card with a title chip keeps one straight line across the chip and the notch" in `compositor.cases.json`, plus a straightness assertion (all painted spans of a line share one normal) in `shared-border.test.ts`.
- The 26 px pair at 1440 was the hover lift. Fix (generic): `🖌️ui/🎨️.css`, `:is([data-card-stack], [data-card-grid]) [data-overview-card-paint] { translate: none !important; transform: none !important }`, so no tiled card lifts (emphasis stays colour/outline). Swipe cards (not in a grid cell) keep `lifted`. Test in `🃏️OverviewCard/🧪️tests/🧩️component`.

## 3. Hub admin (`🌎️hub/🔨️modules/🛡️admin/🧱️elements`)
- `🛡️AdminApp`: the tab navbar item is `shrink overflow-x-auto` with the scrollbar hidden: the tabs scroll inside their cell (0..279) and the locale select stays flush at 279..375; no tab clips at the viewport edge, the cell edge is the divider.
- `🏛️SpacesPage`: below the mobile media query the Table renders `renderMobileRow` rows: per space a vertical `SharedBorderRow` (name cell as button, kind/visibility/members/documents cells, the four actions as a vertical group), `data-row-id` kept. Action buttons are shared by both layouts (`actionButtons`). No table, no horizontal overflow; expanded members panel probed too.
- Tests (`🧪️tests/🛡️admin`): tab bar scrolls inside its cell; narrow spaces render stacked rows. 27/27.

## Test runs
- Hub admin (`vitest-hub-admin.config.ts`): 27/27.
- Staffage teaching suite (`vitest-teaching.config.ts`, package dir): 29 files, 271/271.
- Play suite (`🏢️semio-tech/🎡️play/🧪️tests/🎚️config/🟦️.ts`): 8 files, 81/81.
- UI (`vitest-ui.config.ts`): WindowChrome, OverviewCard, CardStack, LayeredOverview, WindowSilhouette: 117/117 (OverviewCard incl.). Full 🖱️ui regression set not rerun beyond these.

## Not done / notes
- No staffage unit test for the timeline bar change (visual/probe verified only).
- Screenshots were taken per fixed view during the run (play 375 swipe and playground 1440/375, hub spaces 375); not saved as files.
- Hub spaces at 375 uses the stacked rows only for Spaces; other admin tables were already 0 and fit.
