# Result W4: `semio-framework-repo-cli` carved into five producer crates plus a thin command-tree facade

Executor: Sonnet 5.5, foreground, no sub-agents, 2026-10-15. Brief: `📓️WAVE4-BRIEF.md`. Nothing was committed, staged, stashed or checked out. Tools: `repocli-tools/` (`carve_src.py`, `carve_manifests.py`, `carve_launch.py`, `oracle.py`, `per_root.py`, `use_vis_scan.py`, `vis_scan.py`, `unused_imports.py`, `lossless.py`, `verify.ts`, helpers `strip.py`, `sib.py`, `bare.py`, `deps.py`). Logs: `🗑️generated/rcw4-*.log`. Sandboxes live in the session scratchpad only.
`M` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules`; all crate paths below are relative to `M/⌨️cli`.

## 0. Verdict

The split is done and statically verified: every crate (lib and test closure) fits the gate with wide headroom, the two binaries (`repo`, `semio-repo`) keep their sources and their lib API, and the full-resolution differential shows no regression. **No cargo check or test could run**: the sanctioned launcher and the Nx route both fail before rustc for reasons that are not this change (section 5). So nothing here is a compiled or executed result; the type-level facts the oracle cannot see are covered by scans and listed as residual risks (section 7). `Cargo.lock` is not refreshed.

## 1. Census before and after

Canonical counter (`readRustSourceClosureV1` through `🗑️generated/census.ts`, `ONLY=<crate list>`, log `🗑️generated/rcw4-census-after.log`; limits 4000 lines, 32 files, 262144 bytes; target 3500 / 28 / 230000). The crate has no `#[cfg(test)]` code anywhere (grep: 0 hits), so the cfg-`test` closure equals the lib closure for every row; the test adapters under `🧪️tests/` are separate subject files linked by the test host.

| Crate | Files | Bytes | Active lines | Worst ratio | Compilable closure |
|---|---|---|---|---|---|
| `semio-framework-repo-cli` before | 9 | 219,121 | **4,378** (over) | 1.095 | false |
| `semio-framework-repo-cli` after (facade) | 4 | 45,333 | 726 | 0.181 | true |
| `semio-framework-repo-cli-event-stream` (new) | 1 | 34,998 | 829 | 0.207 | true |
| `semio-framework-repo-cli-command-framework` (new) | 1 | 19,684 | 521 | 0.130 | true |
| `semio-framework-repo-cli-tree-source-and-search` (new) | 1 | 26,450 | 507 | 0.127 | true |
| `semio-framework-repo-cli-mcp-verb` (new) | 1 | 17,953 | 321 | 0.080 | true |
| `semio-framework-repo-cli-verbs` (new) | 1 | 76,449 | 1,480 | 0.370 | true |

All rows `overLimit false`, `oversizedDeps 0`. The largest crate is `verbs` at 37% of the line budget; the sum is 4,384 lines (the six extra are doc lines of the rewritten facade and lib root). Longest path in the tree: 163 bytes (limit 240).

## 2. Crate graph

Dependencies point downward only (`verify.ts`: DFS over the manifest edges, no cycle; and the isolated per-root check in section 4.3 shows every sibling is reached through a declared dependency):

```
event-stream  <-  command-framework  <-  tree-source-and-search  <-  verbs
      ^                                                               ^
      +--- mcp-verb ---------------------------------------------------+
facade (semio-framework-repo-cli) depends on all five and owns command-tree + run-and-projection
```

