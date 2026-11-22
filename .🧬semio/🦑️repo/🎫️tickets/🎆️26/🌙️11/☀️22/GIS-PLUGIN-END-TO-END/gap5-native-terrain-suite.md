# Gap 5: Native Terrain Library Suite

Static analysis only. No cargo, build or dev server ran. No source file was changed: none of the remaining failures has a defect that can be proven from source alone, and the audit notes plus the latest logs show the earlier fixture and window fixes are already in the tree.

## Which crate

The "full native library" of 42 tests (32 passed, 10 failed) is the lib test target of the Terrain editor crate, not the `gisterrain` root crate.

| Crate | Package | Tests | Last known result |
| --- | --- | --- | --- |
| `semio-s-artifact-gis-gisterrain-editor` (`--lib`) | `gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust` | about 42 (`editor::gis3d::...`) | 32/42 (log `terrain-worker-custody-lib.log`, 20 Nov ticket) |
| `semio-s-artifact-gis-gisterrain` (`--lib`) | `gisterrain/📦️packages/🦀️rust` | 21, sqlite snapshot laws | 21/21 at 22 Nov 12:56 (`🗑️generated/gisterrain.log`) |
| `semio-s-artifact-gis-gisterrain-editor` (`--test mounted`) | same crate, `🧪️tests/🧩️mounted` | 22 | 22/22 |

The handoff statement "gisterrain library tests pass 21/21" is the root crate. It says nothing about the editor lib suite, so it must not be cited as the answer to Gap 5. The 21/21 run did compile the editor crate non-test code at 12:56 on 22 Nov, after the last edit of `🎮️commands/🌍️geojson/🦀️.rs` (11:33). The `E0599 Error::fault` compile break in the 21:04 20 Nov capture (`terrain-native-history-template-seeds.log`) is therefore stale.

The `[workspace]` root is `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml`. The editor crate is a member.

## Command for the coordinator

```
repo cargo --lang=en --store=shared --configuration=prototype --parallelism=2 -- test --locked --offline --manifest-path "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml" -p semio-s-artifact-gis-gisterrain-editor --lib -- --test-threads=1
```

Run it once on the current checkout and record one number. Add `--nocapture` after `--test-threads=1` to get the `[TRACE]` and stage lines. For triage run these filters first (same prefix, append the filter after `--lib`):

1. `history_operations_receive`
2. `gis_terrain_window_config_isolates`
3. `exaggeration::` (slider press tests, three)
4. `mounted_terrain_contours`
5. `history_`, `conflict_history` (acceptance law tests, the first filter also matches 1)

The ordinary mounted target is `... -p semio-s-artifact-gis-gisterrain-editor --test mounted`.

## The ten failures

Evidence is the last captured run of each, in `☀️20/GIS-TERRAIN-AND-OVERLAYS-END-TO-END/🗑️generated/` unless stated. "Source state" is what the tree contains now.

### 1. Slider press tests (three) and the press History test

Tests, all in `✏️editor/🎮️commands/🧪️tests/🔬️unit/🦀️.rs`, module `exaggeration`:
`a_slider_press_is_one_transaction_one_edit_and_one_row`, `a_cancelled_press_leaves_zero_trace`, `two_presses_are_two_transactions`, and (now removed, see below) `a_committed_press_edited_in_history_replays_deterministically`.

Symptom (`terrain-native-current-press.log`, 19:50, panic at the current line 145 `settle_registered_typed_operation(...)expect("the slider turn and its projection settle")`):
`original fixture settlement stopped after 32768 turns ... pending false, maintenance empty false, stage 104, demand copy_bytes 1, close owner Begin`.

