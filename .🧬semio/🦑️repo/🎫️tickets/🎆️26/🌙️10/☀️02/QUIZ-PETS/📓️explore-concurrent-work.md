# Concurrent work in the tree while "Quiz Pets" starts (read-only exploration)

Scanned 2026-10-02 between 02:59 and 03:31 (host time) on top of `HEAD 48d881aa7ab` (2026-09-30). Read-only: nothing in the
repository was changed except this file. Path shorthands used below:

| Short | Path |
|---|---|
| `Q` | `🧰️framework/🛍️products/❓️quiz` (quiz product) |
| `RX` | `Q/🎯️targets/⚛️react` (react target; `RX/🔨️modules/<m>` = modules, `RX/🟦️.tsx` = package root, `RX/🎨️.css`) |
| `T` / `F` | `Q/🧪️tests` / `Q/🧫️fixtures` |
| `SCH` | `Q/🧬️schema` |
| `S` | `🎓️teaching/🏛️architecture/❓️quiz` (the architecture site) |
| `P` | `🎓️teaching/🛂️proctor` |
| `SV` | `🧰️framework/🛍️products/🖥️server` |
| `TAX` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` |
| `OT` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR` (the old, reopened-and-reused ticket) |

## 0. Decision-relevant facts (read this first)

1. **Everything is one huge uncommitted tree**: 243 `git status --short` entries at 03:00 (262 files at 03:33 with untracked
   directories expanded: 168 modified, 94 untracked; the untracked count was 89 at 03:00, so files are still being added). The whole 2026-09-28 quiz/proctor ticket plus the 2026-10-01/02 hardening
   round is uncommitted on top of `HEAD`, so `git diff` of a shared file shows other people's work, not just yours.
2. **An integration phase is running right now.** `test-e2e` (both topologies, ports 6161/8891 and 6162/8892) is running
   (third wave started 03:18-03:20), a `cargo test -p teaching-proctor` started 03:30:54, `parity/oracle exhaustive --owner Q`
   runs come and go (03:13, 03:19), and a dev site is on 6061. The plan in `OT/📓️deploy-readiness-plan.md` calls for "e2e gate
   (both topologies, 3x green), readiness gate on the final tree". The rehearsal topology **builds the site from the working
   tree**, so a half-written import in `RX/**` or `S/**` can fail somebody's gate run. Keep the tree compiling at every
   moment.
3. **Three workstreams overlap with yours on `RX/**`**, all uncommitted: (a) the **numeric guesses** ticket
   (`QUIZ-SORTING-NUMERIC-GUESSES`, open, actively editing), (b) the finished but unticketed-today **task icons with
   microanimations** (documented in `OT/📓️task-icons.md`, 03:18; it already added a `Motion` type, `Glyph motion`,
   `data-motion`, CSS `quiz-icon-*` keyframes, the `animateIcons` device preference and `data-icon-motion`), (c) the finished
   UI hardening round (`OT/📓️ui-hardening-report.md`). The icons feature is the nearest precedent for pets and must be
   followed, not duplicated (see section 7).
4. **Hot right now (edited within the last ~15 min of the scan, 03:03-03:19)**: proctor `🗄️storage` (Rust + its unit test),
   `SV/🔨️modules/📡️gateway` (Rust + unit test), `Q/🔨️modules/✅️validation` unit test, `T/✅️answer-validation/*`,
   `F/✅️answer-validation`, `F/👥️shared-presence`, `T/👥️shared-presence/🐍️.py`, and `OT/generate_quiz_vectors.py`
   (the shared fixture generator). None of these is in the way of a client-only pets feature, but **fixtures are
   regenerated wholesale** (30+ fixture files change in one second), so never put pet data into a shared fixture.
5. **The most contested files for you** are `RX/🟦️.tsx` (+214/-95 uncommitted, rewritten `QuizApp`), `RX/🎨️.css`
   (+189/-1), `RX/🔨️modules/🌐️i18n/🟦️.ts` (+204/-64), `RX/🔨️modules/🪟️chrome/🟦️.tsx` (+154/-12),
   `RX/🔨️modules/🎛️preferences/🟦️.tsx` (+50/-6), `SCH/{🔣️.json,🟦️.ts,🦀️.rs}` (+117, +70, +143) and `TAX` (+62/-2). `S/🔣️.json`/`S/🟦️.ts` are small and calm (last touched 01:08).
6. **Ports/processes**: 6061 is held by a long-running Vite **dev-site** (bun pid 8208, HMR on) with **no proctor behind it**
   (8791 is not listening); 6161/6162/8891/8892 belong to the e2e gate; no cargo/rustc was running at 03:16 but `cargo test`
   started at 03:30; rust-analyzer in Cursor (2.7 GB) is alive. Pick private ports for your own stack (suggestion: 6191/8921)
   and a scratch `PROCTOR_DATA`.

## 1. `git status` grouped by area

Counts are files (untracked directories expanded), M = modified, ?? = untracked.

