# Result tco2 (executor tco2): tests that need private items live in their crate; gate re-measure

`C` = this ticket. Sonnet 5.5, foreground (verification runs were detached `nohup` jobs polled from the turn because the cargo queue exceeded the 10-minute call limit), no sub-agent, no modifying git command, no package.json/bun.lock edit.

## Decision applied
Tests that need private items are mounted inside the crate (`#[cfg(test)] #[path] mod`), public-API-only tests stay in a `-tests` companion, and every crate keeps its test cfg inside the source gate (measure: `crateTreeHostCfg()+"test"` through `readRustSourceClosureV1`, same reader as `crate-gate8-disk.ts`; gate 32 files / 262144 bytes / 4000 active lines). Measured ratios (files / bytes / lines):

| crate | prod | test cfg | note |
|---|---|---|---|
| `semio-framework-server` | 0.41 / 0.50 / 0.65 | 0.47 / 0.59 / 0.81 | was 0.72 / 1.07 / 1.39 after the gateway tests moved in at 15:25; fixed by (a) carving the presence family and (b) keeping only private-item gateway laws in-crate |
| `semio-framework-server-presence` (new) | 0.03 / 0.14 / 0.21 | 0.06 / 0.24 / 0.34 | carve of `gateway/👥️presence` |
| `semio-framework-server-tests` (companion) | - | 0.44 / 0.55 / 0.68 | public-API gateway laws + authority/policy/storage |
| `semio-framework-os-kernel-dsl-derive-expand` | 0.25 / 0.47 / 0.40 | 0.88 / 0.89 / 0.60 + 1 file for the schema-index law (inlined into the aggregator, no new file) | |
| `semio-framework-os-kernel-dsl-derive-tests` (companion, recreated) | - | 0.22 / 0.06 / 0.05 | source-parsing laws only |
| `semio-framework-os-services` | 0.44 / 0.61 / 0.76 | 0.47 / 0.62 / 0.77 | file-watcher law in-crate |
| `semio-framework-value-snapshot-read` | 0.25 / 0.39 / 0.33 | 0.88 / 0.77 / 0.47 | registry-lock laws in-crate |

(`/Users/ueli/...scratchpad/gate.ts` was the throwaway measurer; not kept.)

## Changes
- **dsl-derive-expand**: `testkit` feature and `testkit` module removed; the private-item laws (source authority, aggregate authority, leaf json/derive, attrs, mandatory mutations, composite attrs/timestamp, diff codecs, schema document index) mount from `derive/🦀️.rs` through `derive/🧪️tests/🦀️.rs` (`#[cfg(test)] mod tests`); dev-dependencies `semio-framework-test-fixtures`, `serde`, `sha2` added; fixture paths re-based to the expand manifest. The three source-parsing laws (macro export roster, product macro owner, base mutation contract) stay in the recreated companion `derive/🧪️tests/🌆️companion` (no `expand` dependency). Companion registered again (root `Cargo.toml` member, `📋️build-owners.json`).
- **services**: file-watcher law mounted inside `👀️file-watcher/🦀️.rs`; `FileStamp`, `previous`, `cancelled` private again; `testkit::file_watcher` and the companion `file_watcher` module removed.
- **value-snapshot-read**: the five test hooks (`state_lock`, `set_cleanup_cursor`, `alias_custody`, `slot_index`, `slot_generation`, `swap_owner`) removed. Companion laws use the existing public `ErasedSnapshotRead::lease_identity()`, `SnapshotReadLeaseRegistryOwner` deref (`identity()`, `clone()`); the lock-contention, lock/poison oracle and swapped-owner laws moved into `🧪️tests/🔒️registry-lock/🦀️.rs` (in-crate). `♻️snapshot-read-retirement` (already in-crate, stale `crate::os_store::` path and the removed hook) fixed to direct field access. Edited: store bin-01 (2 files), bin-02, bin-06, `♻️retirement/📦️backing/🧪️tests`.
- **server**: gateway `testkit` feature/module removed; the presence family (`gateway/👥️presence`, 839 lines, self-contained) is the new crate `semio-framework-server-presence` (`📦️packages/🦀️rust`), re-exported by the gateway through `pub use semio_framework_server_presence::*`; `Member/Room/Outflow/Share` fields that tco had widened for the companion are now `pub` only because the family crosses a crate boundary (previously `pub(super)`). The pure presence laws (colour leases, rooms, pages, budget, state frame) mount inside the presence crate (`👥️presence/🧪️tests/🦀️.rs`); the websocket helpers stay with the gateway tests. In-crate server tests are limited to the two laws that drive sibling-private functions (`🧪️tests/🔒️private`: relay framing, admission request) next to the reference instance (`cfg(test) test_instance` in the package glue); all other gateway laws run again from the companion through the public vocabulary (`🥣️companion/🛰️gateway/🦀️.rs` recreated). Registered: root members, `📋️build-owners.json`, server dependency.
- **store-prelude manifest** (not mine, unblocked the whole root workspace for ~35 min): `🏪️store/🧪️testing/🧭️prelude/📦️packages/🦀️rust/Cargo.toml` `workspace` key was one `../` short ("member of the wrong workspace"); fixed.
- Kept as feature-gated (`cfg(any(test, feature = "testkit"))`, not part of the production surface) because the consuming laws integrate several crates and cannot move: `OwnedSchemaRecordCursor::decode_pages`, `PresenceStore::local_root`, neural-model / one-item-publication serde derives, composition `OwnsRoot`/`LinkRoot` derives. Production-needed `pub` kept: `source_constructor_demand`, `ValidatedShellFrame`, `DocumentCatalogOwner::as_ref`, mcp-bridge `pub(crate)` session items (come from the split-fw carve, not from tests).

