# Result rust-carve (wave 1022a, executor rust-carve)

Tools: `C/rust-carve-tools/` (gate.ts static gate counter with the unit cfg + all features, lib.py edit helpers, locks.py + lock_apply.py Cargo.lock editor, oracle.py rustc --emit=metadata oracle, async_*.py, pre_a.py). Generated: `C/🗑️generated/rust-carve-x/` (lock backups, specs).
Gate = `storage.sourceBoundary`: 262144 bytes / 32 files / 4000 active lines per unit closure (cfg incl. `test` + every feature). Counter reproduces the compiler ingress number exactly (async 4,092 before).
CARGO HOLD respected: no cargo run; verification = static gate + rustc --emit=metadata oracle (extern-crate errors filtered).

## 1. semio-framework-async (start 12:15, applied 12:38-12:47)
| | before | after |
|--|--|--|
| async closure (all features) | 4,092 lines / 194,844 B / 28 files | 3,046 lines (76 %) / 151,396 B / 21 files |
New sibling crates (dirs under `🧰️framework/🔨️modules/⏳️async/`, lib root = the existing module file, no new taxonomy names):
- `semio-framework-async-timer` (`⏰️timer-wheel`): TimerWheel/TimerSleep; async depends on it; `pub(super)` widened to `pub` (hook, `TIMER_ACTIONS_PER_POOL_TURN`) / `pub(crate)`.
- `semio-framework-async-sync` (`🚦️sync-primitives` root + mounted `🪢️oneshot`): Notify/Semaphore/OwnedPermit/oneshot.
- `semio-framework-async-slots` (`🧱️boxed-slots` root + `🗺️sharded-map` + `🎰️slots/📄️pages`): boxed_slots, boxed_fixed_slots, FixedSlotTableBudget, assert_fixed_slot_tables, on_bounded_stack, OriginalFixedSlotPages, ShardedMap.
- `semio-framework-async-testkit` (`🎭️testkit`): ManualRuntime (explicit imports instead of `use super::*`). The async `testkit` feature stays (pool/cancel test hooks).
- No re-exports left in async. 63 importer files (71 incl. companion) re-pointed with `rust-carve-tools/async_b.py` (use-tree aware, owners resolved via closure/mount chain incl. `include!`), 41 consumer Cargo.toml got the new dep next to their async dep (same style), companion tests: `pub use` globs + dev deps, `use testkit::ManualRuntime` -> `semio_framework_async_testkit::ManualRuntime`.
- Registered: framework Cargo.toml members + workspace.dependencies; root + hub workspace.dependencies; `📋️build-owners.json` (4 rows); per-crate `📋️project.json`; db crate-boundaries corpus (io-core, storage-driver-runtime, storage-postgres: +async-sync; io-driver: +async-testkit); hub unit-census reason text.
- Cargo.lock: 190 locks edited by `lock_async.py` (47 packages: 4 new + edges); zero removals in any lock (diff-checked).
- Verified statically: `rustc --emit=metadata` on timer and sync = clean (no warnings); slots = only the missing `semio_framework_value` extern; async root with all features = only missing-extern errors (no internal resolution errors); gate numbers above.
## 2. semio-framework-pack-record-encode (start 12:45, applied 12:47)
| | before | after |
|--|--|--|
| test unit | 21 files / 283,968 B / 1,940 lines | 8 files / 112,272 B (43 %) / 797 lines |
- NEW `semio-framework-pack-record-encode-cursor` (dir `…/🛫️encode/🫳️borrowed/⏳️cursor/📦️packages/🦀️rust`, lib root = the cursor file) owns retained cursor + `🎒️document` cursor (feature `deflate`) + `📨️wire` cursor and their tests/fixtures: lib 106 KB, test unit 172,434 B / 13 files / 1,154 lines (66 %).
- The cursor family used nothing of the encode root except its imports: `use super::*` became explicit imports; cfg(test) `RequestedAllocator` global allocator restored in the new crate (+dev-dep); document test calls `semio_framework_pack_record_encode::encode_document_controlled` (dev-dep on the encode crate).
- Only importer: facade `semio-framework-pack` `record` module (re-points to the new crate; dep added). No compat re-export in the encode crate.
- Registered: framework members, build-owners, profile constraints (optLevel 2 for the new package, both profiles), pack + pack-record test package lists in the two `📜️script.ts`; 125 Cargo.locks edited, 63 plugin locks skipped (they never listed `pack-record-encode`; stale, need `cargo update --offline`: list in `🗑️generated/rust-carve-x/pre-stale-locks.txt`).
- Verified statically: oracle on both crates (lib + `--test`): zero resolution errors besides missing externs / the dsl derive attribute.

