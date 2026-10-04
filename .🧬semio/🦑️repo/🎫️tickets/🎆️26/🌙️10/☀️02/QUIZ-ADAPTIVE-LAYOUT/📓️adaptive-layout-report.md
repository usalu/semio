# 📐 Quiz Adaptive Layout — Report

Owner's goal (2026-10-02): the quiz UI must adapt far better to width and device; desktop, tablet and phone each laid
out by hand. Where an element fits on one line it uses the left and the right (an item with its dropdown beside it),
and breaks onto two lines only where there is no room.

## What the baseline showed

Surveyed with `layout_survey.ts` (Playwright, every screen, 11 viewports/text sizes):

- **Desktop:** every classification item put its dropdown on a second line under the label, even in a 1100 px wide
  pool; bins used `auto-fill`, so two categories left a third of the card empty; the sorting guess sat on a second line;
  1920 px screens used the same 1152 px column as 1280 px.
- **Phone:** every table scrolled sideways — results (+180…+480 px), the leaderboard page (+712 px), the learner's runs
  (+173 px), the crowd figures (+188…+230 px), the radar value tables — and matching squeezed the item label into a 38 %
  column (up to five lines per row). At the largest text size the run screen itself scrolled sideways (+15 px).
- **Preferences:** names of a fixed `6em` width, so "Antworten der anderen" pushed its control out of line.

## Decisions

1. **Rooms, not windows.** Every part measures the container it lies in — `[data-slot="quiz-card-content"]`
   (`quiz-card`), `.quiz-task`, `.quiz-rows`, `.quiz-records` — with container queries in **rem**. A card is laid out the
   same in a phone column, an overview pane, a dialog or a desktop page, and larger text gets the narrower layout.
   The only width media queries left are the home grid's and the first-visit pair's (window-level by nature).
2. **Tiers** (shared vectors `🧰️framework/🛍️products/❓️quiz/🧫️fixtures/📐️adaptive-layout/🔣️.json`):

   | part | room | from | below | from there |
   | --- | --- | --- | --- | --- |
   | `quiz-steps` | card | 40 rem | list, title left / state right | chips in a row |
   | `quiz-run-head` | card | 64 rem | progress below the tasks | tasks left, progress right |
   | `quiz-chip` (classification item) | list | 30 rem | select below the label | label left, select right (aligned column) |
   | `quiz-classify` | task | 84 rem | pool above bins | pool beside bins (2 : 3), sticky |
   | `quiz-sort` | list | 36 rem | label line 1, guess + ↑↓ line 2 | one line |
   | `quiz-sort` (guess preview) | list | 56 rem | preview under the field | preview in a reserved column beside it |
   | `quiz-slot` (matching row) | list | 30 rem | select + ✕ below the label | label left, select + ✕ right |
   | `quiz-match` | task | 64 rem | value cards above rows | value cards in a sticky column beside rows |
   | `quiz-result` | card | 84 rem | figures below the tables | figures beside the tables (3 : 2) |
   | `quiz-settings` | card | 28 rem | name above a full-width control | names column left, controls aligned right |
   | `quiz-kinds` | card | 44 rem | ways to appear stacked | three columns |

3. **Tables fold into records** below the width their columns need: `Records fold={rem}` sets `--quiz-fold`; the frame's
   font size is fold/36 rem, so a single `@container quiz-records (max-width: 36em)` folds every table at its own width.
   Folded: lead cells (name, verdict, rank, total, action) on the first line, facts side by side under their column
   heading (`data-label`), notes (explanation) full width. The leaderboard's sort headings become chips. Every part
   carries an explicit table role (`TABLE`), so a block-displayed table stays a table for assistive technology.
   Folds: classification results 56, sorting results 58, matching results 52, radar values 28, leaderboard page 68
   (every quiz) / 44 (one quiz), leaderboard card 19, learner runs 46.
4. **Crowd figures** fold below `(9 + 3.5 × columns) × 0.75` rem (`plotFold`): each item on its own line over its
   columns, which name themselves under their marks; a matching's many value columns (`data-sparse`) show only those
   somebody chose or that are marked, wrapping in a grid.
5. **Matching is a list, not a table** (`ul.quiz-slots > li.quiz-slot`: label, select, ✕) — the only way rows can
   become two lines on a phone and one on a desktop; selects keep their own names ("Power of LED bulb"). The unused
   keys `quiz.matching.item` / `quiz.matching.actions` were removed (EN, DE).
6. **Sorting row reading order** is now grip, place, label, guess, ↑, ↓ (the guess was after the buttons in the DOM but
   before them on screen once one-line).
