STATUS: UNVERIFIED (compile). Fem2dReceiving and Fem3dReceiving ported and installed; the 3d panels run statik-233 (rc 1, log 🗑️generated/1105b/statik-233.txt) stops upstream in semio-os-store-snapshot-clone (store4) and the framework crates it needs, so no fem crate has been compiled yet. Pane smoke (task 3) not reachable.

## Task 1 verdicts
- statik-232 (2d panels): rc 1, refused before the compiler: `semio-s-artifact-fem-2d-model` with feature `testkit` (which the panels crate enables) measured 262,928 B at the gate (`closure.ts <2d model manifest> testkit`), 🏋️loads file read with 4,776 B left. The earlier measure without features (261,683) hid it. The new port shrinks the file: now 261,921 B (223 B room). Taxonomy accepted the new `🔺️diff/🏋️loads` directory (the gate read it; no taxonomy refusal).
- statik-231 (3d panels): running; no error yet in the log at 16:30 (log 🗑️generated/1105b/statik-231.txt).

## Task 2 Fem3dReceiving (and identical Fem2dReceiving)
Same machine in both crates; one script ported both (scratch only). Changes in `📸️snapshot/native.rs`:
- children are `NativeReceiveSlot<Vec<FemNodeValue>, Nodes>` / materials (was NativePersistentListReceiver); phases 1-4 and 5-8 now share one `child!` macro (the file shrank 36.5 -> 35.5 KB).
- every framework action in advance (admission advance, slot take, slot close, source take_authority) runs inside `NativeDecodeControl::with_allocated_effect(d, ACTION_WORK=4, ..)` measured by `work.measured`; the slot advance is polled with `native_work_limit`; a Pending poll is a typed refusal (synchronous recipient, ruling R2).
- `close_step(effect) -> OriginalRetirementPoll` (shell release = `effect.performed(1)`, then retirement `step_granted`/`begin_granted(effect)`); `close_quote` merges demand and work quotes (`close_work` new, feeds `next_computational_work`).
- `RetireOwned` seed form (`type RetirementCursor = Self`), `RetirementCursor::close_step(effect)->RetirementStep`; the own `ErasedSnapshotRetirement` impl removed (the driver frame `NativeSnapshotReceiving<T,O>` implements it for any RetireOwned).
- `receive_authoring`: `drive_cursor` closure is the 6-argument form, publishes into `output`, records progress via `body.record_progress`.
- UNVERIFIED guesses to check first on a compile: `NativeSnapshotFailure: From<OriginalReceivingPrimary / OriginalRetirementFailure>`, `ACTION_WORK = 4` as the per-action work ceiling, `Poll::map`, macro `self` hygiene in `child!`.
- Not ported: cfg(test) files `🧪️tests/native-tests.rs`, `original-receiving-tests.rs` (use the old `advance(control,body,grant)`, `admit_source`, `close_step(grant)`); they are not on the panels path.

## Run log
- statik-231 (old state, 16:03-16:22): rc 1, only error crate semio-os-store-snapshot-clone (unresolved `retirement::original_vector`, `NativeDecodeConstructionRecipient`). 0 errors before it.
- statik-233 (3d panels with the port, 16:40-17:25): rc 1. 100+ errors, all in framework crates before any fem crate: value receiving (🫴️receiving), job escrow (📤️publication/🪙️escrow), native-owned protocol, ui-contract accessibility, store-snapshot-clone (`semio_framework_value_original_vector` unresolved, `RetainedWorkPoll` used as an outcome, `begin_close` missing on the cursor). Owners per claims: store4 (snapshot-clone), others unclaimed in my reading; none are fem.
- statik-232 (2d) cannot be rerun before the same frontier clears.

## Task 3
Blocked by the crates named above; no statik-owned crate on the path has a known fault beyond the unverified port. Next run once store-snapshot-clone and the listed framework crates are FIXED in `1105b-claims.md`:
`1105b-tools/runtime/tst.sh statik-234 "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" native -p semio-s-artifact-fem-3d-editor-panels --lib`, then the 2d twin, then `statik/smoke.sh`.