## COORDINATOR FREEZE (13:2x)
Coordinator: no new/renamed/deleted `📋️project.json` or Cargo workspace member until "sealed" (rebuild-3, pid 76367). Already applied before the order: the 4 async crates + `pack-record-encode-cursor` (project.json + members stay as they are). Ephemeral and replay carves are prepared as staged apply-scripts (`C/rust-carve-tools/eph_*.py`, `rep_*.py`, dry-run only) and are placed in one compile-atomic step after the seal message.

## 3. semio-framework-os-store-ephemeral (start 13:00, STAGED, not applied: freeze)
Script: `rust-carve-tools/eph_apply.py` (dry-run default, `apply` writes; `CARVE_STAGE=<dir>` writes to a sandbox). Staged result verified in `scratchpad/stage-eph`:
| | before | after (static gate, testkit) |
|--|--|--|
| ephemeral test unit | 27 files / 263,337 B / 3,465 lines | 7 files / 90,920 B (35 %) / 1,288 lines |
| NEW semio-framework-os-store-presence lib | - | 6 files / 85,400 B / 1,227 lines (test unit ~175 KB, fixture outside the sandbox) |
- NEW `semio-framework-os-store-presence` (`🏪️store/👥️presence/📦️packages/🦀️rust`, lib root = existing `👥️presence/🦀️.rs`): peer table + `👥️presence-store-and-peers-publication` (file MOVED into `👥️presence/`) + read + io publication + rejection + retirement (the leaves hold inherent impls of the store types, so they must share the crate). Depends on the ephemeral crate.
- The shared `ArtifactEphemeralOneItemPublication` state machine is driven by both lane stores: new `ArtifactEphemeralOneItemPublication::preparing(..)` constructor, ten state fields + `advance_preparation` widened `pub(super)` -> `pub` (documented), the presence store uses the constructor instead of a struct literal.
- Facade: kernel store root gets `pub use semio_framework_os_store_presence::*;` + `pub use ... as presence;` (keeps the 16 `store::os_store::presence::…` plugin paths); testing prelude re-points `PresenceStore`; kernel + prelude manifests get the dep; presence snapshot dep moves from ephemeral to presence; stray unbalanced `//#endregion` in the ephemeral root replaced by one balanced region pair, presence root has a balanced `Authority` region.
- Registered in the script: root members, build-owners, os crate-boundaries corpus row, cuts.json (ephemeral definition -> TransientStore, new presence cut, snapshot cut consumer), TS readers `tool-job-coverage/publication` and `cad-presence-retirement` (new file path).
- Oracle (lib + `--test`, staged): zero resolution errors for both roots (only missing-extern / extern-derive attributes).
## 4. semio-framework-os-store-replay (start 13:15, STAGED, not applied: freeze)
Script: `rust-carve-tools/rep_apply.py`. Staged result (`scratchpad/stage-rep`):
| | before | after |
|--|--|--|
| replay test unit | 25 files / 269,223 B / 3,098 lines | 13 files / 130,170 B (50 %) / 2,008 lines |
| NEW semio-framework-os-store-replay-operation test unit | - | 12 files / 140,278 B (53 %) / 1,101 lines |
- NEW `semio-framework-os-store-replay-operation` (`🔁️replay/🎮️operation/📦️packages/🦀️rust`, lib root = the existing operation file): preparation traits/task/retirement, native drivers, original history, derived operations and their tests. The `impl ReplayOwnedState` block (inherent impl of an L1 type) moves back into the replay crate as inline module `operation_preparation` in `⏩️replay-owned-state-operations`; `OperationMessageSettlement::new`/`close_cold` widened to `pub`.
- Replay root keeps the name `replay_preparation` through a private `use semio_framework_os_store_replay_operation as replay_preparation;` (no re-export); all `semio_framework_os_store_replay::replay_preparation::` importers (engine, live, store-components, owners, read, prelude, kernel facade alias, paged-one-item, derived/report files mounted by the engine) re-pointed, owner manifests get the dep, OS corpus row added.
- Oracle (staged): replay root zero errors in lib and `--test`; operation root only extern-trait artefacts.
## Open
- After the seal: run `eph_apply.py apply` and `rep_apply.py apply` (each is one compile-atomic step), then `lock_apply.py` for both (tags `eph`, `rep`: set `CARVE_TAG`), then `bun repo cargo --store=shared --configuration=prototype -- check -p <new crates> -p <dependents>` when the cargo hold lifts.
- Pre-existing red rows in the db crate-boundaries corpus (ids, engine-base, io-core drift vs. manifests) are not from this work.

