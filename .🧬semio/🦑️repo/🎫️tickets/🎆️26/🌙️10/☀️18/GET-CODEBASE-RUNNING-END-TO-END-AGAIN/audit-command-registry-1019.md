# Audit Command Registry, 2026-10-19 (aud-cmdreg)

Read-only. No source edited, no build, no `bun repo run`, no test executed. Evidence is static: `jq`, `rg` and file reads of `📋️project.json` files, the dashboard module (`🎛️dashboard`) and the build CLI (`🏗️build/📜️script.ts`). Anything marked "not run" is unverified at runtime.

## 1. Authoritative command surface

- **The dashboard registry is the only surface.** `🎛️dashboard/README.md` ("The dashboard is the only control plane for developers, editors and agents") and `🎮️registry` ("the one source of what can be started"). Interactive view, `semio commands|run|tasks|logs` and the daemon share it.
- **Derived automatically:** every target of every `📋️project.json` (walked by `📚️inventory/🦀️.rs`, `PROJECT_MANIFESTS`). Verb = first name segment if it is in `VERBS`, else `task` (`🪪️command-identity/🦀️.rs:120-124`). Examples: `test-e2e` to `test`, `typecheck-project` to `typecheck`, `deps-browsers` to `task`.
- **Authored:** `metadata.semio.dashboard` in `📋️project.json` (`parameters`, `tools`, `compounds`, `groups`, `ready`, `requires`), schema `🎮️registry/🔣️.json`. The root project holds the global axes (`build-tool`, `cache`, `test-level`, `dependencies`, `build-mode`, `nextest-output`, `cargo-jobs`, `build-budget`) and the root `tools` list. Course declares `compounds` (`course-with-proctor`). Other declarations are in play, demonstrator, hub, dashboard and developer-tools.
- **Other sources:** playground variants from `[[package.metadata.semio.playground]]` through the generated catalog; ticket tools from `🎮️commands.json` in the open ticket folder; repo-domain actions (`repo:tickets…`) in process.
- **No IDE launch projection exists in code.** `.vscode/` is absent from disk and HEAD (`bd9e4e02bd0` deleted `launch.json`, the seed, `settings.json`, `mcp.json`, `extensions.json`: 5 files, 332,543 deletions). `.claude/` holds only `settings.json` and `settings.local.json`, so there is no `.claude/launch.json`. The only generated editor-adjacent file is the headless agent client `.mcp.json`, derived from registry `mcp` declarations (`🤖️agent-clients/🟦️.ts:34`, `workspace:agents-write`).
- **A restoration is still an open user decision, not a plan in code.** `audit-products-1019.md` P0 WP-0: "Decide: restore `launch.json` and seed from `bd9e4e02bd0^`, or confirm the dashboard metadata replaces it. Then fix AGENTS conflict. Needs your decision." `brief-1019.md` and the brief for executors say "user decision pending". Code asserts the opposite: `🧪️tests/🧭️authority/🟦️.ts:352-356` (both launch files must be absent), course `🎓️…/🧪️tests/🎛️dashboard/🟦️.ts:50`, and the library `test-dashboard-control` fixture `absent` list.

## 2. Reachability of this session's commands

| Command | Registry entry | Status |
| --- | --- | --- |
| `bun repo tools setup bazel` | root tool `build-tools` (`tool` choice `all\|nx\|bazel`) | reachable |
| `bun repo cache status\|clear` | CLI op exists (`🏗️build/📜️script.ts:154`, `runBuildCacheOperationV1`); no registry tool. Only `build-reset` (`reset`) exists | **GAP G3** |
| `bun repo --tool=bazel run …` | axis `build-tool` sets `SEMIO_BUILD_TOOL` (`bazel`); offered only for verbs `test check build verify lint serve activate task run dev clean generate reset inspect watch setup` (`🧬model/🦀️.rs:65` `applies`) | reachable for those verbs; **not** for `publish`, `prepare`, `package`, `deploy`, `start`, `format`, `smoke`, `gate`, `bench`, `preview` (**GAP G5**) |
| `bun repo affected --targets=…` | root tool `build-affected` (`targets`, `mode`) | reachable |
| `exec --dry-run` | CLI supports it (`⚙️operations/🔣️catalog.json:58`). Tools `bun-test`, `run-script`, `cargo` call `bun repo exec` with no dry-run parameter, and extra arguments go after `--` to the child program | **GAP G4** |
| `bun repo run … --dry-run` | root tool `build-task` flag `dry-run` | reachable |
| `@teaching/architecture-course:dev/test/test-e2e/build/publish` | all five targets exist (`📋️project.json`); auto-registered with verbs dev, test, test, build, publish; compound `course-with-proctor` declared | reachable; `publish` gets no `build-tool` selector (G5) |
| `@teaching/proctor` chain | `dev`, `test`, `build` exist; no `publish` target exists | reachable; no publish target (not a registry gap) |
| `@semio-tech/semio-tech-play`: `activate-prototype`, `serve`, `test-e2e` | exist, auto-registered (verbs activate, serve, test) | reachable; `prepare-prototype`, `prepare-e2e`, `prepare-release` (verb `prepare`) get no `build-tool` selector (G5) |
| `@semio-tech/mit-bestand-demonstrator`: `activate-prototype`, `serve`, `test-e2e` | exist, auto-registered | same caveat as play (G5) |
| `workspace:deps-browsers` | root target `deps-browsers` (`repo:script` → `sync browsers`); auto-registered with verb `task`; `setup` depends on it | reachable |
| `repo-mcp:build` | project `repo-mcp` (`🔌️mcp/📋️project.json`), targets `build`, `test`; also tool `repo-mcp` (`dev`) | reachable |
| `@semio-tech/framework-os-mcp-rs:build` | project in `💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust`, targets `build`, `dev`, `test`, `publish`; also tool `os-mcp-http` | reachable |

