# Brep Snapshot Size: Why the Forest Stock Open Costs 56 MB

Status: root cause measured; option 1 plus the follow-up cuts implemented (see the two result sections at the end). Receive slot 15,712 B to 9,968 B, forest seed 63.0 M to 28.7 M. `bearbeiten_pane_loads_applies_undoes_and_closes` is STILL red: the load now fits only just beyond the fuel and the rest of the test needs more. See Round 2.

## Verdict

The decoded brep is not large. The forest stock brep is a 13,686 B pack (not the 78 KB STP) with 71 vertices, 126 edges, 57 loops, 57 faces, 252 coedges, 252 loop edges and 57 shell faces: about 870 rows, roughly 0.2 MB as typed rows. No eager dense array, per-item padding, duplicated vertex or edge storage, or oversized fixed row record exists in the snapshot.

The 56 MB is the vacant-receiver seed charge. Native receiving births one receiver per record row and charges its whole vacant byte image (`NativeSeedTemplate::fields`, 1 KB parts) as `copied_bytes` and fuel. The receiver of a row is `6.5 KB + 15.7 KB per non-scalar field`, because every non-scalar field owns an inline `NativeReceiveSlot`, and the slot is 15.7 KB even for a `String`.

## Measured sizes (bytes, `size_of`, debug build, 2026-11-23)

| row | typed row | receiver | rows in forest | seeded |
| --- | --- | --- | --- | --- |
| BrepCoedge | 224 | 146,208 | 252 | 36.8 M |
| BrepEdge | 160 | 69,760 | 126 | 8.8 M |
| BrepLoopEdge | 24 | 22,224 | 252 | 5.6 M |
| BrepFace | 200 | 84,416 | 57 | 4.8 M |
| BrepLoop | 48 | 52,528 | 57 | 3.0 M |
| BrepVertex | 56 | 38,048 | 71 | 2.7 M |
| BrepShellFace | 24 | about 22,000 | 57 | 1.3 M |

Model (rows times receiver bytes): 63,008,608 B, within 12 percent of the 56 MB the host measured (`host-spend.md` Goal 2). Receiver of the whole snapshot: 249,040 B.

Anatomy of one slot (`NativeReceiveSlot<dyn NativeValueSource, String>` = 15,712 B; with `u8` 15,696 B, so the child type does not matter):

- `NativeReceivingChild` 13,280 B, of which `NativeReceivingEffect<OriginalReceivingPrimary>` is 13,120 B.
- `OriginalReceivingPrimary` is 2,544 B (nine `Option<ValueError>` of 64 B, plus inline `NativeAllocationFailure`, `NativeDecodeAllocationOutcome`, `OriginalRetirementFailure`, completion identities) and is embedded four times per effect: producer reply, `producer_reply`, `child` outcome and `receipt_primary`. That is 10.2 KB of the 13.1 KB.
- `RetainedCloneBinding` 976 B, `RetainedCloneClose` 560 B; record state base: `NativeRecordMatcher` 2,736 B, `RetainedOwnedProjectionAdmission` 992 B.
- The child receiver itself lives behind `NativeOwnedStorage` (pointer), so a `String` receiver is only 1,232 B; the slot is 13 times heavier than what it carries.

A coedge has five `String` ids (id, edge, loop_id, next, prev), an `Option<BrepCurve2>` and a `(f64,f64)` range with defaults: six to eight slots, hence 146 KB for a 224 B row.

## The "2.6x transient copy"

It is not a copy-then-publish. The record receive path already seeds in place; the ratio is the accounting of one by-value pipeline per row: the vacant receiver seed (1x, equals the receiver capacity), plus `size_of::<F>() + size_of::<Option<F>>() + usize` per field placement, plus `take_initialized` (`size_of::<T>() + size_of::<Option<T>>()`), plus `publish` charging `copied_items + copied_bytes` as fuel. Those moves are 224 B-scale next to 146 KB seeds, so removing them saves under 3 percent. Nothing in `OriginalRecordMemberSnapshotOpen::publish` or the receive phases double-copies the snapshot; the cost is the receiver seed. It is not avoidable without changing the quote protocol or the receiver size.

## Why no compaction was applied

Per-row cost is `6.5 KB + 15.7 KB x non-scalar fields`, so row-level changes are bounded:

- Drop the four redundant coedge strings (id, loop_id, next, prev are derivable from loop order): coedge 146 KB to 83 KB, saves about 15.8 M (25 percent). About 35 files (sqlite, text grammar, binary, proto, graphql, TS, fixtures); not enough alone.
- Merge coedge into `BrepLoopEdge` (pcurve and prange there) and delete `coedges`: total about 36 M. Still above the target.
- Numeric ids (`u32` leaf fields) on every row additionally: about 23 M total, a schema redesign over 190 referencing files for 2.7x.

None reaches the target of 29,360,128 B per load (half of the 58,720,256 fuel, for two locale loads). Reaching it needs the slot to shrink, a framework change with an invariant conflict, hence an owner decision:

1. Shrink `NativeReceivingEffect`: hold one `OriginalReceivingPrimary` per effect instead of four. The slot drops from 15.7 KB to about 5.5 KB and every receiver shrinks about 2.5x with no schema change (forest about 25 M, fits). Conflict: the effect frame is 150 KB of source, address-captured via `capture_plan`, and its contract says no error frame is allocated after refusal.
2. Box the cold tail of `OriginalReceivingPrimary` (`allocation_failure`, `native_allocation`, `retirement_failure`, completion fields, about 2 KB). About 140 use sites; allocates on the refusal path.
3. Share one slot per field type inside a record (a coedge uses one `String` slot for five fields): about 35 percent; needs derive changes in `✨️derive/🛬️receive`.
4. Quote vacant `None` fields at their discriminant instead of `size_of`: removes the over-charge at its source (most seeded bytes are `None` images) but changes the exact seed quote that `seeded_bytes` and part completion rely on.

Each framework rebuild took 25 to 60 minutes here (shared build dir, several agents editing live), so none was attempted blind.

## Laws

In `📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/`:

- `🪙️budget.json`: language-agnostic budget (pack bytes, row counts, `ceilingSeedBytes` 63,008,608, `targetSeedBytes` 29,360,128, `hostLifetimeFuel` 58,720,256, `loadsPerTest` 2).
- `🦀️.rs`: `forest_stock_brep_seed_bytes_never_exceed_their_pinned_ceiling`, `forest_stock_brep_fits_the_host_lifetime_fuel_for_every_pane_load` and `receive_slot_never_exceeds_its_pinned_size`. Before option 1 the target law was red (63,008,608 against 29,360,128); after it all three are green (see the end).
- `🟦️.ts` twin: asserts the budget arithmetic (`target x loads <= fuel`, `target <= ceiling`); `bun test` 2 pass.

Run: `T/brep-cargo.sh <label> test -p semio-s-artifact-process-process3d-examples forest_stock_brep -- --test-threads=1`. It equals `replay-cargo.sh` with its own artifact dir `native/target-brep`, because the shared `native/target` artifact lock is held for the whole run of the other agent's smoke test; the log goes to `🗑️generated/replay-<label>.txt`. Result: 1 passed, 1 failed (the red target law).

## Files changed

- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs` (two laws and the seed model, no `[DEBUG]` left)
- same directory `🟦️.ts` (TS twin)
- same directory `🪙️budget.json` (new)
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/📦️packages/🦀️rust/Cargo.toml` (dev-dependencies `semio-framework-value-native-owned`, `semio-framework-dsl-record`, `serde_json`)
- `T/brep-cargo.sh` (new helper) and this file.

Not touched: the shared editor smoke test target, any framework crate, the fuel convention, `HostLifetimeStock`.

## Cleanup

Deleted my logs `🗑️generated/replay-m1..m13.txt`, `forest-brep.pack` and `native/target-brep`. Idle test binaries over 20 MB: none exist in `native/build-replay` (the examples test binary is 12 MB). `native/build-domain/.../semio_s_artifact_stdio_txt-7ab71e15a2c787d8` is over 20 MB but belongs to another agent's build dir and was left alone.

## Option 1 result (owner decision implemented)

Change: in `🧳️frame/🦀️.rs` `NativeReceivingEffect` holds one `EffectRelay<E>` (`Empty | Producer(OriginalEffectAction) | Child(RetainedWorkOutcome)`) instead of `producer`, `producer_reply` and `child`. Everything stays inline, no allocation after a refusal; the relay sits at one stable address inside the frame and `capture_plan` now captures it (identity and extent only, no consumer writes through it). `OriginalEffectAction::restore_reply` (producer engine) returns a transferred reply to its action when `finish` refuses, so no second copy is needed on that path.