| Crate | Node (lib root `<node>/🦀️.rs`, package `<node>/📦️packages/🦀️rust`) | Nx project / `semio.id` | Depends on (first-party cli crates) | Content (files did not move) |
|---|---|---|---|---|
| `semio-framework-repo-cli-event-stream` | `📡️event-stream` | `@semio-tech/repo-cli-event-stream-rs` | none | events, Go-twin formatting, three renderers, `graphql_events`; `go_sprint` widened to `pub` |
| `semio-framework-repo-cli-command-framework` | `🧭️repo-cli/🎮️command-framework` | `@semio-tech/repo-cli-command-framework-rs` | event-stream | typed flags, `Command`, argv parser, `Invocation`; `FlagValue::projection` widened to `pub` |
| `semio-framework-repo-cli-tree-source-and-search` | `🧭️repo-cli/🗂️tree-source-and-search` | `@semio-tech/repo-cli-tree-source-and-search-rs` | command-framework, event-stream | `FsTreeSource`, model ids, `StreamOptions`; 7 `pub(crate)` helpers widened to `pub` |
| `semio-framework-repo-cli-mcp-verb` | `🔌️mcp-verb` | `@semio-tech/repo-cli-mcp-verb-rs` | event-stream | `RepoRepository`, `real_repository` |
| `semio-framework-repo-cli-verbs` | `🧭️repo-cli/🚦️verbs` | `@semio-tech/repo-cli-verbs-rs` | command-framework, event-stream, mcp-verb, tree-source-and-search | one handler per verb |
| `semio-framework-repo-cli` (facade, lib + bins `repo`, `semio-repo`) | `📦️packages/🦀️rust` with lib root `⌨️cli/🦀️.rs` | `@semio-tech/repo-cli-rs` (unchanged) | all five | `🌳️command-tree`, `🚀️run-and-projection`, the `repo_cli` facade |

The cycle that the earlier split resolved at module level (`verbs::mcp` to `mcp_verb` to the renderers) is now a crate-level DAG. Cohesion: the event stream is the leaf every other crate reads; the framework is what the tree source and the verbs speak; the verbs are the only crate that depends on the MCP repository.

Each module file has exactly one owning crate (`verify.ts`: 3 `#[path]` attributes remain, all in the facade, each target mounted once). The former crate is a thin facade, not a compatibility layer: `repo_cli` keeps exactly the names the binaries and the eight language-agnostic test adapters import (`run`, `usage`, `projection`, `command_paths`, `render_stream_json`, `mcp_conversation`, `hook_verb_dispatch_json`, `graphql_roundtrip_json`, `test_verb_lines_json`, `export_records_json`) plus `verbs` (explicit `pub use semio_framework_repo_cli_verbs as verbs`, which keeps the old `pub mod verbs` path), and the lib root re-exports `mcp_verb` (`pub use semio_framework_repo_cli_mcp_verb as mcp_verb`). The earlier `pub use <child>::*` globs are gone; no consumer used them (git grep over `*.rs`, `*.ts`, `*.md` found no `repo_cli::Format`, `Command`, `root` or `event_stream` path outside this crate). `run_with` and the other formerly glob-exported names are no longer reachable from outside the crate.

## 3. Files changed

Source (each rewritten with asserted anchors against the SHA-256 recorded before the first edit, `carve_src.py`; the git diff of the nine files is 58 insertions and 52 deletions): the lib root `⌨️cli/🦀️.rs`, `🧭️repo-cli/🦀️.rs` (facade), and the seven module files. The edits per file are: `use super::*;` replaced by an explicit import block; the facade `use` block, mounts and globs removed; `crate::mcp_verb::RepoRepository::open` and `crate::event_stream::format_markdown_result` imported and called bare; the visibility widening in the table above; the `📡️EventStream` region marker pair moved with its content into the event-stream file (nested inside `🧭️RepoCli`), the `🚦️Verbs` pair kept in the facade around the `verbs` re-export. `lossless.py` (trimmed line multiset against the pre-edit copy, `🗑️generated/rcw4-lossless.log`): every body line is preserved; the only differences are mounts and globs, the import blocks, the 10 visibility lines, the two call sites, the doc lines of the facade and the lib root. Region markers: 23 before, 22 after in the nine files, balanced; the one missing is the duplicate-named `🚦️Verbs` pair that the verbs file already carried (the facade keeps its own pair), so every distinct name is still present and balanced.

Created per new crate: `<node>/📦️packages/🦀️rust/Cargo.toml` and `📋️project.json` (10 files). Manifest content: `workspace` depth derived from the path (8 ups for the two root nodes, 9 for the three nested ones, all resolve to the repo root), `role = "library"` (as the sibling repo-domain crates and `graphql-fs`), `id = repo-cli-<name>`, exactly the three `rust-source-boundary` keys at the ceiling, `[lints] workspace = true`, `[lib] path = "../../🦀️.rs"`. Dependency sets are derived from the carved sources and checked against the analysis oracle's unresolved-crate list (4.3). `📋️project.json` carries only name and the `scope:framework`, `language:rust` tags (targets are inferred).

