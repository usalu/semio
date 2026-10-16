# Result W4 REPO-DASHBOARD: `semio-framework-repo-dashboard` carved into eleven producer crates plus a thin facade

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command. Rules obeyed: `AGENTS.md`, `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `PRODUCER-CRATE-RECIPE.md`, `📓️WAVE4-BRIEF.md`.
`D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`. Tools: `🪓️monolith-split/dashboard-carve-tools/` (`carve_a.py` to `carve_e.py`, `register.py`, `manifests.py`, `spec.py`, `union.py`, `deps_check.py`, `standalone.py`, `defscan.py`, `verify.ts`, `census_all.ts`, `lossless.py`, `fmt_check.py`, `unused_imports.py`). Logs: `🗑️generated/dash-*.log`.

## 0. Status in one paragraph

The split is done and statically verified: every resulting crate is inside the gate and inside the wave-4 target (3500 lines / 28 files / 230000 bytes) for its lib and its cfg-test closure, the dependency graph is a DAG, and a union differential (the twelve crates simulated as modules of one crate, compared with the original monolith) shows no name-resolution or module-privacy regression, natively and under `--test`. **No cargo check and no unit test has been run**: `bun repo cargo ...` is refused before any compile (`Finite original maintenance authority required`) and the Nx route is refused too (`Active graph readers retain required Nx pool headroom`), both at the start and at the end. Beyond that, the crates also cannot compile for real yet because three of their dependencies are peer-owned oversized `ui` crates (`oversizedDeps: 3` in the census).

## 1. What the investigation found

- The crate is a binary (`semio`, entry `🚪️entrypoint`) plus a lib that mounted 11 module folders with `#[path]`. Only the binary consumes the lib (`run`); `Cargo.lock` lists no other dependent, so no external Rust consumer needed an edit.
- The modules were cyclic. Four cycles had to be cut, each by moving the smallest cohesive piece to the lower side:
  1. `registry` <-> `daemon::ipc`: the five launch types (`Stop`, `Launch`, `LaunchProcess` from the registry, `TaskLabel`, `Ready` from the wire) moved to the new contract crate `launch`.
  2. `registry` -> `daemon::ipc::stable_hash` -> `registry::xxh3_64` and `env_contract` -> `registry::RUNNER_ENV`: the Hash region and `RUNNER_ENV` moved to `core`.
  3. `daemon::journal` <-> `supervisor` (`WorkspaceLease`) and `daemon::control` <-> `supervisor` (`spawn_daemon`, `status`): the supervisor file was split. The engine stays; the lease and pid files became `🌀️daemon/🔐️lease`; `serve`, `spawn_daemon`, `start_detached`, `status`, `stop` and `connect_or_start` (moved from `control`) became `🌀️daemon/🚦️lifecycle`.
  4. `daemon` <-> `terminal` (`daemon::run` attaches the terminal; `terminal` starts the daemon; the labels are needed below both): the labels became their own crate below the daemon, and the `semio daemon` verb handler became `🌀️daemon/⌨️command`, mounted by the facade.
- Two orphan-rule traps would have been invisible to every static oracle: `impl Outputs for BTreeMap<String, Live>` (trait in `clients`, `Live` in `supervisor`), so `clients` and `groups` live with the supervisor in the daemon crate, not in the wire crate. No other cross-crate trait impl exists (script scan of all `impl .. for`).
- The registry tests build their registry through `inventory::discover_with_catalog`, and the inventory depends on the registry. Moving the registry tests into the inventory crate (they use only public registry API, no private field or method) removed the dev-dependency cycle and brought the registry crate and the inventory crate far below the byte target.
- Source-text readers: `🧪️tests/🎮️registry/🟦️.ts` read the XXH3 vectors from `🎮️registry/🧪️tests/🔬️unit/🦀️.rs`; the hash and its test moved to `🧱️core`, the reader now reads `🧱️core/🧪️tests/🔬️unit/🦀️.rs` (test passes). Every other TS reader (`⌨️controls`, `🎮️registry` graph checks, root `📜️script.ts` RENDERERS reader, `repo-source-ownership`) names a file that did not move.
- `module_path!()` in the daemon integration harness (`--exact <path>::child`) stays valid: the module is now `integration` of the daemon crate root, and the harness strips the crate name.
- All `#[cfg(test)]` items are crate-local; the only ones in the tree (`terminal/📡️sessions`) are used by terminal tests only, so no test of an upper crate depends on a `cfg(test)` item of a lower crate.

