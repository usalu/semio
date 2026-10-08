# R2 Audit: Command Registry And Command Line (fleet-plan §2, §2.4)

Read-only audit of the current implementation against `fleet-plan.md` §2.1–§2.4 and §2.2.1.
Date 2026-10-08. Repository HEAD `2604f70cac1`. At audit start the working tree of the audited paths matched HEAD
(`git status` showed only ticket files); a peer later edited `🚇️pty/🦀️.rs` (uncommitted, §0). No git-modifying command,
no daemon start or stop, no install.

Abbreviations: `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard` (repo-relative). Paths that start with
`📚️library/`, `🧪️test/` or `🧰️…` are shortened from `🧰️framework/🛍️products/🦑️repo/🔨️modules/` or, for `📇️registry/`,
from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`; `♻️`, `🏢️`, `🌎️`, `🎓️`, `✏️s/` and `.vscode/` paths are repo-root relative.
Line numbers are `file:line` in the named file. Statuses: IMPLEMENTED, PARTIAL, MISSING. "Static" means
read from source and not run.

Generated output of this audit: `🗑️generated/r2-audit-cargo-check.txt`, `🗑️generated/r2-audit-cargo-check-retry.txt`,
`🗑️generated/r2-dashboard-project-files.txt`, `🗑️generated/r2-scan-dashboard-decls.mjs` (helper, reproducible with the git grep below).

---

## 0. Verdict And Blockers

**Build state.** Two runs against the state at audit start failed before the dashboard crate was checked.
The failing crate was `semio-framework-ui` (lib):

    🧰️framework/🔨️modules/🖱️ui/⌨️tui/🚇️pty/🦀️.rs:728:107: error[E0277]: `*mut c_void` cannot be sent between threads safely

Line 728 is in the `#[cfg(windows)]` block (`🚇️pty/🦀️.rs:474` to ~930), in the ConPTY output thread. The file was
clean against HEAD (`2604f70cac1`) at that time, so the break was committed. Between the second run and the third, a peer
made an uncommitted edit to the same file (`Shared::raw()`, `git diff --stat`: 10 insertions, 7 deletions). With that edit in
the working tree, `cargo check -p semio-framework-repo-dashboard` **passes** (exit 0, the dashboard crate is checked with no
warnings of its own). So: HEAD does not build the `semio` binary on Windows; the current working tree does. The peer's edit is
uncommitted and may change again. The package name and its `[lib]`/`[[bin]]` wiring are correct
(`📦️packages/🦀️rust/Cargo.toml:3,16,18-20`).

**Overall.** Registry declarations (§2.2 core) and the sources (§2.1) are largely implemented in `🎮️registry` and
`📚️inventory`, and the live launcher is driven by the registry. The dashboard binary no longer reads any launch file.
What is missing: the §2.4 task verbs (`run tasks logs stop restart kill open`), the amendments 1, 3, 5, 6, and 8,
and the runtime wiring of `requires`, `ready`, compound ordering, task labels and tab text into the view and the daemon.
The launch files, and gates that read them, are still in the repository. The README and the usage text are stale in part.

---

## 1. Direct Answers

### Q1. Does code still read `.vscode/launch.json` or `launch.seed`?

**Dashboard binary (non-test Rust under `D`): NO.** `git grep` for `launch.json|launch.seed|LAUNCH_|.vscode`
in `D/**/*.rs` finds only test code (below). `CommandLeaf` has no `Launch` variant:
`D/🌳️command-tree/🦀️.rs:25-29` defines `Process`, `Compound`, `Repo`. `CommandLeaf::Compound` is produced from
registry compounds (`command-tree:70-71`), not from launch configurations.

**Repository code that still READS the launch files (STATIC; all paths repo-relative):**

