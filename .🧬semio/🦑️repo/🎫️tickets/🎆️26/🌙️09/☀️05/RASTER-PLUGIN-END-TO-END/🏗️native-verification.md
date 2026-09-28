# Native Verification Build Investigation

The active native run 7 is still compiling real dependencies; its process handle is live. No replacement was started.

The installed Nx implementation supports `--excludeTaskDependencies`, but that is not evidence that skipping required generators would be correct. The UI axes publisher already calls `writeGeneratedFileIfChanged`, so repeated generator invocation alone does not establish the cause of the observed rebuilds. Shared source changes and concurrent Cargo invocations remain possible contributors. Preserve the existing dependency gates until there is concrete evidence that an unchanged-output generator is causing unnecessary invalidation.

No build configuration or generator behavior was changed. Continue polling session 29929 for the current Raster run and session 18283 for activation 13; previous native runs 5 and 6 are terminal failures with their results recorded in the export notes. Framework progress run 2 is terminal and passed its three selected tests.

## Run 8 Shared Kernel Compilation

Raster native run 8 (handle 12384) ended before tests with five shared retained-clone kernel compilation errors: ordered-map key projection added an extra reference, insertion cursor called old lookup parameters and removed close.push, and OptionCursor projected a captured child reference across a higher-ranked closure. Inspection found the ordered-map changes already being updated in the shared checkout. Only the remaining OptionCursor projection was corrected here: project the child from the closure's borrowed Option and use `Some(_)` for the outer shape branch. No ordered-map edits were made by this task. Full native run 9 is running with the same target/checks. This remains unverified until that run completes.

Run 9 (handle 3965, terminal exit 1) confirmed the Option projection error was gone, but a newly exposed ordered-map insertion branch returned RetainedCloneProgress instead of its declared RetainedOrderedMapInsertProgress. That exact default progress type was corrected (one expression, no change to insertion logic). Run 10 is now the active full native verification. No Raster tests ran in runs 8 or 9.

## Latest Native and Activation Status

Run 10 (handle 3432, terminal exit 1) passed the previous kernel compilation points but stopped before Raster tests in shared UI WGPU draw types: its raster texture keep-key match does not cover the newly introduced `SceneMaterialKind3d::Authored(_)` (around line 319). This is a texture retention path, so adding a blind wildcard would risk evicting authored-material textures. No speculative arm was added. This is a different blocker from the fixed retained-clone compilation errors.

Activation 13 remains live (session 18283, Nx PID 963). Read-only process inspection identified its Cargo PID 2894 with no compiler child; a one-second native process sample explicitly showed Cargo's `prebuild_lock_exclusive` → `LockManager::lock` → `flock` wait. This is evidence of waiting on a build lock, not a completed activation. No process or shared lock was deleted or terminated. Diagnostic outputs remain under generated.

## Authored Texture Retention Compile Repair

Inspection established that SceneAuthoredMaterial3d owns one optional base_color_texture. The WGPU raster_keep_step match now publishes that exact key when present and Pending when absent, preserving the existing bounded one-draw step. The existing all-world/UI/overlay residency law now contains both textured and untextured Authored material draws and expects five live candidate keys. This is a semantic repair, not a wildcard that would discard the resource.

A focused existing ui-rs:test-wgpu-engine target is running (raster-authored-texture-native-1.log), along with full Raster native run 11 (raster-export-native-11.log). These two runs remain unverified until terminal results are inspected.

Focused authored-texture run 1 (48045 terminal exit 1) stopped in shared prepared-scene byte accounting, which had begun calling `capacity()` on a new `texture_key()` helper returning &str. Re-read current source: the peer-added helper correctly exposes both painted and authored keys. The raster keep step now uses that shared helper. Byte accounting obtains the underlying &String explicitly for the two owning variants so capacity remains exact. Focused run 2 is pending; Raster run 11 remains live. No source-length approximation was substituted for capacity.

## Affine Integration Verification

Old Raster run 11 (24715) ended before tests because it compiled the prior shared pixel transform before reading the updated Raster bridge. Full affine Raster run 1 (67192) and shared pixel run 1 (56836) then ended on workspace manifest loading before tests. Read-only inspection found all renderer dependency manifests present; the existing renderer-wgpu lockfile-check target passed (10271, affine-workspace-lockfile-check.log), establishing the workspace metadata could load. An attempted nx exec metadata call (44846) instead stopped on a project graph cycle before Cargo; no graph edits were made.

Full affine Raster run 2 (40235) and native surface paint run 1 (59459) are live. Shared pixels affine native run 2 (61087) passed 38 tests, zero skipped. The focused authored-texture residency run 2 (61422) passed its one selected test, with 725 filtered out, confirming the shared retention/byte-accounting repair. Activation 13 (18283) is still live and predates current affine source; it does not establish a usable current build.

Full Raster affine run 2 (40235) ended before tests on ten missing-field errors in shared XML: XmlDocument.epilog and XmlDoctype.prolog_position. Read-only inspection subsequently confirmed all reported initializers and codecs were being updated by other work. No XML edits were made here. Raster affine run 3 (97875) retries after those source repairs. Surface run 1 (59459) remains live. Mounted affine red run 1 (4223) executed 23 passing/8 failing tests; green run 1 (78690) is active after explicit identity fixture repairs and four new mask-coordinate cases.

## Latest Process and Compile Checkpoint

Mounted full-host compositor completion/cancellation run 1 (40038) passed **2 selected tests, 703 skipped** (`raster-affine-host-1.log`). Mounted affine editing is **35/35 passed**.