## 2. Resulting layout

Module files stay where they are and are mounted with `#[path]` from the lib root of the crate that owns them (each file has exactly one owner; `union.py` asserts it). Dependency keys use the repo's existing alias style (`dashboard_core = { path = .., package = "semio-framework-repo-dashboard-core" }`).

| Crate (`semio-framework-repo-dashboard-…`) | Lib root (`D/…/🦀️.rs`) | Modules | Depends on (dev) |
|---|---|---|---|
| `core` | `🧱️core` (new) | `args`, `catalog`, `options`, `env_contract` (+ `RUNNER_ENV`), content hash (`xxh3_64`), `testkit` feature (`assert_proved`) | serde_json |
| `launch` | `🧾️launch` (new) | `Stop`, `TaskLabel`, `Ready`, `LaunchProcess`, `Launch` | serde |
| `repo-domain` | `🏛️repo-domain` | repo actions | core, repo-* crates (core+testkit) |
| `registry` | `🎮️registry` | commands, declarations, resolution | core, launch, repo-domain, ui locale and tui |
| `inventory` | `📚️inventory` | inventory, `🌳️command-tree`, the laws of the registry (`🎮️registry/🧪️tests`) | core, registry, repo-domain, repo-tickets (launch) |
| `wire` | `🌀️daemon/🧵️wire` (new) | `✉️ipc`, `🟢️ready`+`📡️http`, `📼️replay`, `🚚️transport`, `📜️journal`, `🔐️lease` (new), `📎️connection` as `client` | core, launch, ui tui |
| `labels` | `🖥️terminal/🌐️labels` | every string + tab texts and window titles (moved from the registry) | launch, wire, ui locale and tui (core+testkit, preferences) |
| `preferences` | `⚙️preferences` | preferences, keymap | core, registry, ui locale (core+testkit) |
| `daemon` | `🌀️daemon` | `👥️clients`, `🧩️groups`, `🧠️supervisor`, `🕹️control`, `🚦️lifecycle` (new), the daemon unit and integration tests | labels, wire, ui tui (core+testkit, launch, serde_json) |
| `terminal` | `🖥️terminal` | launcher, windows, panes, sessions, controls | core, daemon, inventory, labels, launch, preferences, registry, wire, ui (core+testkit, serde_json) |
| `cli` | `🧭️cli` | the verbs `commands`, `run`, `tasks`, `logs`, `stop`, `restart`, `kill`, `open` | daemon, inventory, registry, repo-domain, wire (core+testkit, launch) |
| `semio-framework-repo-dashboard` (facade, lib + bin `semio`) | `📦️packages/🦀️rust/🦀️.rs` | dispatch, `proc`, `⌨️usage`, `📇️playground-catalog`, `🔌️plugin-registry`, `🌀️daemon/⌨️command` (new, mounted as `daemon`) | cli, core, daemon, inventory, labels, preferences, repo-domain, terminal, wire, repo-workspace |

DAG (verified by `verify.ts`, normal and dev edges, no cycle):

```
core   launch
 |  \    |
 |   repo-domain      wire(core, launch)
 |   |                |
 +--registry          labels(launch, wire; dev preferences)
 |   |    \             |
 |   |     preferences  daemon(labels, wire)
 |   inventory(core, registry, repo-domain)
 +----+---------+----------+
                terminal(core, daemon, inventory, labels, launch, preferences, registry, wire)
                cli(daemon, inventory, registry, repo-domain, wire)
                facade(+ every crate it dispatches to)
```

## 3. Census (canonical `readRustSourceClosureV1`, host cfg; limit 4000 lines / 32 files / 262,144 bytes; target 3500 / 28 / 230,000)

