# Course shared-border consolidation

## Mechanism added (framework, reused by the course)
- `OverviewCard` (`🧱️elements/🃏️OverviewCard/🟦️.tsx`): new `flush` prop. No body padding; the content is a
  `data-shared-border-group data-orientation="vertical"`, so its children are cells with single dividers and the card's
  silhouette is their outer edge.
- `WindowChrome/🟦️.tsx` re-exports `useSharedBorderFrameRef`; the slim chrome export (`🪟️chrome/🟦️.ts`) exposes it.
- Course chrome (`🪟️chrome/🟦️.tsx`): `CELL`, `GROUP`, `CardSection`, `RowList` (flush group+cell, or `framed` =
  shared-border frame with compositor), `CourseCard flush`, `Dialog flush`, `Segments` as a frame of cells (+ `flush` row),
  `BodyButton row` (full-width cell, no outline), `ProblemNote cell`.

## Spots fixed
- Quiz page: quiz card flush (description/facts, challenge fieldset, hints = cells; fieldset has no own border, so no box in a
  box); tasks card is one flush `RowList` (dividers only, outer edges = card edges); crowd card flush, "Show it now" is a
  full-width row cell (verified live). Footer chips ("Start/Continue ...") are drawn by the silhouette as one path with the
  card (verified by DOM: chip right/bottom == card right/bottom, chip button border 0).
- Run: head card flush (description, steps nav, progress, hint, problem, clock as cells); steps are a shared group, rows
  below 40rem, cells of one row from 40rem (course CSS flips the dividers with a higher-specificity rule); active step marks
  its seam via `data-active`. Task card flush: body, crowd door, figures as cells. `course-run-head` tier removed (fixture).
- Classification: drop zones are dashed frames whose heading/notes/radar are cells and whose chips are a flush `RowList`
  (no chip borders). Sorting: list is a framed `RowList`, rows are cells. Matching: slots are a framed `RowList`; the
  `.course-slot` own borders (hairline + 2px first-row) removed.
- Results: head card flush, new-badge tiles are rows of a `RowList`; table head rules 2px -> hairline (results, leaderboard,
  radar, profile, record/plot CSS).
- Home: leaderboard card flush, period toggle is a flush row on the card edges with one divider below; badge strip is one
  row of equal cells (`.course-badge-strip`) sharing edges (verified live); badges page tiles are rows.
- Identity dialog flush (fieldset, handle, note, problem, working = cells); waiting card flush with row buttons.
- Segments (preferences, leaderboard page) are now a frame whose buttons share dividers (compositor), no wrapping.
- Crowd door (`CrowdDoor`) is a cell group (button row + explanation).

## Left as is, on purpose
- Footer chips and Navbar/Footer (silhouette / other agent).
- Value cards in matching and presence follow buttons: discrete drag/toggle tokens with gaps, not rows.
- Selects/icon buttons inside rows, the lock notice of locked tasks, preferences' labelled controls: single controls in padded
  rows, not adjacent to an edge.

## Files
`OverviewCard/🟦️.tsx`, `WindowChrome/🟦️.tsx`, ui `🪟️chrome/🟦️.ts`, course `🪟️chrome`, `📖️quiz-page`, `⛰️challenge`,
`🗳️crowd`, `▶️run`, `🗂️classification`, `↕️sorting`, `🃏️matching`, `🧩️task`, `🏁️results`, `🏆️leaderboard`, `🏅️badges`,
`🪪️identity`, `📚️app`, `🟦️.tsx` (exports), styles `🧩️task-layouts`, `🗃️record-tables`, `📊️plots-and-crowd`,
`🏁️results-and-preferences`, radar/profile (`border-b`).

## Tests
- New `🧪️tests/🧱️shared-borders/🟦️.tsx` (flush card, RowList, framed list, Segments, row button, cell note; stylesheet checked
  with lightningcss: no borders on rows, flip rule + hairline-only tables). Registered in the course vitest config.
- Updated adaptive-layout fixture (run-head tier removed, kinds 64rem tier named, steps wording) and badges test
  (`learner-pages`: shared cells instead of border-solid/dashed).
- Result: whole course-react vitest suite 47 files / 1121 tests pass (run via bun vitest directly; `repo run` was slow).
- UI: two OverviewCard tests (flush / non-flush) pass 5/5 with the coordinator's `vitest-ui.config.ts` (the stale lift-class test was fixed by the coordinator).

## Follow-up round (live verification in the dev server, offline runs work)
- Task body: the run's task card is flush end to end; `TaskBody` returns cells (clock, prompt + "Task n of m" line in ONE
  section so the header line no longer has its own extra divider, then the task view as a cell).
- Classification: pool and categories are the two cells of one `course-classify` group. Pool has no frame and no dashed
  border, its rows are flush with the card body edges; below 84rem one divider between pool and categories, from 84rem the
  divider turns into one vertical line (`.course-categories` inline-start) shared by both columns, pool stays sticky. Dashed
  bins stay dashed inside padded categories (they touch no frame edge).
- Trailing controls: `.course-cell` (select, move buttons, remove button, guess entry + unit select) = stretch to the row's
  top/bottom/right edges, no border of their own, exactly one hairline to their left (below the label across the row in
  narrow rooms: a hairline above). Applied to classification, sorting (up/down, guess), matching (select, remove, guess).
  `SELECT_CLASS`/`ICON_BUTTON_CLASS` lost their `border`; rows lost their padding (label/grip carry it).
- Sorting/matching bodies are padded sections with a framed `RowList`; mockup previews in rows (`.course-mockup`) are
  padded under a hairline so their frame no longer lies on the row/frame edges.
- Header line / Submit chip: no doubled edges (probe below); the "Task n of m" line shares the prompt's section.

### Live check (Playwright-free DOM probe in the built-in browser)
Probe: for every element with a visible border side, report (a) two different elements whose borders lie on the same line
and overlap > 6px, (b) any element border lying on a card's outer edge. Run on the run screen of every task of
physics (classification, 2 sorting), heating (matching, easy and hard), ventilation (hard: classification with radar,
sorting with guesses), parametric-design (5 classification), grasshopper-data (4 classification with mockups),
excel-workbook (3 classification with mockups), bim-concepts, optimization-decisions, and the results of ventilation.
Result: no overlaps, no border on any card edge after the fixes; remaining "hits" were inside the Excel/Grasshopper mockup
art itself, and radar "values as a table" rows inside a closed `<details>` (not rendered). 375px width checked for steps,
sorting and classification rows. Not touched: value cards of matching (discrete drag tokens with gaps), drop zones' dashed
frames (kept on purpose).
- Tests: full course suite 47 files / 1123 tests green; shared-borders test now also covers the classification cells and the
  `.course-cell` rule.
