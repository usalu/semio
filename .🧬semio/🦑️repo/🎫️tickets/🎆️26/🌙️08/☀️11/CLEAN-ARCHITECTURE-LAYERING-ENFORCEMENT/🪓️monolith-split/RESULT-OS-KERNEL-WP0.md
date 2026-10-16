# 🧭️ Result OS-KERNEL-WP0: skeletons, mount hub, S0 and the WP1 carve of `semio-framework-os-kernel`

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-OS-KERNEL-CRATES.md` (updated in place: sections 13.1 and 13.2). Brief: `📓️WAVE4-BRIEF.md`, `📓️EXECUTOR-RULES.md`. Tools: `okwp0-tools/` (all Python and one TS counter; `touched.txt` lists the repository files edited through them). Logs: `🗑️generated/okwp0-*`. `K` = `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`, `M` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `S` = `M/🏪️store`.

## 0. Verdict

WP0 is done and WP1 is carved. The coordinator's collision policy was followed: no store file was edited except one consumer-side line (below) and the S0 splitter ran only on the one non-store multi-owner file. **Nothing was compiled by cargo**: the launcher refuses before the source gate (`Finite original maintenance authority required`, start and end; `cargo metadata` refused the same way) and the Nx route dies with `NX Original daemon child exited before acquisition` (`🗑️generated/okwp0-nx-check-list-delta.log`). The evidence is the patched union differential (below), a surface-parity scan, an extern-path existence scan, census, static registry checks and the TS corpus test.

* 67 skeleton crates (48 libraries, 19 companions) with manifests, `📋️project.json`, owner rows, root workspace members and 6 launch routes each in both launch files; 9 of the libraries (WP1) and 4 companions are real crates.
* The kernel lib closure shrank from 182 files / 3,994,566 B / 59,414 lines to **152 / 3,622,290 / 53,411** (native, `deflate,sync,ureq,worker`).
* Static test count is unchanged: **1,077 test attributes before (579-file test closure) and 1,077 after** (facade 782 + 13 carved closures). The `cargo test --list` numbers of the plan (1,465 and 72) cannot be reproduced without a compiler; the attribute-level count is the invariant I could check.
* WP5 and WP6 were not touched. Readiness report: section 7.

## 1. What changed

1. **Census re-run** (section 2): identical to the plan's section 1.1 before the work; no drift.
2. **Skeletons** (`okwp0-tools/gen_files.py`): `Cargo.toml` per recipe (`workspace` = repository root, `[lib] path = "../../🦀️.rs"`, three boundary keys at the ceiling, first-party edges of 6.1 gated by target tables, external specs copied from the kernel manifest), `📋️project.json`, and a stub lib root (plus `🧪️tests/🔬️unit/🦀️.rs` for libraries) only where the node has no `🦀️.rs`: 14 library stubs, 14 companion stubs. 34 library nodes keep their existing module file as the crate root in place.
3. **Registries** (`gen_registry.py`, idempotent): 67 members merged into the byte-sorted member list of the root `Cargo.toml`, 67 owner rows, 402 launch routes in each of `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (after the last wgpu-renderer route, order bands `900.26001`, `922.1`, `903.8`, 132 unused keys each).
4. **Mount hub** (`hubsplit.py`): `K/🦀️.rs` 332 to 147 lines; `K/📥️mounts/🌱️wp1-leaves` (facade lines of the carved crates) and `K/📥️mounts/🧪️wp8-tests` (root-mounted tests). Two hubs instead of five, and `io` / `os_store` stay in the root, because the test-configuration differential reported eleven `macro_expanded_macro_exports_accessed_by_absolute_paths` for modules that define `#[macro_export]` macros inside an `include!`d file.
5. **S0** (`s0_io.py`): `M/🚪️io/🦀️.rs` 2,358 to 1,817 lines; 60 codec-registry items moved verbatim to `S/📦️codec/📥️lifted/🚪️io/🦀️.rs`, which the original `include!`s (path literals rebased, region markers kept in the original and repeated balanced around the moved blocks). Lossless: 0 lost, 0 extra non-marker lines. The plan's Appendix C line numbers of this file are stale by +1; runs are derived from item names. S0 on the store files is not done (collision policy).
6. **WP1 crates carved** (section 3 has the graph): `os-list-delta`, `os-semio`, `os-space-sqlite-cells`, `os-plugin-registry-schema`, `os-extension`, `os-pack-tools`, `os-spr`, `os-spr-cli`, `os-dsl`; companions `os-spr-laws-tests`, `os-dsl-tests`, `os-dsl-module-tests`, `os-dsl-fixture-sweep-tests`.
   * Every `crate::os_spr::`, `crate::os_pack::`, `crate::os_dsl::`, `crate::list_delta::`, `semio_format::` and `semio_framework_os_kernel::sqlite_snapshot::` path was rewritten at token level (comments and strings untouched) to the owning crate; `list_delta` macros use `$crate::X`, `$crate::__mutation_diff`, `__value_derive`, `__dsl_record_derive`.
   * `os-spr` keeps the replication re-export surface (modules `format`, `history`, `wire`, `channel`, `command`, ..., 52 explicit re-exports and the four `protocol` module globs) so the facade module `os_spr` is `pub use semio_framework_os_spr::*` plus the cli; `protocol_laws` stays behind feature `protocol-laws`, `mutation_fixture_ops` behind `mutation-testing`.
   * `os-extension` (empty on wasm32-wasip2), `os-pack-tools` and `os-spr-cli` (empty on wasm32) carry a crate-level `#![cfg(...)]`; the facade lists them in the matching target tables.
   * The facade (`K/Cargo.toml`) gained 9 path dependencies (extension under the not-wasip2 table, pack-tools and spr-cli under the not-wasm32 table) and the feature forwards `protocol-laws` and `mutation-testing` to `os-spr`.
   * One consumer-side edit in a store file: `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` mounted `🪐️space/🪶️sqlite/🦀️.rs` as `mod fields`; it is now `use semio_framework_os_space_sqlite_cells as fields;` (the file was last touched 10-14 14:34 by nobody active; its ten `pub(crate)` helpers became `pub`).
   * The 1,126 KB grammar originals fixture is now read at run time (`std::fs`), not embedded.
