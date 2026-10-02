# Task icons with microanimations (2026-10-02)

Every task (question) of the four architecture quizzes now has an icon: one emoji and one looping microanimation.

## Contract (schema-first, TS + Rust twins)

- `TaskIcon = { emoji (1…16 code points), motion }` and `Motion = bounce | pulse | spin | sway | float | flip` in `🧬️schema/🔣️.json`, `🟦️.ts`, `🦀️.rs`.
- Optional `icon` on `ClassificationTask`, `SortingTask`, `MatchingTask`, their `Sheet*Task` presentations and `CatalogTaskView`. Optional keeps the framework domain-neutral; the site's catalog test requires it for every task of every quiz.
- Sheet (`🃏️sheet`) and views (`👁️views`) copy it in both languages (`iconOf` / `task.icon().cloned()`); validation (`✅️validation`) checks emoji length and motion in both languages.

## UI (`🎯️targets/⚛️react`)

- `TaskGlyph` replaces the kind icon in the title chip of the run, results and quiz-page task cards (falls back to the kind icon without an icon); `Glyph` takes `motion` and renders `data-motion`, hidden from assistive technology.
- CSS keyframes `quiz-icon-<motion>` are gated by `prefers-reduced-motion: no-preference` and `.quiz-app:not([data-icon-motion="off"])`.
- New device preference `animateIcons` (checkbox "Animate task icons" / "Aufgabensymbole animieren"), default on.

## Content

| Quiz | Task | Icon |
| --- | --- | --- |
| physics | power-or-energy | ⚡ pulse |
| physics | powers | ☀️ spin |
| physics | energies | 🔋 bounce |
| heating | u-values | 🧱 flip |
| heating | heating-load-and-demand | 🔥 float |
| cooling | air-change-rates | 🌬️ sway |
| cooling | cooling-load-and-demand | ❄️ spin |
| demand | standard-profiles | 🕸️ sway |
| demand | final-energy | 🔌 bounce |

## Tests

- Shared vectors (`generate_quiz_vectors.py`): icons on `heating-systems`, `room-temperatures`, `first-pair`; four rejected vectors (`unknown-icon-motion`, `empty-icon-emoji`, `icon-without-motion`, `undeclared-icon-member`) judged by python-jsonschema; the Python oracles of sheet assembly, learner lifecycle and the leaderboard catalog view carry the icon.
- `🧪️tests/🖼️task-icons` (React): glyph, fallback, keyframes/gates, preference. Site catalog test: every task has an icon with a distinct emoji and a known motion, validated by ajv.

## Verified

- `@semio-tech/quiz` quick tests 273/273, `@semio-tech/quiz-react` 423/423 (incl. 5 new).
- Parity of the quiz owner: 105/105 (the first run, 103/105, differed only in `schema-conformance::fixture-documents` while another session was regenerating fixtures; it passes in isolation and in the full re-run). Rust crate tests 98/98; site tests 40/40.
- Not verified in a browser: the dev proctor would not start (stale dev data, below), so the animations were checked only through the React component test and the CSS assertions, not visually.
- Pre-existing, not from this change: `tsc` errors in `🔨️modules/🛂️manifest`, `🖱️ui` contract and `📐️quantity-formatting` fixture typing; dev proctor data `.🧬semio/🎓️teaching/proctor-dev` is in sqlite format v1 while the proctor reads v2 (so `teaching-proctor` does not start on this machine until that dev data is removed).

## Files

Created: `🧪️tests/🖼️task-icons/🟦️.tsx`, this report.
Updated: quiz `🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs}`, `🔨️modules/{🃏️sheet,👁️views,✅️validation}` (TS + Rust + Rust unit tests), `README.md`, python oracles `🧪️tests/{🃏️sheet-assembly,🧾️learner-lifecycle,🏆️leaderboard}/🐍️.py`, `generate_quiz_vectors.py` and the regenerated `🧫️fixtures`, react target (`🟦️.tsx`, `🎨️.css`, `🔨️modules/{🪟️chrome,▶️run,🏁️results,📖️quiz-page,🎛️preferences,🌐️i18n}`, `📦️packages/🟦️typescript/tsconfig.json`, `🧪️tests/🎚️config`), tests `🏠️home-grid`, `📡️presence-client`, the four energy quizzes and the site `🧪️catalog` test.
