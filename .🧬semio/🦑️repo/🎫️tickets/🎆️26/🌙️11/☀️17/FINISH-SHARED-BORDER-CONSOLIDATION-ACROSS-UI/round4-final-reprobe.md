# Round 4 final independent re-probe (live, in-app browser)

Method: own tab, pane shown and tab fronted before every probe, only `pending:false` readings counted, `border-gap-probe.js` base probe without filters (count = number of violating pairs). Own native vite servers on 5601-5609 (fresh caches `.cache/b/p/m/r4f-*`, all killed at the end, `r4f-start.sh`). Runners: `round3-os-run.js`, `round3-elements-live.tsx` sweep, `round3-cad-live.tsx`, `lv-final-staffage.js`, `lv-final-demo.js`, `lv-final-play.js`, `lv-final-hub.js`, `round2-presentation-sweep.js`, `lv-final-course.js` (lean course crawl: no focus sweep, dialog probed as own subtree).

## Result table (violating pairs; 0 = gate passed)

| App | View / scope | 1440x900 | 768x1024 | 375x812 |
|---|---|---|---|---|
| OS harness | all 72 parts (bands, dialogs, popovers, rich windows, settings, staged-action dialog en+de) | 0 | 0 on 62 parts; 10 parts non-zero, all class A/B/C (below) | 0 on 69 parts; `layout`, `layout-bottom`, `layout-many` 1/2/1, class D |
| CAD harness | aside, attribute pane, inference panel, toggled menu | 0 | 0 | 0 |
| Elements | all 126 stories | 0 | 0 | 0 |
| Staffage | overview, all, animals, collections (+bottom), compare, playground | 0 (overview was 4, fixed) | 0 | 0 |
| Staffage | asset pages walking-commuter, domestic-cat, cafe-chair, circulation-pump: Three views, 3D model (stage ready, 3D toolbar) | 0 | 0 | 0 |
| Demonstrator | introduction, overview, opened pane, back | 0 | 0 (opened pane was 3, fixed) | 0 (back was 1, fixed) |
| semio-tech play | introduction, landing, opened card, back | 0 | 1-2 OPEN (landing, 59 px cards) | 0 (back was 2, fixed) |
| Presentation | 34 slides: rest, every disposition kind selected (incl. pdf), enlarge | 0 (112 readings) | 0 (112) | 0 (110) |
| Hub admin | 7 tabs, spaces expanded, create dialog | 0 | 0 | 0 |
| Course en | 16 home/quiz views + nine non-physics quizzes x easy/hard/expert (all steps filled, submit dialog, results) | 0 (241 readings) | 0 (32 readings: all home views, physics + demand medium with dialog and results) | 3, all class G (247 readings) |
| Course de | home views, quizzes, medium run with submit dialog and results (physics, ventilation) | not requested | 0 (34 readings) | 0 (25 readings) |

## Real defects found and fixed in this pass (each with a test, re-probed 0)

1. Staffage overview at 1440: 4 pairs (1.2-1.28 px) between the card rows of the layered overview. Root cause in the shared `useCardInterlock`: (a) sequential snapping made every card height depend on the order of the writes (the grid rows are `fr` tracks and react to each `min-block-size`), so the measured drift between the hero and the row below was 1.28 px, above the 1 px reach that detects the card above, and the whole row got lift 0; (b) a lift is a negative margin that moves the tracks again, so the drift measured at lift 0 is not the drift after the lift. Fix: all window and card heights are snapped in two batched phases, the reach is `1 + 2 * inset`, and after the lifts are written the seams are re-read and corrected (secant steps, max 4 passes) until every overlap equals the band. Files: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️CardStack/🟦️.tsx`; tests `🃏️CardStack/🟦️.test.tsx` (mocks now respond to lifts, new settle test), CardStack 31/31, OverviewCard 6/6, LayeredOverview green.
2. Demonstrator opened pane (OS shell footer) at 768: 3 pairs, footer chips 1 px above the footer line. Root cause: at tablet `--ui-spacing` is the touch spacing, so `--chip-row-height` (31 px) exceeds the theme-owned `--footer-band-height` (29 px) and the footer's chip stacks (31 px) stick out 1 px above and below the band. Fix: inside `[data-slot="navbar"]` and `[data-slot="footer"]` the chip row is exactly the band. Files: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`; test `🎨️styling/🧪️tests/🧩️suite/📐️geometry/🟦️.ts` (bun, new test passes; one older test of that file about `MODE_CANVAS_INSET_CLASS` was already failing, unrelated). Companion: the shell footer shows the credits as logos on a tablet (`shellFooterCreditsCompact`, `🛍️products/💻️os/.../🏛️ShellFooter/🟦️.tsx`, `🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx`, test `🏛️ShellFooter/🟦️.test.tsx` 2/2). That alone does not make the footer fit (see Open 2).
3. Demonstrator and play at 375, back from an opened card: 1 and 2 pairs, the revealed card (`lifted`) sat 2 px above the shared line with the swipe hints. Root cause: the hover/reveal lift was only neutralised in card stacks and grids. Fix: also inside `[data-layered-overview]`. Files: `🎨️styling/🖌️ui/🎨️.css`; test `🃏️OverviewCard/🧪️tests/🧩️component/🟦️.tsx` (6/6).
4. Fractional grid columns (play landing at 768, 13 columns of 59.08 px): the compositor resolves lines in device-snapped space but the overlay svgs sat at the unsnapped element corner, so neighbours at different fractions painted one seam 0.08 px apart (1.08 px pair). Fix: `sizeSharedBorderOverlay` moves the overlay onto the snapped corner of its parent (`translate`). Files: `🗂️WindowChrome/shared-border.ts`; test `shared-border.test.ts` (19/19). Effect on play 768: 1.08 px became 1.00 px, not yet 0 (Open 1).
Regression checks after these changes: elements 126 stories at all three viewports, OS 72 parts at 768, staffage, demonstrator, play, presentation, hub admin, course 1440/768/375: counts as in the table.

