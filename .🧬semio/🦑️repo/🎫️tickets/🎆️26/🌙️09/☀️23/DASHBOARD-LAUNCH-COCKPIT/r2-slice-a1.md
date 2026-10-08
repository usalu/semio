# R2 Slice A-1: Command Registry And Non-Interactive CLI

## Round 5 (newest): final audit items 2, 4, 8

- **Root delegation dissolved.** Module `📜️root-delegation`, `ROOT_VERBS` and its drift test are deleted; `semio` forwards nothing. Unknown verb -> `[semio] unknown verb "x"` + usage on stderr, exit 2 (`verify`, `test`, `build`, `commit`, `dev`, `setup` are asserted to be refused). Root-script routes are reachable as registry commands: every route had root-project targets (`agents`, `verify`, `setup`, `dev`, `generate`, `schema`, `lint`, `format`, `test`, `clean`, `new`, `bench`, `stdio`, `build`, `cpp`, `publish`) except `os`, `semio` and `examples`, for which I added the targets `os`, `semio`, `examples` to root `📋️project.json` (`bun ./📜️script.ts <route>`, `forwardAllArgs`): `semio run workspace:os -- run <bundle>.studio`. `commit`/`micro-commit` stay agent-skill implementation (`bun ./📜️script.ts …`), untouched. `git grep` found no caller of `semio <root verb>` outside the dashboard README (the retired `semio dev` note).
- **`semio --help` / `-h` / `help`** print the CLI usage on stdout, exit 0; `semio dashboard --help` prints the keyboard help.
- **USAGE lists every flag**, `usage::FLAGS` states them as data per verb, and the drift test `…_every_flag_a_verb_reads_is_in_the_usage_text_the_readme_verb_row_and_the_argument_readers` checks (a) each flag in the verb's USAGE block, (b) each flag in the README verb row, (c) USAGE names no flag FLAGS lacks, (d) the flags the `🧭️cli` source reads (`has`, `value`, `pairs`, `all`, valued-flag lists) equal the FLAGS of its verbs. Also fixed: the verb drift test no longer counts `--help` as a verb.
- **Package-level `dev` scripts** (five, all self-aliases `bun nx run <project>:dev`; each project has its `dev` target in its `📋️project.json`, so each is a registry command and resolves with `run --dry-run`): REMOVED `dev` from `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/…/package.json`, `♻️mit-bestand/🧺️demonstrator/package.json`, `🌎️hub/🔨️modules/🛡️admin/…/package.json`, the quiz `package.json` (plus its `dev:site`, same kind), and `🏢️semio-tech/🎡️play/package.json`, so `npm run dev` no longer starts a server outside the daemon. Their `build`/`test` aliases were not asked about and stay.
- **Docs:** `🌉️mcp/README.md` (run section and the pane paragraph) and root `README.md` MCP row now state that MCP clients start the stdio gateway from configurations `bun ./📜️script.ts agents write` writes, not developers; the HTTP server is `semio run tool:workspace/os-mcp-http` on **8792** (was 6300). Dashboard README: verb table with every flag, no forwarding, root routes are `workspace:<target>` commands, module list without root-delegation.
- **Emoji check:** `bun T/r2-q1a-emoji-check-changed.ts` with the `skipped` list emptied: files=113 problems=0 (two duplicate docstring emojis fixed in `⌨️usage` and `🧭️cli`).

Verified: `cargo test -p semio-framework-repo-dashboard --lib` = 204 passed, 0 failed; `bun test --timeout 240000 ./🧪️tests/🧭️cli/🟦️.ts ./🧪️tests/🎮️registry/🟦️.ts` = 10 pass; `semio commands --check --refresh --root /c/git/semio` = 26645 commands, **0 problems**; binary: `--help`/`-h` exit 0, `verify`/`frobnicate` exit 2.

---

## Round 4 (newest): one stable task handle