7. **Boundary corpus** (`📦️crate-boundaries/🔣️.json`, `🧬️cuts.json`): 9 producer rows and 6 cut rows for the carved crates (skeletons get rows from their own work package, see 13.2 of the plan).
8. **Script selector**: `K/📜️script.ts` target `test-command-transport-native` pointed four filters at kernel module paths that moved; it now runs the three owning crates plus the kernel filter.

## 2. Census

Canonical counter `readRustSourceClosureV1` (`okwp0-tools/closure.ts`, host `aarch64-apple-darwin`, `debug_assertions`). Rows: files / bytes / active lines.

| closure | before | after |
|---|---|---|
| kernel lib native (`deflate,sync,ureq,worker`) | 182 / 3,994,566 / 59,414 | **152 / 3,622,290 / 53,411** |
| kernel lib wasm32-unknown-unknown | 179 / 3,920,933 / 55,898 | 153 / 3,627,614 / 51,821 |
| kernel lib wasm32-wasip2 | 175 / 3,583,426 / 52,474 | 150 / 3,304,211 / 48,691 |
| kernel test native | 579 / 9,131,580 / 104,813 | 450 / 7,068,931 / 90,694 |
| kernel test wasm32-unknown-unknown | 568 / 8,965,353 / 97,690 | 449 / 7,040,807 / 86,481 |

New crates (`🗑️generated/okwp0-census-wp1.log` and the first six in the session log): lib native / test native.

