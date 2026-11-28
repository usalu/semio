# store-tests-05: "original work requires its original caller loan"

Package `semio-os-store-tests-05` (`C:\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\🧪️tests\🌅️bin-05\📦️packages\🦀️rust`), run natively with `store-tests-05-run.sh` (private `C:\t\sl`, `-j 3`, detached).

## Finding
The task named three failing tests. The first run (run1) had **40 of 83 failing**, all with the same `UnsupportedOwner` error from `OriginalWorkAction::prepare` (`🌱️value/🎟️control/🧬️contract/🎟️work-action/⚙️operation/🦀️.rs:140`).

## Root cause: stale tests, not a defect
Authoring through `store.dispatch(.., identity)` now draws its Work from the original caller's loan, handed over by `EntityIdentityAuthority::with_work_lender`. The production route and the shared helper `test_support::with_original_identity` (`🏪️store/🧰️test-support/🦀️.rs`) install that lender inside a funded original turn of `OriginalJobFixtureCaller`. bin-03 and bin-04 already use `dispatch_test_command` or `with_original_identity`.

The bin-05 laws still built a bare `EntityIdentityAuthority::new(CEILING, &mut observer)` with no lender, so the first dispatch had no loan and refused. The guard is correct and stays strict. The tests encoded the pre-loan contract.

## Fix
- Every bin-05 law that dispatches now runs its body as `test_support::with_original_identity(|identity| test_support::block_on_original_turn(async move { .. }))`. Calls pass `&mut *identity`. The hand-built identity, its observer and the trailing `identity.encode` / `pause` lines are removed, because the helper observes the ceiling and settles the cumulative receipt itself. Transformed mechanically by `store-tests-05-loan-rewrite.py`: 14 laws in `📄️document-text-wire-and-command-error-laws`, 10 in `🔀️reconcile-alternative-event-log-and-backbone-port-laws`, 12 in `🕸️composition-graph-child-id-and-group-dispatch-laws`, 3 in `🗜️pack-value-fixture-and-typed-child-member-laws`. Laws that never reach a loan-consuming call (for example `reconcile_alternative_requires_an_existing_checkpoint`) are untouched.
- `🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs`: `author` now calls `semio_os_store_test_support::dispatch_test_command`. The unused `IDENTITY_CEILING` const is removed.
- `pack_wire_value_carries_bytes_exactly_as_the_neutral_corpus_frames_them` (not a loan failure, found when the loan failures cleared): intrinsic `DslValue::Object` keeps entry order on the wire (see the "Schema maps sort key bytes while intrinsic Objects preserve exact ordered occurrences" law in `🌱️value/🧪️tests`). The keyed fixture is now built `archive` then `kind`, the order the language-neutral corpus `🎒️pack-dynamic-bytes-v1` frames. The corpus (Python oracle plus TS pin) is unchanged.

## Result
- run1 (before): 43 passed, 40 failed, all `original caller loan`.
- run2 (after the loan rewrite): 76 passed, 7 failed. **No caller-loan failure remains.**
- The pack corpus fix was applied after run2. A rebuild (run3) was started but killed: peers' concurrent edits invalidated most of the framework and the machine was saturated (about 5 crates per hour). The edit is a pure swap that yields exactly the expected hex, but it is **not yet re-run**.

## Not green: 6 initializer replay laws (peer-owned area, not touched)
Hidden before by the loan error, they now run into genuine initializer behaviour. All six are in the area other agents are editing (`🧵️job/⏱️context/🛬️owned`, store initializer, `🏠️owners/📥️lifted/🏪️store`; the `[DEBUG] receiving outcome` and `[DEBUG] construction` prints in the log are theirs):
- `an_original_caller_with_a_foreign_identity_is_not_stale`, `a_replaced_slot_or_advanced_generation_is_stale_and_cancelled_while_the_current_identity_completes`, `supersession_replacement_and_withdrawal_are_folded_into_the_replay`, `cancellation_at_every_turn_boundary_closes_with_exact_retirement`: `a replay original turn never errs: InvariantViolated "post custody changed its original Source"` (`🔁️replay/🎮️operation/🧬️native/👁️post/🦀️.rs:51`, `validate_context` sees a different `OriginalSourceTurnIdentity` than at capture).
- `cancellation_under_a_single_item_wallet_makes_progress_and_closes`, `replay_equals_direct_application_under_every_grant`: stuck at `ValidateEdit { index: 0 }` for 100000 turns under the one-item wallet. This matches the escrow/initializer hang the other agent is documenting in `store-init-escrow-hang.md` (not yet written when I finished).

These need the peer fixes to land, then one re-run of `store-tests-05-run.sh <tag>`.

## Files
- Edited: the four `🔬️unit/*/🦀️.rs` bin-05 laws above and `🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs`.
- Scripts here: `store-tests-05-run.sh`, `store-tests-05-loan-rewrite.py`.
- `C:\t\sl*` removed.
