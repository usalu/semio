# Result CENSUS4-PKG-51 (prefix `c4p51`): product/quiz files split into cohesive children

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `Q` = `🧰️framework/🛍️products/❓️quiz`. Package 51 of `F/MONOLITH-CENSUS-4.md` section 4 (6 files, 6,861 lines, re-measured: identical). Tools in `F/c4p51-tools/`, raw outputs in `F/🗑️generated/c4p51-*`, pre-edit snapshots in `F/🗑️generated/c4p51-snap/` (`SHA.txt`).

## 0. Verdict

**DONE.** All six files are split, installed and verified; nothing is half applied. Every root is at most 381 lines and every new child at most 479 lines (limit about 1,000); 29 new files in 29 new folders, longest new path 130 UTF-8 bytes. Behaviour is unchanged where it can be run: quiz-react vitest has the identical 926-test status/title set as the baseline (925 passed, the same single pre-existing failure) plus 3 new tests; quiz core vitest 451/451 passed (11 files before, 14 after); the Python oracle gives 4/4 with 0 rows differing; `tsc` reports the same single pre-existing diagnostic; the bundled stylesheet is byte-identical to the original under lightningcss.

| # | Target (under `Q/`) | Before | Root after | Children (largest) | Mode |
|---|---|---:|---:|---|---|
| 1 | `🎯️targets/⚛️react/🎨️.css` | 1,417 | 18 (header plus 11 `@import`s) | 11 under `🎨️styles/` (`🧩️task-layouts` 300) | contiguous sections, original order kept |
| 2 | `🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts` | 1,048 | 103 | 3 (`🔡️german-bundle` 479) | module children: phrase constructor, English bundle, German bundle |
| 3 | `🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx` | 1,103 | 381 | 3 (`🔌️socket` 334) | module children by region (Socket, Rooms, View); React region stays |
| 4 | `🧪️tests/🧗️challenge-ladder/🟦️.ts` | 1,024 | 236 | 4 sibling test folders (`🗂️challenge-classification-hints` 237) | kit plus three `hintsOf` suites |
| 5 | `🧪️tests/🧾️learner-lifecycle/🐍️.py` | 1,113 | 155 | 4 python children (`🧗️challenge-and-sheet` 323) | python children loaded by `_load` |
| 6 | `🧪️tests/🪜️challenge-views/🟦️.tsx` | 1,156 | 288 | 3 sibling test folders (`⏱️challenge-clock-views` 350) | kit plus hint and clock suites |

Sum: 6,861 lines before; 1,181 in the six roots plus 5,841 in the 29 new files (7,022 in all: the added headers, `@import` list, hook cleanup copies and region repeats), all inside the limits.

## 1. Found state

- Fresh slice, no earlier `c4p51` folder or log. The quiz tree already carried peer edits (`M` in the index: the stylesheet facade `🎯️targets/⚛️react/📦️packages/🟦️typescript/🎨️stylesheet/🟦️.ts`, the ASSET-JS-EDGES test re-points, vitest config includes); I split the text as found.

## 2. Baselines (before the first edit)

| Check | Command | Result |
|---|---|---|
| quiz core vitest | `c4p51-tools/vitest.sh core base` (config `Q/🧪️tests/🎚️config/🟦️.ts`, `SEMIO_TEST_LEVEL=long`) | 11 files, 451 tests, 451 passed (`c4p51-vt-core-base.json`) |
| quiz-react vitest | `c4p51-tools/vitest.sh react base` (config `Q/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`) | 31 files, 926 tests, 925 passed, 1 failed (`🎪️pets-on-stage … the client carries the choice, fetches the site's menagerie…`), pre-existing |
| Python oracle | `c4p51-tools/pyrun.py` over the `🧾️learner-lifecycle` case (4 scenarios: registrations, learner-decisions, learner-views, site-catalog) | 4 passed |
| tsc | `c4p51-tools/tsc.sh base` (react tsconfig plus every quiz test and module tree, 136 quiz files) | 1 diagnostic (TS7016 `jsdom` declaration, `🚏️navigation`), pre-existing |

## 2b. Resume state (checked after the 17:10 reset)

