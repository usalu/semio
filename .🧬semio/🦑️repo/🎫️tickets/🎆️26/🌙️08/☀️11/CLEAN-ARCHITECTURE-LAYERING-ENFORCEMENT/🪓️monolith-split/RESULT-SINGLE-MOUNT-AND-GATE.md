# 🧱️ Result SINGLE-MOUNT-AND-GATE: one owner per source file, two gates cleared

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Slice prefix `smg`. Tools: `smg-tools/` (`closure.ts`, `modowners.ts`, `diff_oracle.py`, `checks.py`, `cycles.ts`, `register.py`, `lockfix.py`, `shell_thin.py`, `taxo.ts`, `bk.sh`; file lists `touched.txt`, `deleted.txt`, `new-files.json`). Logs: `🗑️generated/smg-*`. Pre-edit copies of every edited file are in the session scratchpad (`scratchpad/pre/<repo path>`, outside the repo because the ticket path plus a repo path exceeds the length limit). Audit source: `AUDIT-WAVE5-RUST.md` findings F1, F2, F3, F5, F9.

## 0. Verdict

**Four of the five defects are fixed as prescribed (D1, D2, D3, D4); D5 is fixed in the single-owner sense but NOT in the prescribed direction (section 6, deviation 1).** Nothing is compiled by cargo: the launcher was refused by the physical-headroom gate twice (12:26 and 12:34), so every claim rests on static evidence: census, module-owner scan, scoped differential, path, rustfmt, TOML and JSON checks and a dependency-cycle scan.

| Gate | Before | After |
|---|---|---|
| Repo census, oversized crates | 3 (`plugin`, `os-renderer-wgpu` 37 files, `plugin-host-test-component` 41 files) | **1** (`plugin`, peers `pi1`/`pwp7`) |
| Module-owner scan, files reached from more than one package | 116 (42 inside the trees this slice touches) | **45 (0 inside the touched trees)** |
| Dependency graph, 798 packages | n/a | 0 cycles among normal dependencies; the 12 dev-dependency cycles are all in `value`, `dsl-record`, `2d` (none mine) |

## 1. Defects and what was done

### D1. `guest-contract` compiled plugin-owned source (audit F1)

Found: the three mounts of `🔌️plugin/🧬️guest-contract/🦀️.rs` (`operation_authority`, `original_actor_context`, `sqlite_wire`) compiled files that `plugin-abi` and the plugin monolith also compiled; the contract manifest even lacked `semio-framework-job-fault-publication`, which the context call sources import.

Direction: `plugin-abi` is below the contract, so the contract depends on it; the context vocabulary cannot sit in `reactor-context` (rank 15, depends on reactor-app, runtime-ops, extension), so the **vocabulary moved into the contract and the plugin re-exports it**.

* `guest-contract/🦀️.rs`: `pub use semio_framework_plugin_abi::{operation_authority, return_walk_template, sqlite_wire};` and `#[path = "🎟️original-actor-context/🦀️.rs"] pub mod original_actor_context;`.
* New `guest-contract/🎟️original-actor-context/🦀️.rs` (23 lines): the former context root minus the wasm exports; it mounts the unchanged call tree `⚛️reactor/🎟️context/🌉️call/**` (6 production files, the tests and fixtures stay where they are, now owned by the contract; their path literals are untouched). Physical relocation of that tree into the contract folder is left out on purpose: about 15 path literals in TS laws, scripts, `include_str!` fixtures and two `[[test]]` paths read it.
* `reactor/🎟️context/🦀️.rs` (the file the plugin monolith mounts, and the future `reactor-context` root) is now the plugin side only: `pub use semio_framework_plugin_guest_contract::original_actor_context::*;`, the explicit imports the wasm `exports` file takes through `use super::*;`, and `#[cfg(wasm32+p2)] pub mod exports;`. `exports` is unchanged. Every consumer path (`semio_framework_plugin::reactor::original_actor_context::…`, `…guest_contract::original_actor_context::…`) now names one type.
* Manifests: contract `+ plugin-abi, + job-fault-publication, + optional trace`, feature `original-actor-context-testing = ["dep:semio-framework-trace"]`; plugin `+ guest-contract` and the feature forwards `semio-framework-plugin-guest-contract/original-actor-context-testing`; `reactor-context` `+ guest-contract`, same feature forward. These are one-line edits in manifests of the slice `pi1` is flipping; see hand-off 1.
* The contract's `[[test]] original_native_frame_caller` read a renderer file (`🎟️native-caller`, also compiled by `host-plumbing`): moved to `host-plumbing/Cargo.toml` (it already depends on the contract), and the renderer TS script that ran it under the wrong package (`semio-framework-plugin-interpreter`) now names `semio-framework-os-renderer-wgpu-host-plumbing`.
* Last double owner in the touched trees, `🛂️authority/📤️return/🦀️.rs`: one source compiled against two generated binding sets (guest bindings in `plugin-abi`, host bindings in `plugin-host-operation`). It is now the macro `return_walk_template!` (body unchanged, outer doc instead of `//!`), mounted once in `plugin-abi`; the guest invokes it in `authority::return_walk`, the host in `operation::return_walk` through the contract's re-export. A stub crate proved the macro parses and expands.