| # | Reader | Lines | Kind | Runs in |
| --- | --- | --- | --- | --- |
| 1 | `📜️script.ts` `interactivityAllAppDiscovery` reads `.vscode/launch.json` (`INTERACTIVITY_ALL_APP_LAUNCH_FILE`, const at `:8717`) and `.vscode/🧩️launch.seed.jsonc` (`:8718`) | `:9009`, `:9033`, `:9035` | LIVE gate | `verify interactivity` and `verify interactivity apps` (`runInteractivityAudit` `:8079`, `runInteractivityApps` `:8122`) |
| 2 | Same gate requires seven launch rows to exist exactly once (`INTERACTIVITY_ALL_APP_REQUIRED_GATES`) | `:8720-8730`, check `:8976` | LIVE gate | same |
| 3 | `📚️library/⚡️caching/📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts` reads `.vscode/launch.json` | `:55-59` | test | library test |
| 4 | `📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts` reads both files | `:802` | test | library test |
| 5 | `🧪️test/🧪️tests/🧱️command-composition-source/🟦️.ts` reads both files | `:227-233` | test | test harness |
| 6 | `D/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs` writes the `LAUNCH_CONFIGURATIONS` file and reads fixture `🚀️launch-configurations` | `:199-236`, `:203` | stale unit test, does not compile (see §6) | `cargo test` |
| 7 | `D/🌀️daemon/🧪️tests/🔬️unit/🦀️.rs` writes a `.vscode/launch.json` into a temp workspace and asserts a launcher label `publish / launch /` that the registry never produces | `:38-44` | `#[ignore]` stale test | native test only |
| 8 | `D/🧪️tests/🌀️control-plane/🟦️.ts` reads fixture `🚀️launch-configurations` and checks it with `jsonc-parser` only (no dashboard code runs) | `:56-80` | oracle over a fixture, stale | `bun test` in dashboard `test` target (`D/📦️packages/🦀️rust/📜️script.ts:38`) |

**Not readers (references only):** Nx cache-input lists `📚️library/📦️packages/🟦️typescript/📋️project.json:55-56,570-571,599-600`
and `🧪️test/📋️project.json:427-428`; watch ignore globs `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts:78`
and `🏢️semio-tech/🎡️play/🏗️builder/🌐️vite/🟦️.ts:69`; watch policy fixture `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json:11`;
print oracle `🧰️framework/🛍️products/📓️print/🔮️oracles/🔣️.json:79` names `🚀️launch.test.ts`, which no longer exists; `.gitignore:591`.

**Files still tracked:** `.vscode/launch.json` (69,775 lines), `.vscode/🧩️launch.seed.jsonc` (51,424 lines),
`.claude/launch.json` (801 lines, 79 configurations, parses as JSON). No repository code reads `.claude/launch.json`.
The Browser `preview_start` tool description in this session says it starts servers from `.claude/launch.json`; that is a
harness reader outside the repository.

**Generator status.** `📇️registry/🚀️launch/` is absent from the working tree and from `git ls-files`. The plugin registry
project and script no longer mention `launch` (`git grep -i launch` over its `📋️project.json` and `📜️script.ts` is empty).
Yet `.vscode/launch.json` still carries rows that call the removed targets, for example
`bun nx run @semio-tech/plugin-registry:check-launch-seed` (`.vscode/launch.json:63283`), and `…plugin-registry:test -- 🧪️tests/🚀️launch/🟦️.ts` (`:11628`).
These rows are dangling.

### Q2. Is `🎮️registry` wired into the binary and the TUI launcher, or is command-tree still the source?

Trace (all live, STATIC):

1. `🚪️entrypoint/🦀️.rs:4` calls `semio_framework_repo_dashboard::run`.
2. `📦️packages/🦀️rust/🦀️.rs:54-70` dispatches on the verb. No verb or a flag first gives `dashboard`.
3. `dashboard` calls `terminal::run_with` (`:58`), which is `D/🖥️terminal/🦀️.rs:601`.
4. `D/🖥️terminal/🦀️.rs:625` and `:179` call `inventory::start`. The job publishes
   `command_tree::launcher(&registry)` (`D/📚️inventory/🦀️.rs:423`). The `registry` is `Snapshot::registry()` (`inventory:124-128`),
   which calls `Registry::build` (`D/🎮️registry/🦀️.rs:887`).
5. The view consumes `Update::Ready` (`D/🖥️terminal/🦀️.rs:161`).
6. `semio commands` goes `lib.rs:66` → `registry::run` (`registry:1433`) → `inventory::registry_at` (`registry:1436`, `inventory:372`).