## Open (real, not fixed)

1. Play landing at 768 (`lv-final-play.js`): 1-2 pairs of 1.0 px between the vertical seams of horizontally adjacent 59 px cards (Math | Wires | DAG). Diagnosed: the card "Wires" has a degenerate silhouette (its title chip spans the whole 59 px card, so `geometry.outline` and `windowSilhouetteOutline(metrics)` differ in length), so `BorderCompositor.commit` paints its default `borderPath` (left line at x+1) while the neighbour merges the shared seam onto the centerline (x). Needs a compositor decision (let a degenerate window take part with the simplified outline) with new cases in `compositor.cases.json`; not done.
2. OS shell footer at 768 with branded credits: items add up to 972 px in a 768 px footer (the shell's own tab bars, local-only chip, credits logos); chips beyond the right edge are clipped. The pair is gone (fix 2) but the overflow is a layout defect (the credits need to drop or the footer to wrap on a tablet).
3. Designed U-gap kept, needs an owner decision (not a false positive under the radical rule): OS 768 `window-rich-many`, 39.79 px between the Search chip and the Window Options chip of one window cap row (the cap row of a window deliberately has an open gap between its chips, the window outline is one continuous line below).
4. Focus rings of course controls (informational near probe): the primary dialog action carries a 2 px inset accent ring (`box-shadow`), and a just-filled course select an `outline`, both 0-3 px from the card/chip lines. Not borders, base probe 0, but they are not the "seam recolours instead of a ring" the brief prescribes.

## False positives, by class

- A. OS 768 `settings-general|theme|keys|defaults|conflicts`, `display`, `marketplace`, `panel-bottom` (39 px pairs): the floating top-right panel (420 px) lies over the 50 % split of two windows; its left edge is 39 px from the title chip of window b/a. Two layers (floating panel over the canvas), the distance is where the harness happens to place the panel, just under the probe's 40 px reach. At 1440 the same parts are 0 (far apart). round4-course-live.md FP 4.
- B. OS 768 `layout-bottom` (12 px): the bottom-right panel over the canvas windows, same overlay class (FP 5 there).
- C. OS 768 `settings-keys` overlaps (info): keybinding label cell under its shortcut cell (unbordered text boxes).
- D. OS 375 `layout`, `layout-bottom`, `layout-many`: the desktop `Layout` forced into a phone viewport by the harness (`ShellLayout` has no `mobile` prop); the app passes `mobile` below 768 and the real phone parts `layout-mobile*` are 0.
- E. Elements `LayeredOverview/*`: informational overlaps (unbordered paragraph box under the Open chip), count 0; `ReducedMotion` sometimes stays `pending` (running animation), count 0 on every settled re-read.
- F. Course near probe (informational, base 0): focus rings, see Open 4; submit dialog footer ring (`shadow-inset`) vs card line at 0-1.03 px.
- G. Course 375 base `nav[slot=navbar] || select.course-target css:top` at 31-35 px and 1.92 px (`grasshopper-data/hard step4`, `excel-workbook/hard step3`, `bim-concepts/hard step3`): a filled select that the crawler scrolled under the navbar, a border of scrolled content near the sticky navbar at one scroll offset (class 1 of round4-course-live.md); the scroller content moves, the layout does not.
- H. Elements/OS informational overlaps (`ov`) are not counted anywhere.

## Tooling left in the ticket (all temp output stays in `🗑️generated`)
`r4f-start.sh`, `lv-final-staffage.js`, `lv-final-demo.js`, `lv-final-play.js`, `lv-final-hub.js`, `lv-final-course.js`, `lv-vitest-footer.config.ts`.