### D2. `abi` mounted twice (audit F2)

`ui-host` and `ui-backend-webgpu` no longer `#[path]`-mount `🌉️abi/🦀️.rs`; both depend on `semio-framework-abi`. 44 `crate::abi::` paths in 6 files became `semio_framework_abi::`. The flow wasm adapter converted between the two duplicated `AbiErrorCode` types (`surface_abi_failure`, 21 arms, plus the `ui_webgpu::abi` import); it is gone and the six call sites use `abi_failure`. The `ui-host` project input line for the abi file, its handoff fixture (`abi` object removed, `semio-framework-abi` dependency row added) and the workspace-contract test (abi path pair and mount assertion removed, `not.toContain("🌉️abi")` added) follow. Locks: the `semio-framework-abi` edge is added to `ui-host` and `ui-backend-webgpu` in 33 `Cargo.lock` files (5 of them gained the abi package block).

### D3. Gates

* `semio-framework-os-renderer-wgpu` **37 → 13 files** (1,846 → 1,164 lines). Fifteen Shell chunk files were empty or carried only unused imports, dead code or wasm hooks: deleted with their mounts, `SHELL_SOURCE` lines and the world-model test's path list (`smg-tools/shell_thin.py`). Dead items removed: `settings_select_row`, `UI_TERMINOLOGY_NATIVE`, `PREFS_STORE`. The five `#[wasm_bindgen]` hooks (`semioWgpuSetHubEnv`, `…HostWindowBlur`, `…SetHostStorage`, `dumpAgentBridgeOfferScope`, `…SetAgentBridgeConfig`) moved to the renderer root region `🏠️ShellHostHooks`, the cdylib entry. The `const _` size assertion moved next to `SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES` in `shell-docs`. The `pub use …contracts::{…}` find-item line moved to the Shell parent. Nine chunks that only host the monolith tests stay as `#[cfg(test)]` mounts. A dangling doc comment at the end of the storage chunk (a parse error in the base) was removed.
* `semio-framework-plugin-host-test-component` **41 → 16 files**: the refusal artifact (`🗿️artifacts/🚫️snapshot-refusal/**`, 24 files) is the new crate `semio-framework-plugin-host-test-component-refusal` (25 files, 10 embedded) with `schema_descriptor()`; the component keeps `RefusalApp` and its routes and uses the crate (`use … as refusal;`, private). Registered: root workspace member, owner row `@semio-tech/framework-plugin-host-test-component-refusal-rs`, six launch routes in both launch files, `Cargo.lock` package block, taxonomy resolves (`smg-tools/taxo.ts`).

### D4. Dev-dependency back edge (audit F5)

The one test using `os-store-canonical-json` named three types that `semio-framework-pack-json` (already a dependency) exports; the test now names them there and the dev-dependency (and its lock edge) is removed. No inversion was needed.

### D5. Kernel test build mounts the store-testing tree (audit F9)

The testing package's lib root pointed at the real fixture file, which cannot compile as a crate (`use super::tests::{DemoDiff, DemoSnapshot, LossyDiff, assert_fixture_descriptor}`). It now has its own placeholder root (`🧪️testing/📦️packages/🦀️rust/🦀️.rs`, `[lib] path = "🦀️.rs"`, census 1 file) and the kernel test build is the single owner of the 34 files. Not done as prescribed: section 6, deviation 1.

## 2. Census (`ONLY=… bun 🗑️generated/census.ts`, lib target, `smg-census-final.log`)