**Answer:** the registry IS the live source. `command-tree` is a projection of it: `command_tree::discover` is
`tree(&inventory::discover(…))` (`D/🌳️command-tree/🦀️.rs:46`), and `leaves` calls `registry.resolve_with` (`command-tree:70`).
Caveats:

- Circular module dependency: the registry imports `command_tree::RepoAction`, `action_key`, `ANALYZE_SCOPES`,
  `repo_implementation`, `go_binary_path` (`registry:11`, `:952`, `:965`, `:1342-1344`), and command-tree imports the registry (`command-tree:8`).
- The view still uses command-tree wire types (`CommandSpec`, `CommandLeaf`) and keeps only `cmd/args/cwd/env` of each
  resolved process (`command-tree:61-72`). `ready`, `requires`, `group`, `label` and `command_id` are dropped (§3.2).
- `semio dev` bypasses the registry. `D/🛝️playground-session/🦀️.rs:14,19-22` resolves through `load_playground_catalog`
  and repeats the playground branch of `registry:1322-1339`.

### Q3. Are ticket `🎮️commands.json` files read? Are `metadata.semio.dashboard` declarations read from `📋️project.json`?

- **Ticket documents: YES.** `D/📚️inventory/🦀️.rs:233-249` reads the ticket index and, for open tickets only (`:242`),
  parses `🎮️commands.json` (`:243`). `D/🎮️registry/🦀️.rs:948-963` decodes `TicketDeclaration` (`$schema` allowed, `:370`).
  On disk: 8 documents, all in tickets whose `🎫️ticket.json` status is `open`:
  `FIXTURES-ARE-TESTING-EXAMPLES-ONLY`, `PROCEDURAL-FEATURE-COMPLETE`, `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`,
  `WGPU-RENDERER-REACT-PARITY`, `BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT`, `COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE`,
  `NON-DESTRUCTIVE-HISTORY-EDITING`, `UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O`.
- **Declarations: YES.** `D/📚️inventory/🦀️.rs:151` (`dashboard_of` reads `/metadata/semio/dashboard`), `:162-166`
  (project and target level from the manifest), `:103` (manifest wins over graph), `:208` and `:207` (graph metadata fallback).
  The registry decodes them at `registry:899` (target) and `:911` (project).
- **Count (verified with `git grep` and a JSON parse):** `git -c core.quotepath=false grep -l '"dashboard"' -- '*📋️project.json'`
  returns **19 tracked files**. A `node` parse confirms the JSON path in each:
  - 19 files declare `targets.<name>.metadata.semio.dashboard` (**73 target declarations** in total);
  - **2 files** also declare project-level `metadata.semio.dashboard`: the root `📋️project.json` (keys `parameters`,
    `tools`, `compounds`, `groups`) and `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📋️project.json` (`compounds`).
  - The grep produced no false positives.

### Q4. Does the README describe current or stale behaviour?

**STALE** (see §7 for the list). The README describes a launcher fed from `.vscode/launch.json`, VS Code
`${workspaceFolder}`/`${input:id}` substitution, and a verb-based definition of "finite" that the code no longer uses.

### Q5. The requested `cargo check`

Command as requested: `cargo check -p semio-framework-repo-dashboard --message-format=short`, target
`target-fleet-audit-a1`. Final run: **exit 0**, `Checking semio-framework-repo-dashboard`, no warnings or errors from
the dashboard crate. Output: `🗑️generated/r2-audit-cargo-check-final.txt`. Earlier runs (exit 101) failed in
`semio-framework-ui` at `🚇️pty/🦀️.rs:728:107` against the committed state (§0). The crate's test target does not compile
(`cargo check --tests`, exit 101, 12 errors, all in the command-tree unit tests; §6, output
`🗑️generated/r2-audit-cargo-check-tests.txt`).

---

## 2. §2.1 Sources

