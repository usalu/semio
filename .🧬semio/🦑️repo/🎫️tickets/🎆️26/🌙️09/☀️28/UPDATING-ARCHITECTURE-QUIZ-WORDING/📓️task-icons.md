# Icons with microanimations (2026-10-02)

Every task (question), every item (the draggable answers of a classification and a sorting, the rows of a matching), every category (the bins) and every matching dimension (shown on each of its draggable value cards) of the four architecture quizzes has an icon: one emoji and one looping microanimation.

## Contract (schema-first, TS + Rust twins)

- `Icon = { emoji (1…16 code points), motion }` and `Motion = bounce | pulse | spin | sway | float | flip` in `🧬️schema/🔣️.json`, `🟦️.ts`, `🦀️.rs` (renamed from `TaskIcon`, since it no longer belongs to tasks alone).
- Optional `icon` on the three task kinds, their `Sheet*Task` presentations and `CatalogTaskView`; on `Category`, `ClassificationItem`, `SortingItem`, `MatchingItem`, `SheetItem`; on `Dimension` and `SheetDimension`. Optional keeps the framework domain-neutral; the site's catalog test requires it everywhere.
- Sheet (`🃏️sheet`) copies item and dimension icons in both languages (categories travel whole); validation (`✅️validation`) checks emoji length and motion at every place in both languages.
- Radar axes carry no icon: they are neither questions nor answers.

## UI (`🎯️targets/⚛️react`)

- `IconLabel` (chrome) puts the glyph before a label; `Glyph` takes `motion` and `order`. Used by the classification chips and bins, the sorting rows, the matching dimension headings, value cards and rows, the result tables (items, assigned and correct categories, true order, dimension headings), the crowd figures (rows, category columns), the run's task list and the quiz page's task headings. `TaskGlyph` stays in the title chips.
- CSS: keyframes `quiz-icon-<motion>`, played only under `.quiz-app[data-icon-motion="on"]`; neighbours start `--quiz-icon-order × 0.7 s` apart so a list never moves in step.
- Device preferences `animateIcons` + `iconsChosen` (checkbox "Animate icons" / "Symbole animieren"): the learner's choice holds whatever the device asks for; until they choose, a device that asks for reduced motion keeps the icons still and the preferences say why (`effectiveIconMotion`, `withIcons`) — the same rule the pets follow. This matters here: the owner's Remote Desktop browsers report reduced motion, so a media-query-only gate would have left the icons still for them for good.
- The glyph is `aria-hidden`; labels and accessible names are unchanged.

## Content

133 icons in the four energy quizzes: 9 tasks, 109 items, 8 categories, 7 dimensions, written by `add_quiz_icons.py` (kept in this folder; the assignment table is its `ICONS` map). Rules followed: an icon pictures the thing and never its value or category (the electric-car battery is a car, not a battery, because the category "Energy" is a battery); the anonymous profiles A–F get neutral shapes; where a task has many alike things (six windows, four walls) they share the emoji and differ in motion — the site test forbids two items of a task that look and move alike.

## Tests

- Shared vectors (`generate_quiz_vectors.py`): new quiz `icon-demo` (icons on tasks, categories, items and one dimension; one item and one dimension deliberately without) with three sheets; task icons on `heating-systems`, `room-temperatures`, `first-pair`; seven rejected vectors (`unknown-icon-motion`, `empty-icon-emoji`, `icon-without-motion`, `undeclared-icon-member`, `empty-item-icon-emoji`, `unknown-category-icon-motion`, `undeclared-dimension-icon-member`) judged by python-jsonschema; the Python oracles of sheet assembly, learner lifecycle and the leaderboard catalog view carry the icons.
- `🧪️tests/🖼️task-icons` (React): glyph, fallback, every item/category/card icon of the `icon-demo` sheet, keyframes/gates/stagger, preference.
- Site catalog test: every task, item, category and dimension has an icon with a known motion, validated by ajv.

## Verified

Measured on 2026-10-02 after the extension to items, categories and dimensions:

- `@semio-tech/quiz` 297/297, Rust crate 107/107, `@semio-tech/quiz-react` 644/644 with a clean `tsc`, site 137/137.
- Parity of the quiz owner (python-jsonschema and Python references against TypeScript and Rust): 108/108. Taxonomy of the quiz owner clean.
- Browser (own proctor on 8793 with scratch data, own site on 6063, anonymous test learner): the quiz page's task list, the run's task buttons and title chips, the classification chips, the sorting rows, and the matching heading, value cards and rows all show their glyphs from the monochrome emoji face (including the Unicode 13 ones: 🪟 🪨 🫖); no console errors.
- The browser pane reports `prefers-reduced-motion: reduce`: the icons stood still and "Animate icons" showed unchecked with the note under it. After ticking it the root turned `data-icon-motion="on"`, the choice was stored (`animateIcons: true, iconsChosen: true`), the glyphs ran their animations (`quiz-icon-pulse/spin/bounce`, running; the spin glyph's computed transform took 5 values in 2.7 s) and the ten value cards of a matching started 0.7 s apart. `flip` was not on a screen that was looked at and is covered by the CSS test only. The default on a device that allows motion (animated without any choice) is covered by the unit test of `effectiveIconMotion`, not by a browser.
- Not checked in a browser: the results tables and crowd figures after a submitted run (covered by typecheck and the unchanged result/crowd tests, which carry no icons in their fixtures).

## Left as found

- The dev server another session runs on port 6061 answers 500 (`Failed to load PostCSS config … JSON Parse error`): a cached failure from a moment when a `package.json` on its search path was mid-write; both files parse now, the server needs a restart by its owner.
- `teaching-proctor` in `.claude/launch.json` cannot take another data directory or port, and the site's nx targets pin port 6061, so a second pair of servers was started straight from the scripts with `PROCTOR_PORT`/`PROCTOR_DATA`/`TEACHING_ARCHITECTURE_QUIZ_PORT` and stopped afterwards.

## Files

Created: `🧪️tests/🖼️task-icons/🟦️.tsx`, `add_quiz_icons.py`, this report.
Updated: quiz `🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs}`, `🔨️modules/{🃏️sheet,👁️views,✅️validation}` (TS + Rust + Rust unit tests, also `📏️scoring` unit tests), `README.md`, Python oracles `🧪️tests/{🃏️sheet-assembly,🧾️learner-lifecycle,🏆️leaderboard}/🐍️.py`, `generate_quiz_vectors.py` and the regenerated `🧫️fixtures` (`🃏️sheet-assembly`, `🧬️schema-conformance`, and those the earlier task-icon step touched), react target (`🟦️.tsx`, `🎨️.css`, `🔨️modules/{🪟️chrome,▶️run,🏁️results,📖️quiz-page,🗂️classification,↕️sorting,🃏️matching,🗳️crowd,🎛️preferences,🌐️i18n}`, `📦️packages/🟦️typescript/tsconfig.json`, `🧪️tests/🎚️config`), tests `🏠️home-grid`, `📡️presence-client`, `🐾️pet-companions` (label rename only), the four energy quizzes and the site `🧪️catalog` test.
