# Round 2 — course final live verification (own server 5398, `?preview=1`)

Method: shared 5391 server hung, so own Vite (`vite-preview.config.ts`, port 5398, killed afterwards). The page runs in an iframe of exactly 375x812 / 1440x900 inside my own tab (the pane width kept changing under `resize_window`). Driver `round2-final-driver.js` wraps `border-gap-probe.js`: probes the opened page/dialog only (inert layered panes hidden), sweeps every scroller top to bottom, retries up to 4x for the compositor, counts a pair as "inside" only when it shows at rest (scroll top/bottom); pairs seen only mid-scroll are listed as `midscroll` (none occurred). Pairs purely stack/footer svg-vs-svg are `mixed` (framework follow-up agent). All numbers below are `inside` after the fixes.

## Fixes (content inside course cards / chrome)
1. Identity dialog: the handle `input` was a bordered box inside a padded cell (35 px / 8 px inset pairs). Now a vertical group cell: label cell, `.course-field` cell holding the borderless input, rule cell (`🪪️identity`, `.course-field` in `🏁️results-and-preferences`).
2. `.course-alert` (error/notice cell): 4 px `border-inline-start` bar sat 1 px from the card edge. Now an inset `box-shadow` bar (not a border).
3. Sorting rows at narrow width (hard run, guess below label): the guess cell's left border stood 32 px inside the card edge beside an empty grid column. The guess area now spans place+label columns (`"grip guess guess up down"`) and gets its left divider only from 36 rem, where the label is beside it (`🧩️task-layouts`).
4. Short pages: last card now fills the scrollport (`.course-page-frame` / new `.course-page-scroll` in app `Page`: stack `min-height:100%`, last card `flex:1`). Fixes the coordinator item (b): "Show it now"/crowd card and the tasks card ended 8-41 px above the footer (card bottom 740 vs footer 781); now 781/781.
5. Unowned items: home "Cancel" under the navbar is now a band (`SharedBorderRow`, `edges=bottom`, progress cell + `BodyButton cell`, full width, no padding margin) in `🏠️home`; `PresenceFooter` is a flush `SharedBorderRow` group — status, "following" status, one cell per learner (follow button is the cell's body, no border), stop button cell; no `gap-single`, no bordered buttons (`👥️presence`). Presence needs a live roster (server) so it was not rendered live here; covered statically only.
6. Test added (`shared-borders`: no border on `.course-alert`, `.course-field` borderless, no inline-start border on narrow `.course-guess`, guess area, last-card fill).

Coordinator items: (a) "filter chip row vs first list row 0.58 px" and (c) "run view content pair + 2 scroll-edge pairs": not reproducible at 375, 1024, 1200, 1366, 1440, 1500, 1920 on the physics quiz page nor in any run step (the quiz page has no filter chips; I read the report as the card title chip row, which probes 0; the run pairs were the narrow guess inset (fix 3) and fix 4). Some 19-20 px pairs seen mid-run were stale silhouette SVGs (viewBox of the old card height) that vanish after the compositor commits in a fronted tab.

## Probe table (inside count; tests 1125/1125 pass, the 4 `…-kit` files still report "No test suite")
| View / state | 375x812 | 1440x900 |
|---|---|---|
| home (swipe cards / grid) + hint bars | 0 | 0 |
| intro tour steps 1-6 (window silhouette only) | 0 | not run |
| `#learner` with runs (runs table folded, badges) | 0 | 0 |
| `#quizzes`, `#energy` topic grids | 0 | 0 |
| `#board`, `#badges`, `#prefs` (pets on, no cast in preview) | 0 | 0 |
| `#assignments`, `#parametrization`, `#typology`, `#heating-cooling` | 0 | 0 (first two) |
| `#clips`, `#clip-ventilation` | 0 | 0 |
| quiz pages physics, ventilation (resume/start chip group), heating, cooling, demand, parametric-design, grasshopper-data, excel-workbook, bim-concepts, optimization-decisions | 0 | 0 (physics, ventilation, grasshopper-data) |
| crowd closed ("Show it now") / open / hidden-by-hard-run | 0 | not run |
| confirm dialogs: discard open run, submit quiz, switch identity | 0 | not run |
| identity dialog anonymous / pseudonym / name / empty-name error / working | 0 (after fix) | not run |
| physics medium run steps 1-3 (classification, sorting, matching) filled | 0 | not run |
| physics expert: clock closed / running | 0 | not run |
| results page after "Submit quiz" | 0 (+3 mixed stack/footer) | not run |
| ventilation easy steps 1-3 (hints, spider) | 0 | not run |
| ventilation hard steps 1-3 (guess inputs typed) | 0 (after fix 3) | 0 |
| heating easy 1-2 / hard 1-2 (matching, guesses) | 0 / 0 | hard 0 |
| cooling easy 1, demand hard 1-2 | 0 | not run |
| parametric-design easy steps 1-5 | 0 | not run |
| grasshopper-data easy steps 1-4 (mockups marked art, 3 art roots) | 0 | 0 |
| excel-workbook easy 1-3, bim-concepts hard 1-2, optimization-decisions medium 1-2 | 0 | not run |

Visual: screenshot of the ventilation quiz page at 375 confirms the right footer chip group "Resume (Hard) | Start (Easy) >" with one hairline between the chips, and the next card's "Tasks in this quiz" title chip on the left of the same band.

## Not verified live (honest)
- Pets tiles ("play with"): the pets layer renders no cast in this preview (no `[data-pets-play]`), code + unit tests only.
- Presence footer (needs a presence roster), clock warning/up states (5 min wait), home "Cancel" pending bar (transient, 2nd-level state), open native `select` popovers (not DOM), sort-head popovers (learner table is folded at 375 and has no heads; leaderboard sort not exercised), intro tour at 1440, results/run views at 1440 other than the rows above.
- Framework-owned (not touched): `mixed` stack/footer svg pairs, intro-vs-background cross-layer pairs.

## Files
`🔨️modules/🪪️identity`, `🏠️home`, `👥️presence`, `📚️app` (`course-page-scroll`), styles `🏁️results-and-preferences`, `🧩️task-layouts`, test `🧪️tests/🧱️shared-borders`. Ticket tooling: `round2-final-driver.js`.