Edited elsewhere:
- `⌨️cli/📦️packages/🦀️rust/Cargo.toml`: dependencies reduced to what the lib root, the two moved-in modules and the two binaries use (the five new crates, `events`, `graphql`, `hooks`, `mcp`, `metrics`, `model`, `test-runner`, `workspace`, `serde_json`); `serde`, `graphql-fs` and 11 other domain crates moved to the children.
- `⌨️cli/📦️packages/🦀️rust/📜️script.ts`: the `test` route runs `cargo test` for the five children and the facade, bottom up (the pets precedent).
- Root `Cargo.toml`: five members after `⌨️cli/📦️packages/🦀️rust` and before it in byte order (`📡` before `📦` before `🔌` before `🧭`; checked by byte comparison, contiguous, neighbours still sorted). Only the root workspace lists the CLI, so `🧰️framework/Cargo.toml` is untouched.
- `📋️build-owners.json`: five rows around `repo-cli-go` and `repo-cli-rs`, in name order (1,740 owners, names unique).
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 30 routes each, six per crate (`🦀️ semio-framework-repo-cli-<name> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`), groups `4_build`, `4_gate`, `4_test`, orders `900.01521` to `900.0153`, `922.04521` to `922.0453`, `903.58521` to `903.5853`, each unique within its group in both files, key order `name, type, request, command, cwd, presentation`. In `launch.json` they follow the `repo-graphql-schema` block, in the seed the `pets-staging` block. The seed gets them because the brief asks for both files (pets did; the server and graphql-fs carves did not).
- `⌨️cli/README.md`: package list and the Rust layout section.

Consumers re-pointed: none outside this crate needed an edit. Evidence: the eight test adapters, the two binaries, `🔌️mcp/📦️packages/🦀️rust/📜️script.ts` (`-p semio-framework-repo-cli --bin repo`), the Go test strings (`cargo build --release -p semio-framework-repo-cli`), the test host's subject-package inference (`rustSubjectPackage` finds `⌨️cli/📦️packages/🦀️rust`, name `semio-framework-repo-cli`) and the two fixtures (`🦑️repo-source-ownership` anchor `pub mod repo_cli;` is still in the lib root, count 1; `remaining-package-purity-authority` names only the lib root) all resolve against the facade unchanged. No source-text reader (`readModuleTreeSource` or path-tail) reads the moved files: the path-tail git grep found only the two fixtures and other crates' `⌨️cli` folders.

No embedded assets exist in this crate (no `include_str!` or `include_bytes!`), so the asset channel step did not apply. Peer-owned crates (`os-kernel`, `plugin`, `replication`, `ui-*`, `job`, `2d`) were not touched. The GraphQL core is being carved by a peer in parallel (`🔗️graphql/🏗️repo-schema` appeared during this work); the CLI only names `Executor`, `RepoContext`, `RecordingContext` from the core and `FsRepoContext` from the adapter, which the peer's facade still exports.

## 4. Verification with real output

### 4.1 Differential oracle (`oracle.py`, `🗑️generated/rcw4-oracle-union.log`)

`rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_framework_repo_cli --emit=metadata --out-dir <scratch> --error-format=json [--test] <lib root>`, `extern crate` lines removed identically (a no-op here, there are none; no external globs exist). Baseline sandbox: the pre-edit nine files plus the rest of the tree in the same layout. Split sandbox: the edited tree, plus the five crate roots mounted as modules `semio_framework_repo_cli_<x>` of one crate, with every `semio_framework_repo_cli_<x>` path rewritten to `crate::semio_framework_repo_cli_<x>` (union simulation, as in the server carve).

Note on the command: `-o /dev/null` fails on this host with `couldn't create a temp dir: Operation not permitted ... /dev/rmeta...` (also with the sandbox disabled), so the output goes to `--out-dir` in the session scratchpad, is deleted right after each run, and is never used as `--extern`.

