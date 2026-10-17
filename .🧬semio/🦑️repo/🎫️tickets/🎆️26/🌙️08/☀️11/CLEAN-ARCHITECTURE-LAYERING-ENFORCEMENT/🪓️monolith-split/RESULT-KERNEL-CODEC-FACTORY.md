# 🏭 Result KERNEL-CODEC-FACTORY (prefix `kcf`): the four open items of PRIVACY-ORACLE-SWEEP-2

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`, `M` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `S` = `M/🏪️store`. Tools: `F/kcf-tools/` (`dgraph.ts`, `manifests.ts`). Logs: `F/🗑️generated/kcf-*` (oracle runs `k1` and `k2`, census). Sandboxes: `scratchpad/kcf-work`. Time: 21:20 to 21:50.

## 0. Verdict

All four targets are done and statically verified. Nothing is compiled: compile admission is closed for the fleet (wave 6 brief), so every claim below is static evidence from the round-2 oracle, the census and the manifest tools, and the real-check commands are recorded in section 6.

1. **`os-io` ranks below `codec-factory` (27 against 35), so the edge was added, not the trait moved.** `os-store-codec-factory` now depends on `semio-framework-os-io`; acyclic; one consumer (`P::sqlite_codec()`); the oracle's `E0599` on `sqlite_codec` is gone (lib and test).
2. **`ephemeral` presence retirement test: 4 dev-dependencies added** (`semio-framework-async`, `-io-control`, `-job-worker`, `-job-worker-authority`); no path back to `ephemeral` from any of them; the 5 `E0433` rows of the test configuration are gone.
3. **Ledger widening: 13 members reviewed, 6 data fields narrowed back, 7 kept `pub` on evidence of cross-crate use.** The 6 fields (`StackMirrorDirt.{applied,redo}`, `PendingCommandReport.{edit_ids,messages,worst,outbound}`) are `pub(crate)` again, replaced by 17 intent-named methods of the owner, and all 35 field-access sites in 4 consumer files (3 crates plus the codec factory) were rewritten. Final public delta against `HEAD`: 24 pub members, 0 pub data fields (section 3).
4. **Hand-off written:** `F/HANDOFF-3D-MESH-CARVE-IMPORTS.md` (7 rows = 3 causes, exact files and lines; one of them, the missing `point` binding at `reconstruction/🦀️.rs:271`, is a behavioural gap and not only an import). No file of the peer was edited.

Oracle (`okernel`, 49 crates, lib and `--test`, canary reached in both): **0 new errors**, 1 gone in lib, 6 gone in test, privacy 0, trait-scope 4 to 3.

## 1. Target 1: codec-factory needs a trait that lives in `os-io`

**Evidence.** `S/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs:54` (`artifact_codec`) calls `P::sqlite_codec()` under the bound `P: ArtifactSqliteSnapshotHost`. `sqlite_codec` is a method of the blanket trait `ArtifactSqliteSnapshotOperations` (`S/📦️codec/🪶️snapshot-capability/🧰️codec-operations/🦀️.rs`, mounted by `os-io` as `sqlite_snapshot_operations`). The blanket impl builds the codec out of `io_mechanism::export_snapshot_file_read` and `import_snapshot_file_into`, which are items of `os-io`, so the trait cannot sit lower than `os-io` (moving it down would drag the IO mechanism down with it).

**Decision from the rank order (PLAN §4.3, `okwp0-tools/okcrates.RANK`).** `os-io` 27, `os-store-codec-registry` 26, `os-store-engine` 33, `os-store-codec-factory` 35. `os-io` is lower, so the rule says: add the edge. No port is needed (§5.3 prescribes ports only for the upward cuts C1 to C3, L1 to L3).

**Cycle check.** Manifest graph over all 2,408 first-party packages (`kcf-tools/dgraph.ts reach semio-framework-os-io semio-framework-os-store-codec-factory`): no path through normal, build or dev dependencies in either direction. `okeng-tools/graph.py` (rank order over the 18 engine crates plus the lower crates): `back edges 0 | cycle False`, and the new edge shows as `os-store-codec-factory -> ... os-io ...`.

**Edits (all re-read before the edit, anchored, asserted unique).**
* `S/📦️codec/🏭️factory/📦️packages/🦀️rust/Cargo.toml`: `semio-framework-os-io = { path = "../../../../../🚪️io/📦️packages/🦀️rust" }` as the first `[dependencies]` line (the path resolves; `manifests.ts` checked 18 path dependencies).
* `S/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs`: `use semio_framework_os_io::sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations;` (the only consumer; implementors: the blanket impl in `os-io`, unchanged). A scan for `X::sqlite_codec()` outside the fully qualified `<Self as ...Operations>::` form finds exactly this one call in the framework tree.
* `Cargo.lock`: `semio-framework-os-io` added to the `semio-framework-os-store-codec-factory` stanza (the stanza is stale in other respects: it still lists `initialization-replay` and `owned-schema-fields`; the lock refresh is the loop's job, section 6).

**Notes.** `os-io` enables `semio-framework-async/entrypoint` (gates `block_on` to process entry points); the kernel already depends on `os-io` unconditionally, so the feature set of every real consumer is unchanged, but any future consumer of `codec-factory` alone now gets the feature too. Census: `codec-factory` 2 files / 22,856 bytes / 357 lines, inside the limits, `closureCompilable=true`.

## 2. Target 2: `ephemeral` presence test dev-dependencies

**Evidence.** `S/👥️presence/♻️retirement/🧪️tests/🔬️unit/🦀️.rs` (mounted by `os-store-ephemeral` as `presence_retirement`) names `semio_framework_async` (lines 152, 154, 214), `semio_framework_io_control` (5, 15, 346, 356), `semio_framework_job_worker` (149) and `semio_framework_job_worker_authority` (131 to 183). `pos2deps.py` listed exactly these four as the only undeclared heads of the whole closure.

**Direction and registry.** All four are framework-level crates (`🧰️framework/🔨️modules/⏳️async`, `🚪️io/⏱️control`, `🧵️job/👷️worker`, `🧵️job/👷️worker/🎟️authority`), below every `os-store-*` crate; `ephemeral` already depends on `semio-framework-job`. `dgraph.ts reach <each> semio-framework-os-store-ephemeral`: no path with normal, build or dev edges. All four are packages of the root `Cargo.lock`. Only `semio-framework-async` is in the root `[workspace.dependencies]`, and the other three are not, so all four are declared by path (the form `ephemeral` uses for `semio-framework-job`); no features (the test uses nothing that the crates gate behind `testkit`).

**Edits.** Four `path` lines appended to `[dev-dependencies]` of `S/🫧️ephemeral/📦️packages/🦀️rust/Cargo.toml` (21 path dependencies checked), the four names added in sorted order to the `os-store-ephemeral` stanza of `Cargo.lock`. Result in the oracle: the 5 `E0433` rows `cannot find module or crate semio_framework_job_worker(_authority)` of the test configuration are gone. `semio_framework_async` and `_io_control` were not reported by the sandbox (they resolve through the stub layer), so only `pos2deps.py`'s manifest evidence covers them.

## 3. Target 3: the ledger widening (13 members)

The 13 members are exactly the 13 lines of `git diff -- S/📒️ledger/🦀️.rs` against `HEAD` (checked: `13 insertions, 13 deletions`), all `pub(crate)` to `pub`. Every consumer was found by `git grep -F` over all `.rs` of the repository (not only the store tree).

| # | member | users outside `os-store-ledger` | decision |
| ---: | --- | --- | --- |
| 1 | `StackMirrorDirt::CLEAN` | `engine` (2 sites), `engine-live` (1) | stays `pub`: a constant value, already the narrowest form |
| 2 | `StackMirrorDirt::ALL` | `engine` (2), `engine-live` (1) | stays `pub` |
| 3 | `StackMirrorDirt.applied` | `engine-core` (1 read, 2 writes), `engine-live` (2 writes) | field back to `pub(crate)`; owner methods `first_applied`, `dirty_applied_from` |
| 4 | `StackMirrorDirt.redo` | `engine-core` (1 read, 2 writes), `engine-live` (1 write) | field back to `pub(crate)`; owner methods `first_redo`, `dirty_redo_from` |
| 5 | `CursorRevisionAccumulator::hash_record` | at least 7 crates (`engine`, `-live`, `-stage`, `-core`, `one-item-publication`, `initialization-runtime`, `replay`) plus the composition group code | stays `pub`: the domain-separated digest primitive |
| 6 | `CursorRevisionAccumulator::new` | `engine` (1) | stays `pub`: a constructor with a cross-crate caller |
| 7 | `CursorRevisionAccumulator::effective_edit_digest` | `engine` (1), `initialization-runtime` (2) | stays `pub` |
| 8 | `PendingCommandReport.edit_ids` | `engine`, `engine-live`, `codec-factory` | field back to `pub(crate)`; methods `declare_edit_ids`, `declare_edit_ids_unless_declared`, `take_receipt`, `is_idle`, `resident_backings(_mut)` |
| 9 | `PendingCommandReport.messages` | `engine`, `engine-live`, `codec-factory` | field back to `pub(crate)`; methods `record_replay`, `record_edit_messages`, `take_receipt`, `take_messages_and_worst`, `is_idle`, `holds_owner_slot`, `resident_backings(_mut)` |
| 10 | `PendingCommandReport.worst` | `engine`, `engine-live`, `codec-factory` | field back to `pub(crate)`; same methods |
| 11 | `PendingCommandReport.outbound` | `engine`, `engine-live`, `codec-factory` | field back to `pub(crate)`; methods `author_outbound`, `has_outbound`, `take_outbound`, `is_idle`, `holds_owner_slot`, `resident_backings(_mut)` |
| 12 | `ArtifactStorePendingReportRetirement::new` | `engine`, `engine-live`, `engine-stage` | stays `pub` |
| 13 | `ReplayProjection::new` | `engine-core`, the document code in `space-member` | stays `pub` |

Result: **0 of the 13 was used only inside its own crate** (the widening had been driven by real cross-crate uses), so nothing is narrowed by deletion; the six data fields, which several crates mutate directly, are narrowed by giving the owner the methods. Seven members stay `pub` on the evidence of the table.

**New owner API** (all in `S/📒️ledger/🦀️.rs`, docstrings start with a distinct emoji, no comments inside bodies):
* `impl StackMirrorDirt`: `first_applied`, `first_redo` (const, read), `dirty_applied_from(position)`, `dirty_redo_from(position)` (the `min` the callers wrote by hand).
* `impl PendingCommandReport`: `is_idle`, `holds_owner_slot`, `has_outbound`, `author_outbound(iter)`, `take_outbound`, `record_replay(messages, worst)`, `record_edit_messages(edit_id, messages)`, `declare_edit_ids`, `declare_edit_ids_unless_declared(f)`, `take_receipt(undeclared)`, `take_messages_and_worst`, `resident_backings`, `resident_backings_mut`. The last two exist because `resident_backing::close_vec` and `vec_demand` live in `os-store-retirement`, which ranks above the ledger (12 against 10), so the release step cannot move into the ledger without an upward edge.

**Consumers rewritten** (35 sites; re-read, anchored, `git diff --stat` after each file):
* `S/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs`: 6 sites (`sync_cursor` reads, four `mirror_dirt` writes).
* `S/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs`: 3 `mirror_dirt` writes, `pending_report_owner_slots` (now `holds_owner_slot`), the outbound, replay, merge, edit-message, edit-id sites and `flush_outbound`.
* `S/⚙️engine/📥️lifted/🏪️store/🦀️.rs`: resident-backing demand, close step and terminal check (3 sites), the two shell-empty checks, the four receipt builders, outbound and edit-id sites, one docstring.
* `S/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs`: the four-line idle check is `pending_report.is_idle()`.

**Two deliberate semantics, both reasoned and stated:** (a) the shell-empty check in `close_structural_owners_terminal_is_empty` (engine, previously three of the four fields) now uses `is_idle`, which also requires `worst == None`; at that point the report was already replaced by `PendingCommandReport::default()` in the close admission step (engine line 816), and the `Drop` check required `worst == None` anyway. (b) The supersession receipt (engine, previously `edit_ids: Vec::new()` while leaving `edit_ids` untouched) uses `take_messages_and_worst`, not `take_receipt`, so it still leaves a declared id list in place, exactly as before.

**Final public delta of the ledger crate against `HEAD`** (computed by diffing the `pub` member lines of `HEAD:S/📒️ledger/🦀️.rs` and the working file; nothing removed from the public API):

| group | members | count |
| --- | --- | ---: |
| kept from the 13 (cross-crate, section table 1, 2, 5, 6, 7, 12, 13) | `StackMirrorDirt::{CLEAN, ALL}`, `CursorRevisionAccumulator::{hash_record, new, effective_edit_digest}`, `ArtifactStorePendingReportRetirement::new`, `ReplayProjection::new` | 7 |
| new owner methods | `StackMirrorDirt` 4, `PendingCommandReport` 13 | 17 |
| public data fields | none (the pos2 state had 6) | 0 |
| **total added to the public API** | | **24** |

Against the pos2 state (13 widened members, 6 of them mutable public fields) the surface is 11 members larger but exposes no field and no invariant: the `edit_ids` override rule, the worst-level fold and the outbound queue are written once in the owner. `#[doc(hidden)] pub mod engine_api` (pos2's alternative) was not used: it would hide the same 24 names without removing the field access.

