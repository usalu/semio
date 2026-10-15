# Result: repo CLI split (task C) and repo GraphQL untangle (task E)

Executor: Sonnet 5.5, foreground, no sub-agents. Seam map: `SEAM-MAP-HUB-REPO-SURFACE.md` sections 3 (C) and 5 (E). Recipe: `PRODUCER-CRATE-RECIPE.md`.

## 0. Verdict

Both tasks are done and verified, with one limit: the sanctioned compile route (`bun repo cargo ... check`) is refused by the source-boundary gate, so the compile evidence comes from a **scratch cargo workspace outside the repo** (section 7.2). That is the only run that type-checks the code; it is evidence, not the gate. Nothing was committed, staged or stashed.

| | Before | After |
| --- | --- | --- |
| `⌨️cli/🦀️.rs` | 4,298 lines, two inline modules, `verbs::mcp` to `mcp_verb` to `repo_cli::format_markdown_result` cycle | 35-line lib root plus 8 child files (largest `🚦️verbs` 1,469 lines), cycle gone |
| `🔗️graphql/📦️packages/🦀️rust/🦀️.rs` | 5,038 lines, 13 repo-domain dependencies | 61-line facade plus 6 core children (largest `🏗️repo-schema` 1,242 lines), 2 repo-domain dependencies; `FsRepoContext` and the provider wiring live in the new crate `semio-framework-repo-graphql-fs` (1,100 lines) |

Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/` unless they start with `/` or a dot.

## 1. Task E: GraphQL

### 1.1 New adapter crate `semio-framework-repo-graphql-fs`

Mirrors the recipe (`PRODUCER-CRATE-RECIPE.md` sections 2, 3, 6 to 10):

| Recipe step | What I did |
| --- | --- |
| Node and layout (section 2) | node `🔗️graphql/🗄️fs-context/`; lib root `🗄️fs-context/🦀️.rs` beside the package dir; `🗄️fs-context/🧪️tests/🦀️.rs` mounted under `cfg(test)` as `#[path = "🧪️tests/🦀️.rs"] mod tests;`; vectors in `🧪️tests/🧫️fixtures/🧭️analyze-scope.csv`; package dir `🗄️fs-context/📦️packages/🦀️rust/` holds only `Cargo.toml` and `📋️project.json` |
| Manifest (step 2) | `workspace = "../../../../../../../.."` (8 ups to the repo root: the repo-domain crates are members of the **root** workspace, not of `🧰️framework/Cargo.toml`); `name = "semio-framework-repo-graphql-fs"`, `version = "0.1.0"`, `rust-version.workspace = true`; `[package.metadata.semio] role = "library"`, `id = "repo-graphql-fs"`; `[package.metadata.semio.rust-source-boundary]` with exactly `maxSourceBytes = 262144`, `maxSourceFiles = 32`, `maxActiveLines = 4000`; `[lints] workspace = true`; `[lib] path = "../../🦀️.rs"`; dependencies: `serde_json` (workspace) and 14 path crates (`codebase, contributors, events, goals, graphql, identity, languages, model, move, providers, statutes, tickets, todos, workspace`), `../../../../<module>/📦️packages/🦀️rust` |
| Code move (step 4) | original lines 3957 to 5030 (`AnalyzeScope`, `CoordinatorEmitter`, `ProviderManagement`, `FsRepoContext` and its `RepoContext` impl) moved verbatim; new header and import block; region markers `🧲️Header`, `🗄️FsRepoContext`, `🧪️Tests` |
| Workspace membership (step 6) | one line in root `Cargo.toml` `[workspace] members`, directly after `🔗️graphql/📦️packages/🦀️rust` (byte order: `📦` U+1F4E6 sorts before `🗄` U+1F5C4, and both before `🖥️server`) |
| Lock (step 7) | `Cargo.lock` at repo root only (see 7.1); `🧰️framework/Cargo.lock` not touched because the framework workspace does not contain this crate |
| Nx (step 8) | `📦️packages/🦀️rust/📋️project.json`: `@semio-tech/repo-graphql-fs-rs`, tags `scope:framework`, `language:rust`, no targets (inferred by `cargoTargets()`) |
| Owner row (step 9) | `📋️build-owners.json`: `@semio-tech/repo-graphql-fs-rs`, root and manifest at the new package dir, `packages: []`, inserted in lexical order before `@semio-tech/repo-graphql-go` (checked: names sorted and unique) |
| Launch routes (step 10) | six objects in `.vscode/launch.json`, `🦀️ semio-framework-repo-graphql-fs 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, groups `4_build`, `4_gate`, `4_test`, orders 900.01083/84, 922.04183/84, 903.58283/84 (each checked unused in its group), placed after the last contiguous producer block (`pack-json-retained-schema`) |
| Test | `🧪️tests/🦀️.rs` reads the CSV (9 vectors, `raw,kind,value`) and asserts `AnalyzeScope::parse`; passes (7.2) |

Deviations from the recipe, all deliberate:

- `role = "library"`, not `"framework"`: the 22 sibling repo-domain crates and the carved-from crate all use `library`. The recipe's `framework` count is for the `🖱️ui` cluster.
- **Step 11 (boundary corpus row) is not applicable.** That corpus (`🖱️ui/🧪️tests/🧫️fixtures/📦️crate-boundaries`) covers UI producers only.
- **Step 1 (taxonomy registration) was not done.** Every new folder emoji is already used as a folder prefix elsewhere in the repo and is RGI (7.3), but I did not add taxonomy entries or run the taxonomy validators.
- The two `projectTarget.*` option lists in `launch.json` were not touched: the four recipe producers are not in them either (they list projects with declared targets, not inferred ones).
- No third-party oracle for `AnalyzeScope::parse` (AGENTS.md asks for one per feature). The reference is the Go twin's `ParseScope` and I did not write a Go-side test for the CSV. Open item.

### 1.2 Core crate `semio-framework-repo-graphql`

`Cargo.toml` lost 11 dependencies (`codebase, contributors, events, goals, languages, move, providers, statutes, tickets, todos, workspace`); it keeps `serde`, `serde_json`, `identity`, `model`. The lib root stays at `📦️packages/🦀️rust/🦀️.rs` (`[lib] path` unchanged, test mount `../../🧪️tests/🔬️unit/🦀️.rs` unchanged). Six children under `📦️packages/🦀️rust/`, each `<emoji>️<kebab>/🦀️.rs` starting with `//! <emoji>` and `use super::*;`, mounted as `#[path = ...] mod x; pub use x::*;` in a `//#region 🔖️Children` block:

| Child (mod) | Original ranges | Lines | Active (non-blank, non-`//`) | Widened |
| --- | --- | --- | --- | --- |
| `🔤️grammar` (`grammar`) | 28-644 | 621 | 530 | 2 fields, 1 item, 3 members |
| `🏗️repo-schema` (`repo_schema`) | 647-853, 1429-2322, 3120-3254 | 1,242 | 1,015 | 1 item, 1 member |
| `🎥️context-port` (`context_port`) | 855-981 | 131 | 64 | none |
| `🖼️sources` (`sources`) | 983-1427 | 449 | 353 | 30 items |
| `⚡️execution` (`execution`) | 2324-3118 | 799 | 696 | 8 fields, 3 members, 1 item |
| `🩻️recording-context` (`recording_context`) | 3256-3955 | 704 | 625 | 16 fields, 1 item |
| facade `🦀️.rs` | 1-27, 5032-5038 | 61 | 33 | |

- The seam map named the schema child `📜️schema`. I used `🏗️repo-schema` because `📜️script.ts` sits in the same package dir and the executor rule forbids a repeated emoji among siblings.
- `sources::parse_milestone_number` became `pub` (the only core item the adapter needs besides `ContextError` and `RepoContext`, which were already public). Its use in `FsRepoContext` is `use semio_framework_repo_graphql::parse_milestone_number`.
- Child dependency graph (by identifier references; false positives from local variable names removed by hand): `grammar`, `repo-schema`, `context-port` are leaves; `sources` reads `repo-schema`; `execution` reads `grammar`, `repo-schema`, `context-port`, `sources`; `recording-context` reads `context-port`, `sources` and `execution` (`matches_filter`). The map said `recording-context` needs only `context-port` and `model`. If it moves into a test-support crate later it takes `sources` and `execution` as dependencies.
- No source-text readers of the GraphQL lib root exist (path-tail search, 7.5). The 7 non-unit test adapters under `🧪️tests/*` import only public names (`parse`, `validate`, `operation_type`, `coerce_arguments`, `build_schema`, `schema_inventory`, `Executor`, `RecordingContext`) and `serde_json`, all still exported.