Cause: the fixture driver is already the corrected one (one retained dispatch plus the mounted settle loop per tick, preview and cancel receipts must omit the Artifact lane). The failure is now inside the runtime settle contract:
- Stage 104 is `MaintenanceUnit::ToolOverlay` (`🔌️plugin/🪜️maintenance-ladder/🦀️.rs:105`).
- `settle_registered_typed_operation` loops while `!maintenance_terminal_is_empty()`. That predicate includes `!tool_overlay_retirement_pending()`, which is true while a press preview has `refold_owed`, an `original_scrub_dispatch`, `original_press_settlement`, tag capture or window preview retirement outstanding (`🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs:347`).
- The ToolOverlay unit returns `AwaitingInput("tool overlay waits for the original foreground source")` whenever `original_tool_follow` is absent and an action source is still held (`tool_overlay_waits_for_foreground`, same file line 169). The loop then spins 32,768 turns without that foreground source ever advancing, because the foreground dispatch already returned.
- The same preview refold is what the render shows (`overlay_or(snapshot)`), so a live preview and "maintenance terminal empty" can never both hold after a tick. A tick is by design not a quiescent state.
- Gisterrain-specific fact: `setExaggeration` is a retained-command tool (`✏️editor/🦀️.rs:353`, `action_interactive_job ... Migrated`), so each tick is both a retained command job and a scrub tick.

Conclusion: not a Terrain test or leaf bug. The forms fixture (`📋️forms/.../🧪️field-transactions/🦀️.rs` `patch`) uses the same dispatch-then-settle pattern, so the same law applies to it. Fix has to be one of: (a) the settle loop treats a live press overlay (`tool_machines.presses`/`gestures` non-empty and the foreground source released) as settled, or (b) the tick fixture settles only to "pending false" with `app.advance_typed_operation_publication` and does not wait for maintenance terminal-empty (the press is only fully quiescent after release or cancel, which the tests already do). Both change shared runtime or shared fixtures owned by the framework refactor, so they were not edited blind. Next diagnostic (one run): print `tool_machines.refold_owed`, which of the five action-source options is `Some`, and `original_tool_follow.is_some()` in `typed_operation_settlement_refusal` (`🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:358`).

Source state: the History-edited press test was removed from the unit file by the 22 Nov 13:42 commit (`git diff b4927d2c311 01324745f80`). History overwrite coverage for Terrain is now only the generated acceptance laws (item 3). That removal is a coverage loss that should be restored once item 3 passes, using the exaggeration press fixture and `assert_mounted_history_overwrite`.

### 2. Two-window generation (`gis_terrain_window_config_isolates_two_registered_windows_and_reloads`)

Old failure: assertion compared a Store generation token with edit counts (`Some(2)` vs `Some(20)`), plus a `SetCamera` publication stall (copy stock spent from 133,428,248 to 369 bytes over repeated turns, constructor fixed copy term 19,632 bytes).

Source state: the test (`✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🔬️unit/🦀️.rs:160-200`) is already rewritten as relative generation checks: the addressed window's generation must advance, the sibling's must stay equal. The Window Store admission patch from `GIS-WINDOW-STORE-ADMISSION.md` is applied. No later native verdict exists. Nothing further to change statically. Expected: passes if the Store admission patch holds under the 32 KiB mounted turn.

### 3. History: `history_edits_end_to_end`, `conflict_history_edits_end_to_end`, `history_edit_inputs_resolve`, `history_exaggeration_fixture_matches_the_native_fold`

