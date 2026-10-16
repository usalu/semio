# 🧭️ Result OS-KERNEL-WP0: skeletons, mount hub, S0 and the WP1 carve of `semio-framework-os-kernel`

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-OS-KERNEL-CRATES.md` (updated in place: sections 13.1 and 13.2). Brief: `📓️WAVE4-BRIEF.md`, `📓️EXECUTOR-RULES.md`. Tools: `okwp0-tools/` (all Python and one TS counter; `touched.txt` lists the repository files edited through them). Logs: `🗑️generated/okwp0-*`. `K` = `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`, `M` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `S` = `M/🏪️store`.

## 0. Verdict

WP0 is done and WP1 is carved. The coordinator's collision policy was followed: no store file was edited except one consumer-side line (below) and the S0 splitter ran only on the one non-store multi-owner file. **Nothing was compiled by cargo**: the launcher refuses before the source gate (`Finite original maintenance authority required`, start and end; `cargo metadata` refused the same way) and the Nx route dies with `NX Original daemon child exited before acquisition` (`🗑️generated/okwp0-nx-check-list-delta.log`). The evidence is the patched union differential (below), a surface-parity scan, an extern-path existence scan, census, static registry checks and the TS corpus test.

* 66 skeleton crates (48 libraries, 18 companions) with manifests, `📋️project.json`, owner rows, root workspace members and 6 launch routes each in both launch files; 9 of the libraries (WP1) and 3 companions are real crates.
* The kernel lib closure shrank from 182 files / 3,994,566 B / 59,414 lines to **152 / 3,622,290 / 53,411** (native, `deflate,sync,ureq,worker`).
* Static test count is unchanged: **1,077 test attributes before (579-file test closure) and 1,077 after** (facade 782 + 12 carved closures). The `cargo test --list` numbers of the plan (1,465 and 72) cannot be reproduced without a compiler; the attribute-level count is the invariant I could check.
* WP5 and WP6 were not touched. Readiness report: section 7.

## 1. What changed

1. **Census re-run** (section 2): identical to the plan's section 1.1 before the work; no drift.
2. **Skeletons** (`okwp0-tools/gen_files.py`): `Cargo.toml` per recipe (`workspace` = repository root, `[lib] path = "../../🦀️.rs"`, three boundary keys at the ceiling, first-party edges of 6.1 gated by target tables, external specs copied from the kernel manifest), `📋️project.json`, and a stub lib root (plus `🧪️tests/🔬️unit/🦀️.rs` for libraries) only where the node has no `🦀️.rs`: 14 library stubs, 14 companion stubs. 34 library nodes keep their existing module file as the crate root in place.
3. **Registries** (`gen_registry.py`, idempotent): 66 members merged into the byte-sorted block of the root `Cargo.toml` (274 to 340), 66 owner rows, 396 launch routes in each of `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (after the last wgpu-renderer route, order bands `900.26001`, `922.1`, `903.8`, 132 unused keys each).
4. **Mount hub** (`hubsplit.py`): `K/🦀️.rs` 332 to 147 lines; `K/📥️mounts/🌱️wp1-leaves` (facade lines of the carved crates) and `K/📥️mounts/🧪️wp8-tests` (root-mounted tests). Two hubs instead of five, and `io` / `os_store` stay in the root, because the test-configuration differential reported ten `macro_expanded_macro_exports_accessed_by_absolute_paths` for modules that define `#[macro_export]` macros inside an `include!`d file.
5. **S0** (`s0_io.py`): `M/🚪️io/🦀️.rs` 2,358 to 1,817 lines; 60 codec-registry items moved verbatim to `S/📦️codec/📥️lifted/🚪️io/🦀️.rs`, which the original `include!`s (path literals rebased, region markers kept in the original and repeated balanced around the moved blocks). Lossless: 0 lost, 0 extra non-marker lines. The plan's Appendix C line numbers of this file are stale by +1; runs are derived from item names. S0 on the store files is not done (collision policy).
6. **WP1 crates carved** (section 3 has the graph): `os-list-delta`, `os-semio`, `os-space-sqlite-cells`, `os-plugin-registry-schema`, `os-extension`, `os-pack-tools`, `os-spr`, `os-spr-cli`, `os-dsl`; companions `os-spr-laws-tests`, `os-dsl-tests`, `os-dsl-fixture-sweep-tests`.
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

@@CENSUS@@

