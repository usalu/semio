# Round 3 — course (Architektur und Technologie)

Status: PARTIAL. The browser host became unreachable mid-run (PreToolUse hook timeouts), so the crawl of runs, quizzes, 375/768 viewports, de, and zoomed seams was NOT completed. What is below is what was actually measured and changed.

## Finding about the probe itself (root cause of "probe 0, user still sees double seams")
- The round-2 crawler probed only the opened `[data-page]` root. Navbar, footer and the stack/footer "mixed" pairs were excluded. `round3-course-crawl.js` now probes `document.body` (off-screen layered pages tagged `data-border-probe-art` for speed), reports mixed pairs, and adds `round3-course-probe.js` (near-doubles up to 3.2 px: svg-vs-css coincident, same-owner svg pairs, outline/box-shadow lines, focus-visible sweep per control).
- With the whole-body probe the settled state is NOT clean:
  - `#physics` (quiz page, 1440): 4 base / 3 near pairs: stack seam svg:h at 305 vs 307.06 (d 2.06, span 130 px), 329 vs 329.06, 406.1 vs 409.8 (d 3.69).
  - `#quizzes` (topic grid, 1440): 9 base pairs, d=2 both horizontal (y 403 vs 405, spans 1-59, 481-683, 961-1061) and vertical, between row-1 and row-2 cards.
- Diagnosis (measured, not yet fixed): in `useCardInterlock` the lower card is lifted by band=24 so bottom lines coincide, but the silhouette top-edge depth is the cap chip pitch (22: lines at y=1 and 23) while the footer chip pitch above is 24 (lines at h-25 and h-1). The chip top line of card N+1 therefore lies 2 px below the body-bottom line of card N. Also seen: a card's svg (h 101) stale against its box (h 102.69) while the tab was not fronted. Candidate root fix (framework, `CardStack/🟦️.tsx` + `WindowChrome` depth measurement): make cap and footer chips share one pitch (take depth = band for both edges) and re-measure the silhouette on `--card-stack-*` changes. Not implemented: could not verify live. Needs a follow-up with a working browser.

## Probe table (whole body, 1440x900, settled, base/near)
| View | 1440 | 375 | 768 |
|---|---|---|---|
| home | 0/0 | not run | not run |
| #learner, #energy, #board, #badges, #prefs, #assignments, #parametrization, #typology (page root only, old method) | 0/0 | not run | not run |
| #physics quiz page (body) | 4/3 | not run | not run |
| #quizzes (body) | 9/12 | not run | not run |
| all quiz runs/steps/states, results, dialogs, de | not run | not run | not run |
Focus-visible sweep (`probe3`, per control): 0 findings on #learner #quizzes #energy #board #prefs #assignments #parametrization #typology.

## Fixes made (coordinator items F11–F14)
- F11/F11b load fallback (`📚️app/🛟️load-fallback`): the two buttons are now cells of a `SharedBorderRow` (no gap, no native border, no arbitrary-variant hack). Caveat: the fallback now imports `@semio-tech/ui-react/chrome`; if that chunk is what failed to load the fallback would fail too (it is in the main bundle today, not verified).
- F12 quiz page (`📖️quiz-page`): "last result", "resume" and start actions now carry `CELL` instead of `border-e border-normal`.
- Framed containers drop the native `border border-normal`, the frame owns the outline: navigation group (`🚏️navigation`), `RowList framed` and non-flush `Segments` (`🪟️chrome`).
- Access status chip (`🔓️access`): `SharedBorderRow` with one `SharedBorderRowCell`.
- Presentation: removed the native border from `.presentation-interaction-actions` and `.presentation-pdf-page-nav`; the slide-reset button is now a cell of its own `SharedBorderChipGroup` frame (`.presentation-interaction-slide-reset-frame`, hidden together with the button when a disposition is selected). It cannot join the actions group: they live in different parents (slide vs disposition). Presentation was NOT verified live and its vitest suite was not run.
- F13 (hand-rolled `border-inline-start/block-start` in `🧩️task-layouts` and `🏁️results-and-preferences`): NOT migrated. These rules are responsive flips (container queries, grid areas) of dividers that the group/cell mechanism cannot express (it knows only one orientation), and an existing test pins them. They draw exactly one hairline per seam (earlier crawls 0). Needs a responsive-orientation feature in the framework first.

## Tests
- Extended `🧪️tests/🧱️shared-borders/🟦️.tsx`: load-fallback buttons (no gap, no border, one outline), quiz actions carry `CELL` (3), framed `RowList`/`Segments` have no native border. Course vitest via `vitest-course.config.ts`: 1127/1128 passed before the test update, then the shared-borders, load-failures, bounded-load, quiz-card-challenges and access files 50/50. The four `*-kit` files still report "No test suite" (helpers, pre-existing).

## Files
Course: `📚️app/🛟️load-fallback`, `📖️quiz-page`, `🚏️navigation`, `🪟️chrome`, `🔓️access`, test `shared-borders`. Presentation: `🫳️interaction/🟦️.tsx`, `🔲️interaction-handles-and-controls/🎨️.css`. Ticket tooling: `round3-course-probe.js`, `round3-course-crawl.js`.

## Update: root-cause fix applied (unverified live)
- New token `--chip-row-height: round(nearest, max(1.5rem, calc(7 * var(--ui-spacing))), var(--device-pixel))` (next to `--navbar-band-height`, ui `🎨️.css`) and utility `min-h-chip-row`. All cap chips, gaps and footer chips in `WindowChrome` now use it (were `min-h-medium`, 22.4 px), and `OverviewCardAction` reads the same token (was a separate 24 px formula), so both rows have one pitch and the shared band's top and bottom lines coincide.
- Test: `CardStack/chip-row.test.ts` + `chip-row.cases.json` (token declared once; WindowChrome and OverviewCard read it, no `min-h-medium`/own formula). OverviewCard test updated. Framework suites CardStack, OverviewCard, WindowChrome pass via `vitest-ui.config.ts`.
- Old crawler: `round2-course-crawl.js` still probes the page root; the replacement `round3-course-crawl.js` probes the whole body (use it).
- Browser host stayed unreachable (retried), so the 2 px seams at `#physics`/`#quizzes`, the 375/768 widths, runs, states and de are still UNVERIFIED live. Rerun `round3-course-crawl.js` (`view`, `runCrawl`) with a fronted tab to confirm base/near = 0.