Verified by `shasum -c` of `c4p51-snap/SHA.txt` (snapshots intact), `git status` and file reads: three batches are fully applied and nothing is half applied: the Python split (4 children), the i18n split (3 children, `export { QUIZ_BUNDLE_* }` list of the react root re-pointed) and the presence split (3 children, 11 importers re-pointed). CSS, challenge-ladder and challenge-views are untouched.

## 3. What was done, per file

Tools (`F/c4p51-tools/`): `gsplit.cjs` (c4p50 module splitter, plus a `childRoot` option that places children as sibling test folders), `pysplit.py` (c4p21 python splitter, extended for `from x import y` statements), `csssplit.py` (new: contiguous CSS sections, asserts the non-blank line sequence is unchanged), `fixfmt.py` (blank lines around region markers and after the header docstring), `lossless.cjs`, `pychk.py`, `cycles.cjs`, `cssequal.cjs`, `tsc.sh`, `vitest.sh`, `vtsum.py`, `pyrun.py`, `pycmp.py`; specs in `specs/`.

1. **CSS.** Eleven sections, each a contiguous run of the original with a one-line emoji header comment: `🔤️text-and-glyphs` (39), `🎞️icon-motion` (129), `🏠️chrome-and-home` (144), `🧩️task-layouts` (300), `🗃️record-tables` (107), `🏁️results-and-preferences` (151), `🤏️drag-and-spider` (68), `👥️peer-marks` (55), `📊️plots-and-crowd` (227), `⚖️footer-and-forced-colors` (72), `🎚️challenge-keys` (130), under `🎯️targets/⚛️react/🎨️styles/<folder>/🎨️.css`. The root keeps the design-system header and the ordered `@import` list (the package stub `📦️packages/🟦️typescript/🎨️.css` still imports the root). Readers re-pointed: the `@semio-tech/quiz-react/stylesheet` facade now joins the eleven `?raw` sections in import order (it exports a string as before); `🧪️tests/🖼️task-icons` read the file with `readFileSync` and now uses the facade; `🧪️tests/🗣️translation-completeness` globbed the root and now uses the facade; the nx `default` input of `📋️project.json` gained `🎨️styles/**/*`. New test `🧪️tests/🎨️stylesheet-tree` (3 tests) proves the root imports every section once, the facade equals the sections in that order and each section is self-contained, named by its folder emoji and under 1,000 lines; it is registered in the react vitest config and tsconfig.
2. **i18n.** `🏷️phrase` (6), `🔠️english-bundle` (478), `🔡️german-bundle` (479); the root keeps locale helpers, `QuizLabelKey`, the registration, `REJECTION_LABELS`, `TASK_KIND_LABELS` and `quizText`. The react root's `export { QUIZ_BUNDLE_DE, QUIZ_BUNDLE_EN … }` list was re-pointed to the owning children (the only importer that named them).
3. **Presence.** `🔌️socket` (334), `🧭️rooms` (242), `👁️view` (139); the root keeps the React region (provider, hooks, pointer tracking, overlay, status, list). 11 importers re-pointed (`PRESENCE_ANCHORS` now comes from `👁️view`, `QuizPresence` from `🧭️rooms`).
4. **challenge-ladder.** `🧱️challenge-ladder-kit` (124: sheet builders and independent oracle formulas), `💡️challenge-sorting-hints` (257), `🧲️challenge-matching-hints` (150), `🗂️challenge-classification-hints` (237), siblings under `Q/🧪️tests`; the root keeps rule table, points, reach, misses, taskSeconds, acted. Registered in `Q/🧪️tests/🎚️config/🟦️.ts`.
5. **learner-lifecycle (Python).** `🧗️challenge-and-sheet` (323: randomness, challenge ladder with hints and clock, sheet), `✅️answers-and-scoring` (263), `📇️handles-and-learner` (268), `🎞️replay-and-site` (175); the root keeps the module docstring, the loader, the four oracle handlers and the adapter registration. Import graph is a DAG (`sheet` ← `scoring` ← `learner` ← `site` ← root).
6. **challenge-views.** `🪞️challenge-views-kit` (295: fixtures, hint questions, helpers), `💬️challenge-hint-views` (242), `⏱️challenge-clock-views` (350); the root keeps the task-kind, time-up and results suites. Each suite file carries its own `afterEach` cleanup. Registered in the react vitest config and tsconfig.