| Config | Baseline errors | Split errors | Non-external | New in split | Gone in split |
|---|---|---|---|---|---|
| native | 304 | 312 | 265 / 265 | 8 x `E0432` (one per extra `use` line of an external crate: codebase 1, graphql 2, identity 1, model 1, serde_json 3) | none |
| `--test` | 304 | 312 | 265 / 265 | same 8 | none |

No new `E0412`, `E0425`, `E0422`, `E0423`, `E0603`, `E0616`, `E0624`, no unused-import warning for an internal sibling import (0 warnings). The 265 non-external baseline errors are the same in both trees (unresolved external crates and the missing `serde` attribute under other wordings).

Sensitivity: in a throwaway copy, removing `use ...::Format;` from the tree source and `use super::verbs;` from the command tree produced 94 new errors (82 x `E0433 cannot find module or crate verbs`, plus `Format` ones), so a missing import or module name is caught. The oracle cannot see trait-in-scope and field/method privacy, hence 4.2.

Wasm: the crate builds only natively (binaries over `std::fs` and `std::process`, no wasm consumer, no `cfg(target_arch)`); the `cfg(unix)` / `cfg(not(unix))` branch in `verbs` is unchanged and the Windows target is not installed here, so it was not analysed.

### 4.2 Definition and visibility scans

- `use_vis_scan.py`: every name imported from a carved sibling crate (51 imports in 9 files) is a top-level `pub` item of the owning crate: 0 violations. Seeded: making `go_sprint` and `tree_filter` `pub(crate)` again in a copy gives 2 violations.
- `vis_scan.py` (non-`pub` fields and methods of lower crates that a dependent mentions): 16 candidates, all reviewed and benign (`From` trait impls, `Flag::parse` which dependents mention only as `str::parse`, `Profile::parse`, `Format::parse` and the free `parse`; `TreeSource` and `Repository` trait impl methods; `RepoRepository.root`, `.profile` and `.context` which dependents mention only as `invocation.root()`, local variables or `invocation.context()`). Seeded: `FlagValue::projection` back to `pub(crate)` is reported against `run-and-projection`.
- Traits in scope, by hand (the oracle cannot check them): the facade's old private `use` brought `RepoContext` and `Write` to every child. Method calls on concrete `FsRepoContext` values need `RepoContext` in `verbs` (`context.goals()`, `.tickets()`, `.file_move()`, ...), which the first draft had dropped; it is imported now (found by this audit, not by the oracle). `dyn Write` receivers need no import (confirmed with a rustc probe), `command_framework` names `dyn Write` and imports it, `run-and-projection` implements `Write` and imports it.
- `unused_imports.py`: every external import in the seven files is used (the two trait imports `RepoContext` are used through method calls).

### 4.3 Isolated per-crate-root analysis (`per_root.py`, `🗑️generated/rcw4-oracle-perroot.log`)

Each of the six lib roots analysed alone, native and `--test`: the set of unresolved external crates equals the manifest's `[dependencies]` in all 12 runs (event-stream 5, command-framework 4, tree-source 12, mcp-verb 10, verbs 22, facade 14), no undeclared use, no unused declaration, 0 other errors. Output: `OK`.

### 4.4 Static checks (`verify.ts`, `🗑️generated/rcw4-verify-static.log`: `ALL STATIC CHECKS PASSED`)

All six `Cargo.toml` parse under Bun's TOML; names match; `package.workspace` resolves to the repo root; `[lib] path` equals `<node>/🦀️.rs`; every path dependency exists with the expected package name; boundary keys exact; owner rows match name, root, manifest and exist; the cli members are byte-sorted and contiguous with sorted neighbours; the 30 routes exist exactly once in `launch.json` and in the seed with the exact command, group, type, cwd and key order; both JSON files parse (`launch.json` 9,141 configurations; the seed parses with `Bun.JSONC`, 6,173); every `#[path]` resolves, each mounted once. rustfmt (`rustfmt +nightly-2026-07-20 --edition 2021 --check --config-path rustfmt.toml --config skip_children=true`): exit 0 on all nine files and the binary, and no diff hunk touches a `use`, `mod` or `#[path]` line; several bodies show pre-existing line-wrapping debt (for example the event-stream summary `if`, which differs at HEAD too) that I left alone.