Coexistence proof (read from the frame):
- producer and child never coexist: `drive_original_producer_dyn` refuses to run while a child exists, `drive.finish` clears the producer before `retain_child`, and the settled-state checks (`producer.is_none()` next to a retained child) already required it. `producer_reply` was never persistent: it was filled and drained inside one call, so it is now a local, and a failing `finish` puts it back into the action.
- child and `receipt_primary` DO coexist: receipt staging reads the child result while writing `receipt_primary`, and settlement returns both in one `NativeReceivingEffectFailure{child, receiving}`. Hence the minimal second copy stays: `receipt_primary` remains its own inline `OriginalReceivingPrimary`. This is why the slot cannot reach the 5.5 KB estimate (that assumed all four copies fold into one).

Measured (debug, `size_of`):

| item | before | after |
| --- | --- | --- |
| `NativeReceiveSlot` (String child) | 15,712 | 10,608 |
| `NativeReceiveSlot` (u8 child) | 15,696 | 10,592 |
| forest stock brep seed model | 63,008,608 | 44,230,992 |
| effect seed parts (`vacant_field_count`) | 36 | 34 |

Laws now (shared `🪙️budget.json`, TS twin `bun test` 3 pass): `receive_slot_never_exceeds_its_pinned_size` (ceiling 10,608; TS asserts at least 32 percent below the 15,712 baseline), `forest_stock_brep_seed_bytes_never_exceed_their_pinned_ceiling` (ceiling lowered to 44,230,992), `forest_stock_brep_fits_the_host_lifetime_fuel_for_every_pane_load` (green, but see the correction). Correction to the earlier budget: `arm_host_lifetime` runs once per `assert_pane_smoke_by_example` call, so a lifetime holds one pane load, not two; `loadsPerTest` 2 became `loadsPerLifetime` 1 and the target became the whole fuel, 58,720,256. The target law therefore only proves the stock brep alone fits; it does not prove the pane load fits.

Pinned fixtures re-pinned because the effect has 34 instead of 36 seed parts (arithmetic, not behavior): `resident-owner.json` seedParts 80 to 76, `source-bank.json` and its TS twin vacantFrameFields 36 to 34, `struct-seed.json` fixedParts 19 to 17 and Row parts 21 to 19, schema work `📐️.json` turns for the flattened matcher 157 to 153.

Verification (shared artifact dir `native/target-brep`, helper `brep-cargo.sh`; `brep-cargo2.sh` is the same with a second artifact dir):
- value-native: `semio-framework-value-native-receiving-frame` all green (unit, resident owner and vacant custody laws), `semio-framework-value-native-owned` all green after the re-pins (native_work 16, record_matcher_driver 2, others ok), `semio-framework-value-native-receiving-original` lib 9 passed.
- record receiving (`semio-os-store-member-open-store`): `a_receiver_between_one_and_two_half_shares_reaches_ready_without_draining_its_pinned_stock`, `a_large_original_drawing_settles_in_turns_proportional_to_its_size_over_the_grant` and `foreign_declared_snapshot_envelope_retains_original_request_and_error_receipt` pass; `genuine_original_drawing_member_receives_closes_and_funds_exact_handoff` (100,000-cut loop) was still running after more than 15 CPU minutes when this was written, no result yet.
- job (`semio-framework-job --features testkit`): lib 13 passed, 2 failed (`genuine_original_page_publication_keeps_the_exact_output_before_receipt_refusal`, `genuine_worker_preparation_uses_the_admitting_ledger_and_one_current_fuel`). Both fail identically at HEAD (I swapped the two changed files for `git show HEAD:` copies, ran the lib tests, restored mine): left `(65626,65626)` against `(0,16384)`, a heap-observer versus receipt mismatch independent of this change. The integration tests `original-caller-capture` and `original-child-source` show the same heap-versus-receipt signature but were not baselined; their run is queued in `🗑️generated/replay-w3.txt`.
- editor `process3d_bearbeiten_pane_smoke bearbeiten_pane`: `bearbeiten_pane_renders_the_loaded_forest_in_en_and_de` passes, `bearbeiten_pane_loads_applies_undoes_and_closes` still fails with "host lifetime fuel exhausted across fixture turns" (223 s).

What is left for the editor test: the stock brep is now about 44 M of the 58.7 M fuel, and the rest of the load (five tool-solid breps, the flow, the process snapshot, history) needs more than the remaining 14.5 M. All of those use the same slots, so they shrank too, which means the load needed more than about 84 M before. Further cuts, each measured against the same laws: fold the frame's `outcome` and the five refusal options into `receipt_primary` (they are moved into it at settlement; about 0.8 KB per slot, 8 percent); one slot per field type inside a derived record (about 35 percent); drop the four derivable coedge strings (id, loop_id, next, prev; about 10 M); shrink `ValueError` (64 B, about 25 copies per effect). Together these plausibly reach the 2.3x the load needs; none changes the refusal contract.

