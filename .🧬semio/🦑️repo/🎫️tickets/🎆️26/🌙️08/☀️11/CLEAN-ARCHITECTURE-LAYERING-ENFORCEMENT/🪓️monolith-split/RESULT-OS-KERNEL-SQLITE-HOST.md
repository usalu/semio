# 🪶 RESULT OS-KERNEL-SQLITE-HOST and WP7 sync (Sonnet executor, prefix `oksql`, started 2026-10-16 04:55)

## 1. Verdict (rewritten after each milestone, last 07:00)

* M0 recovery: done. The previous executor's `oksh_apply.py … write` ran to completion at 04:29; nothing was half-applied, nothing rolled back.
* M1 the four SQLite-host crates (`os-store-space-history`, `-text-io`, `-codec-registry`, `os-io`): the strongly connected component is gone, edges point down only; 5 defects of the earlier carve found and fixed (section 3.1). Statically verified, no rustc evidence yet.
* M2 WP7 `os-store-sync` / `-sync-native` / `-sync-wasm`: **applied at 06:45**, after the engine slice had carved `os-store-engine` and `-space-member` (06:37). The plan's cut was cyclic (host owns the actor spawn); inverted as for the SQLite host (section 3.2). Within budget (census section 5), 0 lost items, test count 88 -> 91 (+2 new platform pins, +1 peer file), surface 57 old names / 0 lost.
* Real compile: **not obtained.** The launcher reaches cargo but every call of this slice was refused at the infra gate `Insufficient physical compiler initial headroom` (06:00, 06:35, 06:54 without the mutex, 06:56 with the mutex; logs `oksql-cargo-1..4.log`). All claims below are static evidence (scoped patched differential, scans), stated as such. A retry is scheduled after the 10-minute wait the brief prescribes; its result is appended to section 5 if it runs.
* Open items for others: section 6.

## 2. Found state (recovery, before any change by this executor)

Evidence read: `oksh-tools/*`, `🗑️generated/oksh-*.log`, `okwp23-tools/touched.txt`, file mtimes, `lost2.py`.

* `🗑️generated/oksh-apply-write1.log` (04:29) ends with `applied`: 35 file operations (4 new, 31 edited), kernel manifest +4 dependencies. `okwp23-tools/touched.txt` lists the 34 paths of that batch (lines 144 to 177). All four crate manifests exist and the four crate roots are the real `🦀️.rs` nodes (no skeletons).
* Pre-carve edits by `pre1.py` / `pre2.py` (03:48 to 03:50): `ArtifactSqliteSnapshotHost` was split by layer. The core hooks stay in the trait; the derived `sqlite_codec` became the blanket trait `ArtifactSqliteSnapshotOperations` (file `📦️codec/🪶️snapshot-capability/🧰️codec-operations/🦀️.rs`, assigned to `os-io`). 147 consumer files (plugins, `mcp`, kernel tests) were re-pointed at 03:50 to `<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()`. No `ArtifactSqliteSnapshotHost>::sqlite_codec` form remains in any `.rs` file (checked with `git grep`, only tool dumps under `okwp56-tools/pre-a1` keep the old text).
* Hand edits after the apply, by the previous executor (found by `find -newermt 04:29:50` over the store, `🚪️io` and kernel trees): `🚪️io/📦️packages/🦀️rust/Cargo.toml` 04:30, `📦️codec/📥️lifted/🚪️io-composer/🦀️.rs` 04:32, `📜️space-history/📦️packages/🦀️rust/Cargo.toml` 04:32 (manifest-audit fix), `🚪️io/🦀️.rs` 04:34 (rustfmt fix of a `pub macro_rules` qualifier), kernel `📦️packages/🦀️rust/🦀️.rs` 04:34. Nothing was edited after 04:35.
* Lost-item audit (`oksh-tools/lost2.py`, re-run by this executor): 1 item lost, `artifact_retire_struct!(HistoryColumn…)` in `♻️retirement/🦀️.rs`, the same benign relocation already classified in RESULT-OS-KERNEL-WP2-3 section 4. No other loss.
* Dependency direction in the live manifests: `space-history` depends on none of the other three; `text-io` depends on `space-history`; `codec-registry` on `space-history` and `text-io`; `os-io` on all three. The upward `ArtifactSqliteSnapshotHost` defaults are gone: its remaining defaults call only `sqlite_snapshot_workspace` inside `space-history`.
* Previous executor's unfinished verification: the union differentials `pre2` (without the four crates) and `post1` (with them) were run, but no keyed diff between them was produced. This executor redoes that scoped diff.
* Defect found on first read (fixed by this executor, section 4): `🚪️io/🧪️tests/🔬️io-mechanism-laws/🦀️.rs` named `semio_framework_os_kernel::io::io_mechanism::IoRunControl` twice, an upward edge from `os-io` to the kernel.

