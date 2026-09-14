# 📓️ Opus `dashboard` — the `🎛️dashboard` module, its Rust crate and the repo-domain leaves

Scope: plan §1 outcome 4, §2 row `🎛️dashboard`, §3, §5. Owner of `🔨️modules/🎛️dashboard/` and of the
now-deleted `🎮️commands/`.

## 1. What was done

### 1.1 New module `🔨️modules/🎛️dashboard/`

`🎮️commands/` is gone. Its nine command files are now domain sub-folders of the dashboard module,
each pulled into the crate godfile with the same `#[path]` pattern the CLI used:

| new sub-folder | former `🎮️commands` folder |
| --- | --- |
| `🖥️terminal/🦀️.rs` | `🎛️terminal-dashboard` |
| `🌀️daemon/🦀️.rs` | `🖥️terminal-dashboard-daemon` **+** the `🔖️Ipc` and `🔖️Daemon` regions of the CLI godfile |
| `🌳️command-tree/🦀️.rs` | `🌳️command-tree-discovery` |
| `🌊️workflow/🦀️.rs` | `🌊️workflow` |
| `🔌️plugin-registry/🦀️.rs` | `🔌️plugin-registry` |
| `🛝️playground-session/🦀️.rs` | `🛝️playground-development-session` |
| `📇️playground-catalog/🦀️.rs` | `📇️playground-catalog-query` |
| `📜️root-delegation/🦀️.rs` | `📜️root-script-delegation` |
| `⌨️usage/🦀️.rs` | `⌨️cli-usage-presentation` |

`🎮️commands/🔣️.json` became `🎛️dashboard/🔣️.json` (kind `collection`, nine members, directories and
ids updated, `framework.repo.command.*` → `framework.repo.dashboard.*`).

New crate `📦️packages/🦀️rust` — `semio-framework-repo-dashboard`, `[lib] path = "🦀️.rs"`,
`role = "library"`, `[lints] workspace = true`, with `📋️project.json`
(`@semio-tech/repo-dashboard-rs`: `build`, `test`, `test-{quick,long,exhaustive}`) and `📜️script.ts`
(`BuildScript`/`TestScript` over `runCargoTestBudgeted`). Registered in the root `Cargo.toml`
members list next to the CLI crate.

The crate godfile also carries the support regions the commands need, moved out of the CLI godfile
so nothing is duplicated: `🔖️Args`, `🔖️Proc`, `🔖️Catalog`, `🔖️Options`, `🔖️EnvContract`, plus
`pub use daemon::ipc`.

### 1.2 The CLI crate now depends on the dashboard crate

Edits to `🔨️modules/⌨️cli/📦️packages/🦀️rust/🦀️.rs` were kept to the top-of-file module block and
the `run()` dispatch arms, as instructed (a concurrent `cli` agent owns the rest of that file):

- the nine `#[path = "…🎮️commands/…"] pub mod …;` declarations were replaced by
  `pub use semio_framework_repo_dashboard as dashboard;` and `pub use dashboard::args;`
  (so every `crate::args::…` the CLI agent writes keeps resolving);
- the regions that moved (`Args`, `Proc`, `Catalog`, `Options`, `EnvContract`, `Ipc`, `Daemon` and
  the old `Tests` region that only exercised them) were removed by their `#region`/`#endregion`
  markers; `🔖️Workspace` stays in the CLI;
- every dispatch call was repointed (`terminal_dashboard::` → `dashboard::terminal::`,
  `terminal_dashboard_daemon::` → `dashboard::daemon::`, `cli_usage_presentation::` →
  `dashboard::usage::`, `playground_development_session::` → `dashboard::playground_session::`,
  `playground_catalog_query::` → `dashboard::playground_catalog::`, `root_script_delegation::` →
  `dashboard::root_delegation::`, `command_tree_discovery::` → `dashboard::command_tree::`,
  `plugin_registry::run`/`workflow::run` → `dashboard::…`);
- a `"command-tree" => dashboard::command_tree::run(&root, &parsed)` arm was added.

`Cargo.toml` of the CLI gained `semio-framework-repo-dashboard` and lost `ui_tui`, `ui_styling` and
`dispatch_macros` — after the move nothing in the CLI names them any more (verified by grep before
removing).

### 1.3 Repo-domain leaves, in process (plan outcome 4)

