# L11 semio-plugin-instance-app build

## Activity check
plugin-app-impl/natural-publication (03:13) and plugin-app-impl/lib (03:16) were touched in the last 15 min at start; no other file under the instance folders changed afterwards. Waited the 10 min window, no further edits, then proceeded.

## Design evidence
- No ticket under 26/11/2x mentions natural-source or reserved picker; evidence is the code only.
- The natural source is a whole new pipeline: decl-media `original_media/natural_source*.rs`, `ArtifactReservedToolInput::NaturalPaged{port,source}`, tool `import-natural-source` (`NATURAL_SOURCE_TOOL_ID/SCHEMA/CONTRACT`, wire must be empty: `admits_paged_natural_request` requires `raw_wire_bytes == 0`), registered in instance-jobs ingress, consumed by decl natural-file `NaturalFileImportJob.pending_paged`, and driven by `semio-plugin-instance-natural-publication` through the `NaturalPublicationHost::natural_dispatch` hook. Only the instance-side producer (render prepare + dispatch mount) was missing.
- The picker: `ArtifactReservedToolInput::OriginalPicker` and `RawMediaPicker` still exist and are used by the PDF plugin; the render `advance_reserved_picker` and its `original_picker` preparation flag were deleted by the refactor while the consumer stayed. Rather than a compat shim, it was re-implemented on the same shared paged-input path as natural (empty wire, `import-media` verb).
- `ToolOperations::public_tool_contracts` was removed from instance-jobs on purpose (jobs diff) while the `PluginApp::tool_public_contracts` trait method still returns `Vec`; the list is now built in the app impl from `bounded_contracts()` + framework/app registrations.

## Changes
- `.../instance/🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs`: `advance_reserved_picker`, `advance_reserved_natural_source`, shared `advance_reserved_paged` + `donate_paged`; `assemble_reserved_request` treats `OriginalPicker`/`NaturalPaged` like `Media` (tool id from header 1).
- `.../instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs`: `advance_original_reserved_picker`, `advance_original_reserved_natural_source` (mirror `advance_original_reserved_media`, mount via `advance_original_reserved_mount`).
- `.../instance/🔌️plugin-app-impl/🦀️.rs`: `tool_public_contracts` builds the contracts inline.
- `.../plugin/⏪️time-travel/📒️time-travel-ledger/🦀️.rs`: `original_close_diagnostic` now `pub`.

## Checks
- `cargo check -p semio-plugin-instance-app` (framework workspace, tmpfs 3g, jobs 2): Finished, 0 errors (this includes typed-ops and instance-commands as dependencies). Log: 🗑️generated/l11-check3.log.
- 2D board-host `cargo check --tests`: see 🗑️generated/l11-board-host.log (result appended below).

## Round 2: semio-plugin E0432 (history-edit-acceptance)
- Cause: the acceptance crate (`semio-plugin-testing-acceptance`, 🧪️tests/🧪️history-edit-acceptance) had dropped `assert_latest_history_preview` and `assert_mounted_history_overwrite` (working tree vs HEAD; last edits 2026-11-21 21:04, not active) while `semio-plugin`'s `app::history_edit_acceptance` re-export and two plugin test consumers (xlsx base editor unit test, gisterrain commands unit test) still use them. No replacement exists; the same refactor added `with_acceptance_caller` and `assert_closes_live_history_session` (used by the puzzle 3d `@close` law arm).
- Fix: restored both asserts in 🎛️acceptance-policy-pump-and-drive/🦀️.rs in the new style (preview one funded through `with_acceptance_caller`), re-exported them from the crate root. The removed `TimeTravelLedger::preview_due` field is replaced by the equivalent assertion `has_pending_work() || preview_progress().is_some()` on the deferred draft.
- Check: `l1-native.sh 2d check --tests -p semio-s-artifact-puzzle-2d-board-host` Finished, 0 errors (803 crates, log 🗑️generated/l11-board-host3.log).
