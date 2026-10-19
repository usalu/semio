# Result rs-sweep: static Rust resolution sweep of every crate touched since 2026-10-19 00:00

Executor `rs-sweep` (Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo/nx/`bun repo`). `C` = this ticket folder. Tools: `C/rs-sweep-tools/`. Run output and first-touch backups: `C/🗑️generated/rs-sweep-run/` and `C/🗑️generated/rs-sweep-x/`. HARD FREEZE respected: none of the 891 edited files is under `📚️library/`, a root `📜️script.ts`, a `package.json`, a `🔣️taxonomy.json` or `.vscode/` (`🗑️generated/rs-sweep-x/changed-files.txt`, grep: 0 hits).

## 0. Verdict

The checker found **11,501 definite resolution defects in 386 of 830 touched packages** (tool-final semantics, state before my first edit). After the automated and manual root-cause fixes **1,622 remain in 169 packages**; 217 packages are clean. Nothing was compiled by cargo (blocked), so every number is static. The checker is validated against rustc on 16 fixture cases (section 3) and the rustfmt syntax gate is clean on all 10,843 Rust files of the touched closures.

The remaining 1,622 are structural: items that moved to other crates through the rg-* splits with no unique or acyclic new home (section 6). They need an owner decision, not a mechanical edit.

## 1. Scope and method

* Scope: 830 packages whose manifest, root or any module-closure file (`mod`, `#[path]`, `include!`) has an mtime at or after 2026-10-19 00:00 (list: `🗑️generated/rs-sweep-run/scope.txt`). That includes all crates of rg-core, rg-refused, rg-plug, rg-fem3d, rg-rest, rg-test, split-s, split-fw, tco2, vcs-bound, fixups3/4, rust-mounts (git diff plus untracked new crates are covered by mtime; the auto-commit dates are fake).
* Parser: tree-sitter-rust (`web-tree-sitter` 0.20.8 from `node_modules` plus `tree-sitter-rust.wasm` from `tree-sitter-wasms` 0.1.13 found in the tool cache; nothing installed, `findWasm` searches `node_modules` first). Grammar gaps (`unsafe extern`, async closures) hit 76 files; each is cross-checked with `rustfmt +nightly-2026-07-20` (all parse), the affected modules are marked macro-opaque and skip the duplicate checks.
* Module tree per target (lib, bin, `[[test]]`, bench, example): `mod`, `#[path]` (string escapes), `include!`; `include!`d files resolve nested `mod` against the included file (verified with rustc, fixtures `ok_include_mod` and `e0583_include_mod_wrong_dir`), crate-root `extern crate` aliases join the extern prelude, `#[macro_export]` macros live at the root, item-level macro invocations make a module opaque (names declared as `enum|struct|fn ... Name` inside them are collected).
* Environments: lib host, lib test, lib wasm32 (only for crates with wasm-only cfg or wasm-only dependencies); a path is reported when it fails in an environment where it is active. cfg is three-valued; duplicates use a feature-assignment enumeration so `cfg(x)` / `cfg(not(x))` pairs never collide. Dependency crates are looked up with `cfg(test)` off and only `pub` items reachable through `pub` modules, `pub use` and globs are visible.
* Checks (all of the brief): first segment of every `use` (and every `crate::`/`super::`/`self::`/extern-crate path in bodies, macros included) resolves to a module-scope name, extern prelude (declared deps, dev-deps per env, target-specific deps, `extern crate` aliases) or std; every path exists as module or item; privacy of each hop including cross-crate `pub` and private imports; duplicates per module and namespace (items, imports, impls, inherent methods); `include_str!/include_bytes!/include!` targets; Cargo: parse, `[lib]/[[bin]]/[[test]]/[[bench]]/[[example]]/build` paths and duplicate target names, dependency path exists, package name equals key or `package`, `workspace = true` entry exists, dependency has a lib, requested dependency features exist, `[features]` entries (`dep:`, `x/f`, `x?/f`) reference declared deps/features, `required-features`, `cfg(feature)` in source against declared features, workspace membership ("believes it's in a workspace when it's not"), workspace members/`workspace.dependencies` targets, duplicate member names, path dependency inside another workspace root.

## 2. Findings by kind (before and after, same tool version)

| kind | before | after |
| --- | ---: | ---: |
| cfg-feature-undeclared | 26 | 0 |
| duplicate-impl | 16 | 3 |
| duplicate-method | 6 | 6 |
| duplicate-name | 304 | 5 |
| feature-dep-undeclared | 1 | 0 |
| missing-extern-alias | 269 | 1 |
| mod-missing | 2 | 0 |
| private-item | 340 | 6 |
| undeclared-dependency | 2645 | 43 |
| unresolved-import | 1551 | 648 |
| unresolved-path | 3780 | 660 |
| unresolved-qualifier | 2561 | 250 |

Other counters: 1.10 M paths resolved; 9.5 k paths not judged because the module has item-level macros that may define the name.

## 3. Verification (exact commands, outcome)