Bug from the real-workspace cutover: `run` printed place `1` (its own slice of the listing), `tasks` printed `128`, `stop 1` hit an old exited session. Cause: the handle was a *position*, computed per listing (and A-2's `select` preferred positions over ids).

- Handle = the **session id**, the same everywhere: `run` (detach / wait-ready / attached summary), `tasks` (text and `--json`; the `place` field is gone), `logs`, `stop|restart|kill`, `open`. Positions are no longer printed or accepted (a bare number answers `no task matches`).
- New `cli::pick(sessions, handle)`: exact session id; a group id (`group-…` without `.n`, or `session.group`) = all members of the launch; a command id = its live session, else its latest, several live = error listing them; a unique part of a session id or of a command id; ambiguity always errors with the candidates. `logs`/`open` need exactly one session (open prefers the one with a ready address), `stop|kill` act on every live session picked (error `has no live task to stop; <latest>` otherwise), `restart` on the picked sessions; `--group` still sends `StopGroup`/`KillGroup` for a compound. A-2's `daemon::control::select` is no longer used by the CLI (no daemon change needed; numbering was never the daemon's).
- `run --detach --wait-ready` keeps the ready URL as the last line, alone (journey asserts it); a running task with the same command id, words and parameters is reused under the same handle.
- Tests: `a_handle_is_a_session_id_a_group_a_command_or_a_unique_part_and_never_a_position` (unit); journey `🧪️tests/🧭️cli/🟦️.ts` against an isolated daemon (`SEMIO_DASHBOARD_INSTANCE=cli-<pid>-<time>`, `daemon serve` in the foreground, temp workspace): run -> tasks (text and json) -> reuse -> logs/open by handle, command id and group -> stop by command id, again (refused), run again with a new handle -> logs of the ended one -> stop by handle -> run -> stop by group. Wired into the dashboard `test` target.
- NOTE: `🧪️tests/🧭️cli/🥒️.feature` already existed (five scenarios mapped by `cli::tests::every_scenario_of_the_cli_feature_is_proved_by_a_test`); I overwrote it by mistake, restored the five scenarios (bodies re-written from their titles and tests) and appended the four handle scenarios, all mapped.

Verified: `cargo test … --lib -- cli:: registry:: inventory:: command_tree:: root_delegation:: usage:: the_dispatch` = 68 passed; `bun test --timeout 240000 ./🧪️tests/🧭️cli/🟦️.ts ./🧪️tests/🎮️registry/🟦️.ts` = 10 pass (55 s); `semio commands --check --refresh --root /c/git/semio` = 26573 commands, 0 problems.

---

## Round 3 (newest, from V-1 battle tests)

| Id | Result |
| --- | --- |
| F9 bare `--param` | CONFIRMED with the current binary: `run playground:s --param hub --param local-only --dry-run` sets `S_HUB_URL` and `S_LOCAL_ONLY`; a bare choice id is refused (``--param cache: a choice parameter needs a value, write `--param cache=<value>` ``) |
| F10 `SEMIO_APP` | The name is dead: nothing in the repo reads `SEMIO_APP`. The doors read `VITE_SEMIO_APP_ID` (React, already set) and `SEMIO_APP_ID` (wgpu serve meta tag `🌐️server/🟦️.ts:72`, native renderer `🧊️renderer/🦀️.rs:19682`), which was never set. Now `env_contract::build_dev_env` sets `SEMIO_APP_ID` from the catalog row's `app` beside `VITE_SEMIO_APP_ID` (rows without `app` pin none). Test `a_playground_pins_the_app_of_its_catalog_row_for_every_door`; TS oracle compares both names. V-1's 144 differences are the old dead name: its expectation should read `SEMIO_APP_ID` |
| F11 `--env` and ready port | FIXED: extra environment is merged into the final environment before `ready` is read (values with `{port}` expand afterwards); a `ready.portEnv` that names a parameter reads that parameter's `valueEnv` variable from the final environment; a playground's ready port follows `S_OS_PORT`. Real workspace: `run @teaching/architecture-quiz:dev --env TEACHING_ARCHITECTURE_QUIZ_PORT=6099 --dry-run` -> ready port 6099. Test `the_ready_port_reads_the_final_environment_…` (hub, quiz, playground, effect vs extra precedence, bad port) plus four new TS oracle cases |
| F12 port 6300 | `tool:workspace/os-mcp-http` now declares ready `8792` / `S_OS_MCP_PORT` and runs `--port {port}` (root `📋️project.json`). 8792 sits with the service ports (hub 8787, admin 8790, proctor 8791; catalog ports are 6xxx up to 6300). Request: the manual default `process.env.S_OS_MCP_PORT ?? "6300"` in `…/🚀️bootstrap/📜️script.ts:339` should become 8792 |
| `commands --check` ports | NEW problem: a port claimed by commands of different owners that can run together. Owner = project of a target or tool, ticket of a ticket tool, the playground itself; the claims are default ready ports of long-running commands plus every playground port (react, wgpu, user slots). Same-owner claims are alternatives (storybook scopes, hub variants, inspector tool + targets) |
| `commands --check` graph | NEW problem: a playground renderer whose Nx target (`@semio-tech/framework-os-dev:dev-<v>-react-dev`, `…-wgpu-dev`, `run-<v>-native-dev`) is not in the graph. Fixture graph gained the owner project so the fixture still passes its own check |
| Graph `errors` | DONE: `read_graph` reads the published graph's top-level `errors` array (`name`, `pluginName`, `pluginIndex`; other fields ignored) and keeps it in the snapshot (version 5); every entry is a problem (file `.nx/workspace-data/project-graph.json`, at `errors[i]`, N-1's wording), so `commands --check` is not healthy on a partial graph. Test `a_partially_published_graph_is_a_problem_per_recorded_plugin_error` (no array, empty array, three entries) |
| Port design | Documented in the README ("A service port belongs to one owner") and the registry feature. No schema flag is needed: commands of one owner (the targets of a project and the tools that project declares) are alternatives, so 6010 (storybook targets + `storybook-static` tool) and 6274 (`dev-mcp` + `mcp-inspector-os`) are alternatives by rule and no port changed |
| Bootstrap default 6300 | already gone from the bootstrap script (no `S_OS_MCP_PORT` and no "6300" left there); nothing to edit |