## SEALED 13:14: ephemeral + replay APPLIED (13:14-13:16)
- `eph_apply.py apply` (18 files) and `rep_apply.py apply` (28 files) done; `📋️project.json` + members + build-owners + corpus rows placed.
- Locks: manifests were not recorded, so `rust-carve-tools/lock_sync.py` derives the edges from the manifests (new packages + edges to presence / replay-operation, presence-native-snapshot removed from ephemeral): 117 locks changed, 72 skipped as stale (lack deps of the carved packages; includes the 63 plugin locks without pack-record-encode) -> need `cargo update --workspace --offline` per workspace (root update attempt 13:15 failed `Insufficient physical compiler initial headroom` while peers held cargo).
- Verification: `check -p ephemeral -p presence -p replay-operation -p replay -p read -p testing-prelude -p engine` queued detached (log `🗑️generated/rust-carve-x/check-root1.log`), waiting on memory capacity behind peers.

## VERIFICATION (13:17-14:10, cargo, one at a time, logs `🗑️generated/rust-carve-x/check-*.log`)
Fixes found by the real compiler: replay inline module needed `use semio_framework_value::ValueError;`; presence dev-dep `semio-framework-os-spr` needs `features=["protocol-laws"]` (tests of the moved retirement/rejection use `protocol_laws`). Root and framework `Cargo.lock` needed one `update --workspace --offline` each (cargo produced the canonical result; hand edits were close).
- `check -p os-store-ephemeral -p os-store-presence -p os-store-replay-operation -p os-store-replay -p os-store-read --tests` (root): GREEN (warnings only).
- `check -p async-timer -p async-sync -p async-slots -p async-testkit -p async -p pack-record-encode -p pack-record-encode-cursor -p pack-record -p pack --tests` (framework): all libs GREEN; `semio-framework-async-tests` (companion) lib-test fails with 10 errors NOT from the carve (stale test wrappers: `RetireOwned`/`RetirementCursor` methods not imported in `cancel-return`/`wake` test wrappers, `Lane: Deserialize` needs `wasm-pool-host`); none names a moved symbol.
- `check --workspace --keep-going` framework: only 3 pre-existing peer failures (3d-mesh-document `set_shading_job`.., math-rank `released_bytes`, tool-run-entities `Sync` on retirement dyn). Root workspace: failures only in infinite-board-dag-graph/normal-gestures (private `BoardPropertyAudit`), db-io-registry (`protocol_error_retirement_demand`), plugin-* (`decode_presence_native`, `pack_json`, job trait items): peer drift, none involving async-sync/slots/testkit/timer, presence, replay-operation, pack cursor. `semio-framework-os-store-one-item-batch` fails at `paged-one-item/🧬️mutation-apply` E0631 (rust-port rewrite, not touched by me); testing-prelude/engine therefore not compiled on their own.
- Gate after: async 3,046 lines; pack-record-encode test 112 KB / cursor 172 KB; ephemeral test 90.9 KB; presence ~175 KB; replay test 130 KB; replay-operation 140 KB (all <= 67 % of 262,144 B).
- Stale locks (73 plugin/standalone workspaces): `rust-carve-tools/update_locks.sh` runs `update --workspace --offline` per workspace sequentially, DETACHED, pid 22085, status `🗑️generated/rust-carve-x/lock-updates/status.txt` (handed over; `done` is the last line). Hub lock was edited by lock_async (not stale).
