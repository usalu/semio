# W0 — Decouple the `semio` Dashboard Binary From the Repo CLI / MCP Crate

Status: done. `semio-framework-repo-cli` no longer depends on `semio-framework-repo-dashboard`;
the dashboard crate owns the `semio` binary, its dispatch, its Nx targets, its installation module
and its launch tests. Not run by this slice (by instruction): `install`, daemon start/stop,
`--release` build.

## What moved where

| Concern | Before | After |
| --- | --- | --- |
| `semio` `[[bin]]` | `⌨️cli/📦️packages/🦀️rust/Cargo.toml` → `⌨️cli/🚪️entrypoint/🦀️.rs` | `🎛️dashboard/📦️packages/🦀️rust/Cargo.toml` → `🎛️dashboard/🚪️entrypoint/🦀️.rs` |
| Verb dispatch `pub fn run(argv)` | first region of `⌨️cli/🦀️.rs` | `// #region 🔖️Dispatch` of `🎛️dashboard/📦️packages/🦀️rust/🦀️.rs` (`semio_framework_repo_dashboard::run`) |
| Root discovery for the dispatch | cli dep `semio-framework-repo-workspace` | new dashboard dep `semio-framework-repo-workspace` (dashboard → repo domain crate, never the reverse) |
| Installation record / native launch / `dashboardInvocation` | `⌨️cli/📦️installation/🟦️.ts` | `🎛️dashboard/📦️installation/🟦️.ts`, recognises `@semio-tech/repo-dashboard-rs:<run\|daemon\|workflow\|preferences>` |
| Script verbs `build` (release + install), `install`, `preferences`, `run`, `daemon`, `workflow`, `test execution`, `dashboardExecutable` | `⌨️cli/📦️packages/🦀️rust/📜️script.ts` | `🎛️dashboard/📦️packages/🦀️rust/📜️script.ts` |
| Nx targets `run`, `daemon`, `workflow`, `install` (dependsOn `build`), `preferences`, cached `build` with `{projectRoot}/dist/build` | `@semio-tech/repo-cli-rs` | `@semio-tech/repo-dashboard-rs` |
| Launch/installation test + vectors | `⌨️cli/🧪️tests/🧊️execution/{🟦️.ts,🥒️.feature}`, `⌨️cli/🧫️fixtures/🧊️execution/🔣️.json` | `🎛️dashboard/🧪️tests/🧊️execution/…`, `🎛️dashboard/🧫️fixtures/🧊️execution/🔣️.json` |
| Root scripts `dashboard`, `dashboard:start\|status\|stop\|install\|preferences` | `@semio-tech/repo-cli-rs:*` | `@semio-tech/repo-dashboard-rs:*` |

What the cli package keeps: crate `semio-framework-repo-cli` with bins `repo` (MCP stdio server) and
`semio-repo` (repo CLI); script verbs `build` (now `cargo build -p semio-framework-repo-cli`),
`test`, `repo`, `mcp`; Nx targets `build`, `test*`, `repo`, `mcp`.

Dispatch behaviour is identical. The one structural change: the "bare or flag-first argv is the
dashboard" rule became the private pure function `invocation(argv) -> ParsedArgs` so it can be
unit-tested (`argv.first().is_none_or(starts_with("--"))` ≡ the old
`is_empty() || first().is_some_and(…)`; a `dashboard` verb reaches the same `terminal::run_with`).
The `plugin registry` guard and the `root_delegation` fallback are verbatim. `root_delegation`
needs only `args::ParsedArgs` + `proc::spawn_inherit`, `plugin_registry::run` only
`catalog::generated_dir` + `proc::spawn_inherit` — all already inside the dashboard crate.

Re-verified: after removing lines 1–30 of `⌨️cli/🦀️.rs` no `semio_framework_repo_dashboard`
identifier remains in the cli crate (the removed `use std::path::PathBuf` was used only by the
dispatch; `mcp_verb` has its own import).

Decisions:
- Dashboard crate `[package.metadata.semio] role`: `library` → `tool` (it now ships a binary, as
  the cli crate does). `cargoDependencyDirections.ownerRoles` in `🔣️taxonomy.json` admits `tool`
  under `^🧰️framework/`; no direction rule involves `tool`.
