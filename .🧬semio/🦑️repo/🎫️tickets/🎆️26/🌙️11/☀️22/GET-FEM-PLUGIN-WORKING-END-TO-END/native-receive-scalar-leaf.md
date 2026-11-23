# NativeReceive scalar-leaf path and honest receiving: result and open items

Scope: framework `NativeReceive` derive (`🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive`), owned receiving runtime, FEM 2D/3D snapshot receiving. Private dirs `C:\t\nr`, `nrb`, `nrtmp` (deleted at the end). Builds `-j 3` after the RAM notice.

## Verdict
- 2 of the 5 targets are fixed or measurably close: capacity law passes; actions and capacity per node cut by 2.4x and 3.1x.
- 4 targets are still open: 10k-node fixture and scaling (copy charge 395 KB/node vs 52 KB/node allowed), `native_owner_receives…` and the sqlite `…actual_interior_owned_work` (cancel while a child action is pending), and the 1000-cut load test (not run).
- The tree is flapping under me: the fleet replica rewrote the derive and tree files at 03:39, 05:17 and earlier from older snapshots and erased my edits three times. Re-check the "Changed files" list below before building.

## Measurements (2D `receive_authoring`, debug, per node)
| | before | after |
|---|---|---|
| FemNode receiver size | 66192 B | 22432 B |
| retained capacity charged | 182 KB | 58 KB (limit 64 KiB) |
| copy bytes charged | 686 KB | 395 KB (limit 52 KB) |
| funded actions (copied_items) | 1056 | 430 |
| work units | 1430 | 653 |

Where the remaining 395 KB comes from (action histogram of a 2-node receive, 1395 actions): about 300 of them charge 1 to 8 KB each, because the effect-frame quotes are sized by the inline failure types (`OriginalReceivingPrimary` 2544 B, `OriginalRetirementFailure` 336 B, `NativeReceivingEffect` 5680 B) even on the success path. 214 actions charged exactly 2880 B = 2544 + 336 (two reply slots). The remaining ~430 actions per row are the sealed-shared id (`SealedShared<String>`: two shared receivers of 12.4 KB each, bind/birth/take/close/release) and row adoption by the list builder.

## Design implemented
1. Scalar-leaf path (derive): a bare primitive field (`bool f32 f64 u8..u64 usize i8..i64 isize`, no flatten) owns no `NativeReceiveSlot`, no projection alias and no close frontier. One funded action (25) projects the child (a borrow, no allocation), reads it with `NativeScalarValue::read` into the record's `pending_*` value and charges `size_of::<Option<T>>()`; one action (13, stage 3) places it. Slot-less fields drop 15.7 KB each and about 7 funded actions plus about 7 close actions per field. Close of a half-received leaf reuses the existing `pending`/`initialized` custody (`retirement.begin_granted`, `remove_field`).
2. Ordinal key fast path (derive, flat records only): when the original source reports `record_key(i) == Ordinal(n)` the scan is one action (26) instead of a key matcher with a 17.5 KB heap frame and about 125 actions per key. Name keys keep the matcher unchanged. Flattened and tuple records keep the old path.
3. One failure owner per record: the per-record `effect` and `retirement_effect` fields (12 KB, never read) are removed from record and enum receivers. Failure frames now live only in the slots that drive child effects. Enum seed table lost the two matching parts.
4. Honest copy charging (partial): `acknowledge_success_demand` and `finish_success_demand` quote the live successful bytes (step, work, tags) instead of failure-sized structs. Little effect on the measured copy (the 2880 B charge is quoted elsewhere); the complete fix needs the remaining quote sites in `🧳️frame/🦀️.rs` and `🏭️producer-engine/🦀️.rs` (`reply_release_demand`, `transfer_demand`, `resident_success_ack_demand`) to charge by the live variant.
5. Original-loan close for records (derive): `close_poll` threads `Option<&mut OriginalRetirementCloseLoan>` to slots (`close_original_into`), default slots, matcher, projection, retirement and binding, and the record receiver implements `requires_original_birth/control` and `close_original_birth_step`/`close_original_step` in the same owner order as the close path (the tuple receiver already did).
6. Product receivers `Fem2dReceiving` and `Fem3dReceiving`: same original-loan close plumbing for the node/material slots and the retirement; `advance` skips its top-level `checkpoint` while a child action is pending so the pending child can finish before cancellation (unverified, see open).

## Changed files
Framework:
- `✨️derive/🛬️receive/🦀️.rs` (leaf, ordinal path, effects removed, original close, state helpers) and `🔀️enum/🦀️.rs` (effects removed)
- `🛬️decode/🧳️owned/🌳️value/🦀️.rs` (`plan_leaf_stage`)
- `🛬️decode/🏭️construction/📦️effect/🧳️frame/✅️success/🦀️.rs` and `🎟️control/📨️reply/🏭️producer-engine/🦀️.rs` (honest success quotes)
- new test `🛬️decode/🧳️owned/📦️packages/🦀️rust/tests/native-record-ordinal-leaf.rs` (ordinal scalar record: size under 8 KiB, at most 40 funded actions, 40 cancelled prefixes close)
Product:
- `🗿️artifacts/◻️2d` and `🧊️3d` `🧬️schema/📸️snapshot/native.rs` (original close, pending-child checkpoint)
- `🧪️tests/native-tests.rs` 2D and 3D: `native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix` now runs through `original_owners::with_decoding_granted` with a counting cancel callback (receive, cancel at checkpoint N, close by the decode owner, final cut asserts the rows); all my `[DEBUG]` tests removed.
Peer debug lines removed (they broke isolated builds): `🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs` and `…/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs` (`[DEBUG] … semio_framework_job::testkit`).