Files changed by option 1 (framework):
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🎟️control/📨️reply/🏭️producer-engine/🦀️.rs` (`restore_reply`)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/resident-owner.json`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🧪️tests/source-bank.json` and `source-bank.test.ts`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🧪️tests/🧫️fixtures/🌾️struct-seed.json`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🎟️work/📐️.json`
- examples laws: `🪙️budget.json`, `🦀️.rs`, `🟦️.ts` (slot law, loadsPerLifetime, new ceilings)
- `T/brep-cargo2.sh` (new helper)

Cleanup: deleted `replay-c*`, `replay-v*`, `replay-w1`, `replay-w4`, `replay-b1` logs where finished, `native/target-brep2`, and idle test binaries over 20 MB in `native/build-replay` (editor `process3d_bearbeiten_pane_smoke` 1.35 GB, `native_work`, `native_fields`, `native_sequence`, `native_shared`, `refusal`). The running `semio_os_store_member_open_store` test binary (84 MB) was kept.

## Round 2: fold, shared slots (coordinator order, steps 1 and 2 done; 3 and 4 judged not worth it)

Implemented, one rebuild:
- Step 1, fold: the frame's `outcome`, `grant_refusal`, `effect_refusal`, `work_reporting`, `work_overrun` and `receipt_refusal` now live directly in `receipt_primary` (`native_allocation`, `quote_refusal`, `primary`, `work_refusal`, `work_overrun`, `receipt_refusal`), the very fields settlement used to move them into. Effect seed parts 34 to 28; effect 7,376 B. One trap found by the history acceptance run: `resident_success` required `receipt_primary.is_empty()`, which a settled allocation outcome now breaks, so a successful archive load refused with "native receiving original type, field or phase is invalid" (`history_edits_end_to_end`, the pane smoke loads). Fixed with `OriginalReceivingPrimary::is_empty_apart_from_native_allocation`.
- Step 2, shared slots: in `✨️derive/🛬️receive/🦀️.rs` non-scalar, non-flattened fields of the same type share one `NativeReceiveSlot` of the record (a coedge's five `String` ids use one slot). Fields are received one at a time and the existing stages 4 and 7 already close the slot after each take, so after stage 5 the shared slot is terminal-empty and stage 6 resets it to `Default` (as the sequence receiver does between rows). An earlier draft of this change that also demanded an empty slot at stage 1 was wrong (stage 1 runs on a freshly born slot) and was removed.
- Step 3, shrink `ValueError` (64 B: kind, `Cow<'static, str>` 24 B, `RetainedCloneProgress` 32 B): not done. The progress is four `usize` counters carried by value in about 25 copies per effect; narrowing them changes every receipt type, and saturating them would break fixtures that use `usize::MAX >> 1` stocks. The gain was estimated at 5 to 6 percent for a repo-wide type change.
- Step 4, drop the four derivable coedge strings: not done, and it no longer pays. With shared slots the five coedge strings cost one slot, so removing four of them saves a few hundred bytes of pending fields per row, not a 10 KB slot. They are derivable (id, loop_id, next, prev follow from loop order and `emit_loop`), but the change touches about 35 files in every format for under 1 percent.

Measured:

| item | before round 2 | after |
| --- | --- | --- |
| `NativeReceiveSlot` (String child) | 10,608 | 9,968 |
| forest stock brep seed model | 44,230,992 | 28,748,576 |
| `NativeReceivingEffect<OriginalReceivingPrimary>` | about 7,900 | 7,376 (relay 2,744, `receipt_primary` 2,544) |
| effect seed parts | 34 | 28 |

Laws re-pinned: `receive_slot_never_exceeds_its_pinned_size` 9,968; `forest_stock_brep_seed_bytes_never_exceed_their_pinned_ceiling` 28,748,576; TS twin asserts the slot is at least 32 percent below 15,712 (now 36.6). `semio_framework_value_native_*` laws and fixtures re-pinned in round 1 are unchanged by round 2.