| crate (`semio-framework-…`) | files | bytes | active lines | over limit | binding | oversized deps |
|---|---:|---:|---:|---|---|---:|
| `os-renderer-wgpu` (was 37 / 146,504 / 1,846, binding files) | 13 | 115,311 | 1,164 | no | bytes | 1 (`plugin`) |
| `plugin-host-test-component` (was 41 / 52,210 / 626, 20 embedded) | 16 | 32,436 | 331 | no | files | 1 (`plugin`) |
| `plugin-host-test-component-refusal` (new) | 25 | 19,556 | 300 | no | files | 0 |
| `plugin-guest-contract` (was 10 files in the base closure) | 7 | 88,836 | 677 | no | bytes | 0 |
| `plugin-abi` | 6 | 71,929 | 880 | no | bytes | 0 |
| `plugin-host-operation` | 3 | 15,827 | 141 | no | files | 0 |
| `plugin-reactor-context` (placeholder root) | 1 | 334 | 4 | no | files | 0 |
| `ui-host` | 6 | 129,849 | 2,619 | no | lines | 0 |
| `ui-backend-webgpu` | 3 | 47,744 | 872 | no | lines | 0 |
| `abi` | 2 | 45,149 | 1,118 | no | lines | 0 |
| `os-store-space-schema` | 16 | 55,795 | 899 | no | files | 0 |
| `os-store-testing-fixtures` (was 20+ files, double mount) | 1 | 595 | 4 | no | files | 0 |
| `os-kernel` | 5 | 116,802 | 1,623 | no | bytes | 0 |
| `os-renderer-wgpu-host-plumbing` | 7 | 73,167 | 1,239 | no | lines | 0 |
| `os-flow-wasm-domain` | 6 | 78,571 | 1,545 | no | files | 1 (`plugin`) |

Whole repo (`census.ts` without `ONLY`, same run): 728 crates, 487 members, 628 compilable, **1 oversized** (`plugin`, 157 lib files; peers).

## 3. Verification: every command run and its real result

1. `bun smg-tools/modowners.ts … smg-modowners-before.tsv` (patched copy of `audit5-tools/modowners.ts`): 2,479 manifests, 116 files owned by more than one package. After: `…-after.tsv` 50, `…-final.tsv` **45**; filtered to the trees edited here (guest-contract, context, `🚪️io`, `🧬️schema/🪶️sqlite`, abi, ui-host, webgpu, store testing and schema, test component, Shell, renderer root, native-caller, flow wasm): 42 before, **0 after**. The 45 left are outside this slice (plugin artifact families `✏️s`, `value/♻️retirement`, derive, kernel with `dsl` companion and with two space artifacts, csv with trace and mcp, presence testing, and five plugin-host/interpreter test or runtime files).
2. Scoped differential `smg-tools/diff_oracle.py` (RESOLUTION-ORACLE method: `rustc +nightly-2026-07-20 --edition 2021 --emit=metadata -o /dev/null`, sandboxes outside the repo, extern crates and external globs neutralised the same way on both sides; base = the pre-edit files from the scratchpad copies). Error multisets by (code, message):

   | crate and configuration | base | now | new | gone | reading of the new rows |
   |---|---:|---:|---:|---:|---|
   | guest-contract lib / `--test` / feature | 99 / 198 / 126 | 43 / 133 / 70 | 1 each | 57 / 66 / 57 | only the unresolved extern `semio_framework_plugin_abi` |
   | plugin native lib | 965 | 951 | **0** | 14 | |
   | context root, wasm32-wasip2 | 97 | 114 | 36 | 19 | all names that `exports` now takes from the neutralised extern glob `pub use …guest_contract::…::*`; each of the 6 names is in the contract's `pub use` list, no other name is missing |
   | plugin-abi native / wasip2 | 89 / 188 | 89 / 188 | **0 / 0** | 0 / 0 | the macro expands in the guest module without a new row |
   | host-operation | 17 | 18 | 2 | 1 | the extern macro `return_walk_template!` is unresolved in the sandbox |
   | ui-host lib / `--test` | 121 / 240 | 127 / 275 | 7 / 36 | 1 / 1 | only the unresolved extern `semio_framework_abi` |
   | ui-backend-webgpu native / wasm32 | 1 / 513 | 1 / 519 | 1 / 7 | 1 / 1 | same |
   | space-schema lib / `--test` | 161 / 197 | 161 / 197 | 0 / 2 | 0 / 2 | swapped extern names (`canonical_json` gone, `pack_json` in) |
   | test-component | 369 | 241 | 1 | 129 | the unresolved extern `…test_component_refusal` |
   | renderer top native / wasm32 | 150 / 144 | 140 / 143 | **0** / 13 | 10 / 14 | the 13 are the moved hooks naming `shell::…` re-exports (neutralised globs) and `#[wasm_bindgen]`, the same names fail in the base chunk files |
   | flow domain adapter native / wasm32 | 52 / 62 | 51 / 61 | **0 / 0** | 1 / 1 | |

   The refusal crate has no base; a current-only run gives 132 errors, all from the neutralised nested glob `sqlite_snapshot::{*, …}` and the leading-colon `::semio_framework_os_kernel::` paths (the same rows exist in the base component). The oracle is blind to typeck, privacy of fields and methods, trait scope and Cargo facts; the extern-name rows above are the price of running it per crate.
   Not run: renderer top and plugin `--test` closures. The test closure reads `✏️TextEditor/🧫️fixtures/📝️explicit-draft/🔣️.json`, which is absent on disk (pre-existing, foreign).