`🌳️command-tree/🦀️.rs` grew three things.

**A leaf is no longer only a process.** `CommandNode.spec: Option<CommandSpec>` became
`CommandNode.leaf: Option<CommandLeaf>` with

```rust
pub enum CommandLeaf { Process(CommandSpec), Repo(RepoAction) }
```

**`inject_repo_domain(root, trie)`** runs after `inject_playground_dev` and adds:

- `tickets/<id>/{show,files,close,reopen}` — one branch per ticket found under
  `.🧬semio/🦑️repo/🎫️tickets`, listed newest first, the branch label carrying `<id> [status] <title>`.
  Every ticket is listed rather than only the open ones, because `reopen` is only meaningful on a
  closed one and `close` only on an open one; the domain function refuses the wrong pairing itself.
- `goals/{list,tree}`
- `analyze/{all,rust,typescript,go,python,markdown}`
- `tree/{monorepo,goal,statute,territory}`
- `statutes/catalog`

`VERB_ORDER` was extended with `tickets, goals, analyze, tree, statutes` so the repo branches sort
directly after the nx verbs.

**`mod repo_domain`** is the adapter layer between the wizard and the Rust domain crates. It uses
the same functions the `semio` verbs use, never a spawn:

| leaf | domain call |
| --- | --- |
| `tickets/<id>/show` | `tickets::TicketService::read` + `encode_ticket_document` |
| `tickets/<id>/files` | recursive `TicketStore::entries` over the ticket folder |
| `tickets/<id>/close` | `TicketService::close` with `bulk: true, no_management: true` |
| `tickets/<id>/reopen` | `TicketService::reopen` with `client: "claude-code", no_management: true` |
| `goals/list`, `goals/tree` | `goals::Goals::list` over `FsGoalStore`, `goals::build_goal_tree` + `render_goal_tree` |
| `analyze/<scope>` | `codebase::Codebase::glob_by_extension` → `statutes::SourceSet` → `statutes::analyze` → `filter_ignored` |
| `tree/monorepo` | `codebase::CodebaseContext` → `tree::MemoryTreeSource` → `tree::build_monorepo_tree` → `tree::tree_outline` |
| `tree/goal` | `tree::build_goal_tree` + `count_open_{subgoals,tickets}` |
| `tree/statute`, `tree/territory` | `tree::build_{statute,territory}_tree` over a `StatuteCatalog` adapter backed by `statutes::statutes()`/`policies()` |
| `statutes/catalog` | `statutes::statutes()` |

Three port adapters live with the consumer, as the domain crates ship no production implementation
of them: `SystemClock` (`tickets::Clock`, wall clock via a hand-rolled civil-from-days conversion —
no date crate enters the runtime), `CoordinatorEmitter` (`events::Emitter` for goals) and
`DeclaredStatuteCatalog` (`tree::StatuteCatalog`). The issue tracker is the real
`providers::system_management_provider()`; the two mutating leaves pass `no_management: true`, so a
single keystroke in a wizard never reaches `gh` or the network.

**Implementation selection.** `SEMIO_REPO_IMPLEMENTATION` (`rust` default, `go`):

```rust
pub fn repo_implementation() -> RepoImplementation
pub fn go_binary_path(root: &Path) -> PathBuf   // ⌨️cli/📦️packages/🐹️go/semio-repo[.exe]
impl RepoAction { pub fn go_argv(&self) -> Vec<String>; pub fn execute(&self, root: &Path) -> String }
```

With `go` the very same wizard paths are emitted, but each leaf is a `CommandLeaf::Process` that
spawns the Go binary with `go_argv()` into the pseudo-terminal window instead. Discovery itself
(which tickets and goals exist) always reads through the Rust crates, because it is read-only.

**The terminal.** `🖥️terminal/🦀️.rs` gained `open_output_window` (extracted from `spawn_output`, which
now reuses it) and `show_repo_output`, which runs `RepoAction::execute` in process and feeds the
rendered text into a `TerminalState` output window with no PTY session attached. `Dashboard` carries
the repository root so the action can run.

### 1.4 Windows