- The installation cache key `tools/dashboard-cli/<sha256>/semio` is unchanged on purpose: the
  record of the currently installed executable matches that pattern and must stay valid.
- `SEMIO_TEST_CLI` (env var of four ignored daemon tests) keeps its name: it names the `semio`
  executable, not the cli crate, and the file is under concurrent edit by sibling slices.
- No `test-execution` Nx target was added (there was none before); the case runs through
  `bun ./📜️script.ts test execution` as before.

## Files

Created
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🚪️entrypoint/🦀️.rs`

Moved (plain `mv`, content preserved incl. peers' uncommitted edits)
- `⌨️cli/📦️installation/🟦️.ts` → `🎛️dashboard/📦️installation/🟦️.ts` (prefix → `@semio-tech/repo-dashboard-rs:`)
- `⌨️cli/🧪️tests/🧊️execution/🟦️.ts` → `🎛️dashboard/🧪️tests/🧊️execution/🟦️.ts` (synthetic graph root `cli` → `dashboard`)
- `⌨️cli/🧪️tests/🧊️execution/🥒️.feature` → `🎛️dashboard/🧪️tests/🧊️execution/🥒️.feature`
- `⌨️cli/🧫️fixtures/🧊️execution/🔣️.json` → `🎛️dashboard/🧫️fixtures/🧊️execution/🔣️.json` (7 vectors renamed)

Removed
- `⌨️cli/🚪️entrypoint/🦀️.rs` (+ empty folders `⌨️cli/🚪️entrypoint`, `⌨️cli/📦️installation`, `⌨️cli/🧪️tests/🧊️execution`, `⌨️cli/🧫️fixtures/🧊️execution`)

Updated (all under `🧰️framework/🛍️products/🦑️repo/🔨️modules/` unless rooted)
- `⌨️cli/🦀️.rs` — dispatch region + imports removed, header rewritten
- `⌨️cli/📦️packages/🦀️rust/Cargo.toml` — `[[bin]] semio` and the dashboard dependency removed, description
- `⌨️cli/📦️packages/🦀️rust/📜️script.ts` — only `build|test|mcp|repo`
- `⌨️cli/📦️packages/🦀️rust/📋️project.json` — five targets removed, `build` uncached
- `⌨️cli/README.md`
- `🎛️dashboard/📦️packages/🦀️rust/Cargo.toml` — `[[bin]] semio`, dep `semio-framework-repo-workspace`, role, description
- `🎛️dashboard/📦️packages/🦀️rust/🦀️.rs` — `🔖️Dispatch` region, header
- `🎛️dashboard/📦️packages/🦀️rust/📜️script.ts` — `build|install|preferences|test|run|daemon|workflow`, exports `dashboardExecutable`
- `🎛️dashboard/📦️packages/🦀️rust/📋️project.json` — five targets added, cached `build`
- `🎛️dashboard/🧪️tests/🔬️unit/🦀️.rs` — new `dispatch_reads_bare_and_flag_first_invocations_as_the_dashboard`
- `🎛️dashboard/README.md`
- `📚️library/⚡️caching/🚀️bootstrap/📜️script.ts` — dynamic import → `../../../🎛️dashboard/📦️installation/🟦️.ts`
- `📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts` — reads the dashboard `📋️project.json` for `SEMIO_NATIVE_OWNER_PROGRESS`
- `📚️library/🧫️fixtures/🧱️cargo-transaction-command-source/🔣️.json` — native-build consumer: cli script → dashboard script (still 18 consumers)
- `📚️library/📦️packages/🟦️typescript/📋️project.json` — named input `cargoTransactionCommandSources`: same swap
- `📚️library/🧫️fixtures/🦑️repo-source-ownership/🔣️.json` — cargo-bin owner → `🎛️dashboard/🚪️entrypoint/🦀️.rs`, anchor `semio_framework_repo_dashboard::run(`
- `📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts` — entrypoint path asserted on the dashboard manifest
- `package.json` (root) — six `dashboard*` scripts
- `Cargo.lock` — written by cargo: cli loses `semio-framework-repo-dashboard`, dashboard gains `semio-framework-repo-workspace`

Not touched, still naming `@semio-tech/repo-cli-rs:<run|daemon|install|…>`: `.vscode/launch.json`,
`.vscode/🧩️launch.seed.jsonc` (another slice deletes them). Until then those entries point at
targets that no longer exist. A repo-wide `git grep --untracked` for `repo-cli-rs`,
`⌨️cli/📦️installation`, `⌨️cli/🚪️entrypoint`, `semio_framework_repo_cli::run(` and the old
execution paths finds nothing else (the cli `📋️project.json` name itself excepted). The root
`📜️script.ts` never referenced the dashboard binary (its `--bin semio` is `semio-framework-os-kernel`).

## Verification (commands and real output)

Baseline before editing: `cargo check -p semio-framework-repo-cli --message-format=short` →
`Finished … in 12.73s`, and it checked `semio-framework-ui`, `semio-framework-repo-dashboard`.

After the move:

```
$ cargo check -p semio-framework-repo-cli --message-format=short
    Checking semio-framework-repo-cli v0.1.0 (…/⌨️cli/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1.46s          (EXIT=0; last rerun 0.16s)
$ cargo check -p semio-framework-repo-dashboard --message-format=short
    Checking semio-framework-repo-dashboard v0.1.0 (…/🎛️dashboard/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 8.76s          (EXIT=0; last rerun 0.16s)
$ cargo tree -p semio-framework-repo-cli -e normal | /usr/bin/grep -c -E 'semio-framework-repo-dashboard|semio-framework-ui '
0
$ cargo metadata --no-deps   (bins per package)
semio-framework-repo-cli        bins= ['repo', 'semio-repo']
semio-framework-repo-dashboard  bins= ['semio']
```

Moved binary, private target dir `T=.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-decouple`:

```
$ CARGO_TARGET_DIR=$T cargo build -p semio-framework-repo-dashboard --bin semio
   Compiling semio-framework-repo-dashboard v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 21.06s         (EXIT=0)
$ $T/debug/semio --help
semio [dashboard] [--root PATH] [--config JOURNAL] [--language en|de] [--appearance dark|light] [--terminology native|reuse] [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows]
Ctrl+B: n commands, h tasks, p settings, f refresh, e cancel discovery, r restart, c cancel, k kill, d detach, Q shutdown
exit=0
$ $T/debug/semio command-tree --root <🗑️generated/decouple/tiny-root>
semio
  goals
    list
    tree
  analyze
  …                                                                    exit=0
```

Every side-effect-free arm, run with cwd = a tiny root holding `.git` and a one-line `📜️script.ts`
that prints its argv:

```
semio verify taxonomy report   → [DEBUG] root script argv ["verify","taxonomy","report"]   exit=0   (root_delegation)
semio plugin registry check    → plugin registry catalog is missing: generated/🔌️plugins.json …  exit=1   (plugin registry arm)
semio plugin list              → [DEBUG] root script argv ["plugin","list"]                 exit=0   (guard falls through)
semio catalog --json           → []                                                         exit=0
semio daemon bogus             → usage: semio daemon start|stop|status|attach|serve         exit=1
semio preferences show --config <tmp> → {"language":"en","appearance":"dark","terminology":"native","renderer":"react","layout":"tabs"}  exit=0
semio dashboard --help         → semio [dashboard] [--root PATH] …                          exit=0
```
Not exercised: `repo-view`, `workflow`, `dev` (they start work).

Rust tests (same private target dir, `SEMIO_TEST_PATH=$PATH`, artifact dir in the ticket):

```
$ cargo test -p semio-framework-repo-dashboard --lib
running 59 tests
test tests::dispatch_reads_bare_and_flag_first_invocations_as_the_dashboard ... ok
test result: ok. 55 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 3.28s
$ cargo test -p semio-framework-repo-cli --lib
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
$ cargo test -p semio-framework-repo-cli -p semio-framework-repo-dashboard --no-run
  Executable unittests ../../🦀️.rs (…semio_framework_repo_cli-…)
  Executable unittests 📦️mcp-main.rs (…repo-…)
  Executable unittests 📦️cli-main.rs (…semio_repo-…)
  Executable unittests 🦀️.rs (…semio_framework_repo_dashboard-…)
  Executable unittests ../../🚪️entrypoint/🦀️.rs (…semio-…)             EXIT=0
$ CARGO_TARGET_DIR=$T cargo build -p semio-framework-repo-cli --bin semio-repo --bin repo
    Finished `dev` profile …  (0 lines "Compiling semio-framework-ui|repo-dashboard"); `semio-repo --help` and `semio-repo mcp --dry-run` exit 0
```
The cli lib has no unit tests; its eight `⌨️cli/🧪️tests/*/🦀️.rs` adapters use only `repo_cli` and
run through the `🧪️test` harness (not run here, not touched).

The four ignored `SEMIO_TEST_CLI` tests, run once with `SEMIO_TEST_CLI=$T/debug/semio … -- --ignored --test-threads=1`:
`3 passed; 1 failed`.
- passed: `actual_cli_daemon_releases_parent_pipes_before_shutdown`,
  `actual_cli_first_frame_is_fast_and_settings_survive_reopening`,
  `actual_cli_launcher_receives_keys_and_runs_a_launch_configuration`.
- failed: `actual_cli_view_auto_starts_daemon_and_detaches_without_stopping_tasks` —
  `🌀️daemon/🧪️tests/🔬️unit/🦀️.rs:210: native daemon connection: No such file or directory (os error 2)`.
  Pre-existing and unrelated to the move: the identical panic at the identical line occurs with
  `SEMIO_TEST_CLI` = the installed executable built from the pre-move cli crate
  (`tools/dashboard-cli/f708544c…/semio`): `test result: FAILED. 0 passed; 1 failed; … 58 filtered out`.
  Cause not investigated; no code changed for it.

TypeScript:

```
$ bun 🎛️dashboard/📦️packages/🦀️rust/📜️script.ts
error: usage: bun ./📜️script.ts <build|install|preferences|test|run|daemon|workflow> [args…]
$ bun ⌨️cli/📦️packages/🦀️rust/📜️script.ts
error: usage: bun ./📜️script.ts <build|test|mcp|repo> [args…]
$ bun ⌨️cli/📦️packages/🦀️rust/📜️script.ts daemon status
error: unknown command "daemon"; usage: bun ./📜️script.ts <build|test|mcp|repo> [args…]
$ SEMIO_TEST_ARTIFACT_DIR=<ticket> bun 🎛️dashboard/📦️packages/🦀️rust/📜️script.ts test execution
 2 pass  0 fail  15 expect() calls          (launch vectors, Nx createTaskGraph: run → [run], install → [build, install], immutable installation)
$ bun test ./…/📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts
 7 pass  0 fail  219 expect() calls
$ bun test ./…/📋️native-orchestration/🧪️tests/📋️owner-cmd-policy/🟦️.ts
 7 pass  0 fail                              (66.79s)
$ bun test ./…/📚️library/🧪️tests/🧱️cargo-transaction-command-source/🟦️.ts
 6 pass  3 fail
```
The three `cargo-transaction-command-source` failures pre-exist and do not involve this slice's rows:
1. `resolves every anonymous owner and semantic context` — taxonomy returns `null` for
   `🎛️owned-execution` (taxonomy file untouched by me).
2. `moves behavior out of command modules and rebinds every live API consumer` — stops at the
   first consumer, `🌎️hub/📦️packages/🦀️rust/📜️script.ts: buildRepositoryCargoArtifacts`: the
   fixture's `native-build` owner does not declare `buildRepositoryCargoArtifacts` (checked: `false`),
   which fails every consumer of that function, the old cli row included.
3. `registers exact Bun, Nx, cache, package, and launch closure` — `🧹️normalization/🟦️.ts` is
   missing from the named input.
For the swapped row itself, an inline check printed: dashboard script `listed: true | imports
native-build: true`, cli script `listed: false | imports native-build: false`, `consumers: 18`.

Wiring proof for the root scripts. The moved targets are not in the cached Nx graph, so no graph
run was made; instead:
- the real front door, which never builds a graph for these targets:
  ```
  $ bun nx run @semio-tech/repo-dashboard-rs:run --outputStyle=stream -- --help
  $ bun ./…/🚀️bootstrap/📜️script.ts nx run @semio-tech/repo-dashboard-rs:run "--outputStyle=stream" -- --help
  semio [dashboard] [--root PATH] …          real 0.21   exit=0     (installed executable, launched by the bootstrap wrapper)
  ```
- `dashboardInvocation`: `{"old":null,"run":[],"daemon":["daemon","status"],"workflow":["workflow"],"preferences":["preferences","show"],"install":null,"installed":true}` (`old` = `@semio-tech/repo-cli-rs:run`).
- graph inputs read directly: `🎛️dashboard/📦️packages/🦀️rust/📋️project.json` now carries
  `build, test, test-quick, test-long, test-exhaustive, run, daemon, workflow, install, preferences`;
  the cli one `build, test, test-quick, test-long, test-exhaustive, repo, mcp`; nx's own
  `createTaskGraph` over the dashboard manifest is asserted by the execution test above.

WRITTEN BUT UNVERIFIED
- `bun nx run @semio-tech/repo-dashboard-rs:install` / `:build` through a freshly constructed Nx
  graph (would start the 20+ min release build and reinstall).
- `BuildScript`, `InstallScript`, `RunScript`, `DaemonScript`, `WorkflowScript`, `PreferencesScript`
  of the dashboard script as executed code paths (moved verbatim; only routing, `test execution` and
  `dashboardExecutable` on an installed record were executed).
- The cli script's new `build` verb (`cargo build -p semio-framework-repo-cli` in the shared target dir) and its `repo`/`mcp` verbs were not executed; the same cargo invocation was proven in the private target dir.
- `dashboard:start|status|stop|preferences` root scripts (same recognition path as `dashboard`, proven only by `dashboardInvocation` vectors).
- Windows / Linux / devcontainer runs.

## Things the coordinator should know

- Contact with the developer's running dashboard. The ignored test
  `actual_cli_first_frame_is_fast_and_settings_survive_reopening` has a fifth trial that runs
  `bun run dashboard -- --root <actual workspace> --config <temp journal>`; I ran the ignored set
  before reading that. It attached one extra view (the installed executable, via the renamed root
  script) to the real workspace daemon, opened settings, cancelled discovery and detached. Checked
  afterwards: daemon pid 10248 alive with its original start time (15:32:10), `daemon.pid`,
  `installed.json`, `events.jsonl`, `commands.json`, `manifests.json` mtimes unchanged, no
  preferences journal written in the workspace, no leftover process. Nothing was stopped, started
  or reinstalled. The second run (single failing test against the installed executable) used
  fixture roots only. Side benefit: trials 3 and 4 show `bun run dashboard` reaching the installed
  executable through `@semio-tech/repo-dashboard-rs:run` with a first frame of 234 / 218 ms.
- Stale ignored build output left in place: `⌨️cli/📦️packages/🦀️rust/dist/build/semio` (3.5 MB,
  gitignored, the staging copy of the currently installed executable). Nothing reads it any more;
  the next release build writes `🎛️dashboard/📦️packages/🦀️rust/dist/build/semio`.
- The private target dir `.🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-decouple` (46 MB: debug
  `semio`, `semio-repo`, `repo`) is left for re-verification; delete when done.
- A dashboard that is already running may still list launcher entries naming
  `@semio-tech/repo-cli-rs:*` from its cached command catalog until discovery is refreshed (`Ctrl+B f`).
- `🧰️…/⌨️cli/📦️packages/🐹️go/🔬️_test.go` expects `cargo build --release -p semio-framework-repo-cli`
  in `.devcontainer/post-create.sh` and the native bootstrap scripts; `post-create.sh` does not
  exist and `🔩️native/🥾️bootstrap/{🐚️.sh,🔵️.ps1}` do not contain that string today (pre-existing
  drift of the repo-client bootstrap, not the dashboard) — untouched, test not run.
- Scratch output under `🗑️generated/decouple/` was removed after the runs; all quoted lines above
  are copied from it.
