# 🗺️ E2E Map, Wave 1024d (map-d, 2026-10-24)

Scope: the targets that define "runs end to end" for the dashboard, the architecture course and print.
Method: graph-free. Read `📋️project.json` files directly (1483 indexed via `rg --files -g '📋️project.json'`, names read from the `name` field), `package.json` workspace deps, and `Cargo.toml` path deps. No `bun repo` calls, no builds, no edits outside this report. The name index is in the agent scratchpad `/private/tmp/claude-501/-Users-ueli-Documents-semio/3d0a4ac3-5d70-4c6b-91df-262fe6307e4a/scratchpad/index.tsv`.

Legend: class `T` test, `B` build, `G` codegen/generate, `D` dev server (continuous), `E` e2e, `S` source-only check (no build), `I` install/side-effect, `P` prerequisite. "dependsOn" is target-level; "children" is `metadata.semio.execution.children` (orchestration edges). Rust crates without a `📋️project.json` have no targets; they are built through the owning project's `compile` target.

---

## 1. Dashboard (`@semio-tech/repo-dashboard-rs`, binary `semio`)

Root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/📋️project.json` (projectType library).
Cargo: crate `semio-framework-repo-dashboard`, `[[bin]] semio` (entrypoint `…/🎛️dashboard/🚪️entrypoint/🦀️.rs`).

| target | class | command (`repo ./📜️script.ts …`) | dependsOn / children | notes |
|---|---|---|---|---|
| compile | B | `compile` | none | `rustBuildIntent`: package `semio-framework-repo-dashboard`, binary `semio`, fallback bounded-cargo; cache driver-owned |
| build | B | `build` | children `compile` | cache false |
| test | T | `test` | children `compile` | orchestration, live |
| test-unit | T | `test unit` | none | cache true |
| test-quick | T | `test quick` | children `compile` | live |
| test-long | T | `test long` | children `compile` | live |
| test-exhaustive | T | `test exhaustive` | children `compile` | live |
| test-coverage | T | `test coverage` | children `compile` | live |
| test-authority | T | `test authority` | children `compile` | live |
| test-execution | T | `test execution` | children `compile` | live |
| test-load | T | `test load` | children `compile` | live, uses `🧪️tests/🧭️journeys/**` |
| test-smoke | T | `test smoke` | children `compile` | live |
| test-journeys | E | `test journeys` | children `compile` | live, "journeys" = end-to-end user flows |
| test-cold-start | E | `test cold-start` | children `build` | "Verify Cold Dashboard Launch": real lazy build plus native commands check, then warm replay; live |
| test-windows | T | `test windows` | none | cache true |
| test-process-deadline | T | `test process-deadline` | none | real supervised processes, live |
| test-dispatch | S | `test dispatch` | none | exact Rust law ownership |
| test-language | S | `test language` | none | language negotiation, no install |
| test-source-freshness | S | `test source-freshness` | none | cargo inputs, no native build |
| test-ci-source | S | `repo ./📜️script.ts test ci-source` | none | workflow/bootstrap declarations |
| run | D | `run` (forwards args) | children `build` | continuous; the human entry is `repo dashboard` (not used by agents, AGENTS.md) |
| daemon | D | `daemon` (start/stop/status/attach) | children `build` | session daemon |
| preferences | D | `preferences` (show/set) | children `build` | continuous |
| install | I | `install` | dependsOn `build` | mutates host store |

Dashboard closure (Cargo path deps read from `Cargo.toml`, every project that owns a `📋️project.json`):

| project | path (under repo root) | targets |
|---|---|---|
| @semio-tech/repo-dashboard-rs | `…/🎛️dashboard/📦️packages/🦀️rust` | as above |
| @semio-tech/repo-dashboard-cli-rs | `…/🎛️dashboard/🧭️cli/📦️packages/🦀️rust` | none (tags only) |
| @semio-tech/repo-dashboard-core-rs | `…/🎛️dashboard/🧱️invocation-catalog/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-daemon-rs | `…/🎛️dashboard/🌀️daemon/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-wire-rs | `…/🎛️dashboard/🌀️daemon/🧵️wire/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-inventory-rs | `…/🎛️dashboard/📚️inventory/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-labels-rs | `…/🎛️dashboard/🖥️terminal/🌐️labels/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-launch-rs | `…/🎛️dashboard/🧾️launch/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-launcher-rs | `…/🎛️dashboard/🖥️terminal/🚀️launcher/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-preferences-rs | `…/🎛️dashboard/⚙️preferences/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-registry-rs | `…/🎛️dashboard/🎮️registry/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-repo-domain-rs | `…/🎛️dashboard/🏛️repo-domain/📦️packages/🦀️rust` | none |
| @semio-tech/repo-dashboard-terminal-rs | `…/🎛️dashboard/🖥️terminal/📦️packages/🦀️rust` | none |
| @semio-tech/repo-workspace-rs | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust` | build (script), test (script) |
| @semio-tech/ui-locale-rs | `🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust` | test, test-retirement |
| @semio-tech/ui-styling-tokens | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust` (crate `semio-framework-ui-styling`) | generate, preview-generated, check-generated, fonts, test, test-rs, test-py, test-relative-sizing (dependsOn `workspace:deps-browsers`), test-verification-contract, test-color-primitives (dependsOn `workspace:deps-browsers`) |
| journey-fixture | `…/🎛️dashboard/🧪️tests/🧫️fixtures/🧭️journeys/🏗️workspace` | none (fixture project) |

Dashboard crates with no `📋️project.json` (built by the Cargo closure only, no targets): `semio-framework-repo-dashboard-feedback`, `-keymap-editor`, `-source-view` (all under `…/🎛️dashboard/🖥️terminal/`); the workspace-view crate under `…/🎛️dashboard/📂️workspace/🖥️view` (package name not checked).

Dashboard edges: no target-level cross-project `dependsOn`. Edges are Cargo path deps (above) plus `children` into the dashboard's own `compile`/`build`.

**Dashboard "end to end" target set**: `build` (compile) → `test-cold-start` (cold launch) → `test-journeys` and `test-smoke` (live journeys). Source gates `test-ci-source`, `test-dispatch`, `test-language`, `test-source-freshness` need no build.

---

## 2. Architecture course (`@teaching/architecture-course`)

Root: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json` (TS, Vite, ports 6061 dev, 6063 beside-stack).

| target | class | command | dependsOn | notes |
|---|---|---|---|---|
| preflight | S | `preflight` | none | |
| ci-browser-prepare | P | `ci-browser-prepare` | none | |
| build | B | `build` (`--configuration={courseMode}`, default release in plan) | assets:build, ui-styling-tokens:generate | output `.🧾️store/build.json`, closure `tsconfig.site.json` |
| typecheck | T | `typecheck` | framework-rs:generate, ui-contract-rs:generate, ui-rs:generate, framework-actor-rs:typegen, ui-styling-tokens:generate, assets:build | |
| test | T | `test` (plan runs `-- quick`) | same as typecheck | modules `🧪️tests/**/🟦️.ts` |
| check | T | `check` | proctor:build | |
| test-e2e | E | `test-e2e` | assets:build, ui-styling-tokens:generate, proctor:build | parameters: `execution-mode` (release, sets SEMIO_BUILD_MODE), `serial`, `keep`, `proctor`, `topology` (choice `both` default, `dev`, `rehearsal`), `grep`, `project`, `headed`, `workers` |
| dev | D | `dev` | assets:build, ui-styling-tokens:generate, proctor:build | continuous, port 6061 (`TEACHING_ARCHITECTURE_COURSE_PORT`); stack `beside` uses 6063, proctor 8793 |
| dev-site | D | `dev-site` | assets:build, ui-styling-tokens:generate | continuous, port 6061 |
| publish | I | `publish` | assets:build, ui-styling-tokens:generate, architecture-course:build | |
| deploy-check | S | `deploy-check` | assets:build, ui-styling-tokens:generate, architecture-course:typecheck, architecture-course:publish, proctor:build | |
| docker-image-build / -check / -publish, docker-stack-check, docker-stack-bundle | I/S | `docker-*` | docker-image-build depends on proctor:build | deployment, not e2e |

Proctor (course backend, Rust app): `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json` (`@teaching/proctor`, projectType application). Targets: `build` (B), `test` (T), `dev` (D, continuous, dependsOn build), `check` (T, dependsOn build), `rebuild`, `health`, `backup`, `restore`, `erase`, `prune`, `capacity` (all dependsOn build). Cargo path deps: `🎓️teaching/🛂️proctor/🏗️bootstrap`, `…/🔨️modules/🧠️domain`, `…/🔨️modules/🧱️base`, test crates `…/🧪️tests/🌐️end-to-end` and `…/🧪️tests/🔬️conformance`, plus `🧰️framework/🔨️modules/🧪️test/🧲️fixture-reader`.

TS/Rust closure of the course (`package.json` workspace deps plus target `dependsOn`):

| project | path | targets used |
|---|---|---|
| @semio-tech/course | `🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript` | test (deps: pets, assets (dev), framework (dev), process (dev)) |
| @semio-tech/course-react | `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/📦️packages/🟦️typescript` | test, typecheck (deps: framework, framework-server, pets, pets-react, course, ui-react) |
| @semio-tech/pets | `🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript` | test, typecheck (dev-dep pets-react) |
| @semio-tech/pets-react | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript` | test, typecheck, dev (continuous); dep pets |
| @semio-tech/framework | `🧰️framework/📦️packages/🟦️typescript` | test, typecheck, test-* |
| @semio-tech/framework-server | `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript` | test |
| @semio-tech/ui-react | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript` | lint, test, typecheck, canonical-architecture |
| @semio-tech/ui-styling | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript` | generate (noop, dependsOn ui-styling-tokens:generate), test, twin, test-geometry |
| @semio-tech/assets | `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript` | build (course prerequisite), check-generated, preview-generated, logo, generate-logo |
| @semio-tech/ui-styling-tokens | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust` | generate (prerequisite), check-generated |
| @semio-tech/framework-rs | `🧰️framework/📦️packages/🦀️rust` | generate (prerequisite), test |
| @semio-tech/ui-contract-rs | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust` | generate (prerequisite), test, check |
| @semio-tech/ui-rs | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust` | generate (prerequisite), test, check |
| @semio-tech/framework-actor-rs | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust` | typegen (prerequisite), test |
| @semio-tech/course-rs | `🧰️framework/🛍️products/📚️course/📦️packages/🦀️rust` | build, test (not in the TS `dependsOn`) |
| @semio-tech/pets-rs | `🧰️framework/🛍️products/🐾️pets/📦️packages/🦀️rust` | build, test (no edge from course found) |