Note on nesting: the repo-mcp instructions say nested tasks inherit `SEMIO_BUILD_TOOL`. So `prepare-*` may get bazel when reached through a chain started with `build-tool=bazel`. Not verified at runtime.

## 3. Dashboard buildability and pinning tests

- **Targets of `@semio-tech/repo-dashboard-rs`:** `build`, `compile`, `install`, `run`, `daemon`, `preferences`, `test`, `test-quick`, `test-unit`, `test-authority`, `test-execution`, `test-windows`, `test-journeys`, `test-smoke`, `test-load`, `test-coverage`, `test-long`, `test-exhaustive`. Root scripts: `dashboard` (`:run`) and `dashboard:install` (`:build --exclude-dependencies --skip-cache`).
- **Status (not run):** an installed executable exists (`.🧬semio/🦑️repo/⚡️cache/tools/dashboard-cli/713c…/semio`, 5 MB, 2026-10-09; record `⚡️cache/🎛️dashboard/installed.json` version 2). 170 files under the dashboard module are newer than that record, so the next `semio`/`dashboard` start will rebuild by design. `daemon.pid` names PID 99855, which is not alive, so the daemon pid file is stale.
- **Tests that pin the registry:**
  - `🎮️registry/🟦️.ts`: independent TS oracle over the frozen `🏗️workspace.json`, runs the native `semio` binary (`commands --json`, `commands --check`, `run --dry-run`), Ajv against `🧬️schema/🎮️registry/🔣️.json`, XXH3 vectors. Run by the default dashboard `test` path (`📦️packages/🦀️rust/📜️script.ts:81`).
  - `🎮️registry/🥒️.feature` mapped to `🎮️registry/🧪️tests/🔬️unit/🦀️.rs`; `📚️inventory/🧪️tests/🔬️unit/🦀️.rs`; `🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs` and `🌳️command-tree-projection`.
  - `🧭️cli/🟦️.ts` (CLI verbs), `🧪️tests/🧭️authority/🟦️.ts` (launch absence; AGENTS protected text), `🧪️tests/🧭️authority/🥒️.feature`.
  - Course `🎓️…/🧪️tests/🎛️dashboard/🟦️.ts`, validating the dashboard fixture; course test config includes it.
  - Library `test-dashboard-control` (`📚️library/🧪️tests/🎛️dashboard-control/`, schema `🧬️schema/🔣️.json`, fixture `🧫️fixtures/🔣️.json`).
- **Pinned stale references:** see G2 and G6.

## 4. Gap list

