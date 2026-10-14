# Lexical Receiving Control Guard Audit

Read-only proposed-design review, 2026-10-13. Guard not present in source at inspection; no builds or production edits.

## Ordinary Nested Loan Behavior

`SqliteSnapshotReceivingControl` borrowing `&mut SqliteSnapshotControl`, storing prior private required flag, setting true on entry and restoring prior in Drop is allocation-free. Rust exclusive loans force nested guards to drop in reverse order: inner sees previous=true and restores true; outer restores its original false/true. Result error and normal unwind run Drop and restore the original scope requirement without an explicit caller reset. With panic=abort there is no continuation requiring restoration.

The guard must remain inside `TypedFileSettlement` through successful close/publication callbacks and refused residual settlement. `RefCell<Guard>` body reborrows need Deref through RefMut -> guard -> control; drop all RefMut/Port loans before post-body observer settlement. Nested validator settlement may temporarily borrow another guard from the same control only while outer observer remains body-mode and does not touch its RefCell. Do not drop the inner guard before its owner receive completes.

The required mode must propagate explicitly into `with_workspace_recipient`, `with_retirement_child` and `with_original_allocation`. Original allocation child should retain required=true and installs the actual port; it must not interpret port presence as permission to reset. Dropping a child does not restore parent's counters by copying its ledger: cumulative ledger is Lent and stays shared. Existing workspace/retirement functions copy back actual forwarded/limits after Result; original-allocation uses local forwarded=0 to match start-relative native receiver and does not copy that local counter to parent. Preserve this behavior; admission requirement is independent of forwarded counter.

## Concrete Mutable-Target Escape

Public `DerefMut<Target=SqliteSnapshotControl>` permits safe replacement of the entire target: `std::mem::replace(&mut *guard, SqliteSnapshotControl::new(other_long_lived_callback, limits))`. With another suitably long-lived callback the caller can move the required original control out and leave a standalone control in its place. Guard Drop restores the previous flag on the replacement. Private guard fields and absence of a flag setter therefore do not make the requirement unforgeable. `mem::forget(guard)` conversely leaves the original required flag active after the loan ends; it does not disable it, but lexical restoration is not guaranteed against forgotten guards.

This is a concrete API limitation, not a claim current first-party producers perform replacement. If requirement is intended as ordinary first-party discipline, document/test that limited contract and prohibit replacement in Source laws. If it must prevent authority replacement, unrestricted DerefMut is incompatible with that promise: expose narrow guarded operations/reborrow into internal functions rather than a public freely replaceable control target, or make immutable original ledger identity/requirement part of the receiving operation's checked loan so a replacement cannot pass. Final aggregate settlement remains necessary but cannot be the sole mode-reset defense when unrelated slack exists.

## Prebirth Checks And Laws

Nonzero `admit_allocation_bytes` and `allocation_stage` reservation must check required mode + port presence before callback/counter update. Zero reservations remain allowed. Port native receipts and genuine allocation_stage_native remain allowed, with existing original receipt invariant. Do not switch unforwarded cold validators to the native-settlement API as a workaround.

Test nested false->true->true->true->false restoration; already-required parent remains true; error/panic closure restoration with zero births; all three child constructors; bound original child accepts and forwards exact bytes; unbound stage/direct admission refuses before birth despite unrelated native slack. Add a safe-control-replacement adversarial compile/runtime law if the public contract promises original authority cannot reset. Existing current source tests were not modified or executed by this audit.