| Area | Files | M / ?? | What is in it |
|---|---|---|---|
| Site `S` (`🎓️teaching/🏛️architecture/❓️quiz`) + 4 energy quiz files | 38 | 21 / 17 | deploy hardening (`🚀️deploy/*`, new `Dockerfile.dockerignore`, new `🚀️deploy/🧬️schema`), new e2e gate (`🎭️e2e/*`), local stack (`🧱️stack`), 11 new site test dirs, README (+338), vite config, `🌐️.html`, `🟦️.ts`, `🔣️.json` (catalog wording), the 4 `⚡️energy/*/❓️quiz/🔣️.json` (wording + task icons) |
| Proctor `P` | 21 | 20 / 1 | domain/edge Rust: `🎭️actors`, `🔭️projections`, `⌨️cli`, `🗄️storage`, `🎚️config`, `🧩️instance`, tests, `🧪️tests/🏋️capacity` (new), README, bootstrap launcher split |
| Quiz react target `RX` | 29 | 28 / 1 | UI hardening + icons + guesses: root `🟦️.tsx`, `🎨️.css`, 24 modules, new `⚖️legal` module, package.json/tsconfig/project.json, test config |
| Quiz tests `T` | 40 | 28 / 12 | 7 new react test dirs from UI hardening, `🖼️task-icons`, `🪪️identity-shapes` (4 hosts), many modified suites |
| Quiz fixtures `F` | 18 | 12 / 6 | regenerated en masse by `OT/generate_quiz_vectors.py`; 6 new fixture dirs |
| Quiz schema `SCH` | 4 | 4 / 0 | `🔣️.json`, `🟦️.ts`, `🦀️.rs`, Rust unit test: `Handle`, `IdentityClaim`, `Limits`, `Motion`, `TaskIcon`, `SortingAnswer.guesses` |
| Quiz core modules `Q/🔨️modules` | 15 | 15 / 0 | `✅️validation`, `🃏️sheet`, `👁️views`, `👥️presence`, `🧾️lifecycle`, `📏️scoring` (TS + Rust twins + unit tests) |
| Quiz other | 3 | 3 / 0 | `Q/README.md` (+142), `Q/🔮️oracles/🔣️.json` (colord, aria-query, i18next), `Q/📦️packages/🦀️rust/Cargo.toml` |
| Server `SV` | 21 | 18 / 3 | edge hardening: gateway, authority, storage, contract, tests, new `🚦️throttle` module and fixture |
| UI module `🧰️framework/🔨️modules/🖱️ui` | 3 | 3 / 0 | `🎨️styling/🏗️builder/🌐️vite/🟦️.ts` (host document per language), its test, `🧱️elements/🥞️LayeredOverview/🟦️.tsx` |
| Assets module | 1 | 1 / 0 | `🖼️assets/🏗️builder/📦️publication/🟦️.ts` (Windows path containment fix) |
| Repo infra | 1 | 1 / 0 | `TAX` (kinds `teaching-quiz-stack/e2e/e2e-learner`, ~25 new test/fixture dir names, module `⚖️legal`, contract `dockerfile-dockerignore`) |
| Editor/agent config | 3 | 3 / 0 | `.vscode/launch.json` (generated), `.vscode/🧩️launch.seed.jsonc` (source), `.claude/launch.json` |
| Root | 5 | 5 / 0 | `package.json` (+3 scripts), `bun.lock` (+6: aria-query, colord, i18next, types), `Cargo.lock` (+43), `.dockerignore`, `.github/workflows/architecture-quiz.yml` |
| Tickets `.🧬semio` | 54 | 5 / 49 | old ticket `OT` (scripts, audits, reports, notes) and today's `QUIZ-PETS`, `QUIZ-SORTING-NUMERIC-GUESSES` |

Root `Cargo.toml` is **not** modified (mtime 2026-09-30 12:01). The workspace member list is untouched; `Q/📦️packages/🦀️rust/Cargo.toml`
and `SV/📦️packages/🦀️rust/Cargo.toml` are (dev-dependencies for oracles).

## 2. Files by modification time (newest first, top 80)

Scan covered `Q`, `SV`, `🎓️teaching`, `🧰️framework/🔨️modules/🖱️ui`, `.vscode`, `.claude` and the four root files
(1525 files, `node_modules`/`target`/`dist` skipped), run at 03:18:46. Marks are relative to the scan start:
**H30** = within 30 minutes (since 02:49), **H2h** = within 2 hours (since 01:19), **TODAY** = since 00:00. A re-scan at
03:31 found 12 files in H30, 85 in H2h, 103 further ones earlier today and 1325 older (HEAD-state) files, so the tree
is quieting down but not quiet. `ts` = modification time on 2026-10-02.