Before (`ONLY=semio-framework-repo-dashboard bun census.ts`): lib 35 files / 619,196 bytes / 10,096 lines (over, binding = lines, ratio 2.524, 4 embedded files, 10,306 bytes). Test selection (cfg `test`): 80 files / 1,004,299 bytes / 14,029 lines (33 embedded, 141,501 bytes).

After (`census_all.ts`, log `🗑️generated/dash-closure-after.log`; the test column is cfg `test`, plus `feature="testkit"` for core):

| Crate | lib: files / bytes / lines | test selection: files / bytes / lines |
|---|---|---|
| facade | 5 / 14,185 / 255 | 11 / 80,587 / 417 |
| core | 1 / 18,627 / 312 | 3 / 23,358 / 413 |
| launch | 1 / 2,583 / 65 | 1 / 2,583 / 65 |
| repo-domain | 1 / 21,363 / 465 | 4 / 61,147 / 513 |
| registry | 5 / 115,315 / 1,401 | 5 / 115,315 / 1,401 (no unit tests of its own) |
| inventory | 2 / 39,801 / 596 | 9 / 118,673 / 1,380 |
| wire | 9 / 121,495 / 2,121 | 9 / 121,495 / 2,121 |
| preferences | 3 / 34,478 / 533 | 14 / 154,187 / 815 |
| labels | 1 / 21,576 / 289 | 4 / 83,126 / 367 |
| daemon | 6 / 76,645 / 1,527 | 19 / 206,199 / 2,839 |
| terminal | 6 / 128,443 / 2,153 | 15 / 216,536 / 3,191 |
| cli | 1 / 28,769 / 457 | 3 / 42,444 / 582 |

All twelve rows are `ok` for both selections, below the target, and the largest are terminal tests (3,191 lines, 216,536 bytes) and wire (2,121 lines, 121,495 bytes). The census tool (`🗑️generated/dash-census-after.log`) reports `overLimit false` for all twelve; `closureCompilable` is true for core, launch and repo-domain and false for the others only because each of them reaches `oversizedDeps: 3`, the peer-owned `ui` crates (not mine). Bin target `semio`: 1 file, 5 lines.

Embedded assets: no embedded file is large (the four of the lib closure are the keymap defaults, the publication declaration, the cache-partition policy and the native image schema, 10,306 bytes in total). They stayed `include_str!` leaves of `preferences` and `registry`; moving them to a runtime channel would add a runtime failure mode for 10 KB and was not needed for any gate.

## 4. Changes beyond wiring (all compile-checked at resolution level only)

1. Visibility: `catalog::configured_catalog_root` `pub(crate)` -> `pub` (used by `inventory` and `plugin_registry`), `ready::http` `pub(super) mod` -> `pub mod` and `http::probe` `pub(crate)` -> `pub` (used by the supervisor in another crate), `testkit::assert_proved` `pub(crate)` -> `pub`. No other item, field or method needed widening (definition scan below).
2. `registry`: `LABEL_CELLS` is now `u16` and the ticket title is cut with `elide_end(.., LABEL_CELLS)` (the removed `cap` helper only converted `usize` to `u16`); the Labels region left for the labels crate; `Stop`, `Launch`, `LaunchProcess`, the Hash region and `RUNNER_ENV` left; the Wire region became a private `use dashboard_launch::{..}` (no re-export, consumers import from the launch crate).
3. `ipc` no longer defines `TaskLabel` and `Ready`; it imports them from the launch crate. `stable_hash` calls `dashboard_core::xxh3_64`.
4. `supervisor` keeps the engine only (lines up to `impl Supervisor`); lease/pid and lifecycle moved verbatim (only `super::client::X` -> `X` imports in the lifecycle file).
5. Tests: the unit-test file of `repo_domain` lost the registry-grammar test (moved to the registry tests, now in the inventory crate; the repo-domain proof lists both files as sources); the registry tests lost the XXH3 test (to `core` tests) and the tab-text test (to the labels tests); the daemon unit tests lost `unknown_subcommand_returns_usage_without_side_effects` (to the facade tests, calling `crate::daemon::run`); the facade test file was split, `assert_proved` became the `core` testkit (`#[cfg(any(test, feature = "testkit"))] pub mod testkit`), and its dependants enable it as a dev-dependency feature. The daemon unit tests keep testing the wire (replay, journal, protocol vectors) through its public API from the daemon crate.
6. `📜️script.ts` of the facade: `test` now runs `cargo test` for all twelve packages bottom up (`const crates = [..., crate]`). `README.md`: the packages paragraph describes the crates.