All 12 rows are far below 3,500 / 28 / 230,000 in the lib and test closure; the first `os-dsl` cut had a 36-file test closure (over the 32 ceiling), which is why its unit tests moved to the companion (now 12 files). The skeleton crates that keep an existing root still show the monolith closure through their `#[path]` mounts until their work package carves them.

## 3. Crate graph

`os-list-delta`, `os-semio`, `os-space-sqlite-cells`, `os-plugin-registry-schema` are leaves; `os-extension` -> `os-semio`; `os-pack-tools` -> (none first-party; externals `os-pack`, `pack-source`, `dsl-record`, `async`, `value`); `os-spr` -> `os-pack-tools` (not wasm32); `os-spr-cli` -> `os-pack-tools`, `os-spr` (not wasm32); `os-dsl` -> `os-semio`, `os-pack-tools` (not wasm32); companions -> their crate and the facade (dev). 66 manifests, 255 first-party edges, all strictly decreasing along the rank order of section 4.3 (`verify_static.py`: 0 failures). Corrections to the plan's 6.1/6.2 are listed in section 13.2 of the plan.

## 4. Files

Created: 66 `Cargo.toml` + 66 `📋️project.json`, 28 stub lib roots, 14 stub unit-test files, `K/📥️mounts/{🌱️wp1-leaves,🧪️wp8-tests}/🦀️.rs`, `S/📦️codec/📥️lifted/🚪️io/🦀️.rs`, `okwp0-tools/*`, this file.
Edited (81 repository files, `okwp0-tools/touched.txt`, plus the registries): `K/🦀️.rs`, `K/Cargo.toml`, `K/📜️script.ts`; the lib roots and test files of the nine WP1 nodes and their module files (`M/🪡️list-delta`, `🧬️semio`, `🪐️space/🪶️sqlite`, `🔌️plugin/📇️registry/…`, `🧩️extension`, `🎒️pack/{⌨️cli,🚦️control}`, `📡️spr/**`, `🗣️dsl/**`); `M/🚪️io/🦀️.rs`; one store file (above); the two corpus JSON files; root `Cargo.toml`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`; `PLAN-OS-KERNEL-CRATES.md`; `📓️FLEET-CLAIMS.md`.
Not touched: `Cargo.lock` (root, framework and the 176 plugin locks: refresh in the compile window), any other store file, `🧑‍💻dev/📤️distribution/🧾️manifest.json`, the 40 text-pinned scanners of 8.5 (phase 2), taxonomy registrations. Longest new path: 161 bytes.

## 5. Defects found by the evidence (fixed)

1. `include!` hub for modules that define `#[macro_export]` macros (see 1.4): found by the test-configuration hub differential; `io` and `os_store` are back in the root.
2. Two real regressions of the first WP1 cut found by the union differential: `use crate::control;` beside `pub mod control;` in `os-pack-tools` (E0255), and a private `use crate::wire::{ProtocolLimits, …}` in the `os-spr` root that shadowed the `pub use self::wire::*` glob (E0603 for `semio_framework_os_spr::ProtocolLimits`, 14 sites in `os-spr-cli`); it is now a `pub use`.
3. The S0 staging file began with inner doc comments (`//!`), illegal in an included file (E0753); it now starts with plain comments.
4. Sandbox artefact characterised, not fixed: names reached through a glob of a carved module whose own imports fail (extern crates are unresolved in the sandbox) disappear from consumer files (e.g. `crate::Edit`, `Mutation`, `OpText`). `import_diff.py` proves that all 193 such new unresolved imports are either names of the carved facade modules (surface parity proven by `pubnames.py`) or extern crate identifiers (proven to exist by `extcheck.py`).
5. Pre-existing and unrelated: `🗣️dsl/🧪️tests/🧱️architecture` fails 2 of 7 (`🕸️graph` manifest missing; framework `diagnostic` dependency list), the workspace-contract test "proves current STDIO TXT and GLTF wrapped origins through kernel aliases" fails on an `✏️s` plugin leaf (0 declarations), and the dsl root and the protocol-laws unit file each have one unbalanced `//#region` marker at HEAD.

## 6. Verification (commands run, real results)

@@VERIFY@@

## 7. WP5 / WP6 readiness (no store file edited)

Checked at 21:43 against the files of section 1.4 of the plan:

* **Member-hydration batch has NOT landed.** `RetainedPersistedDocumentHydration<P, M>` still has two type parameters (`S/🧾️document/📜️history/💧️hydration/🦀️.rs`, last edit 10-14 14:34); the only artefacts are read-only notes (`member-hydration-next-owned-source-plan.md` 10-15 00:10, `📓️2026-10-14-member-hydration-current-frontier.md` 10-14 22:12, `member-hydration-producer-census.md`). No source edit under `S/🏗️initialization/🏠️owners/🏭️member` since 10-14 22:02, none in `S/🧩️composition/🚪️open/🏭️operation` since 10-14 21:25.
* **Store-producer capabilities are partly in the code.** `create_member_store` / `open_member_store` in `S/🦀️.rs` already take `producer: &G, catalog: &C` (root last edited 10-15 04:25); `ArtifactStore::new::<ArtifactSnapshotPublication>` is still selected at four sites of the root (lines 909, 1054, 10251, 10942). `store-producer-current-readonly.md` (10-14 20:25) is the last note.
* **Store tree quiet.** Newest edit under `S/` outside my files: 10-15 17:15 (`📒️ledger/📨️messages`, `👥️presence/📸️snapshot`, both already extracted peer crates); `S/🔗️read`, `🔁️replay`, `🧬️schema`, `🧵️canonical-edit`, `🎮️one-item`, `🧬️snapshot-clone`, `🚪️io`, `🧾️document`: 10-14 14:34 to 20:35. No peer note about the store newer than 10-15 00:43 in either ticket folder (the 10-15 17:00+ notes of `MODULAR-MONOREPO-BUILDS…` are build-cache and wgpu work).
* **Scheduling:** WP2/WP3 (store base: ledger, replay, read, canonical-json, publication, text-io, snapshot-clone, one-item-batch) can start now on a quiet tree after S0 of the store root; WP5 (engine) and the three files of the hydration batch should wait until that batch lands or is cancelled; WP6 (`os-store-document`, `-member-open-store`) likewise. WP5 owns `K/🦀️.rs`'s `os_store` mount line, and the mount of `S/🦀️.rs` stays in the root (finding 5.1).

## 8. Residual risks and hand-offs

1. **No cargo evidence, no lock refresh.** Type-level properties (method/field privacy beyond what the union saw, trait scope, coherence, derive-emitted paths) are covered by scans only. First window: `check -p` for the 12 new crates with `--tests`, `wasm32-unknown-unknown` and `wasm32-wasip2`, then `update --workspace --offline` for the root, framework and plugin locks.
2. **The oracle is weaker than a compiler**: extern crates are unresolved, globs from failed imports vanish (finding 5.4), and rustc stops before the privacy pass once errors exist. Compensations: `extcheck.py` (0 missing of 998 checked extern paths; the rest unknown because a glob chain was not followed), `pubnames.py` (facade surface parity: `os_pack` 212 = 212, `os_spr` 436 = 436, `os_semio`, `os_extension`, `native_catalog` equal; `os_dsl` loses only the test-only `fixture_sweep` module; `list_delta` and `os_plugin_module_schema` gain `__*` hidden re-exports and `native_catalog`), `scans.py` (0 suspicious impls, 2 member-name hits triaged as false positives).
3. **Facade globs.** `os_spr` and `os_dsl` are single-crate glob re-exports (`pub use <crate>::*`), not explicit lists; the explicit generated lists (plan 8.1) belong to phase 2 once all crates exist.
4. **Companions** `os-dsl-tests` and `os-dsl-fixture-sweep-tests` depend on the facade (`dev`) and still use `use super::*` shapes through host modules; their test-configuration differential result is in section 6; the first compile may still find private items I could not see.
5. **Taxonomy.** The new folders are unregistered (`directory-kind-unresolved`, e.g. `S/⚙️engine/{🧱️core,🎞️live,🗄️stage}`, the bin folders): the verifier of one `S/` scope crawls the repository for 4 minutes at the current load, so only three scopes were probed; the registration sweep of the compile-window checklist (item 6) owns it.
6. **Lock-step text readers.** Not yet re-pointed: `K/📋️project.json` input list (`📡️spr/🦀️.rs`, still valid), the `🕸️graph` / `🗣️dsl` ownership fixtures, `🧪️lib-wiring-ratchet`, `🖥️os-source-topology` (all still name files that exist); the 93 Nx targets of `K/📋️project.json` besides `test-command-transport-native` need the phase-2 selector table.
7. **The 1,465 + 72 count gate** needs `cargo test -- --list` per package; the static attribute count (1,077 = 1,077) is the substitute.
