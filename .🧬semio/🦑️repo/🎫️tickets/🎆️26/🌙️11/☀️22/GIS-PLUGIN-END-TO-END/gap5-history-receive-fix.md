# Gap 5: History Operation Receiving Stall

Static analysis only. No cargo or build ran.

## Where the stall comes from

- `receive_to_completion` (`dsl/🫴️receiving/🦀️.rs`) raises "original receiving made no progress" when one cursor step returns zero work, zero progress and no native transition.
- Terrain operations use `original_pack_value_operation_receiving!(GisTerrainMutation, tagged_key = ...)`, so the cursor is `OriginalTaggedPackValueReceiving` (`📦️pack-value/🏷️tagged/🦀️.rs`), not the ordinal `OriginalOperationReceiving`.
- That cursor and its typed binding (`NativeSourceReceiving`, `dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic`) only act when `funded(grant, demand)` / `admits(effect, demand)` holds. An unpayable quote returns `Default` progress with zero work, which is the idle signature above. No refusal is raised.
- The binding's first typed quotes are the whole receiver storage of `GisTerrainMutation`: `reserve_demand` and `seed_demand` of `NativeOwnedStorage<Receiver<TaggedValue, GisTerrainMutation>>` (phases 6 and 7). The enum receiver holds the carriers of all three variants, so even the smallest operation (tag 0, 34 wire bytes) pays the quote of the largest variant. The size of the operation does not matter, only the quantum.
- The test quantum is `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation`. In the failing captures (`☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/🗑️generated/terrain-native-history-issuers.log`, `...targets-owned-home.log`, 21 Nov 18:26 and later) that grant was 32 KiB copy bytes, below the 33,976 B history replay step and the receiver quote.

## Why other operations pass

`check_receiving_round_trip` and the other crates' receiving tests run with a 64 KiB copy quantum and small enum receivers. Only Terrain combines a large enum receiver with the 32 KiB mounted quantum. The same grant is the policy of both long acceptance laws (`acceptance_policy()` is `INTERACTIVE_COMPONENT`, `🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🎛️acceptance-policy-pump-and-drive`), so `history_edits_end_to_end` and `conflict_history_edits_end_to_end` replay operations through the same receiving path and would have stalled or spun on the same quote; they have no separate defect.

## Fix

1. Root cause (already in the tree, uncommitted, 22 Nov 15:02): `MountedOwnerPolicyV1::LARGEST_INDIVISIBLE_STEP_COPY_BYTES` is 377,696 and `INTERACTIVE_COMPONENT.maximum_copy_bytes` is `covering_copy_bytes(...)`, 524,288 (`🔌️plugin/🏇️mounted-owner/🦀️.rs`). The test and both acceptance laws read this one policy, so they now receive under a quantum that covers the receiver quote. Not verified: that the Terrain receiver quote is at most 524,288 B.
2. Diagnostic (this change): the stall message now names the unpayable quote and the quantum, `original receiving made no progress: quote Ok(RetirementDemand{...}) against grant RetainedCloneGrant{...}`. If the test still stalls, its panic shows the exact quote, so the policy constant can be set from a measured number instead of guessed.
3. Unit test: `receive_to_completion_names_the_unpayable_quote_and_the_quantum_when_no_turn_progresses` with a zero copy quantum.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs`: `stalled_at`, quote and grant in the idle message.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🦀️.rs`: new test.

## Tests to rerun, in order

1. DSL receiving crate (the new test and the existing `original_pack_value_receiving_decodes_the_same_value_as_the_pack_rt_oracle`).
2. Terrain editor lib: `-- history_operations_receive --nocapture`. If it fails, read the quote in the message. If it exceeds 524,288 B, raise `LARGEST_INDIVISIBLE_STEP_COPY_BYTES` to that quote and keep the power-of-two covering rule.
3. `history_edits_end_to_end` and `conflict_history_edits_end_to_end`, then the whole `--lib` run.