| Source (§2.1) | Status | Evidence |
| --- | --- | --- |
| Nx targets, `continuous`, `configurations` from each `📋️project.json` | IMPLEMENTED | `D/📚️inventory/🦀️.rs:153-167` (targets, `continuous` `:164`, `configurations` `:164`), manifest walk `:253-278`, graph `:192-211` |
| Root `package.json` scripts that call Nx | IMPLEMENTED | `inventory:214-221` (prefix `bun nx ` or `nx `; excludes `dashboard` and `dashboard:*`, which are the entry scripts) |
| Playground variants from the generated catalog | IMPLEMENTED (consumer) | `inventory:223-231`, `D/📦️packages/🦀️rust/🦀️.rs:201-215`, `registry:873`. Generator scans `[[package.metadata.semio.playground]]` (`📇️registry/🎮️playground/🔎️discovery/🟦️.ts:181` under the plugin folder; 61 `Cargo.toml` files carry the key) |
| `metadata.semio.dashboard` in `📋️project.json` | IMPLEMENTED | §1 Q3 |
| Ticket commands, open tickets only | IMPLEMENTED | `inventory:233-249`, `registry:948-963` |
| Repo-domain actions, in process | IMPLEMENTED | `registry:948-968`, `command-tree:302-325` (ticket index), `command-tree:148-161` (`RepoAction`) |

---

## 3. §2.2 Declarations And §2.2.1 Amendments

### 3.1 Fields

| Field | Status | Evidence / gap |
| --- | --- | --- |
| Target `verb` (default leading word, else `task`) | IMPLEMENTED | `registry:346`, `:901`, `:905`; `verb_of` `:156-161`; schema `🧬️schema/🎮️registry/🔣️.json:193` |
| Target `ready {port, portEnv, path}` | IMPLEMENTED (declaration) | `registry:342`, validation `:786-794`, resolution `:1285-1297`. Runtime detection MISSING, see §3.2 |
| Target `requires` (reference strings) | PARTIAL | stored and validated (`:902`, `:1066-1067`) and resolved by `launch` (`:1226-1228`); never started or awaited on the view path (§3.2) |
| Target `parameters` | IMPLEMENTED | `registry:904`, `:1131-1138`; schema `:196` |
| Project `parameters` (root project only) | IMPLEMENTED | `registry:913-915`. Root `📋️project.json` declares cache, test-level, dependencies, build-mode, nextest-output, cargo-jobs, build-budget |
| Project `tools` | IMPLEMENTED | `registry:983-1001`; root declares 10 |
| Project `compounds` | IMPLEMENTED | `registry:1003-1026`; root 3, quiz 1 |
| Project `groups` | PARTIAL | `registry:362` `target: String` (amendment 3 not met), validation `:922`, glob `:1054-1061`. All 11 root groups use `target` |
| Parameter `choice / flag / text` with effects, `appliesTo`, `default`, `required` | IMPLEMENTED | `registry:795-848`, `Parameter` `:473-486`; schema `:81-123`. Tool scope gap: amendment 4 |
| Built-in playground parameters `renderer, example, user-slot, app-role` | PARTIAL | `registry:1143-1147` (`app-role` present). Extras not in §2.2: `language`, `terminology`, `appearance` (`:1148-1150`), listed in `PLAYGROUND_PARAMETERS` (`:875`) |
| Free extra arguments at launch | PARTIAL | `resolve(id, chosen, extra)` (`:1191`), `extra` appended after `--` (`:1302`), CLI `--extra` (`:1439`). No `run` verb (§5). TUI typed extras UNVERIFIED |
| Runner variables never stored (`NX_DAEMON`, `FORCE_COLOR`, …) | IMPLEMENTED | `registry:207`, `:768`, `:837`, check `:1365`; schema `EnvName` `:20-25` |
| Literal absolute paths rejected | IMPLEMENTED | `registry:210-213`, `:764`; schema `Text` `:26-31` |
| `ready.printed` | MISSING | `registry:342` (`ReadyDeclaration` has `port`, `portEnv`, `path` only; `deny_unknown_fields`); schema `:124-134`; 0 uses in the 73 declarations |

### 3.2 Runtime Consumers (what happens after resolution)

These are not §2.2 declaration fields, but they decide whether declared behaviour happens.