Verification (private build dir hardlinked from `native/build-replay`, because two builders on one build dir produced "can't find crate" races):
- `process3d_bearbeiten_pane_smoke`, whole target: 17 passed, 1 failed. Passing: `history_edits_end_to_end`, `conflict_history_edits_end_to_end`, `bearbeiten_pane_renders_the_loaded_forest_in_en_and_de`, all composition tests. Failing: `bearbeiten_pane_loads_applies_undoes_and_closes`, "host lifetime fuel exhausted across fixture turns".
- `semio-framework-job --features testkit`: lib 15 passed and every integration target passed (the two earlier pre-existing failures did not reproduce, another agent's change).
- examples laws: 9 passed.

Why the pane test still fails (instrumented run, instrumentation removed): with the fuel refusal disabled the load completes at about 61.46 M fuel spent against 58.72 M, then the next step (`AddStep`, then undo, redo, undo, close) stalls ("abandoned its pending publication") because the lifetime has no fuel left. So the load alone is about 4.7 percent over, and the remaining steps need more; I could not measure how much more, but with five tool-solid breps, the flow and the snapshot all using the same row receivers, a further 15 to 25 percent is the realistic gap.

Levers left, by measured yield (none attempted yet):
1. Share the effect frame per record instead of per slot: after sharing, a record still pays 9,968 B per distinct field type and about 8 KB per optional field's default slot, of which 7.4 KB is the same `NativeReceivingChild` frame; only one child is ever active. One frame per record would cut a coedge from about 51 KB to about 24 KB (more than 40 percent of all row cost). It needs the slot and default-slot `action` to borrow a frame owned by the record (58 uses in two files, an address that must stay stable for `capture_plan`).
2. Make `prange: (f64, f64)` two required scalars (`prange_start`, `prange_end`): removes a slot plus a default slot, about 18 KB per coedge (252 coedges, about 4.5 M, 7 percent). A schema change in every format and fixture of the brep snapshot.
3. Columnar rows: a `Vec<String>` or `Vec<f64>` element costs only its own 1.2 to 1.5 KB receiver, a struct row costs 15 KB or more, so storing loop edges and coedges as parallel columns would remove most of the remaining row cost; a schema redesign.
4. `RetainedCloneBinding` (976 B) appears three times per record (record state, matcher, projection).

Files changed in round 2:
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/📬️outcome/🛬️receiving/🦀️.rs` (`is_empty_apart_from_native_allocation`)
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🦀️.rs` (shared slots)
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧳️frame/resident-owner.json`, `🛬️decode/🧳️owned/🔗️sealed-shared-receiver/🧪️tests/source-bank.json` and `.test.ts`, `🛬️decode/🧳️owned/🧳️original/🧪️tests/🧫️fixtures/🌾️struct-seed.json`, `🧩️record/🔎️schema/🎟️work/📐️.json` (seed part arithmetic after round 2: seedParts 64, vacantFrameFields 28, schema turns 141, struct-seed unchanged at 17 and 19; `value-native-receiving-frame`, `-owned` and `-receiving-original` all green after these values)
- examples `🪙️budget.json`
- `T/brep-cargo2.sh`, `T/retry2.sh` (helpers, private build dir `native/build-mine`, removed after the run)

Cleanup: deleted `native/build-mine`, `native/target-brep*`, my logs, `quiet-run.sh` and the temporary fixture and debug-test edits (no `[DEBUG]` left).

Not rerun after round 2: the `semio-os-store-member-open-store` record tests (the 100,000-cut drawing test takes over 15 CPU minutes) and the stdio and process3d crates other than the examples laws and the pane smoke target.

## Round 3: one frame per record, and what the real cost is

Done and verified (private hardlinked build dir `native/build-mine`, removed afterwards):
- `NativeReceiveSlot` and `NativeDefaultSlot` take a frame holder `F: SlotFrame` (default `OwnedFrame`, behavior of every other user unchanged). A derived record keeps ONE `NativeReceivingChild` frame and its slots use `SharedFrame`, which finds the frame at a fixed byte distance inside the record receiver (anchored at seed time and after a slot reset), so a moved receiver stays valid and `capture_plan` stays stable. Records without slots or defaults carry no frame. The record machine (`record-machine`) is the generic home of the record logic; the derive only emits the state.
- Receiver sizes: `BrepCoedge` 146,208 to 24,576, `BrepVertex` 38,048 to 19,056, `BrepEdge` 69,760 to 19,344, `BrepFace` 84,416 to 23,184, `BrepLoop` 52,528 to 20,272, `BrepLoopEdge` 22,224 to 16,496. New law `brep_row_receivers_never_exceed_their_pinned_size` plus the `recordReceiver` section of the shared budget and its TS twin (13 expectations pass); forest seed ceiling re-pinned to 17,557,728.
- Verification: examples laws 10 passed; whole `process3d_bearbeiten_pane_smoke` target 17 passed, 1 failed (`bearbeiten_pane_loads_applies_undoes_and_closes`, lifetime fuel); value-native frame, owned and receiving-original crates all green; `semio-framework-job --features testkit` lib green and integration targets green except `original_job_paged_source_preserves_header_authority_bytes_and_zero_heap_borrows`, `original_in_place_payload_builder_preserves_fixed_grants_source_pages_and_all_cancellation_frontiers` and `original_step_context_forwards_independent_work_funding` (heap-observer versus receipt mismatches of about 0.9 MB, same signature as the earlier job failures that another agent now owns); `semio-os-store-member-open-store` record tests 4 passed including the 100,000-cut drawing test (6,284 s).

Correction of my cost model (measured, this is the important result): the fuel is NOT driven by receiver size. Instrumented runs show:
- Each `SemioBrepSnapshot` open costs about 14.3 M bytes and 29 K items regardless of receiver size: 15.8 M before this round, 14.3 M after shrinking every row receiver by up to 83 percent. Four opens (about 52.7 M) plus the rest give about 60 M of the 58.72 M lifetime. The first open holds only 84 records, so the cost is about 170 KB and 350 items per record, not a function of `size_of`.
- Vacant-seed charges are small (a few hundred KB for the whole load, almost all in 314 `StringCursor`s); record-machine actions total about 250 KB per open. The bulk is the many small receiving steps (about 29 K items per open at about 490 B each), whose quotes are dominated by the 1 KB binding and projection types (`RetainedCloneBinding` 976 B, `RetainedOwnedProjection` 1,024 B, `RetainedOwnedProjectionAdmission` 992 B) that appear in almost every step. Turning pairs and all-scalar structs into direct reads (tried, then reverted) removed receivers but changed the cost by only 3 percent, which confirms it.
- With the lifetime refusal disabled the load completes (about 59.99 M) and the next failure is "original fixture settlement stopped after 4161 turns (fuel 2,000,000)" in the history undo, a separate per-call fuel limit.

Next lever (not done): shrink the binding and projection types, or the number of steps per field. `RetainedCloneBinding` is a lease owner plus a `ControlledRetirement` close gate plus a projection, about 976 B inline and quoted in nearly every step; boxing or sharing it, and not re-quoting a bound projection every action, is what can move the 350 items and 490 B per item. That is a change in `retained-clone` and the controlled retirement, not in receivers.

Cleanup: private build dirs, logs and all temporary instrumentation removed (no `[DEBUG]` remains); the reverted experiments (pair and all-scalar-struct leaf reads) are not in the tree.

## Round 4: where the real forest load spends its fuel (measured, not yet fixed)

Findings, all with temporary `[DEBUG]` instrumentation that is removed again (grep-checked):
- The earlier "bearbeiten_pane passes past the fuel wall" result was a FALSE POSITIVE. The fixture command arrived with the default example id (`timber-beam-joinery`) because `command_from_action` read only `exampleId`/`id`, while the typed `SetActiveExample` serialises `example_id`; the cheap timber document loaded. Fixed: `✏️editor/🪟️editor-impl/🦀️.rs` now also reads `example_id`. With the real forest the load still exhausts the host lifetime fuel.
- Derived-record field slots no longer bind their own alias: `SlotFrame::BORROWED` (true for `SharedFrame`) makes `NativeReceiveSlot` skip the Bind step and check the captured source coordinates instead (the record's own slot alias keeps the source alive). Saves the 2 KB alias quote per field; measured saving on a small row is only about 3 percent.
- Per-step histogram of the forest load (completed receiving steps, about 144 K steps and 63 M bytes at the failure): local advances of the row slots dominate; of the record-machine actions, action 15 (`retire_field`, one close rung per call) is about 60 percent (BrepEdge 25 K, BrepCurveLineNativePayload 11 K, BrepVertex 7 K), about 210 rungs per edge. Slot close rungs: `retirement` 31.9 K and `backing` 26.8 K of 60 K, mostly String (about 17 rungs per String slot) and SemioPoint3 slots.
- Every close rung charges the dispatch header `metadata()` = 680 B (`Selection` 304 + resident answer 336 + work 40), compounded once per nesting level. The 336 B answer is `OriginalRetirementFailure`, kept inline on purpose (no allocation on failure). Charging only the Ok-sized header (tried) is refused by the exact law `actual_closed_receiver_header_moves_its_existing_allocation_and_runs_its_destructor_once` (quote must equal charge), so the header must shrink in the quote itself.
- Seed parts: a `Fields` template seeds one part per receiver field with a 56 B state header each (String receiver about 52 parts, BrepEdge about 25); grouping parts up to 16 KB would cut the item count about 10x but changes the part-size law.

Law added: `native_receive_row_steps_per_record_never_exceed_the_pinned_budget` (owned tests, budget in `🧪️tests/🧫️fixtures/📐️.json`, TS twin `🧪️tests/steps.test.ts`, bun test passes). The law measures with a one-work-unit cut per turn, so the byte copy of a String counts one step per byte; the pinned values are 812 steps and 582,591 copied bytes (before 834 and 594,047). It is a regression guard, not the O(fields) target.

Job tests: `semio-framework-job --features testkit` has 3 failures (`genuine_owned_park_keeps_alias_on_post_refusal_and_never_renews_its_seven_fuel_donor`, `original_child_source_issues_business_identity_once_and_closes_every_paid_prefix_under_same_parent`, `original_close_context_denied_work_and_deadline_preserve_owned_source`: alias duplicate metadata not funded, identity ledger zeros, source close incomplete). They fail identically with `BORROWED` off. Their sources do not use derived records. I could not run a pre-frame baseline (HEAD already contains the frame work and lacks other agents' crates).

Next levers, by measured yield: (1) shrink `Selection` and the resident answer so one rung quotes under 300 B; (2) collapse trivial rungs (`authority` identity retire, `begin_close` of an empty slot) into the neighbouring rung, keeping one receipt per release; (3) group seed parts to 16 KB; (4) a close of a receiver with no custody as one box release.

## Round 5: retirement redesign (partial, target test still red)

Measured cause: `fuel` equals copied bytes almost one to one (54.6M work against 55.8M copied bytes at exhaustion). Per `String` field the old pipeline spent about 39 KB and 39 steps: slot admission 5 KB, receiver and cursor seeds 7 KB, two alias bindings 4 KB, a 38-rung retire ladder (27 rungs only for the slot authority, 10 per binding) at 680 B per rung.

Changes (all under `🧰️framework/🔨️modules/🌱️value`):
- `RetainedCloneBinding` closes a counted alias in one step (`close_one`, `close_demand`; new `retire_shared_alias` on `SealedShared` and `SharedOriginalStorage`). Falls back to the old ladder for a last alias. `alias_copy_bytes` now quotes the real writes (lease slot plus flag), not two whole 976 B bindings.
- `NativeReceiveSlot` releases an idle authority in one step (`authority_release_demand`), 27 rungs to 1.
- `NativeReceiverBackingClose`: take-box and drop-box fused into one rung; the success quote carries `Option<RetainedCloneStep>` instead of the 336 B failure answer; the failure path pays `refusal_extra` only when refused.
- `NativeStringReceiver` rewritten: no cursor, no binding, one funded step for short text, one per page for long text.
- `String` record fields are leaf reads (derive `text_leaf`, machine `FieldShape.text`, `plan_text_stage`, `field_text`, action 27): no slot, no projection, no receiver.

Measured (owned crate, ordinal-keyed rows, steps and copied bytes to receive and retire): coedge 350 steps / 122,044 B to 35 / 3,159 B; loop edge 82 / 26,163 to 19 / 2,386. Law `forest_text_rows_cost_steps_proportional_to_their_fields` (budget `forestRows` in `🧪️tests/🧫️fixtures/📐️.json`, TS twin `🧪️tests/steps-forest.test.ts`, bun 1 pass).

Real forest load, `bearbeiten_pane_loads_applies_undoes_and_closes`: STILL RED. Before (coordinator): about 339M fuel, 740K steps. Now: 223 s run became about 8 s; at the 58.72M wall the load had done 137K items; a probe with the wall lifted reached 136M work and 325K items when the 30 s deadline hit, load not finished. So the load is still above 136M, I could not measure its total. Remaining cost by counter: child close ladders of non-text receivers (about 26M of 136M), slot retire rungs (14M), and per nested field projection admission (2072 plus 3040 B), nested record binding and seeds (Pt3, enum curves, Vec rows).

Not done: nested record fields as leaf or borrowed (admission 5 KB plus teardown 1 KB per nested field is the largest remaining item), one-box release of a whole record receiver, shrinking the dispatcher `Selection` (304 B), steps-per-record law on the real forest rows (only the synthetic law exists), one-box-release law. Not rerun after the changes: `semio-framework-job --features testkit`, record-machine, receiving-frame, receiver-protocol, receiving-original, `semio-os-store-member-open-store`, the full smoke target, process3d host tests. Owned crate: all targets passed.

Files changed: `🛬️decode/🧳️owned/🧬️protocol/♻️backing/🦀️.rs`; `🧬️protocol/📦️packages/🦀️rust/🦀️.rs` (debug removed, net unchanged); `🛬️decode/🧳️owned/🌳️value/🦀️.rs`; `🏗️record-machine/🦀️.rs`; `✨️derive/🛬️receive/🦀️.rs`; `♻️retirement/📦️allocation-return/🔗️shared/📦️original/🦀️.rs`; `♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs`; `🧬️retained-clone/🔗️source/📦️packages/🦀️rust/🦀️.rs`; `🧪️testing/📥️admission/receiver-driver.rs` (`close_traced`); `🧳️owned/🧪️tests/🦀️.rs`, `rows.rs`, `steps-forest.test.ts`, `🧫️fixtures/📐️.json`; `artifact-app-laws/🎟️fixture-dispatch.../🦀️.rs` (debug removed, `FUEL` restored). Helpers `retire-cargo.sh`, `retire-fw.sh`. All `[DEBUG]` code of mine removed; large build dir `🗑️generated/native/build-retire` still present.

### Round 5 continued (coordinator follow-up): more cuts, still red

Further changes (framework `🔨️modules/🌱️value`):
- Nested non-leaf record fields read a borrowed child (`project` each step) instead of admitting and retaining a projection per field (record machine `field_plan`/`field_work`/`field_nested`, `RecordSlot::slot_work/slot_nested` take the child ref; enum machine passes its own source). The 5 KB admission and 1 KB teardown per nested field are gone.
- Vacant seed parts quote at most `SEED_WRITE_BYTES` (32) and a record receiver seeds as ONE funded part (`template`: `seed_all_size/seed_all_write`). Law `struct-seed.json` re-pinned to one part, TS twin updated.
- Absent `#[value(default)]` fields with an allocation-free default (`NativeDefault::PURE`, `pure_default`) are written in one inline step (`FieldShape.pure_default`); `NativeDefaultSlot` starts without authority so an unused default slot is terminal-empty and needs no close ladder. Synthetic coedge with two absent defaults: 97,030 B to 3,079 B.
- Dispatcher header slimmed: only `Selected` stays resident; every demand is requoted. Rung prefix 384 to about 296 B.
- One-box release of a whole finished record receiver: `NativeReceiver::flat_close_demand/flat_close` (record machine implements it: idle fields, alias return, partial storage release, closing flag), dispatcher code 7 = flat close plus box release in one rung. Law `closed-header.rs` updated to the fused model (work 3), TS twin `closed-header.test.ts`.
- `alias_copy_bytes` quotes only the lease slot and flag.

Synthetic rows (steps / copied bytes, receive and retire), before to after: coedge 350 / 122,044 to 45 / about 3.3 KB; vertex with nested point 89 / 22,300 to 65 / 3,797; loop of four edges 288 / 53,076 to 177 / 28,040; face with three inner strings 106 / 37,228 to 95 / 25,356; coedge with absent defaults 129 / 97,030 to 49 / 3,079. Law `forest_text_rows_cost_steps_proportional_to_their_fields` pins all six (budget `forestRows`, twin `steps-forest.test.ts`).

Real load (wall lifted for a probe, test then passes): 140M work / 569K items, 128M / 499K, 116M / 485K after the last cut; before this work 339M / 740K. With the unchanged wall the test is still red (about 2x over). Full smoke target: 17 passed, 1 failed (the target test), as at the start. Owned crate, receiver-protocol, record-machine, receiving-frame, receiving-original, job (testkit): green after re-pinning the closed-header and struct-seed laws. `semio-os-store-member-open-store` ran without the 100,000-cut drawing test (about 100 min); see below.

Measurement caveats: my phase counters sum copied bytes and items per host call, which batches many steps, and `original_work_units_spent` inside the open operation did not include the nested effect work, so I could not attribute the remaining 116M work to a phase. What is known: Receive phase 443K items, CloseReceiving 246K items (the parsed record tree and receivers retired node by node), Parse about 1M work. Next cuts, untested: bulk release of the parsed record tree (needs an exact subtree release quote per owner type), enum payload borrowed child (enum machine still admits a projection), and per-quote shrinkage of the sequence receiver (16 KB vacant seed per list).