## 5. Registrations (recipe steps)

- Manifests (`manifests.py`, from `spec.py`): `D/<node>/📦️packages/🦀️rust/Cargo.toml` with `package.workspace` resolving to the repo root, explicit `version`, `edition`, `rust-version.workspace`, `role = "tool"` (like the facade), `id = repo-dashboard-<x>`, exactly the three `rust-source-boundary` keys at the ceiling, `[lints] workspace`, `[lib] path = "../../🦀️.rs"`; core carries `testkit = []`. The facade manifest keeps its package, `[lib]`, `[[bin]]` and drops every direct dependency it no longer uses (ui crates, serde, the eight repo domain crates), keeping the eleven it dispatches to plus `serde_json` and `repo-workspace`.
- `📋️project.json` per new crate: `@semio-tech/repo-dashboard-<x>-rs`, tags `scope:framework`, `language:rust` (targets are inferred); the facade project is untouched.
- Root `Cargo.toml` `[workspace] members`: eleven rows inserted at their byte-sorted place (the `dashboard` run is byte-sorted, checked).
- `📋️build-owners.json`: eleven rows (`root` = package dir, `manifest` = its `📋️project.json`, `packages: []`) in name order inside the existing `@semio-tech/repo-dashboard-*` block; names unique.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 66 routes (6 per crate: build, check, test x prototype, release; names, groups `4_build`/`4_gate`/`4_test`, commands as the pets routes), identical in both files, inserted after the `pets-staging` test routes. Orders `900.017xx`, `922.048xx`, `903.588xx`, checked unused in both files (peers were using `900.0150x`, `922.0460x` and `903.5860x` while I worked) and unique after insertion.
- Boundary corpus row (recipe step 11): not applicable, the corpus is the UI cluster. Taxonomy: no registered folder vocabulary covers the dashboard tree (its existing folders such as `🌀️daemon` are not in `🔣️taxonomy.json` either); the six new folders use RGI emoji that already prefix folders in the repo and none repeats among its siblings: `🧱️core`, `🧾️launch`, `🌀️daemon/🧵️wire`, `🌀️daemon/🔐️lease`, `🌀️daemon/🚦️lifecycle`, `🌀️daemon/⌨️command`. Longest path of the tree: 144 bytes.

## 6. Consumers re-pointed

Every `crate::<module>` of the former monolith in the moved files: `crate::args|catalog|options|env_contract` -> `dashboard_core`, `crate::registry` -> `dashboard_registry` (launch types -> `dashboard_launch`, tab and window labels -> `dashboard_labels`), `crate::inventory|command_tree` -> `dashboard_inventory`, `crate::repo_domain` -> `dashboard_repo_domain`, `crate::preferences` -> `dashboard_preferences`, `crate::ipc|daemon::client` -> `dashboard_wire`, `crate::daemon::control|supervisor::spawn_daemon` -> `dashboard_daemon`, `crate::terminal::labels` -> `dashboard_labels`, `crate::terminal::run` -> `dashboard_terminal`. The daemon crate keeps `super::ipc`-style paths through one private `use dashboard_wire::{client, ipc, journal, lease, ready, replay, transport};` at its root, so the supervisor, the control surface and their tests are byte-identical to before apart from the listed edits. The facade dispatch table (`match parsed.verb`) keeps the same arms; the root `pub use daemon::ipc` re-export is gone. Files changed in `D`: 45 `.rs` files (8 new: the `core` root, its unit tests and its testkit, the `launch` root, the `wire` root, `lease`, `lifecycle` and the `⌨️command` verb; 37 edited), 12 `Cargo.toml` (11 new, the facade edited), 11 new `📋️project.json`, `README.md`, `📜️script.ts` and `🧪️tests/🎮️registry/🟦️.ts`; the unrelated peer edits to `🌳️command-tree/🚀️launch/🟦️.ts`, `🧪️tests/🚀️launch/🟦️.ts`, `🧪️tests/🧫️fixtures/🚀️launch/🔣️.json` (and their share of `📜️script.ts`) are not mine. Outside `D`: root `Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`.