- `requires`: `Launch.requires` is produced (`registry:1228`), but `command_tree::leaves` keeps only `launch.processes` (`command-tree:61-75`). `requires` is not read by `command_tree`, `terminal` or `inventory` (grep empty). MISSING at runtime.
- `ready` (detection and URL): the live daemon has no ready detection. `D/🌀️daemon/🦀️.rs` has no `ready` tracker, and `ready_url` is only a struct field (`:155`), never assigned. The tracker (`READY_HOSTS` `D/🌀️daemon/📼️replay/🦀️.rs:29`, `Tracker::new(None)` `:506`) lives in an orphan file (§7). MISSING.
- Compounds: the view spawns each member independently (`D/🖥️terminal/🦀️.rs:429`), without waiting for `ready` and without `SpawnGroup` (`D/🌀️daemon/🦀️.rs` defines `SpawnGroup`, unused by the view). `stop: together` is not applied. PARTIAL (members start; ordering and stop-together MISSING).
- Task label on the session: the view builds `SessionCommand { cmd, args, cwd, env, cols, rows, ..Default::default() }` (`D/🖥️terminal/🦀️.rs:236`). `command_id`, `label`, `group` and `ready` are never set. MISSING.

### 3.3 Amendments (§2.2.1)

| # | Amendment | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `requires` entries and compound members share `{ "run", "parameters", "env" }` | MISSING | `MemberDeclaration` has `run` and `parameters` only (`registry:354`), `requires: Vec<String>` (`:346`, `:350`). Schema `CompoundMember` `:152-165` has no `env`; `Reference` `:53-58` is string-only. 9 member entries in the repo, 0 with `env` |
| 2 | No fixed `env` in a target declaration | IMPLEMENTED | `TargetDeclaration` (`registry:346`) has no `env`, `deny_unknown_fields`; schema `:189-199` |
| 3 | Groups name `targets: [..]`, no `target` key | MISSING | `registry:362` `target: String`; schema `:178-188` `target`; `D/🧬️schema/🎮️registry/🔣️.json:183`; root project 11 groups use `target` |
| 4 | `appliesTo.verbs` selects every command kind, tools included; `nxFlags` ignored for non-Nx commands | PARTIAL / MISSING | Tools get only their own parameters: `registry:1140` (`Action::Tool => entry.own.clone()`), so axes never reach tools. `nxFlags` on tool parameters is a declaration error (`registry:995`) instead of ignored. Schema `AppliesTo` `:61` says "every command Nx runs" |
| 5 | Free extra environment: `semio run … --env KEY=value` | MISSING | No `run` verb (§5). `resolve` takes extra args only (`registry:1191`, `:1302`); `registry::run` has no `--env` (`:1433-1460`) |
| 6 | `ready.printed: true` takes the whole printed URL | MISSING | see §3.1 |
| 7 | Confirmed readings: `{workspace}` and parameter tokens in defaults; `portEnv` without `port` reads the resolved env; group `projects` are Nx patterns | IMPLEMENTED | default token substitution `registry:1252`; `portEnv` fallback `:1286-1293`, validation `:791`; patterns `glob` `:198-203`, `:1054-1061` |
| 8 | Playground facts (data dir per slot, hub join, local-only, example lock, viewer suffix) move into plugin registry metadata with a reader | NOT STARTED | Plan places it after L-S2 and A-1. No reader: the playground row parsed in `inventory:228` has `variant, pluginId, app, ports, userPorts, examples` only; grep for `localOnly|dataDirectory|hubJoin|viewerRole|exampleLock` in the plugin registry is empty |

---

## 4. §2.3 Identity, Labels, Verbs

