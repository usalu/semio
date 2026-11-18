# Round 2 follow-up — framework card layout

Servers started natively and killed afterwards (staffage 5392, demonstrator 5393, course 5396 with `vite-preview.config.ts` because the shared 5391 server hung at 90 % CPU, play 5397 via `vite-play-frozen.config.ts`). Own browser tab, closed. Probe: `followup-probe.js` (shared `border-gap-probe.js` + wait for the compositor) and `round2-probe.js` for the course.

## Probe pitfall (explains the "132 pairs, 2 px apart" in the presentation/hub/site report)

The shared-border compositor commits in `requestAnimationFrame`. In a Browser-pane tab that is not fronted (or while the pane is hidden) the frame never fires, `data-shared-border-pending` stays set and every window still paints its own unmerged silhouette inset 1 px: adjacent cards then look like two lines 2 px apart. That was not a silhouette/stroke-inset bug. Always `tabs_select` the own tab before probing and check `pending:false` in the result (`followup-probe.js` reports it). With the tab fronted the play grid had 2 real pairs left (below), now 0.

## Fixes (all in `🧰️framework/🔨️modules/🖱️ui` unless noted)

1. **Swipe mode hints** (`🧱️elements/🥞️LayeredOverview/💡️hints`): the neighbour hint was a small button centred in its 28 px band, so the rest of the band (and the corners) was an empty gap between navbar/footer/edge and the card (navbar||card d=30). The hints are now bars filling their whole band: up/down span the full width, left/right span the height between the up/down bars; every band is bordered by the compositor frame and flush with navbar/footer/view edge and the card. Generic, fixes course, staffage, play, demonstrator.
2. **Grid cards not filling their cell** (`🎨️styling/🖌️ui/🎨️.css`, `:where([data-card-grid-cell]) > .contents > *`): staffage cells hold a `display: contents` wrapper, so the `section` card was the flex item, kept `flex: 0 1 auto` and shrank to its text (card 148 px in a 288 px cell: the d=24.8 card-vs-card pair). The fill rule now also reaches through `.contents`.
3. **Footer seam** (`🧱️elements/🔲️WindowSilhouette/shared-border.ts` `createSharedBorderSnap`, `🧱️elements/🗂️WindowChrome/shared-border.ts`): a scroller's rounded scroll range left the last card row 0.3 px above the footer (scrollHeight/clientHeight are integers, content is fractional), so the two logical edges were not coincident and painted two lines 2.3 px apart. The compositor now clusters the box edges of one commit (0.5 px tolerance, no chaining) before registering silhouette segments, so such edges are one shared line. Language-agnostic cases: `🧱️elements/🔲️WindowSilhouette/snap.cases.json` + `snap.schema.json` + `snap.test.ts`.
4. **Title chip vs footer chip a few pixels apart** (`🧱️elements/🃏️CardStack/bands.ts`): in a shared band the left title chip and the upper card's right footer chip could end 0.8-7 px apart (two vertical lines side by side, 6 play cards). Chips closer than `CARD_BAND_MIN_GAP` (40 px, equal to the probe's gap) but not abutting (> 0.25 px) are now `stacked` (footer row then cap row, zero gap). Two new cases in `bands.cases.json`.
5. **Play lift** ("interlock shifts cards 25 px"): the play landing rendered its cards in plain `div`s, not `CardGridCell`s, so `useCardInterlock` lifted the card itself (margin on a `size-full` card, 25.6 px too high, d=27.56 pairs). `🏢️semio-tech/🎡️play/🏠️play-landing.tsx` now wraps each card in `CardGridCell` (product-side one-liner; the framework behaviour is unchanged).
6. **UIIntroduction box**: it is a modal dialog on its own veil, not a docked cell. Placement distance and viewport inset (`INTRODUCTION_INFO_BOX_GAP_PX`, `📐️introduction-geometry`) are now 0: the box docks flush to its anchor element and to the viewport edge. Its inside already had no box-in-box (back/next are cells of the footer chips; internal pairs probed: 0). Tests updated (`🧪️tests/🧪️owned-locale-detector-retirement/🎬️uiintroduction-appearance-2-and-more/🟠️introduction-rect-relative-host`). A centred (anchor-less) box over the card grid still produces probe pairs against the cards behind its veil (cross-layer: the veil dims and blocks everything beneath), see tables.

