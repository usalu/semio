# Round 4 live crawl: course, OS harness, staffage

Method: own tab in the in-app browser, pane shown and tab fronted before every probe, own native vite servers (course 5591, OS 5592, staffage 5593; all killed). Whole-body base probe plus near-double probe (`round3-course-probe.js`); dialogs are probed as their own subtree (root = the open dialog, the modal underlay is excluded). Focus sweep per control on every view and step. Tooling: `r4-util.js`, `lv-course-run4.js` (now `de`-aware), `round3-course-crawl.js` (dialog root, clip-aware near probe, d=0 coincident opaque svg pairs not counted).

## Counts (base violations / near, informational rows aside)
| View | 1440 en | 768 en | 375 en | 1440 de |
|---|---|---|---|---|
| home views: learner, quizzes, energy, board, badges, prefs, assignments, parametrization, typology, heating-cooling, parametric, grasshopper, excel, bim, optimization, physics page | 0 | 0 after fixes (see FP 2) | 0 (board: FP 3) | 0 |
| quiz pages + all steps (filled, focus sweep) + submit dialog (subtree) + results, medium, all 10 quizzes (physics, ventilation, heating, cooling, demand, parametric-design, grasshopper-data, excel-workbook, bim-concepts, optimization-decisions) | 0 | 0 (FP 1, 2) | 0 (FP 1) | physics, grasshopper-data, excel-workbook, bim-concepts medium + expert: 0 |
| physics easy / hard / expert (steps, dialog, results) | 0 after fixes | not run | not run | expert: 0 |
| classify (pool/categories), match, crowd results, guess input | covered by the quiz runs above (grasshopper-data, excel, bim, demand carry classify/match/crowd tasks), 0 | 0 | 0 | 0 |
| OS harness, all 72 parts incl. staged action dialog en+de | not rerun | 61 clean + staged dialog 0; 10 parts non-zero, all FP 4-6 | not rerun | n/a |
| Staffage: overview, all, animals, walking-commuter, domestic-cat, cafe-chair, circulation-pump, collections, compare, playground | not rerun | 0 after fix 4 | not rerun | n/a |

Not run (time): easy/hard/expert for the nine non-physics quizzes, de at 768/375, OS and staffage at 1440/375 in this round (round 3 had them 0), crowd results with real crowd data (offline server; crowd view shows the empty state).

## Task 1: card interlock (the lift was inset aware, the seam still was not one line)
Verified live: `#learner` 768 (was 1.72/2.25 px) and physics quiz page 1440 (was 2.06/1.69 px) are now exact; navbar-vs-first-card is 0 at 1440. Root causes found (two, both in the shared framework):
1. Fractional card heights (e.g. 69.59, 162.72 px): the silhouette svg is snapped to device pixels (69 vs 70) while the box is not, so the painted lines were 0.3 to 0.6 px apart. Fix: `useCardInterlock` snaps the window stack and the card up to whole device pixels (`min-block-size`, reset on every measure) with the pure `snapCardHeight` (bands.ts / bands.rs, vectors in `bands.cases.json` `snaps`, schema extended; TS + Rust tests).
2. A card inside a flex/grid cell that clips it (course home grid, `course-home-cell`, fractional `fr` tracks): the snapped card overflowed its cell by 0.5 px so the lifted neighbour was 0.5 px off (assignments at 768). Fix: the lift absorbs the measured drift `above.bottom - below.top` (`lift = band.lift - drift`), rounded to 1/1000 px.
Files: `🧱️elements/🃏️CardStack/{🟦️.tsx,bands.ts,bands.rs,bands.test.ts,bands.test.rs,bands.cases.json,bands.schema.json,🟦️.test.tsx}`. Tests: CardStack 30/30 (vitest), `repo cargo test card_band` 3/3 passed (light crate). `bands.cases.json` remains the single source of truth.

## Real defects fixed (each with a test, re-probed 0)
1. Results page: `.course-results` was a flex column with `gap-double`, so the result cards sat 6.4 px apart and the interlock could not engage (8.39 px pairs on every results page at 1440). Removed the gap (`🏁️results/🟦️.tsx`); test in course `🧱️shared-borders`.
2. Expert run page: the sr-only clock announcer (`ClockWatch`, empty live region) was wrapped in a visible padded `CardSection`, a 12.7 px empty cell below the progress row (37 px pair to the card bottom). Moved the announcer into the always present progress cell (`▶️run/🟦️.tsx`); test in `🧱️shared-borders`.
3. Segments (leaderboard period/quiz choice, `🪟️chrome`): cells of a wrapping row drew their right divider on top of the card edge (0.5 px pair at 768). Segments is now a `SharedBorderRow wrap` (the existing wrap mechanism tags line ends); test in `🧱️shared-borders`.
4. Staffage navbar at 768: the search form had no room, shrank to 0 width and its 18 px input overflowed onto the language choice (25 px pair). The search is shown from `lg` only (`📚️app/🟦️.tsx`); test in staffage `🧷️shared-borders` (8/8).
5. Added OS harness parts `r4-staged-dialog(-de)` (round3-os-parts.tsx): staged action dialog open, `SharedBorderRow edges=["top"]` row: 0 at 768 (en, de).

Verified after the other agent's `--shared-border-flow` change: course steps, classify, match, results and guess views at 1440/768/375: 0 new violations.

## False positives / non-defects (explained, filtered or listed)
1. Content scrolled against the footer or navbar: a border of scrolled content lies 0.5 to 3 px from the footer top or navbar bottom at one scroll offset (heating step2 `li.course-slot`, results `tr`/`th`, `h4`, `select`; "Show it now" is clipped by the scroller). Scroll position, not layout; the scroller ends 48 px above the footer when pets are shown. The near probe is now clip-aware.
2. Vertical jogs between a navbar button's right border and the first card's chip edge (0.05 px, 0.94 px, 1 px): two different rows with text-sized widths; a seam between rows cannot have them coincide in general. Parametric-design 768 steps 2/3: card top 1 px below the navbar after a scroll of 1 px.
3. Wrapped `Segments` rows: the divider between line 1 and line 2 is 34 to 36 px from the row's bottom line with a fully bordered cell box in between (stagger of cells of different widths). Probe has no concept of an enclosing cell; filtered in the crawler by a documented regex, visually verified.
4. OS harness 768, `settings-*`, `display`, `marketplace`, `panel-bottom` (8 parts, 39 px pairs): the floating top-right panel (420 px) lies over the 50 % split of window b; its left edge is 39 px from window a's right edge, with the title chip "b" exactly in between. Incidental distance below the probe's 40 px threshold; at 1440 the lines are far apart (0).
5. OS harness 768 `layout-bottom` (12 px) : "Utilities" chip of a window and the bottom-right panel edge; same overlay class (panel floats over canvas windows by design).
6. OS harness 768 `window-rich-many` (39.79 px): Search chip and Window Options chip of one window separated by the designed U-gap of the cap row, which shrinks below the probe threshold at this width.
7. d=0 `svg:h+svg:h` near pairs: two silhouettes paint the same opaque line at the same y (card seam); one visible line. Not counted.

## Open items
- Easy/hard/expert for the nine other quizzes, `de` at 768/375, and 1440/375 for OS and staffage were not rerun this round.
- Vite dev servers on this machine spin at ~100 % CPU after a while (kill -9 and restart; load average 18); tool calls above 45 s time out.
- Course vitest: shared-borders 18/18 style suites run individually (shared-borders, live-regions); the full course suite was not rerun.