| ts | Mark | Bytes | Path |
|---|---|---|---|
| 03:18:54 | H30 | 100858 | `F/👥️shared-presence/🔣️.json` (seen only in the 03:31 re-scan) |
| 03:17:05 | H30 | 22827 | `P/🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs` |
| 03:17:02 | H30 | 8413 | `T/👥️shared-presence/🐍️.py` |
| 03:14:02 | H30 | 50810 | `P/🔨️modules/🗄️storage/🦀️.rs` |
| 03:05:54 | H30 | 17317 | `F/✅️answer-validation/🔣️.json` |
| 03:05:16 | H30 | 1831 | `T/✅️answer-validation/🟦️.ts` |
| 03:05:10 | H30 | 68752 | `SV/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs` |
| 03:05:00 | H30 | 25065 | `Q/🔨️modules/✅️validation/🧪️tests/🔬️unit/🦀️.rs` |
| 03:04:25 | H30 | 143973 | `SV/🔨️modules/📡️gateway/🦀️.rs` |
| 03:04:22 | H30 | 2839 | `T/✅️answer-validation/🦀️.rs` |
| 03:04:10 | H30 | 2776 | `T/✅️answer-validation/🥒️.feature` |
| 03:03:59 | H30 | 5102 | `T/✅️answer-validation/🐍️.py` |
| 02:48:06 | H2h | 27497 | `Q/README.md` |
| 02:48:06 | H2h | 33930 | `P/🔨️modules/🧩️instance/🦀️.rs` |
| 02:46:26 | H2h | 3350 | `RX/🧪️tests/🎚️config/🟦️.ts` (vitest `include` list) |
| 02:46:26 | H2h | 2509 | `RX/📦️packages/🟦️typescript/tsconfig.json` (`include` list) |
| 02:45:41 | H2h | 21044 | `P/🔨️modules/🎭️actors/🧪️tests/🔬️unit/🦀️.rs` |
| 02:45:36 | H2h | 25484 | `T/⚖️partial-credit-scoring/🟦️.ts` |
| 02:45:20 | H2h | 39248 | `SV/🔨️modules/🎭️authority/🦀️.rs` |
| 02:45:19 | H2h | 23720 | `T/🗳️crowd-answers/🟦️.ts` |
| 02:44:48 | H2h | 7865 | `T/📐️quantity-formatting/🟦️.tsx` |
| 02:44:15 | H2h | 36981 | `SCH/🦀️.rs` |
| 02:43:53 | H2h | 3569 | `T/🖼️task-icons/🟦️.tsx` |
| 02:43:46 | H2h | 9061 | `SCH/🧪️tests/🔬️unit/🦀️.rs` |
| 02:43:40 | H2h | 30468 | `RX/🟦️.tsx` |
| 02:43:38 | H2h | 24143 | `T/⌨️task-keyboard/🟦️.tsx` |
| 02:43:35 | H2h | 12125 | `Q/🔨️modules/📏️scoring/🧪️tests/🔬️unit/🦀️.rs` |
| 02:43:30 | H2h | 44802 | `T/📡️presence-client/🟦️.tsx` |
| 02:43:27 | H2h | 41355 | `T/🏠️home-grid/🟦️.tsx` |
| 02:42:21 | H2h | 10084 | `F/📐️quantity-formatting/🔣️.json` |
| 02:41:37 | H2h | 15473 | `RX/🔨️modules/🏁️results/🟦️.tsx` |
| 02:41:37 | H2h | 41128 | `RX/🔨️modules/🌐️i18n/🟦️.ts` |
| 02:41:35 | H2h | 2666 | `S/🧪️tests/🧪️catalog/🟦️.ts` |
| 02:41:28 | H2h | 25004 | `S/README.md` |
| 02:41:22 | H2h | 16295 | `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/❓️quiz/🔣️.json` |
| 02:41:22 | H2h | 18627 | `.../⚡️energy/❄️cooling/❓️quiz/🔣️.json` |
| 02:41:22 | H2h | 22860 | `.../⚡️energy/🔥️heating/❓️quiz/🔣️.json` |
| 02:41:21 | H2h | 28019 | `.../⚡️energy/🧲️physics/❓️quiz/🔣️.json` |
| 02:41:15 | H2h | 18859 | `Q/🔨️modules/👥️presence/🧪️tests/🔬️unit/🦀️.rs` |
| 02:41:08 | H2h | 11860 | `RX/🔨️modules/↕️sorting/🟦️.tsx` |
| 02:41:07 | H2h | 11020 | `RX/🎨️.css` |
| 02:41:00 | H2h | 11062 | `Q/🔨️modules/👥️presence/🦀️.rs` |
| 02:40:54 | H2h | 13165 | `RX/🔨️modules/▶️run/🟦️.tsx` |
| 02:40:53 | H2h | 11109 | `RX/🔨️modules/📖️quiz-page/🟦️.tsx` |
| 02:40:48 | H2h | 14516 | `RX/🔨️modules/🪟️chrome/🟦️.tsx` |
| 02:40:35 | H2h | 28166 | `Q/🔨️modules/✅️validation/🦀️.rs` |
| 02:40:24 | H2h | 10645 | `RX/🔨️modules/🎛️preferences/🟦️.tsx` |
| 02:40:09 | H2h | 17069 | `T/🧾️learner-lifecycle/🦀️.rs` |
| 02:40:09 | H2h | 22491 | `Q/🔨️modules/🧾️lifecycle/🧪️tests/🔬️unit/🦀️.rs` |
| 02:40:09 | H2h | 20539 | `Q/🔨️modules/👁️views/🧪️tests/🔬️unit/🦀️.rs` |
| 02:39:14 | H2h | 10333 | `RX/🔨️modules/📏️quantity/🟦️.ts` |
| 02:37:33 | H2h | 39725 | `Q/🔨️modules/✅️validation/🟦️.ts` |
| 02:36:49 | H2h | 75384 | `F/📊️crowd-view/🔣️.json` |
| 02:36:48 | H2h | 49087 | `F/🧬️schema-conformance/🔣️.json` |
| 02:36:48 | H2h | 223393 | `F/🏆️leaderboard/🔣️.json` |
| 02:36:47 | H2h | 329506 | `F/🧾️learner-lifecycle/🔣️.json` |
| 02:36:46 | H2h | 96136 | `F/🏅️badge-rules/🔣️.json` |
| 02:36:46 | H2h | 95567 | `F/🕸️profile-similarity/🔣️.json` |
| 02:36:45 | H2h | 89909 | `F/📏️sorting-concordance/🔣️.json` |
| 02:36:45 | H2h | 100411 | `F/🃏️sheet-assembly/🔣️.json` |
| 02:36:12 | H2h | 39412 | `T/🧾️learner-lifecycle/🐍️.py` |
| 02:36:04 | H2h | 23433 | `SCH/🟦️.ts` |
| 02:35:59 | H2h | 11316 | `T/🏆️leaderboard/🐍️.py` |
| 02:35:59 | H2h | 4927 | `T/🃏️sheet-assembly/🐍️.py` |
| 02:35:33 | H2h | 50576 | `SCH/🔣️.json` |
| 02:35:04 | H2h | 9533 | `Q/🔨️modules/🃏️sheet/🧪️tests/🔬️unit/🦀️.rs` |
| 02:34:30 | H2h | 57464 | `SV/🟦️.ts` |
| 02:34:23 | H2h | 18143 | `SV/🔨️modules/🧬️contract/🦀️.rs` |
| 02:34:12 | H2h | 13703 | `Q/🔨️modules/👁️views/🦀️.rs` |
| 02:34:12 | H2h | 4283 | `Q/🔨️modules/🃏️sheet/🦀️.rs` |
| 02:34:09 | H2h | 3556 | `Q/🔨️modules/🃏️sheet/🟦️.ts` |
| 02:34:04 | H2h | 11010 | `Q/🔨️modules/👁️views/🟦️.ts` |
| 02:33:42 | H2h | 45300 | `P/🧪️tests/🏋️capacity/🟦️.ts` |
| 02:32:40 | H2h | 29578 | `S/🎭️e2e/🚶️learner/🟦️.ts` |
| 02:32:17 | H2h | 4502 | `RX/🔨️modules/🤏️drag/🟦️.ts` |
| 02:17:51 | H2h | 4912 | `S/🧪️tests/🏆️live-leaderboard/🟦️.ts` |
| 02:17:20 | H2h | 11101 | `S/🧪️tests/🗣️both-languages/🟦️.ts` |
| 02:13:52 | H2h | 5169 | `S/🧪️tests/🪪️first-visit/🟦️.ts` |
| 02:12:15 | H2h | 20663 | `P/🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs` (older version of the storage test) |
| 02:12:14 | H2h | 9577 | `S/🧪️tests/🧱️local-stack/🟦️.ts` |
| 02:07:18 | H2h | 24040 | `P/🔨️modules/⌨️cli/🦀️.rs` |
| 02:07:17 | H2h | 22351 | `P/🔨️modules/🔭️projections/🦀️.rs` |
| 02:07:09 | H2h | 23272 | `P/🔨️modules/🎭️actors/🦀️.rs` |
| 02:05:58 | H2h | 9489 | `T/🪪️identity-shapes/🐍️.py` |
| 02:03:59 | H2h | 39495 | `P/README.md` |
| 02:02:58 | H2h | 8678 | `S/🏗️builder/🌐️vite/🟦️.ts` |
| 02:00:17 | H2h | 19017 | `S/🎭️e2e/🟦️.ts` |
| 02:00:02 | H2h | 2800 | `S/🎭️e2e/🎚️config/🟦️.ts` |
| 01:40:57 | H2h | 158367 / 687083 | `.vscode/🧩️launch.seed.jsonc` / `.vscode/launch.json` (generated from the seed) |

