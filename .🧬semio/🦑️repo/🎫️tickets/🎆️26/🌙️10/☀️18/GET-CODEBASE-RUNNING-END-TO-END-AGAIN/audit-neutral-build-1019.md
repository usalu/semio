# 🧭️ Audit: Tool-Neutral Build Layer, Nx Coupling, Launch Surface (2026-10-19)

Read-only audit by `aud-neutral`. Static inspection of git-tracked files and `HEAD` (`bd9e4e02bd0`). No builds, no tests, no `bun repo` runs, no git-modifying commands, no sub-agents. Paths are repo-relative. Abbreviations: `B/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`, `L/` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `BS/` = `L/⚡️caching/🚀️bootstrap/`.

## 0. Blocking finding: the launch surface is absent

| Fact | Evidence |
|---|---|
| No `.vscode/` directory in the working tree and no tracked `.vscode/*` at `HEAD` | `git ls-files .vscode` returns 0 rows |
| Removed by `bd9e4e02bd0` (12:21, auto-commit, 9,970 files changed) | `git show --stat bd9e4e02bd0` lists `.vscode/launch.json` (-203,209), `.vscode/🧩️launch.seed.jsonc` (-129,137), `.vscode/extensions.json`, `.vscode/mcp.json`, `.vscode/settings.json`. Same commit also removes `.cursor/mcp.json`, `.devcontainer/editor-overlay.json`, `.github/workflows/architecture-quiz.yml` |
| Codex entrypoint report claims the launch surface exists and routes backend choice through `bun repo` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/entrypoint-2026-10-19.md` (section "Changes") |
| Only copies remain under overlays (`.🧬semio/🌐hub/*/.vscode/`) | `find` results |

Not restored. Decision needed from the user: restore from `bd9e4e02bd0^` (what the commit message calls "launch surface contracts") or treat the deletion as intended. If restored, see section 4 for what must change first.

## 1. Neutral build model

### 1.1 Schema and tool selection

| Item | Location |
|---|---|
| Neutral build schema, `BuildTool` enum `["nx","bazel"]` | `B/🧬️schema/🔣️.json:544` (`$defs.BuildTool`); `BuildPolicy` and `BuildPolicyAuthority` in the same file |
| Policy file, `defaultTool: "nx"` | `B/🔣️policy.json:3` |
| Tool parse, precedence: `--tool` / `--tool=` argv, then `SEMIO_BUILD_TOOL` env, then `policy.defaultTool` | `B/🧭️invocation/🟦️.ts:126-137` (`parseBuildInvocationV1`) |
| Call site (`SEMIO_BUILD_TOOL` taken from capabilities) | `B/📜️script.ts:88` |
| Env propagation to children (`SEMIO_BUILD_TOOL: selected.tool`) | `B/📜️script.ts:100` in `runOwnedBuildInLeasesV1` (starts `:94`) |
| Dashboard UI parameter `build-tool` (values `nx`, `bazel` mapped to env) | root `📋️project.json` (metadata.semio.dashboard.parameters) |
| Selection validation (argv grammar for run, run-many, affected, exec, watch) | `B/🧭️selection/🟦️.ts:129-144` |
| CLI help text `bun repo [--tool=nx|bazel]` | `B/📜️script.ts:89`; test `🧪️tests/🧭️entrypoint/🟦️.ts:25` |

Note: there is no separate neutral operation schema. The operation surface is the argv grammar in `B/📜️script.ts:116-166` plus `validateBuildTaskArgumentsV1`.

### 1.2 Neutral operations and implementation status

| Operation | Neutral dispatch | nx | bazel |
|---|---|---|---|
| `run <project:target[:cfg]>` | `B/📜️script.ts:116`, `:168-172` | Yes. `nxBuildArgumentsV1` (`B/🧭️invocation/🟦️.ts:141-154`) then `runOwnedNxInvocationV1` (`BS/📜️script.ts:595`) | Yes. `selectBuildTasksV1` (`B/🧭️selection/🟦️.ts:59-66`) then `runBazelTasksV1` (`B/🔧️tools/🏗️bazel/📜️script.ts:340`, call at `B/📜️script.ts:195`) |
| `run-many --targets` | `selection.ts:73` | Pass-through to nx | Yes, same branch |
| `affected --targets` | `selection.ts:73-74` | Pass-through to nx affected (not verified) | Branch present; parity with nx not verified |
| `exec --projects -- <cmd>` | `selection.ts:67-68` | Pass-through (not verified) | Branch present (`selection.ts:67`) |
| `watch --projects -- <cmd>` | `B/📜️script.ts:167`; `selection.ts:98` | Shared watcher code, called before the tool branch | Shared |
| `--dry-run` / `--graph` | `invocation.ts:141-154` maps `--dry-run` to nx `--graph=stdout`; bazel uses `planBazelTasksV1` (`📜️script.ts:181`) | Yes | Yes (planning only, no execution) |
| `graph` / `show projects|project|targets` | `B/📜️script.ts:145-166` via `B/🕸️graph/🟦️.ts` (native TS inference) | Tool-independent, no nx call | Same |
| `dependencies sync|lock` | `B/📜️script.ts:120` | Tool-independent | n/a |
| `cargo` | `B/📜️script.ts:124` | Tool-independent | n/a |
| `tools setup [nx|bazel|all]` | `B/📜️script.ts:131-138` | `BS/📜️script.ts` `--version` probe | Bazel adapter `B/🔧️tools/🏗️bazel/📜️script.ts` |
| `reset` | `B/📜️script.ts:139-144` | `invalidateBuildGraphV1` + nx reset | `resetBazelBuildStateV1` (`B/🔧️tools/🏗️bazel/💾️storage/🗑️reset/🟦️.ts`) |
| `preflight` | `B/📜️script.ts:90` | Tool-independent | n/a |
| Cache control | No dedicated op. Flags `--skip-cache`, `--skip-remote-cache` (`selection.ts:94`, `invocation.ts:141-154`); dashboard "cache" parameter in root `📋️project.json` | Alias to `--skip-nx-cache` | `skipCache` in `BazelTaskRequestV1` |
| `test bazel [phase]`, `build canvas|storybook|...` | `B/📜️script.ts:56` (TestScript), `:25` (BuildScript) | n/a | `bazelTestCommandsV1` |

Verdict: the neutral layer is real and both backends are wired. Gaps are parity (affected, exec), no bazel runtime success (section 5), and no first-class cache operation.

## 2. How `bun repo run <project>:<target>` executes under nx today

Call chain:

1. Root `package.json` script `repo` runs `bun ./…/B/📜️script.ts repo` (root `package.json`, scripts block). `setup` and `dashboard*` run `bun repo run …`.
2. `B/📜️script.ts:81` `RepoScript.run` → `:86` `runOwnedBuildInvocationV1`.
3. `B/🧭️invocation/🟦️.ts:126` `parseBuildInvocationV1` selects `nx` (policy default).
4. Graph load: `loadBuildInvocationGraphV1` plus `withRepositoryBuildGraphArtifactV1` (`B/📜️script.ts`, ~lines 150-172).
5. `B/📜️script.ts:168-172`: `runOwnedNxInvocationV1(root, nxBuildArgumentsV1(args), …)`.
6. `BS/📜️script.ts:349` `NxScript` launches the Nx CLI resolved by `require.resolve("nx/bin/nx.js")` (`BS/🛠️tools/📜️script.ts`). The Nx binary lives in the tools partition; `/.nx/installation` is a symlink into `.cache/b/t/m/nx/…`.

Project graph inference (Nx does not infer on its own):

- `nx.json:82-89` loads one plugin: `L/🟨️.mjs`, `include: ["📋️project.json"]`.
- `L/🟨️.mjs:73` `nodes()` returns the portable graph publication (`graphPublication`). Nx consumes the neutral TS inference result.
- `L/🟨️.mjs:58` `projectBuildTargetForNxV1` maps each target to an Nx executor. Kinds: `@semio-tech/repo-lib:command`, `@semio-tech/repo-lib:owner-command`, `nx:noop`.
- Executor definitions: `L/📦️packages/🟦️typescript/📋️executors.json:3` (`command`) and `:8` (`owner-command`). The string `repo:script` appears only as the authored token in `project.json` files and the dashboard authority schema (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🧭️authority/🔣️.json:116`). Nx never sees it.
- Inputs: default `["default","^default"]` plus engine named input (`L/🟨️.mjs:60`). Outputs come only from `project.json`.

Caching:

| Item | State |
|---|---|
| `nx.json:91` `cacheDirectory` = `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts` | Dead path. Directory does not exist. Overridden at runtime |
| Runtime cache dir | `NX_CACHE_DIRECTORY` set from the bounded namespace (`L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts:65`) to `.cache/b/n/m/nx/artifacts` |
| On-disk state | `.cache/b/n/m/nx/` contains only `graphs/` (empty). No `artifacts/` directory exists. `.cache/b/n/m` holds 3 files in total |
| Verdict | Nx task caching is wired but not demonstrated. No artifact has ever been stored on disk. Acceptance run (`bun repo run @teaching/architecture-quiz:check`) aborts in the dependency bootstrap before Nx runs (`result-nxpart.md`, HANDOVER 2.5) |

Target declarations (3,792 targets in 1,448 tracked `📋️project.json`, excluding tickets):

| Field | Count |
|---|---|
| executor `repo:script` | 3,647 |
| executor missing (inherits the plugin default) | 144 |
| executor `repo:noop` | 1 |
| `cache: true` / `false` / unset | 841 / 1,716 / 1,235 |
| with `inputs` / `outputs` / `dependsOn` | 1,082 / 302 / 239 |
| `options.command` starts with `bun ./…📜️script.ts` | 3,791 of 3,792 |

## 3. Remaining direct nx coupling outside the nx adapter

Scope: production code excludes `BS/` and `B/🔧️tools/🏗️bazel/`, which the audit treats as the nx and bazel adapters. Tests are counted separately.

### 3.1 Production code

| Coupling | Count | Locations |
|---|---|---|
| `require`/`import` of `nx`, `@nx/*` | 6 lines in 4 files | `L/⚡️caching/📇️inventory/🟦️.ts` (2 lines: `nx/src/utils/workspace-root`, `@nx/devkit`, `nx/src/project-graph/nx-deps-cache`); `L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts` (2 lines: `nx/src/tasks-runner/create-task-graph`, `nx/src/hasher/...`, `nx/src/daemon/client/...`); `…/🎯️selection/📥️scope/🔌️sdk/🟦️.ts` (1); `…/🕸️graph/🔏️seal/📥️membership/📥️journal/👷️worker/📜️script.ts` (1, `nx/src/native`) |
| Direct Nx CLI spawn or Nx binary resolution | 2 | `L/⚡️caching/🔁️verification/📋️orchestration/🟦️.ts:92` (`nx/bin/nx.js run probe:…`; also writes an `nx.json` fixture at `:35`). `BS/🛠️tools/📜️script.ts` (adapter, acceptable) |
| Files mentioning `nx.json` (production, non-test) | 20 files, 29 lines | Mostly name lists (Go/Rust folder classifiers, OS MCP binary gate), `L/⚡️caching/📋️project.json:271` (`{workspaceRoot}/nx.json` input). Root workspace detection no longer uses `nx.json` (`result-nxneutral.md`) |
| Nx internals reached via sdk | 4 deep paths | `nx/src/daemon/client/daemon-environment`, `nx/src/utils/workspace-context` (frozen authorizers), `nx/src/native` Watcher, `nx/src/project-graph/nx-deps-cache` |

Top offenders by count in production: `L/⚡️caching/🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts` (2), `L/⚡️caching/📇️inventory/🟦️.ts` (2), `…/📥️journal/👷️worker/📜️script.ts` (1), `…/🎯️selection/📥️scope/🔌️sdk/🟦️.ts` (1).

### 3.2 Tests

| Coupling | Count |
|---|---|
| `nx`/`@nx` imports in test files | 38 lines across about 20 files. Top: `B/🧪️tests/🕸️graph/🟦️.ts` (7), `L/⚡️caching/🧪️tests/🕸️daemon/📥️journal/🟦️.ts` (6) |
| `nx` invocation or literal in tests and fixtures | 332 lines; largest: `🎛️dashboard/🧫️fixtures/♻️reuse/🔣️.json` under `🔨️modules/` (63 lines, tool-literal fixtures) |

### 3.3 Product `package.json` scripts (AGENTS: "package.json MUST call nx")

Scope: 306 tracked `package.json` files, 711 scripts (root included).

| Form | Count | Example |
|---|---|---|
| `bun --cwd <dir> repo run <project:target>` (neutral) | 701 | `🔌️plugins/✒️writer/📦️packages/🟦️typescript/package.json`: `test => bun --cwd "../../../../.." repo run @semio-tech/writer-js:test` |
| `bun nx run <project:target>` (direct nx) | 6 | `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json`: `build`, `test`, `test:e2e`, `typecheck` via `bun nx run @teaching/architecture-course:*`; `✏️s/…/🦀️rust/package.json`: `verify-fem3d-numerical-child-{native,source}` |
| `bun repo run …` at root (neutral, no `--cwd`) | 3 | root `package.json`: `setup`, `dashboard`, `dashboard:install` |
| `bun ./…/B/📜️script.ts repo` (direct script, no nx) | 1 | root `package.json` `repo` |
| Names containing `nx-` (not nx calls) | 5 | `L/📦️packages/🟦️typescript/package.json`: `test:nx-project-inference*`, `test-nx-owner-arguments`, `probe-nx-owner-arguments` |

Sample of 20 across the groups requested:

| Group | Scripts | Sample |
|---|---|---|
| ✏️s | 357 | `bun --cwd … repo run @semio-tech/writer-writer:build` (neutral); `bun nx run @semio-tech/fem-3d-rs:verify-fem3d-numerical-child-native` (direct nx) |
| 🎓️teaching | 4 | `bun nx run @teaching/architecture-course:build` (direct nx) |
| 🏢️semio-tech | 5 | `bun --cwd "../.." repo run @semio-tech/semio-tech-play:build` (neutral) |
| ♻️mit-bestand | 5 | `bun --cwd "../.." repo run @semio-tech/mit-bestand-demonstrator:build` (neutral) |
| 🌎️hub | 13 | `bun --cwd "../../.." repo run os-hub-ts:test` (neutral) |

Verdict: the AGENTS rule "package.json MUST call nx" contradicts the neutral design the repo MCP describes ("bun repo run ... nx remains the default"). The 701 neutral scripts already follow the neutral rule. The 6 direct `bun nx run` scripts bypass the neutral layer and cannot run under bazel. The AGENTS text also says `📜️scripts.ts` (typo for `📜️script.ts`). AGENTS.md must not be edited by this agent; the user must decide.

## 4. `.vscode/🧩️launch.seed.jsonc` vs `.vscode/launch.json`

Both files are absent from `HEAD`. Last-known state at `bd9e4e02bd0^`, read with `git show`:

| Check | Result |
|---|---|
| Configurations in `launch.json` | 12,768 (all `node-terminal`, all `launch`); 4 compounds; 128 inputs; no duplicate names |
| `"name"` tokens in seed | 12,772 (equals 12,768 configs + 4 compounds, so the counts match) |
| Seed strict JSON parse | Fails: "Invalid control character at line 35302 column 27" (raw control byte). Content-level sync not proven |
| Entries calling nx directly (command or args) | 0 |
| Entries starting with `bun repo` | 11,673 |
| Entries whose text contains ` exec` (with `repo`) | 1,766 |
| Entries using `--tool` | 8 |
| Entries pointing into ticket folders (`.🧬semio/…/tickets/…`) | 3,557 (not registered neutral commands) |
| Entries mentioning `nx` in name or command | 57 (ticket scripts such as `nx-promotion`, `nx-selected-reader-test`) |

Verdict: launch entries are tool-neutral for the registered commands (`bun repo run`), not nx. But 3,557 entries bypass the neutral registry by running ticket-local scripts, which conflicts with "register all executable commands" and makes launch depend on ticket folders that are swept.

## 5. Prioritized gap list and disjoint work packages

### 5.1 Gaps (priority order)

| # | Gap | Evidence | Blocks |
|---|---|---|---|
| G1 | Launch surface deleted at HEAD | section 0 | All developer commands (AGENTS: "all devs use launch.json") |
| G2 | Seed is not valid JSON (raw control byte at 35302:27) | section 4 | Any regeneration |
| G3 | Acceptance never reached (`@teaching/architecture-quiz:check`); dependency sync stale since 19:01 | HANDOVER 2.3, 2.5 | Every nx and bazel run |
| G4 | Nx task cache unproven; `nx.json:91` dead path | section 2 | Caching claims |
| G5 | 6 direct `bun nx run` scripts in ✏️s and 🎓️teaching | section 3.3 | Bazel parity; neutral rule |
| G6 | 6 production nx import sites and one nx-spawning verifier outside the adapter | section 3.1 | Adapter boundary; tool neutrality |
| G7 | Bazel never executed; `node_modules` dangling; `@semio-tech/schema-validator-ts` unresolved (codex `bazel-2026-10-19.md`) | HANDOVER 2.10 | Bazel acceptance |
| G8 | Parity unverified for `affected`, `exec`, and `--dry-run` | section 1.2 | Neutral contract |
| G9 | No first-class cache op; cache control only through flags | section 1.2 | Interaction-friendly cancel and cache UX |
| G10 | 34 tracked hash-named `bun.lock` snapshots (`.333ee2a`, `.3a0f1e4`, `.48ebeba`, `♻️mit-bestand/.1f1b746`, …), last touched in `8f91299152c` | `git ls-files` | Repo hygiene, repeated lock churn |
| G11 | AGENTS line "package.json MUST call nx" conflicts with neutral design | section 3.3 | Standing rule; user decision |
| G12 | 3,557 launch entries run ticket-local scripts | section 4 | Launch registry correctness |

### 5.2 Suggested disjoint work packages

| WP | Owner scope (exclusive files) | Task | Depends on |
|---|---|---|---|
| WP-L (launch) | `.vscode/**`; a new launch-generation subcommand in the root `📜️script.ts` only | Decide G1 (restore from `bd9e4e02bd0^` or retire). Fix seed JSONC (G2). Re-point the 3,557 ticket entries to `bun repo run` / `bun repo exec` (G12). Regenerate `launch.json` | User decision on G1 |
| WP-N (nx boundary) | `L/⚡️caching/📇️inventory/🟦️.ts`; `L/⚡️caching/🕸️graph/🔏️seal/**/🔌️sdk/🟦️.ts`; `…/📥️journal/👷️worker/📜️script.ts`; `L/⚡️caching/🔁️verification/**` | Move the 6 nx import sites and the verifier's nx spawn behind `BS/` (G6). Behaviour unchanged, tests moved with them | None; coordinate with the cache owner |
| WP-C (nx config and cache) | `nx.json`; `BS/📜️script.ts`; `L/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/**` | Remove or correct `nx.json:91` (G4). Run one real `check` with `NX_CACHE_DIRECTORY` and show `nx/artifacts` populated and a second run served from cache. Gate on G3 | G3 (deps sync green) |
| WP-P (product scripts) | `✏️s/🔌️plugins/🏗️fem/…/🦀️rust/package.json`; `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json` | Replace `bun nx run X` with `bun --cwd <dir> repo run X` (G5). Verify each target exists in its `project.json` (teaching has `build`, `test`, `test-e2e`, `typecheck`) | Launch and build runnable first |
| WP-B (bazel runtime) | `B/🔧️tools/🏗️bazel/**`; root `MODULE.bazel`, `.bazelrc` | Run the dry-runs listed in HANDOVER 2.10, then one real `test`. Resolve G7 and G8. No success claim without a run | G3 |
| WP-D (deps) | `BS/📦️dependencies/**` | Get `bun repo dependencies sync --all` green (G3) | None; blocks WP-C and WP-B |
| WP-H (hygiene) | None to edit by agents. The user removes G10 files with `git rm` after approval | Agents must not run git-modifying commands (AGENTS) | User approval |
| Policy (user only) | `AGENTS.md` (must not be edited by agents) | Decide G11: keep "package.json MUST call nx" or change to "MUST call `bun repo run`" | User decision |

Ordering: decide G1 and G11 first (user). Then WP-D, WP-L and WP-N in parallel. WP-C and WP-B after WP-D is green. WP-P last.

## 6. Limits of this audit

- Static only. Nothing was built, run or tested. Every "not verified" above stays unverified.
- Counts use the scope stated per row. They differ from HANDOVER and `audit-nx-coupling.md` (for example 3,647 `repo:script` targets here vs 3,375 product-only there) because the scope differs.
- The seed was not parsed beyond the strict-JSON failure, so content-level seed/launch sync is unproven.