## Tests, before and after
- Framework `semio-framework-value-native-owned`: all suites green after the leaf, ordinal and effect-removal changes (19 suites ok; includes the new ordinal test, 2 of 2). In the last run 6 tests fail only because the fixture `🔗️sealed-shared-receiver/🧪️tests/🧫️fixtures/📐️.json` was missing from the tree at that moment (replica flux); one earlier run showed 50 failures from a changed testing-admission harness, not from record code.
- Pre-existing red, not mine: `semio-framework-value-native-schema-engine` tests (missing `settlement_of`), `semio-framework-value-native-decoding-tests` (62 compile errors, trait drift), derive `canonical_tree::octet_roles…`, `snapshot_projection::…`, `factory::…`, and derive receive tests `derived_record_and_enum_frontiers…` (asserts `try_project_owned`, absent at HEAD), `generated_original_closure_ports…` (expects `OriginalRetirementPoll`), `generated_receivers_use_the_canonical_nested_primary…`.
- 2D snapshot-tests lib, final build: `receiving_charges_below_sixty_four_kibibytes…` ok (was failing); 10k-node and scaling fail on copy; `native_owner_receives…` fails in the decode-owner close (see open). Other 37 tests: not re-run after the framework change.
- 3D: `native.rs` and `native-tests.rs` edited identically; not compiled (peer breakage in `🏪️store/🏗️initialization/🔁️replay` stopped `cargo check` at 834:316). The `Fem3dReceiving` close fix from the earlier pass is unbuilt as well.

## Open, with the exact cause
1. Cancel with a pending child action: failing cuts 237, 241, 245, 250, 254, 258, 262, 266, 270 (every ~4th checkpoint from 237). A `[DEBUG]` caller location on `tree::invalid()` pinned the refusal to `NativeReceiveSlot::begin_close_demand` (`🌳️value/🦀️.rs:44`, `if self.advance_pending(){return Err(invalid())}`): the product receiver quotes the close of a node slot whose original action is still pending, because the cancelled turn left it unresolved. The original-loan close plumbing did not change the failing cuts; the `advance` change (skip the top checkpoint while a child is pending) targets this but its build was blocked by peer breakage, so it is unverified. The same cause explains the sqlite `…actual_interior_owned_work` failure (same message); 3D twin unchecked.
2. Copy charge 395 KB/node vs 52 KB/node: needs (a) the remaining E-sized quotes charged by live variant (see item 4), (b) fewer actions in the sealed-shared id and list row adoption (about 430 per row, three quarters of the copy), (c) re-seeding a recycled row receiver. A peer is landing receiver seed templates for records (`receiver_template`, seeded in field-sized parts) in the same derive file; its seed table now excludes slot-less leaves and the removed effect fields, so it composes with this change.
3. Load test `every_actual_checkpoint_prefix…`: unchanged semantics, not run; calls per row dropped from about 1060 to 430, so the O(calls x decode) loop is about 2.5x shorter.
4. Replica clobbering: re-apply from `scratchpad` scripts if the derive or `plan_leaf_stage` edit is missing (`grep -c scalar_leaf` in the derive, `grep -c plan_leaf_stage` in the tree file).

## Round 2
- Edits intact: `scalar_leaf` count 2 in the derive, `plan_leaf_stage` count 1 in the tree file, at start and before each build (no clobbering seen this round).
- Build (native cargo, private `C:\t\nr`, `-j 3`, script `nr2-build.sh` in the ticket folder; cold 2D snapshot-tests lib build 28 min) of the `advance` fix (product `native.rs` line 254, skip top checkpoint while a child is pending): `native_owner_receives…` STILL FAILS in 2.1 s: `original decode recipient closes: OriginalRetirementFailure { primary: InvalidValue "native receiving original type, field or phase is invalid" }`. So the pending-child skip alone does not fix it; log: `🗑️generated/r2-build1.log` line ~99250.
- Reading the close path: the refusal is the quote at the top of `Fem2dReceiving::close_poll` (`self.close_demands(body)` -> list receiver -> node slot `begin_close_demand`, which rejects while `advance_pending()`), reached through `close_original_step`. `NativeReceiveSlot::close_borrowed` also refuses (`action.pending()`), and nothing in the close route first settles/recovers a pending original action of a slot; the slot's `advance` only recovers while not `closing`. Fix direction (unbuilt): when `requires_original_control()` is true and a slot action is pending, the receiver's close route must first drive that slot's `advance` recovery (same `body/maximum_work`) before quoting close, or the slot must quote close of a pending action as "recover the recorded action" instead of invalid.
- A product-only `[DEBUG]` print in `close_poll` (phase/pending/control flags) was built to pin which cut and level; the second build never finished: the host CPU stayed at ~98% from other sessions, my build advanced ~10 crates per 30 min (131 of ~650 after 4 h; peers' edits also triggered rebuilds). I stopped it; debug line removed (grep verified), my cargo PIDs killed, `C:\t\nr`, `nrb`, `nrtmp` deleted.
- Goal 2 (copy 395 KB -> 52 KB), goal 3 (3D twin, load test): not started; no code written this round, since nothing could be verified on this host load. Plan unchanged from the Open section: quote effect frames by live variant (`reply_release_demand`/`transfer_reply_demand` use `size_of::<Option<Result<Output,Failure>>>()`, i.e. failure-sized; quote the success arm when the recorded reply is `Ok`), and fewer per-row actions in `SealedShared<String>` bind/birth/take/close/release.