## Verification
- Source-gate ratios above: measured with the canonical reader (`readRustSourceClosureV1` + `crateTreeHostCfg()` plus `test`); all test cfgs <= 1.00; server, presence and both companions <= 0.90; expand and snapshot-read 0.88-0.89 on their tightest axis.
- Compile (`bun repo cargo ... check --tests --keep-going`, root workspace, detached jobs polled from the turn). First useful run 15:43-18:26: after splitting the `mandatory-mutation-descriptor` law, `semio-framework-os-kernel-dsl-derive-tests` compiled with zero errors. Last run (log `C/🗑️generated/tco2-b4.log`, 20:14-21:11): server, presence, server-tests, dsl-derive-expand, os-services(-tests) and the set-A crates are NOT reached, because the peer value carve currently fails first (`value-retained-clone-ordered-map` `clone-cursor`: missing `nested_retained_clone_*` / `retained_clone_grant_admits`; earlier `value-intrinsic`, `value-list-persistent`, `value-registry`). "After" error counts for those crates are therefore unknown; the failures are not caused by this task.
- NOT run: `cargo test --no-run` per crate; the framework-workspace check of `semio-framework-value-snapshot-read`.

## Set A (library carve defects), statically fixed after re-reading the current sources (the Oct-18 log was stale)
| crate | fix (errors in the old log) |
|---|---|
| `ui-render-text` | `FontSlot` `pub(super)` (3) |
| `surface-paint` | `AdjustmentParamsJson` `pub(super)` (3) |
| `editor` | `TextRangeJson` `pub(super)` (3) |
| `os-infinite-board-engine` | `force::apply_force_layout`/`apply_ported_force_layout` calls now name the crate-level functions (4); `use std::hash::{Hash, Hasher}` in `🎟️icon-paint-cache` (4); dependency `semio-framework-async` (1); `guard_decoded`/`from_value_controlled` (2) depend on the value API in flux |
| `os-infinite-world-components` | dependency `semio-framework-hash` (1) |
| `os-mcp-schema` | `EXPORTS: [SchemaExport; 75]` (1) |
| `os-flow-host-source` | `pub(super)` at the crate root -> `pub(crate)` (1) |
| `plugin-host-wit` | dependency `semio-framework-os-kernel` (2) |
| `plugin-interaction` | dependencies already present (3, resolved by peers) |
| `plugin-window-config` | 31 `RetireOwned` bound errors: derive already fixed by `type_mentions_directly`; unverified |
| `ui-wgpu-raster` | `super::` errors resolved by a peer (raster root shrank); unverified |
| not touched | `plugin-interpreter-runtime` (16: `crate::interpreter` path, `GUEST_DIAGNOSTICS_CAPACITY_BYTES`, `CoreError -> TurnFault`, `checkpoint_layout`), `plugin-operation` (1: E0446 `ToolCancellationLeaseOriginal`), `renderer-wgpu-media-slots` (2: `NativeDecodeContinuation`, `RetireOwned` derive vs trait): carve areas under active peer edits; need a fresh compile once the value crates build |

## Open
- Re-run when the value carve compiles: `check --tests --keep-going` for the crates above, then `cargo test --no-run -p <crate>` per touched crate, plus the framework-workspace checks of `value-snapshot-read`, `ui-render-text`, `surface-paint`, `editor`.
- `🔒️dependencies.json` is a generated lock listing manifests per dependency: rerun `dependencies lock` (new crate `semio-framework-server-presence`, new dev-dependencies of dsl-derive-expand, new dependencies of board-engine, world-components, plugin-host-wit).
- Residual feature-gated hooks (see Changes) need a coordinator ruling if they must go as well; `Member/Room/Outflow/Share` of the server gateway are `pub` because the presence family is its own crate.
- Cargo.lock was refreshed with `update --workspace --offline` at ~15:50; the later manifest edits (set A) need another refresh by the runner.