| Item | Status | Evidence |
| --- | --- | --- |
| Command id grammar (Nx, `playground:`, `tool:`, `compound:`, `group:`, `script:`, `ticket:<YY/MM/DD/SLUG>/<id>`, `repo:<key>`) | IMPLEMENTED | `registry:58-117` (`CommandId::parse` `:72-100`, `Display` `:103-117`), schema `CommandId` `:243-256` |
| `key=value` parameters on an invocation | IMPLEMENTED | `parse_invocation` `registry:120-126`, `format_invocation` `:129-132` |
| Verb: declared, else leading word of a known verb, else `task` | IMPLEMENTED | `registry:42-46`, `verb_of` `:156-161`, `verb_rank` `:164-166`; compound verb `:1023` and `:1050` |
| Long-running = Nx `continuous`, tool `continuous`, playground dev, or declared `ready` | IMPLEMENTED | `registry:909`, `:946`, `:999`, `:1051`, `:1062` |
| `TaskLabel { verb, owner, subject, qualifier, parameters, members }` computed at launch | IMPLEMENTED (registry side) | type `registry:19-28`; computed `:1230-1233`, `:1304`, `:1350` |
| `TaskLabel` travels with the session (daemon schema) | MISSING on the live path | view never sets `label` (`D/🖥️terminal/🦀️.rs:236`). Two further `TaskLabel` types exist: `D/🌀️daemon/🦀️.rs:79-88` (live, inline `ipc`) and orphan `🌀️daemon/✉️ipc/🦀️.rs:76` |
| Tab text `verb subject [qualifier]`, 24 cells, middle elision, ` ·2` duplicates | IMPLEMENTED as pure functions, NOT WIRED | `tab_text` `registry:268-281`, `tab_texts` `:285-291`, `TAB_CELLS` `:218`. No caller outside tests. The view titles output windows with `"{cmd} {args}"` (`D/🖥️terminal/🦀️.rs:251`) and status `[status detail] cmd args` (`:335`) |
| Window title as a pure function of the label | IMPLEMENTED as function, NOT WIRED | `window_title` `registry:294-301`; no caller |
| Status glyph and colour role | MISSING | not found in `D/🖥️terminal` |

---

## 5. §2.4 Non-Interactive Surface

Dispatch is `D/📦️packages/🦀️rust/🦀️.rs:57-69`. Unmatched verbs fall through to `root_delegation` (`:68`), which runs
`bun ./📜️script.ts <verb> …` (`D/📜️root-delegation/🦀️.rs:9-16`).

| §2.4 verb | Status | Evidence |
| --- | --- | --- |
| `semio commands [words…] [--json]` | IMPLEMENTED (static) | `lib.rs:66`, `registry:1433-1460` (`--json` `:1450`, search `:1448-1449`); extras `--all --check --refresh --resolve --extra --root --snapshot --fresh-graph` |
| `semio run <id> [--param k=v]… [--detach] [--wait-ready] [-- args…]` | MISSING | no `run` arm in `lib.rs:57-69`. `semio run …` falls to `root_delegation` and `bun ./📜️script.ts run …`. The root script has no `run` branch (grep `"run"` finds only nx invocations). Behaviour of that delegation UNVERIFIED |
| `semio tasks [--json]` | MISSING | no dispatch |
| `semio logs <task> [--follow]` | MISSING | no dispatch |
| `semio stop\|restart\|kill <task>` | MISSING (CLI); protocol present | daemon `ClientMsg` has `Stop`, `Restart`, `Kill`, `StopGroup`, `KillGroup` (`D/🌀️daemon/🦀️.rs:39-58`); no CLI maps to them |
| `semio open <task>` | MISSING | no dispatch |
| `semio daemon start\|status\|stop` | IMPLEMENTED | `D/🌀️daemon/🦀️.rs:1096-1123` (`start` `:1101`, `stop` `:1115`, `status` `:1116`); `start_detached` `:1061`; `lib.rs:61` |
| Exit code of an attached `run` equals the task's | MISSING | no attached `run` |
| Task reference `<task>` resolves to a running session | MISSING | sessions keyed by `session_id`; no lookup |

**Extra surface not in §2.4** (listed for the coordinator): `dashboard`, `preferences`, `dev`, `catalog`, `command-tree`
(`--dump-tree`), `plugin registry generate|check`, `workflow`, `repo-view` (internal, used by `registry:1344`), `daemon serve|attach`,
and root delegation for every other verb. `semio dev` is a second playground resolver (§1 Q2).

---

## 6. Tests And Fixtures