## 7. Verification (every command run, with its actual result)

1. Baseline before the first edit: census row above; a pristine copy of `D` in the scratchpad (`git` status of `D` was clean); original errors of the monolith under the patched oracle: native 323, `--test` 440, all of them unresolved external crates (`serde`, `serde_json`, `ui_*`, repo crates) and the `app_labels!` macro output.
2. **Union differential** (`union.py`, `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null`, extern crates and external glob imports neutralised identically in both trees, as in `📓️RESOLUTION-ORACLE.md`; the twelve crates are mounted as modules of one crate, `crate::` rewritten to the owner module, `dashboard_*` crate paths to `crate::dashboard_*`, `pub(crate)` and `pub(super)` to `pub(in <owner>)`, so cross-crate privacy is real): native 325 vs 323 errors, `--test` 442 vs 440. The multiset difference is exactly the moved external references: 4 `cannot find attribute serde` (the derives of the five launch types), `serde` import in the launch crate, 5 `serde_json` (catalog, to core), 2+2 `ui_tui` (lifecycle, labels), 3 `crate::DashboardLabels` unresolved imports (macro output of `app_labels!`, same count and sites as the baseline's `super::labels::DashboardLabels`). **Only-split: no E0412, E0425, E0422, E0423, E0432 or E0603 for an in-repo name; only-baseline: the same moved lines** (`🗑️generated/dash-union-differential.log`).
3. **Sensitivity** (negative control in a scratch copy): `pub mod http` -> `pub(super) mod http`, `configured_catalog_root` -> `pub(crate)`, `verb_rank` -> private: the differential reports `E0603 module http is private`, `E0603 function configured_catalog_root is private` (6 sites) and `E0603 function verb_rank is private` (3 sites), so cross-crate visibility is genuinely covered.
4. **Per-crate dependency completeness** (`deps_check.py`, each crate compiled as its own root, native and `--test`): the set of unresolved external and dependency crate names equals the declared dependencies for all twelve (no missing, no unused, every dev-dependency used by a test): `DIFFS: 0` (`dash-deps-check.log`). **Standalone resolution** (`standalone.py`): every error that does not name a missing crate is a consequence of that missing crate (`!`-typed receivers, `mismatched types`, the `DashboardLabels` macro type), none an intra-crate name (`dash-standalone.log`).
5. **Definition scan** (`defscan.py`): members (methods, fields, associated fns) that are not `pub` in a crate and whose name a depending crate uses: seven hits, all name coincidences with public members or with std/serde methods (`mcp`, `ready`, `bytes`, `command`, `port`, `connect`, `input`, `root`, `session`, `text`). By construction no further widening is needed: every edge between two new crates was an edge between two sibling modules before, which already required `pub`, except the three restricted items listed in section 4, and the daemon tests (which used wire items from the daemon root, also requiring `pub`). Limit of the oracle: field and method privacy (E0616, E0624) lives in typeck, which the missing external crates cut short.
6. Losslessness (`lossless.py`, line multiset of all `.rs` before and after, `use`/`crate::`/`super::`/crate-name prefixes normalised): everything removed or added is an intended edit (mounts, docs, imports, the widened visibilities, the new files). `//#region` markers: the multiset of `#region` and `#endregion` lines is identical before and after (no loss, no gain; the registry `Tests` region left with the registry tests and core gained one).
7. rustfmt (`fmt_check.py`, `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config skip_children=true,unstable_features=true`): all 45 changed or new `.rs` files parse (`checked 45 failed 0`). Import runs of the carved files are sorted (stage E).
8. Static checks (`bun verify.ts`, log `dash-verify.log`): the twelve `Cargo.toml` parse under `Bun.TOML`, package names match, `package.workspace` resolves to the repo root, `[lib] path` and every path dependency resolve to the expected package, boundary keys exact, all twelve in the workspace members once (dashboard members byte-sorted), owner rows and `📋️project.json` names agree and are name-sorted, the 66 route commands exist exactly once in each launch file with unique orders, no cycle, all 43 `#[path]` mounts and 69 `include_str!`/`include_bytes!` targets exist, longest path 144 bytes: `ALL STATIC CHECKS PASSED`. `.vscode/launch.json` (9,259 configurations) and the seed (6,291) parse; `📋️build-owners.json` parses.
9. TypeScript: `bun test ./🧪️tests/🎮️registry/🟦️.ts -t "XXH3"` from `D`: 1 pass, 3 expects (reads the moved vectors); `bun test ./🧪️tests/⌨️controls/🟦️.ts`: 5 pass, 141 expects (reads the unmoved Rust test files). The remaining registry TS tests need the installed binary (`Build mode must be prototype or release`, 6 fail without it) and two `🧭️authority` tests fail on repo-root editor/agent configuration files; neither touches the files I changed (not caused by this change; not rerun against the original tree).
10. Real compile: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-repo-dashboard-core --message-format=short` -> `error: Finite original maintenance authority required` (`dash-cargo-check-core.log`); `bun repo run @semio-tech/repo-dashboard-launch-rs:check --configuration=prototype --parallel=1 -- --lib --offline` -> `error: Active graph readers retain required Nx pool headroom` (`dash-nx-check-launch.log`). Both at the start and at the end. So **nothing has been type-checked or executed**.

## 8. Residual risks and next steps

1. First real compile. Not covered by anything I could run: trait-method resolution (a method call that needed a trait the old module glob supplied), field and method privacy, and type inference across the new boundaries; the edits are import and path level except the lifecycle/lease split, the `LABEL_CELLS` type change and the moved tests. The daemon crate's `use dashboard_wire::{..}` root imports are used only through `super::` paths of the children, which rustc counts as used.
2. `Cargo.lock` (root) is not refreshed (the launcher is refused). When `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline` works, expect: eleven new packages; `semio-framework-repo-dashboard` depends on the eleven crates plus `serde_json` and `repo-workspace` (loses `serde`, the ui crates and the eight repo domain crates); `dashboard-core` -> serde_json; `launch` -> serde; `repo-domain` -> core, serde, serde_json and the eight repo crates; `registry` -> core, launch, repo-domain, serde, serde_json, ui, ui-locale; `inventory` -> core, registry, repo-domain, repo-tickets, serde, serde_json; `wire` -> core, launch, serde, serde_json, ui; `labels` -> launch, wire, ui, ui-locale; `preferences` -> core, registry, serde, serde_json, ui-locale; `daemon` -> labels, wire, ui; `terminal`, `cli` as the table. A `--locked` resolve is out of date until then. `🔒️dependencies.json` (generated 2026-10-03, last touched by a peer) lists the facade manifest as a `serde`/`serde_json` user; the new manifests are not in it (as for the pets and server carves).
3. The census flags three oversized `ui` dependencies for every crate except core, launch and repo-domain; none of the new crates will compile through the gate until the peer carve of `ui` lands.
4. `cfg(windows)` blocks (transport `platform`, the integration harness) are unchanged text but were not resolved by the oracle (no Windows target installed). The crate builds for no wasm target, so no wasm oracle run applies.
5. The crates keep `role = "tool"` like the facade and the sibling repo crates; the pets and server carves used `product`. If the descriptor gate wants another role it is a one-line change per manifest (`manifests.py`/`spec.py`).
6. Two modules are tested from a crate above them: the daemon unit tests exercise the wire (ipc, replay, journal, ready) through its public API, and the registry laws run in the inventory crate through a real discovery. That keeps the dependency edges downward, but `cargo test -p semio-framework-repo-dashboard-wire` and `-registry` run no test; the facade `test` script runs all twelve packages.
7. The two `GRAPH_REUSE` re-exports (`inventory::GRAPH_REUSE`) predate this change and have no user; left alone. `Read`/`Write`/`Seek` imports flagged by the unused-import heuristic are trait imports used through method calls.
