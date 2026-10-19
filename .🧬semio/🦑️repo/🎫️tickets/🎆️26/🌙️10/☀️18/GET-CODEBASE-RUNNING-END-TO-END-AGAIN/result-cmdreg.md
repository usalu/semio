# Result cmdreg: command registry reaches every executable command (G2, G3, G4, G5)

`R/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/`. No git-modifying command, no sub-agent, no dashboard rebuild or install, `AGENTS.md` and its pinned protected text untouched, `.vscode/` untouched.

## Changes

Root `📋️project.json`, `metadata.semio.dashboard` (schema-first: validated against `R/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json` `ProjectManifest`):
- G5 tool selector: axis `build-tool` (`SEMIO_BUILD_TOOL=nx|bazel`) now applies to every verb of the registry: added `start prepare package typecheck smoke gate format publish deploy preview bench` (`typecheck` was also missing). It now equals `VERBS` (24, from `R/🎛️dashboard/🎮️registry/🪪️command-identity/🦀️.rs`) plus the fallback `task` plus the tool verbs `reset inspect`. The CLI reads the backend from the environment independent of the verb. Reaches `@teaching/architecture-course:publish`, play and demonstrator `prepare-*`, `deploy-check`, `activate-*`, `serve`, `test-e2e`.
- G5 `cache` axis (`--skip-cache`): added `generate start prepare package typecheck smoke gate format publish deploy preview bench`. Left out on purpose: `dev run watch clean reset inspect setup` (continuous or no task cache; `run` would clash with the own `cache` parameter of tool `run-targets`).
- G3: new tools after `build-reset`: `build-cache-status` (verb `inspect`, `bun repo cache status`) and `build-cache-clear` (verb `reset`, `bun repo cache clear`); the `build-tool` axis applies to both verbs, so `--tool` selection works for them.
- G4: new tool `build-exec` (`bun repo exec --projects={projects} --dry-run={dry-run} --`, command words in Arguments). The exec forms `bun-test`, `run-script`, `native-cargo` gained the same `dry-run` choice (`false` default, `true`). It is a token inside the pre-`--` words because extra tool arguments go after `--` to the child program; `--dry-run=false` is a no-op in the CLI (`validateBuildTaskArgumentsV1` boolean, `nxBuildArgumentsV1` drops it, exec plan detection only on bare/`=true`). `run-targets` and `build-affected` (no `--` in their command) gained a `dry-run` flag (`--dry-run`), like `build-task`.
- G2 fresh Play build: the IDE launch entries `📦️build🏢️semio-tech🎡️play🆕️fresh` and `📦️build🦑️workspace🚀️release🆕️fresh` are replaced by a registry parameter `generation=fresh` (env `SEMIO_NATIVE_BUILD_GENERATION=fresh`) declared on target `build` of `🏢️semio-tech/🎡️play/📋️project.json` and on root target `release`. Together with the global axes `build-mode=ship` (release env + `--skip-cache --skip-remote-cache`) it reproduces the former launch.

Tests and fixtures:
- `🏢️semio-tech/🎡️play/🧪️tests/🧪️playfreshbuild/🧫️fixtures/🔣️.json`: `launch` replaced by `retiredLaunchFiles` (the two `.vscode` files, must be absent) and `dashboard` (registry project, parameter, value, axis ids); unused `environmentProtocol` removed.
- `…/🧪️playfreshbuild/🟦️.ts`: asserts the retired files are absent and that the registry (play `generation` + root `build-mode`/`cache` axes) reproduces the fresh arguments and environment; same for the workspace `release` target. Dropped the `jsonc-parser` use.
- `R/📚️library/🧪️tests/🎛️dashboard-control/🧫️fixtures/🎮️owners.json` (language-agnostic data): rows for the new tools and dry-run variants, existing exec rows now carry `--dry-run=false`, new `axisVerbs`.
- `R/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts`: `build-tool` verbs must equal the Rust `VERBS` (parsed from source) + fallback + tool verbs, `cache` verbs = `VERBS` minus the uncached set + fallback, cache tools use applied verbs, every `bun repo exec` tool has the pre-`--` `dry-run` choice, flag-form dry-run on the other tools; new test that every preview/cache word of the registry is accepted by `validateBuildTaskArgumentsV1` and translated by `nxBuildArgumentsV1` (no `--dry-run*` leaks to Nx, `--graph=stdout` only when planning).
- `R/🎛️dashboard/README.md`: daily-work rows for `build-exec` and the cache tools, `build-tool` named among the global axes.