The other 16 widenings of pos2 item 7 (outside the ledger, not part of this target and not reviewed): `replay::new_counted`, `one-item/publication::{mint, new}`, `replay` read `new`, `operation-identity` `new` x2, `backbone/retirement::{admit_attached, constructor_capacity_bytes}`, canonical-edit lifted `new`, `read::new`, `owned-schema/fields::new`, `messages/clamp/settlement::new`, `canonical-edit/source::new`, `sync` `authority`, `MemberOpenFrame.{snapshot_start, history_start}`. Their recorded use sites are other crates' engine files for 11 of them; for `canonical-edit/source::new`, `sync authority` and the two `MemberOpenFrame` fields the recorded user is a unit-test file (`F/pos-tools` plan `pos2-okwp8-all-widen-plan.json`), which may belong to the same crate or to the relocated kernel tests; those four are the first candidates for a `pub(crate)` or `pub(super)` re-check.

## 4. Target 4: hand-off to the 3d mesh carve

`F/HANDOFF-3D-MESH-CARVE-IMPORTS.md`. Seven oracle rows, three causes, re-verified in the working tree on 21:28: `HashMap` at `M3/🛠️modeling/🦀️.rs:575` (2 rows; line 12 needs `HashMap` in the `std::collections` import), `Vec3f64` at `M3/🛠️modeling/🦀️.rs:856, 857, 858, 889` (4 rows; the alias is private at `M3/📄️document/🦀️.rs:590`), the local `point` at `M3/🛠️modeling/🔁️reconstruction/🦀️.rs:271` (1 row; the `let point=self.snapshot.soup.positions[self.cursor];` of the baseline was dropped when the weld key moved into `mesh_weld_key`; keep line 270 byte-identical because `M3/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts:153` pins it). `M3` = `🧰️framework/🔨️modules/🧊️3d/🥽️mesh`. No peer file was touched.

