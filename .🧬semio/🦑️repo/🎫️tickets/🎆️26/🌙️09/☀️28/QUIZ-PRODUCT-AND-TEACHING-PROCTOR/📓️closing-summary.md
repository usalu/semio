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

## Revision 2026-09-29 (night) — sharing inside the quizzes and a live grid backdrop

Design `📓️design.md` §17 (supersedes the §15 rule "never share answers or drags"); reports `📓️core-ts-report.md`,
`📓️core-rust-report.md`, `📓️conformance-report.md` (`📊️crowd-view`), `📓️proctor-report.md` (watch, crowd),
`📓️layered-overview-report.md` §00 (grid rest), `📓️react-report.md` (night section).

- **What the others think, live**: thinking room `<catalog>/quiz/<quiz>/thinking` with `ThinkingState { tag, answers }`,
  drafts published at most twice a second and aggregated by item id, because every learner's sheet is shuffled
  differently. Classification shows counts per category, sorting shows the others' places and their mean, matching shows
  the values given (`ThinkingMatchingAnswer`: values, never the publisher's card indices). Cursors in the quiz room
  anchor to `item:<id>` / `category:<id>` and carry `drag { item }`. Bounds in both cores: 64 tasks and 64 entries,
  no repeated sorting item, finite values.
- **What everyone answered, persisted**: projection `CrowdView` per quiz over all submitted results (query
  `quiz.crowd`, core `crowdView` in both cores, count keys in code-point order). Shown on the quiz page, in the run when
  nobody else is thinking along, and as an "Everyone" column on the results page. Preference "Show what others think"
  / "Zeigen, was die anderen denken".
- **Live grid backdrop**: `LayeredOverview` rest mode `"grid"` with `gridTracks` (quiz 1 : 1.5 : 1 × 1 : 1.4 : 1; play and
  demonstrator keep `"panorama"`). At rest all nine pages are mounted, live and inert, each behind its card's cell under
  the glass; revealing a card zooms its page to full size. The framework presence socket gained `watch` (≤ 16 scopes,
  interval ≥ tick, `watched` frames with snapshots, read-only, admission per watched scope). The home joins its room
  and watches the page, quiz and thinking rooms at 4 Hz, so every backdrop page draws the others on its own anchors.
- **Coordinator fix**: home asked for each quiz's crowd once at mount, so "What everyone answered" in the backdrop went
  stale after someone else submitted. The crowds now refresh whenever the polled leaderboard counts a new submitted
  run. Test added to `🧪️tests/🏠️home-grid`.
- Shared fix by the react owner: `WindowChrome` silhouettes measure in the element's own pixels, so cards inside scaled
  pages are no longer cut off.

Verification (coordinator):
- nx `test` passes for all 7 projects (quiz, quiz-react 230, quiz-rs, proctor, site, framework-server TS and Rust).
- Parity 93/93, level exhaustive.
- Taxonomy clean (0 errors, 0 warnings) for `❓️quiz` and `🎓️teaching`.
- The hub still compiles against the framework `watch` change (old build-dir layout, private dir).
- Built-in browser, two devices (`localhost` / `127.0.0.1`), 1440 × 900:
  - all nine `[data-layered-pane]` are mounted with real content at `data-rest="grid"`; the physics backdrop page
    shows "Learning now: 1" while the peer is in a run;
  - in the same quiz, device 1 shows "What others think now · Thinking along: 1" with the peer's draft places per item.
    Place 9 on the peer is shown as "⌀ place 9" on device 1's place 4; items outside device 1's draw show no hint;
  - the peer's cursor is drawn on the same item's handle across different shuffles;
  - after each submission, device 1's leaderboard card and backdrop page updated without a reload, and so did the
    backdrop crowd (runs 2 → 3, after the fix);
  - the peer's cursor is drawn inside device 1's backdrop leaderboard page and on the matching home card;
  - results show "What everyone answered · Submitted runs: n";
  - a fresh tab has no console errors, and the proctor log has no errors.
- The Browser pane was hidden during the walk, so the page reported `visibilityState: hidden` and polling paused (by
  design); the walk made it visible by overriding `document.visibilityState` in the page, for debugging only.

## Revision 2026-10-02 — one dev command, end to end, ready to deploy

Owner request: "Make sure that everything works end to end. Add a dev architekturundtechnologie quizze which starts
both backend and frontend. Have everything ready to be deployed." Design `📓️design.md` §18; plan
`📓️deploy-readiness-plan.md`.

### Audits (read-only, before any change)

| Report | Result |
|---|---|
| `📓️audit-final-requirements.md` | 12 of 15 requirements met, 3 partly (no `clip` leaf yet; nothing published; no presence before identity); 5 defects |
| `📓️audit-deploy-security.md` | 3 blockers: learner ids readable through idempotency receipts, client ids never validated, no flood protection; 13 should, 12 notes |
| `📓️audit-final-i18n-a11y.md` | 7 defects, 25 improvements; key sets of both languages identical |

### What was built (one report per work package)

- **Dev command and end-to-end gate** (`📓️dev-e2e-report.md`): `bun nx run @teaching/architecture-quiz:dev` starts the
  proctor, waits until it is ready, then the site; one stop ends both; a proctor already running is reused. Launch row
  `🛠️dev🏛️architektur-und-technologie❓️quizze`; site alone `…:dev-site`; the compound row is gone. `…:test-e2e` drives the
  real site with Playwright in a dev topology and a cross-origin release rehearsal (production-mode proctor, sealed
  Content-Security-Policy).
- **Edge hardening** (`📓️edge-hardening-report.md`, `📓️signup-allowance-report.md`): receipts keyed per principal;
  admission before placement; 16 KiB bodies; token buckets per address for commands, reads, socket upgrades and
  sign-ups (1 200 at once + 100 per hour); socket and in-flight caps; WebSocket frame limits; bounded, fairly shared
  presence; grouped commits; only the core routes; fixed public error texts. Capacity gate `@teaching/proctor:capacity`.
- **Domain hardening** (`📓️domain-hardening-report.md`): ids and slugs validated at every door; handles normalized by the
  server (Latin alphabet, no invisible or look-alike characters); no roster actor — one stream per handle, recall is a
  read; caps (100 000 registrations, 1 000 runs per learner, 200 per quiz, 2 000 answers per run); leaderboard
  `{ rows ≤ 100, learners, own? }` updated only by submissions, badges and registrations; `proctor health`, `backup`,
  `restore`, `erase`, `prune`; undecodable events fail loudly; the real catalog's seven badges tested. Storage format v2.
- **Deploy** (`📓️deploy-readiness-report.md`): distroless image (31.5 MB, uid 65532, read-only root, pinned bases,
  catalog validated in the build, stack files inside); compose with dropped capabilities, limits and log rotation;
  Caddy with body cap, timeouts, security headers and a retry window; workflow on `main` only with pinned actions and
  an immutable `sha-<commit>` tag; Content-Security-Policy on the site; drift check and JSON Schema for
  `🚀️deploy/🔣️.json`; readiness gate `…:deploy-check` (8 steps); runbook in the site README, section "Deploy".
- **UI** (`📓️ui-hardening-report.md`, `📓️ui-polish-report.md`): no default language; matching and sorting keyboard
  fixes; forced-colors rules and label contrast; confirmation before an anonymous learner switches identity; honest
  identity wording; footer with a privacy notice and `site.legal` links; `429`/`503` as transient shortages; refused
  sign-ups explained; no layout shift from the crowd; reflow at 200/400 % and largest text; two typecheck gates.
- Found on the way: the site did not build from a fresh clone (two git-ignored generated sources; now dependencies of
  the targets); `@semio-tech/assets:build` failed on Windows; the image build can crash in rustc (the Dockerfile tries
  twice); old dev data in storage format v1 is moved aside by the dev launcher with one clear line.

### Verification by the coordinator on the final tree (2026-10-02, 09:30–11:00 local time)

| Gate | Result |
|---|---|
| `nx run-many -t test` for quiz, quiz-react, quiz-rs, proctor, site, framework-server (TS and Rust) | 7 of 7 projects pass |
| `@semio-tech/quiz-react:typecheck`, `@teaching/architecture-quiz:typecheck` | both exit 0 |
| Parity, level exhaustive, owner `❓️quiz` | 105/105 |
| Taxonomy `❓️quiz`, `🎓️teaching`, `🖥️server` | clean, 0 errors, 0 warnings each |
| `cargo check -p semio-hub --tests` (old build-dir layout) | exit 0 against the final server crate |
| `…:deploy-check`, all 8 steps (drift, typecheck, catalog, CDN artifact, image build, image check, stack check, end-to-end gate), last run 11:10–11:32 | **pass**: `ready to deploy`, end-to-end 31 passed in the rehearsal and 31 in the dev topology; two warnings: `site.legal.imprint` and `site.legal.privacy` are not set |
| `…:test-e2e` history of the morning | earlier runs were red because of 3–4 specs of `🧪️tests/🐕️pet-walk` (ticket QUIZ-PETS, another session), which sat in the `desktop` project and made Playwright skip `presence` and `shortage`; that session moved them into a project of their own and fixed them. This ticket's specs passed in every run except one dev-topology run that hit `net::ERR_NO_BUFFER_SPACE` (the host ran out of socket buffers while several sessions ran gates) |
| `@teaching/proctor:capacity` (300 learners beside an abusive script, default cap) | 2 of 3 runs pass; in one run 2 of 3 162 presence joins of the hall were reset before their welcome beside the socket flood (a client reconnects; the gate counts it) |

Browser walk through the new dev command (`.claude/launch.json` `architektur-und-technologie-quizze`), two devices:
- the command moved the v1 dev data aside with one message, built and started the proctor, waited for it, started the
  site; stopping left no process and no port;
- a stale learner id in the browser fell back to the introduction; a Cyrillic look-alike handle was refused with the
  character named; `"  prüferin   MÜLLER "` on the second device continued as "Prüferin Müller" with the open run;
- an anonymous learner switching identity is told the progress cannot be continued by anyone;
- English and German each set the document language, the title and the number format;
- in the same quiz, device 1 saw device 2's draft values per item; when device 2 left, 0 of 10 items moved;
- device 2's answers survived a server restart and a reload; its submission (39.7 %) reached device 1's leaderboard
  card and page without a reload ("Learners in total: 1");
- a fresh tab had no console error; the proctor logged no error.
The dev server reloaded the page whenever the other session saved a file of the pets module (it is in the quiz's
module graph); the walk therefore ran with `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` (`…-steady` entry).

### Not done, and what only the owner can do

- Nothing was published: no `docker push`, no workflow run, no DNS. Never built: `linux/arm64`.
- Owner steps, in order (runbook: site README "Deploy"): imprint and privacy URLs into `site.legal` of
  `🚀️deploy/🔣️.json`; DNS for both hosts and the GitHub Pages verification record; Pages source "GitHub Actions" with
  the custom domain; push to `main`, run the workflow, make the GHCR package public; four lines on a `linux/amd64`
  Docker host; the verify table; the daily backup line and an uptime probe.
- Known limits: several addresses together can fill the registration cap faster than one (41 days / addresses; `prune`
  removes registrations nobody played under); one flooding address can halve the presence budget of a hall; plain TCP
  connections are bounded by Caddy, not by the proctor; high-contrast mode was checked by computed styles, not by eye;
  the navbar title ends in an ellipsis at 320 px with the largest text.
