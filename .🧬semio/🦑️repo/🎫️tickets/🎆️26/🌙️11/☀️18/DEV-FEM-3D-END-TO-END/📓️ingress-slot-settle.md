# Command-ingress slot never re-lends (root cause, fix, regression test)

## Root cause (proven natively)
The Source return table has 16 cells (`ORIGINAL_RETURN_ESCROW_CELLS`). Every lend of `OriginalActorReturnSlot` claims two rows (home + donor); settling them leaves tombstones that only free up through acknowledge + `reclaim_one`. `claim_original_sized_return` reclaims ONE row per call on a full table and returns `Ok(false)` (`claim_reason == "reclaimed_one_row"`). So per turn: settle OK, lend claims home, donor claim hits the full table, reclaims one, returns false. Next turn `settle_step` sees `written == false` with a preissued home capture and CANCELS it (`cancel_preissued_capture` + `settle_return`), turning the just-claimed row back into a tombstone. Each turn pays ~12 items / a few KB (cancel + settle + reclaim) and nothing completes: `ready false` forever, status stays CommandPending. The first lends succeed while the table has free cells, which is the single `ready true`. Grant, fuel and yield are irrelevant (not the cause).

Reproduced with a unit test mirroring the production lifecycle (new donor per turn, `refreshed_turn` origin refresh, `acknowledge_return_rows_locally`, new Source epoch per turn): lend, return_owner, repeat. Unfixed code fails at round 16 with ample grant (64 items / 64 KiB per turn) and reason `reclaimed_one_row`.

## Fix (slot lifecycle layer)
`OriginalActorReturnSlot::lend_step` now first reserves every row its claim ladder needs in one go (`OriginalCallerReturnPlace::reserve_rows` -> `reserve_original_return_rows`), reclaiming acknowledged settled rows across calls (rows freed stay free, nothing is cancelled in between). The ladder then completes within one turn, so no half-prepared place is left for the next `settle_step` to undo.

Files:
- `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/📦️return/🦀️.rs` (lend_step reservation; existing test now asserts settle completes; new test `retained_owner_returns_and_lends_again_on_every_later_turn_within_a_small_turn_grant`, 300 rounds)
- `🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🦀️.rs` (`reserve_rows`)
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/⏸️continuation/🎟️source/🦀️.rs` (`reserve_original_return_rows`)
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🪙️return-escrow/🦀️.rs` (`free_rows`)

## Tests (native, `cargo test` with CARGO_BUILD_RUSTC=rustc RUSTC_WRAPPER=)
- semio-framework-actor-retained-context `--lib actor_return`: 2/2 pass (new test fails without the fix).
- Same crate full lib: `tests::actor_reserved_redemption_preserves_aggregate_receipt_and_repeated_fresh_borrow` aborts (`owner.identity.unwrap()` None, tests/🦀️.rs:40); it does not use the slot, so it is not caused by this change.
- semio-framework-job-context `--features testkit --test original-source-return-place`: 8 of 10 fail with `injected publisher receipt refusal: false` style unwraps on `refreshed_turn(q, ..)` fixtures; none reach the new code (reserve is only called from lend_step); baseline not verifiable without git restore, treat as pre-existing/in-flight.
- No reactor-level native harness drives `drive_command_ingress`; `🧪️tests/🔬️command-ingress-terminal` is source/status-only, so no reactor regression test was added.

## Remaining caveat
If the real wasm guest does not acknowledge rows locally (no `acknowledge_return_rows_locally`, delivery not LocalWitness), reclaim waits for host acks; `reserve` then returns false until they arrive (same wait as before, minus the livelock). The `ingress.lend_step` claim_reason probe should now no longer print `reclaimed_one_row` forever.