Verified (final, after the graph-error check): `cargo test -p semio-framework-repo-dashboard --lib` = 202 passed, 3 failed; all three are A-2's `daemon::integration` tests (Windows path case `only.CMD` vs `only.cmd`; two timing tests under a loaded machine), every registry/inventory/cli/command-tree/root-delegation/usage test passes; `semio commands --check --refresh --root /c/git/semio` = 26564 commands, **0 problems**; `bun test ./🧪️tests/🎮️registry/🟦️.ts` = 8 pass, 317 expects (one earlier run under load failed once and passed on re-run).

---

## Round 2b (newest)

| # | Item | Status |
| --- | --- | --- |
| 1 | `semio dev` second resolver | DISSOLVED, no shim. Module `🛝️playground-session`, its `dev` dispatch arm, `options::parse_lock`, `DevOptions.skip_*` are deleted. `semio run playground:<variant> --param renderer=… --param example=…` covers it (parameters `hub`, `data`, `local-only`, `app-role` from the catalog row; `SKIP_PLUGIN_BUILD=1` etc. are `--env`). Catalog aliases (`puzzle 3d`) are searchable (`semio commands puzzle 3d`). `dev` is now a root-script route (`ROOT_VERBS`, drift-tested) so `semio dev storybook\|mcp …` still reach the root router |
| 2 | `registry::xxh3_64` public | KEPT (Bun's XXH3 vectors still pinned) |
| 3 | Shared ellipsis helpers | DONE: registry uses `ui_tui::tui::text::{elide_end, elide_middle}`; private `take_cells*`/`elide_*` deleted |
| 4 | README daemon sections | DONE: new "The Daemon" section (hello/protocol/build id, group start, daemon-side ready detection, 2 MiB ring + rotating logs, per-view cursors/flow control, journal, Windows pipe security, stop ladder, 16 views/128 sessions) from `r2-slice-a2.md` and the source constants |
| 5 | `semio workflow run` | DISSOLVED: no caller anywhere (one historic `🌊️workflow.json` in a ticket), it spawned `cursor-agent` outside the daemon. Module `🌊️workflow`, Nx target `workflow`, `WorkflowScript`, the `dispatch_macros` dependency and the verb in `📦️installation` are removed. Agent processes are ordinary tasks: declare them as ticket `🎮️commands.json` tools/compounds and `semio run` them |
| 6 | Usage = dispatch = README | DONE: `NATIVE_VERBS`/`INTERNAL_VERBS` in the lib; test `the_dispatch_table_the_usage_text_and_the_readme_verb_table_name_the_same_verbs` parses the dispatch arms, `USAGE` and the README table and fails on any drift (README table now lists dashboard, preferences, catalog, plugin registry, command-tree too) |
| 19 | `launch_json` | RENAMED `launch_plan_json` (registry, cli, tests; A-3's call site agrees). No "launch.json" vocabulary left in my folders |
| M-2 | `--env` tokens | DONE: values expand `{workspace}`, `{port}` (the process's ready port) and chosen `{parameter}`; unknown tokens are errors, braces that are no token stay |
| add-on | `--param <id>` | DONE: a bare id switches a `flag` on, a bare choice/text id is refused with ``--param x: a choice parameter needs a value, write `--param x=<value>` ``; unknown ids stay the registry's to refuse |

Verified: `cargo test -p semio-framework-repo-dashboard --lib` = **180 passed, 0 failed** (CARGO_TARGET_DIR `target-fleet-a1`, `CARGO_BUILD_BUILD_DIR` private); `semio commands --check --root /c/git/semio` = 20409 commands, **0 problems** (target 8792, playground 155, ticket 1084, tool 11, compound 4, group 9, repo 10354); `bun test ./🧪️tests/🎮️registry/🟦️.ts` = 8 pass (re-run after all changes).

### Requests (outside my folders; for the coordinator to dispatch)

- `📜️script.ts:335-353` (`DevScript.run`: branches `s`, `resolvePlaygroundDevApp`, `runFrameworkOsPlaygroundDev`, fallback `runFrameworkOsPlaygroundDev("s")`) and helpers `📜️script.ts:123` (`resolvePlaygroundDevApp`) and `:154` (`runFrameworkOsPlaygroundDev`): a TypeScript playground resolver beside the registry. Keep only `storybook`, `storybook-static` and `mcp`; replacement for playgrounds: `semio run playground:<variant>`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:337-350` (`resolveNxInvocation` for `workspace:dev` / `@semio-tech/framework-os-dev:dev`: the `multi`, `served` and catalog branches resolve playgrounds again) with tests `…/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts:1050,1051,1216,1229,1240` and `…/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts:2193-2195` (`frameworkOsPlaygroundDevEnv` incl. `SKIP_ENGINE_BUILD`): same resolver in a third place; the registry already emits `@semio-tech/framework-os-dev:dev-<variant>-<renderer>-dev`.
- `📋️project.json:1220` target `workspace:dev` and `:263` (tool `os-mcp-http` runs `bun nx run workspace:dev -- mcp http os`): after the playground branches go, `workspace:dev` only routes `mcp`/`storybook`; consider pointing the tool at its real target `@semio-tech/framework-os-mcp-rs:dev` (what the bootstrap rewrites it to).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/…` none; docs mentioning `semio dev <variant>`: none found by `git grep` (only Vite banners `semio dev ·` in `🧑‍💻dev/🔌️vite-plugins/🟦️.ts:697,1265`, which are log text, not the verb).
- `.🧬semio/…/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🌊️workflow.json`: ticket data for the dissolved verb; its owner should turn the tasks into `🎮️commands.json` tools or drop it.

---

## Round 2a

Date 2026-10-08. Dashboard `D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`. Peer-facing API: `r2-slice-a1-api.md`.
Target dir of all runs: `.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-a1`. The real daemon was never started or stopped; end-to-end
runs used a throw-away workspace in the scratchpad (deleted afterwards) with its own daemon.

## 1. Status Per Item

| # | Item | Status |
| --- | --- | --- |
| 1 | Broken command-tree tests; `cargo check -p semio-framework-repo-dashboard --tests` | DONE and VERIFIED (first fix compiled; the whole test target compiles now). Launch-configuration tests deleted, the rest re-expressed against the registry |
| 2 | Circular registry/command-tree dependency | DONE. Repo-domain code (`RepoAction`, adapters, `action_key`, `ANALYZE_SCOPES`, implementation choice, Go binary path, `repo-view` verb) moved to new module `🏛️repo-domain`; registry imports it, `🌳️command-tree` imports the registry (one direction) and is a pure projection: `Leaf { id, parameters }`, `LauncherRow`, `tree`, `tree_json`. `CommandSpec`/`CommandLeaf` are gone; the only start input is `registry::Launch` (`Registry::resolve(id, &Request)`). Schema `🧬️schema/🔣️.json` updated (`InvocationLeaf`) |
| 3 | §2.2 + amendments | DONE: 1 (object form for `requires` and compound members with `parameters`+`env`), 3 (`targets: [..]`), 4 (axes reach tools by verb when they change more than Nx flags; `nxFlags` ignored on tools), 5 (`Request.env` / `--env`), 6 (`ready.printed`, `Ready.printed`), 8 (playground facts, below). Amendment 2/7 were already in place. Schema `🧬️schema/🎮️registry/🔣️.json` updated (`Member`, `Requires`, `Group.targets`, `printed`, `RegistryEntry.listed/mutating`, no `script`) |
| 3b | Coordinator decision: drop root scripts | DONE: no `Kind::Script`, no `script:` id, no `Facts.scripts`, no snapshot part; a project may now be called `script` |
| 4 | §2.4 CLI verbs | DONE and exercised end to end on a temp workspace (section 3). Root delegation reduced to the verbs the root script routes (`ROOT_VERBS`, drift-tested against the router); any other verb prints the usage and exits 2 |
| 5 | Usage and README | DONE: `⌨️usage` rewritten with a test; `README.md` rewritten (registry sources, declarations, CLI, launcher, keys from the A-3 keymap, no launch file, no `dashboard:start` etc.) |
| 6 | Tests | DONE and RUN (section 4) |

## 2. Changes

- New modules: `🏛️repo-domain` (+tests), `🧭️cli` (+tests). `📦️packages/🦀️rust/🦀️.rs` wires both and dispatches `commands run tasks logs stop|restart|kill open`; `root_delegation::run(root, argv)` now forwards raw argv (it used to drop all flags).
- `🎮️registry`: `Request`, `RunPolicy { reuse_published_graph, repo }` with `RunPolicy::current()`, `Member`, `canonical_value` (example by slug or bare id), `nx_flags` de-duplicated, `resolve_with` validates the extra environment, tool axes, group `targets`, catalog-driven playground parameters `hub`/`data`/`local-only` and `viewerPath` ready path. `registry::run` (the `commands` verb) moved to `🧭️cli`.
- `📚️inventory`: no scripts, `Update::Ready(Vec<LauncherRow>, String)`, catalog facts read (`hub`, `dataDir`, `userDataDir`, `localOnly`, `viewerPath`), snapshot version 4, walk skips `fixtures`, and a second manifest that reuses a project name is reported as a problem and never replaces the first.
- Amendment 8 pipeline (outside my folders, necessary): `🎮️playground/🔎️discovery/🟦️.ts` and `📽️projection/🟦️.ts` of the plugin registry read/emit the new optional row keys (`hub`, `data_dir`, `user_data_dir`, `local_only`, `viewer_path`); rows added to `🌎️hub/🧩️compositions/🪐️space/…/Cargo.toml` (`s`) and `🌀️procedural/…/Cargo.toml` (`generation3d`); catalog regenerated with `bun ./📜️script.ts generate` (generated dir is git-ignored). The "example lock" fact needs no new key: choosing `example` already exports `PLAYGROUND_LOCKED_EXAMPLE_ID`.
- Data fixed so the workspace is clean (`commands --check` = 0 problems): root `📋️project.json` groups `target` -> `targets` (11), `hub: true` pinned on the three `playground:s` compound members; pets `dev` parameter `port` (a reserved token) renamed `stories-port`.
- Fixture `🧫️fixtures/🎮️registry/🏗️workspace.json` extended (object-form requires, printed ready, multi-target group, catalog facts); L-1's `🧫️fixtures/🎮️registry/🥒️.feature` edited for the new truth and mapped to Rust tests by a test.
- `📦️packages/🦀️rust/📜️script.ts` test target builds the debug `semio` and runs the registry TS oracle.

## 3. CLI Contract (binary `semio`)

| Verb | Contract |
| --- | --- |
| `commands [words…] [--json] [--all] [--check] [--refresh] [--root P] [--snapshot P]` | all words must occur in label or id; `--all` adds closed-ticket commands; `--check` prints problems on stderr and `<n> commands, <m> problems`, exit 1 on any |
| `run <id> [--param k=v]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [--json] [--timeout S] [--raw] [-- args…]` | `--dry-run`: print the resolved `Launch` JSON (schema `Launch`), start nothing, no daemon. Attached: stream the last process's output (control sequences stripped unless a terminal or `--raw`), exit with its exit code. `--detach`: return when the daemon accepted the launch (prints `place session status label`). `--wait-ready`: return when every process that declares `ready` printed its address, which are the last stdout lines (default timeout 600 s). A live task with the same command id, words and chosen parameters is reused. Errors (unknown command/parameter/value, missing required parameter, bad `--env`, runner variable) exit 2 with a message on stderr and nothing on stdout. Repo commands run in process under the Rust implementation. Starts the daemon when none runs |
| `tasks [--json]` | list (place, session, status, label, ready url); falls back to the journal when no daemon runs |
| `logs <task> [--follow] [--raw]` | replay (daemon) or log files (no daemon) |
| `stop\|restart\|kill <task> [--group] [--timeout S]` | waits until the task left its state; `--group` stops/kills the whole compound |
| `open <task> [--print]` | prints the ready URL and opens the system browser |
| `daemon start\|status\|stop\|attach` | unchanged (A-2) |

`<task>` = session id, place in `tasks`, command id (live run preferred), or a unique part of either (A-2's `select`).

## 4. Verification

- `cargo test -p semio-framework-repo-dashboard --lib` : 163 passed, 0 failed (includes peers' tests). Filtered to my modules (`registry:: command_tree:: inventory:: repo_domain:: cli:: usage:: root_delegation::`): 56 passed before the last additions, all green after (registry 28, inventory 11, command-tree 7, repo-domain 4, cli 9, usage 1, root-delegation 4 ...).
- `bun test ./🧪️tests/🎮️registry/🟦️.ts` (needs `SEMIO_DASHBOARD_BIN` or a built debug binary): 8 pass, 284 expects. Independent TS resolver vs `semio run --dry-run` on 27 selections (targets with axes/configurations/ready/requires, tools, groups, compounds, ticket tools, playgrounds incl. slots/hub/local-only/viewer/native), `commands --json --all` id set vs an independent walk of the fixture (and `RegistryEntry` validation), Ajv for declarations/outputs, negative cases, root-script refusal, Bun's XXH3 vs the Rust vectors.
- Real workspace (`semio commands --check --root /c/git/semio`, debug binary): 26274 commands, **0 problems**. By kind (`commands --json --all --refresh`): target 14660, playground 155, tool 10, compound 4, group 11, ticket 1080, repo 10354.
- End to end on a temp workspace with its own daemon (daemon started with `daemon serve` because the harness forbids detached process creation, see requests): attached run exits with the task's code (0 and 3), `--detach --wait-ready` printed the ready URL in 0.9 s and the URL answered, second call reused the task, `tasks`/`tasks --json`/`logs`/`open --print`/`restart`/`stop`, `requires` (client waited for the service), compound start and `stop --group`, offline `tasks`/`logs` after `daemon stop`.

## 5. Requests And Findings For Others

- A-2: (a) the daemon writes `[semio] waiting for the services and members before it` into the PTY stream of every group member, so it appears in `run`/`logs` output; it should be a session notice, not output. (b) `semio daemon start` (and `run`'s auto start) fails in this harness with `CreateProcessW detached failed: Access is denied (os error 5)`; I could not verify the detached start path. (c) A-2's `SessionCommand.cwd` is canonicalised by the daemon, so a client cannot compare it; the CLI reuses a task by command id, words and chosen parameters instead.
- A-3: `command_tree::launcher` still exists (`LauncherRow`) but you use your own entry list; nothing else is required from A-1.
- V-1 / `m1a-coverage.ts`: not re-run. Known differences from its expectations: an example is exported as the catalog slug (`🎬️demo`), not the bare id (both are accepted as input); `--projects=` of the TS resolver is `-p` in the registry; `nxFlags` are de-duplicated.
- Open: `semio dev` (`🛝️playground-session`) is still a second playground resolver (audit finding 4); I did not route it through the registry.
- C-1a's `mcp` field of `ToolDeclaration` and `McpExposure` of the schema were kept untouched.
- Environment notes: `📋️project.json` / registry files intermittently refuse writes (EINVAL/permission) while another process holds them; edits were retried.
