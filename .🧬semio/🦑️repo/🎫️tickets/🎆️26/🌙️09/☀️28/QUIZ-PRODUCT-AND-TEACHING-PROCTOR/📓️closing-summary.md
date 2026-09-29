# 🧑‍🏫 Quiz Product and Teaching Proctor — Closing Summary

Normative design: `📓️design.md` (with §13 audit revisions). Contract: `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json`.
Per-package details, commands and results: `📓️core-ts-report.md`, `📓️core-rust-report.md`, `📓️conformance-report.md`,
`📓️proctor-report.md`, `📓️react-report.md`, `📓️content-report.md`, `📓️site-infra-report.md`. Audits:
`📓️audit-spec-conformance.md`, `📓️audit-agents-rules.md`, `📓️audit-content.md`. Blocker fixed on the way:
`📓️stash-conflict-resolution.md`.

## What exists now

| Area | Path | Content |
|---|---|---|
| Quiz product (framework) | `🧰️framework/🛍️products/❓️quiz` | render-independent declarative quizzes: JSON Schema contract with TS + Rust type twins; seven domain modules with TS and Rust twins side by side (`🎲️randomness`, `🃏️sheet`, `✅️validation`, `📏️scoring`, `🏅️badges`, `🧾️lifecycle`, `👁️views`); README with the domain model |
| Quiz renderer | `❓️quiz/🎯️targets/⚛️react` (`@semio-tech/quiz-react`) | onboarding (introduction → anonymous/pseudonym/name), home, run player (classification with spider diagrams, sorting, matching; keyboard + drag), results with solutions and sources, leaderboard; proctor client with per-record local-first outbox, multi-tab merge, cancellable submission, en/de (*du*), theme and text size |
| Proctor (server) | `🎓️teaching/🛂️proctor` (crate `teaching-proctor`, bin `proctor`) | framework `ServerInstance`: roster + learner actors over the pure quiz lifecycle, enrolment saga, projections (learner, run, leaderboard), four storage roles over one SQLite file (WAL), static site hosting, production gating, `serve`/`check`/`rebuild` |
| Quizzes | `🎓️teaching/🏛️architecture/⚡️energy/{🧲️physics,🔥️heating,❄️cooling,📊️demand}/❓️quiz/🔣️.json` | Physikalisches Verständnis, Heizen, Kühlen, Energiebedarf — every item with en/de label and a sourced explanation |
| Site + catalog | `🎓️teaching/🏛️architecture/❓️quiz` (`@teaching/architecture-quiz`) | catalog (introduction, four quizzes, 7 badges incl. Heating expert, Numerical brain, Pattern seer), Vite site for `quizze.architektur-und-technologie.de`, `🚀️deploy/{Dockerfile,compose.yaml,Caddyfile}` |

## Scoring (partial credit)

Sorting and matching use magnitude-weighted pair concordance: every pair of items counts with the distance of their
true values (log10 for quantities spanning orders of magnitude), a misordered pair costs its weight. Swapping near
neighbours costs little, swapping a tea light and a nuclear plant costs a lot. Classification with spider-diagram
profiles gives partial credit by profile similarity. Oracles: Kendall τ (unit weights) and Spearman ρ (rank weights)
from scipy/jStat, distances from scipy/mathjs.

## Verification (run by the coordinator at the end, plus each owner's report)

| Check | Result |
|---|---|
| `bun nx run-many -t test` for `@semio-tech/quiz`, `@semio-tech/quiz-react`, `@semio-tech/quiz-rs`, `@teaching/proctor`, `@teaching/architecture-quiz` | all 5 succeeded |
| Protocol v2 parity `parity exhaustive --owner ❓️quiz` (Python/numpy/scipy oracle vs TS vs Rust) | 75/75 |
| `verify taxonomy report` for `🎓️teaching` and `❓️quiz` | clean, 0 errors, 0 warnings |
| Protocol v2 contract for the quiz and proctor owners | 0 breaches in ticket files |
| `docker-image-build` + `docker-image-check` | image built (8 m 56 s); ready in 758 ms, site + SPA fallback, cleartext 403, clean stop, SQLite on the volume |
| `cargo test -p semio-framework-server` (after the shared rehydration fix) | 89 + 5 + 3 passed |
| Browser walk (dev site + dev proctor, two origins as two devices) | intro → pseudonym → heating run → answers saved → second device recalls a differently spelled handle and resumes the same seeded run with the same answers → submit → results with partial credit → leaderboard without learner ids → German locale; no console errors |

## Shared changes outside the ticket's trees

- `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs`: a freshly placed actor is rehydrated from its
  snapshot and events before deciding (previously every restart broke commands on actors with history — hub included).
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🟦️.ts` + subpath `@semio-tech/ui-react/i18n`: the i18n port left
  the 11,800-line barrel (re-exported unchanged); the quiz site's main chunk went from 1,691 kB to 414 kB.
- Registrations: products `🔣️.json`, root `package.json` (workspaces, scripts), `Cargo.toml` members/dependency,
  `.vscode/launch.json` + seed, `.claude/launch.json`, `🔣️taxonomy.json`, `.gitignore`, `.dockerignore`, shared Vite
  builder (stale shim fix, referenced-assets-only copy).
- Four stale stash-pop conflicts in the repo library resolved by union (`📓️stash-conflict-resolution.md`); the git
  index still lists them as unmerged until their owner stages them.

## Known limits and follow-ups

- `cargo check -p semio-hub --tests` crashes inside rustc (`STATUS_NO_MEMORY` while compiling `syn`/serde build
  scripts under hub's feature set) on this host; the server change has no public API diff and the server suite passes.
- 42 of 847 ui-react tests fail (Tree windowing, shell layout `NaNpx`, missing `--base` palette token, missing
  Playwright browser); none involve the i18n move.
- `workspace:verify -- interactivity apps` still reports 66 pre-existing errors, none from this ticket.
- The repo `.venv` has scipy 1.17.1 while `uv.lock` pins 1.18; the harness installs locked versions on first run.
- `🎬️clip` leaves of the teaching tree are intended but out of scope.
- The presentation product has the same react-target layout breach the quiz product avoided (task chip offered).