| crate | lib native | lib wasm32-unknown | lib wasip2 | test native |
|---|---|---|---|---|
| os-list-delta | 1 / 49,995 / 910 | 1 / 49,995 / 910 | 1 / 49,995 / 910 | 2 / 65,536 / 1267 |
| os-semio | 1 / 32,097 / 426 | 1 / 32,097 / 426 | 1 / 32,097 / 426 | 6 / 56,812 / 584 |
| os-space-sqlite-cells | 1 / 9,406 / 152 | 1 / 9,406 / 152 | 1 / 9,406 / 152 | 3 / 10,143 / 181 |
| os-plugin-registry-schema | 4 / 34,941 / 157 | 4 / 34,941 / 157 | 4 / 34,941 / 157 | 7 / 39,931 / 185 |
| os-extension | 1 / 14,306 / 297 | 1 / 14,306 / 297 | 1 / 14,306 / 0 | 3 / 21,197 / 424 |
| os-pack-tools | 2 / 38,494 / 678 | 1 / 27,195 / 0 | 1 / 27,195 / 0 | 5 / 55,273 / 861 |
| os-spr | 3 / 68,413 / 1149 | 3 / 68,413 / 794 | 3 / 68,413 / 794 | 8 / 173,736 / 2990 |
| os-spr:protocol-laws,mutation-testing | 4 / 134,671 / 2257 | 4 / 134,671 / 1869 | 4 / 134,671 / 1869 | 8 / 173,736 / 2990 |
| os-spr-cli | 1 / 36,320 / 809 | 1 / 36,320 / 0 | 1 / 36,320 / 0 | 2 / 56,938 / 1207 |
| os-dsl | 19 / 90,192 / 1512 | 18 / 84,744 / 1422 | 18 / 84,744 / 1422 | 19 / 90,192 / 1512 |
| os-spr-laws-tests | 1 / 446 / 0 | 1 / 446 / 0 | 1 / 446 / 0 | 28 / 79,271 / 1479 |
| os-dsl-tests | 1 / 1,975 / 0 | 1 / 1,975 / 0 | 1 / 1,975 / 0 | 24 / 170,545 / 1931 |
| os-dsl-fixture-sweep-tests | 1 / 239 / 0 | 1 / 239 / 0 | 1 / 239 / 0 | 11 / 61,317 / 1035 |
| os-dsl-module-tests | 1 / 2,651 / 0 | 1 / 2,651 / 0 | 1 / 2,651 / 0 | 25 / 46,645 / 682 |


All 14 rows are far below 3,500 / 28 / 230,000 in the lib and test closure; the first `os-dsl` cut had a 36-file test closure (over the 32 ceiling), which is why its unit tests moved to the companion (its closure is now equal to its lib closure; the unit tests live in `os-dsl-module-tests`). The skeleton crates that keep an existing root still show the monolith closure through their `#[path]` mounts until their work package carves them.

## 3. Crate graph

`os-list-delta`, `os-semio`, `os-space-sqlite-cells`, `os-plugin-registry-schema` are leaves; `os-extension` -> `os-semio`; `os-pack-tools` -> (none first-party; externals `os-pack`, `pack-source`, `dsl-record`, `async`, `value`); `os-spr` -> `os-pack-tools` (not wasm32); `os-spr-cli` -> `os-pack-tools`, `os-spr` (not wasm32); `os-dsl` -> `os-semio`, `os-pack-tools` (not wasm32); companions -> their crate and the facade (dev). 66 manifests, 255 first-party edges, all strictly decreasing along the rank order of section 4.3 (`verify_static.py`: 0 failures). Corrections to the plan's 6.1/6.2 are listed in section 13.2 of the plan.

## 4. Files