### 4.5 TS tests

`bun test ./💥️nested-cargo-collision-authority/🟦️.ts` 25 pass; `./🎯️cargo-target-discovery-skip/🟦️.ts` 2 pass; `./🚧️cargo-discovery-exclusions/🟦️.ts` 3 pass, 2 fail with `SEMIO_TEST_ARTIFACT_DIR required` (environment). `./🦑️repo-source-ownership/🟦️.ts` and `./🧱️cargo-dependency-direction/🟦️.ts` fail with `ENOENT` on `📚️library/🧫️fixtures/<case>/🔣️.json` (the tests read `🧫️fixtures/` instead of `🧪️tests/🧫️fixtures/`; the previous CLI result recorded the same path bug for the ownership test), before any assertion runs, so they say nothing about this change. Instead I checked the one CLI row by hand: the ownership fixture's anchor `pub mod repo_cli;` is present once in the lib root owner file.

## 5. Real compile attempts (start and end)

| Command | Result |
|---|---|
| `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-repo-cli --message-format=short` (baseline, before any edit; `🗑️generated/rcw4-baseline-check.log`) | `error: Finite original maintenance authority required` (cache-maintenance stage, before any rustc) |
| same, `-p semio-framework-repo-cli-event-stream` (after the carve; `rcw4-final-check-event-stream.log`) | the same error |
| `bun repo run @semio-tech/repo-cli-event-stream-rs:check --configuration=prototype --parallel=1 -- --lib --offline` (`rcw4-final-nx-check-event-stream.log`) | `Repository target execution contract refused` in `cargoTargets` of `📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs` |
| control: the same Nx route on the untouched `@semio-tech/repo-graphql-fs-rs` (`rcw4-control-nx-check-graphql-fs.log`) | the identical error, so it is the build infra, not these crates |
| `bun repo cargo ... -- update --workspace --offline` (`rcw4-lock-update-root.log`) | fails (launcher), `Cargo.lock` has no `semio-framework-repo-cli-*` entry |

So the gate verdict for the new crates and the real `check`/`test` are open. Census shows each closure passes the source boundary with `oversizedDeps 0`.

## 6. Lock file

Not refreshed (launcher down). The root `Cargo.lock` needs five new packages and a rewritten dependency list for `semio-framework-repo-cli`: `update --workspace --offline` once `bun repo cargo` works; expect the same peer-churn drift the previous result described.

## 7. Residual risks (first real compile)

1. Type-level facts the analysis oracle cannot reach because every external type is unresolved: trait-in-scope for calls on concrete external types (audited by hand, section 4.2), coherence, `pub`-in-private-module lints (`unreachable_pub` is not enabled in the workspace lints), field and method privacy beyond what the scans cover, and `unused_qualifications` warnings. The most likely first-compile findings are therefore a missing trait import in one of the five files or a `dead_code` warning for a `run_proj` item that no longer has an external user.
2. `use super::verbs;` in the command tree imports a `pub use <crate> as verbs` re-export of the facade; it is a path form the oracle resolved (the seeded removal fails), but only rustc can confirm the macro-free case end to end.
3. `Command`/`Invocation`/handler wiring across crates: the handlers are `pub fn` of the verbs crate (checked), `Action` is `fn(&mut Invocation<'_>) -> Result<(), CliError>` and unchanged.
4. Lock refresh and the Nx graph inference of the five new projects have not run (the Nx route fails for every crate right now).
5. Taxonomy validators were not run on the new folders; all folder names are existing RGI emoji folders (`📦️packages` with `🦀️rust` under existing nodes) and `📦️packages` is the only new child of each node, so no sibling emoji duplicate arises.
6. The Windows `cfg(not(unix))` branch in `verbs` is unchanged but unanalysed on this host.
7. No new tests were written: the eight language-agnostic cases (`🧭️command-parsing`, `🖨️render-formats`, `📖️usage-text`, `🔁️graphql-verb-roundtrip`, `🔌️mcp-verb-handshake`, `🧪️test-verb-planning`, `📤️export-verb-records`, `🪝️hook-verb-dispatch`) and their third-party references in `🔮️oracles` are the behavioural cover and still target the facade API; they have not been run either.