Raster affine run 3's Nx session 97875 exited **137**, without a native test result or explanatory diagnostic. Crucially, its child Bun PID 74866, nextest PID 74868 and Cargo PID 74872 remained alive afterward, writing the same log. This is not a completed native test run; do not launch a duplicate while those descendants are active. No cause such as memory exhaustion is inferred from the exit code alone. Inspect the existing process/log before deciding the next action.

Surface native run 1 (59459) ended before Rust tests on two new shared retained-clone preparation compile errors. The factory constructor could not infer the edit generic, and live authority was calling nonexistent stamped_mutation_id. The constructor now explicitly supplies `<P, M, E>`. Store inspection confirmed that admission copies its stamped mutation identity into stamped_edit_id, and batched folding uses that accessor for the first mutation, so preparation now uses the same existing accessor. No authority identity is synthesized differently. These two narrow repairs are pending verification in surface native run 2 (75160, raster-affine-surface-native-2.log); shared-pixel and mounted passes do not validate them.

Activation 13 (18283) remains live and predates affine changes. Current Raster document integration, native paint, archive/selection/cancellation follow-ups and live UI acceptance remain unverified.

Subsequent process inspection confirmed all three Raster run 3 descendants had exited, with no additional log result. Run 3 is now terminal/unverified, not active. Full affine native run 4 is started after the shared preparation fixes; its log is raster-affine-native-4.log. No duplicate descendants were running when it began.

Native surface paint run 2 (75160) passed 66 tests with 196 filtered, exit 0. This also compiled through the retained-clone preparation repairs. Full Raster run 4 (63457) is confirmed live; dependencies are still compiling. Its source now includes asset-replacement capacity regressions, so expect 317 tests if those sources were included. Raster TypeScript asset-replacement green run 3538 passed 134 tests after red run 61622's missing-module failure (127 passed). Both TS handles are terminal. See asset-replacement.md for the new publication-order fix and exact native scope still pending.

## Layer Transform Checkpoint

Full affine run 4 (63457) ended before tests with one test-only API mismatch: encode_pack returns Vec<u8>, so its erroneous unwrap was removed from the mask linkage regression. Layer-transform native run 1 (18490, raster-layer-transform-native-1.log) retries current source, including the complete layer transform mutation and four additional native tests (expected 321). It remains live. Native mask linkage, asset capacity, archive/deletion/cancellation fixes and new transform publication/history remain unverified until terminal results are inspected. Activation 13 remains live and predates these changes.

Raster TypeScript green run 3 (41757) passed 139 tests, zero failures. This includes independent Three.js composition and semantic payload/diff checks. An additional ambiguous full/partial patch regression is being verified in green run 4.

TypeScript green run 4 (29252) is terminal, exit 0: **140 passed, zero failed**. Full/partial ambiguity and null partial-field acceptance are now explicitly covered. Source review also fixed the new mutation's missing entry in the ordered KINDS vocabulary. Protocol inspection confirmed wire tags are looked up by keyword in the normative protocol file, not inferred from Rust enum position; the new metadata tag 16 matches its protocol record. No tag reorder is needed.

The additional singular-patch regression failed as intended in TS red run 2 (19836, exit 1: 139 pass/1 fail). The diff parser now calls the shared inverse validator, matching native rejection. TS green run 5 (48286, exit 0) passed all 140 tests, 582 assertions. This is the latest TypeScript source checkpoint. The oracle catalog's supported kinds list also includes change-layer-transform.

## First Current Full Native Result

Layer-transform native run 1 (18490) is terminal, exit 1: **321 tests run, 317 passed, 4 failed, 0 skipped**, 16m32s total. Four failures: editable_archive_restores_nested_masks_assets_adjustments_and_history (parent Pack/SPR Identity), png_command_download_survives_operation_retirement_and_preserves_history (job-session.terminal-fault), retained_image_export_completes_or_cancels_without_mutating_history (media export job faulted), raster_owner_caps_and_all_mutation_variants_retire_one_owner_per_grant (16 fixture variants vs 17 descriptors).

All four new layer transform tests, mask affine linkage law, full-capacity pixel/mask retained history, selection pruning, and other 317 cases passed. Archive fixture now sets the required dialect, as production initial app envelopes do. The retirement fixture now includes ChangeLayerTransform. Export has temporary test-only [DEBUG] timing/error logs around its advance step; these MUST be removed after diagnosis. Native run 2 (75362) is active against those changes. No shared job runtime or watchdog policy was changed.

## Current Green Full Native Checkpoint

Full Raster layer-transform run 2 (75362) is terminal, exit 0: **321 tests passed, zero skipped** in 22m29s total. It verifies the canonical-dialect archive fixture and complete retirement fixture fixes. Both previously failing export tests passed too; their prior terminal faults did not reproduce and the cause remains unestablished. The retained Nextest artifact directory contains only binary metadata, not success stdout, so it supplies no timing diagnosis. Do not attribute the earlier failures to load without evidence.

Removed both temporary test-only [DEBUG] export timing statements. A focused media_export run (raster-export-clean-native-1.log) now verifies the uninstrumented source, with NEXTEST_SUCCESS_OUTPUT=immediate (confirmed in local Nextest help) so successful test output is retained in the ticket log. No watchdog policy or export timing contract was changed.

Native paint green run 93882 is terminal, exit 0: **68 passed, 196 filtered**. It verifies locked target admission, inherited locks and gesture revision invalidation. Mounted revision run 75604 is terminal, exit 0: **43 passed**; it captured the initial eight revision cases. A second mounted run (44843, raster-stroke-revision-mounted-2.log) verifies the current twelve-case fixture, expected total 47. Shared pixels/host TS green run 82434 passed **150 tests**, including strict source checking.

Focused uninstrumented export session: **69035**, live at launch. Mounted current-fixture session: **44843**. Do not poll terminal sessions 75362, 93882 or 75604 again.