Not in the top 80 but relevant: root `package.json` 00:33:25, `bun.lock` 00:53:23, `Cargo.lock` 01:01:19 (all TODAY),
`.claude/launch.json` 00:16:34 (TODAY), `TAX` 01:30:02 (TODAY, i.e. **not yet touched for `🖼️task-icons` or `🏋️capacity`**),
root `Cargo.toml` 09-30 (untouched). Notes outside the scan: `OT/generate_quiz_vectors.py` 03:17:02,
`OT/📓️task-icons.md` 03:18:58, `OT/🎫️ticket.json` 03:15:43, `QUIZ-SORTING-NUMERIC-GUESSES/parity3.txt` 03:19:10.

Reading the pattern: a very large burst at **02:32-02:48** (guesses + task icons across schema, both cores, fixtures, react
target, site content, READMEs), then a Rust/test/fixture tail at 03:03-03:19. Nobody touched `RX/**` after 02:46 as of the
scan, but both open workstreams still have verification loops running.

## 3. Today's other ticket: `QUIZ-SORTING-NUMERIC-GUESSES` (open)

Files in the folder: `🎫️ticket.json` (02:33, goal `🎯runningframework🎯runningproducts`, status open, no client/llm entry),
`📓️design.md` (02:47), `before.sha`/`after.sha`/`before2.sha`/`after2.sha` (hash lists of all fixtures before/after
regenerating them; 3165 bytes each), `oracle1.txt`, `parity1.txt`, `parity2.txt`, `parity3.txt` (outputs of the repo test
harness).

**What it changes** (from `📓️design.md`; every item confirmed present in the working tree by grep/diff):

- Schema (schema-first, both twins plus a Python reference): `SortingAnswer` gains optional
  `guesses: { <item id>: number }` in the quantity's **base unit** (never SI-prefixed); keys are sheet items, values finite
  (positive on a logarithmic scale); guessed items must appear in `order` in non-decreasing guess order. `ThinkingAnswer`
  (crowd draft in `Q/🔨️modules/👥️presence`) may carry `guesses` too (finite, at most `THINKING_LIMIT`). Guesses never enter
  scoring.
- Core: `✅️validation` (TS, Rust, Rust unit tests), `👥️presence` (Rust + unit tests), scoring/lifecycle/views unit tests.
- React target: `RX/🔨️modules/↕️sorting` (per-item "Guess for <item>" field, preview `= 500 kg`, commit on Enter/blur,
  reorder by guess among occupied places, hand move drops the guess), `RX/🔨️modules/📏️quantity` (new `parseQuantity`, inverse
  of `formatQuantity`), `RX/🔨️modules/🏁️results` ("Your guess" column), `RX/🔨️modules/🌐️i18n` (23 `guess` hits: both
  languages), `RX/🟦️.tsx` (re-exports `parseQuantity`, `ordered`), possibly `RX/🎨️.css` and `🧩️task`.
- Tests/fixtures: `T/📐️quantity-formatting` + `F/📐️quantity-formatting` (`parses` vectors, `Intl.NumberFormat` oracle),
  `T/⌨️task-keyboard`, `T/🗳️crowd-answers` (ajv), `T/⚖️partial-credit-scoring`, `T/✅️answer-validation/{🐍️.py,🥒️.feature,🦀️.rs,🟦️.ts}` +
  `F/✅️answer-validation` (30 `guess` hits), `T/🧾️learner-lifecycle`.