Course "end to end" target set: `@teaching/proctor:build` → `@teaching/architecture-course:typecheck`, `:test`, `:build`, `:check` plus `@teaching/proctor:test` → `@teaching/architecture-course:test-e2e` (default `topology` is `both` in project.json, the plan and `run-e2e.sh` use `dev`). Browser e2e needs Playwright (`workspace:deps-browsers`).

---

## 3. Print

| project | path | targets |
|---|---|---|
| @semio-tech/print | `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript` | build (dependsOn fonts, deps-tectonic, generate; runs tectonic template compile), test (dependsOn build), check, generate, generate-viz, fonts, deps-tectonic, deps-tex, watch (D), build-viz, test-viz, test-viz-full (dependsOn build-viz), test-macro-staging, test-toolchain, test-native-grammar, test-viz-fixtures, test-measurement, generate-font-metrics |
| @semio-tech/print-rs | `🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust` | build, test, test-snapshot-sqlite (+ native, source), produce-logical-owner, test-logical-owner, test-native-emission (configs prototype, release) |
| @semio-tech/print-viz-inference | `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript` | build, test |
| @semio-tech/print-tests-rs | `🧰️framework/🛍️products/📓️print/🧪️tests/🥣️companion/📦️packages/🦀️rust` | none (companion tests) |
| @semio-tech/presentation | `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript` | test (dev-deps process, framework) |
| @semio-tech/presentation-react | `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react` | test (deps: framework, presentation, ui-react) |
| @semio-tech/mit-bestand-bericht | `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript` | build (complete: build-zwischenbericht, build-forschungsbericht, build-kompaktbericht), build-kompaktbericht, build-zwischenbericht, build-forschungsbericht (each dependsOn print:fonts, print:deps-tectonic, print:deps-tex, print:generate, + generate-actor-network for the two long reports), test (dependsOn build), watch* (D), generate-actor-network, akteursnetz-validate, check-generated |
| @semio-tech/mit-bestand-praesentation-projektetage | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript` | build (no dependsOn), test (no dependsOn), dev (D, port 6050 via `PRAESENTATION_PROJEKTETAGE_PORT`), canonical-architecture |

Print edges: the bericht and the deck depend on `@semio-tech/print:*` and `@semio-tech/presentation(-react)` only via TS `dependsOn`/package deps. The projektetage build/test carry no target dependsOn, only package deps (presentation, presentation-react, process).

Print closure, package-only (no `📋️project.json` target edges): `@semio-tech/framework-filesystem` (`🧰️framework/🔨️modules/📁️filesystem`), `@semio-tech/framework-sqlite-snapshot-ts` (`🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🟦️typescript`), `@semio-tech/schema-validator-ts` (`🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript`), `@semio-tech/ui-styling`, `@semio-tech/framework`, `@semio-tech/print-viz-inference`. Cargo path deps of print-rs: `🔨️modules/⚠️diagnostic`, `🔨️modules/🌱️value/✨️derive/🚪️io/📝️text/📸️snapshot`, `🔨️modules/🎒️pack` (+ `🔤️json`), `🔨️modules/🗣️dsl/🧬️schema` (+ `✨️derive`).

Print "end to end" target set: `print-rs:test` (native), `print:build` (fonts + tectonic templates + generate), `print:test`, `print:check`, `presentation:test`, `presentation-react:test`, `mit-bestand-bericht:build` (three reports), `mit-bestand-praesentation-projektetage:build`, and `mit-bestand-praesentation-projektetage:dev` (headless smoke on 6050).

Brief item `print:build-flyer` does not exist: no `📋️project.json` and no package.json script contains a `build-flyer` target (only the LaTeX template `🧾️template/📰️flyer` remains, under `🧰️framework/🛍️products/📓️print`).

---

## 4. `e2e-orch-tools/plan.json` (course steps it runs, tool bazel)

Phases: cold, warm (repeats cold; each step `warm.mode` "zero" with maxSeconds 600), e2e, probe. Orchestrator `run.sh` needs `--go` for cold, warm, e2e and probe; `preflight` and `report` run without.

Course (`products.course`, order 3, cargoLane `ecourse`, ports 6061, 6063, 8791, 8793, needs playwright, bazel, node):

Cold steps, group `core`:
1. `proctor-build`: `repo run @teaching/proctor:build --configuration={courseMode}` (timeout 21600 s)
2. `typecheck`: `repo run @teaching/architecture-course:typecheck` (10800 s)
3. `test`: `repo run @teaching/architecture-course:test -- quick` (10800 s)
4. `build`: `repo run @teaching/architecture-course:build --configuration={courseMode}` (10800 s)
5. `check`: `repo run @teaching/architecture-course:check` (10800 s)
6. `proctor-test`: `repo run @teaching/proctor:test` (10800 s)

E2E: `test-e2e`, ticket script `e2e-tools/run-e2e.sh course --tool=bazel --topology={courseTopology}`, needs `core`, port 6061, timeout 21600 s. `courseMode` defaults to release, `courseTopology` to dev (`run.sh` flags `--course-mode`, `--course-topology`).
Probe: none.
Verify targets: `@teaching/architecture-course:test-e2e`, `:dev`, `:dev-site`.

Other products in plan.json (out of scope this wave, listed for the print overlap): `demo` (Mit Bestand Demonstrator) cold steps include `@semio-tech/mit-bestand-bericht:build-kompaktbericht`, `:build`, and `@semio-tech/mit-bestand-praesentation-projektetage:build` (all print-side); `play` has no print steps.

---

## 5. Minimal end-to-end proof per product

Run these with `cd /Users/ueli/Documents/semio` first. Each line is one `bun repo` call; `--tool=bazel` as in plan.json.

Dashboard:
```
bun repo --tool=bazel run @semio-tech/repo-dashboard-rs:build
bun repo --tool=bazel run @semio-tech/repo-dashboard-rs:test-cold-start
bun repo --tool=bazel run @semio-tech/repo-dashboard-rs:test-journeys
```
Source gates, no build needed: `bun repo --tool=bazel run @semio-tech/repo-dashboard-rs:test-language`, `…:test-dispatch`, `…:test-source-freshness`. The human entry `repo dashboard` is not part of an agent proof (AGENTS.md).

Course:
```
bun repo --tool=bazel run @teaching/proctor:build --configuration=release
bun repo --tool=bazel run @teaching/architecture-course:typecheck
bun repo --tool=bazel run @teaching/architecture-course:build --configuration=release
bun repo --tool=bazel run @teaching/architecture-course:test -- quick
bun repo --tool=bazel run @teaching/architecture-course:check
bun repo --tool=bazel run @teaching/proctor:test
sh "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh" course --tool=bazel --topology=dev
```
The last line is the e2e gate (plan.json uses it). A direct `bun repo --tool=bazel run @teaching/architecture-course:test-e2e` needs `topology` set to `dev` (CLI spelling of the dashboard parameter not checked).

Print:
```
bun repo --tool=bazel run @semio-tech/print-rs:test
bun repo --tool=bazel run @semio-tech/print:check
bun repo --tool=bazel run @semio-tech/print:test
bun repo --tool=bazel run @semio-tech/presentation:test
bun repo --tool=bazel run @semio-tech/presentation-react:test
bun repo --tool=bazel run @semio-tech/mit-bestand-bericht:build
bun repo --tool=bazel run @semio-tech/mit-bestand-praesentation-projektetage:build
bun repo --tool=bazel run @semio-tech/mit-bestand-praesentation-projektetage:dev
```
The last line is continuous (port 6050), stop it after the headless smoke. No flyer target exists.

---

## 6. Notes and open points

- Shared prerequisites reached by the course: `assets:build`, `ui-styling-tokens:generate`, `framework-rs:generate`, `ui-contract-rs:generate`, `ui-rs:generate`, `framework-actor-rs:typegen`, `proctor:build`. The root `prepare` target (`workspace`) also depends on `framework-schema:generate`, `framework-graph:generate`, `plugin-registry:generate`.
- Dashboard closure has no target-level dependsOn into other projects; it is Cargo-driven through `compile`.
- Print's `print:build` depends on `print:fonts`, `print:deps-tectonic`, `print:deps-tex`, `print:generate` (all within the print project).
- Playwright browsers (`workspace:deps-browsers`) gate course e2e and the ui-styling-tokens browser tests.
- Not checked: the full command text of every dashboard test script (`📜️script.ts`), and the dashboard launcher `repo dashboard`.

Files changed: this report only (`…/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-map-1024d.md`). No other files, no processes started.