Created: 67 `Cargo.toml` + 67 `📋️project.json`, 28 stub lib roots, 14 stub unit-test files, `K/📥️mounts/{🌱️wp1-leaves,🧪️wp8-tests}/🦀️.rs`, `S/📦️codec/📥️lifted/🚪️io/🦀️.rs`, `okwp0-tools/*`, this file.
Edited (81 repository files, `okwp0-tools/touched.txt`, plus the registries): `K/🦀️.rs`, `K/Cargo.toml`, `K/📜️script.ts`; the lib roots and test files of the nine WP1 nodes and their module files (`M/🪡️list-delta`, `🧬️semio`, `🪐️space/🪶️sqlite`, `🔌️plugin/📇️registry/…`, `🧩️extension`, `🎒️pack/{⌨️cli,🚦️control}`, `📡️spr/**`, `🗣️dsl/**`); `M/🚪️io/🦀️.rs`; one store file (above); the two corpus JSON files; root `Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`; `PLAN-OS-KERNEL-CRATES.md`; `📓️FLEET-CLAIMS.md`.
Not touched: `Cargo.lock` (root, framework and the 176 plugin locks: refresh in the compile window), any other store file, `🧑‍💻dev/📤️distribution/🧾️manifest.json`, the 40 text-pinned scanners of 8.5 (phase 2), taxonomy registrations. Longest new path: 161 bytes.

## 5. Defects found by the evidence (fixed)

1. `include!` hub for modules that define `#[macro_export]` macros (see 1.4): found by the test-configuration hub differential; `io` and `os_store` are back in the root.
2. Two real regressions of the first WP1 cut found by the union differential: `use crate::control;` beside `pub mod control;` in `os-pack-tools` (E0255), and a private `use crate::wire::{ProtocolLimits, …}` in the `os-spr` root that shadowed the `pub use self::wire::*` glob (E0603 for `semio_framework_os_spr::ProtocolLimits`, 14 sites in `os-spr-cli`); it is now a `pub use`.
3. The S0 staging file began with inner doc comments (`//!`), illegal in an included file (E0753); it now starts with plain comments.
4. Sandbox artefact characterised, not fixed: names reached through a glob of a carved module whose own imports fail (extern crates are unresolved in the sandbox) disappear from consumer files (e.g. `crate::Edit`, `Mutation`, `OpText`). `import_diff.py` proves that all 193 such new unresolved imports are either names of the carved facade modules (surface parity proven by `pubnames.py`) or extern crate identifiers (proven to exist by `extcheck.py`).
5. Two mistakes of my own rewriting tools, caught by the test-configuration union and fixed: a second-pass resolver replaced `crate::sparse_record_diff` (flat name) by the bare crate name and dropped the macro name (laws files, 138 rewrites redone from the pristine snapshot), and `semio_format::` was also replaced where it followed `os_store::` (63 sites in the dsl unit test). Both rewrites were redone from the snapshot and re-run through the differential.
6. The companion host modules (`os-dsl-module-tests`) need the private imports of the module under test (`HashMap`, `EdgeLabel`, ...): they were found missing by the differential and copied into each host; `#[path]` literals inside an inline module need `#[path = "."]` because rustc builds the path through the (non-existent) module directory.
7. Sandbox-only ambiguity: `format` imported from `os-spr` by `use super::*` collides with the prelude `format!` macro because the extern `protocol::format` is unresolved (3 sites in the `os-spr` unit test); with the real crate `format` is a module (type namespace) and cannot collide.
8. Interrupted patch of this session: the `sed` that was cut by the usage limit was a one-line fix of `okwp0-tools/hub_oracle.py`; I re-read it, all 21 tool scripts compile, and every repository file I edited re-passes rustfmt, path resolution, the static registry checks and the union differential (section 6). Nothing half-applied remained in the tree.
9. Pre-existing and unrelated: `🗣️dsl/🧪️tests/🧱️architecture` fails 2 of 7 (`🕸️graph` manifest missing; framework `diagnostic` dependency list), the workspace-contract test "proves current STDIO TXT and GLTF wrapped origins through kernel aliases" fails on an `✏️s` plugin leaf (0 declarations), and the dsl root and the protocol-laws unit file each have one unbalanced `//#region` marker at HEAD.