* `bun C/rs-sweep-tools/selftest.ts`: 16 of 16 fixture cases agree with rustc (`rustc +nightly-2026-07-20 --emit=metadata --error-format=json`): E0432 (x2), E0603 (x3), E0433 (x2), E0428, E0252, E0255, E0119, E0592, E0583 (x2) and two clean controls (`C/rs-sweep-tools/fx/`). This is the language-agnostic fixture test plus the third-party (compiler) cross-check required by AGENTS.
* `bun C/rs-sweep-tools/main.ts --scope C/🗑️generated/rs-sweep-run/scope.txt --syntax --tag final`: rustfmt syntax gate over 10,843 Rust files of the touched closures: 0 syntax errors; 1,622 findings; checker stable after the last edits.
* `RS_SWEEP_BEFORE=1 ... --tag beforefinal`: replays the pre-edit tree by reading first-touch backups (caveat: the 48 files edited by two early passes hold a backup taken after the first pass, so before is slightly undercounted).
* NOT run: any cargo build or check (blocked), `cargo metadata`, tests of the edited crates. Wrong-entity risk of the unique-name relocation is mitigated (section 4) but not compiler-proven.

## 4. Fixes applied (root causes, all outside the frozen paths)

Tools: `fixdeps.ts` (dependencies and `extern crate` aliases), `fixfeat.ts` (features), `repoint.ts` (source edits; planned from the checker's own errors, re-checked after every pass). Every edited file is backed up at first touch in `🗑️generated/rs-sweep-x/backup/` (`ledger.json` maps path to backup name; list: `changed-files.txt`, 891 files: 222 manifests, 669 Rust files). Per-pass logs: `repoint-applied-*.log`.

1. **Missing dependencies** (the largest class, about 2.6 k findings): about 400 dependency lines added over 222 manifests, as `workspace = true` when the package's workspace has the entry, else a path (or the repo's most common registry version); normal vs dev section by environment of use; refused when a normal-dependency cycle would result or when the target is a member of another, non-excluded workspace (the value-fix agent hit this once: `semio-framework-value-snapshot-read` got `artifact-reference` and `os-io` from a mounted os/store test file; they were removed by value-fix and the guard now prevents it).
2. **Lost `extern crate` aliases** (`store`, `protocol`, `vcs`, `pack`, ...): added at the target root following the repo convention; `ui_tui`-style `use X as alias` bindings restored from the majority convention when 80 % of bindings elsewhere agree.
3. **Moved items** (about 1,300 path rewrites): paths whose item now lives in another module or crate are rewritten to the path of the unique entity of that name (same crate, direct dependencies, then every crate), validated by the resolver to reach the same entity; weak or generic names (`new`, `tree`, `Chute`), `Type::Variant` forms and ambiguous names are never rewritten (bad rewrites of the first pass, 33 variant to struct rewrites, were reverted).
4. **Unimported qualifiers**: about 280 `use` lines inserted where `Name::` was used without import and the entity is unique (std types from a fixed table).
5. **Private chain**: 340 privacy errors fixed by widening the definition or re-export to `pub` (cross-crate) or `pub(crate)` (same crate).
6. **Duplicates**: about 190 duplicate or redundant imports dropped, 6 byte-identical duplicate items removed, empty conflicting impl blocks removed (puzzle 5d `ColdMutationKind` x10), the fem 3d `guards` file that held two concatenated versions reduced to the newer one (identity checks), the energy editor unit test that held the same 43 lines twice reduced, `pub pub mod` (2 files, pre-existing) repaired.
7. **Renames**: `inverse_fem3d_mutation_cold` and `inverse_fem2d_mutation_cold` renamed to the name 255 importers use (definition only; no other user of the `_cold` name exists).
8. **Features and manifests**: 23 undeclared features declared as `name = []` (cfg(feature) was dead code; many are mirror/test crates), `semio-framework-ui-tui-unit-laws` moved `ui-tui-terminal` from dev to normal dependencies (a feature may not enable a dev-dependency).

## 5. Pre-existing defects found (reported, mostly fixed above)

`pub pub mod` in plugin `♻️cancellation` and store `📬️one-item-owners` (syntax; fixed), duplicate `use` of the same crate in `plugin ⏪️time-travel` (fixed), duplicated 200-line blocks in fem 3d `guards` and energy editor tests (fixed), 255 stale importers of the `_cold` inverse (fixed), `ReplacePartGrip`-style empty duplicate `impl` blocks in puzzle 5d mutations (fixed).

## 6. Open items (1,622 findings; owner decision needed)

By crate: see the table below; by cause:

* **puzzle-3d editor family (about 250)**: `crate::editor::puzzle3d::commands::*` (114 `use`) and `window_ownership`/`standards` paths do not exist in the lib any more. The items moved to `commands-*`, `editor-modes`, `editor-session` crates, and adding the dependency would create cycles (`puzzle-3d-editor-snapshot -> editor-modes` 14, `puzzle-3d-editor <- commands-runtime`). Needs the surface layout decision of the splitter.
* **Companion test crates** `plugin-tests-05/-11-2/-12-2/-14-2/-16/-01` (about 270): `crate::app_testing::{artifact_app_laws, mutation_fixture::job_close}` and `reactor_core_test_hosts` now live in other companions (`tests-04-2` mounts `testing-laws/unit-1`, `tests-03`); no companion depends on the host of the fixtures. They need either a fixtures support crate or dependencies between companions.
* **Dependency cycles refused** (4): `host-winit -> ui-engine-tests`, `plugin-retained-command -> plugin-testing-laws`, `puzzle-2d-v1 -> puzzle-2d-unit-tests`, `puzzle-3d-editor-snapshot -> puzzle-3d-editor-modes`; plus `actor -> pack-json` (2) and `program-bridge -> js-sys` (2, registry crate without a version in the workspace).
* **Names defined by macros** that rustc sees but item-level macro analysis does not: `patch/row_patch/record_patch` of `energy-model-diff-patch` (6 private-import errors), `bounded_first_step_tool_proofs`, `machine::Machine` (`statechart!`).
* **Ambiguous mirror copies**: names defined in several mirror/test crates (`TOOL_ID`, `WINDOW_KIND_ID`, `mutations`, `Puzzle2dMutation`, `Puzzle5dKindWeights`, `ToolRunMemberFoldRefusal`, `PendingElementBuildStage`, `ArtifactStorePreparedBinding`, `typed_operation_diagnostics`, `history_fold_refusal_controlled`, `live_visual`, `app_fixture`) have no single new home; the sweep refuses to guess.
* **Residual duplicates**: `impl MutationKind` for `CommitSpaceCheckpoint`/`RemoveSpaceCheckpoint` (two non-empty impls each, `store/🧬️schema/🧬️mutations`), `ColdDiffAlgebra` for `Puzzle5dWindowTransientDiff`, six identical-name methods of `MountedTypedCommandFullOperation` in two `producer` impl blocks, `durable_group_test_outcome` (two different functions, one public generic, one private), `retained` (two different modules both re-exported at the root of `semio-framework-replication`, `format::retained` and the mutation-retained crate; present at HEAD already), `FemSparse test_operation`-style leftovers are fixed.
* `missing-extern-alias` 1: `machine` in puzzle-3d-editor transform-tool test (`extern crate self as machine` convention).
* Cargo.lock: roughly 222 manifests changed; every affected workspace lock needs `cargo update --workspace --offline` (lockref owner) before any `--locked` step.

Rerun: `bun C/rs-sweep-tools/main.ts --scope C/🗑️generated/rs-sweep-run/scope.txt --tag NAME [--syntax]` (about 90 s, 4 min with the syntax gate; first run regenerates the file list), `--repoint [--apply]` plans/applies the source fixes, `bun C/rs-sweep-tools/fixdeps.ts FINDINGS.jsonl [--apply]`.

## 7. Findings by crate (before / after)