| Item | Status | Evidence |
| --- | --- | --- |
| Registry unit tests | MISSING | `D/🎮️registry/🧪️tests/🔬️unit/🦀️.rs` is one line (`use super::*;`) |
| Inventory unit tests | MISSING | `D/📚️inventory/🧪️tests/🔬️unit/🦀️.rs` is one line |
| Registry feature and adapter (`🥒️.feature` + Rust adapter + oracle) | MISSING | `D/🧪️tests/` has features for `⌨️controls`, `⚙️preferences`, `🌀️control-plane`, `🌳️command-tree-projection`, `🧊️execution` only |
| Registry fixture `D/🧫️fixtures/🎮️registry/🏗️workspace.json` (364 lines) | UNUSED | `git grep` for `fixtures/🎮️registry` and `🎮️registry/🏗️workspace` is empty |
| Command-tree unit tests | STALE, does not compile (VERIFIED by `cargo check --tests`: 12 errors) | `D/🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs`: `segment_key` not in scope (`:13-14`), `LAUNCH_CONFIGURATIONS` (`:203`), `crate::inventory::commands` (`:210`), `inject_repo_domain` (`:117`), `nx_env` (`:216`, `:224`), `CommandSpec` passed where `&CommandSpec` is expected (`:222`), `into_command_node("root", "semio")` where `command-tree:93` takes `depth: usize` (`:118`, `:193`), `strip_jsonc` (`:237`), `collect_inferred_targets` (`:191`). The registry and inventory unit test files compile (only unused-import warnings) |
| Daemon launch test | STALE, `#[ignore]` | `D/🌀️daemon/🧪️tests/🔬️unit/🦀️.rs:38-44` (§1 Q1 row 7) |
| Projection feature | CURRENT for the tree only | `D/🧪️tests/🌳️command-tree-projection/🥒️.feature`, adapter `🦀️.rs` (gated behind `sut`) |
| `🧫️fixtures/🚀️launch-configurations` | STALE | readers: command-tree test (`:200`, does not compile) and control-plane oracle (§1 Q1 row 8). Feature file present |
| `🧫️fixtures/🗣️launch-axes` | STALE | readers: command-tree test (`:24`) and control-plane `🟦️.ts:44-52` (Nx task graph of `@semio-tech/framework-os-dev`, not the registry) |
| `🧫️fixtures/🔎️launcher` | Launcher vectors, not a launch file | `D/🖥️terminal/🧪️tests/🔬️unit/🦀️.rs:27,41`, control-plane `🟦️.ts:13` |
| `⌨️usage` text test | CURRENT (static) | `D/⌨️usage/🧪️tests/🔬️unit/🦀️.rs:5-9` pins `USAGE` (`⌨️usage/🦀️.rs:4`) |
| Dashboard `test` target | Includes a stale oracle and a non-compiling Rust unit target | `D/📦️packages/🦀️rust/📜️script.ts:34-39` |

---

## 7. README And Usage Text

**README (`D/README.md`) is stale.**

- `:21-22` the launcher offers "every terminal configuration and compound of `.vscode/launch.json`". Stale.
- `:26-28` the entry `build / launch / <name> — <command>` and the search example. Stale.
- `:29` "Scripts and launch configurations are available on the first frame". Stale (no launch configurations).
- `:31-35` `${workspaceFolder}`, `${env:NAME}`, `${input:id}`, `sh -c` and `cmd.exe /d /s /c`, and "another debugger type". VS Code semantics. Stale (the registry uses `{workspace}`/`{param}` tokens, `registry:1252`, `:1282`).
- `:36-38` "A finite task (every verb except `start`, `dev`, `serve`, `watch`, `activate` and `preview`)". Stale: the code uses Nx `continuous`, tool `continuous` and declared `ready` (`registry:909`, `:999`, `:946`).
- `:128-129` fixture `🚀️launch-configurations` "pins launch-configuration resolution". Stale.
- `:142-143` "runs the selected launch configuration to its exit code". Stale.
- `:115-116` target list omits `test`, `test-quick`, `test-long`, `test-exhaustive` (`D/📦️packages/🦀️rust/📋️project.json`). Minor.
- `:128` is a sentence fragment. Minor.
- The README does not document `semio commands`, `command-tree`, `catalog`, `dev`, or the §2.4 verbs.

**Usage text (`D/⌨️usage/🦀️.rs:4`)** describes the existing verbs correctly and omits `run`, `tasks`, `logs`, `stop`,
`restart`, `kill`, `open`. Its test (`⌨️usage/🧪️tests/🔬️unit/🦀️.rs:8`) pins the same string, so adding verbs means updating both.

