# Round 2 Audit: Developer Configuration Surfaces And Execution Mechanisms

Date: 2026-10-08. Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`. Brief: `r2-fleet-brief.md`. Binding design: `fleet-plan.md` §1–§6.

Method: read-only. Nothing tracked was edited, no git-modifying command ran, no install, no daemon, no Nx task was started. Evidence comes from file reads, `git grep`, `jq`/Bun JSONC parsing, and the published Nx project graph (`.nx/workspace-data/project-graph.json`, 1,272 nodes, written 2026-10-08 02:37). Helper: `r2-validate-run-refs.ts` (this folder); its output is in `🗑️generated/run-resolution.json`. Other agents' files in `🗑️generated/` were left alone. `AGENTS.md` was not edited.

Verdict codes: KEEP = canonical as-is. DERIVE = generated from named canonical source X. DISSOLVE = remove, replaced by the dashboard or by a canonical source.

## 1. Findings That Matter

1. The dashboard is not yet the only control plane. Four other entry paths start processes today: the VS Code Run panel (`.vscode/launch.json`, 4,667 rows), Claude Code preview (`.claude/launch.json`, 79 rows), 134 root `package.json` scripts, and the native `semio` fallback that spawns `bun ./📜️script.ts` for any unknown verb (`🎛️dashboard/📜️root-delegation/🦀️.rs`, dispatch at `🎛️dashboard/📦️packages/🦀️rust/🦀️.rs:52-68`).
2. The Rust registry does not read the launch files. `🎮️registry/🦀️.rs` declares its sources as Nx targets, root scripts, the playground catalog, `metadata.semio.dashboard`, `🎮️commands.json` of open tickets and the repo domain. No production source in the dashboard references `.vscode/` or `launch.json` (the only hits are in `🧪️tests/` and `🧫️fixtures/`). The dashboard README (lines 19–24, the launcher section) still says the launcher offers `.vscode/launch.json` configurations. That sentence is stale. The launch files can be removed without a dashboard code change, but six gates and tests read them (§4.4).
3. The launch files are broken and drifting. 21 rows in `.vscode/launch.json` and 18 in the seed reference Nx targets that do not exist; 1 row in `.claude/launch.json` does too. All 7 `🛠️dev🎛️dashboard*` rows call `@semio-tech/repo-cli-rs` targets `run|daemon|workflow|install|preferences`. The dashboard project is `@semio-tech/repo-dashboard-rs`; `repo-cli-rs` has only `build check mcp repo test test-quick test-long test-exhaustive`. The "Dashboard" Run-panel entry therefore cannot start the dashboard.
4. The launch file is a hand-maintained orphan. The `📇️registry/🚀️launch/` generator no longer exists (the directory is not tracked). `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json:154-155` already records that "no generator reads it and the recording ticket deletes it". Launch rows have not been regenerated from the seed: 1,195 names exist only in `launch.json`, 33 only in the seed.
5. Launch rows encode ticket and environment state. 1,665 of 4,667 rows point into `.🧬semio/.../🎫️tickets/` (command or cwd). 1,925 rows pin `NX_DAEMON`, 2,626 carry an `env` block. The dashboard's own rule (README, §2.2 of the fleet plan) says these are never stored.
6. The MCP server definition exists in four copies that drift. The `semio` gateway points at `--folder .🧬semio/🔗space/os-mcp` in `.mcp.json`, `.vscode/mcp.json` and `.cursor/mcp.json`, and at `--folder .` in `.codex/config.toml`. That folder does not exist in this checkout and is gitignored (`.gitignore`, `.🧬semio/🔗space/`). Three of the four `semio` entries use it, so it is a candidate cause of the `semio` CONNECT_TIMEOUT seen in this session (UNVERIFIED; the codex entry uses `.`).
7. Hook configuration is three disabled copies plus a switch in `📋️config.toml`. The `.cursor/hooks.json`, `.github/hooks/repo.json` and `.claude/settings.json` hook blocks are commented out. Their commands (`bun ./📜️script.ts dev mcp stdio <client> hook <Event>`) cannot work: `runMcpStdioRepo` accepts exactly one profile argument (`📜️script.ts`, `runMcpStdioRepo`, about line 490). `.github/hooks/compose-repo.json` is a byte-identical duplicate under a stale name.
8. The devcontainer lifecycle calls a binary that is not built. `lifecycle/🟦️.ts:420-426` runs `target/release/semio configure --repo`. `target/` does not exist. `semio` is the dashboard crate's binary name (`🎛️dashboard/📦️packages/🦀️rust/Cargo.toml:18-20`), and the repo CLI's binaries are `repo` and `semio-repo`. The Go `configure` verb itself says "repo config generation is disabled; edit checked-in config files manually" and only removes git hooks and installs micro-commit hooks (`💻️client/⌨️cli/🧩️component/🐹️.go:39211`). The success message "Repo hook configuration synced" is therefore misleading.
9. There is a direct policy conflict on `.vscode/settings.json`. The repo CLI's `systemPolicy` (`🧩️component/🐹️.go:20765`) flags `.vscode/settings.json` as a breach that must move into `.devcontainer/devcontainer.json`, and the same policy requires extension recommendations to be derived from devcontainer. The tree still has `.vscode/settings.json` (78 keys). The devcontainer copy holds 28 of those keys, five with different values. Native Windows and macOS developers get settings only from `.vscode/`, so the policy conflicts with the native-host rule in AGENTS.md.
10. Root `package.json` has 134 scripts, of which 4 are bootstrap-only (`nx`, `setup`, `dashboard`, `dashboard:install`). The other 130 are aliases of Nx targets or variants the dashboard already discovers (§6).
11. Several declarations are stale or lethal. `📋️project.json` (root) puts `gemini --yolo` and `kiro-cli chat --trust-all-tools` into the dashboard tools; both disable tool approval prompts (owner decision, §7). `.vscode/settings.json` points the LaTeX Workshop build at `mit-bestand/bericht/📜️script.ts`, which does not exist (the directory is `♻️mit-bestand/📋️bericht/`, no script). Dependabot has 14 of 15 directory entries pointing at paths that do not exist (`/js/compose`, `/py/compose`, `/net/Compose`, `/go/cli`, `/rs/compose` and others).

## 2. How The Dashboard Starts Today

Chain for `bun run dashboard` (root `package.json`):

1. `dashboard` → `bun nx run @semio-tech/repo-dashboard-rs:run --outputStyle=stream` (Nx target `run`, `📦️packages/🦀️rust/📋️project.json`).
2. That target → `bun ./📜️script.ts run` (`📦️packages/🦀️rust/📜️script.ts`, `RunScript`).
3. `dashboardExecutable()` reads `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json` (`📦️installation/🟦️.ts:6-7`), which records the immutable executable. The executable on this host is `.🧬semio/🦑️repo/⚡️cache/tools/dashboard-cli/<hash>/semio.exe`. The first start, when no record exists, runs `cargo build --release --bin semio` and installs it.
4. The native `semio` with no arguments opens the TUI.

Other dashboard entry points:

- `bun run dashboard:install` → `repo-dashboard-rs:install` (dependsOn `build`).
- `dashboard:start|status|stop` → `repo-dashboard-rs:daemon -- start|status|stop`. `dashboard:preferences` → `repo-dashboard-rs:preferences`.
- Nx targets of the dashboard project: `build check test test-quick test-long test-exhaustive run daemon workflow install preferences`.
- The Run-panel "🛠️dev🎛️dashboard" row runs `repo-cli-rs:run`, which does not exist (§1.3).

The native verb table (`📦️packages/🦀️rust/🦀️.rs:52-68`) implements `dashboard preferences repo-view daemon workflow dev catalog command-tree commands plugin registry`. Anything else goes to `root_delegation::run`, which spawns `bun ./📜️script.ts <verb> …`. `run tasks logs stop restart kill open` from fleet-plan §2.4 are not implemented (the dispatch table in `🎛️dashboard/📦️packages/🦀️rust/🦀️.rs` has only the verbs above). They would fall through to the root script, which has no such routes.

## 3. Inventory And Judgement

Legend for "Starts processes outside dashboard": Y means the mechanism runs commands on its own when a user or client triggers it. "Bootstrap" means it runs only at environment setup.

### 3.1 Editor (VS Code)

| Surface | Size / content | Purpose and consumers | Origin | Duplication and drift | Starts processes outside dashboard | Verdict |
|---|---|---|---|---|---|---|
| `.vscode/launch.json` | 69,775 lines, 3.35 MB; 4,667 `node-terminal` rows, 82 `promptString` inputs, 4 compounds; groups `4_gate` 3,200, `3_dev` 635, `9_gates` 453, `4_build` 260, others | VS Code Run panel. Also read by tests and gates (§4.4) | Generated historically; generator removed; hand-edited | 1,195 names not in seed; 21 broken rows; 1,508 command paths and 1,665 rows in ticket folders | Y (Run panel shells) | DISSOLVE |
| `.vscode/🧩️launch.seed.jsonc` | 51,424 lines; 3,594 entries (3,505 objects, 89 `"@generated:<x>"` placeholders); 34 inputs, 4 compounds, `devLaunchers`, `projectLaunchers` (group/order/emoji/skip rules) | Input template for a generator that no longer exists | Hand-authored template | 33 names not in `launch.json`; 18 broken rows | N (not read by anything at runtime) | DISSOLVE. Move the verb order and skip rules into the registry if still needed |
| `.vscode/settings.json` | 163 lines, 78 keys | VS Code / Nx terminal env, Playwright, ESLint, Prettier, Python, cmake, LaTeX, Copilot commit instructions, ports | Hand-written | 28 keys mirrored in `devcontainer.json` (5 values differ). Policy says it must not exist (§1.9). Broken: LaTeX tool path, Copilot instructions path (`.agents/skills/committing.SKILL.md` does not exist; it is `commit/SKILL.md`), `python.testing.pytestArgs` (`compose/py`, `compose/engine` missing), `eslint.workingDirectories` (`elements/ui` missing), sqltools db under `example/` missing, `python.defaultInterpreterPath` is Windows-only | N | KEEP as the native-host canonical; DERIVE the devcontainer subset from it; fix the broken paths; decide the policy conflict (owner) |
| `.vscode/extensions.json` | 36 recommendations | VS Code | Hand-written | devcontainer list has 34: 3 extensions only here (`ms-vscode-remote.remote-containers`, `vscode-icons-team.vscode-icons`, `James-Yu.latex-workshop`), `anthropic.claude-code` only in devcontainer | N | KEEP as canonical; DERIVE devcontainer list from it |
| `.vscode/mcp.json` | 31 lines | VS Code MCP client; `repo` with profile `copilot`, `semio` gateway | Hand-written | Same `semio` entry as `.mcp.json` and `.cursor/mcp.json` (§4) | Y (MCP server started by editor) | DERIVE (§4) |
| `.vscode/tasks.json` | does not exist | `.gitignore:591` has an exception for it | Dead exception | none | none | DISSOLVE the exception |

### 3.2 Agent Clients

| Surface | Size / content | Purpose and consumers | Origin | Duplication and drift | Starts processes outside dashboard | Verdict |
|---|---|---|---|---|---|---|
| `.claude/launch.json` | 801 lines, 79 configs (50 bun, 5 bash, 24 attach-only with `port`/`url`, no command) | Claude Code preview (`preview_start` reads it by name) | Hand-written | 62 names not in `launch.json`; 1 broken row (`@semio-tech/framework-rs:test-snapshot-sqlite-io`) | Y | DISSOLVE. Preview needs a client change: start through `semio run <id> --wait-ready` and open the reported URL |
| `.claude/settings.json` | 84 lines (JSONC); 6 LSP plugins; `hooks: {}` with commented hook sample (`target/release/semio hook …`); allow list with stale entries (`net/Compose/…`, `py/main.py`, `js/compose/…`, all missing); deny list | Claude Code | Hand-written | Hook sample duplicates `.cursor/hooks.json` and `.github/hooks/repo.json`; `target/release/semio` is not built | Y when hooks enabled | KEEP (client-canonical); prune stale allow entries; DERIVE hooks from `📋️config.toml` (§5) |
| `.claude/settings.local.json` | 12 lines; gitignored (`*.local.*`) | Local Claude settings. `enabledMcpjsonServers` lists `neo4j-semio`, `neo4j-elements`, `neo4j-coda`, `neo4j-reuse`, `neo4j-metabolism`, `neo4j-extra` (none defined in any `.mcp.json`); `enableAllProjectMcpServers: true` | Local | Stale names | N | KEEP local, drop the stale names; a repo-level file must not carry them |
| `.cursor/mcp.json` | 30 lines | Cursor MCP; `repo` profile `cursor`, `semio` gateway | Hand-written | Same as §4 | Y | DERIVE (§4) |
| `.cursor/hooks.json` | 107 lines; `hooks: {}`; 2 active lines; commented commands `bun ./📜️script.ts dev mcp stdio cursor hook <event>` (non-functional) | Cursor hooks | Hand-written | Duplicates `.github/hooks/repo.json` | Y when enabled | DERIVE from `📋️config.toml` or delete the dead block |
| `.cursor/plans/` | 429 files | Plan documents. Excluded per brief | — | — | — | not audited |
| `.codex/config.toml` | 11 lines; `repo` profile `codex`; `semio` gateway with `--folder .` | Codex MCP | Hand-written | Diverges from the other three (`--folder .`) | Y | DERIVE (§4) |
| `.codex/plans/declarative-actions.md` | 58 lines | Plan document, not configuration | — | — | — | not configuration; KEEP or move to a ticket |
| `.agents/skills/{clean,commit,merging,micro-commit}/SKILL.md` | 4 skill documents | Agent instructions; `clean` runs `bun ./📜️script.ts clean` (including `clean stray-processes`, which kills processes); `commit` and `micro-commit` run `bun ./📜️script.ts commit|micro-commit …` | Hand-written | Referenced by a wrong path in `.vscode/settings.json` and `devcontainer.json` | Y (agent-run) | KEEP; fix the references |
| `skills-lock.json` | 10 lines; one skill (`migrate-oai-app` from `modelcontextprotocol/ext-apps`) | Skill installer lock; taxonomy knows it (`🔣️taxonomy.json:23028`). No installer reference found in repo | Generated by an installer (UNVERIFIED) | Skill not vendored under `.agents/skills` | N | KEEP if an installer uses it (UNVERIFIED) |

### 3.3 Container And CI

| Surface | Size / content | Purpose and consumers | Origin | Duplication and drift | Starts processes outside dashboard | Verdict |
|---|---|---|---|---|---|---|
| `.devcontainer/devcontainer.json` | 227 lines, JSONC; 34 extensions; 28 settings; 10 forwarded ports with labels; 18 mounts; lifecycle `bun nx run workspace:setup`, `bun ./📜️script.ts setup devcontainer start|attach` | Dev Containers | Hand-written | `forwardPorts` must equal launch rows (README; `runtime-bootstrap` test reads `launch.json`). `SEMIO_GITKRAKEN_WORKSPACE_NAME: "compose"` is a legacy name. Stale settings path (`.agents/skills/committing.SKILL.md`) | Bootstrap (postAttach installs GUI tools) | KEEP; DERIVE `forwardPorts`/`portsAttributes` from dashboard `ready.port` declarations; DERIVE settings and extensions from `.vscode/` |
| `.devcontainer/Dockerfile`, `docker-compose.yml`, `.dockerignore`, `README.md` | Image and compose stack; README documents lifecycle and ports | Container build | Hand-written | README says ports are "the launch rows" | Bootstrap | KEEP; fix README wording |
| `.github/workflows/architecture-quiz.yml` | 108 lines; `workflow_dispatch`; jobs run `bun nx run @teaching/architecture-quiz:{test,publish,docker-image-build,docker-image-publish}` (all resolve in the graph) | CI | Hand-written | none | N (CI) | KEEP (CI is outside the dashboard by design) |
| `.github/dependabot.yml` | 72 lines; 15 directory entries | Dependency updates | Hand-written | 14 of 15 directories do not exist (only the uv entry at `/` is valid) | N | DERIVE from current manifests (root `package.json`, `Cargo.toml`, `go.work` modules, `pyproject.toml`, `uv.lock`) |
| `.github/hooks/repo.json` | 62 lines; `hooks: {}` plus commented Copilot hook sample | Copilot agent hooks | Hand-written | Byte-identical to `compose-repo.json` (§5) | Y when enabled | DERIVE from `📋️config.toml` |
| `.github/hooks/compose-repo.json` | 62 lines, identical to `repo.json` | none (stale name from the former "compose" repo) | Stale copy | Duplicate | Y when enabled | DISSOLVE |

### 3.4 Command Roots And Build Configuration

| Surface | Size / content | Purpose and consumers | Origin | Duplication and drift | Starts processes outside dashboard | Verdict |
|---|---|---|---|---|---|---|
| `package.json` (root) | 277 lines; 134 scripts (§7); `workspaces`; `semio.workspace` block; Nx patch; devDependencies | bun, humans, README, dashboard (`script:` entries; `GRAPH_OWNERS` includes `package.json`) | Hand-written | 130 scripts are aliases or variants (§7) | Y (via bun/Nx) | KEEP the bootstrap set; DISSOLVE the rest into dashboard discovery. AGENTS.md's rule that package.json "MUST call nx" needs an owner decision (§10) |
| `nx.json` | 110 lines; plugins: `@nx/js`, `@nxlv/python`, repo plugin `🧰️…/📚️library/🟨️.mjs`, test plugin; `cacheDirectory .🧬semio/…/⚡️cache/nx`; `defaultBase "⛳wip"` | Nx | Hand-written | none | Nx daemon | KEEP |
| `📋️project.json` (root) | 3,215 lines, 90 KB; 295 targets, all `nx:run-commands` calling `bun …📜️script.ts …`; `metadata.semio.dashboard`: 7 parameters, 10 tools, 3 compounds, 11 groups | Nx; dashboard | Hand-written | Several `…-native`/`…-oracle` pairs; `verify-policy-breach-*` (12 near-identical). `gemini --yolo` and `kiro-cli --trust-all-tools` tools | Y (tools are declared processes) | KEEP as the canonical root declaration; owner decides the tools with approval-bypass flags |
| 📜️script.ts (root) | 26,242 lines, 1.88 MB, 20 classes, 22 top-level routes: `os semio examples setup start dev generate scale-fixture new schema lint verify format test bench stdio build cpp publish clean micro-commit commit`. `verify` alone has about 60 literal sub-verbs plus owned-route dispatch. `setup` has `postinstall git native deps prepare devcontainer` | AGENTS.md mandates it as the implementation of every permanent command; Nx targets call it | Hand-written | `dev` duplicates native `semio dev`; `catalog` and `plugin registry` duplicate native verbs; `setup devcontainer` duplicates lifecycle code | Y (each route) | KEEP as the implementation layer (AGENTS.md); no new routes; split the monolith later |
| `📦️packages/🦀️rust/📜️script.ts` (dashboard) | 85 lines; `build install preferences test run daemon workflow` | Nx targets of the dashboard | Hand-written | none | Y (`run` execs the native binary) | KEEP |
| `bunfig.toml` | 3 lines: `[install] linker = "hoisted"` | bun (Electron and Playwright native deps) | Hand-written | none | N | KEEP |
| `Cargo.toml` (workspace) | 544 lines; 120 workspace members; `trim-paths` | cargo, rust-analyzer (`linkedProjects`), Nx cargo plugin | Hand-written | none | N | KEEP |
| `go.work` | 32 lines; 28 `use` entries; `go 1.25` | Go toolchain; launch rows set `GOWORK`; repo CLI test runner | Written by `🏠️workspace` Go module (write call found; UNVERIFIED which runs) | none | N | KEEP; DERIVE from the workspace module if it already writes it |
| `pyproject.toml` | 62 lines; uv project `monorepo`, Python `>=3.14,<3.15`, dev and test groups, pytest `testpaths` `🧰️framework`, `✏️s`, `♻️mit-bestand` | uv, pytest, VS Code Python | Hand-written | VS Code `pytestArgs` (`compose/py`) is stale | N | KEEP |
| `CMakePresets.json` | 9 configure presets (`base ninja-base ninja-release-base linux macos linux-release macos-release windows cxx26`), 6 build presets | CMake, cmake-tools (`cmake.useCMakePresets`), root `cpp*` scripts | Hand-written | root `cpp:configure|build|test` scripts duplicate what the presets already describe | N | KEEP the presets; DISSOLVE the `cpp*` scripts into dashboard targets |
| `Monorepo.sln` | 68 lines; 2 real `.csproj` projects (`🧪️Semio.Repo.Test.csproj`, `🔷️.csproj`) and 6 solution folders | Visual Studio, `dotnet.defaultSolution` in VS Code and devcontainer, Nx `namedInputs` `dotnet` | Visual Studio artifact | No Nx target invokes it; the `🔷️dotnet` project does not reference it. Dependabot's nuget entries are stale | N | DISSOLVE if .NET is not in use (UNVERIFIED usage) |
| `.config/nextest.toml` | 21 lines; profiles `fundamental quick long exhaustive`; `live-database-lanes` group | cargo-nextest via `🧪️testing/🦀️cargo` | Hand-written | 4 copies: root, `✏️s/`, `🌎️hub/`, `🎓️teaching/`. Root equals teaching; `✏️s` and `🌎️hub` differ | N | KEEP root; DISSOLVE the copies or DERIVE them from root |
| `CMakeLists.txt` (root) | exists | CMake presets | Hand-written | none | N | KEEP |
| `rust-toolchain.toml`, `rustfmt.toml`, `.prettierrc.json`, `tsconfig.json` | found in the tree, not in the brief's list | toolchain and formatter config | Hand-written | none | N | KEEP |

### 3.5 Locks, Ledgers, Ratchets, Local State

| Surface | Size / content | Purpose and consumers | Origin | Duplication and drift | Verdict |
|---|---|---|---|---|---|
| `🔒️dependencies.json` | 4,477 lines, 198 KB; 256 entries; `generatedAt 2026-10-03`; `commit` recorded | Dependency freeze ledger, read by `verify dependencies` (root target `verify-dependencies-freeze`, `write-baseline` regenerates it) | Generated | none | KEEP (generated; derived by its script) |
| `🚚️migration.json` | 13 lines; `unmanagedTests` total 48 keyed by area | Described as a "shrink-only test-migration ratchet"; no code reads `unmanagedTests` (only the file itself and a test string) | Hand-written | orphan; conflicts with AGENTS.md "MUST NOT leave any migration" | DISSOLVE (or wire to `verify` if the owner wants the ratchet) |
| `.🧬semio/🦑️repo/📋️config.toml` | 9 lines; `[logging]`: `session=false`, `operations=true`, `plan=true`, `detail="standard"` | Hook logging switch. The disabled hook blocks say "disabled while logging is off" | Hand-written | This is the canonical switch the hook files should derive from | KEEP; make it the source for hook derivation |
| `.🧬semio/🦑️repo/compose-micro-commit-bun` | 25 bytes: an absolute macOS path `/Users/ueli/.bun/bin/bun` | none | Stale local artifact | DISSOLVE |
| `.repo/` (repo root, untracked; not shown by `git status`) | 913 files: `cache/` (neo4j cypher logs, java uninstall logs, emscripten, vcpkg), `sign-commit.log`, `🎫/26/07` remains | Legacy local state from a previous tool; only `.storybook/main.ts` ignores it | Local legacy | not a configuration surface | DISSOLVE (local delete; owner decision) |
| `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard/installed.json` | JSON record `{version, platform, path}` | Dashboard installation record | Generated | cache, not configuration | KEEP (dashboard-internal) |

## 4. MCP Definitions: Duplication Matrix

| File | Repo server profile | `semio` gateway | `--folder` | `--scopes` | Consumer |
|---|---|---|---|---|---|
| `.mcp.json` | `client` | `bun ./📜️script.ts dev mcp stdio os --folder .🧬semio/🔗space/os-mcp --scopes …` | `.🧬semio/🔗space/os-mcp` | 6 scopes | Claude Code |
| `.vscode/mcp.json` | `copilot` | same as `.mcp.json` | same | same | VS Code |
| `.cursor/mcp.json` | `cursor` | same | same | same | Cursor |
| `.codex/config.toml` | `codex` | same arguments | `.` | same | Codex |

Findings:

- The four `semio` entries are identical except `.codex/config.toml`, which uses `--folder .`. Only the repo profile name varies by client, and that difference is real (it selects the Go repo client profile through `SEMIO_REPO_MCP_CLIENT`).
- All four point at `.🧬semio/🔗space/os-mcp`, which does not exist here (`.🧬semio` contains `🌐os`, `🎓️teaching`, `🦑️repo`). Nothing in the repo creates it.
- The only automated check, `policyMcpConfigBreaches` (`📜️script.ts`, `POLICY_MCP_CONFIG_PATHS`, about line 17406), validates the `repo` entry only, not `semio`. It also lists `.windsurf/mcp.json` and `.kiro/settings/mcp.json`, which do not exist.
- Verdict: DERIVE all four from one declaration (`📋️project.json` `metadata.semio.dashboard.tools` already contains `repo-mcp` and `os-mcp-stdio`; add the client profile and folder there) through one `script.ts` subcommand that writes the client files. Remove the `.claude/settings.local.json` neo4j names.

## 5. Hooks

| Surface | State | Command shape | Works today |
|---|---|---|---|
| `.claude/settings.json` (commented) | disabled | `target/release/semio hook <Event> claude-code` | No: `target/` is not built; `semio` is the dashboard binary, which forwards `hook` to `script.ts` |
| `.cursor/hooks.json` (commented) | disabled | `bun ./📜️script.ts dev mcp stdio cursor hook <event>` | No: `runMcpStdioRepo` rejects more than one profile argument |
| `.github/hooks/repo.json` (commented) | disabled | `bun ./📜️script.ts dev mcp stdio copilot hook <Event>` | No, same reason |
| `.github/hooks/compose-repo.json` | disabled | identical to `repo.json` | No |
| `📋️config.toml [logging] session=false` | canonical switch | n/a | n/a |

Hook producers: `lifecycle/🟦️.ts:420-426` (`configureRepoHooks`) calls `target/release/semio configure --repo` (Rust path) or the Go `semio-repo configure` when `SEMIO_REPO_IMPLEMENTATION=go`. The Go `configure` removes git hooks and installs micro-commit hooks; it does not write agent hook files. Verdict: DERIVE all agent hook files from `📋️config.toml` plus the hooks module (`🪝️hooks`, Rust and Go), through one writer; until then remove the dead commented blocks and `compose-repo.json`.

## 6. Root `package.json` Scripts: Redundancy Under Dashboard-As-Sole-Control-Plane

Counts of the 134 scripts (`🗑️generated/root-scripts.tsv`):

| Class | Count | Examples | Under sole-control-plane |
|---|---|---|---|
| A. Pure Nx alias (`bun nx run <project>:<target>`, no arguments) | 81 | `build:print`, `build:mit-bestand:demonstrator`, `test:quiz:rs`, `publish:teaching:architecture-quiz`, `dev:storybook*` (about 13) | Redundant: the dashboard discovers the Nx target directly |
| B. Nx target with arguments or variant (`-- <args>`, `--outputStyle`, dashboard `-- start`) | 44 | `dev:puzzle:2d|3d|5d`, `dev:puzzle:3d:concrete-forest`, `dev:fem:*`, `dev:trinity:*`, `build:cad:concrete-forest`, `dashboard:start|status|stop` | Redundant: replaced by playground variants and parameters (`renderer`, `example`, `user-slot`) and by `semio daemon start|status|stop` |
| C. Other | 9 | `nx` (bootstrap bridge), `dashboard`, `dashboard:install`, `dashboard:preferences`, `cpp`, `cpp:configure`, `cpp:build`, `cpp:test`, `wasm:flow-core` | `nx`, `dashboard`, `dashboard:install` are bootstrap and KEEP. `dashboard:preferences` becomes `semio preferences`. `cpp*` become CMake-preset targets. `wasm:flow-core` calls bare `nx` instead of `bun nx` |

Concrete suspects:

- `dev:puzzle:5d:capsule-dream` has the same body as `dev:puzzle:5d` (`bun nx run workspace:dev -- 5d`), with no `capsule-dream` argument. Either the mapping is wrong or it is a leftover (intent UNVERIFIED).
- `wasm:flow-core` uses `nx run …` without `bun`, unlike the other 133 scripts.
- `publish:teaching:architecture-quiz` and the CI workflow both call Nx targets; CI does not call any root script, so CI is not affected by removing aliases.

Keep set if the dashboard is sole control plane: `nx`, `setup`, `dashboard`, `dashboard:install`. That is 4 scripts. The other 130 are redundant as entry points. The fleet plan §2.1 still lists root scripts as a command source; the owner should decide whether the source stays (for scripts with no Nx equivalent) or goes (§10).

Documentation still citing these scripts: README lines 655–663 and 860–866 (`bun run lint|format|test|dev|dashboard|generate`, and `purge` which does not exist).

## 7. Declarations Inside The Dashboard Model

Root `📋️project.json` `metadata.semio.dashboard` (verified):

- Parameters (7): `cache`, `test-level`, `dependencies`, `build-mode`, `nextest-output`, `cargo-jobs`, `build-budget`.
- Tools (10): `repo-mcp`, `os-mcp-stdio`, `os-mcp-http`, `mcp-inspector-os` (ready port 6274), `bun-test`, `native-cargo`, `gemini --yolo`, `kiro-cli chat --trust-all-tools`, `f3d`, `gitkraken`.
- Compounds (3): `s-with-hub`, `s-with-os-mcp`, `s-users-with-hub`.
- Groups (11): `process-extension-catalogs`, `sourcing-extension-catalogs`, `extension-catalogs`, `process-extension-tests`, `sourcing-extension-tests`, `stdio-artifact-tests`, `puzzle-spatial-tests`, `snapshot-sqlite-parent-baselines`, `snapshot-sqlite-native`, `snapshot-sqlite-source`, `dag-actor-wasm`.

Target-level declarations: 73 targets in 19 manifests (16 in root; `🧑‍💻dev/📦️packages/🟦️typescript` 12; `🌎️hub` 18 across two manifests). Fields used (counted per target): `ready` 32, `parameters` 42, `requires` 15. This covers about 2% of the 4,667 launch rows.

Verification of run references (`r2-validate-run-refs.ts`): 6,134 references scanned across launch files, root scripts, root project tools, compounds and groups, CI workflow, devcontainer, editor settings and MCP files. Result: 6,085 resolve; 40 point at missing targets (all in launch files, see §1.3, §8); 9 are glob patterns, not checked. Every project name referenced resolves in the graph. No compound member names a missing project.

## 8. Broken Or Stale References (Verified)

Launch files (graph check):

- `.vscode/launch.json`: 21 rows, 17 distinct missing targets. Examples: `repo-cli-rs:run|daemon|workflow|install|preferences`, `plugin-registry:check-launch-seed|reconcile-launch-seed|test-launch-seed-reconcile|test-launch-name-contract|test-launch-placement-contract`, `framework-rs:test-artifact-kind|test-fixture-ownership|test-snapshot-sqlite-io`, `repo-lib:test-process-tree-termination|test-draw-destination-observation|lint-framework-module-product-direction`, `framework-io-schema-rs:test-binding`, `plugin-registry` launch gates.
- `.vscode/🧩️launch.seed.jsonc`: 18 rows, same set.
- `.claude/launch.json`: 1 row (`framework-rs:test-snapshot-sqlite-io`).
- `repo-cli-rs:test -- execution` resolves, but it runs the repo CLI's `test` target, not the dashboard's `test execution` (`📦️packages/🦀️rust/📜️script.ts`). The row tests the wrong project.

Other references:

- `.vscode/settings.json`: LaTeX Workshop tool `bun ${workspaceFolder}/mit-bestand/bericht/📜️script.ts latex` (path missing); `latex-workshop.latex.search.rootFiles.include` `mit-bestand/bericht/**`, `print/template/**` (missing); `python.testing.pytestArgs` `compose/py`, `compose/engine` (missing); `eslint.workingDirectories` `elements/ui` (missing); sqltools database `example/metabolism/.compose/kit.db` (missing); `github.copilot.chat.commitMessageGeneration.instructions` `.agents/skills/committing.SKILL.md` (missing; the file is `.agents/skills/commit/SKILL.md`).
- `.devcontainer/devcontainer.json`: same Copilot instructions path; sqltools path; `"SEMIO_GITKRAKEN_WORKSPACE_NAME": "compose"` (legacy name).
- `.claude/settings.json`: allow entries `Read/Edit(net/Compose/Compose.cs)`, `net/Compose.Grasshopper/…`, `py/main.py`, `js/compose/compose.ts`, `js/compose/components/**`, `js/js/…` (all missing).
- `.github/dependabot.yml`: `/js/compose`, `/js/desktop`, `/js/docs`, `/py/compose`, `/py/engine`, `/net/Compose`, `/net/Compose.Grasshopper`, `/net/Compose.Tests`, `/net/Compose.Grasshopper.Tests`, `/go/cli`, `/go/mcp`, `/repo/client`, `/go/compose`, `/rs/compose` (missing); `/` (uv) is the only valid entry.
- `.mcp.json`, `.vscode/mcp.json`, `.cursor/mcp.json`, `.codex/config.toml`: `--folder .🧬semio/🔗space/os-mcp` (missing) in three; `.` in codex.
- `Monorepo.sln`: paths are valid for the two projects.
- `README.md` lines 660–661, 863: launch-row claims; `purge` root command (no such script).
- `🎛️dashboard/README.md` lines 19–24: launcher reads `.vscode/launch.json` (not implemented); line 128: `🧫️fixtures/🚀️launch-configurations` (a fixture, not a source).

## 9. Launch-File Consumers To Remove With The Files

Reading them in code (a gate or test fails if the file goes without a change):

- `📜️script.ts` `verify interactivity` launch gate (`INTERACTIVITY_ALL_APP_LAUNCH_FILE`, `INTERACTIVITY_ALL_APP_LAUNCH_SEED_FILE`, about lines 8717–8730).
- `📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts:55-59` (devcontainer forwarded ports must equal launch rows).
- `📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts:802`.
- `🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts:227-233` and `🧪️test/📋️project.json:427-428` (inputs).
- `📚️library/📦️packages/🟦️typescript/📋️project.json:55-56, 570-571, 599-600` (inputs).
- `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json:154-155` (already records the deletion).
- Ignore lists only: `♻️mit-bestand/…/🏗️builder/🌐️vite/🟦️.ts:78`, `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts:69`, `🧰️…/💻️os/…/🧫️fixtures/👁️watch-policy.json:11`.
- Documentation: `🎓️teaching/…/❓️quiz/README.md:44`, `🎓️teaching/…/🐾️pets/README.md:358-359`, `🧰️framework/🛍️products/🎤️presentation/README.md:22`, `📓️print/README.md:22`.

## 10. Conflicts With The Goal

AGENTS.md (not edited):

- Lines 50–51: "All devs are using `launch.json` and never use the cli." and "You MUST register all executable commands there by following the existing order, grouping and naming." This is a direct conflict with the owner's goal. Owner must replace it with: the dashboard and `semio` are the only developer entry point; commands are declared in `📋️project.json` `metadata.semio.dashboard` and Nx targets.
- Lines 8: "`package.json` MUST call `nx` to run `📜️scripts.ts …`". The file name is `📜️script.ts` (typo). The rule also requires the root `package.json` to remain a bridge to Nx. This is a tension with removing the 130 aliases; the owner should say whether the bridge is the bootstrap set only.
- Line 6–10 otherwise matches the tree: `project.json` calls only `📜️script.ts` (verified for all 295 root targets).

Other files that conflict with the goal: `README.md` 660–661 and 863 (launch-row golden paths; `purge`); `🎛️dashboard/README.md` 19–24 and 128 (launch file read by launcher; not implemented); `.devcontainer/README.md` (ports are "the launch rows").

## 11. Owner Decisions Needed

1. `.vscode/settings.json` versus the repo policy that forbids it (§1.9). Recommended: keep it for native hosts; derive the devcontainer subset; change the Go policy.
2. Whether Claude Code preview is still needed. If yes, it needs a client change: start through `semio run … --wait-ready` and open the URL.
3. Whether root `package.json` remains a command source (fleet plan §2.1) or only the bootstrap set (§6).
4. Approval-bypass tools in the dashboard declarations: `gemini --yolo`, `kiro-cli chat --trust-all-tools` (`📋️project.json`).
5. Whether `Monorepo.sln` and the nuget dependabot entries stay (no current .NET build target found).
6. Whether `🚚️migration.json` (unmanagedTests ratchet) is wanted; no reader exists.
7. Whether `.🧬semio/🔗space/os-mcp` should be created, or the `--folder` argument changed in all four MCP files.
8. Whether `dev:puzzle:5d:capsule-dream` should run `capsule-dream` (§6).

## 12. Recommended Dissolution Order (Not Executed)

1. Fix the 40 broken refs and the stale paths in §8 (none needs a dashboard change; `.vscode`, `.claude`, `.github`, `.devcontainer`, `.agents` references).
2. Delete `.github/hooks/compose-repo.json`, `compose-micro-commit-bun`, `🚚️migration.json` (after decision 6), the `.repo/` local state (after decision), and the dead commented hook blocks.
3. Derive MCP client files from one declaration (§4). Add a `script.ts` subcommand that writes all four and extend `policyMcpConfigBreaches` to check the `semio` entry.
4. Derive devcontainer ports (`forwardPorts`, `portsAttributes`) and the extension and settings subsets from the dashboard `ready.port` declarations and `.vscode/`. Update `runtime-bootstrap` to read the declarations instead of `launch.json`.
5. Remove `.claude/launch.json`, then `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, with the gates and tests in §9 and the `.gitignore` exception at line 591. Replace the `verify interactivity` launch gate with a declaration-based gate.
6. Reduce root `package.json` to the bootstrap set once the dashboard covers the aliases (§6), and update README lines 655–663 and 860–866.
7. Derive `.github/dependabot.yml` from the real manifests; keep `.github/workflows/architecture-quiz.yml`.

## 13. Not Run Or Unverified

- No Nx task, `semio` binary, dashboard install or daemon was run. "Broken" means "the Nx graph has no such target" or "the path does not exist", not an observed failure.
- The `semio` MCP server CONNECT_TIMEOUT in this session is not attributed; the missing `--folder` is a candidate cause only.
- Whether `semio configure` or the Go `semio-repo configure` ever writes agent hook files: the code reads as git-hook only; not run.
- The native `semio` fallback to `bun ./📜️script.ts`: confirmed in `root_delegation::run` and the USAGE text; not executed.
- `skills-lock.json` consumer (no installer reference found in repo).
- `Monorepo.sln` usage outside Visual Studio (no Nx target found; not run).
- `🔒️dependencies.json` regeneration: `verify-dependencies-freeze-write-baseline` exists; not run.
- Count of root `script.ts` verify sub-verbs is approximate (about 60 literal names plus owned routes).
- Playground catalog `🤖️generated/🚀️playgrounds.json` exists (158 KB, dated 2026-10-06) and is read by the registry. Its freshness was not checked.

## 14. Files Written

- This report: `r2-audit-config-surfaces.md` (in this ticket folder).
- Helper, read-only: `r2-validate-run-refs.ts` (this folder). Run: `bun r2-validate-run-refs.ts`.
- Generated outputs (`🗑️generated/`): `run-resolution.json`, `run-resolution-summary.txt`, `root-scripts.tsv`, `launch-json-commands.txt`, `launch-names.txt`, `seed-names.txt`, `project-manifests.txt`, `project-names.txt`. Delete them after the coordinator has read this report.
