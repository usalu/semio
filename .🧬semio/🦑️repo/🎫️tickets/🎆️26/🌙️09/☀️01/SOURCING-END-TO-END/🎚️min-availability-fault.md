# Minimum availability fault (browser journey step 14): status 2026-11-24

## Not reproduced natively so far
Laws added in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (all pass under the browser per-turn grant, with the live-cleanup clock driven 256 units after each edit at one item and 4096 bytes per unit):
- `the_min_availability_filter_settles_under_the_browser_grant_and_keeps_the_instance_alive`: typed command, value then two steps, then a query edit.
- `the_min_availability_wire_payload_applies_in_every_number_form`: action endpoint with `{"value":3}`, `{"value":4.0}`, `{"delta":1}`, `{"delta":-1}`, `{"value":"5"}` (passing before the gesture forms were added).
  Gesture forms (`gesture`/`commit`/`abort`) were added last and NOT yet run: the disk filled (ENOSPC, 117 MiB free) during that build.

## Ruled out
- Retained-bytes accounting `(1, 0)` at `📬️config-store-preparation/🦀️.rs:77`: gate is the fixed 4 KiB grant, not the bytes; zero bytes does not matter (law passes).
- Codec: op text/binary round trip and the config diff/inverse for the scalar already have passing tests.

## Strongest lead (unverified)
The browser delivers the number input as a CONTINUOUS control: `useContinuousTriggerLane` (`🗣️Interpreter/⚡️action-dispatch/🟦️.tsx:38`) dispatches `change` with `{ value, gesture, commit }` (ticks with `commit:false`, a release with `commit:true`, `abort` on blur/retire). Query/module/typology never carry `gesture`. In the guest, any action with a non-empty `gesture` goes through the scrub/press path (`🧵️instance/🧾️typed-operations/📥️staging/⌨️typing/🏷️tag/🦀️.rs` `capture_original_tool_tag` -> `OriginalScrubDispatch` over `PressLeaf<M, CM>`, `🛠️tool-machine/🦀️.rs:151`), while `sourcing_curation_command_from_action` (`✏️editor/🦀️.rs:148`) ignores `gesture`/`commit` and emits a plain absolute `SetFilterMinAvailability` config mutation on every tick and again on commit (the release repeats the tick's value, so it is a no-op edit). That is the only sourcing command whose config edit rides a scrub press; the cleanup of the press leaf (`PressLeaf::Config(CM)`) is the suspect for `runtime live cleanup faulted` (the cleanup job reports a prior Fault/Cancelled outcome, the first fault is swallowed by `RuntimeLiveCleanupJob::advance_source`).

## Next steps once disk is freed
1. Run `verify-cargo.sh minav ... --lib min_availability -- --test-threads=1 --nocapture` (law with gesture forms is already in the test file) and read the `[DEBUG]` lines; a failing gesture form names the press leaf as root cause.
2. Fix in the framework press/scrub config leaf retirement if generic; else make the sourcing command a proper scrub tool.
3. Then the curation lib baseline 99/0 with `--test-threads=1`.
4. Remove the `[DEBUG]` println in the wire law before closing.

## Blocker
Disk full (`/dev/disk3s5`, 100%): Bash cannot start (no space for its output file). `🗑️generated/native-wasm` is 26 GB and `native/build` 52 GB; stale artifacts there need deleting by the user.
