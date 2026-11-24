# L42 Clipboard Admission Slot Release

## Failure
`gesture_kinds::cancelling_clipboard_admission_restores_the_unpublished_operation_slot` (gesture-kinds test, line 297): after cancelling the Cut mount at phase 4 and draining `maintenance_source_close_step`, `typed_operation_reservations[0] == Some(896)` stays held. Mount and ingress were released correctly.

## Root Cause
- The slot is reserved in `admit_original_tool_lease` (producers, `typed_operation_reservations[id % SLOTS] = Some(id)`).
- Phase 0..3 (permit still in `pending_tool_lease_admission`): `close_original_tool_lease` (docs/lease) releases the slot via `release_reserved_emit_slot`. That is why the phase-0 sibling passes.
- Phase 3 moves the permit into `OriginalEmissionAdmission.mounted` and records `owner.operation`. From then on the only release was phase 7 publication (`insert_admitted` + `release_reserved_emit_slot`).
- The cancel path goes through `close_original_reserved_mount` (plugin-app-impl/reserved-mount). It retires window captures, mounted operation, lease and source, then `take()`s the empty shell and drops `owner.operation` without returning the slot. Nothing else clears it, so the slot leaks.
- l37's change (`typed_operation_slot_is_vacant` treats queued `reserved_commits` as occupied, keyed leases, latest-wins permit reuse) is not involved. It only reads vacancy and never cleared this reservation. The leak was hidden until l36's ingress fix let the test reach line 297.

## Fix
The shell retirement now returns the publication slot it owns. The release is guarded by identity (`release_reserved_emit_slot` only clears `Some(id)`), so it does nothing after a normal phase-7 publication has already cleared the slot. It also never touches a foreign slot.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🔖️reserved-mount/🦀️.rs` `close_original_reserved_mount`: `if let Some(operation)=self.media.reserved_mount.take().and_then(|owner|owner.operation){self.tools.release_reserved_emit_slot(operation.operation.0)}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs` `advance_original_reserved_mount` phase 8: same release on the shell take, so the live phase-7 "mounted absent" path cannot leak either.

## Verification
Not compiled or run (build lane busy). Rerun the 3d editor tests: both `cancelling_clipboard_admission_*` tests should pass.