## 6. Verification (commands run, real results)

| check | command / tool | result |
|---|---|---|
| cargo gate, start | `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-kernel --message-format=short --lib` | `Rust crate Source refused before compiler: semio-framework-os-kernel (Rust crate Source boundary exceeded before compiler byte allocation)` (expected: the kernel is over every ceiling) |
| cargo gate, end | same launcher, `check -p semio-framework-os-list-delta`, and `metadata --no-deps --offline` | `error: Finite original maintenance authority required` (both) |
| Nx route, end | `bun repo run @semio-tech/os-list-delta-rs:check --configuration=prototype --parallel=1 -- --lib --offline` | graph and index steps run, then `NX Original daemon child exited before acquisition` (exit 1); no gate verdict, no compile (`🗑️generated/okwp0-nx-check-list-delta.log`) |
| mount hub, lossless | `hubsplit.py dry` | 14 units, 332 to 147 root lines, 0 lost, 0 extra non-blank lines |
| mount hub, differential | `hub_oracle.py` (A = hubbed root, B = original root; patched oracle, native lib) | lib: 1 new (a lumped import message split), 10 fewer; test: 12 new of which 11 `macro_expanded_macro_exports_accessed_by_absolute_paths` (cause of finding 5.1, fixed by reverting the `io`/`os_store` units) |
| S0 split | `s0_io.py dry|apply` | 147 items, 60 moved; 0 lost, 0 extra; regions balanced in both files |
| **union differential** (A = pristine snapshot + the 81 touched files with the 12 carved crates and companions as modules of the facade, `crate::` rewritten per module, `pub(crate)` rewritten to `pub(in <module>)`; B = pristine snapshot; patched oracle of `📓️RESOLUTION-ORACLE.md`; errors keyed by code, highlighted text and file) | `union.py u7` (lib, wasm32-unknown-unknown, wasm32-wasip2) and `union.py u11` (test, wasm test) | lib: A internal 1,077 (external 2,956) vs B 1,012 (2,861): **0 new except 12 keyed entries**, all split pieces of lumped `unresolved imports` messages in store consumers and carved-crate files (explained below), 133 fewer; wasm32-unknown-unknown: 1,079 vs 950, 12 new (same), 69 fewer; wasm32-wasip2: 978 vs 872, 12 new (same), 63 fewer; test: 2,677 (8,416) vs 2,569 (8,239), 22 new, 152 fewer; wasm test: 2,608 (7,994) vs 2,429 (7,882), 22 new, 79 fewer. Facade-path propagation artifacts, excluded from `onlyNew`: 186 / 157 / 238 / 236. No new E0412, E0425, E0422, E0423, E0603, E0616, E0624 or E0432 of an internal path remains |
| union, explanation of the residual new keys | `import_diff.py u7 lib,wasm,wasip2` | new unresolved imports 194 / 192 / 172 (lib/wasm/wasip2) and 261 / 257 (test/wasm test), all explained (names of the carved facade modules, proven to exist by `pubnames.py`, or extern crate identifiers, proven by `extcheck.py`), 0 unexplained |
| union, sensitivity | `union.py s1 lib --seed private` (`ordinal` of `os-space-sqlite-cells` made private in the sandbox) | reported 10 new `E0603 function ordinal is private` in the two store consumers |
| facade surface parity | `pubnames.py` (old kernel root from the snapshot against the live hub + crates, globs expanded through extern crates) | `os_pack` 212 = 212, `os_spr` 436 = 436, `os_semio` 24 = 24, `os_extension` 15 = 15, `native_catalog` 9 = 9; `os_dsl` 16 to 15 (only the test-only `fixture_sweep` module); `list_delta` 13 to 16 (`__dsl_record_derive`, `__mutation_diff`, `__value_derive`, hidden); `os_plugin_module_schema` 9 to 10 (`native_catalog`) |
| extern path existence | `extcheck_run.py` (name index of the target crates, cfg-agnostic, globs followed) | 558 paths found, 70 not followed (glob chains), 0 missing; sensitivity: a bogus name in `semio_framework_mutation_diff` is reported |
| impl placement / orphans, private member names | `scans.py` | 0 suspicious impls; 2 member-name hits (`is_ready`, `to_json`), both false positives (trait method / foreign type) |
| census | `census_all.py`, `closure.ts` | section 2; all 14 crate rows inside 3,500 / 28 / 230,000 in lib and test closure |
| static test count | `testcount.py` (attribute-level, test closure native, facade + 13 carved closures) | before 1,077, after 1,077 |
| registries | `verify_static.py` | 67 manifests parsed (Bun TOML), name/id/role/boundary keys/workspace/lib path/lints/project name correct, 256 first-party edges all pointing down the rank order, 67 members byte-sorted and unique, 67 owner rows, 402 routes with the exact command in each launch file, no duplicate order keys, new folder emoji unique among siblings, `failures 0` |
| TOML | `@iarna/toml` and `Bun.TOML` on the 67 manifests + the facade | identical JSON for all 68, 0 problems; the corpus test below re-parses the 9 production rows with both |
| paths / rustfmt / regions | `verify_paths.py` (163 files touched or created) | `rustfmt +nightly-2026-07-20` parses all (0 failures); 262 `#[path]`/`include*!` literals resolve; longest relative path 161 bytes; regions balanced except two files that are unbalanced at HEAD (dsl root, protocol-laws unit file) |
| both launch JSON files parse | `verify_static.py` (strict JSON and `Bun.JSONC`) | yes, 12,681 and 9,624 configurations (peers keep adding routes) |
| TS: crate-boundary corpus | `bun test ./🧪️tests/📦️crate-boundaries/🟦️.ts` in `💻️os` | 70 pass, 0 fail, 2,995 expects (includes the 9 new rows, 6 new cuts, the token law and the four cfg source caps) |
| TS: dsl architecture | `bun test ./🧪️tests/🧱️architecture/🟦️.ts` in `🗣️dsl` | 5 pass, 2 fail (both unrelated: `🕸️graph` manifest ENOENT; framework `diagnostic` dependency list) |
| TS: workspace contract (STDIO wrapped origins) | `bun test ./🟦️.ts -t "proves current STDIO TXT and GLTF …"` in `📚️library/🧪️tests/🔬️workspace-contract` | fails on a plugin leaf declaration (`✏️s/…/🧊️gltf/…/🌳️node/🏷️rename/🦀️.rs`, expected 1, received 0) before reading any file of this task |
| taxonomy | `taxonomy-scope.ts` on `S/⚙️engine`, `S/📒️ledger`, `S/🧪️tests/🏙️bin-01` | the new folders are `directory-kind-unresolved`, the new `📦️packages/🦀️rust` folders resolve; registration is the compile-window sweep |
| `[DEBUG]` logs | none added |