Files: `📋️project.json`, `🏢️semio-tech/🎡️play/📋️project.json`, the playfreshbuild fixture and test, dashboard-control `🟦️.ts` and `🧫️fixtures/🎮️owners.json`, `R/🎛️dashboard/README.md`, this report.

## Verification (exact)

All from `/Users/ueli/Documents/semio`.
- `bun test ./R/📚️library/🧪️tests/🎛️dashboard-control/🟦️.ts` -> 9 pass, 1 fail. The 1 failure (`ordinary build entry launches native dashboard commands…`, fixture row `@semio-tech/repo-dashboard-rs:workflow status`: the test's SQL oracle says `build`, the row expects `native`) is in a fixture row and a build-entry route that other work changed; not touched by this task. My tests (`native owner controls…`, `exec preview words…`) pass; the manifest also validates with Ajv2020 and the own schema compiler inside that test.
- Independent Ajv2020 validation of `ProjectManifest` for `📋️project.json` and `🏢️semio-tech/🎡️play/📋️project.json`: both true.
- `cd 🏢️semio-tech/🎡️play && ../../node_modules/.bin/vitest run --config ./🧪️tests/🎚️config/🟦️.ts ./🔨️modules/📦️site/🆕️fresh-build/🟦️.ts` -> 10 passed (`bun ./📜️script.ts test` itself needs the process envelope and refuses outside `bun repo`).
- `bun test ./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎛️dashboard/🟦️.ts` -> 4 pass.
- `bun test ./R/🎛️dashboard/🧪️tests/🎮️registry/🟦️.ts` with `SEMIO_DASHBOARD_BIN` = installed binary `713c31da…` (2026-10-09), `SEMIO_BUILD_MODE=prototype`: 6 pass, 5 fail. Without the override it throws "Dashboard is not installed". The binary is stale against the source (error "workspace:test: unknown parameter cache; it accepts none"); the oracle reads the frozen `🏗️workspace.json`, not the root manifest, so these failures are independent of this change. Needs the coordinator's dashboard rebuild.
- `bun test ./R/🎛️dashboard/🧪️tests/🧭️cli/🟦️.ts` (same override): 2 pass, 1 fail (`SEMIO_SCRIPT_PROCESS_INVOCATION` null check) = stale binary, same cause.
- `bun test ./R/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts`: not completed. It walks about 820,000 source directories and hit my 560 s alarm; not re-run.
- Rust: `perl -e 'alarm 590; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- test -p semio-framework-repo-dashboard-registry --lib` -> not run to tests. `cargo metadata --locked --offline` failed: "cannot update the lock file Cargo.lock because --locked was passed" (the lock does not match the current manifests; the dependencies owner is working on it). No Rust source was changed by this task.
- `semio commands --check` and `semio run --dry-run` against the new tools: not run (needs the rebuilt dashboard binary).

## Open items
- After the coordinator rebuilds the dashboard: `semio commands --check`, then `semio run --dry-run tool:workspace/build-exec --parameter projects=workspace --parameter dry-run=true`, `tool:workspace/build-cache-status` with `build-tool=bazel`, and the TS registry oracle without override.
- G6 not done: the synthetic `@fixture/quiz` workspace and the path `🎓️teaching/❓️quiz/…` are used in the frozen `🏗️workspace.json`, registry and inventory unit tests (about 40 lines in 6 Rust and TS files). A rename needs a Rust test run, which was blocked.
- G1 (AGENTS.md launch sentence and pinned `protectedInstructions`) untouched per the user decision.
- Dashboard-control fixture row `…:workflow status` vs the build-entry route (see above) needs its owner.
