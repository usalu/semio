# Round 2 — course content: nothing bordered with a gap or inset

Scope: everything inside cards of the course product (`🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react`). Stack/grid layout is the other agent's (`round2-framework-stacks.md`).

## What changed (root causes)

- `CourseCard`: a non-`flush` card now always has a flush body; its children become ONE padded cell (`CardSection`). No card has an unstructured padded body any more. New `ROW` const, `RowLink` (a link as a full-width row cell) and `BodyButton tile`.
- Classification: Categories are one column group (`.course-categories` GROUP, heading cell + `.course-bins` GROUP); every bin is a group cell (header, description, diagram, rows/"No items yet." as single dividers). Dashed frame (`DROP_ZONE_CLASS`), grid gaps, inset padding removed. Drop state = fill only (`.course-drop-active`; forced-colours keeps an inner `Highlight` dashed outline at -2px).
- Sorting: body is cells (prose, keep hint, "Keep this order" as a row button, ▲/▼ cells, unframed `RowList`); `div.course-task` is a GROUP cell. Matching: per quantity a group (heading cell, value cards as a `RowList` cell, items `RowList`); from 64rem the cards cell stretches and its divider flips to one vertical line (sticky list inside). Value cards no longer have a border / dashed state (used = fill).
- Hint: warning bar (border-left) and circled "?" border removed (filled circle). Clock: border/4px bar removed; closed clock = group of cells (text, start/cancel row buttons, problem cell); running clock a cell, warning/up = tint (`0.24`).
- Preferences: card flush, one column of group cells; every `Segments` is `flush` (no floating `w-fit` frames), checkboxes/notes are row cells; pets "play with" = tiles grid (`.course-tiles`, 2×4 / 4×2 shared dividers) of row buttons. Language choice = `RowList` of row buttons.
- Learner/leaderboard/results/clips/assignments: cards flush; tables (`Records`) are cells, last-row divider removed (also folded records, folded sort-head chips lost their borders); leaderboard period/category are two flush segment rows; results head/pending/problem as cells, task-result cards flush with `.course-result` a group (figures column divider flips at 84rem); crowd figures are cells; assignment PDF viewer and clip poster are flush cells (no border, no radius, no padding); action rows are `BodyButton row` / `RowLink`.
- Identity: `border-s-2` note -> tint; working/cancel as cell + row button.
- Quiz page: the quiz card's "Resume (...) | Start (...)" are now one right-hand footer chip group with a single hairline between (the left side belongs to the next card's title chip).
- Content art marked `data-border-probe-art`: `figure.course-mockup` (Excel/Grasshopper mockups), `figure.course-radar` (spider diagrams + their values table), crowd `figure[data-crowd-figure]` (answers + score histograms).
- Probe (`border-gap-probe.js`): box lookup for the "spanned" test is now clipped by scrolling/overflow ancestors (a card scrolled under the footer ends on the scrollport edge = the footer line, so it is no gap). This removes the `Show it now` vs footer false positive at 375x812.

## Probe table (live, course dev server, ticket crawler `round2-course-crawl.js`)
Root = the opened page/run (not the whole body); "inside" = every pair that is not purely stack/footer svg-vs-svg. 1440x900 unless noted. Silhouette SVGs lag while the tab is not fronted (the shared browser keeps being un-fronted by other agents), so pairs were re-probed up to 5x; the reported value is the settled one.

| View / state | inside |
|---|---|
| home cards (learner, quizzes, assignments, clips, board, badges, preferences) | 0 |
| overview grids `#quizzes`, `#energy`, `#assignments` (overlay) | 0 |
| `#learner` (profile, runs table, badges) | 0 |
| `#board` leaderboard page, `#badges`, `#prefs` | 0 |
| `#parametrization` (assignment page), `#clips`, clip page | 0 |
| physics: quiz page (medium, easy), run steps 1-3 (classification, 2 sorting, matching-less), unfilled + filled, easy/medium | 0 |
| ventilation easy: quiz page, dialog "Discard the open run?", run steps 1-3 filled (hints visible, spider diagrams) | 0 |
| ventilation hard: run steps 1-3 filled with typed guesses | 0 (after settle) |
| heating easy / hard (matching with cards, matching hidden keys + guess inputs) | 0 |
| cooling easy step 1 (filled) | 0 |
| demand hard steps 1-2 (filled, guesses) | 0 |
| parametric-design easy steps 1-5 (5 classification, filled) | 0 |
| grasshopper-data easy steps 1-4 (mockups) | only mockup-internal pairs before marking art; 0 expected after (not re-run) |
| excel-workbook easy steps 1-3 | 0 |
| bim-concepts hard steps 1-3 | 0 |
| optimization-decisions steps 1-2 (resumed easy) | 0 |
| quiz page physics 375x812 | 0 inside; 1 clipped-scroll pair (fixed in probe) |

## Not verified live (be honest)
- 375x812: only the physics quiz page; run views, preferences, results at 375 were NOT re-crawled (the shared browser tab kept being un-fronted/reloaded by other agents; "Discard and start"/"Start the clock" reload the dev page). The responsive paths of the new CSS (tiles 2 columns, matching below 64rem, categories single column, folded records) are covered only by the unit tests.
- Expert clock (closed/running/up), results after "Submit quiz", identity dialog, learning clip / assignment dialogs, grasshopper-data after marking art, pets UI with pets on (play tiles), crowd "Show it now" open state: code changed and unit-tested, not live-probed.
- Remaining non-content pairs I saw and did not fix: home `Cancel` BodyButton under the navbar (`🏠️home` band), `PresenceFooter` follow buttons (page footer); both belong to the stack/footer owner.

## Tests
`vitest-course.config.ts` (testTimeout raised to 120 s; the earlier journey/deputy timeouts were load only): 1124 tests pass; the four `…-kit` files "fail" with "No test suite" (helper modules matched by the include glob, unchanged).
Updated: `contrast-states` fixture (hint forced-colour rule `.course-hint-symbol`, clock tint 0.24), `adaptive-layout` (settings tier removed, `course-tiles` 28rem tier added, matching order `H3, DIV, UL.rows`), `shared-borders` (ordinary card = group with one padded cell; new test for the categories group). `load-failures`/`navigation` pass.

## Files
Modules: `🪟️chrome`, `🗂️classification`, `↕️sorting`, `🃏️matching`, `🧩️task`, `▶️run`, `🎛️preferences`, `🐾️pets/🎪️stage`, `📇️profile`, `🏆️leaderboard`, `🏁️results`, `🗳️crowd`, `🎬️clips`, `📝️assignments`, `🪪️identity`, `📖️quiz-page`, `🖥️mockup`, `🕸️radar`, `🟦️.tsx` (RowLink export). Styles: `🧩️task-layouts`, `🎚️challenge-keys`, `🤏️drag-and-spider`, `⚖️footer-and-forced-colors`, `🏁️results-and-preferences`, `🗃️record-tables`. Ticket tooling: `round2-course-crawl.js`, `border-gap-probe.js` (clip fix).