The PTY spawn path already compiles and works on this host: `ui_tui`'s `tui::pty` has a real
**ConPTY** implementation under `#[cfg(windows)]` (`CreatePseudoConsole`/`ResizePseudoConsole`), so
`terminal::run`'s `Pty::spawn` is native on Windows and no piped-stdio fallback is needed. The gap
was the **daemon supervisor**, which was gated `#[cfg(all(unix, not(target_arch = "wasm32")))]` and
answered every Windows spawn with `ServerMsg::Error{"PTY supervisor requires unix tui-terminal"}`.
That gate is now `any(all(unix, not(target_arch = "wasm32")), windows)` on `LiveSession`,
`spawn_session`, `input`, `resize`, `kill` and `tick`, and the unix-only imports (`Read`,
`Ordering`, `Duration`) and the accept-loop-only `has_client`/`detach_client` were narrowed so the
Windows build is warning-free. `serve` itself stays unix-only (it binds a `UnixListener`); the
Windows named-pipe listener is out of this agent's scope and is called out in §4.

### 1.5 Language-agnostic case + schema

`🎛️dashboard/🧪️tests/🌳️command-tree-projection/` — `🥒️.feature`, `🦀️.rs` (Rust subject, `sut`-gated),
`🧫️fixtures/🏗️workspace.json`. The fixture is a frozen workspace (two `📋️project.json` manifests, two
`🎫️ticket.json` documents — one open, one closed — and one `🎯️goal.json`) which the adapter
materialises into `ctx.work_dir` and then projects with
`command_tree::tree_json_text(&root)`. Two scenarios:

- `@id-the-fixture-workspace-projects-its-command-tree` `@level-fundamental` `@mode-conformance`
- `@id-the-projection-does-not-depend-on-directory-order` `@level-quick` `@mode-property`

`🔮️oracle/🔣️.json` records the `noOracleDecisions` entry `repo-dashboard-command-tree`
(`substitutes: specification-vectors, metamorphic-laws`, `coversMutations: false`).

`🧬️schema/🔣️.json` (draft 2020-12) describes the tree JSON: `CommandNode`, `ProcessLeaf` and
`RepoLeaf` (with the `action` key pattern and `goArgv`).

### 1.6 launch seed

`.vscode/🧩️launch.seed.jsonc`:

- the four `🛠️dev🎛️dashboard…` entries were **not** repointed — the `semio` binary still lives in the
  CLI crate, so `run`/`daemon`/`workflow` stay on `⌨️cli/📦️packages/🦀️rust/📜️script.ts`;
- added `🛠️dev🎛️dashboard🌳️command-tree` (`3_dev`, order 1.4) →
  `…⌨️cli/…/📜️script.ts run command-tree --dump-tree`;
- added `🧪️test🧰️repo🎛️dashboard🦀️rust` → `bun nx run @semio-tech/repo-dashboard-rs:test` and
  `🧪️test🧰️repo🎛️dashboard🥒️parity`, at the end of the `🧪️test🧰️repo…` group.

Regenerated with the plugin-registry script (output in §2).

## 2. Verification — real output

### 2.1 Baseline before the move