Decision: nothing to finish or roll back; continue with verification and fixes on top of the applied state.

## 3. What changed

### 3.1 The four SQLite-host crates

Design unchanged from the previous executor: the core host trait `ArtifactSqliteSnapshotHost` and its receiving defaults live in `os-store-space-history`; the derived operations trait `ArtifactSqliteSnapshotOperations` (blanket implementation, `sqlite_codec`) lives in `os-io`; no trait in a lower crate calls into a higher one. Crate edges (manifests and `oksql-tools/updeps.py`): `space-history` <- `text-io` <- `codec-registry` <- `os-io`; no mounted file names a higher crate or the kernel.

Defects found and fixed by this executor (none were visible to the earlier differential, because the sandbox resolves the kernel alias):

1. `os-io` did not re-export `publish_native_snapshot` (two kernel callers, `crate::io::publish_native_snapshot`). Fixed in `M/🚪️io/🦀️.rs`.
2. `os-io` kept about 40 kernel-alias paths `store::NativeSnapshot…Owner/BodyWallet`, `store::channel::…`, `store::snapshot_read::ErasedSnapshotRead`, `store::serialization::ArtifactSqliteSnapshot{Export,Import}File` in its lib and in the `io-mechanism-laws` test (`store` is `extern crate self` of the kernel, absent in a real crate). Rewritten to `semio_framework_io_control`, `semio_framework_os_channel`, `semio_framework_os_store_read`, `semio_framework_os_store_text_io` (`oksql-tools/fix_store_alias.py`); manifest gained `semio-framework-os-channel` and dev `serde_json` (test fixture reader). Two further kernel paths `semio_framework_os_kernel::io::io_mechanism::IoRunControl` in the same test now use `IoRunControl` via `use super::*`.
3. The exported macros `field_set_mutations_text!` / `_binary!` moved into `os-store-text-io` but still expanded against `$crate::{__pack_json, __diagnostic, __value, os_pack, OpText, OpBinary, PackError, ProtocolError}`, names only the old kernel root offered. The text-io root now re-exports exactly those (`oksql-tools/macros.py`: 9 names missing -> 0; also 0 for all macros of `os-io` and the sync crates).
4. Privacy of kernel-hosted tests that left their crates: `space_history::…::snapshot::admission` `pub(crate)` -> `pub`; `os-io` `export_snapshot_file_read` / `import_snapshot_file_into` `pub(crate)` -> `pub`; the typed-snapshot test imports `TypedFileSettlement` from `space_history::…::settlement`.
5. Stale `pub macro_rules` rustfmt error of the previous executor's last state: gone (the file was already fixed at 04:34; verified by `verify_paths`).

### 3.2 WP7 sync

**Plan defect.** `S/🔄️sync/🦀️.rs` was to become `sync` (shared half incl. `ArtifactHost`), `sync-native` (`native_actor`) and `sync-wasm`. But `ArtifactHost::open` / `close_key` call `spawn_actor` and own the native runner types (`ArtifactActorRunnerHandle`, tickets, terminal jobs) in `OpenDocument` / `ArtifactHostState`: `sync -> sync-native` and `sync -> sync-wasm` back edges. Cut instead along the real seam:

* `os-store-sync` keeps the platform-neutral registry as `ArtifactHostCore<Runner>` (open documents, event fan-out, presence cadence, hub binding, host reference counting), with `ArtifactHostState<Runner>`, `OpenDocument<Runner>`, `ArtifactChannelsOf<Runner>`; `SyncSession::attach` is generic over the runner slot.
* The platform `ArtifactHost` (same names and signatures the consumers use: `new`, `open`, `close_key`, `closing_runner`, `local_hub_ready`, `set_document_socket_dialer`, …) lives next to the actor it spawns: `os-store-sync-native` (with `ActorRunner`, tickets, folder event-log storage) and `os-store-sync-wasm` (with `WasmActorHub` and the page's socket dialer). `type ArtifactChannels` is a platform alias. Each host exposes `core()`.
* Pre-edit (script `sq1.py`, blocks `sq1_blocks.py`, follow-ups `sq2.py`, `sq3.py`, applied in the monolith first and verified by the differential): `mod native_actor` / `mod wasm_actor` became files under `🖥️native/🎭️actor` and `🌐️wasm/🎭️actor`; retained-turn fixtures moved with the native actor; echo-suppression source pins split per platform (each actor pins its own file, count 1 where the shared test used 2).
* Carve (`oksql-tools/oksql_apply.py write`, wrapper `oksql.py` over the wave-3 tools; 17 files, 0 violations, 0 back edges): roots of the three crates with explicit `pub use` lists (native: the five runner types, `ArtifactChannels`, `ArtifactHost`, the folder-storage names), three manifests (target-gated `tokio` incl. `rt`/`time` for the native actor, `tokio-tungstenite`, `os-services`, `js-sys`, `wasm-bindgen`), kernel mount `os_store::sync` = shim module re-exporting the three crates per cfg, kernel manifest (optional target-table dependencies, `sync` feature forwarding, dev-dependencies with `testing`), 5-line append to the Store root (`mod sync_tests`, the 2,856-line unit test relocated to the kernel root as in WP2-3, pending the companion crates), three corpus producer rows. `vcs::` and the explicit `::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot` paths the tool left were rewritten to `semio_framework_os_vcs` / `semio_framework_os_store_text_io` (alias scan: 0 hits in the three trees).
* Test gates: `fixtures` is `pub mod` and `retained_turn_fixtures` / `fixture_runner_handle` are `pub`, both under `cfg(any(test, feature = "testing"))`; the crates declare the `testing` feature and the kernel dev-depends on both with it. The tool had replaced `use super::*` inside `mod actor_tests` of the relocated unit test by a list that misses the file's own helpers (`test_pool`, `DemoSnapshot`, `DemoMutation`, …, found by the test-config differential); restored (the script now does it).
* Text pins updated: `reactor-contract-oracles` reads the module trees of the three roots (`checks 13` pass); exact-law names of the kernel script (`os_store::component::sync_tests::…`) and of the plugin script (retained-turn laws now run in package `semio-framework-os-store-sync-native`).

## 4. Edits by this executor (repository files)

* Part 1: `M/🚪️io/🦀️.rs`, `M/🚪️io/📦️packages/🦀️rust/Cargo.toml`, `M/🚪️io/🧪️tests/🔬️io-mechanism-laws/🦀️.rs`, `M/🚪️io/📸️typed-snapshot/🧪️tests/🦀️.rs`, `S/🚪️io/🦀️.rs`, `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`.
* Part 2: everything under `S/🔄️sync/` that `oksql-tools/touched.txt` lists plus `S/🔄️sync/🖥️native/🎭️actor`, `🌐️wasm/🎭️actor`, `🖥️native/🧪️tests/🔬️retained-turn-fixtures` (moved from `🧪️tests/🔬️native-actor-retained-turn-fixtures`), `🖥️native/🧪️tests/🔬️echo-suppression-pin`, `🌐️wasm/🧪️tests/🔬️echo-suppression-pin`, the three `Cargo.toml`.
* Outside the slice (one-line or mount edits, recorded): `S/🦀️.rs` (+5 lines, `mod sync_tests`), `K/🦀️.rs` (`pub mod sync` mount -> shim module), `K/Cargo.toml` (sync dependencies, feature, dev-dependencies, `tokio-tungstenite` moved to dev), `M/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts`, `M/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` (law names), `K/📜️script.ts` (one law name), `🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (+3 producer rows).

## 5. Verification (real results)

| gate | command | result |
|---|---|---|
| lost items, four crates | `python3 oksh-tools/lost2.py` | 4,012 / 4,634, 1 lost, benign `artifact_retire_struct!(HistoryColumn…)` (same as WP2-3) |
| lost items, sync tree | `python3 oksql-tools/lost_sync.py` | 375 -> 386 items, 5 listed as lost, all renames of the same item (`impl ArtifactHostState` generic, `struct ArtifactChannels` alias, moved `include!`, two impl headers normalised): 0 real loss |
| edges down | `updeps.py` over the 7 crates | all down; `sync` has no edge to `native`/`wasm` |
| kernel-alias paths | `aliases.py` | 7 crates: 0 (`$crate::…` macro paths excluded) |
| `$crate` macro surface | `macros.py` | 0 missing |
| manifest vs sources | `manifest_audit.py` | 0 missing for the 7 crates (the `trace-memory-testing` and dev-only reports are false positives of cfg(test) items) |
| surface parity | `pubnames.py io os_store`; `surface.py` for sync | `io`: 95 -> 118, 0 lost; `os_store`: 281 -> 293, 0 lost; `sync`: 57 old names, 0 lost, 38 gained (items widened for the relocated unit test, module names) |
| census (lib / test, worst of native + both wasm targets) | `census_all.py` | `space-history` 17 files / 94,101 B / 923 lines; `text-io` 9 / 113,499 / 1,772; `codec-registry` 10 / 95,883 / 1,533 (test 14 / 104,719 / 1,573); `os-io` 8 / 165,633 / 2,102 (test 12 / 209,247 / 2,380); **`sync` lib 2 / 114,905 / 2,000, test 12 / 193,999 / 2,767; `sync-native` lib 3 / 150,566 / 2,825, test 6 / 182,430 / 3,428; `sync-wasm` lib 2 / 55,889 / 1,060, test 4 / 57,087 / 1,076**; all within 3,500 / 28 / 230,000 |
| paths, rustfmt parse, regions | `verify_paths_sql.py` (63 files), `verify_paths23.py` | 118 literals resolve, 0 rustfmt errors; one region finding in `S/🦀️.rs` (not mine) |
| TOML | `bun` `Bun.TOML.parse` | the 4 + 3 manifests and the kernel manifest parse |
| registry | `verify_static.py`; `bun test 🧪️tests/📦️crate-boundaries/🟦️.ts` in `💻️os` | 0 failures, 402 routes; corpus gate 91 pass / 0 fail |
| TS oracle | `coldDocumentPairIngressOracle` | `checks 13` |
| scoped patched differential, lib + wasm32-unknown-unknown, whole tree with the 18 engine crates (`ush.py fin1`) | keyed errors in `🔄️sync` files | **0** |
| same, wasm32-wasip2 (`fin2`) and the earlier `post2/post3` rows | `diffk.py … base` | only the classified sandbox residue (derive-helper twin `value`, `compose_thunk` macro hoist, extern-glob twins `HybridLogicalTimestamp` / `ErasedSnapshotRead` / `InteractionState`, `::semio_framework_async` absolute path of the exported thunk macros) |
| same, test config (`fin2`, before the `actor_tests` fix) | `diffk.py fin2 base test` | real: relocated unit test lost its nested glob (14 `test_pool`, `DemoSnapshot`, `DemoMutation`, `ensure_demo_codec_registered`, …); fixed |
| rerun after the fix (`fin3`, test and wasm32-unknown-unknown test) | keyed errors in `🔄️sync` files | test: 109 entries, all in the sandbox classes: derive helpers `value`/`FromValue`/`ToValue`/`serde`/`dsl`/`artifact` of neutralised extern derives (41), and `crate::__os_spr::{AckStage, ArtifactId, ArtifactBootstrap, decode_client_frame}` / `crate::os_spr::{wire, ArtifactId}` extern-glob imports (68, twins of the base errors in the old root). No `cannot find function/type` left. wasm32-unknown-unknown test: 100 entries in the same classes, no `E0425/E0422/E0433`. One entry I could not classify statically: `unresolved import super::native_actor` in `🔬️backbone-parity` (the parent unit file imports it from the native crate; expected to resolve, to be settled by the real compile) |
| real compile | `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p <the 7 crates> --lib --tests --keep-going --message-format=short`, one batched call under `cargo-mutex.lock` | **refused three times at the infra gate** `Insufficient physical compiler initial headroom`, before any rustc: `oksql-cargo-4.log` (06:56, deficit 7,479,296 B), `oksql-cargo-5.log` (07:09, after the prescribed 10-minute wait and 9 minutes of waiting for the mutex held by cp2; available 4,262,895,616 B against 4,294,967,296 B required, deficit 32,071,680 B). Automatic recovery selected 0 units (`plannedBytes 0`). Mutex released after each call (07:09:58 last). `oksql-cargo-3` (06:50) ran before the mutex rule reached me; it was refused at the same gate before cargo and wrote nothing. Nothing was deleted or moved in the image. |

## 6. Foreign errors, hand-offs

* **Compile admission**: see the real-compile row. Retry after 10 minutes under the mutex; if it reaches cargo, errors in my seven crates are mine, anything else is reported here.
* `os-store-space-schema` (WP2-3 crate): seven mutation files use the derive paths `dsl::MutationLeaf` / `dsl::Mutations`; `dsl` is the kernel alias and the crate's manifest has no `dsl` dependency. Will not resolve in a real build.
* The macro-`$crate` hazard (3.1 item 3) and the kernel-alias hazard (3.1 item 2) are invisible to the union oracle; `macros.py` and `aliases.py` check any crate key. `aliases.py` over the other carved WP2-3 store crates found only the `space-schema` case above.
* `Cargo.lock`: os-io gained dependencies, the three sync crates changed theirs; left to the lock refresh loop.
* Engine slice: nothing blocks me; the kernel `document_materialize` shim already re-exports `semio_framework_os_store_document::*` (the union reports `create_document_envelope` only as an extern-glob artifact).
* Companion crates `os-store-sync-tests-1/-2` still stubs: the unit test stays relocated in the kernel root (`mod sync_tests`), as in WP2-3; WP8 moves it.
* Kernel manifest: `semio-framework-os-services`, `-actor`, `ureq` stay optional (the plan keeps them in the `sync` feature); only `tokio-tungstenite` left the kernel.