- It regenerates **all** shared vectors through `OT/generate_quiz_vectors.py` (103 KB, last written 03:17) and verifies with
  `./📜️script.ts parity exhaustive --owner 🧰️framework/🛍️products/❓️quiz` (105 cases/executions). State at 03:15: 105/105
  parity (an earlier 103/105 was a transient from another session regenerating fixtures); `oracle exhaustive` at 03:00 showed
  34/35 passed with 1 failing, being worked on at 03:17-03:19 (`T/👥️shared-presence/🐍️.py`, `F/👥️shared-presence`).

**Files it owns or will keep touching** (do not edit these for pets): `SCH/*`, `Q/🔨️modules/{✅️validation,👥️presence,🃏️sheet,👁️views}/*`,
`RX/🔨️modules/{↕️sorting,📏️quantity,🏁️results}/*`, `T/{✅️answer-validation,📐️quantity-formatting,🗳️crowd-answers,⚖️partial-credit-scoring,⌨️task-keyboard,👥️shared-presence}/*`,
`F/{✅️answer-validation,📐️quantity-formatting,👥️shared-presence}/*`, `OT/generate_quiz_vectors.py`, and shared lines in
`RX/🔨️modules/🌐️i18n/🟦️.ts`, `RX/🟦️.tsx`, `RX/🎨️.css`.

## 4. Old ticket `OT`: workstreams, ownership, sharing rules, broken states

Coordination plan (`📓️deploy-readiness-plan.md`, 00:08): five parallel agents, now all reported. Note files between agents:
`🗒️edge-needs-from-domain`, `🗒️domain-needs-from-edge`, `🗒️domain-notes-for-ui`, `🗒️ui-notes-for-{e2e,deploy,domain}`,
`🗒️deploy-notes-for-domain-and-edge`, `🗒️edge-notes-for-deploy`.

| Workstream | Owns | Report / state |
|---|---|---|
| **ui** | `RX/**`, react tests and fixtures (new dirs `🌍️language-choice`, `📢️live-regions`, `🌗️contrast-states`, `📇️learner-pages`, `🚦️rate-limits`, `🎭️identity-step`, `🔏️privacy-notice`), site shell (`S/🟦️.ts`, `S/🌐️.html`, `S/🔣️.json`, `S/🏗️builder/🌐️vite/🟦️.ts` shared with deploy), `🥞️LayeredOverview`, host-document template in the ui styling vite builder | `📓️ui-hardening-report.md`: all D/I items fixed; react suite 364 tests then; **final** |
| **domain** | quiz cores (TS + Rust), `SCH/*`, conformance cases, proctor `🎭️actors/🔭️projections/❓️queries/⌨️cli` | `📓️domain-hardening-report.md`; `cargo test -p teaching-proctor` was 74 + 15 + 13 green at its time; **final** |
| **edge** | `SV/**`, proctor `🧩️instance`, `🎚️config`, storage receipts, `🏋️capacity` | no `edge-hardening-report.md` exists; notes only; its `🏋️capacity` dir is **not registered in `TAX`** (reported as `directory-kind-unresolved`, a failing `verify taxonomy --scope 🎓️teaching`) |
| **deploy** | `S/🚀️deploy/*`, workflow, `.dockerignore`, readiness gate `deploy-check` | `📓️deploy-readiness-report.md`: gate has result placeholders (`GATE_RESULT`, `TEST_RESULT`...), i.e. not finalised; a long `deploy-check` run started 02:47-02:49 is no longer running at 03:30 |
| **dev-e2e** | `S/🧱️stack`, `S/🎭️e2e/**`, nine site test dirs, launch rows `dev`, `dev-site`, `test-e2e`, the `drag` ghost fix | `📓️dev-e2e-report.md`: `RESULTS-PENDING`; the gate is being run now (3x green is the plan) |
| **icons** (unticketed today, `OT/📓️task-icons.md`) | schema `TaskIcon`/`Motion`, `Glyph`/`TaskGlyph`, CSS keyframes, `animateIcons` preference, catalog icons in the 4 energy quiz files | **finished** 03:18; quiz-react 423/423, parity 105/105, site 40/40 |

**Sharing rules and conventions stated in the notes**

- "I rewrite exactly those places now and parallel edits would collide" (ui to domain): owners announce a whole path set
  and others stay out of it; others make **"small anchored edits"** in a foreign file and tell the owner (domain in
  `🧩️instance`/`🎚️config`).
- "The site's vite config is edited by both of us; my edits are small and anchored on the `semioHostHtmlVitePlugin({ ... })`
  spec and on `siteTitle`."
- **Host trap** (`📓️deploy-readiness-report.md` O7, `📓️conformance-report.md`): a file loaded by a running bun process cannot be
  truncated on Windows; `writeFileSync` leaves the old tail and Python's `open(..., "w")` fails with `Errno 22`; it corrupted
  two files for three minutes. **"Source files are to be changed with the editor tools only."** Same lesson in
  `MEMORY.md` (Windows loaded file no truncate).
- Registrations belong to one hand: "your new test directory needs its entry in `TAX`... I did not add it, so that we do
  not both add it" (domain to edge).
- New test/fixture directory names need a **non-generic emoji**: `📄️` is refused by the harness, hence `📰️host-document`;
  sibling directories must not share an emoji (`👥️presence-client` was renamed `📡️presence-client`).
- Test-oracle libraries are dev dependencies of the react package only and are registered in `Q/🔮️oracles/🔣️.json`
  (colord, aria-query, i18next next to lightningcss/lodash/ajv). Every feature needs a language-agnostic test plus a
  third-party oracle (AGENTS.md).