## 5. Verification (commands run, real results)

| gate | command | result |
| --- | --- | --- |
| rank order, changed edges | `python3 okeng-tools/graph.py` (rank list of `okwp0-tools/okcrates.RANK`) | `back edges 0 | cycle False`; `os-io` 27 below `codec-factory` 35 |
| cycle, new edges | `bun kcf-tools/dgraph.ts reach <a> <b>` for `os-io` to `codec-factory` and for each of the 4 dev crates to `ephemeral` (normal+build, and with dev) | all `null` (2,408 packages indexed, no duplicate package name) |
| TOML and paths | `bun kcf-tools/manifests.ts` on both edited manifests; `Bun.TOML.parse` on `Cargo.lock` | codec-factory 18 path deps ok, ephemeral 21 path deps ok, 0 duplicates, `problems 0`; the lock parses (593,038 bytes) |
| rustfmt parse | `rustfmt --edition 2021 --emit stdout --config skip_children=true` on the 5 edited `.rs` | all 5 parse |
| source-boundary census | `ONLY=<6 crates> bun F/🗑️generated/census.ts` (`kcf-census.log`) | whole workspace: 766 crates, 766 compilable, 0 oversized; `ledger` 9 files / 88,566 B / 1,238 lines; `engine-core` 3 / 184,198 / 2,693; `engine-live` 3 / 190,324 / 2,705 (was 191,674 / 2,720); `engine` 4 / 254,081 / 3,451 (was 254,589 / 3,452); `codec-factory` 2 / 22,856 / 357; `ephemeral` 6 / 133,436 / 1,964; all `overLimit=false` |
| oracle, kernel family, lib and test | `POS_PREFIX=kcf python3 pos.py index --force`; `POS_PREFIX=kcf python3 pos2batch.py --jobs 1 --label k1 --cfgs lib,test --no-base okernel` (452 s, rc 0) | **canary reached** (E0308, E0616, E0624 reported) in both; lib 37,804 errors (37,278 stub noise), test 40,996 (40,247); privacy 0 and 0; trait-scope 3 and 3 (v2: 4); resolve 55 and 59 (v2: 55 and 64); type 429 and 646 |
| diff against the v2 run of PRIVACY-ORACLE-SWEEP-2 | keyed non-stub rows (`code`, file tail, message; line numbers excluded) of `pos2-okernel-v2-result.json` against `kcf-okernel-k1-result.json` | **0 new rows** in lib and test; gone: lib 1 (`E0599 sqlite_codec` in codec-factory), test 6 (the same plus 4 `E0433 semio_framework_job_worker_authority` and 1 `E0433 semio_framework_job_worker` of the presence test) |
| sensitivity control for the narrowed fields | `pos.py run families/okernel.json --cfgs lib --label k2 --no-base` with 4 sandbox-only `--mutate` re-introductions of a direct field access (`kcf-okernel-k2-report.txt`, 82 s) | privacy 4: `E0616` for `outbound` (engine), `edit_ids` (engine-live), `worst` (codec-factory) and `applied` of `StackMirrorDirt` (engine-core). So the 0 privacy rows of `k1` are evidence that no external access to the six fields is left in the 49-crate family |
| repository-wide consumer scan | `git grep -F` for `pending_report`, `mirror_dirt`, `PendingCommandReport`, `StackMirrorDirt`, `sqlite_codec()` over all `.rs`, `.ts`, `.json` | users only in the 4 files above; one TS law pins `pending_report: std::mem::ManuallyDrop<PendingCommandReport>` (unchanged text, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🗄️artifact-store/🟦️.ts:1277`) |
| real compile | not run (compile admission closed, wave 6 brief) | commands in section 6 |

The ICE body replaced by the oracle (`StoreLog::open_conflicts`, `engine-core` line 631) and the remaining rows (55 resolve, 3 trait-scope on `progress` of an uninferred `_` in `child` and `snapshot-clone`, 429 type rows) are the same stub-layer residue as in PRIVACY-ORACLE-SWEEP-2 section 2; none is in a file or site of this slice.

## 6. Real-check commands and hand-offs

Run in this order when compile admission opens, one launcher call at a time under the fleet mutex (`🗑️generated/cargo-mutex.lock`):

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- update --workspace --offline
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-store-ledger -p semio-framework-os-store-engine-core -p semio-framework-os-store-engine-live -p semio-framework-os-store-engine -p semio-framework-os-store-codec-factory -p semio-framework-os-store-ephemeral -p semio-framework-os-store-document -p semio-framework-os-store-space -p semio-framework-os-kernel --lib --tests --keep-going --message-format=short
```

(The store crates are members of the root workspace; no `--manifest-path`.) What the oracle cannot see and the compile must confirm: (a) borrow checking of the closures that take `self.log` while `self.replay.pending_report` is borrowed mutably (`take_receipt` at engine lines 974, 989 and 1333; disjoint fields, edition 2021 precise capture, expected to pass); (b) that the four new presence-test dev-dependencies need no feature (`testkit` is declared by `job-worker` and `job-worker-authority`; the test calls nothing gated that I could find); (c) the hand-written `Cargo.lock` stanzas, which the `update` call will normalise.

Hand-offs: the 3d mesh carve owner (`HANDOFF-3D-MESH-CARVE-IMPORTS.md`); the compile queue (above); a re-check of the four test-only widenings listed at the end of section 3; the `ri2` renderer privacy errors (298, PRIVACY-ORACLE-SWEEP-2 section 5) remain with `ri2`/`ri3` and were not touched.

## 7. Files

Edited: `S/📦️codec/🏭️factory/📦️packages/🦀️rust/Cargo.toml`, `S/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs`, `S/🫧️ephemeral/📦️packages/🦀️rust/Cargo.toml`, `S/📒️ledger/🦀️.rs`, `S/⚙️engine/🧱️core/📥️lifted/🏪️store/🦀️.rs`, `S/⚙️engine/🎞️live/📥️lifted/🏪️store/🦀️.rs`, `S/⚙️engine/📥️lifted/🏪️store/🦀️.rs`, `Cargo.lock` (2 stanzas: `codec-factory` +1 line, `ephemeral` +4 lines). Created: `F/HANDOFF-3D-MESH-CARVE-IMPORTS.md`, `F/RESULT-KERNEL-CODEC-FACTORY.md`, `F/kcf-tools/dgraph.ts`, `F/kcf-tools/manifests.ts`, `F/🗑️generated/kcf-*` (oracle `k1`, `k2`, census, manifest index). No `[DEBUG]` line was added, nothing was deleted, no peer file was touched.