**Schema roots.** `D/🧬️schema/🎮️registry/🔣️.json:6` sets the root `$ref` to `#/$defs/TicketCommands`, so the file titled
"Dashboard command registry" validates ticket documents at its root. `D/🧬️schema/🔣️.json` is the command-tree document
(title "Dashboard command tree"), consistent with `command-tree --dump-tree`.

---

## 8. Other Findings

1. **Dead files.** `D/🌀️daemon/📼️replay/🦀️.rs` (673 lines), `D/🌀️daemon/🚚️transport/🦀️.rs` (596 lines) and
   `D/🌀️daemon/✉️ipc/🦀️.rs` (459 lines) have no `mod` or `#[path]` referencing them (`git grep` for `replay/`, `transport/`,
   `ipc/`, `mod replay|transport|ipc` finds none). The live daemon has its own inline `pub mod ipc` (`D/🌀️daemon/🦀️.rs:12-445`)
   and `supervisor` (`:446`). The ready tracker §3 needs exists only in the orphan `replay` file.
2. **Duplicated wire types.** `TaskLabel` and `Ready` are defined three times: `registry:19-37`, `daemon/🦀️.rs:79-97`, and the
   orphan `ipc` file `:76-92`. §3 says the single home is `daemon::ipc`.
3. **Declared but unused daemon messages.** `SpawnGroup`, `StopGroup`, `KillGroup` exist (`daemon/🦀️.rs:39-58`) with no
   live caller besides the daemon itself (static).
4. **Parallel resolver.** `semio dev` resolves playgrounds without the registry (`D/🛝️playground-session/🦀️.rs:14`).
5. **Group schema vs data.** The schema, the Rust type and the 11 root groups all use `target` (singular). Amendment 3 is
   unimplemented in both code and data.
6. **Nx-based gates still stand for launch files.** The `verify interactivity` gates (§1 Q1 rows 1–2) fail when a launch
   row is removed. Dissolving `.vscode/launch.json` requires changing `📜️script.ts:8717-8730` and `:9009-9045` first (L-1 scope).

---

## 9. Scope Of Verification

| Check | Result |
| --- | --- |
| `cargo check -p semio-framework-repo-dashboard --message-format=short`, audit-start state (twice) | EXIT 101, blocked by `semio-framework-ui` (`🚇️pty/🦀️.rs:728:107`), committed HEAD content |
| Same command, after a peer's uncommitted fix to `🚇️pty/🦀️.rs` | EXIT 0; dashboard crate checked, no warnings (`🗑️generated/r2-audit-cargo-check-final.txt`) |
| `cargo check -p semio-framework-repo-dashboard --tests` | EXIT 101, 12 errors, all in `🌳️command-tree/🧪️tests/🔬️unit/🦀️.rs` (`🗑️generated/r2-audit-cargo-check-tests.txt`) |
| `cargo test` of the dashboard crate | NOT RUN (test build fails, §6). Runtime tests UNVERIFIED |
| Runtime behaviour of `semio commands`, `semio run`, the TUI launcher | UNVERIFIED (nothing run; no install, no daemon start) |
| `bun test` of `D/🧪️tests/🌀️control-plane/🟦️.ts` and the `verify interactivity` gate | UNVERIFIED (not run) |
| Repo-wide claims about readers | `git grep` over tracked files; `.cursor/plans` and ticket records excluded from the reader list (documents, not readers) |

Commands used (repo root `C:\git\semio`, Bash, `git -c core.quotepath=false`):

- `git grep -l '"dashboard"' -- '*📋️project.json'` (19 files), parsed with `node` (`🗑️generated/r2-scan-dashboard-decls.mjs`).
- `git grep -n -E 'launch\.json|launch\.seed|🧩️launch|LAUNCH_CONFIGURATIONS|launch-configurations|launch-axes|\.claude/launch' -- '*.rs' '*.ts' '*.py' '*.sh' '*.feature' '*.json' ':!.vscode/*' ':!*.md' ':!.🧬semio/*'`
- `CARGO_TARGET_DIR=/c/git/semio/.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-audit-a1 cargo check -p semio-framework-repo-dashboard --message-format=short` (run three times, see §9)
- `cargo clean -p semio-framework-repo-dashboard` with the same audit target dir (to force the crate's own check), then the same check, and `cargo check -p semio-framework-repo-dashboard --tests`