7. **Touch:** on `pointer: coarse` task controls are ≥ 2.5 rem tall and grips/icon buttons as wide (value-card grips
   1.75 rem wide so two cards fit a phone row).
8. **Wide screens:** run and results pages go up to 100 rem, home panes (`PageFrame wide`) up to 90 rem; running text
   keeps `80ch` (`.quiz-prose`). The radar keeps ≤ 28 rem instead of growing with a wide bin.
9. **Drag ghost** is an empty shallow copy of the source's list holding the copy of the row, so container queries give
   the ghost the same one- or two-line shape as its source.
10. **Overview list mode** (phones, short windows, large text): cards were sized by their content, which a size
    container cannot report — a 1280 × 800 window at the largest text showed a 150 px card. Cards of the list now take
    `min(100%, 36rem)` (`.quiz-home-entry`).

## Tests

- **Unit (vitest + lightningcss, `🧰️framework/🛍️products/❓️quiz/🧪️tests/📐️adaptive-layout/🟦️.tsx`, 9 cases):** every
  `@container` rule as lightningcss parses it must be exactly the fixture's tiers (room, `>=`, rem, value, part), no
  undocumented tier, every room a named inline-size container, every fold the one 36em query with the fold/36 yardstick,
  `plotFold` vectors, `Records`/`TABLE` roles, and the reading order of chip, sort and slot rows. Passed 9/9 before the
  concurrent sorting rewrite (see below); 8/9 after it.
- **Real browser (Playwright, `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/📐️adaptive-layout/🟦️.ts`, project `layout`):**
  phone 375, tablet 768, desktop 1280, wide desktop 1920 — task list vs chips, pool item line vs stacked, panes vs
  stacked, sorting row, touch target ≥ 40 px, matching row, value cards beside rows, result table vs records, and no
  box scrolling sideways on identity, run, results, leaderboard, profile, preferences and quiz pages; each observed
  layout is also checked against the tier for the measured room width. **4/4 passed** (run with `layout_spec.ts`
  against the steady site, 2026-10-03).
- **Updated:** `⌨️task-keyboard` (matching as a list; sorting tab order guess → ↑ → ↓; ghost in a copy of its list),
  `💭️crowd-client` (`.quiz-records` container rule), `🗣️translation-completeness` (scroll frames include `.quiz-records`
  with `position: relative`). Oracle `quiz-react-lightningcss` gained `quiz-react-adaptive-layout`; taxonomy member
  lists gained `📐️adaptive-layout`; vitest include and e2e project `layout` registered (`presence` now waits for it).
- **Survey after the change:** no element scrolls sideways at 320, 375, 375 @ 1.5×, 430, 600, 812 × 375, 768, 1024,
  1280, 1280 @ 1.5×, 1920 (only a 3 px residue on one figure at 375 before the sparse matching figure).
- **Typecheck:** clean on the first pass; the site's `layout` spec and config have no errors.

## Not green, and why

Another session is implementing run challenges (keys shown/hidden, timed tasks, hints) across the quiz core and the
React client at the same time. Its unfinished state, not this ticket, causes: the typecheck errors in `matching`,
`results`, `leaderboard`, `quiz-page`, `classification`, `crowd`, `session`, `deputy` and several tests (fields made
optional, `Best`, `challenge`); the failing `learner-journey`, `deputy-decisions`, `crowd-client` fixtures and
`translation-completeness` rejection list; the `⌨️task-keyboard` sorting case (completeness now fails at its line 225,
before the changed tab-order assertion); `home-grid` labels showing `{{challenge}}`; and `withUnit is not defined` in
the half-rewritten sorting module, which fails the adaptive-layout sorting-order case. That session already builds on
`.quiz-sort` / `.quiz-guess` and announces a `.quiz-sort-key` column; its CSS is not part of this ticket. The e2e gate
(`test-e2e`) was not run in its own throw-away stacks, because they build the proctor from that changing tree.

## Ticket files

- `layout_survey.ts` — walks every screen at 11 viewports and writes screenshots plus an overflow report.
- `report_digest.py` — prints the findings of a survey report.
- `overview_probe.ts` — opens the overview at one viewport/text size and lists mode, card boxes and reloads.
- `container_rules_probe.ts` — prints how lightningcss parses the stylesheet's containers.
- `steady_site.sh` — a second, non-watching dev site on 6065 (stop the listener on 6065 before starting it again).
- `layout_spec.ts` — runs the e2e project `layout` against a running site.
