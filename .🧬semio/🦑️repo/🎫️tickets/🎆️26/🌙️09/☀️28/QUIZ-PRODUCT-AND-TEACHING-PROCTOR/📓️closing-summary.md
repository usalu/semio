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

## Revision 2026-09-29 — card-grid UI, CDN site, zero-touch proctor, shared presence

Design: `📓️design.md` §14 (grid, CDN, Docker) and §15 (presence). References: `📓️ui-reference-play-demonstrator.md`,
`📓️explore-cdn-docker-deploy.md`.

| Area | Result |
|---|---|
| UI | Every screen in the play/demonstrator language (`WindowChrome` cards, `Navbar`, tokens, light/dark). Home = 3 × 3 grid with the leaderboard in the centre (2 columns on tablet, 1 on phone). The play/demonstrator card is extracted once as `🧱️elements/🃏️OverviewCard`, used by the quiz, play and the demonstrator; slim subpath `@semio-tech/ui-react/chrome`. |
| Site on the CDN | `quizzes.architektur-und-technologie.de`: release build bakes `VITE_PROCTOR_URL` (`https://proctor.quizzes.architektur-und-technologie.de`); `publish` verifies and stages `dist/pages/quizzes` (`CNAME`, `.nojekyll`, `404.html`, `_headers`); manual GitHub Pages workflow `.github/workflows/architecture-quiz.yml`. |
| Proctor on Docker | API only (site hosting removed, CORS max-age 7200); proctor-only image with production defaults baked in; `🚀️deploy/compose.yaml` = proctor + Caddy (automatic TLS for `proctor.quizzes.…`), named volumes, health-gated start; GHCR publish verb + workflow job. Host contract: DNS + ports 80/443 + `docker compose up -d`. |
| Shared presence | Framework presence WebSocket `GET /scopes/{scope}/presence/ws` (latest state per session, coalesced 100 ms batches, limits, origin check, `ServerModule::presence_admission` default) with TS client twin; quiz `Place`/`Cursor`/`PresenceState`/`CursorState` in both cores; proctor rooms (roster + per place); client shows who is online and where, "learning now" per quiz, online dots, and others' cursors and keyboard focus anchored to shared cards (never items or drags), with a "Show others' cursors" preference. |
| Schema | `Quiz.emoji` / `CatalogQuizView.emoji` required; presence `$defs`. |

Verification (coordinator, end of revision): nx `test` for `@semio-tech/quiz`, `@semio-tech/quiz-react`,
`@semio-tech/quiz-rs`, `@teaching/proctor`, `@teaching/architecture-quiz`, `@semio-tech/framework-server`,
`@semio-tech/framework-server-rs` — all 7 succeeded; Protocol v2 parity exhaustive 84/84 (11 cases); taxonomy clean for
`🎓️teaching`, `❓️quiz`, `🖥️server`; `cargo check -p semio-hub --tests` compiles against both framework changes (old
cargo layout in a private build dir, see below); browser: grid at desktop in German/dark, two devices (localhost vs
127.0.0.1) — "Online: 2" on both, device B's labelled cursor rendered on device A's leaderboard card, no console errors.
Site-infra: `docker-image-build` (459 s, 97.1 MB), `docker-image-check` and `docker-stack-check` (TLS through Caddy,
cross-origin, presence through Caddy) green; production rehearsal walk cross-origin green. Not run (need the owner's
credentials, outward publishing): GHCR push, GitHub workflow.

Findings for other tickets:
- Windows cargo builds of large graphs (hub, play wasm prerequisites) fail with rustc `STATUS_NO_MEMORY`: cargo's
  `build-dir-new-layout` puts every unit's `out` dir into rustc's `PATH` (~85 KB for hub), beyond the 32,767-character
  environment limit. Workaround used: `CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false`
  with a private build dir. Task offered: "Fix cargo PATH overflow on Windows builds".
- The play nx build could not be run for the same reason; play (72 pass, 3 unrelated fails) and demonstrator component
  tests pass with the shared `OverviewCard`.
- Site main JS 614 kB (gzip 167 kB) after adopting the design system; next lever: per-icon modules in `@semio-tech/assets`.
- Docker Desktop on this host leaves undeletable socket files on stop; site-infra moved them aside
  (`%LOCALAPPDATA%\*-stale-20260929*`, safe to delete).

## Revision 2026-09-29 (evening) — layered home like semio-tech play

Design `📓️design.md` §16; reference `📓️ui-reference-layered-landing.md`; reports `📓️layered-overview-report.md`,
`📓️react-report.md` (layered section).

- **Shared element** `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview` + `🔨️modules/🥞️layered-overview-geometry`
  (exported via `@semio-tech/ui-react/chrome`): strip of real pages behind ONE glass veil with a `clip-path` hole,
  compact card overlay, reveal on hover and keyboard focus (page clear), conceal on leave/blur, 500 ms eased glide,
  pointer pan, reduced motion, hash open/Escape/Overview with focus management, lazy boot with budget, list mode on
  phones. Tests on a language-agnostic fixture with `polygon-clipping` and `d3-ease` oracles (127 pass, 19/19 mutations
  killed).
- **Play and demonstrator** now use it (849 → 206 and 892 → 207 lines); nine of the ten landing defects fixed (the
  brand bar is visible again, keyboard reveal/conceal, focus management, one veil layer, no per-frame React render).
- **Quiz home**: nine real pages behind the glass (learner profile, read-only quiz pages, introduction, full
  leaderboard, badges, preferences); hovering a card shows its page clear and never starts a run; opening a card
  routes to `#page`. Presence knows the new pages (`Screen` + rooms in both cores, conformance, proctor admission now
  derived from the core's screen list).

Verification (coordinator): nx `test` green for the quiz, proctor and server projects (7); ui-react 962 pass with the
22 known pre-existing failures unchanged; play 44 pass / 3 pre-existing; demonstrator 19 pass (branding test needs the
flow-core wasm); parity 84/84; taxonomy clean for `❓️quiz` and `🎓️teaching`, and no error in any directory this
ticket created under `🖱️ui` (that scope, play and the demonstrator carry older debt — task offered). Built-in browser
at 1440 × 900: at rest the learner page is blurred behind the veil and nine compact cards sit in the 3 × 3 grid;
hovering the leaderboard glides the strip to the centre cell, hides the veil and shows the full leaderboard crisp;
hovering "Heizen" shows the heating quiz page (description, both tasks) crisp; leaving restores the veil; fresh tab
without console errors.