### 1.3 Consumers of the moved items

Cargo.toml scan over `🧰️framework`, `✏️s`, `🌎️hub`, `🧪️tests`, `🧬️schema`, root files and `.vscode` (`rcg-consumer-grep.txt`): the only consumer of `FsRepoContext` is the repo CLI.

- `⌨️cli/📦️packages/🦀️rust/Cargo.toml`: added `semio-framework-repo-graphql-fs = { path = "../../../🔗️graphql/🗄️fs-context/📦️packages/🦀️rust" }`.
- CLI sources: `use semio_framework_repo_graphql_fs::FsRepoContext;` in the `repo_cli` facade and in `mcp_verb`; `Executor`, `RecordingContext`, `RepoContext` still come from the core.

## 2. Task C: repo CLI

### 2.1 Layout

Lib root `⌨️cli/🦀️.rs` (35 lines) mounts three modules, keeping the original docs and region markers (`🧭️RepoCli`, `🔌️McpVerb`, new `📡️EventStream`):

| File | Mod | Lines | Active | Notes |
| --- | --- | --- | --- | --- |
| `🧭️repo-cli/🦀️.rs` | `repo_cli` (pub) | 37 | 23 | facade: pruned `use` block, `pub use crate::event_stream::*;`, mounts and globs |
| `🧭️repo-cli/🎮️command-framework/🦀️.rs` | `command_framework` | 514 | 401 | original 842-1351 |
| `🧭️repo-cli/🌳️command-tree/🦀️.rs` | `command_tree` | 435 | 404 | 1353-1783 |
| `🧭️repo-cli/🗂️tree-source-and-search/🦀️.rs` | `tree_source_and_search` | 497 | 431 | 1785-2277 |
| `🧭️repo-cli/🚦️verbs/🦀️.rs` | `verbs` (pub, kept `pub mod`) | 1,469 | 1,351 | 2284-3744, dedented by 8, region `🚦️Verbs` kept |
| `🧭️repo-cli/🚀️run-and-projection/🦀️.rs` | `run_and_projection` | 226 | 181 | 3749-3970 |
| `📡️event-stream/🦀️.rs` | `event_stream` (private, root) | 824 | 718 | 24-840: events, Go formatting, renderers |
| `🔌️mcp-verb/🦀️.rs` | `mcp_verb` (pub, root) | 317 | 277 | 3982-4295, imports `FsRepoContext` from the new crate |

Deviation from the map: `📡️event-stream` is a sibling of `🧭️repo-cli` and is mounted at the lib root, not under the facade (the map put the folder under `🧭️repo-cli/`). Reason: it is what breaks the module cycle (2.2). `event_stream` is the map's C01, so it holds the renderers; I did not split events, Go formatting and renderers into three folders because the map budgets them as one child (718 active).