| crate | before | after |
| --- | ---: | ---: |
| `semio-s-artifact-puzzle-3d-mutations` | 932 | 14 |
| `semio-s-artifact-puzzle-3d-editor` | 730 | 214 |
| `semio-framework-plugin-tests-01` | 352 | 8 |
| `semio-framework-pets-staging` | 348 | 12 |
| `semio-s-artifact-puzzle-2d-v1` | 252 | 33 |
| `semio-framework-pets-staging-draft` | 210 | 4 |
| `semio-s-artifact-fem-2d-mutations` | 197 | 0 |
| `semio-s-artifact-fem-2d-editor` | 190 | 21 |
| `semio-s-artifact-fem-3d-editor` | 174 | 18 |
| `semio-s-artifact-puzzle-3d-host` | 166 | 0 |
| `semio-s-artifact-puzzle-5d-dispatch` | 162 | 0 |
| `semio-framework-os-store-replay` | 156 | 0 |
| `semio-s-artifact-puzzle-5d-interaction` | 146 | 8 |
| `semio-s-artifact-draw-drawing-editor` | 142 | 10 |
| `semio-s-artifact-fem-3d-mutations` | 135 | 0 |
| `semio-s-artifact-fem-3d-model` | 132 | 0 |
| `semio-framework-plugin-tool-run` | 129 | 26 |
| `semio-s-fem-analyses` | 109 | 25 |
| `semio-framework-plugin-child-group-wire-laws` | 98 | 3 |
| `semio-framework-plugin-reactor-turn` | 96 | 6 |
| `semio-s-artifact-stdio-dwg-snapshot` | 96 | 6 |
| `semio-s-artifact-procedural-2d-editor` | 86 | 37 |
| `semio-framework-os-store-engine-stage` | 79 | 36 |
| `semio-framework-plugin-tests-05` | 78 | 34 |
| `semio-framework-os-store-codec-registry` | 76 | 3 |
| `semio-s-artifact-puzzle-5d-tests` | 76 | 67 |
| `semio-framework-pack-record-encode` | 73 | 10 |
| `semio-s-fem-static` | 71 | 2 |
| `semio-framework-os-mcp-probe-document` | 70 | 0 |
| `semio-framework-plugin-time-travel` | 70 | 0 |
| `semio-framework-replication-mutation-retained` | 70 | 6 |
| `semio-framework-os-store-durable-group-tests` | 69 | 12 |
| `semio-framework-plugin-tests-14-2` | 67 | 52 |
| `semio-framework-ui-tui-engine` | 66 | 8 |
| `semio-framework-value-native-owned` | 65 | 0 |
| `semio-framework-os-renderer-wgpu-frame-job` | 64 | 0 |
| `semio-framework-ui-tui-core` | 61 | 0 |
| `semio-framework-os-config-tests-2` | 60 | 0 |
| `semio-framework-plugin-tests-16` | 60 | 35 |
| `semio-s-artifact-cad-cad` | 60 | 18 |
| `semio-framework-os-kernel-db-storage` | 58 | 0 |
| `semio-s-artifact-fem-3d-editor-results-window` | 58 | 22 |
| `semio-s-artifact-procedural-generation3d` | 57 | 8 |
| `semio-s-artifact-stdio-docx-base-model` | 57 | 0 |
| `semio-framework-plugin-time-travel-turns` | 56 | 0 |
| `semio-framework-os-history` | 55 | 0 |
| `semio-framework-plugin-retained-command` | 54 | 34 |
| `semio-framework-os-command-tests` | 53 | 0 |
| `semio-s-artifact-energy-model-editor` | 53 | 25 |
| `semio-s-artifact-puzzle-2d-mutations` | 53 | 9 |
| `semio-framework-job` | 52 | 2 |
| `semio-framework-os-store-child` | 52 | 0 |
| `semio-framework-plugin-tests-12-2` | 52 | 31 |
| `semio-framework-repo-dashboard-terminal` | 52 | 10 |
| `semio-framework-os-config-tests-1` | 51 | 0 |
| `semio-framework-plugin-host-test-component` | 50 | 2 |
| `semio-s-artifact-draw-drawing-editor-interaction` | 50 | 8 |
| `semio-s-artifact-fem-2d-editor-interaction` | 50 | 3 |
| `semio-s-artifact-layout-layout-editor` | 49 | 6 |
| `semio-s-artifact-puzzle-2d-tests` | 49 | 0 |
| `semio-framework-os-config` | 46 | 2 |
| `semio-framework-plugin-instance-stores` | 46 | 0 |
| `semio-framework-plugin-tool-run-turns` | 46 | 0 |
| `semio-s-artifact-puzzle-2d-mut-any-5` | 46 | 42 |
| `semio-framework-os-store-initialization-runtime` | 45 | 0 |
| `semio-framework-plugin-tests-04-2` | 44 | 0 |
| `semio-framework-plugin-tests-17` | 44 | 2 |
| `semio-s-artifact-puzzle-2d-retained` | 44 | 0 |
| `semio-framework-plugin-instance-dispatch` | 43 | 1 |
| `semio-framework-value-tests` | 43 | 10 |
| `semio-framework-os-store-operation-wire` | 42 | 2 |
| `semio-framework-plugin-testing-laws` | 42 | 0 |
| `semio-s-artifact-fem-2d-model` | 42 | 0 |
| `semio-s-artifact-puzzle-5d-puzzle5d` | 42 | 12 |
| `semio-s-artifact-puzzle-5d-retained` | 42 | 3 |
| `semio-s-artifact-stdio-docx-strict` | 42 | 1 |
| `semio-s-artifact-puzzle-2d-text-inverse` | 41 | 6 |
| `semio-s-artifact-energy-model-mut-window-layer-optics` | 40 | 40 |
| `semio-hub-inference-contract-laws` | 38 | 14 |
| `semio-s-artifact-raster-raster-editor` | 38 | 6 |
| `semio-framework-os-kernel-db-artifact` | 36 | 0 |
| `semio-framework-plugin-tests-06` | 36 | 4 |
| `semio-framework-plugin-tests-06-2` | 36 | 0 |
| `semio-framework-replication-history-index` | 36 | 12 |
| `semio-framework-value-registry` | 36 | 0 |
| `semio-framework-plugin-decl-registry` | 34 | 0 |
| `semio-framework-plugin-tests-11-2` | 34 | 31 |
| `semio-framework-value-snapshot-read` | 34 | 0 |
| `semio-s-artifact-energy-model-snapshot` | 34 | 0 |
| `semio-s-artifact-stdio-docx` | 34 | 11 |
| `semio-framework-os-kernel-neural-engine-model` | 33 | 0 |
| `semio-framework-plugin-reactor-app` | 33 | 0 |
| `semio-framework-ui-tui-laws` | 32 | 12 |
| `semio-s-artifact-procedural-3d-editor` | 32 | 8 |
| `semio-s-artifact-puzzle-5d` | 32 | 18 |
| `semio-s-artifact-sequence-sequence-v1` | 32 | 8 |
| `semio-framework-os-renderer-wgpu-kernel-pool` | 31 | 0 |
| `semio-framework-os-store-ephemeral` | 31 | 0 |
| `semio-framework-os-store-one-item-publication` | 31 | 0 |
| `semio-framework-plugin-decl-views` | 31 | 0 |
| `semio-framework-os-kernel-db-storage-fs` | 30 | 0 |
| `semio-framework-os-renderer-wgpu-host-winit` | 30 | 14 |
| `semio-framework-os-renderer-wgpu-program-bridge` | 30 | 24 |
| `semio-framework-plugin-surface-fixture` | 30 | 0 |
| `semio-framework-plugin-testing-acceptance` | 30 | 0 |
| `semio-framework-ui-wgpu-hosting` | 30 | 0 |
| `semio-s-artifact-architect-program-panels` | 30 | 0 |
| `semio-framework-os-renderer-wgpu-media-slots` | 29 | 0 |
| `semio-framework-plugin-tests-02` | 29 | 0 |
| `semio-framework-tool-run-tick-wire` | 29 | 0 |
| `semio-s-artifact-puzzle-5d-catalog` | 28 | 14 |
| `semio-s-artifact-reasoning-wires-snapshot` | 28 | 0 |
| `semio-framework-os-kernel` | 27 | 3 |
| `semio-framework-os-renderer-wgpu-native-probe` | 27 | 3 |
| `semio-s-artifact-fem-3d-editor-interaction` | 27 | 0 |
| `semio-s-artifact-stdio-docx-base` | 27 | 1 |
| `semio-s-artifact-stdio-semio-value-mutations` | 27 | 7 |
| `semio-framework-os-store-durable-group` | 26 | 2 |
| `semio-hub-space-foundation` | 26 | 0 |
| `semio-framework-pack-json` | 25 | 0 |
| `semio-s-artifact-fem-3d-editor-session` | 25 | 8 |
| `semio-s-artifact-norm-din18599-checks` | 25 | 25 |
| `semio-framework-os-kernel-db` | 24 | 0 |
| `semio-framework-os-kernel-db-io-driver` | 24 | 0 |
| `semio-framework-os-store-engine-owners` | 24 | 0 |
| `semio-framework-plugin-instance-jobs` | 24 | 0 |
| `semio-framework-server` | 24 | 24 |
| `semio-s-artifact-fem-2d-engine` | 23 | 0 |
| `semio-s-plugin-energy-engine-electrical` | 23 | 0 |
| `semio-framework-os-renderer-wgpu-host-plumbing` | 22 | 0 |
| `semio-framework-plugin-tests-03` | 22 | 2 |
| `semio-framework-plugin-tests-09-2` | 22 | 6 |
| `semio-framework-os-renderer-wgpu-asset-decode` | 21 | 0 |
| `semio-framework-os-renderer-wgpu-kernel-protocol` | 21 | 6 |
| `semio-framework-actor` | 20 | 2 |
| `semio-framework-job-worker` | 20 | 0 |
| `semio-framework-plugin` | 20 | 6 |
| `semio-framework-plugin-testing-fixtures-publication` | 20 | 0 |
| `semio-framework-ui-tui-terminal` | 20 | 0 |
| `semio-s-artifact-puzzle-2d-connect-handles-preparation` | 20 | 0 |
| `semio-framework-os-kernel-db-engine` | 19 | 0 |
| `semio-framework-tool-run` | 19 | 0 |
| `semio-s-artifact-flow-flow-viewer` | 19 | 1 |
| `semio-s-artifact-sequence-sequence-tests` | 19 | 19 |
| `semio-framework-plugin-decl-app` | 18 | 0 |
| `semio-framework-plugin-instance-app` | 18 | 0 |
| `semio-framework-plugin-tests-05-2` | 18 | 0 |
| `semio-framework-replication` | 18 | 4 |
| `semio-s-artifact-fem-3d-viewer` | 18 | 4 |
| `semio-s-artifact-wfc-bitmap-inferences` | 18 | 0 |
| `semio-s-plugin-energy-engine-sim-unit-tests` | 18 | 1 |
| `semio-framework-plugin-testing-fixtures` | 17 | 0 |
| `semio-framework-plugin-tests-13` | 17 | 0 |
| `semio-framework-replication-wire-core` | 17 | 0 |
| `semio-s-artifact-fem-2d-results-state` | 17 | 0 |
| `semio-s-plugin-energy-engine-sim` | 17 | 0 |
| `semio-framework-os-kernel-db-cluster` | 16 | 0 |
| `semio-framework-os-renderer-wgpu-runtime-mailbox-tests` | 16 | 1 |
| `semio-framework-plugin-window-config` | 16 | 6 |
| `semio-framework-ui-tui-widgets` | 16 | 0 |
| `semio-framework-value` | 16 | 0 |
| `semio-s-artifact-energy-model-editor-results` | 16 | 1 |
| `semio-framework-os-renderer-wgpu-frame-actions` | 15 | 0 |
| `semio-s-artifact-stdio-docx-transitional` | 15 | 1 |
| `semio-s-fem-analyses-assembly` | 15 | 10 |
| `semio-s-fem-analyses-model` | 15 | 9 |
| `semio-framework-os-history-schema` | 14 | 0 |
| `semio-framework-os-store-sync-tests-1` | 14 | 0 |
| `semio-framework-plugin-job` | 14 | 0 |
| `semio-framework-plugin-tests-10` | 14 | 0 |
| `semio-s-artifact-norm-iso16757-editor` | 14 | 4 |
| `semio-s-artifact-puzzle-3d-precompute` | 14 | 0 |
| `semio-s-artifact-stdio-deflate-examples` | 14 | 0 |
| `semio-s-artifact-wfc-3d-inferences` | 14 | 0 |
| `semio-s-artifact-wfc-grid2d-inferences` | 14 | 0 |
| `semio-framework-artifact-infinite-dag` | 13 | 0 |
| `semio-framework-os-store-space-history` | 13 | 0 |
| `semio-framework-action-bus` | 12 | 0 |
| `semio-framework-os-inference` | 12 | 0 |
| `semio-framework-os-kernel-db-engine-handle` | 12 | 0 |
| `semio-framework-os-store-tests-08` | 12 | 0 |
| `semio-framework-plugin-extension` | 12 | 0 |
| `semio-framework-plugin-runtime-ops` | 12 | 0 |
| `semio-framework-plugin-tests-07-2` | 12 | 0 |
| `semio-s-artifact-block-3d-schema` | 12 | 0 |
| `semio-s-artifact-fem-2d-boundary` | 12 | 0 |
| `semio-s-artifact-fem-3d-boundary` | 12 | 0 |
| `semio-s-artifact-flow-flow-editor` | 12 | 12 |
| `semio-s-artifact-norm-vdi3805-inferences` | 12 | 0 |
| `semio-s-artifact-puzzle-2d-unit-tests` | 12 | 3 |
| `semio-s-artifact-puzzle-5d-store-preparation` | 12 | 3 |
| `semio-s-artifact-wfc-2d-inferences` | 12 | 0 |
| `semio-s-plugin-energy-engine-precompute` | 12 | 0 |
| `semio-framework-os-mcp` | 11 | 4 |
| `semio-framework-plugin-tests-19` | 11 | 4 |
| `semio-framework-replication-format` | 11 | 0 |
| `semio-s-artifact-puzzle-2d-snapshot` | 11 | 3 |
| `semio-s-plugin-energy-engine-site` | 11 | 0 |
| `semio-framework-job-registry` | 10 | 0 |
| `semio-framework-os-renderer-wgpu-kernel-runtime` | 10 | 0 |
| `semio-framework-os-spr-laws-tests` | 10 | 0 |
| `semio-framework-os-store-durable-group-wire` | 10 | 2 |
| `semio-framework-os-vcs` | 10 | 0 |
| `semio-framework-pack-format` | 10 | 2 |
| `semio-framework-plugin-instance-children` | 10 | 10 |
| `semio-framework-plugin-tests-02-2` | 10 | 0 |
| `semio-hub-stdio-catalog` | 10 | 8 |
| `semio-s-artifact-draw-drawing-scene` | 10 | 0 |
| `semio-s-artifact-draw-drawing-scene-tests` | 10 | 10 |
| `semio-s-artifact-fem-3d-editor-model-window` | 10 | 0 |
| `semio-s-artifact-puzzle-2d-mut-any-10` | 10 | 0 |
| `semio-s-artifact-puzzle-3d-editor-modes` | 10 | 0 |
| `semio-s-artifact-puzzle-3d-retained-document` | 10 | 0 |
| `semio-s-artifact-puzzle-5d-mut-any-5` | 10 | 3 |
| `semio-framework-os-kernel-db-sync-laws` | 9 | 6 |
| `semio-framework-server-tests` | 9 | 9 |
| `semio-framework-os-run` | 8 | 0 |
| `semio-framework-os-store-canonical-json` | 8 | 0 |
| `semio-framework-os-store-command` | 8 | 0 |
| `semio-framework-os-store-one-item` | 8 | 0 |
| `semio-framework-pack-record-preflight` | 8 | 0 |
| `semio-framework-plugin-testing-caller` | 8 | 0 |
| `semio-framework-replication-mutation-messages` | 8 | 0 |
| `semio-hub-space-app` | 8 | 2 |
| `semio-s-artifact-fem-3d-editor-commands` | 8 | 8 |
| `semio-s-artifact-flow-flow-editor-leaves` | 8 | 8 |
| `semio-s-artifact-puzzle-2d` | 8 | 1 |
| `semio-framework-os-kernel-db-fault-testing` | 7 | 0 |
| `semio-framework-os-store-one-item-batch` | 7 | 7 |
| `semio-framework-plugin-tests-07-3` | 7 | 1 |
| `semio-framework-plugin-tests-08` | 7 | 0 |
| `semio-hub-stdio` | 7 | 7 |
| `semio-s-artifact-architect-program-architect` | 7 | 5 |
| `semio-s-artifact-fem-2d` | 7 | 1 |
| `semio-s-artifact-fem-3d` | 7 | 2 |
| `semio-s-artifact-puzzle-2d-schema` | 7 | 0 |
| `semio-fem-mutation-bridge` | 6 | 0 |
| `semio-framework-artifact-workflow-run` | 6 | 0 |
| `semio-framework-artifact-workflow-run-tests` | 6 | 0 |
| `semio-framework-io-control` | 6 | 0 |
| `semio-framework-os-infinite-board-schema` | 6 | 6 |
| `semio-framework-os-kernel-db-ids` | 6 | 0 |
| `semio-framework-os-kernel-db-sync` | 6 | 0 |
| `semio-framework-os-renderer-wgpu-browser-worker` | 6 | 0 |
| `semio-framework-os-services-tests` | 6 | 0 |
| `semio-framework-os-store-engine-core` | 6 | 6 |
| `semio-framework-os-store-ledger` | 6 | 6 |
| `semio-framework-os-store-tests-01` | 6 | 0 |
| `semio-framework-plugin-document-media` | 6 | 0 |
| `semio-framework-plugin-instance-commands` | 6 | 0 |
| `semio-framework-tool-run-tests` | 6 | 0 |
| `semio-framework-ui-host-tests` | 6 | 3 |
| `semio-framework-ui-tui-frame` | 6 | 0 |
| `semio-puzzle-mutation-bridge` | 6 | 0 |
| `semio-s-artifact-energy-model-diff` | 6 | 6 |
| `semio-s-artifact-energy-model-editor-edit` | 6 | 0 |
| `semio-s-artifact-fem-3d-editor-visual-job` | 6 | 0 |
| `semio-s-artifact-flow-flow-unit-tests` | 6 | 6 |
| `semio-s-artifact-puzzle-2d-interaction` | 6 | 3 |
| `semio-s-artifact-puzzle-5d-document-records` | 6 | 3 |
| `semio-s-artifact-puzzle-5d-mut-any` | 6 | 6 |
| `semio-s-artifact-puzzle-5d-mut-any-8` | 6 | 0 |
| `semio-framework-os-store-space-schema` | 5 | 4 |
| `semio-framework-plugin-composition-publication` | 5 | 0 |
| `semio-s-artifact-animate-presentation-animate` | 5 | 3 |
| `semio-s-artifact-fem-2d-editor-session` | 5 | 3 |
| `semio-s-artifact-fem-3d-mesh` | 5 | 0 |
| `semio-s-artifact-puzzle-2d-v1-editor-tests` | 5 | 0 |
| `semio-s-plugin-energy-epw` | 5 | 1 |
| `semio-framework-job-payload` | 4 | 0 |
| `semio-framework-os-flow-host-core` | 4 | 0 |
| `semio-framework-os-infinite` | 4 | 3 |
| `semio-framework-os-infinite-world-scene` | 4 | 0 |
| `semio-framework-os-kernel-db-io-registry` | 4 | 0 |
| `semio-framework-os-list-delta` | 4 | 0 |
| `semio-framework-os-renderer-wgpu-frame-tests-1` | 4 | 0 |
| `semio-framework-plugin-host-wake` | 4 | 0 |
| `semio-framework-plugin-interpreter-runtime` | 4 | 0 |
| `semio-framework-plugin-reactor-lifecycle` | 4 | 4 |
| `semio-framework-plugin-tests-15` | 4 | 0 |
| `semio-framework-value-factory` | 4 | 2 |
| `semio-s-artifact-cad-cad-play` | 4 | 0 |
| `semio-s-artifact-energy-model-session` | 4 | 0 |
| `semio-s-artifact-energy-model-viewer` | 4 | 2 |
| `semio-s-artifact-procedural-3d-geometry` | 4 | 4 |
| `semio-s-artifact-puzzle-5d-diff` | 4 | 0 |
| `semio-s-artifact-sequence-sequence` | 4 | 4 |
| `semio-s-artifact-stdio-deflate-schema` | 4 | 0 |
| `semio-s-artifact-stdio-docx-base-mutations` | 4 | 4 |
| `semio-s-artifact-writer-writer` | 4 | 3 |
| `semio-s-fem-app-surface` | 4 | 0 |
| `semio-s-fem-sparse-pcg` | 4 | 0 |
| `semio-s-plugin-energy-engine-kernel` | 4 | 0 |
| `semio-framework-artifact-workflow-workflow-schema` | 3 | 0 |
| `semio-framework-artifact-workflow-workflow-schema-laws` | 3 | 0 |
| `semio-framework-os-dsl-module-tests` | 3 | 0 |
| `semio-framework-os-host-space` | 3 | 0 |
| `semio-framework-os-renderer-wgpu-frame-presentation` | 3 | 0 |
| `semio-framework-os-renderer-wgpu-runtime-mailbox` | 3 | 3 |
| `semio-framework-os-renderer-wgpu-shell-tests-chrome-6` | 3 | 0 |
| `semio-framework-os-renderer-wgpu-shell-tests-docs-2` | 3 | 0 |
| `semio-framework-os-store-composition` | 3 | 3 |
| `semio-framework-os-store-document` | 3 | 3 |
| `semio-framework-os-store-engine` | 3 | 0 |
| `semio-framework-plugin-host` | 3 | 0 |
| `semio-framework-plugin-host-effects` | 3 | 0 |
| `semio-s-artifact-fem-2d-mesh` | 3 | 0 |
| `semio-s-artifact-puzzle-2d-engine` | 3 | 3 |
| `semio-s-artifact-puzzle-2d-history-edit-runtime-tests` | 3 | 0 |
| `semio-s-artifact-puzzle-2d-mut-any` | 3 | 0 |
| `semio-s-artifact-puzzle-3d` | 3 | 1 |
| `semio-s-artifact-puzzle-5d-kind-catalogs` | 3 | 0 |
| `semio-s-artifact-sourcing-curation` | 3 | 2 |
| `semio-framework-async` | 2 | 2 |
| `semio-framework-course-schema` | 2 | 0 |
| `semio-framework-os-flow-vcs` | 2 | 0 |
| `semio-framework-os-infinite-board-normal-fill` | 2 | 0 |
| `semio-framework-os-kernel-db-artifact-contract` | 2 | 2 |
| `semio-framework-os-kernel-db-artifact-engine` | 2 | 0 |
| `semio-framework-os-kernel-db-io-core` | 2 | 0 |
| `semio-framework-os-kernel-db-wal` | 2 | 0 |
| `semio-framework-os-kernel-db-wal-log` | 2 | 0 |
| `semio-framework-os-store-config` | 2 | 2 |
| `semio-framework-os-store-engine-live` | 2 | 2 |
| `semio-framework-os-store-space-member` | 2 | 2 |
| `semio-framework-os-store-testing-prelude` | 2 | 2 |
| `semio-framework-plugin-host-shard` | 2 | 0 |
| `semio-framework-plugin-reactor-context` | 2 | 0 |
| `semio-framework-print-tests` | 2 | 0 |
| `semio-framework-repo-dashboard-inventory` | 2 | 0 |
| `semio-framework-ui-host` | 2 | 0 |
| `semio-hub-raster` | 2 | 2 |
| `semio-hub-stdio-office` | 2 | 2 |
| `semio-s-artifact-cad-cad-editor` | 2 | 2 |
| `semio-s-artifact-energy-model-mutation-fixtures` | 2 | 2 |
| `semio-s-artifact-energy-model-mutations` | 2 | 0 |
| `semio-s-artifact-fem-3d-editor-panels` | 2 | 0 |
| `semio-s-artifact-flow-flow-mutations` | 2 | 0 |
| `semio-s-artifact-gis-gismap-gis2d` | 2 | 2 |
| `semio-s-artifact-gis-gismap-gis2d-unit-tests` | 2 | 1 |
| `semio-s-artifact-procedural-3d-geometry-compute` | 2 | 2 |
| `semio-s-artifact-puzzle-3d-editor-dispatch` | 2 | 0 |
| `semio-s-artifact-puzzle-3d-editor-snapshot` | 2 | 16 |
| `semio-s-artifact-puzzle-5d-diff-2` | 2 | 0 |
| `semio-s-artifact-puzzle-5d-host-admission-tests` | 2 | 0 |
| `semio-s-artifact-puzzle-5d-mut-any-3` | 2 | 0 |
| `semio-s-artifact-puzzle-5d-mut-any-4` | 2 | 0 |
| `semio-s-artifact-puzzle-5d-v1-unit-tests` | 2 | 1 |
| `semio-s-artifact-raster-raster-host-retirement-bounds` | 2 | 2 |
| `semio-s-artifact-sourcing-curation-tests` | 2 | 2 |
| `semio-s-artifact-stdio-tiff-mutation-regressions` | 2 | 2 |
| `semio-s-artifact-wfc-2d` | 2 | 2 |
| `semio-s-artifact-wfc-grid2d` | 2 | 0 |
| `semio-s-artifact-writer-writer-inferences` | 2 | 2 |
| `semio-s-artifact-writer-writer-tests-2` | 2 | 2 |
| `teaching-proctor-domain` | 2 | 2 |
| `semio-framework` | 1 | 0 |
| `semio-framework-3d-mesh-document` | 1 | 0 |
| `semio-framework-artifact-flow-flow-snapshot-laws` | 1 | 0 |
| `semio-framework-canvas-scene` | 1 | 0 |
| `semio-framework-job-publication` | 1 | 0 |
| `semio-framework-os-renderer-wgpu` | 1 | 1 |
| `semio-framework-os-semio` | 1 | 0 |
| `semio-framework-os-store-tests-04` | 1 | 1 |
| `semio-framework-plugin-decl-surface` | 1 | 0 |
| `semio-framework-plugin-interpreter` | 1 | 1 |
| `semio-framework-plugin-operation` | 1 | 0 |
| `semio-framework-plugin-publication-mounted` | 1 | 0 |
| `semio-framework-plugin-store-stage-laws` | 1 | 0 |
| `semio-framework-plugin-tests-03-2` | 1 | 0 |
| `semio-framework-ui-tui-unit-laws` | 1 | 0 |
| `semio-framework-value-edit` | 1 | 1 |
| `semio-framework-value-retirement` | 1 | 0 |
| `semio-framework-value-schema` | 1 | 0 |
| `semio-s-artifact-energy-model` | 1 | 0 |
| `semio-s-artifact-flow-flow-original-command-retirement-tests` | 1 | 0 |
| `semio-s-artifact-norm-en1996-schema` | 1 | 0 |
| `semio-s-artifact-puzzle-2d-board-host` | 1 | 0 |
| `semio-s-artifact-puzzle-3d-retained-action` | 1 | 0 |
| `semio-s-artifact-puzzle-5d-mut-any-2` | 1 | 0 |
| `semio-s-artifact-puzzle-5d-v1` | 1 | 0 |
| `semio-s-artifact-stdio-docx-strict-mutations` | 1 | 0 |
| `semio-s-artifact-stdio-png-tests` | 1 | 1 |
| `semio-s-artifact-wfc-grid3d` | 1 | 0 |
| `semio-s-fem-sparse` | 1 | 1 |
| `semio-s-fem-sparse-subspace` | 1 | 1 |
