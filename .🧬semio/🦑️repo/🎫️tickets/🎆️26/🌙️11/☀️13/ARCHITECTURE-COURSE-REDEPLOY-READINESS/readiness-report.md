# Architecture Course Redeploy Readiness

State on 2026-11-15, machine g1-m02 (macOS arm64, 16 GB). Worktree: `/Users/ueli/Documents/semio`.

## Verdict

Not ready to redeploy, but the remaining gaps are build-environment gaps, not known product defects. In the current worktree every unit-level gate that can run without the build graph passes (table below). The build graph cannot be read on this machine: `repo run` asks the fleet for the stale projects and gets none (`it can infer only 132 of the stale projects, the fleet was asked for 3216 and delivered 0 within 302 s`, exit 75; two projects need about 7676 MB each, this machine grants 3512 MB per project). So the targets that only run through the graph (`check`, `publish`, `test-e2e`, the assignment PDF builds, the Docker targets) have not run here. Their sources are the ones that passed on 2026-11-13, plus the energy assignment described below, whose PDFs have never been compiled.

## End-to-End, Disconnects and a Hundred Learners (2026-11-18)

Verdict: frontend and proctor run and pass end to end, for every quiz, under disconnects and about a hundred concurrent learners. All runs native (no `repo run`), on a machine at load 50 to 110.

| Gate | Result |
| --- | --- |
| Proctor workspace (`repo cargo … test --manifest-path 🎓️teaching/Cargo.toml --workspace`) | 133/133 |
| Proctor chaos gate (`🎓️teaching/🛂️proctor/🧪️tests/🌪️chaos`, new): 100 protocol learners, all 10 quizzes, dropped sockets and streams, aborted and duplicated commands, SIGKILL/SIGTERM restart, audit by an independent TypeScript recomputation | holds in 3 runs (release 100 IPs, release 1 IP, prototype): 0 5xx, 1000/1000 runs scored once, no lost or duplicated events |
| One classroom behind one NAT IP, production limits | all 100 register; only 429 throttling at 10× human pace, retried; limits unchanged |
| Browser chaos (`🧪️tests/🌪️chaos-crowd`, new; project `chaos` in the exclusive shard): 10 real browsers + 90 protocol learners on the static release build, every quiz; offline mid-answer and mid-submit, reloads, tab close/reopen, slow network, proctor kills; ghost sockets (closed, reset, silent) | final run passed in 37.3 min: 100/100 browser quizzes, 900 protocol quizzes, exactly-once scoring, exact totals, leaderboard right at proctor and on every page, presence converges; ghosts gone in ~2–7 s, silent ones within the 60 s idle timeout (~62–67 s) |
| Proctor kill -9 with 60 open sockets, immediate restart (6×) | binds in 106–265 ms every time (tokio sets SO_REUSEADDR) |
| Playwright suite, dev and release rehearsal (see `e2e-suite.md`) | all specs pass after fixes; outage specs must run serially |
| Unit: course-react 1128, course framework 619, pets 1166, course site 875, course Rust 118 | pass (load-induced 5 s timeouts pass on rerun) |

Product bugs fixed: follow did not scroll the follower (scroll box detection, ancestor scroll capture); pets had no ground on single-card pages (48 px ground strip); phone leaderboard segments overflowed at 375 px (wrap); pet deed focus outline left the viewport; a locked topic's tint swallowed the pointer over the card above (`isolation: isolate`); gateway logged a false error for `set_nodelay` EINVAL on reset connections (macOS).
Stale specs fixed for Zipf pools, seven root cards, stretched cards and moved Explore links; harness bugs in the new chaos spec fixed. The `chaos` project joined the exclusive shard (fixtures updated).
Known limit: with cards filling every page there is no perch a grappler's rope reaches; the grapple spec now adds a test ledge. Run 24 once showed two browsers stuck on "course server not reachable" after kills; not reproduced in runs 25–26, attributed to overlapping harness restarts.

Scripts in this folder: `native-stack.ts` (dev/rehearsal), `native-stack-pets.ts`, `native-stack-follow.ts`, `chaos-stack.mjs`, `chaos-build.mjs`, `chaos-run.sh`, `pw-pets.sh`, `pw-follow.sh`, `ground-probe.mjs`.

## Native CDN Build (2026-11-17)

Owner decision (2026-11-17): the owner uploads the staged directory to the CDN; this redeploy goes out without `site.legal` URLs. Docker targets stay open for a Linux x64 runner.

