# Round 3 live verification (in-app browser, own tab, servers 5491 OS / 5492 staffage / 5493 course; all killed)

Method notes: results are only trusted when the compositor is not stuck (a hidden or unfronted pane stops rAF; `tabs_select` right before every call, and `data-shared-border-pending` is checked before the probe; the probe itself re-arms the attribute, so "pending after" is not meaningful). Staffage server runs with watch off (stale ticket-module cache: use fresh file names). Playwright Chrome still cannot launch. Vite 5491 once spun at 95 % CPU and had to be `kill -9`ed.

## Probe counts (violations; base probe, magenta-free)
| Product / view | 1440x900 | 768x1024 | 375x812 |
|---|---|---|---|
| OS: all 70 parts of `round3-os-live.tsx` (bands, route admission, subfooter, corner trigger, staged reference chips, Paint2d, NodeGraph peers, time travel, rich window panes, dialogs, settings, popover, ...) | 0 (after fix below) | not run | 0 (70/70, except the three desktop `layout*` harness parts rendered at 375: 1 / 2 / 1, see Open) |
| Staffage: overview, all, animals, collections (+bottom), compare, playground | 0 (after 2 fixes below) | not run | 0 |
| Staffage: asset pages walking-commuter, domestic-cat, cafe-chair, circulation-pump, "Three views" (unavailable cells) | 0 | not run | 0 |
| Staffage: 3D model view with toolbar (walking-commuter, domestic-cat, circulation-pump; stage ready) | 0 | not run | 0 |
| Course: home, learner, quizzes, energy, board, badges, prefs, assignments, parametrization, typology, heating-cooling, parametric, grasshopper, excel, bim, optimization, 2 clips (body probe, base+near, focus sweep) | 0 (board: one 0.5 px near pair seen once during the crawl, not reproducible in 3 re-probes) | learner 2 (+2 near) | quizzes, learner, energy, board, prefs, physics: 0 |
| Course: quiz page physics | 2 + 2 near (stack seams 2.06 and 1.69 px) | not run | 0 |
| Course: physics run medium step 1-3 (filled), easy/hard/expert, other 9 quizzes, results, de | partial, see below | not run | not run |

## Fixed in this pass (each with a test, re-probed 0)
1. OS `StagedActionDialog` (ShellHost/📝️staging): the Cancel/Execute row sat inset (7 px) inside the padded dialog (2 pairs at 1440 for en and de). Dialog is now `p-0` with a padded content div and a full-width unframed `SharedBorderRow` with `border-t` flush at the dialog edge. Test: `staging/dialog-borders.test.ts` (new, passes).
2. Staffage collections: a section that ends in a card grid (`edges=top right left`) still drew its vertical-cell block-end divider 1.5 px beside the grid's bottom line (9 pairs). Generic CSS rule in ui `🎨️.css`: a framed cell of a vertical group whose `data-shared-border-edges` omits `bottom` draws no block-end divider. Test: `SharedBorderRow/round3-framework.test.tsx` (12/12 pass).
3. Staffage playground timeline ruler: end tick `border-l` coincided with the section's right edge (d=0). Ruler is content art: `data-border-probe-art`. Test in staffage `shared-borders` (7/7 pass).
All other round 3 fixes (OS F2-F10, staffage toolbar/notices/detail tables/collections edges, course `--chip-row-height`, `#quizzes` grid) held live. The `#quizzes` topic grid, which was 9 base pairs before the `--chip-row-height` token, is now 0.

## Open (not fixed)
- Course stacked-card seams (framework, CardStack / WindowChrome silhouette). Reproduced stably on the physics quiz page at 1440 (2.06 px between "Physical Understanding" and "Tasks in this quiz", 1.69 px between "Tasks in this quiz" and "What everyone answered") and on `#learner` at 768 (1.72, 2.25), also navbar-vs-first-card 1-2 px at quiz page / results. Measured: both cap rows and footer rows are 24 px now (`--chip-row-height` works) and `useCardInterlock` lifts the lower card by `band` (24) or 0 (no footer above) while the overlay lines are painted 1 px inside the card box (`M1,1 ... V h-1`), so the logical box edge coincides but the two painted lines are 2 px apart. The compositor does not dedupe them (both svgs paint), so the fix belongs in either the interlock (lift = band + 2 and 2 for the "none" mode, with `bands.cases.json` + a Rust mirror update) or in compositor ownership for coincident silhouette segments across cards. Not done: it changes the shared bands contract used by Rust/wgpu targets and needs its own cycle.
- Course: quiz runs only partially crawled (physics medium steps 1-3 filled: 0 except the stack seam above, results page shows the same seams). Crawler needs the identity dialog handled and survives only until another agent's HMR reload; `lv-course-run4.js` handles the dialog and resumes via sessionStorage. Easy/hard/expert, the other nine quizzes, submit dialogs and `de` were NOT run. The crowd and submit dialogs probe non-zero with the whole-body probe because the page behind a modal is probed (false positive class: underlay of a modal), not investigated further.
- OS harness: `layout`, `layout-bottom`, `layout-many` at 375 (desktop `Layout` forced into a phone viewport, not a shipped state; `layout-mobile*` parts are 0).
- Informational overlaps (ov, not counted): palette/find close button over input, window-search overlay spans, keybinding rows, rich-window toggle over action items.
- 768x1024 not run for OS and staffage.

## Tooling left in the ticket
`lv-staffage-view2.js`, `lv-staffage-run3.js`, `lv-course-run4.js`, `lv-vitest-staging.config.ts`.
