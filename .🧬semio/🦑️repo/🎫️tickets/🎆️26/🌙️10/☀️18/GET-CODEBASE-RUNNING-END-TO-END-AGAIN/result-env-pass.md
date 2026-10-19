# Result env-pass (2026-10-19 19:10-20:35)

Executor `env-pass`. Task: run the 118 envelope-gated suites through `bun repo run <project>:<target>`.
Outcome: **0 of 118 suites run. Every `bun repo run` is blocked by the dependency sync and the sync itself fails on a code defect.** No source file was edited (also respects the 20:00 coordinator HARD FREEZE).

## Evidence

| run | target | result |
|---|---|---|
| 1 | `repo:test-caller-environment` (first attempt, 19:15) | queued 590 s on `native-fixed-image:.cache/b/d` shared access (deps sync held it) -> SIGALRM |
| 2 | `repo:test-caller-environment` (19:26-19:36, after the sync loop paused) | not queued on the image, but the run itself performs the dependency-store producer (progress resources `dependencies:store-ownership:state|list|readText:N`, `retire-obsolete:N`; those names exist only in `runJavascriptDependencyStoreProducerV1`, `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:448`). 87,528 wait lines, reached list:1,890,000 in 571 s, not finished -> SIGALRM |
| 3 | `workspace:test-build-tool-bazel` (executor bzl-suite, observed, not mine) | same producer: state -> list -> readText:1.23M (304 s) -> SIGALRM |
| 4 | deps4 loop `bun repo dependencies sync --all` sync30 (19:30-19:37) | `Bun current manifest changed during verification: …/🛠️tools/package.json` (peer churn) |
| 5 | sync31 (19:37-20:30), undisturbed, 53 min | walk reached javascript-installed-paths and javascript-source-provider, then **`error: Same canonical Native and prospective Source provider instance required`** (the open blocker already recorded in `result-deps4.md` attempt 14) |

Root cause chain: `repo run` -> zero-touch dependency readiness -> producer re-derives the full workspace ownership walk (~2-4 M operations, 25-50 min under load 35) whenever any package.json/lock/loaded module changed; the walk is not persisted when the process is killed by the 590 s alarm, so no alarm-bound caller can ever finish; the unbounded walk itself ends in the provider-instance defect. Two concurrent `repo run` plus the loop sync contend for the same exclusive leases.

Consequence for the 118 suites (`tst4-tools/envelope-gated.txt`): all are **queued/blocked**. I stopped issuing runs after run 2 so I would not compete with the sync loop for its exclusive leases.

## Suite -> target map (derived, not run)

Machine data: `env-pass-tools/map.json` (nearest project), `env-pass-tools/refs.txt` (script files that reference each suite path), `env-pass-tools/drive.sh <budgetSeconds> <perTargetSeconds> <project:target>...` (runs targets one at a time, logs filtered output, appends `status.tsv`).

Wired through an exact target (project `repo` = `⚡️caching/📋️project.json`, dispatcher `⚡️caching/📜️script.ts` `inferenceTests`):
- `repo:test-caller-environment` -> #52; `repo:test-playground-preparation-source` -> #53; `repo:test-import-edges` -> #55,#56,#57; `repo:test-pruning` -> #59,#61,#62; `repo:test-graph-seal` -> #29; `repo:test-graph-cache-waiter` -> #26; `repo:test-graph-publication` -> #27; `repo:test-task-child` -> #36; `repo:test-toolkit-use` -> #37; `repo:test-toolkit-outputs` -> #17; `repo:test-rust-units` -> #51,#6; `repo:test-repository-exact-cargo` -> #66; `repo:test-cargo-launcher` -> #70; `repo:test-project-admission` -> #74; `repo:test-graph-bounded` -> #12; `repo:test-sdk-snapshot` -> #41; `repo:test-native-input-vocabulary` -> #105.
- `@semio-tech/repo-lib:test-build-surfaces` -> #108, `test-locale-law-ownership` -> #109 (packages/typescript script).
- Own projects: `repo-vite-storage` #4, `repo-nx-output-custody` #17, `repo-build-reports` #75, `repo-storybook-storage` #76, `repo-graph-maintenance` #80-82, `repo-build-file-membership` #83, `repo-task-closure` #92, `repo-configured-source` #96, `@semio-tech/cargo-workspaces` #98-104, `repo-next-storage` #111,#112, `repo-next-inputs` #113,#114, `@semio-tech/repo-test-domain` #115-118 (target names to be read from each `📋️project.json`).
- Dashboard #2,#3: `🎛️dashboard/📦️packages/…/📜️script.ts` (needs the dashboard install, not available).

Not reachable from any target I could find by path reference (candidate wiring gaps, to verify once runs work by diffing the files bun prints): #1, #5, #7, #8, #10, #11, #13-16, #18-25, #28, #30-35, #38-40, #42-50, #54, #58, #60, #63, #64, #65, #67-69, #71-73, #77-79, #84-91, #93-95, #97, #106, #107, #110. Many of these are probably covered through the aggregate `test` targets of `repo-lib` (`test-<slug>` arrays in `📦️packages/🟦️typescript/📜️script.ts`) or the generic `test` target; the string search cannot see globbed/derived suite lists.

## Needed to unblock (not done: needs library edits, frozen)
1. Fix `Same canonical Native and prospective Source provider instance required` in the dependency producer (owner deps4, `…/📦️dependencies/💾️storage/🟦️.ts` ~line 344-347 `sourceProvider`/`provider`), then get one green `bun repo dependencies sync --all` with no package.json edits during its ~50 min walk.
2. Persist the ownership walk (or make `repo run` refuse with a clear "dependencies not ready, run sync" instead of silently re-running the 50-minute producer inside every run); today every alarm-bound `repo run` is a doomed sync.
3. After the green sync: `env-pass-tools/drive.sh 560 280 repo:test-caller-environment …` over the list above, then fix failures at the root cause.

## Hygiene
No file under the frozen paths edited. Temporary logs under `🗑️generated/env-pass` deleted. No modifying git command, no sub-agent, no cargo, no `run-many`/bare `verify`; two `bun repo run` attempts of mine (both killed by SIGALRM) are the only runs started.
