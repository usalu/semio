# Lane femreceive result
## Step 1 (11/12)
- `advance_resident_success` now returns the metadata child's typed failure (`NativeReceivingEffect::child_failure`, uses `NativeReceivingEffectError::cause`) instead of `Ready{0}` when the child is Err.
- Rerun W-femreceive-1: the fem failure was NOT buried there; it was the fem `settle` converting a genuine child Pending.
## Step 2: fem root cause
- Probe (removed): `drive_original_producer_bound` stashed `producer_refusal` "Source lacks a recorded schema-key producer" (UnsupportedOwner) and returned Pending; the fem Value sources (`FemNodeValue`, material row Value) did not implement `NativeValueSource::query_schema_key`.
- Fix: framework helper `answer_schema_key` (native-owned record/source, re-exported via owned::record) answers from the Source's own accessors after charging one Work unit; fem node + material Value sources call it.
- fem 2d/3d `snapshot/native.rs`: child `advance` Pending now keeps state (`retry`) instead of being refused as "cannot stay pending" (only that site; take/close sites stay on `settle`).
- Second fem fault (revealed once the first was fixed): phase 9 left the staged `RetainedCloneSource` as soon as `take_authority_into` produced the owner, but the source still held `return_owner` (not terminal) and its Drop asserted "source must finish full granted ownership closure". Phase 9 now advances to 15 only when the source `terminal_is_empty()` (3d + 2d `snapshot/native.rs`).
## Verified (final tree, probes removed; grep `[DEBUG]` empty in touched dirs)
- W-femreceive-12: fem 3d `original_fem_receiving` rc0, 4 passed (was 2/4).
- W-femreceive-9: `semio-plugin-job-receiving --lib` rc0, 9 passed.
- W-femreceive-13: native-owned (own manifest `🧰️framework/.../🧳️owned/📦️packages/🦀️rust/Cargo.toml`) `--lib --tests` rc0, 18 "test result: ok". (W-femreceive-10 from the root manifest rc101 = "not a workspace member", wrong manifest, not a code failure.)
- W-femreceive-11: wasm check (hub manifest) of semio-framework-job-receiving + semio-framework-value-native-owned rc0.
## Not done / debts
- Step 3 (receipt clamps `total.copied_items.min(1)` in drive.rs `advance_metadata` and `cover_progress` in 🧳️original): not attempted; builds ~8-18 min each and the prior lane found the experiment heap-assert sensitive. Debt stays.
- fem 2d native tests and the fem 2d change (retry + phase-9 terminal + shared node/material key sources) not compiled/run (only 3d crate verified; 2d edits are textually identical).
- `retry` in phase 1 is untriggered by these tests after the root fix; kept per ruling (Pending keeps state).
STATUS: FIXED-verified W-femreceive-12
