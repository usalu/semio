# r11-store-t2 execution report

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`, from `store_close_releases_a_returned_read_before_its_displaced_root` up to (excluding) `space_checkpoint_commits_dirty_members_and_pins_their_checkpoints` (the boundary to t3; the `register_space_documents_*` test just before it is mine).

## Result
- Gate `--tests` (`T/🗑️generated/r11-store-t2/check6.txt`, identical to check4/check5): 0 errors with a line in my range. Remaining 868 errors sit in t1's range (lines 0-3800, ~270), t3's range (7684+, ~220) and other test files.
- Because the crate's test target does not compile yet, rustc has not reached borrowck for the target, and I could NOT run `cargo test ... --lib`. No pass/fail counts exist for my area. The unit file was not touched by any peer between 04:50 and 05:13 (no `r11-store-t1`/`t3` gate dirs exist), so I stopped polling.

## What changed per category
1. Command dispatch needs a caller-owned `EntityIdentityAuthority` (677, not named in the brief): all 142 `X.dispatch(cmd).await` in the range became `test_support::dispatch_test_command(&mut X, cmd).await` (script `T/r11-store-t2-dispatch-rewrite.py`, balanced-paren aware, idempotent). `commit_space_checkpoint` in `register_space_documents_*` and the three `apply_ops_binary` calls (new helper `apply_ops_under_fixture_identity`) build the fixture authority inline/with the helper.
2. Retirement cursors on the new contract: `DemoRetainedCloneEditCursor` (+ `close_demand`, `retire_owned(mutation, grant)` returning the original on refusal, `artifact_retirement_box_close_step`), `RetainedTextEditCursor`, `ProbedRetainedClonePreparation(Factory)` (forwards `begin_demand`, grant-funded `begin`, `next_close_*`), `FaultingEphemeralTask` (now `ErasedSnapshotRetirement`, trait `ArtifactEphemeralPreparationTask` only keeps `advance`/`begin_close`).
3. Grants: `ArtifactStoreOneItemGrant {maximum_items, maximum_bytes}` -> `uniform_one_item_grant(items, bytes)` (every byte axis = the old ceiling, depth 64 = the fixture depth) and `fixture_one_item_grant(row)` for neutral fixtures. Close turns of publications/retirements are driven on the exact quoted per-axis demands: `close_retained_clone_preparation_publication` (asserts copy+capacity <= fixture byte ceiling and `fits(grant)`), `quoted_ephemeral_close_grant`, `drive_retirement_within(owner, items, bytes)` (fixture ceilings kept), `test_support::drive_retirement` elsewhere.
4. `SnapshotRetirementStep::{Pending,Blocked,Complete}` laws re-expressed: "blocked on a live reader/peer" is now "a funded turn at the exact quoted demand makes zero progress" (`close_stalls_at_reader_boundary`, backbone live-peer test: `Progress(default)` while the peer lives, non-default progress after it drops); released_items became `copied_items`, per-axis `released_bytes` stays asserted via `fits`.
5. Store/Presence APIs: `retire_snapshot_read_erased(&mut Option<_>, grant)`, `maintenance_retirements_demands/step(grant)`, `maintenance_local_reads_step(grant)`, `PresenceStoreRetirement`/cursor/conflict/string-vector retirements driven by `drive_retirement`, `presence.local` is a `ManuallyDrop<Option<Arc>>`.
6. Upper-bounded laws replaced faithfully: the old 13-byte conflict close and 7-byte presence close bounds are replaced by quoted-demand driving (those bounds were `maximum_bytes` pairs that no longer exist).

## Open items / risks for the runtime test run (not verified)
- `ProbedRetainedClonePreparationFactory::begin` forwards the inner receipt unchanged although it boxes one extra wrapper; if the store checks the birth receipt against `size_of_val` of the returned box, that test (`interrupted_retained_clone_publication_handoffs_to_store_maintenance`) will need the wrapper size added to demand and receipt.
- `Progress(default)` as the "blocked" witness assumes the lib yields (never errors) on a live reader/peer, as the disposer/backbone code does.
- `close_retained_clone_preparation_publication` asserts the quoted copy+capacity stays within the lifecycle fixture's 4096; if a quote exceeds it that is a lib/law question for `r11-store`.
- Outside my files: `🔌️plugin/🖥️host/🦀️.rs:6150` still calls `(codec.apply_ops_binary)` with 3 arguments (needs the identity authority).
- `close_demo_artifact_store` / `close_durable_publication` (t1 range, still the 676 API) are used by many of my tests and must keep their `(&mut store)` / `(&mut publication)` signatures.
- Left untouched: the `dispatch` calls from `space_checkpoint_commits_dirty_members_and_pins_their_checkpoints` on (t3).

Scratch/inputs under `T/🗑️generated/r11-store-t2/` (patch scripts `p1..p7.py`, `patch.py`, `errs.sh`, `check*.txt`).
