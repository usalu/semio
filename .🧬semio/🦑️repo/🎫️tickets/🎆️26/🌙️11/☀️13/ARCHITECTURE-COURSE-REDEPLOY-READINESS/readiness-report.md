# Architecture Course Redeploy Readiness

State on 2026-11-15, machine g1-m02 (macOS arm64, 16 GB). Worktree: `/Users/ueli/Documents/semio`.

## Verdict

Not ready to redeploy, but the remaining gaps are build-environment gaps, not known product defects. In the current worktree every unit-level gate that can run without the build graph passes (table below). The build graph cannot be read on this machine: `repo run` asks the fleet for the stale projects and gets none (`it can infer only 132 of the stale projects, the fleet was asked for 3216 and delivered 0 within 302 s`, exit 75; two projects need about 7676 MB each, this machine grants 3512 MB per project). So the targets that only run through the graph (`check`, `publish`, `test-e2e`, the assignment PDF builds, the Docker targets) have not run here. Their sources are the ones that passed on 2026-11-13, plus the energy assignment described below, whose PDFs have never been compiled.

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