3. `python3 smg-tools/checks.py smg-tools/touched.txt`: 109 touched paths (27 Rust, 11 Cargo.toml, 48 lock files, 6 JSON/JSONC). Result: 0 rustfmt parse failures (stdin mode), 419 `#[path]`/`include!` literals checked and 0 missing, longest path 212 bytes (139 characters, limit 240 bytes), 0 TOML or JSON parse failures. 15 deleted paths in `smg-tools/deleted.txt`.
4. `bun smg-tools/cycles.ts`: 798 packages, 0 normal-dependency cycles; the new edges (`plugin → guest-contract → plugin-abi`, `ui-host → abi`, `webgpu → abi`, `component → refusal`) close nothing.
5. `bun phc-tools/verify_registry_host.ts smg-tools/registry-plan.json`: `ALL REGISTRY CHECKS PASSED` for the new crate (manifest, boundary keys, byte-sorted members and owner rows, six launch routes in each launch file).
6. `bun smg-tools/taxo.ts` (real `matchDirectoryKind`): `🎟️original-actor-context` and the refusal `📦️packages/🦀️rust` chain resolve.
7. `bun test … workspace-contract/🟦️.ts -t "UI-host metadata binds…"`: fails at the first read, because the schema file `📚️library/🧬️schema/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json` does not exist (pre-existing). A script replaying the assertions that do not need the schema: all fixture `internalDependencies` rows match the manifest (including the abi row), the entry file has no abi mount; the fixture's `project` mirror was already stale (`$schema`, `nx:run-commands`) before my edit.
8. Real compile, both attempts refused before the compiler (mutex taken and released each time, nothing deleted in the image): `Physical compiler admission refused {"availableBytes":4262871040,"requiredBytes":4294967296}` (32,096,256 bytes short), logs `smg-cargo-root.log` (12:26) and `smg-cargo-root-2.log` (12:34). Command to run when headroom returns, root workspace (take `🗑️generated/cargo-mutex.lock` first):
   `cd /Users/ueli/Documents/semio && NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-plugin-guest-contract -p semio-framework-plugin-host-test-component-refusal -p semio-framework-os-store-space-schema -p semio-framework-os-store-testing-fixtures -p semio-framework-plugin-reactor-context --lib --keep-going --message-format=short`
   Framework workspace (`ui-host`, `webgpu`, `abi` are members there): the same command with `-p semio-framework-ui-host -p semio-framework-ui-backend-webgpu -p semio-framework-abi … --manifest-path 🧰️framework/Cargo.toml`. A lock refresh comes first (`-- update --workspace --offline` per workspace, section 6). Then `--tests` for `guest-contract`, `ui-host`, `webgpu` and `space-schema`.

## 4. Files

Edited: `ui-host` (`🦀️.rs`, `📡️event`, `🪟️window`, two unit-test files, `Cargo.toml`, `📋️project.json`); `ui-backend-webgpu` (`🦀️.rs`, `🧊️surface-adapter`, one render unit test, `Cargo.toml`); flow wasm (`🔤️argument-helpers`, `🛠️adapter-ops`, `🧩️domain-adapter`); library fixture JSON and `workspace-contract` test; guest-contract (`🦀️.rs`, `Cargo.toml`); `⚛️reactor/🎟️context/🦀️.rs`, `reactor-context/Cargo.toml`, plugin `Cargo.toml`; `🚪️io/🛂️authority/🦀️.rs` and `📤️return/🦀️.rs`; `plugin-host-operation/🦀️.rs`; renderer `host-plumbing/Cargo.toml` and the renderer TS script; space-schema `Cargo.toml` and its mutation unit test; test component (`🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`, refusal root); renderer Shell parent, the storage and panel-records chunks, `shell-docs` staging file, renderer root, world-model unit test; `os-store-testing-fixtures/Cargo.toml`; root `Cargo.toml`, `📋️build-owners.json`, both launch files; 48 `Cargo.lock`.
Created: `guest-contract/🎟️original-actor-context/🦀️.rs`; the refusal crate `Cargo.toml` and `📋️project.json`; `os-store-testing-fixtures` placeholder `🦀️.rs`.
Deleted (15 Shell chunk folders, each only `🦀️.rs`): `🎬️shell-actions`, `🧩️session-and-topology`, `💬️chrome-dialog-and-tooltips`, `🧭️navbar-footer-and-overlays`, `📨️command-dispatch`, `🖼️chrome-frame-and-main-window`, `🏛️space-admin-and-settle`, `🧰️window-actions-and-commands`, `🎓️tutorial-runtime`, `🗨️chrome-overlays-and-tour`, `📤️icon-export`, `🪟️tree-windows`, `🔖️identity-and-types`, `🖱️pointer-input`, `🧵️shell-async-runners`.