These four are generated by `semio_plugin::history_edit_acceptance_law!("gis", Gis3dPlayApp, gis3d_app_manifest_for_tests, "../..")` in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`. The macro is current (`🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs:172`, four arms, three test names generated for Terrain).

Progress since 32/42, in order of the captures:
- Missing schema index entries for the elevation mutation: fixed (schema index regenerated, `gis-history-schema-index-publication.log`). `history_edit_inputs_resolve` and `history_exaggeration_fixture_matches_the_native_fold` pass in every capture from 18:18 on.
- Missing replay semantic issuer: Terrain now provides `document_store_owners_source_demands` and `build_document_store_owners`.
- TimeTravel close demand 42,568 bytes over the 32,768 byte mounted copy budget: shared Store patch (explicit tag over initialized storage) applied.
- Remaining at the last completed run (`terrain-native-history-targets-owned-home.log`, 19:33): `history_edits_end_to_end` and `conflict_history_edits_end_to_end` still running after 2,578 s with no verdict; `[repo] still working` only. Not diagnosed from the log.
- `history_operations_receive_under_mounted_copy_quanta` (`unit/🦀️.rs:~50-70`): fails on the first operation, tag 0 (ChangeExaggeration, 34 wire bytes), with `OriginalReceivingPrimary ... "original receiving made no progress"` even under the mounted preparation quantum (64 items, 32 KiB). `receive_to_completion` (`🗣️dsl/🫴️receiving/🦀️.rs:258`) reports this when a cursor step returns zero work, zero progress and no native transition. That means the incremental receiver for `GisTerrainMutation` cannot pay its next quote under the quantum for the smallest operation, so the larger ones cannot pass either. Candidate cause: the enum receiver's first step quote depends on the largest variant (`ReplaceElevationSource` carrying `Option<ElevationSource>` with `Vec<Vec<i32>>` rows). After the rectangular-grid cut (`ElevationSourceFields.rows_millimetres: Vec<Vec<i32>>`) the incremental binding in `🧬️schema/⚙️operations` must charge per row, as `GIS-TERRAIN-HISTORY-RUNTIME.md` describes. This is not verified. Diagnostic: run only `history_operations_receive` with `--nocapture`, and add the quote that stalls to the panic message (the test already prints tag and wire bytes).

Because the failing tag-0 case is the same code path the History acceptance law uses to receive operations on replay, a fix for the quote stall is expected to unblock the two long-running acceptance tests too.

### 4. Contour lifecycle (`mounted_terrain_contours_regenerate_and_undo_as_one_overlay_mutation`)

The test body (generate, assert one overlay mutation, repeat for a no-op, regenerate at 100 m, Undo twice, Redo) passes its assertions. The failure is at the end, in `context::close(&mut app)`:
`registered fixture did not reach its exact terminal-empty witness, last close quote RetirementDemand{copy_bytes:32, release_bytes:32, depth:1} against remaining grant {maximum_items:1, ...} on rung RetainedFields` (`terrain-updated-press-example-contours.log`, 15:45).

Cause (inferred from `close_registered_fixture_app`, `🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:466-540`): the close loop retires the Store's retained document fields one quoted step at a time and gives up after `maximum_idle_turns` idle turns. The contour overlay replaces `imported_map.routes` with up to 512 `DslValue` route trees, so retiring the Redo/Undo alias snapshots takes many more 32-byte steps than the three other mounted tests that close cleanly, and the closing loop ends before the terminal-empty witness. `generate()` and `replace_contours` in `🎮️commands/📉️contours/🦀️.rs` are correct and unit-tested (pure tests pass in every capture). Not a Terrain logic defect. If the next run still fails here, limiting the test to one regenerate (so fewer alias snapshots exist at close) is a valid scoping change, but only after the shared close loop has been checked for an exact idle-streak accounting bug (progress counted only when `batch_progress != default`).

Source state: `🎮️commands/📉️contours/🦀️.rs` was edited 21 Nov 21:04, after this log. No newer verdict exists.

## What is stale

- The "32 of 42" figure and its list of 10 names (`terrain-worker-custody-lib.log`): fixture drivers, History schema coverage, issuers, camera config and demo mutation expectations have all been changed since. The press-preview and two-window items were already rewritten before the last runs.
- `native-terrain.md` and `terrain-acceptance.md` report "mounted 22/22" and "pointer journal 18/18"; those are other targets. Do not substitute them for the lib number.
- `terrain-native-current-mounted-and-unit.log` (22:26 on 21 Nov) ends in an `E0277 DecodedMissingSamples: NativeDefault` compile failure in the inferences crate. `🧬️schema/📸️snapshot/🦀️.rs:173` now implements `NativeDefault`, so that is fixed.

## Files changed

None. All three open items (press settle, receiving quote, contour close) require a first native run to confirm the proposed cause, and the first two point at shared framework code that another agent is refactoring.

## After the coordinator's run

Record: total passed of the 42, and each failing test with its first panic line. Items 1 and 3 should be assigned to the framework owner of the settle loop and of the receiving quote; if the receiver quote is confirmed for `GisTerrainMutation`, the fix belongs in `gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations`, with a per-row charge for the rectangular elevation grid.