## 7. WP5 / WP6 readiness (no store file edited)

Checked at 21:43 against the files of section 1.4 of the plan:

* **Member-hydration batch has NOT landed.** `RetainedPersistedDocumentHydration<P, M>` still has two type parameters (`S/🧾️document/📜️history/💧️hydration/🦀️.rs`, last edit 10-14 14:34); the only artefacts are read-only notes (`member-hydration-next-owned-source-plan.md` 10-15 00:10, `📓️2026-10-14-member-hydration-current-frontier.md` 10-14 22:12, `member-hydration-producer-census.md`). No source edit under `S/🏗️initialization/🏠️owners/🏭️member` since 10-14 22:02, none in `S/🧩️composition/🚪️open/🏭️operation` since 10-14 21:25.
* **Store-producer capabilities are partly in the code.** `create_member_store` / `open_member_store` in `S/🦀️.rs` already take `producer: &G, catalog: &C` (root last edited 10-15 04:25); `ArtifactStore::new::<ArtifactSnapshotPublication>` is still selected at four sites of the root (lines 909, 1054, 10251, 10942). `store-producer-current-readonly.md` (10-14 20:25) is the last note.
* **Store tree quiet.** Newest edit under `S/` outside my files: 10-15 17:15 (`📒️ledger/📨️messages`, `👥️presence/📸️snapshot`, both already extracted peer crates); `S/🔗️read`, `🔁️replay`, `🧬️schema`, `🧵️canonical-edit`, `🎮️one-item`, `🧬️snapshot-clone`, `🚪️io`, `🧾️document`: 10-14 14:34 to 20:35. No peer note about the store newer than 10-15 00:43 in either ticket folder (the 10-15 17:00+ notes of `MODULAR-MONOREPO-BUILDS…` are build-cache and wgpu work).
* **Scheduling:** WP2/WP3 (store base: ledger, replay, read, canonical-json, publication, text-io, snapshot-clone, one-item-batch) can start now on a quiet tree after S0 of the store root; WP5 (engine) and the three files of the hydration batch should wait until that batch lands or is cancelled; WP6 (`os-store-document`, `-member-open-store`) likewise. WP5 owns `K/🦀️.rs`'s `os_store` mount line, and the mount of `S/🦀️.rs` stays in the root (finding 5.1).

