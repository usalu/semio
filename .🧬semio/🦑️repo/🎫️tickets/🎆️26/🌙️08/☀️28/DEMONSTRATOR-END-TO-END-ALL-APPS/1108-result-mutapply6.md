# mutapply6 (takes over mutapply5)

## Root cause of the turn-175 stall (FIXED, verified W-mutapply6-14: original_member_wire_batch... passes)
- Not the canonical cursor and not json receiving. At turn 175 the json receiver is in the owned handoff (`taking_owned`, canonical cursor already taken, typed output projection closing): `OriginalMemberBatchReceiving::advance_turn` quoted work `1` for this stage (`else{1}`), but the stage runs `output.close_step`, which needs `close_work_demand(body)` (> 1). `RetainedBinding::close_one` saw `effect.maximum_work_units()<work`, returned default progress -> zero step.
- Fix: new `take_owned_work_demand` on json `OriginalJsonReceiving` (exact per arm: unit pay 1, output/canonical-projection `close_work_demand`, typed source `next_take_work_demand`; the cursor arm keeps the whole turn ceiling as the Canonical stage does) and on the lifted binary receiver (+1 unit pay); trait `ArtifactOriginalOperationReceiving::take_owned_work_demand`; member `advance_turn` quote uses it for a ready/handoff receiver.
- Method that found it: eprintln probes with a warm-up eprintln in the test (first stderr use allocates and trips the heap receipt); the decisive probe was a zero-result print in `OriginalMemberBatchReceiving::advance` with `ready/handoff` flags.

## Further production faults behind the stall (all fixed)
1. `close_active` source branch checked `wires[index]` empty BEFORE every take step; a multi-step source handback returns the bytes in an early step, so the second step refused. The emptiness check now guards only the write of the returned bytes.
2. `close_active` source-admission branch called `return_inputs` while an admission held a projection answer ("original projection frame and answer must return before its inputs"). It now discards a successful projection / takes the failure first; `active_demands` quotes `discard_projection_demand` / `take_projection_failure_demand` accordingly.
3. `NativeReceiverBackingClose` (value/decode/owned/protocol/backing) kept its Work binding after a dispatch completed, so the next dispatch from another turn failed "native receiver backing changed original Work scope". `finish` now clears it.

## Test fixes
- settle axis loop: a denied grant with zero items/depth cannot begin a retirement scope (`with_retirement` refuses); asserted as such instead of unwrapping.
- final `MemberStoreOwnedBatch` close runs inside an `OriginalJobFixtureCaller` effect (`with_original_effect`), retired with `caller.retire(STOCK)`; the legacy `testing::with_effect` had no producer invocation.

## Result
- `original_member_wire_batch_preserves_sources_and_exact_typed_receipts`: PASS (W-mutapply6-14).
- `original_member_close_recovers_every_pending_original_handback_exactly_once`: now runs the whole corpus and fails only the last assertion `pendings>0` ("no original handback ever stayed pending"): no handback in this corpus returns Pending any more (all source/lease closes complete synchronously under `clone_source_testing::drive`). Needs a fixture case that makes the first take_authority turn pend (see mutapply2 note); weakening the assertion is not allowed.

## Probes
- All `[DEBUG] mutapply5/6` probes removed (grep over the framework tree: none).

## Full lib (W-mutapply6-19): 9 passed, 1 failed
- Green: original_member_wire_batch_preserves_sources_and_exact_typed_receipts, copy-query law, prepared_candidate, all other member_owned_batch tests.
- Red: original_member_close_recovers_every_pending_original_handback_exactly_once, only its last assertion pendings>0. A staged-Work driver (first entry 1 unit) produced no Pending either (reverted). With a Vec<u8> lease every handback completes synchronously; the law needs a fixture whose lease owner really suspends (producer returning reply.pending()), not a weaker assertion.

## mutation_apply laws (-p semio-os-store-one-item-batch --test mutation_apply, one at a time with --exact; script 1108-mutapply6-laws.sh, outputs W-mutapply6-lawN.txt / W-mutapply6-laws.out)
- Some laws abort the whole binary on panic ("original work issuer retains an issued action or committed proof"), so always use --exact.
- Green: mutation_apply_admission_scalar_fixture_matches_independent_oracle, mutation_apply_wire_values_round_trip_through_the_value_contract.
- Red laws 1-5 (cold_replay x4, config_apply_reaches_prepared...): "original fixture receiving scope did not open" in the fixture Receiver::run. I switched it to with_native_receiving_quoted(&mut out,(grant,RECEIVER_WORK),..) (RECEIVER_WORK=1<<20); W-mutapply6-23 still fails identically at cold-replay tests line 35 (feeder.run(grant, source.advance_chained)). Next: find why receiving.open / original_receiving_outcome returns false (grant vs source.demands(4096) plus scope control extent, fuel).
- Red law 6 (long bounded_changed_paths): fixture SnapshotRead retirement "native retirement requires its original producer invocation" (legacy close_step outside a producer action; use drive_reply/close_into_step like the prepared loops).
- Laws 7-15 not run individually yet (structural edits inverse rows, paged_invalid_edit_receipt, paged_preparation, diagnostics_gate_atomic_publication [aborts], cancellation_after_decision [aborts], recovers_pending_original_replies, refuses_semantic_errors, inverse_cancellation, rejects_same_data_different_method_table).

## Not done
- PostFixture dev-dep LOCK for the four replication retained test crates.

## Files changed
- pack/json/decode/receiving (take_owned_work_demand; probes removed), pack/json/encode/tree (probes removed)
- value/decode/owned/protocol (probe removed), protocol/backing (Work binding cleared on dispatch completion)
- store/composition/emission/receiving (+tests), lifted operation-wire receiving json + binary + trait (take_owned_work_demand)
- paged-one-item/mutation-apply/tests (Receiver quoted scope)

STATUS: BLOCKED turn-175 stall FIXED-verified W-mutapply6-19 (lib 9/10); remaining: handback-pending law needs suspending lease fixture, mutation_apply laws (Receiver scope does not open, abort-on-panic), PostFixture LOCK not started