## 5. Consumer re-points outside the slice (one-liners, brief rule 4)

`🌊️flow/🕸️wasm` (identity mapper removed, 6 call sites); the renderer TS script (package name of one cargo test); `♾️infinite/🌍️world/🗃️model/🧪️tests/🔬️unit` (13 deleted Shell chunk paths removed from `SHELL_SOURCE()`); the library fixture and test above.

## 6. Deviations, foreign findings and hand-offs

**Deviation 1 (D5).** The kernel test build should depend on the testing package as a dev-dependency; I did not do that, because it cannot be done safely without a compiler: (a) the fixture root names `DemoSnapshot`, `DemoDiff`, `LossyDiff` and `assert_fixture_descriptor` from the 12,634-line kernel unit-test module (605 uses of `DemoSnapshot` there, 40 more in 30 other kernel test files), and `DemoSnapshot` implements `ArtifactStoreSnapshot`, `ArtifactSqliteSnapshotHost`, `ArtifactDsl`, `ArtifactPack` (about 30 names to re-path, a thread-local decode counter the tests read); (b) a fixtures crate that depends on the kernel facade makes a dev-dependency cycle (two kernel copies, split trait identities), so it would have to depend on the lower crates only, and the `dsl::Mutations` derive names `::protocol::…` (the replication crate under that rename) which no carved manifest provides; (c) `fixture_member_factory!` in the kernel tests implements `MemberFactory` for `ArtifactStore<DemoSnapshot, _>`, which breaks the orphan rule wherever `DemoSnapshot` lives once `ArtifactStore` and `MemberFactory` sit in lower crates (this already affects the kernel test build today, independent of my edit). The single-owner defect is gone (34 shared files → 0). Blueprint for the WP8 slice: move the four types and the decode counter into the fixtures crate on lower-crate paths, give the member-factory impls a home in the crate that defines the trait or the type, then replace the kernel's `mod fixture_mutations;` and `tests` re-exports by the dev-dependency and point the fixtures `[lib] path` back at `../../🦀️.rs`.

**Hand-off 1 (`pi1`, reactor-context flip).** `reactor/🎟️context/🦀️.rs` is the file `pi1` will make the `reactor-context` root. It is now the plugin-side root (re-export + wasm `exports`); the call sources are owned by `guest-contract` and must not be mounted by the flipped crate. `reactor-context/Cargo.toml` and the plugin `Cargo.toml` already carry the `guest-contract` edge and the `original-actor-context-testing` forward; keep both when regenerating manifests. This is the only edit inside the files of `pi1`'s scope, and it is the one the brief prescribes.

**Hand-off 2 (locks).** The lock files are edited textually, no cargo ran. Complete: root `Cargo.lock` and `🧰️framework/Cargo.lock` for abi, canonical-json and the refusal crate. Stale until `update --workspace --offline`: the guest-contract edges are missing in the plugin-workspace locks that do not list `semio-framework-plugin-guest-contract` yet (154 locks for the plugin edge, 136 for `reactor-context`; 10 and 11 locks lack the `job-fault-publication` and `plugin-abi` packages).

**Foreign, pre-existing, not touched:** the test component glue mounts `#[cfg(test)] #[path = "../../🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]`, which exists nowhere (the refusal tree has only a TS test there); the test closures of the plugin and the renderer top reference the missing TextEditor fixture above; the ui-host handoff schema file is missing; the kernel unit tests carry the orphan-rule problem of deviation 1(c); `plugin-host-operation` and `plugin-abi` now share the template by macro, and the host operation crate reaches it through the contract re-export.

**Not done:** physical relocation of `🌉️call/**` into the contract folder; trimming the now mostly unused import block of the Shell parent (kept for the nine test hosts); a real compiler verdict for any changed crate.