| ID | Severity | Gap | Evidence |
| --- | --- | --- | --- |
| G1 | P0, user decision | AGENTS.md line 51 says devs use `launch.json` and "never use the cli". This contradicts the dashboard and `semio`. The sentence is also pinned as `protectedInstructions` in `🧪️tests/🧫️fixtures/🧭️authority/🔣️.json` (schema `🧬️schema/🧭️authority/🔣️.json:301`), and `🧭️authority/🟦️.ts:105` treats the AGENTS row as modeled. Changing AGENTS.md needs a coordinated fixture change, and we may not edit AGENTS.md. | AGENTS.md:51; authority fixture and test above |
| G2 | P0 | Play fresh-build test still expects IDE launch files: `🏢️…/🎡️play/🧪️tests/🧪️playfreshbuild/🧫️fixtures/🔣️.json` `launch.files` = `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json`. `🟦️.ts:40-43` and `:87` read them, so it will throw on the missing files. Not run. | playfreshbuild fixture and test |
| G3 | P1 | `bun repo cache status\|clear` has no registry entry, so it is not in the dashboard, `semio commands` or the launcher. | `🏗️build/📜️script.ts:154`; root `tools` list has no cache entry |
| G4 | P1 | `exec --dry-run` is not reachable. `bun-test`, `run-script` and `cargo` tools have no dry-run parameter, and extra arguments go to the child command. | `⚙️operations/🔣️catalog.json:58`; root tools `bun-test`, `run-script`, `cargo` |
| G5 | P1 | The `build-tool` (and `cache`) axis is scoped to a verb list that omits `publish`, `prepare`, `package`, `deploy`, `start`, `format`, `smoke`, `gate`, `bench`, `preview`. Bazel cannot be selected for `@teaching/architecture-course:publish`, play/demonstrator `prepare-*`, or `deploy-check`. | root `parameters[build-tool].appliesTo.verbs`; `🎮️model/🦀️.rs:65` |
| G6 | P2 | Registry unit tests still use the pre-rename path `🎓️teaching/❓️quiz/…/📋️project.json` (`🎮️registry/🧪️tests/🔬️unit/🦀️.rs:446`; `📚️inventory/🧪️tests/🔬️unit/🦀️.rs:63`). These are synthetic, but they keep the old course name alive. | the two lines |
| G7 | P2 | `HANDOVER.md:232` cites Go test `TestSystemPolicy/the_checked-in_devcontainer_block_equals_its_derivation` (`.vscode/settings.json` drift). Searching the working tree found no such test, and `.vscode/settings.json` is deleted. The reference is stale; the devcontainer derivation source is gone. Needs a check before anyone relies on it. | `rg` over the tree (no match); HANDOVER:232 |
| G8 | P2 | Installed dashboard executable is stale against 170 changed sources and the daemon pid is stale. The first start rebuilds (by design). Expect a long first start. | `⚡️cache/🎛️dashboard/installed.json`; `daemon.pid` |
| G9 | P3 | Leftover `.vscode/launch.json` name used as a synthetic path (harmless). | `📚️library/🧪️tests/🪡️planner-blockers/🟦️.ts:37` |

Not gaps: `workspace:deps-browsers`, `repo-mcp:build`, `framework-os-mcp-rs:build`, `tools setup bazel`, `affected`, `run --dry-run`, the course five targets, play/demonstrator `activate-prototype`/`serve`/`test-e2e`.

## 5. Suggested executor task

Title: "Close command registry gaps G3, G4, G5 and fix G2, G6". Schema-first, foreground, one owner for the root `📋️project.json` (it is shared; re-read before edit).

1. Add registry tools to the root `metadata.semio.dashboard.tools`: `cache-status` (`bun repo cache status`), `cache-clear` (`bun repo cache clear`), and a dry-run parameter for `bun-test`, `run-script`, `cargo` (where the exec form applies). Confirm each with `semio commands --check` after a build in a slice, not now.
2. Widen the `build-tool` and `cache` `appliesTo.verbs` to cover `publish`, `prepare`, `package`, `deploy`, `start`, `format`, `smoke`, `gate`, `bench`, `preview`. First verify that the build CLI honours `SEMIO_BUILD_TOOL` for those verbs.
3. Update `🧪️tests/🧫️fixtures/🎮️registry/🏗️workspace.json`, the oracle `🎮️registry/🟦️.ts` and `🧭️authority` expectations for the new entries.
4. Retire or rewrite play `🧪️playfreshbuild` so it stops expecting `.vscode/` files (G2).
5. Replace the `quiz` paths in the two registry unit tests (G6).
6. Ask the user (G1) whether the AGENTS.md launch sentence is retired. Only with consent: edit AGENTS.md and the authority fixture `protectedInstructions` in one change.
7. Verification to run later, each in the foreground: `bun repo run @semio-tech/repo-dashboard-rs:test-unit`, `…:test` (registry oracle), `bun repo run @teaching/architecture-course:test` (dashboard test), `bun repo run @semio-tech/semio-tech-play:test` (playfreshbuild), library `test-dashboard-control`, and `semio commands --check`.

## 6. Limits

- Static reads only. Nothing was built, executed or validated at runtime.
- Verb derivation and `appliesTo` gating were read from source, not exercised.
- G7 is a negative search over the working tree (`rg`), not a git history search.