- Never delete another agent's ticket `🗑️generated/` (an agent once wiped the shared folder).
- Two gate runs on the same fixed ports collide (6161/8891, 6162/8892); the second fails immediately.
- `bun nx ...` sometimes fails to start under load (plugin worker timeout): `NX_PLUGIN_NO_TIMEOUTS=true` avoids it.
- Dev proctor data `.🧬semio/🎓️teaching/proctor-dev` is storage format v1 and the proctor reads v2: `dev` fails until it is
  removed or `PROCTOR_DATA` points at a scratch folder (the folder was touched at 02:49; unverified whether it is v2 now).
  Use a private `PROCTOR_DATA` as the ui agent did (site 6071, proctor 8801).
- `i18n` rules from the UI work (pinned by tests that will fail on a lazy pets change): **no default language**, English first
  then German, informal "du", key sets of both bundles identical (`🗣️translation-completeness` scans sources: counts are
  "Label: n" with no plural forms, one German word per concept, `role="list"` on every `list-none` list, glyph-only symbols
  must be `aria-hidden` plus a word), nothing in a sentence may carry an English `{{detail}}`.

**Currently reported broken or open state** (all pre-existing for pets, but they colour what "green" means)

- `:typecheck` of the react package exits 1 (133 errors, 0 in the quiz target) because git-ignored generated bindings
  (`🛂️manifest`, `🖱️ui/🧬️contract/🧵️retained`, `🎭️actor/🤖️generated`) are stale; the quiz target itself is clean. Icons note adds:
  `tsc` errors in `📐️quantity-formatting` fixture typing (pre-existing).
- `ui-react:test-exhaustive`: 17 failing tests (subset of a 20-failure baseline), none in `LayeredOverview`.
- Taxonomy: `verify taxonomy report --scope 🎓️teaching` has at least one error (`🏋️capacity` unregistered); `🖼️task-icons`
  is also missing from `TAX` (TAX last written 01:30).
- The site has no typecheck target; ad-hoc `tsc` finds two errors in site files.
- Browser verification of the icons animation was **not** possible (dev proctor would not start); pets will want a real
  browser check, hence a private proctor with scratch data.

## 5. Running processes and ports (snapshots 03:16 and 03:30)

| What | Detail |
|---|---|
| 6061 | `bun` pid 8208 = Vite dev server of the site (`dev-site`, started 02:46:47, HMR/watch **on**, config loaded by bun `--configLoader native`), parents `bun ./📜️script.ts dev-site` (23804) and `bun nx run @teaching/architecture-quiz:dev-site` (55128, 53360, node 47756). `.claude/launch.json` row `architecture-quiz-site` points here, so `preview_start` would reuse it |
| 8791 | **not listening**: that dev site has no proctor |
| 6161 / 8891 | e2e gate, dev topology: vite (bun) + `proctor-<pid>-<guid>.exe` from `.🧬semio/🎓️teaching/proctor-bin/` |
| 6162 / 8892 | e2e gate, rehearsal topology: static origin (bun pid of `test-e2e`) + proctor in production mode |
| e2e gate | bun `📜️script.ts test-e2e` (pids 24924 at 03:05, 56812 at 03:19:58), `bun nx run @teaching/architecture-quiz:test-e2e` (49956 / 57820 / node 44352, started 03:18:09), Playwright `cli.js test` and `workerProcessEntry.js` workers (new ones at 03:30) |
| parity / oracle | `bun ./📜️script.ts parity exhaustive --owner 🧰️framework/🛍️products/❓️quiz` (56360 at 03:13) and `oracle exhaustive` (56832 at 02:59), each spawning python hosts from `.🧬semio/🦑️repo/⚡️cache/tests/hosts/python-env-*` |
| cargo | none at 03:16; **`cargo test -p teaching-proctor` (pids 25696, 11404) started 03:30:54**; an earlier cargo (55572) appeared at 02:59:19. Concurrent cargo builds contend for the target dir lock |
| rust-analyzer | Cursor's rust-analyzer (pid 22640, ~2.7 GB since 02:23) + 2 proc-macro servers; its flycheck can hold the build-dir lock too |
| editors | Cursor with the Vitest explorer (3 node workers since 02:25), cursorpyright (node 57676); stale Playwright `test-server` (node 58340, from a March ticket config, `127.0.0.1`, no port in the listing) |
| other | Docker Desktop is running (backend since 01:37); Claude Code sessions alive: pid 41912 (opus-5-5, since 22:23), 24200 and 28352 (sonnet-5-5, 02:27/02:28), 55824 (opus-5-5, 02:52, most likely this coordinator). Which session owns which workstream is not recorded |

