# Archive receiving regression: shared-slot `share_check`

## Root cause
`NativeReceive` derive (`🌱️value/✨️derive/🛬️receive/🦀️.rs`) gained shared receiving slots for same-typed non-leaf fields plus a `share_check` at field stage 1 that returned `invalid()` when the slot was not `terminal_is_empty()`. Stage 1 runs after the funded admit action (14) has already initialized the slot, so the slot is by definition non-empty there. The check fired for every shared field (the owner included), so every record with two same-typed nested fields failed with `native receiving original type, field or phase is invalid` at the archive door.

Slot reuse is already guaranteed by the stage machine: stage 4 plans and runs begin-close (action 21), stage 7 runs close (action 15), stage 6 `share_reset` replaces the closed, empty slot with a fresh default before the next field.

## Fix
Removed `share_check`; kept the shared slots and `share_reset`.

## Law
`🌱️value/🛬️decode/🧳️owned/📦️packages/🦀️rust/tests/native-record-ordinal-leaf.rs`: `two_same_typed_nested_fields_share_one_receiving_slot_and_both_arrive` (record of two `Point` fields). Passes natively (3/3 in the file).

## Second fix (blocker, not the regression)
A concurrent split of `artifact_app_laws` into `semio-plugin-testing-instance-laws` left `semio-plugin-testing-acceptance` (`🧪️tests/🧪️history-edit-acceptance`) unable to compile (`render_registered_fixture_app` not found). Added the `semio-plugin-testing-instance-laws` dependency in its `Cargo.toml` and pointed `💾️reload-and-child-history-scenarios/🦀️.rs` at `semio_plugin_testing_instance_laws::render_registered_fixture_app`.

## Results (native, `--test-threads=1`)
- curation lib: 99 passed, 0 failed (the 4 regressed tests pass; baseline was 98).
- `aussuchen_pane_smoke`: 1/1 pass.
- `semio-framework-value-native-owned` full suite (`--no-fail-fast`): all pass except `actual_native_schema_matcher_driver_visits_flattened_original_names` in `native_work` and `record_matcher_driver` (turns for "alpha" 141 vs 153). That test uses `NativeRecordMatcher` only (no `NativeReceive` derive), so it is unrelated to this fix; probably the peer's effect-frame/receipt work.
- Disk filled (ENOSPC at link) mid-run; freed old logs and stale curation test binaries under `🗑️generated`.

## Wasm
The derive is a proc macro on the shared receiver path, so the sourcing wasm needs a rebuild to pick up the fix.