```
$ cargo test -p semio-framework-repo-cli
running 23 tests
…
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### 2.2 After the move

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-dashboard -p semio-framework-repo-cli
running 28 tests
test command_tree::tests::discover_carries_the_repo_domain_branches ... ok
test playground_catalog::tests::table_text_preserves_catalog_order_and_row_wire_format ... ok
test command_tree::tests::repo_action_carries_its_go_argv ... ok
test playground_session::tests::leaves_unknown_catalog_result_unresolved ... ok
test playground_session::tests::resolves_longest_multi_word_alias ... ok
test daemon::tests::unknown_subcommand_returns_usage_without_side_effects ... ok
test command_tree::tests::tree_json_states_every_leaf_kind ... ok
test tests::args_split_verb_segments_and_flags ... ok
test tests::env_contract_sets_locks_only_for_individual ... ok
test command_tree::tests::go_implementation_projects_process_leaves_at_the_go_binary ... ok
test tests::ipc_frame_roundtrip_and_output_codec ... ok
test command_tree::tests::segment_key_strips_emoji_prefix ... ok
test root_delegation::tests::preserves_root_script_verb_and_positional_segments ... ok
test tests::daemon_supervisor_ping_appends_event_log ... ok
test plugin_registry::tests::check_reports_missing_generated_files ... ok
test tests::ipc_nonblocking_decoder_preserves_fragmented_and_concatenated_frames ... ok
test tests::ipc_nonblocking_decoder_rejects_oversized_prefix_without_allocating ... ok
test command_tree::tests::discover_builds_verb_first_level ... ok
test tests::parse_lock_all_is_case_insensitive ... ok
test usage::tests::usage_reference_preserves_every_registered_command ... ok
test workflow::tests::platform_probe_command_matches_host ... ok
test workflow::tests::scheduler_respects_scope_and_dependencies ... ok
test workflow::tests::unavailable_runners_are_filtered ... ok
test plugin_registry::tests::check_reports_invalid_json_and_passes_when_valid ... ok
test tests::resolve_port_prefers_explicit_then_catalog_then_fallback ... ok
test command_tree::tests::repo_actions_run_in_process_against_the_domain_crates ... ok
test tests::playgrounds_json_text_falls_back_to_empty_array_when_missing ... ok
test tests::playgrounds_json_text_passes_generated_content_through_verbatim ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

All 23 original tests survived the move (the two `command-tree`/`usage` ones were adjusted only
where the code they assert on moved: the usage string gained the `command-tree` line and its
literal-matching test was updated with it). The five new tests are
`discover_carries_the_repo_domain_branches`, `repo_action_carries_its_go_argv`,
`tree_json_states_every_leaf_kind`, `repo_actions_run_in_process_against_the_domain_crates` and
`go_implementation_projects_process_leaves_at_the_go_binary`.

`repo_actions_run_in_process_against_the_domain_crates` is the runtime proof that the in-process
path really calls the domain crates: it runs `StatutesCatalog`, `TreeStatute`, `TreeTerritory`,
`GoalsList` and `TicketShow` against an empty temp root and asserts on what they answered.

### 2.3 Headless dashboard run on this Windows host (`--dump-tree`)

```
$ cargo build -p semio-framework-repo-cli --bin semio      # succeeded
$ ./target/debug/semio.exe command-tree --dump-tree > dump-tree.json
exit=0
7878570 bytes
root verbs: ['dev', 'build', 'test', 'verify', 'lint', 'format', 'generate', 'publish',
             'tickets', 'goals', 'analyze', 'tree', 'statutes', 'add-widget-retained-check', …]
tickets children: 3365 -> ['25/11/17/REFACTOR', '25/11/18/BREADCRUMB-RENDER-ERROR', …]
goals    children: 2 -> ['list', 'tree']
analyze  children: 6 -> ['all', 'go', 'markdown', 'python', 'rust']
tree     children: 4 -> ['goal', 'monorepo', 'statute', 'territory']
statutes children: 1 -> ['catalog']
```

The first ticket branch, verbatim from the dump:

```json
{
 "key": "close", "label": "close",
 "leaf": { "action": "ticket.close:25/11/17/REFACTOR",
           "goArgv": ["ticket", "close", "25/11/17/REFACTOR", "--bulk"],
           "kind": "repo" }
}
```

3365 real tickets were read through `tickets::TicketService::list` over `FileTicketStore`, on
Windows, in one process, with no Go binary involved.

With the Go implementation selected:

```
$ SEMIO_REPO_IMPLEMENTATION=go ./target/debug/semio.exe command-tree --dump-tree
{
 "children": [ { "key": "catalog", "label": "catalog",
   "leaf": { "args": ["statute", "list"],
             "cmd": "C:\\git\\semio\\🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🐹️go\\semio-repo.exe",
             "cwd": "", "env": [], "kind": "process" } } ],
 "key": "statutes", "label": "statutes"
}
```

### 2.4 Protocol v2 harness

```
$ bun "./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts" discover | grep dashboard
test-framework-products-repo-modules-dashboard-f213ad-command-tree-projection	🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🌳️command-tree-projection	[rust]

$ bun "…/📜️script.ts" contract --case "🌳️command-tree-projection"      # no breach names this case
$ bun "…/📜️script.ts" subject fundamental --case "🌳️command-tree-projection" --implementation rust
[test] level=fundamental cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0

$ bun "…/📜️script.ts" parity quick --case "🌳️command-tree-projection"
[test] level=quick cases=1 executed=2 passed=2 failed=0 errored=0 parity=0/0
```

Note for the other executors: `--case` matches the case **directory name including its leading
emoji** (`selectCases` compares `entry.case`, which is `basename(caseDir)`), so
`--case 🌳️command-tree-projection`, not `--case command-tree-projection`. The command sequence in
`📓️harness-verification.md` §1 omits the emoji and therefore selects zero cases.

### 2.5 launch seed regeneration

```
$ bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages) -> …/🤖️generated
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json