## 8. Residual risks and hand-offs

1. **No cargo evidence, no lock refresh.** Type-level properties (method/field privacy beyond what the union saw, trait scope, coherence, derive-emitted paths) are covered by scans only. First window: `check -p` for the 12 new crates with `--tests`, `wasm32-unknown-unknown` and `wasm32-wasip2`, then `update --workspace --offline` for the root, framework and plugin locks.
2. **The oracle is weaker than a compiler**: extern crates are unresolved, globs from failed imports vanish (finding 5.4), and rustc stops before the privacy pass once errors exist. Compensations: `extcheck.py` (558 extern paths found, 70 not followed through glob chains, 0 missing), `pubnames.py` (facade surface parity: `os_pack` 212 = 212, `os_spr` 436 = 436, `os_semio`, `os_extension`, `native_catalog` equal; `os_dsl` loses only the test-only `fixture_sweep` module; `list_delta` and `os_plugin_module_schema` gain `__*` hidden re-exports and `native_catalog`), `scans.py` (0 suspicious impls, 2 member-name hits triaged as false positives).
3. **Facade globs.** `os_spr` and `os_dsl` are single-crate glob re-exports (`pub use <crate>::*`), not explicit lists; the explicit generated lists (plan 8.1) belong to phase 2 once all crates exist.
4. **Companions** `os-dsl-tests` and `os-dsl-fixture-sweep-tests` depend on the facade (`dev`) and still use `use super::*` shapes through host modules; their test-configuration differential result is in section 6; the first compile may still find private items I could not see.
5. **Taxonomy.** The new folders are unregistered (`directory-kind-unresolved`, e.g. `S/⚙️engine/{🧱️core,🎞️live,🗄️stage}`, the bin folders): the verifier of one `S/` scope crawls the repository for 4 minutes at the current load, so only three scopes were probed; the registration sweep of the compile-window checklist (item 6) owns it.
6. **Lock-step text readers.** Not yet re-pointed: `K/📋️project.json` input list (`📡️spr/🦀️.rs`, still valid), the `🕸️graph` / `🗣️dsl` ownership fixtures, `🧪️lib-wiring-ratchet`, `🖥️os-source-topology` (all still name files that exist); the 93 Nx targets of `K/📋️project.json` besides `test-command-transport-native` need the phase-2 selector table.
7. **The 1,465 + 72 count gate** needs `cargo test -- --list` per package; the static attribute count (1,077 = 1,077) is the substitute.