## 4. Verification actually run

| Check | Command | Result |
|---|---|---|
| quiz-react vitest | `c4p51-tools/vitest.sh react final` (long level) | 34 files, 929 tests: 928 passed, 1 failed. Per-test (status, full title) list equals the baseline's 926 for 926; the 3 extra are the new `stylesheet-tree` tests. The one failure is the pre-existing `🎪️pets-on-stage … lets go of both when pets are switched off` (also failing in the baseline, `c4p51-vtset-react-base.txt`) |
| quiz core vitest | `c4p51-tools/vitest.sh core final` | 14 files, 451/451 passed; before 11 files, 451/451; status/title list identical |
| Python oracle | `pyrun.py` over the `🧾️learner-lifecycle` case, `pycmp.py` | 4/4 passed before and after, 0 rows differ |
| Python static | `pychk.py` | all files compile, no undefined global, 0 top-level statements lost (added: loader, region markers, child docstrings) |
| tsc | `c4p51-tools/tsc.sh final` (react tsconfig plus all quiz tests and module trees; 136 quiz files before, 150 after) | one diagnostic before and after: TS7016 `jsdom` types in `🚏️navigation` (pre-existing) |
| Lossless TS | `lossless.cjs` (log `c4p51-lossless.log`) | f2, f3, f4, f6: 0 lines lost; added only child docstrings, repeated region markers and (f6) the three `@see` lines plus the `afterEach` block copied into two suites |
| Lossless CSS | `csssplit.py` asserts the ordered non-blank line sequence; `cssequal.cjs` bundles the new root with lightningcss and minifies the original | sequence equal; 19,812 bytes both, identical |
| Region markers | `lossless.cjs` | none lost (f2 6 to 12, f3 8 to 8) |
| Import cycles | `cycles.cjs` over the 18 new TS files and their parents | 0 strongly connected components above size 1 |
| Taxonomy | `bun F/tf8-tools/taxsim.ts 🔣️taxonomy.json <29 new dirs>` (`c4p51-newdirs.txt`) | `validateTaxonomy` 0 problems; all 29 resolve (`crate-module-children`, `members-of-tests`, `presence`); no taxonomy edit needed. An `🇬🇧️` flag emoji folder did not resolve, so the bundles use `🔠️`/`🔡️` |
| Folder names, emoji | scripts | every folder is `<emoji>U+FE0F<kebab>`, every file starts with a docstring of its folder emoji, every emoji is already a taxonomy folder prefix; sibling emoji unique among the new tests folders (two clashes, `🧱️` and `💡️`, were renamed to `🪞️` and `💬️` before the final run; the old `📐️` duplicate in tests predates this slice) |
| Path length | `c4p51-newdirs.txt` run | longest 130 bytes |
| Debug logs | grep `[DEBUG]` over new TS | none |

## 5. Launch registration and hand-offs

- No new launch command is needed: the quiz tests run through the package routes (`bun ./📜️script.ts test`) whose vitest configs now list the new files; the Python case runs through the repo test host. `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` are unchanged by this slice.
- Not run: the repo test host (`semio_repo_test` coordinator) on the Python case (the single-adapter `pyrun.py` mirrors it, as in package 21), ESLint, a Storybook or browser render of the split stylesheet (the lightningcss bundle equality stands in for it).
- The Python children are loaded from the adapter's folder; if a future coordinator sandboxes only `🐍️.py` they must be declared as inputs.
- Pre-existing and not caused or fixed here: the `🎪️pets-on-stage` failure; the `jsdom` types diagnostic.

## 6. Files touched

Created (29): the files listed in `F/🗑️generated/c4p51-new-files.txt`. Rewritten roots: the six in the table. Edited: the react package root `🎯️targets/⚛️react/🟦️.tsx` (re-pointed `QUIZ_BUNDLE_*` export), 11 module importers of presence, `🎨️stylesheet/🟦️.ts`, `🖼️task-icons`, `🗣️translation-completeness`, `📋️project.json`, `tsconfig.json`, the two vitest configs. Snapshots and logs stay in `F/🗑️generated/c4p51-*` and tools in `F/c4p51-tools/`.