$ grep -n "dashboard🌳️command-tree|test🧰️repo🎛️dashboard" .vscode/launch.json
115:      "name": "🛠️dev🎛️dashboard🌳️command-tree",
8315:      "name": "🧪️test🧰️repo🎛️dashboard🦀️rust",
8322:      "name": "🧪️test🧰️repo🎛️dashboard🥒️parity",
```

## 3. Files created, changed and removed

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🔣️.json`
- `…/🎛️dashboard/{🖥️terminal,🌀️daemon,🌳️command-tree,🌊️workflow,🔌️plugin-registry,🛝️playground-session,📇️playground-catalog,📜️root-delegation,⌨️usage}/🦀️.rs`
- `…/🎛️dashboard/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs,📋️project.json,📜️script.ts}`
- `…/🎛️dashboard/🧬️schema/🔣️.json`
- `…/🎛️dashboard/🔮️oracle/🔣️.json`
- `…/🎛️dashboard/🧪️tests/🌳️command-tree-projection/{🥒️.feature,🦀️.rs,🧫️fixtures/🏗️workspace.json}`

Changed:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/{🦀️.rs,Cargo.toml}`
- `Cargo.toml` (workspace member)
- `🧅️layering.json` (the shrink-only ratchet key `🎮️commands/🌳️command-tree-discovery/🦀️.rs` repointed
  to `🔨️modules/🎛️dashboard/🌳️command-tree/🦀️.rs`, allowance unchanged at 1)
- `.vscode/🧩️launch.seed.jsonc` and the regenerated `.vscode/launch.json`

Removed:

- `🧰️framework/🛍️products/🦑️repo/🎮️commands/` (whole tree, plan §2 "Removed at the end")

## 4. What is left

1. **`bun ./📜️script.ts verify layering` cannot run on this host** — it dies before reaching the
   ratchet with an unrelated, pre-existing taxonomy failure:
   `generatorContracts["wgpu-frame-worker"] tracked output "…/🎞️frame-worker/🤖️generated/🟨️.js" is
   missing`. The ratchet key was therefore repointed rather than recomputed. The audit wave should
   re-run it once that generator output exists; the `catalog` region's
   `🧰️framework/🛍️products/💻️os/…/📇️registry/🤖️generated` literal now sits in the dashboard godfile
   rather than in the CLI godfile, so a new baseline row may be needed there.
2. **The daemon still listens only on unix.** `supervisor::serve` binds a `UnixListener`; on Windows
   it returns `dashboard daemon listen is unix-only in this build`, so `semio daemon start|attach`
   is unix-only even though the sessions it supervises are now ConPTY-capable. `ipc::pipe_name` and
   `ipc::connect` already exist for Windows; only the listener side is missing.
3. **No Go twin of the dashboard**, by design (plan §2: "Rust only (TUI)"), so the case carries a
   Rust subject alone and rests on a recorded no-oracle decision. When a second implementation of
   the discovery walk exists, the case becomes differential without changing the fixture.
4. **The Go leaves are unverified end to end** — `SEMIO_REPO_IMPLEMENTATION=go` projects the correct
   argv (§2.3), but `⌨️cli/📦️packages/🐹️go/semio-repo` does not exist yet in this checkout, so no
   Go leaf was actually spawned. The argv vocabulary (`ticket read|files|close|reopen`,
   `goal list|tree`, `analyze --scope`, `tree monorepo|goal|statute|territory`, `statute list`)
   must be confirmed against the Go CLI's real verb table by the `cli`/`go-split` executors.
5. **`tree/monorepo` walks the whole repository** (`CodebaseContext::load_files`) and is therefore
   slow on this monorepo; it is correct but wants progress/cancellation before it is comfortable in
   a wizard. `analyze/all` has the same shape.
6. The concurrent `cli` agent's new regions in the CLI godfile were mid-edit and briefly did not
   compile during this session; the final combined `cargo test -p … -p …` above was taken after they
   compiled again. Nothing outside the top-of-file module block and the `run()` arms was touched by
   this agent.