## Probe tables (count of foreground pairs; tab fronted, compositor idle)

### Course (5396, `?preview=1`)

| view | 1440x900 | 375x812 |
|---|---|---|
| home (intro skipped) | 0 | 0 |
| `#energy` topic grid (top / bottom) | 0 / 0 | 0 / 0 |
| `#physics` quiz page (top / bottom) | 1 (content: filter chip row vs the first list row below, `css:bottom` double 0.58) / 1 (same) | 1 (content: "Show it now" button vs footer, 34 px) |
| run view (top / mid / bottom) | 1 (content: "Show it now" button vs footer 22 px) / 2 (scroll edge: cards scrolled under navbar/footer) / 0 | not run |
| identity dialog before the run | 9 (modal over the page, cross-layer) | - |
| home with the introduction open | 5 (box vs navbar/cards behind its veil) | - |

Course content for its owner: the "Show it now" button sits inside the last card 22-34 px above the footer (its own `css:bottom` border); the filter chip row on the quiz page doubles a line with the first list row (0.58 px).

### Staffage (5392)

| view | 1440x900 | 375x812 |
|---|---|---|
| overview | 0 | 0 (was 2) |
| category humans (top / bottom) | 0 / 0 | 0 (was 2) |
| all assets top / mid / bottom | 0 / 0 / 0 (was 1 card-card 24.8, 1 footer seam) | not re-run |
| collections (top / mid / bottom) | 0 / 0 / 0 (bottom was 4, d=1.9) | not re-run |

### mit-bestand demonstrator (5393, plugin panes cannot boot: crashed-plugin shell state, see `round2-demonstrator.md`)

| view | 1440x900 | 375x812 |
|---|---|---|
| overview, introduction skipped | 0 | 0 (was 1) |
| opened generator | 0 | not re-run |
| overview with the introduction open | 4 (centred box vs cards behind its veil, 19-33 px) | 6 (box vs side hints/cards/ripple, 11-36 px) |

### semio-tech play (harness, 5397)

| view | 1440x900 | 375x812 |
|---|---|---|
| landing, intro dismissed (scroll top / mid / bottom) | 0 / 0 / 0 (was 132 with the compositor idle, then 2 chip pairs and 2 lifted-card pairs) | 0 (swipe, was 1: navbar||card 30) |

## Tests

- Regression set (`vitest-ui.config.ts`: WindowChrome, OverviewCard, CardStack, LayeredOverview, WindowSilhouette, `owned-locale-detector-retirement`): 679/681 pass. The two failures are the shell-components Panel `tabBarHost="chrome"` tests (`panel-header-height` vs `calc(-1 * var(--navbar-height))`, trailing end `99.2px` vs `96px`): the concurrent OS-chrome work, not these changes.
- New/extended: `snap.test.ts` (5 cases + schema), `bands.cases.json` (+2), introduction placement test (gap zero, flush docking).
- Play suite (`🏢️semio-tech/🎡️play/🧪️tests/🎚️config/🟦️.ts`): 81/81.

## Not done / notes

- Probe false positives I did not suppress: pairs between a floating modal (introduction info box, identity dialog) and the page behind its veil, and cards scrolled under the navbar/footer (scroll edge). They are cross-layer, not card layout.
- Staffage 375 all assets / collections and demonstrator 375 opened panes were not re-run after the hint change (same code path as the pages that were).
- `vite-play-frozen.config.ts` (HMR/watch off) needs a restart (`restart-play.sh`) after each source edit; do not use it for edit loops.
- The shared course server on 5391 is hung (not started by me); I left it alone.
