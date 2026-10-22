# Result store-split (wave 1022a): semio-framework-os-store-engine source-bound split

## Start 06:52 (10-22)
Executor store-split. Live edits, no commits, tools in `store-split-tools/`.

## Findings
- Gate: `storage.sourceBoundary` 262144 bytes / 32 files / 4000 active lines per crate closure (`readRustSourceClosureV1`). Counter: `store-split-tools/closure.ts` (`V=1 bun closure.ts <lib root>`), census: `store-split-tools/census.sh <dir>`.
- Engine closure measured 06:44: 249,214 bytes / 13 files / 3,266 lines (the 06:27 gate failure counted ~277 KB; a peer shrank `⚙️engine/📥️lifted/🏪️store/🦀️.rs` at 06:29 and mounted `derived_operation_store` + `original_report_replay_store`, so the crate is only 12.9 KB under the bound and grows with every peer edit).
- Engine = inherent `impl ArtifactStore` blocks (Rust forbids an inherent impl outside the type's crate), so sibling crates can only take code that works on the component bundle `StoreParts`/`StorePartsRef` (engine-live) as extension traits.
## Plan
1. `semio-framework-os-store-engine-member-group` (21.5 KB): `🧩️composition/📬️publication/🤝️group/🦀️.rs` becomes the trait `ApplyBatchGroup` over `StoreParts`.
2. `semio-framework-os-store-engine-close` (~13 KB): `close_cursor_admit`, `close_structural_owners_terminal_is_empty`, `owned_roots_terminal_is_empty` as traits over `StoreParts`/`StorePartsRef`.
3. Re-point importers (space-member, os-kernel root, testing prelude, bin-01 test), source-text law of `owners/🏭️member/📥️source`, registries (root + os Cargo.toml, build-owners.json, crate-boundaries corpus), locks.
- 06:58 check1 pid 39957 log 🗑️generated/store-split-x/check1.cargo.log (close, member-group, engine, space-member, testing-prelude)

## Applied 06:57 (atomic, one script run: `store-split-tools/{mk_manifests,carve_close,carve_member_group,apply_rewire}.py`)
Engine closure now 215,633 bytes / 12 files / 2,940 lines (was 249,214 / 13 / 3,266); headroom 46.5 KB (17.7 %).
- NEW `semio-framework-os-store-engine-member-group` (`🧩️composition/📬️publication/🤝️group/{🦀️.rs,📦️packages/🦀️rust}`, 22.5 KB): `trait ApplyBatchGroup<P,Mu>` implemented for `StoreParts` (stage/adopt/abort_apply_batch_group); method bodies verbatim (`self.parts()`→`self`, `self.reserve_edit_history_slot()`→`self.owners.reserve_edit_history_slot(self.log)`). Callers: `self.parts().stage_apply_batch_group(..)` in space-member; tests via the prelude (`ApplyBatchGroup as _`).
- NEW `semio-framework-os-store-engine-close` (`⚙️engine/🧯️close/{🦀️.rs,📥️lifted/🏪️store/🦀️.rs,📦️packages/🦀️rust}`, 15.9 KB): `StoreCloseAdmission` (`close_cursor_admit`, `close_take_backbone_retirement`) for `StoreParts`, `StoreCloseWitness` (`snapshot_read_leases_terminal_is_empty`, `close_structural_owners_terminal_is_empty`, `owned_roots_terminal_is_empty`) for `StorePartsRef`; engine trait impl calls `self.parts().close_cursor_admit(..)` / `self.parts_ref().owned_roots_terminal_is_empty()`.
- Engine edits: removed `pub mod member_group` mount; cursor-admission child lost the close block; close-host + runtime-seed children import the two traits `as _`; engine Cargo.toml +close.
- Re-pointed: store root `use ..._engine_member_group as member_group`, testing prelude, os-kernel/space-member/prelude Cargo.toml, bin-01 test (2 call sites). No compat re-export.
- Source-text law `owners/🏭️member/📥️source` (`🔣️.json` + schema 2 spots): `storeCloseCursor.path` → `⚙️engine/🧯️close/📥️lifted/🏪️store/🦀️.rs`; ran `store-split-tools/owner-law.ts`: 79 checks green (includes the `cursor-current-refusal-owner-lost` mutation).
- Registered: root `Cargo.toml` members (2), `📋️build-owners.json` (2 rows), os crate-boundaries corpus (2 rows), `📋️project.json` per crate.
- Region markers: cursor child 12/12 balanced; new close lifted file one balanced `🔖️ArtifactStore` pair.
- 07:17 check1 failed on stale root Cargo.lock (--locked). chain1 pid 53888: update --workspace --offline then check (logs update1.cargo.log, check2.cargo.log, rc in chain1.rc)
- 07:48 check2 (after lock update, 22 min queue) gate passed; error E0599 in close witness (StoreOwners method needs Send+Sync+OpBinary bounds): fixed; unused imports dropped from member-group. chain2 pid 68700 check3 incl. os-kernel (log check3.cargo.log, rc chain2.rc)
- 07:59 check3 result: close, bundle, engine, member-group, space-member, testing-prelude all compile (warnings only); os-kernel fails ONLY at `🕰️history/🎮️command/🦀️.rs:70` E0599 begin_close on OriginalHistorySource (peer, not this split). Removed 9 import names made unused by the move from lifted/store/🦀️.rs. chain3 pid 82613 check4 --tests incl. tests-01 (check4.cargo.log, chain3.rc)

## Verification (end 08:12)
- Gate: crate-boundaries corpus test (`bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts -t "semio-framework-os-store-engine"`): 8 pass (engine, core, live, stage, bundle, owners + the two new rows). Final closures (`store-split-tools/closure.ts`): engine 215,183 B / 12 files / 2,938 lines (was 249,214 / 13 / 3,266; bound 262,144; 17.9 % headroom); close 15,944 B / 2 files; member-group 22,314 B / 1 file. The compiler ingress (source admission of all 205 units) passed in every cargo run below (no "Source refused").
- `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline` (root, 22 min queue): +2 packages in Cargo.lock.
- `check -p engine-close -p engine-member-group -p engine -p space-member -p testing-prelude [-p os-kernel]` (check3): all five Finished with warnings only. `--tests` run (check4): engine, close, member-group, space-member, prelude, fixtures compile; `semio-framework-os-store-testing-doubles` fails with 142 peer errors (`Mutation: ColdMutation<P>` bound missing in `🧪️testing/🎭️doubles/🗄️store`, peer's ColdMutation refactor), so `os-store-tests-01` (my 3 call-site edits in bin-01) is WRITTEN BUT UNVERIFIED.
- `semio-framework-os-kernel` fails with ONE error not from this split: `🕰️history/🎮️command/🦀️.rs:70` E0599 `begin_close` on `OriginalHistorySource<Edit<M>>` (file edited by a peer at 07:54; also the `🪪️member-facets/🧊️reference` file carries `os_store` tokens that fail the space-member crate-boundaries row, and durable-group-wire is 1,624 > 1,500 active lines in the corpus row; replay and ephemeral exceed 262,144 bytes only under cfg `test`, 269,223 / 263,337).
- Plugin workspace: apps-a's live runner (`🗑️generated/apps-a-x/a278c2b1-libs.log`, flow ext workspace) now passes source admission and compiles engine-close, engine, engine-member-group; it stops only at the same os-kernel history/command error.
- Source-text laws: `owners/🏭️member/📥️source` self-test 79 checks green with the moved `storeCloseCursor` path (`store-split-tools/owner-law.ts`).
## Open / hand-over
- Plugin and other standalone `Cargo.lock` files (64 + 45 plugin workspaces, hub, ✏️s) need `update --workspace --offline` again for the 2 new packages (apps-a/apps-b drivers do it per workspace).
- Remaining headroom owners: replay (214 KB), live (195 KB), core (189 KB) grow with peer edits; `derived_operation_store` / `original_report_replay_store` (16 KB, peer's new mounts in lifted/store) are the next carve if the engine grows again (not moved: the peer edits them live and several plugin crates call them).
