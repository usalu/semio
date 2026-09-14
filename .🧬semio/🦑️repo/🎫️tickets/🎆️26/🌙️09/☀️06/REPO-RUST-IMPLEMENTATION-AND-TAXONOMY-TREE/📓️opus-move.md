# 📓️ Opus executor report — `🚚️move`

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move`.
Session: resumed after a rate limit killed the first executor mid-job (its last note was
"All 4 pass. Now the other four cases" — the 4 were the Rust crate's in-crate unit tests, the four
cases were the harness cases still missing a second implementation).

## 1. What the module is

A pure planning library. Every operation is a function `plan_*(&Workspace, …) -> Result<Plan,
MoveError>` over an **in-memory** `Workspace` (relative slash-separated path → `Entry::File(String)`
/ `Entry::Directory`, held in a `BTreeMap`). No planner ever writes: a `Plan` carries `Change`s,
`Message`s, `PlanEvent`s and counters, and is handed to an `Executor` trait. Two executors ship:
`Workspace` itself (in-memory, what every test uses) and `FileSystemExecutor` (a real directory
tree). The section grammar comes from the `🗣️languages` crate and is re-exported explicitly
(`Language`, `Section`, `parse_sections`) so a client of `move` never reaches past it — there is no
second parser anywhere in this module.

Public surface (`📦️packages/🦀️rust/🦀️.rs`, 1288 lines):
`Workspace`, `Entry`, `clean_relative`, `Change`, `Message`, `PlanEvent`, `Plan`, `MoveError`,
`Executor`, `execute`, `FileSystemExecutor`, `title_case_token`, `simple_upper`, `simple_lower`,
`apply_rename_casings`, `find_section`, `unique_strings`, `policy_section_start_match`,
`policy_section_end_match`, `split_header`, `merge_headers`, `extract_package`, `extract_imports`,
`format_imports`, `plan_rename`, `plan_folder_create`, `plan_folder_move`, `plan_folder_delete`,
`plan_file_create`, `plan_file_move`, `plan_file_delete`, `plan_agents_docs_path`,
`plan_agents_docs_removal`, `plan_section_create`, `plan_section_move`, `plan_section_delete`,
`plan_integrate`, `plan_extract`.

## 2. What this session added

Four **Go subject adapters** for the language-agnostic cases, so the recorded vectors are now proven
against two implementations instead of one:

- `🧪️tests/📑️section-move/🐹️.go`
- `🧪️tests/🧲️section-extract/🐹️.go`
- `🧪️tests/📥️file-integrate/🐹️.go`
- `🧪️tests/🚚️file-folder-move/🐹️.go`

They import `github.com/usalu/semio/repo/move` (plus `…/workspace`, `…/languages`, `…/model` for the
result and section types). The Go twin is filesystem-based (`ToolSectionMove`, `ToolExtract`,
`ToolIntegrate`, `ToolFileMove`, `ToolFolderMove` against the global `workspace.RootDir`), so each
adapter materializes the recorded workspace into a fresh `os.MkdirTemp` under the scenario work
directory, points `workspace.RootDir` at it, runs the tool and reads the tree back. `workspace.RootDir`
is assigned directly and **not** through `workspace.SetRootDir`, because that helper calls
`FindRepoRoot` and would walk up out of the scratch directory into the real checkout.
`events.Emit` is a no-op without `COMPOSE_SERVER_ADDR`, so nothing leaves the process.

Projections are byte-for-byte the shapes the Rust adapters already produced (`renamed`, `roundTrips`,
`refusals`, `extracted`, `arithmetic`, `integrated`, `readBack`, `moves`). Key order does not matter
under `ordered-json-v1` ("array order significant; key order never is"), array order does — every
array is built in vector order and directory listings are sorted, matching the Rust `BTreeMap` walk.

Launch entries added to `.vscode/🧩️launch.seed.jsonc` (after the `🪝️hooks` group, following the
existing naming) and regenerated into `.vscode/launch.json`:
`🧪️test🧰️repo🚚️move🦀️rust`, `🧪️test🧰️repo🚚️move🐹️go`, `🧪️test🧰️repo🚚️move🥒️parity`.

## 3. Verification — real output

```
$ RUSTC_WRAPPER="" cargo test -p semio-framework-repo-move
running 4 tests
test tests::casings_cover_upper_title_and_lower ... ok
test tests::workspace_applies_a_directory_move ... ok
test tests::rename_moves_deepest_paths_first ... ok
test tests::section_move_rewrites_both_markers ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ RUSTC_WRAPPER="" cargo clippy -p semio-framework-repo-move --all-targets
    Checking semio-framework-repo-move v0.1.0 (…/🚚️move/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1.95s
```

(clippy emits no diagnostic for this crate; the warnings printed in the same run belong to
`semio-framework-repo-identity` and `semio-framework-repo-workspace`, other owners' crates.)

```
$ SEMIO_TEST_BUDGET_MS=600000 bun ./…/🧪️test/📜️script.ts subject fundamental --owner 🚚️move --implementation rust
[test] level=fundamental cases=5 executed=16 passed=16 failed=0 errored=0 parity=0/0

$ … subject fundamental --owner 🚚️move --implementation go
[test] not-exercised …/🔤️rename-casings (recorded no-oracle decision repo-move-token-casings …)
[test] level=fundamental cases=5 executed=12 passed=12 failed=0 errored=0 parity=0/0 not-exercised=1

$ … oracle fundamental --owner 🚚️move
[test] not-exercised …/🚚️file-folder-move (recorded no-oracle decision repo-move-paths …)
[test] not-exercised …/📥️file-integrate (recorded no-oracle decision repo-move-sections …)
[test] not-exercised …/🔤️rename-casings (recorded no-oracle decision repo-move-token-casings …)
[test] not-exercised …/🧲️section-extract (recorded no-oracle decision repo-move-sections …)
[test] not-exercised …/📑️section-move (recorded no-oracle decision repo-move-sections …)
[test] level=fundamental cases=5 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=5

$ … parity fundamental --owner 🚚️move
[test] level=fundamental cases=5 executed=28 passed=28 failed=0 errored=0 parity=12/12
```

`contract --owner 🚚️move` reports **no breach naming this module**; the breaches it prints are
repo-wide and belong to `🗒️note` fixtures and the `testing/discovery` baselines.

The `oracle` phase is empty by design: `🔮️oracle/🔣️.json` declares no oracles and three
`noOracleDecisions` (`repo-move-token-casings`, `repo-move-sections`, `repo-move-paths`), because
region markers, entity-emoji section names, the `AGENTS.md` heading index and the exact three-fold
casing rewrite are this repository's own grammar and no third party shares that definition. The
substitutes are recorded vectors plus metamorphic laws (the round-trip scenarios), and the twelve
cross-subject pairs above are the differential evidence that has now been added on top.

## 4. Open — a real Go/Rust divergence, deliberately not papered over

`🔤️rename-casings` has **no Go adapter**, and this is not an oversight.

1. The workspace-level rename does not exist in `github.com/usalu/semio/repo/move`. The Go package
   carries only `ApplyRenameCasings` (content-level); the walk lives in
   `🔨️modules/⌨️cli/📦️packages/🐹️go/🐹️.go` as `cli.ToolRename` (line ~3711). Per plan §2 the
   rename belongs in `🚚️move`; `⌨️cli` is owned by the concurrent `go-split` agent and I did not
   edit it.
2. Even importing `cli.ToolRename` from the adapter would not pass, because the two implementations
   genuinely disagree on a **scoped** rename. Probe:
   `$TICKET/🗑️generated/move/probe/main.go` (run with `GOWORK=off GOFLAGS=-mod=mod go run .`),
   vector `scoped-to-one-directory` (`old=model new=shape scope=model`):

   | | Rust `plan_rename` / recorded vector | Go `cli.ToolRename` |
   | --- | --- | --- |
   | scope root directory | kept as `model` | renamed to `shape` |
   | `foldersRenamed` | 0 | 1 |
   | message | `…: 2 files edited, 2 files renamed, 0 folders renamed` | `…: 2 files edited, 2 files renamed, 1 folders renamed` |

   Cause: `ToolRename` walks from `RootDir/<scope>` but computes `rel` against `RootDir`, so the
   scope root itself is `rel == "model"` rather than `"."` and enters the rename list. The scope you
   name therefore disappears. The Rust implementation and the recorded vectors keep it.

   I consider the Go behaviour the defect and left the Rust behaviour and the fixture as they are.
   This is a decision the coordinator may want to overturn; if the Go behaviour is declared
   canonical, `🧫️fixtures/🔤️rename-vectors.json` (`scoped-to-one-directory`, and
   `only-a-filename-changes` whose scope root `docs` happens not to contain the token) and
   `plan_rename` both change.

   Registration is all-or-nothing (`validateRegistration` in the harness rejects an adapter that
   leaves a planned scenario unregistered), so a partial Go adapter for the three agreeing scenarios
   is not possible either.

3. Follow-up for whoever moves the rename: once `ToolRename` lives in `repo/move` and the scope-root
   question is settled, the adapter is a mechanical copy of `🧪️tests/🚚️file-folder-move/🐹️.go`
   (same `materialize`/`render`/`expect` helpers) with `stats` read from
   `result.Data.(map[string]int)`.

## 5. Other things found, not fixed (not this module's files)

- `📦️packages/🐹️go/🔬️_test.go` (written by `go-split`) fails 5 of its tests:
  `TestSectionListCommand`, `TestDefinitionListCommand`, `TestSectionListIDs`, `TestToolSectionList`,
  `TestToolDefinitionList` — all `File not found: repo/client/main.go` / `compose/js/index.ts`, i.e.
  the godfile-era tests still point at paths the taxonomy move deleted. Pre-existing, untouched by
  this session, and in a file the `go-split` agent owns. `go build` and `go vet` of the package are
  clean.
- The Rust crate's own `[lints] workspace = true` clippy run is clean, but the shared crates
  `identity` (2) and `workspace` (1) emit clippy warnings that surface in any `-p …-move` run.

## 6. Files created or changed by this session

Created:
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/🧪️tests/📑️section-move/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/🧪️tests/🧲️section-extract/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/🧪️tests/📥️file-integrate/🐹️.go`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/🧪️tests/🚚️file-folder-move/🐹️.go`
- `$TICKET/🗑️generated/move/probe/{main.go,go.mod}` (the scoped-rename divergence probe)
- `$TICKET/📓️opus-move.md` (this file)

Changed:
- `.vscode/🧩️launch.seed.jsonc` (three `🚚️move` test entries)
- `.vscode/launch.json` (regenerated by
  `bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate`)

Untouched on purpose: everything under `📦️packages/🐹️go/` and `⌨️cli` (owned by `go-split`), and
the module's fixtures, features, schema, oracle manifest and Rust crate, all of which were already
complete and green when this session resumed.