Files that running **bun** processes have loaded (do **not** rewrite them with a script; use the editors' Edit): the site's vite
config (`S/🏗️builder/🌐️vite/🟦️.ts`) and the shared ui styling vite builder, `S/📦️packages/🟦️typescript/📜️script.ts`,
`P/🏗️bootstrap/🟦️.ts`, `S/🎭️e2e/**`, `S/🧱️stack/🟦️.ts`, the repo library bootstrap scripts, Playwright specs under `S/🧪️tests/**`
while the gate runs. Vite itself reads `RX/**` per request (not held open), but HMR on 6061 reloads whatever is open there on every save.

## 6. Hot-spot files your task will touch

Time = last write (2026-10-02 unless stated); `diffstat` is `git diff --stat` against `HEAD`; the diff nature says whose work it is.

| File | Last write | Size (bytes) | Uncommitted diff | Contested? |
|---|---|---|---|---|
| `RX/🟦️.tsx` | 02:43:40 | 30468 | +214/-95: `QuizApp` split into `ClientProps`/client views, `documentTitle`, `ConnectionStatus` announcer, `connectionState`, legal footer, language chooser, many new re-exports (`parseQuantity`, `TaskGlyph`, `Dialog`, `ProblemNote`, `HOME_PAGES`...), `QuizOptions.legal` | **very high** (ui round + icons + guesses). Add pets with one export line, one option (`pets?`) and one mount element; do not restructure |
| `RX/🎨️.css` | 02:41:07 | 11020 | +189/-1: icon motion keyframes + gates (`quiz-icon-*`, `data-icon-motion`), forced-colors rules, peer ink, focus outlines | **high**; append a clearly delimited `/* 🐾 ... */` block at the end |
| `RX/🔨️modules/🌐️i18n/🟦️.ts` | 02:41:37 | 41128 | +204/-64: ~80 new keys in `QUIZ_BUNDLE_EN` and the German bundle, neutral language chooser, `applyLocale(undefined)` | **very high**; both bundles are edited by several people. Add a contiguous `pets: { ... }` group in each bundle |
| `RX/🔨️modules/🎛️preferences/🟦️.tsx` | 02:40:24 | 10645 | +50/-6: `animateIcons` preference, `EveryLanguage`, `everyLanguage`, `LanguageChoice` | high; a `pets` flag mirrors `animateIcons` (read at line ~61, written ~48, checkbox ~152) |
| `RX/🔨️modules/🪟️chrome/🟦️.tsx` | 02:40:48 | 14516 | +154/-12: `Dialog`, `Mark`, `Missing`, `ProblemNote`, `Glyph motion`, `role="list"`, `BodyButton` wrapping | high; avoid, pets are not chrome |
| `SCH/🔣️.json` | 02:35:33 | 50576 | +117/-24: `Handle`, `Motion`, `TaskIcon`, `icon` on 6 task types, `SortingAnswer.guesses`, `Identity` oneOf | high; **do not touch** (a schema field for pets would force fixtures + 3 twins + generator changes) |
| `SCH/🟦️.ts` | 02:36:04 | 23433 | +70/-17: `Handle`, `MOTIONS`, `TaskIcon`, `IdentityClaim`, `Limits`, `REJECTIONS` reflowed | high; do not touch |
| `SCH/🦀️.rs` | 02:44:15 | 36981 | +143/-21: `Motion`, `TaskIcon`, `Handle`, `guesses` on `SortingAnswer` (derive lost `Eq`) | high; do not touch |
| `S/🔣️.json` (site catalog) | 01:08:08 | 7107 | +14/-14: wording only | **low** (quiet since 01:08), but the catalog JSON schema is strict; pets must not add keys here |
| `S/🟦️.ts` (site entry) | 01:08:29 | 1616 | +5/-3: `legal: site.legal` passed to `mountQuiz` | **low**; the natural place to pass `pets` (domain-specific extension) as an option |
| root `package.json` | 00:33:25 | 24102 | +3: scripts `dev:...:site`, `test:...:e2e`, `check:...:deploy` | low; only edit if you add a script (AGENTS.md says scripts go to `📜️script.ts`, `package.json` only calls nx) |
| root `Cargo.toml` | 09-30 12:01:28 | 72489 | none | untouched; not needed for pets |
| `.vscode/🧩️launch.seed.jsonc` | 01:40:57 | 158367 | +122/-10: dev/health/backup/restore/erase/gate rows, compound removed | medium; source of `.vscode/launch.json` |
| `.vscode/launch.json` | 01:40:57 | 687083 | +149/-27 (generated from the seed) | medium; **never hand-edit**; regeneration rewrites a 687 KB file |
| `.claude/launch.json` | 00:16:34 | 13074 | +15/-1: `architecture-quiz-site` (6061, `PROCTOR_PORT` 8791) and `architektur-und-technologie-quizze` | low |
| `TAX` | 01:30:02 | 1152260 | +62/-2: stack/e2e kinds, ~25 test dir names in two lists (about lines 10927-10940 and 15586-15625) and the module `⚖️legal` (line ~12209), contract `dockerfile-dockerignore` | **medium-high**; 1.1 MB, single hand per edit, already behind (task-icons, capacity missing) |

Other shared registration points a new `🐾` module/test needs:

- react vitest includes: `RX/🧪️tests/🎚️config/🟦️.ts` (explicit list, last edited 02:46:26) and `RX/📦️packages/🟦️typescript/tsconfig.json`
  (explicit list). `RX/📦️packages/🟦️typescript/📋️project.json` inputs were turned into globs (`🧪️tests/*/🟦️.tsx`,
  `🧫️fixtures/*/🔣️.json`), so no edit needed there.
- `TAX`: tests list, fixtures list, modules list (a `🐾️pets` module name needs an entry; the emoji `🐾` is used for a `trail`
  member in unrelated lists of other products, no sibling clash found among the quiz react modules).
- `Q/README.md` (27 KB, edited 02:48): the Domain model section already documents icon motion (lines ~79-80); a pets section
  belongs next to it and `S/README.md` (25 KB, 02:41) for the site-specific pet cast.

## 7. Recommended rules for the pets agents

**Never clobber**

1. Re-`Read` a shared file **immediately before every `Edit`**, never edit from an older read (the 02:32-02:48 burst shows files
   changing within seconds of each other). If the `Edit` fails on a mismatch, re-read and redo it; never "fix" by `Write`.
2. Small, targeted `Edit`s with unique multi-line anchors (e.g. the lines around the `animateIcons` field) in shared files;
   **never `Write` a whole shared file** and never rewrite a file that has uncommitted changes of others (all 150+ listed above).
3. No scripted rewrites (no `bun -e`, Python `open("w")`, sed -i, regeneration scripts, formatters, `eslint --fix`, `prettier`,
   `cargo fmt`) over anything under `RX`, `Q`, `S`, `SV`, `P`, `.vscode`, `TAX`: running bun processes hold files on this host
   and a scripted rewrite leaves stale tails or fails; formatters reformat other people's pending hunks. Create new files with
   `Write` only when the path does not exist.
4. Prefer **new files** over edits: put pets into a new react module (suggested `RX/🔨️modules/🐾️pets/🟦️.tsx` plus pure
   logic in `🟦️.ts` beside it), new tests (`T/🐾️pets/...`) and new fixtures (`F/🐾️pets/...`), and wire them with a minimal
   number of one-line anchored edits in `RX/🟦️.tsx` (export + option + mount), `RX/🎨️.css` (one appended delimited block),
   `RX/🔨️modules/🌐️i18n/🟦️.ts` (one contiguous `pets` group per bundle), `RX/🔨️modules/🎛️preferences/🟦️.tsx` (the `pets`
   flag next to `animateIcons`), `RX/🧪️tests/🎚️config/🟦️.ts` + `tsconfig.json` (one include line each), `TAX` (module, test,
   fixture names), READMEs (one section each).
5. Do **not** touch the quiz schema (`SCH/*`), the cores (`Q/🔨️modules/*`), `OT/generate_quiz_vectors.py`, any `F/<existing>` or
   `T/<existing non-react>` file, `SV/**`, `P/**` or deploy files. Pets are a client concern; the architecture site's cast
   (sunny, cloudy, housy, solary, radiatory, pumpy, windowy, waly, battery...) is **site-specific**, so it belongs next to
   `S/🟦️.ts` (e.g. a `S/🐾️pets/🟦️.ts`-style definition passed to `mountQuiz(..., { pets })` as `legal` is) while the framework
   keeps a domain-neutral `PetDefinition` seam. This also keeps `S/🔣️.json` (strict catalog schema) untouched.
6. Register in `TAX` with **one** agent doing it in one batch at the end (the file is 1.1 MB and already missing entries for
   `🖼️task-icons` and `🏋️capacity`). Do not fix their omissions as a side effect unless coordinated; mention them to the
   coordinator instead.

**Keep other people's gates green**

7. Keep the tree compiling at every save: land the new module and its tests first, and the wiring edit in `RX/🟦️.tsx` last,
   in a single step. The e2e rehearsal topology builds the site from the working tree; a failing import breaks a 3x-green run.
8. Pets must be `pointer-events: none` and `aria-hidden` (decorative), must not intercept clicks or drags (the e2e `dragInto`
   follows `quiz-drop-active` and Playwright refuses clicks under overlays), must not move layout (the e2e notes already flag
   layout shifts from the crowd line), must not log to the console, create `<style>`/`<script>`, call `setAttribute("style")`
   or fetch anything (the e2e device guard fails on console errors, failed requests and CSP violations; CSP is
   `style-src 'self'`+hash with `style-src-attr 'unsafe-inline'`, `img-src 'self' data:`). Inline SVG or hashed bundled assets only;
   bundle budget: entry script 260 kB gzip, stylesheet 60 kB gzip, 4 MB total (2.13 MB now).
9. Follow the icon precedent: gate all motion by `@media (prefers-reduced-motion: no-preference)` and a device preference
   (`pets`, default on, "off" attribute on `.quiz-app`, e.g. `data-pets="off"`), persisted through `readPreferences/writePreferences`.
   Pause animation when the document is hidden and when idle; the e2e notes measured that a driven page already keeps a core
   busy (4 workers max on this 16-thread host), so prefer CSS animations and a throttled timer over free-running
   `requestAnimationFrame`.
10. Every visible string and accessible name in **both** languages (English first, then German, "du"), no default language,
    identical key sets, counts as "Label: n"; the stylesheet must stay parseable by lightningcss (`🌗️contrast-states` parses it
    and requires a forced-colors rule for every stateful selector).
11. Tests: pure behaviour logic (cursor-follow gaze, blink/fidget scheduling, walking on element rects, pair interactions) as a
    deterministic function with language-agnostic JSON vectors in a **new** `F/🐾️...` fixture, a vitest in a **new**
    `T/🐾️.../🟦️.tsx`, and a third-party oracle (AGENTS.md); never put pet cases into `✅️answer-validation`, `🧾️learner-lifecycle` or other
    multi-host cases, since the `parity/oracle exhaustive --owner Q` loops that other agents run would pick them up and
    expect all hosts. Test dir emoji must be specific (not a generic one like `📄️`) and unique among siblings.

**Operations**

12. Do not run `cargo` (pets are TypeScript only; `cargo test -p teaching-proctor` is running and builds contend for the target
    lock, rust-analyzer is also active), do not start `test-e2e`, `deploy-check`, `parity` or `oracle` runs while the other gates
    are in flight (fixed ports 6161/6162/8891/8892; all-hosts load). Run only
    `bun ./📜️script.ts test` in `RX/📦️packages/🟦️typescript` (vitest/jsdom) and, if needed, `verify taxonomy report --scope`
    for the quiz scope. Set `NX_PLUGIN_NO_TIMEOUTS=true` for any `bun nx` call.
13. For browser checks use a **private stack**: Vite on e.g. 6191 and a proctor on 8921 with `PROCTOR_DATA` pointing at a scratch
    folder (the shared dev data folder was a format-v1 database the proctor refuses; do not delete it). The running 6061 site
    has no backend and HMRs on every save, so it is only good for looking at the introduction/language screens.
14. Keep scratch files in the pets ticket folder (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/`), outputs under its
    `🗑️generated/` which only the pets agents may delete. Never touch other tickets' folders or `🗑️generated`.
15. No git commands that modify anything; `git diff <file>` on a shared file shows the other agents' hunks, so when
    reviewing your own change diff only your new files or compare with a copy you saved before editing.