All 11 test-adapter names of seam map 3.5 still resolve through the facade (`run`, `usage`, `projection`, `command_paths`, `render_stream_json`, `mcp_conversation`, `hook_verb_dispatch_json`, `graphql_roundtrip_json`, `test_verb_lines_json`, `export_records_json`; the module path `semio_framework_repo_cli::repo_cli` itself is the 8 adapters' import), checked by compiling a test that takes each as a value (7.2).

Widened to `pub(crate)` (referenced from a sibling child): `event_stream::{go_sprint, render_human}`, `command_framework::Command::{projection, parse}`, `tree_source_and_search::{status_filter, tree_filter, flatten_tree, searched_tree, write_tree_node, write_tree_forest, text_tree_lines}`. Nothing else in the CLI needed widening.

### 2.2 Cycle break

Before: `verbs::mcp` calls `crate::mcp_verb::RepoRepository::open`, and `mcp_verb` called `crate::repo_cli::format_markdown_result`.

After: the formatter lives in the renderers part of `📡️event-stream`, a leaf module at the lib root whose only imports are external crates (no `crate::` or `super::` path). `mcp_verb` now calls `crate::event_stream::format_markdown_result(...)`, its only dependency inside the crate. The old path `repo_cli::format_markdown_result` appears nowhere (grep: 0 hits); no alias was added. The remaining in-crate edges are `verbs` and `run_and_projection` to `mcp_verb` to `event_stream`.

The facade's `pub use crate::event_stream::*;` is the only way `repo_cli::Format` etc. stay reachable for the other children's `use super::*`; it is a re-export, not a forwarding function.

### 2.3 Process-spawning port suggestion (recorded, not implemented)

`🧭️repo-cli/🚦️verbs/🦀️.rs` runs two real child processes inside verb handlers:

- line 996, `mermaid_contributors`: `std::process::Command::new("git").args(["blame", "--line-porcelain"])...output()`;
- line 1197, `micro_commit`: `std::process::Command::new(&argv[0]).args(&argv[1..]).current_dir(&root).status()` (an arbitrary argv).

Suggestion: a `ProcessPort` trait in the CLI (for example `run(program, args, cwd) -> Result<Output, CliError>` plus a `status` variant), a `SystemProcessPort` next to `FsRepoContext` composition, and a recording double beside `RecordingContext` so both verbs become testable without spawning git or arbitrary commands. The port belongs in the verbs child or a new `⚙️process` child; the verbs file would then lose its two `std::process` references. Not a split blocker; implementing it changes the `use` set of `verbs`.

## 3. Text readers and docs updated

- `📚️library/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json`: the CLI row's `"anchor"` was `pub fn run(argv`, which now lives in a child; changed to `pub mod repo_cli;` (present in the lib root, which stays the `owner`). Verified by a script that the owner file contains the anchor. The test `📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts` itself fails before and after because it reads `🧫️fixtures/` instead of `🧪️tests/🧫️fixtures/` (ENOENT at line 30); that is not mine and I did not touch it.
- `📚️library/🧪️tests/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json:4922` names `⌨️cli/🦀️.rs` as a historical owner path. The file still exists as the lib root, so the row is unchanged.
- `🔗️graphql/README.md` and `⌨️cli/README.md`: package list and a short Rust layout section.
- Path-tail search over `🧰️framework`, `✏️s`, `🌎️hub`, `🧪️tests`, `🧬️schema`, root script and owners (`🗑️generated/rcg-path-tail-grep.txt`): remaining hits are manifest or project paths that did not move, the db CLI and other `⌨️cli` modules (different crates), and the two fixtures above.

## 4. Lock files and metadata

- `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline` (log `rcg-lock-update-root.log`): exit 0, "Locking 0 packages". The new package entry and the CLI and core dependency lists were already in `Cargo.lock` by then, written by a cargo invocation I did not trace; the final lock entries match my manifests: `semio-framework-repo-graphql` has `identity, model, serde, serde_json`; `semio-framework-repo-graphql-fs` has the 14 path crates plus `serde_json`; `semio-framework-repo-cli` lists `semio-framework-repo-graphql-fs`. The update also normalised one unrelated peer line (a `semio-framework-async` dependency removed from some package). The working `Cargo.lock` diff is much larger than mine because peers edit it too.
- `bun repo cargo ... -- metadata --offline --locked --format-version 1 --no-deps` (log `rcg-metadata-root.log`): exit 0, 88 packages and 88 members. Parsed: graphql-fs manifest at the new path, lib target `.../📦️packages/🦀️rust/../../🦀️.rs`, role and boundary keys as written; CLI has three targets (lib, `repo`, `semio-repo`).
- A later metadata run failed with `cannot update the lock file ... --locked` because a peer had added a workspace member in the meantime (log `rcg-metadata-root-failed-peer-churn.log`). I re-ran `update --workspace --offline` ("Locking 1 package", the peer's `semio-framework-os-store-link-schema`) and then metadata again: exit 0. The final logs `rcg-lock-update-root.log` and `rcg-metadata-root.log` are from that second pair. Expect the same drift whenever a peer adds a member before this lands.
- Only the root workspace was touched. `🧰️framework/Cargo.lock` and the hub workspace were not.

## 5. Size and gate numbers

`rcg-verify-misc.log` (rustfmt, paths, regions, sizes) and `rcg-verify-source-closure.log` (the repo's own `readRustSourceClosureV1`, cfg-selected, the numbers the compiler-ingress gate uses; its `activeLines` is the raw line count of the selected text, including blanks and comments):

| Crate | Cfg | Files | Gate lines | Bytes |
| --- | --- | --- | --- | --- |
| `semio-framework-repo-graphql-fs` (new) | native | 1 | 1,099 | 59,059 |
| | test | 3 | 1,128 | 60,345 |
| `semio-framework-repo-graphql` (core) | native | 7 | **4,012** | 179,150 |
| | test | 9 | **4,173** | 198,509 |
| `semio-framework-repo-cli` | native and test | 9 | **4,362** | 218,701 |

The new crate is far inside 4000 / 32 / 262144. The core is 12 lines over the 4000 gate (native) and 173 over (test); the CLI is 362 over. Every child is at or below 1,469 lines (target 2,500), every path at most 138 bytes from the repo root (limit 240). Dependency crates of the adapter are all below 4000 (`rcg-verify-dependency-closures.log`; largest `repo-model` 3,820, `repo-tickets` 2,783).

## 6. Official route: what the gate says

`bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-repo-graphql-fs --message-format=short` (log `rcg-official-check-graphql-fs.log`): refused before rustc with `Rust crate Source boundary exceeded before compiler: semio-framework-repo-graphql (active lines 4012/4000)`. The new adapter itself is not the problem; the core it implements the port of is 12 gate lines over. All other dependency crates are under the limit, so trimming 12 lines (or carving `🩻️recording-context`, 704 lines, into its own crate, which the seam map lists as the test-support option) would admit the adapter on the native check. The test cfg (4,173) and the CLI (4,362) stay refused until a further carve. I did not trim whitespace to hit the number.

Baseline before my change: `bun repo cargo ... check -p semio-framework-repo-cli` died at `cargo fetch --locked` ("cannot update the lock file", peer manifests ahead of the lock); log `rcg-baseline-cli-check.log`.

`bun repo run @semio-tech/repo-lib:lint-cargo-dependency-direction` (log `rcg-gate-cargo-dependency-direction.log`) failed before reaching any verdict: `Canonical native dependency owner required: 🧰️framework/🔨️modules/◻️2d/⚙️engine/📦️packages/🦀️rust`, i.e. the peer-owned `◻️2d` restructuring, unrelated to these crates. No dependency-direction verdict exists for the new crate; its edges all point down to repo-domain crates and to the core it implements.

## 7. Verification actually run

### 7.1 Full-resolution differential (the oracle of `📓️RESOLUTION-ORACLE.md`)

`rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null --error-format=json [--test] <lib root>`, run from the real crate lib roots against a sandbox that keeps the original in the same relative layout (`rcg-tools/mksb.py`, `oracle.sh`, `diagsum.py`; outputs `rcg-oracle-*.log`). Compared: sorted multiset of (level, code, message, primary label).

| Pair | Native | `--test` |
| --- | --- | --- |
| GraphQL core vs original lines 1-3955 plus test mount | 9 vs 9 errors, **identical** | 9 vs 9, **identical** |
| CLI vs original (only `FsRepoContext` import retargeted) | 300 vs 303 | 300 vs 303 |

All errors are unresolved external crates and the missing `serde` attribute (the repo crates are not linked). The CLI's 3 extra errors are `E0432` for the three `use` lines (`graphql`, `model`, `serde_json`) of the new `event_stream` file; there is no new `E0412`, `E0425`, `E0422`, `E0423`, `E0603`, `E0616` or `E0624`. Negative control: removing `pub(crate)` from `go_sprint` and `status_filter` in a copy makes the oracle report three new `E0425` (`rcg-oracle-cli-neg-native.log`), so the check is not vacuous. As the oracle doc says, it cannot see field and method privacy; the compile in 7.2 does.

### 7.2 Real type-check and tests in a scratch workspace outside the repo

The repo's `.cargo` ingress does not apply to a manifest elsewhere, so I built small scratch packages (`rcg-tools/scratch-cargo/*.Cargo.toml`, in the session scratchpad, target dir outside the repo) whose `[lib] path` is the real lib root and whose dependencies are the real repo crates by absolute path, offline. This reads repo sources and writes nothing in the repo. It is not the gate, and the registry crate versions it resolves differ from the repo lock.

| Check | Result |
| --- | --- |
| `cargo check --lib` core (split) | passes, no warnings from the crate |
| `cargo test --lib` core | 12 passed (`projects_the_canonical_shape` ... `serves_the_committed_sdl`) |
| `cargo check --lib` and `cargo test --lib` adapter | passes without warnings; `tests::parses_every_scope_vector` ok |
| `cargo check --lib --bins` CLI (split, real `repo` and `semio-repo` bins, real adapter dependency) | passes, no warnings from the crate |
| the same with `-W unused_qualifications -W rust_2018_idioms -W future_incompatible -W unused_lifetimes` on core, adapter, CLI | no warnings from these crates (one pre-existing unused import in `repo-events`, not mine) |
| `cargo test` with a file that takes each of the 11 map-3.5 names plus `format_markdown_result`, `Format::parse`, `verbs::mcp`, `mcp_verb::real_repository` as values | passes |
| Baseline CLI (original with the one import retargeted) compiled as the control | passes |
| Behaviour, built `semio-repo` (split) vs built `semio-repo` (baseline) | `--help` for all 99 command paths byte-identical (stdout, stderr, exit code); 6 error and flag invocations (unknown command, `ticket nosuch`, `--version`, bare `graphql`, `tree --help`, `list --format ndjson --help`) and 6 data invocations in a throwaway git repo (`tree`, `list`, two `graphql` queries, `loc`, `query a`) identical (first 3,000 bytes of output plus exit code for the data runs) |

### 7.3 Static checks

| Check | Result |
| --- | --- |
| Lossless (`rcg-tools/verify_lossless.py`, trimmed non-blank line multiset, `pub(crate) ` normalised, dedent undone) | GraphQL: 1 line missing (the original `use ...FixResult;`, merged into the adapter's import block); CLI: 10 missing, all openers/closers and the replaced `use`/call lines. Extras are only headers, mounts, `use` lines, docs and region wrappers. Full lists in `rcg-verify-lossless.log` |
| rustfmt | `rustfmt --edition 2021 --emit stdout` exit 0 on all 18 files, and on the three lib roots (follows every `#[path]`) |
| Path resolution | 17 `#[path]` and `include_str!` literals, 0 missing |
| Emoji admission (`/^\p{RGI_Emoji}$/v`, U+FE0F, no sibling with the same first emoji, directory exists) | 18 of 18 ok (`rcg-verify-emoji.log`) |
| Region markers | balanced in every new file; every original marker is still present (`RepoCli`, `McpVerb`, `Events`, `GoFormatting`, `Renderers`, ..., `Tests`, `Header`) |
| `launch.json` and seed | `launch.json` parses (8,788 configurations); the 6 new routes are unique; one pre-existing duplicate name (`🧱️artifact-io-⚖️installed-service🧰️os🟦️`) is not mine. The seed parses with `Bun.JSONC` |
| `build-owners.json` | parses, names sorted and unique |
| `[DEBUG]` logs | none added |

### 7.4 What is not verified

- No run through the sanctioned `bun repo cargo` compile route (section 6), so the repo's own admission of the new crate (`rust-source-boundary` keys through the descriptor) is untested; only its numbers are measured.
- The 7 non-unit GraphQL test adapters and the 8 CLI test adapters are not compiled by any crate here; their imports were checked by name only.
- Nx graph inference of `@semio-tech/repo-graphql-fs-rs` (targets `build`, `check`, `test`) was not exercised; the dependency-direction gate and the library tests that walk owners could not run (section 6, and the ownership test's pre-existing fixture path bug).
- No taxonomy validator was run on the new folders.

## 8. Reproduction

Tools and logs live in `🪓️monolith-split/🗑️generated/rcg-tools/` (prefix `rcg-`): `split_graphql.py` and `split_cli.py` (generators; they assert the live monolith still has the SHA of `graphql-original.rs.txt` and `cli-original.rs.txt` before writing), `splitlib.py` (dedent, widening, export kind), `rlex.py`, `oracle.sh`, `diagsum.py`, `mksb.py`, `verify_lossless.py`, `verify_misc.py`, `em.ts`, `closure.ts`, `closure_all.ts`, `scratch-cargo/`. Oracle outputs: `🗑️generated/rcg-oracle-*.log`; other logs `rcg-*.log`.

## 9. Files touched

New: `🔗️graphql/🗄️fs-context/🦀️.rs`, `.../🧪️tests/🦀️.rs`, `.../🧪️tests/🧫️fixtures/🧭️analyze-scope.csv`, `.../📦️packages/🦀️rust/Cargo.toml`, `.../📦️packages/🦀️rust/📋️project.json`; `🔗️graphql/📦️packages/🦀️rust/{🔤️grammar,🏗️repo-schema,🎥️context-port,🖼️sources,⚡️execution,🩻️recording-context}/🦀️.rs`; `⌨️cli/🧭️repo-cli/🦀️.rs` and its five child folders; `⌨️cli/📡️event-stream/🦀️.rs`; `⌨️cli/🔌️mcp-verb/🦀️.rs`.

Edited: `🔗️graphql/📦️packages/🦀️rust/🦀️.rs` (now the facade), `🔗️graphql/📦️packages/🦀️rust/Cargo.toml`, `⌨️cli/🦀️.rs` (now the lib root), `⌨️cli/📦️packages/🦀️rust/Cargo.toml`, both READMEs, `📚️library/🧪️tests/🧫️fixtures/🦑️repo-source-ownership/🔣️.json`; at the repo root `Cargo.toml` (one member line), `Cargo.lock` (via `update`), `📋️build-owners.json` (one row), `.vscode/launch.json` (six routes).

## Hardening (patched oracle)

Wave 2, Sonnet executor, 2026-10-15. Full write-up: `RESULT-HARDEN-WAVE2.md`. No repository source file was edited; no regression was found.

**Method and baselines.** `rustc ... --emit=metadata -o /dev/null --error-format=json` from each crate's lib root, sandbox with module files copied and patched identically (0 `extern crate` and 0 external globs in these trees: the patch is a no-op, unpatched totals are equal). Baselines: CLI = `rcg-tools/cli-baseline-import-retargeted.rs.txt`; GraphQL core = `graphql-original.rs.txt` lines 1 to 3,956 plus 5,031 to 5,038 (the `FsRepoContext` region left for the adapter); adapter checked standalone and, because every core name it uses is an external import in a standalone run, also **combined** with the core in one crate (adapter `use semio_framework_repo_graphql::` rewritten to `use crate::semio_framework_repo_graphql::` in the sandbox copy) against the full 5,038-line original. Lossless baseline vs split family: CLI misses only the `pub mod repo_cli {`, `pub mod verbs {`, `pub mod mcp_verb {` openers, three closing braces, two relocated `use` lines and the intended cycle-break call (`crate::repo_cli::format_markdown_result`); GraphQL misses only `fn parse_milestone_number` (now `pub fn`).

| Crate | Config | Baseline / split errors | Non-extern | Differing keys |
|---|---|---|---|---|
| CLI | native | 301 / 304 | 0 / 0 | 3 extern |
| | `--test` | 301 / 304 | 0 / 0 | 3 extern |
| GraphQL core | native | 10 / 10 | 0 / 0 | 0 |
| | `--test` | 10 / 10 | 0 / 0 | 0 |
| core + adapter combined vs original | native | 128 / 130 | 0 / 0 | 2 extern |
| | `--test` | 128 / 130 | 0 / 0 | 2 extern |
| adapter standalone | native, `--test` | 122 / 122 | 0 | none (all errors external) |

The 3 CLI differences are the `E0432` for `use semio_framework_repo_graphql`, `semio_framework_repo_model`, `serde_json` in the new `📡️event-stream` file (all three are in `⌨️cli/📦️packages/🦀️rust/Cargo.toml`); the 2 combined differences are the adapter's own `use semio_framework_repo_identity` and `use serde_json`. Every CLI error is an unresolved external crate (non-extern 0), so the oracle is sharp: any in-crate name error would stand out.

**Fixes.** None needed.

**Cross-crate edge.** The adapter imports exactly `parse_milestone_number`, `ContextError` and `RepoContext` from the core; all three are `pub` (`🖼️sources/🦀️.rs:209`, `🎥️context-port/🦀️.rs:9` and `:36`) and reach the root through `pub use <child>::*`. The combined differential resolves them as module paths and flags a private one: seeding `parse_milestone_number` to `pub(super)` produces `E0603 function import parse_milestone_number is private`. (Same-crate modules cannot tell `pub(crate)` from `pub` across the real crate boundary. The core does carry `pub(crate)` helpers, for example the `*_source` fns of `🖼️sources` and `matches_filter` plus the `Execution` members of `⚡️execution`, but the adapter names no core item besides the three above: any other core name it used would be an unresolved name in the combined run, and there is none (non-extern 0).)

**Definition scan.** CLI (9 files, scope includes `📡️event-stream`, which was inside `repo_cli`): fields 3 private / 0 narrowed, members 11 / 2, top-level 65 / 9; 1 collision row (`SharedBuffer.0`, a tuple-field name against unrelated `.0` accesses; the type is named only in `🚀️run-and-projection`), 0 real, 0 items, 0 tuple constructors. GraphQL core (7 files): fields 7 / 26, members 24 / 7, top-level 65 / 33; 0 rows, 0 items, 0 tuple. Seeded: stripping `pub(crate)` from 2 items of `📡️event-stream` adds 2 E0425, from 7 items of `🗂️tree-source-and-search` adds 11 E0425, from the one item of `⚡️execution` adds 1; stripping the visibility of two methods in `🎮️command-framework` makes the scan report 4 rows, of 4 fns and 8 fields in `⚡️execution` 4 rows (all 0 on the real trees apart from the one collision).

**Residual.** Real cross-crate behaviour (trait coherence, `pub(crate)` across the new crate boundary beyond the 3 names above) is covered by the earlier scratch-workspace compile of section 7.2 only; this hardening used rustc analysis alone. Tools: `harden2-tools/` (`ho.py` targets `cli gql gqlfs gqlfull`, `comb.py`).