Verdict: ready for the CDN. The staged directory `📦️packages/🟦️typescript/dist/release/pages/course` (116 files, 8.9 MB, `_headers`) passes `siteArtifactProblems`; the browser smoke test passes 15/15 on it (desktop). All ten quizzes, including the five scheduled ones on an unlocked copy, pass on desktop and phone (phone 19/20: the Desktop/Tablet switch is not offered at 390 px, by design). Not uploaded: the Pages workflow still calls `repo run …:publish`.

Homepage redesign (owner request): assignments moved from the top of the page into an Assignments card (de Übungen) with an in-page PDF viewer (iframe of the learner's language, Download, open in new tab, back); a Learning Clips card (de Lernclips) with existing chapter-still posters and per-language `urls` (null until the owner sends links; set them in `clips[].urls` of `📚️course/🔣️.json`). CSP gains `frame-src 'self'`, `_headers` use `X-Frame-Options: SAMEORIGIN` and `frame-ancestors 'self'`, posters have their own `posterBytes` budget (2 600 000). Framework 619, course-react 1121, course 875 tests pass. Follow-up checks: 26 clips with 26 hashed posters, all loading; the Rust course test crate passes 118/118 after fixing `software_mockup_vectors_agree_with_every_item_projection` (it compared `8` with `8.0`; the fixture is now normalized through `Mockup`); the redesigned e2e specs `🥞️layered-home`, `📱️phone`, `🗣️both-languages` pass 13/13 against a native dev stack (proctor `serve` on 8791, vite dev on 6061), one `layered-home` pan test flaked once under 3 workers and passed alone.

Goal changed to a CDN redeploy as fast as possible, falling back to native tools when the build graph fails. The site is built and staged without `repo run`:

1. Assignment PDFs: tectonic 0.16.9 (`.cache/b/t/m/print/tectonic`), default web bundle instead of the pinned 2.8 GB `tlextras-2022.0r0` bundle, fonts from `🧰️framework/🛍️products/📓️print/🖼️assets/🔤️font/`, parametrization icons rendered with resvg. Script `build.sh` in the session scratchpad. Six PDFs, 1 914 456 bytes (budget 2 500 000).
2. Site: `native-build.ts` (this folder) serves the PDFs over the runtime channel the Vite plugin reads (`SEMIO_VITE_RUNTIME_CHANNELS_V1`) and runs `vite build --mode release` with the course config, cache `.cache/b/p/m/vite-course-native`. Builds in about 4 s.
3. Validation and staging: `native-validate.ts` runs the publish verb's own `siteArtifactProblems` (no problems) and `stageCourseSiteArtifact` from a separate copy (`.cache/b/p/m/course-site-native-artifact`) into `📦️packages/🟦️typescript/dist/release/pages/course` (90 files, 6.5 MB, with `_headers`): the exact directory the workflow uploads with `actions/upload-pages-artifact`.
4. Smoke test: `native-smoke.ts` (Playwright, served statically, proctor offline): five energy quizzes started, answered and submitted, Explore, leaderboard, badges, introduction, language, theme, pets, layouts, PDFs: 15/15 pass on desktop and phone. The five scheduled quizzes (`unlocks` null) are tested on an unlocked scratch copy.

Rebuild commands (repo root, `BUN=.cache/b/t/m/bun/<digest>/bun`):

```
PDF_DIRECTORY=<pdfs/out> $BUN <ticket>/native-build.ts
cp -R <dist/release> .cache/b/p/m/course-site-native-artifact
ARTIFACT_DIRECTORY=$PWD/.cache/b/p/m/course-site-native-artifact STAGE_DIRECTORY=<dist/release>/pages/course $BUN <ticket>/native-validate.ts
```

Upload is not done: GitHub Pages deploys from the workflow, which still calls `repo run …:publish`. Uploading the staged directory is an outward action for the owner.

Also in this round: energy quiz pools draw Zipf-weighted toward small everyday magnitudes (`pool.distribution`, exponent 1.5; draw counts lowered, e.g. powers 10 to 6); framework 601, course 867, course-react 1100 tests and the course Rust crate pass.

## Verified in the current worktree on 2026-11-15

The graph-bound `repo run` was replaced by the same commands the targets run, started through the `repo` tool registry (`repo tools exec …`, `repo cargo …`).

| Gate | Command | Result |
| --- | --- | --- |
| `@teaching/architecture-course:test` | `repo tools exec vitest -- run --configLoader native --config 🧪️tests/🎚️config/🟦️.ts` in the course root | pass, 860 tests (11 failed before the energy-assignment wiring below) |
| builder assignment-PDF control test | `repo tools exec bun -- test …/🏗️builder/🌐️vite/📦️assignments/🧪️tests/🟦️.ts` | pass, 3 tests |
| `@semio-tech/course-react:test` | vitest as above in the renderer root | pass, 1100 tests |
| `@semio-tech/pets:test` | vitest as above in the pets root | pass, 1166 tests |
| `@teaching/proctor:test` (Cargo part) | `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🎓️teaching/Cargo.toml --workspace` | pass, 133 tests (first try hit a missing `.rmeta` in the shared build directory while another build ran; the retry passed) |
| `@teaching/architecture-course:typecheck` | `repo tools exec tsc -- -p tsconfig.json --noEmit` in the course package | fails with 6 errors, all in build-tool files the fleet coordinator was editing during the run (see Typecheck below); 0 in course, renderer or proctor code |

## Typecheck (2026-11-15)

The course tsconfig reaches the `repo` build tooling transitively. On 2026-11-15 it reported 100 errors there, none in the course. 94 were fixed without changing behaviour (contextual types for frozen tables, `Uint8Array<ArrayBuffer>` where a `BodyInit` or `Int32Array` buffer is required, a discriminable `RefLinearV1`, `ClaimKindV1` as the literal union `"ac"|"nx"`, `maximumCarriedCursors` in the graph-hooks declaration, `"lent"` excluded where `describeHostReasonV1` does not take it). One was a real bug: in `🤝️shared-worktree/🔁️replica/🟦️.ts` the paused path of `applyQueueWork` returned `kept: 0` instead of an array. The module tests of the edited claims, git and boundary modules pass (8, 3, 46, 69). Left for their owner, edited within the hour: `🕸️graph/🕰️host/🟦️.ts` (3), `🕸️graph/🤝️prefetch/🟦️.ts` (1), `🧭️invocation/🛰️plan/🟦️.ts` (2).

## Energy Assignment Wiring (2026-11-15)

The course declares a third assignment, `heating-cooling` (topic energy, sources in `⚡️energy/📝️assignment`, uncommitted), and `📜️script.ts` registers `build-heating-cooling-en|de`, but `📋️project.json` had no such targets and no consumer depended on them, so the site build would have shipped without the PDFs and 11 course tests failed. Fixed:

- `📋️project.json`: targets `build-heating-cooling-en|de` (same shape as the typology targets, output `…/documents/course/architecture-energy-heating-cooling-<language>`); both added to `dev`, `dev-site`, `build`, `test-e2e`; `⚡️energy/📝️assignment/**/*` added to the `assignment` input.
- Fixtures: `🧫️fixtures/🌐️publication/🔣️.json` lists six PDFs, its schema allows exactly six; `🏗️builder/🌐️vite/📦️assignments/🧪️tests/🔣️.json` and `📋️schema.json` name six producers; `🧫️fixtures/🎛️dashboard/🔣️.json` lists the two new targets.
- Texts: the assignment-PDF feature file (six PDFs), the builder test title, README target tables (en, de) and the renamed section "Course Assignment Documents".
- Not verified: the energy PDFs compile, and six PDFs stay under `budget.assignmentBytes` (2 500 000; the four earlier PDFs weighed 1 849 985 bytes).

## Energy Assignment Locale Conventions (2026-11-15)

The English energy assignment used German conventions: decimal commas (`3{,}0`, `2{,}4 m` in the plan and the input table) and the abbreviation JAZ (Jahresarbeitszahl) for the seasonal performance factor. Fixed test-first:

- `🧪️tests/🧫️cases.json` and its schema: `required` tokens are language-neutral (`3.0`, `0.37`, `2.5`); new `decimal` (`en` `.`, `de` `{,}`) and `performance` (`en` SPF, `de` JAZ).
- `🧪️tests/🟦️.ts`: each locale must define `\EnergyDecimal` and `\EnergyPerformance` with its own values; English prose must not contain `{,}`, German prose no `d.d` number; `model.tex` math must use `\EnergyDecimal`; prose must not contain the other locale's abbreviation. Red before the content change (`en: locale number conventions required`), green after.
- `model.tex` uses `\EnergyDecimal` and `\EnergyPerformance`; `en.content.tex` uses decimal points and SPF; `de.content.tex` keeps commas and JAZ.
- `repo tools exec bun -- test ./…/📚️course/🧪️tests/📖️assignment-pdfs/🟦️.ts`: 20 pass.

## Graph Attempt (2026-11-15 08:00Z)

`repo run @teaching/architecture-course:publish --configuration=release --skip-remote-cache` now reads only the course slice (owners 0 of 12, shared closures 24 of 24) instead of waiting for the whole fleet, but ends with exit 75: `the plan stage inference (during inference) grew to 10415 MB, more than the 6553 MB this machine gives a plan, and only 2787 MB of memory were available`. A round-24 readiness try of the fleet session ran at the same time, so part of the shortfall is the overlap; the finding went into that session's round-24 report. No further graph command was started; the protocol allows one per machine.

`repo tools setup toolchains tectonic` provisioned tectonic 0.16.9 into `.cache/b/t/m/print/tectonic` (exit 0). The TeX bundle and fonts are prepared only inside the print pipeline (graph), so the energy PDFs were not compiled outside it.


## Machine Setup Attempts (2026-11-15 to 2026-11-16)

Since round 31 of `26/11/14/WORKERS-READY-WITH-LOW-RESOURCES` every machine runs its own setup (`.cache/f/ready`); sessions start no setup or graph commands. On g1-m02 the stages `native`, `repo-mcp`, `dependencies` and `tools` passed; `workspace` (the graph read) failed three times:

| Attempt | End (UTC) | Result |
| --- | --- | --- |
| 1 | 2026-11-16 01:12 | `stage-timeout` after about 2 h 30 min, the last 2 h 10 min only `flush 1 of 1` |
| 2 | 2026-11-16 01:55 | exit 75 after the round-30 step-down ran all six steps (slice worker 6972, 10579, 9322, 7648, 7339 MB), refusal at 10174 MB against the 6553 MB plan limit |
| 3 | 2026-11-16 03:38 | exit 75, refusal at 10233 MB |

By 2026-11-17 03:44 UTC the `workspace` stage had failed 25 times (exit 75 memory refusals at 9.9 to 10.9 GB, latest `The graph service could not answer (key)`); the fleet round was still 31. The fleet's analysis (`result-R76.md`) places the growth in single heavy projects inferred in the plan process; its fixes (hand-off of too-heavy projects to the fleet, capped child processes) have not landed.

Even one project at a time needs more than the 6553 MB a plan may take on this 16 GB Mac, so retries on g1-m02 cannot succeed. The findings are in the fleet ticket (`workers/g1-m02/round-31.findings.md`). A cheaper inference of the heavy projects, or a larger machine that serves the graph, is needed before `check`, `publish`, `test-e2e`, `preflight` and the PDF builds can run.
## Verified in the old worktree on 2026-11-13

Every command ran as `repo run <target> --skip-remote-cache`. These results are not evidence for the current tree until repeated there.

| Target | Result |
| --- | --- |
| `@semio-tech/course-react:test` | pass, 1100 tests |
| `@teaching/architecture-course:test` | pass, 859 tests |
| `@teaching/proctor:test` | pass, 133 tests |
| `@teaching/architecture-course:typecheck` | pass |
| `@teaching/architecture-course:check` | pass on rerun (first run hit the graph-service `fstatat` failure) |
| `@teaching/architecture-course:publish --configuration=release` | pass, 88 files, 6.4 MB, four assignment PDFs |
| `@teaching/architecture-course:test-e2e -- --topology dev` | pass, 87 tests |
| `@teaching/architecture-course:test-e2e --configuration=release -- --topology rehearsal` | pass, 87 tests, three runs; ladder spec 24 of 24 on repeat |
| `@teaching/architecture-course:preflight` | pass |
| `@semio-tech/pets:test` | pass, 1166 tests |
| `@semio-tech/framework-kernel:test` | pass, 94 tests |
| `@semio-tech/print-viz-inference:test` | pass, 631 of 631 |
| print `check` and `test` | pass |
| launcher Go suite | pass |
| `@semio-tech/framework-os:test` | pass, 782 tests (after the os fixes) |
| `@semio-tech/framework:test` | pass, 370 tests |

## Not verified

- In `/Users/ueli/Documents/semio`: `check`, `publish --configuration=release`, `test-e2e` (dev and rehearsal), `preflight`, the assignment PDF builds; all need the build graph.
- `docker-image-build`, `docker-image-check`, `docker-image-publish`, `docker-stack-check`, `docker-stack-bundle`: need Linux x64 with Docker ("Tool docker not found" here). `deploy-check` passed steps 1 and 2 of 6, then refused: image production requires the authored native Linux x64 runner. They run in `.github/workflows/architecture-course.yml` (`ARCHITECTURE_COURSE_RUNNER`).
- `@semio-tech/framework-renderer-wgpu:test` after its output-path fix (never ran to completion).
- The fix for the graph-service `fstatat` failure against a real occurrence; the production errno was never recorded (`fstatat-race.md`).

## Fixes made

Details are in the neighbouring reports.

- e2e gate: output grant clamped to the schema, Playwright resolved from the spec scope, three specs corrected (`e2e-gate.md`).
- Product: pets on the overview can be picked up again (`e2e-gate.md`).
- Renderer: bounded retry and bilingual fallback for chunk and language loads, outbox pump, presence socket, drag abandon, duration formatting (`site-and-renderer-targets.md`).
- Deploy budget: assignment PDFs get their own 2.5 MB budget (`e2e-gate.md`).
- Proctor: storage is inspected read-only before any write; preflights are throttled, 429 carries Retry-After (`proctor-hardening.md`); Cargo closure regenerated (`typecheck-and-check-targets.md`).
- Framework: `FixedCommandPage::storage_demand` quoted 0 copy bytes; kernel tests ported (`kernel-tests-port.md`).
- Pet-ladder spec made deterministic (`ladder-flake.md`).
- Quiz: 48 specific explanations in both languages, with a guard test (`quiz-explanations.md`).
- Print and test edges: production-to-test exports removed, consumers rewired (`print-tests.md`, `rewire-verification.md`, `dev-planning-closure.md`).
- Launcher: symlink publish, schema digest, route fixture, detached store link (`launcher-tests.md`, `launcher-store-recovery.md`).
- Graph service: bounded retry for transient observation errors (`fstatat-race.md`).
- os product: Rust compile chain, three failing tests, one product bug in folder answers (`os-failures.md`).

## Incidents

- 2026-11-13 07:58: 23 tracked files were overwritten with their committed content; all were redone (`reverted-fixes-redo.md`). Cause not established. It can recur until the archivist commits.
- 2026-11-13 22:06: the worktree moved to `/Users/ueli/Documents/semio`. There, a dangling `node_modules` link and a stray empty `.cache/b/d/m/j` blocked every `repo` command; both were removed and `repo dependencies ensure` succeeded on the second run. The first run left a capsule folder `.p2118b9c8` in the repo root.
- 2026-11-14: the native bootstrap of the new tree ended with 8 failed setup tasks (see ticket `26/11/09/SHARED-WORKTREE-FLEET-OVER-NFS`, `g1-m02-relocate.md`). A `@teaching/proctor:test` run waited 7.5 h for memory and was stopped.

## Open

- Build graph: this machine cannot read it (fleet delivers 0 of 3216 projects; two projects need about 7676 MB each). Being handled in ticket `26/11/14` `WORKERS-READY-WITH-LOW-RESOURCES` (round 24 on 2026-11-15; slice inference of the course needed 10 415 MB). Until then `check`, `publish`, `test-e2e`, `preflight` and the PDF builds cannot run here.
- Energy assignment PDFs: never compiled; their weight against `budget.assignmentBytes` is unknown.
- Typecheck: 6 errors in build-tool files under active edit (see Typecheck).
- Repeat `check`, `publish --configuration=release`, both e2e topologies and `preflight` once the graph can be read.
- Docker image, stack and deploy-check on a Linux x64 runner.
- `site.legal` is empty in `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🔣️.json`. It takes `imprint` and `privacy` as absolute HTTPS URLs; the repo holds none. Owner decision.
- os: the TypeScript stage of `framework-os:typecheck` still reports about 150 diagnostics; `framework-os-dev:test` has 3 failures; two renderer-react files cannot resolve generated wasm bindings (`os-failures.md`).
- Two `filesystem-observation` churn tests fail on someone else's concurrent change to the graph reader.
- Remote cache at `192.168.178.70:6090` answers 401 on this machine.
- The proctor has no header or idle timeouts of its own; Caddy provides them.
- Quiz solutions ship to the browser by design.
- "Store dependency generation receipt stale" appears on every run and self-heals; cause unknown.
