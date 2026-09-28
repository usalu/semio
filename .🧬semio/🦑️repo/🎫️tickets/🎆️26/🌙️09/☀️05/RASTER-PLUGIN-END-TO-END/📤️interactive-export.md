# Interactive Raster Export Boundary

Current source inspection, September 27. No retained Raster export implementation or runtime claim yet.

Raster `ArtifactEditor::export_media` still calls `raster_composite_media` for `image:out`. `raster_composite_job` drains `RasterStackPreparation::advance` in a loop, and `raster_composite_image` drains composition in a second loop. The underlying work is incremental, but the user-facing export path does not yield or expose cancellation between those grants. The synchronous artifact pack export is a separate editable-save path and needs its own ownership/export verification.

The framework now exposes `ArtifactMediaExportJobRequest`, `build_media_export_job`, `ArtifactMediaExportCompletion`, `ArtifactMediaExportCredit`, `ArtifactOutputChunks` and submit/poll/cancel media-export methods. Layout has the concrete reference in `✏️editor/⚙️engine/📤️export/🦀️.rs`: `LayoutMediaExportJobFactory` registers `export-media:layout:out` as a Migrated HostOnly tool and produces sealed segmented media rather than a final unbounded String. Its snapshot close lease and chunk/completion close ladder are required ownership behavior, not optional cleanup.

Raster needs a HostOnly `export-media:image:out` job with bounded source preparation (32768 pixels), compositing (4096 pixels), PNG encoding (the existing encoder consumes 4096 raw bytes per step), then bounded base64 output pages. Preserve the existing `2d.image` structured-media contract, credit both schema and output bytes, seal once, and publish only complete output. Keep a snapshot close lease through every cancellation/fault path. A job must not simply move the current draining loops into one scheduler step.

Required tests: neutral composite fixture and Sharp-decoded expected PNG pixels, small/large and transparent canvases, progress through every stage, zero grant no work, incomplete result refusal, cancellation during source/composite/encoding/output, exact chunk limits/output credits, queued export through the mounted app, no document/history mutation, snapshot retirement and cancellation after output allocation. Native and browser implementations should share the output contract; the end-user file workflow and two-client behavior remain acceptance requirements.

## Adjacent Layer Workflow Gap

Source inspection also found that the layer panel offered only Pixel and Group creation despite the existing Adjustment command and Inspector. A localized Add Adjustment row and neutral panel fixture were added, with updated virtualized window offsets and a native panel projection test. This improves access to existing non-destructive brightness/contrast editing. Native validation is pending; existing adjustment algorithm/oracle and semantic history evidence does not prove this new control is live.


## Follow-Up API Inspection

Confirmed the current Layout media factory requests a Migrated resumable HostOnly contract and directly returns an ArtifactReservedToolJob from build_media_export_job. Its result wrapper credits the schema bytes, transfers a sealed ArtifactOutputChunks queue into ArtifactMediaExportResult::structured, completes once, and closes inner resources before releasing the completion cell. The Raster snapshot close lease must be retained and checked with can_release before dropping its Arc; the framework owns the final bounded snapshot disposal.

The existing PNG encoder processes 4096 raw scanline bytes per advance with fixed-Huffman DEFLATE, PNG CRC and Adler validation. It has no TypeScript twin in the shared pixels module yet. A proper export implementation must account for that multiimplementation gap and test decoded PNG pixels against Sharp; no retained export implementation has been authored in this continuation. The existing Rust compositor preparation has a 1024-layer/32-depth descriptor cap and 32768-pixel copy grants.


## Retained Export Implementation In Progress

Added a HostOnly export-media:image:out factory and concrete retained job. Work is split into source preparation, composition, PNG encoding and base64 pages. It credits schema/output bytes, seals only a complete queue, reports localized stage previews and carries the snapshot close lease. New neutral vectors specify grants, stage labels, output schema and RGBA pixels; Rust/TypeScript page encoding shares those vectors, with Node Buffer and Sharp as independent test oracles. Initial TypeScript run failed due to absent implementation before production code was added. Native tests cover core stages/cancellation, output chunk caps and queued app completion/cancellation/history preservation. First validation runs are pending; no working runtime export claim yet.


Export TypeScript final run 2 passed 96 tests, including neutral progress schema, native-platform base64 oracle and Sharp PNG channels. Native run 1 failed before Raster compilation: shared framework table read-only cell used nonexistent Label::data. Replaced that single call with Label::try_from and the local table-window admission error, matching adjacent constructors. This is a compile prerequisite repair; no table feature change is claimed. Native export run 2 follows.

Progress now uses monotonically counted checkpoints (at stage boundaries and every 64 work units), because the media poll API exposes checkpoint progress rather than preview messages. Output queues/completion are shared with the framework operation; this job releases its handles during close and lets the framework drain after job retirement. Calling completion.close_take before the framework drain would block cancelled or superseded completed output, so that incorrect ordering was removed before native validation.


Current verification scope: shared neutral page/PNG/progress tests pass in TypeScript (96 total). Native export run 2 is still compiling after the framework Label prerequisite repair. The existing framework batch export adapter was inspected: registration of export-media:image:out makes PluginApp::export_media drive the same submit/poll API; UI/workflow entry points use externally addressable operations. Direct ArtifactEditor batch serialization remains for existing batch callers. Editable artifact-pack saving remains a separate unfinished bounded-export path. The complete image editing goal remains active.

## End-User Download Gap — 2026-09-28

Source tracing found that retained media export alone is not an editor download: the current media-out ABI aggregates the result, while shell file export saves the editable archive. A separate `exportPng` command is required. Its schema and a native end-to-end download-retirement/history test were authored before the implementation. It will reuse bounded export work and the framework's existing segmented download lane, with a private output queue that survives command retirement.

Activation 12 failed in unrelated shared OS infinite imports (`SceneViewportMask3d`, `World3dPresentation`, `World3dPresentationClear`); no browser validation was performed. Native export run 2 remains running.

## PNG Command Implementation Checkpoint

The editor now declares `exportPng` / `export-png`, its empty-argument schema, a separate resumable HostOnly factory and proof, EN/DE palette description, and an Export PNG row in the Layers panel. The command and media routes share the staged export engine. The command owns a private segmented output queue and hands it to `ArtifactDownloadOutput`; job retirement drains only untransferred output. Retained binary ingress validates the decoded command before rendering. Native tests cover the real command/download ACK/retirement/drain path, localized unlocked export on protected layers, and cancellation before and during work without history changes. These native tests are authored, not yet verified.

Raster TypeScript run 3 completed: **97 pass, 0 fail**. Native export run 2 failed before compiling Raster on shared stdio UI label type errors; those exact lines were already repaired by another contributor when inspected, so no stdio changes were made. Native export run 3 is now running against the new command.

## Remaining User-Facing Progress Gap

The command route makes export discoverable and downloads through the existing host lane, but source inspection does **not** prove user-facing progress/cancellation yet. `MountedTypedCommandFullOperation::drive_worker_step` currently retires checkpoint payloads without exposing their progress to the UI. `PluginRuntime.subscribeOperationProgress` carries UI dirty scopes for tool runs, not arbitrary command checkpoint data. `ToolCancellationHandle` is public on the native app, but no renderer command for cancelling this typed export was found. Backend cancellation tests alone therefore do not complete the end-user requirement. Add an owner-qualified framework progress/cancel surface (or integrate export into the existing tool-run lifecycle) and test the actual button and progress display. Do not label this requirement complete based on this command implementation.

### Progress Integration Investigation

`ToolRunDefinition` already supplies localized stages and counters, a nonmutating mode, a visible run panel and cancel lifecycle. Its job port carries host effects, but it has no segmented-download ownership API; merely emitting a download effect would bypass the existing typed completion/ACK retirement path. Conversely, typed command workers already retain their operation/generation/cancellation lease and sealed download publication, but `drive_worker_step` only retains outcomes for retirement and does not publish progress. A domain-neutral operation progress/cancel surface should preserve these exact ownership and publication checks. Avoid a Raster-only global progress registry or an unbounded whole-image effect.

Activation 13 was started after inspecting the formerly missing 3D types: current WGPU source now reexports `SceneViewportMask3d` and the presentation types through `component::ui`. No changes were needed in those shared files.

Native export run 3 reached Raster after the shared dependencies compiled. Production Raster compiled; the test target failed on two authored test names: the private action bridge was addressed through the outer module, and the factory test used an obsolete controller constant name. Tests now call the public `ArtifactEditor::command_from_action` and use `RASTER_PLAY_CONTROLLER_ID`. Native run 4 is active. Activation 13 has successfully built the shared framework surface WASM component and remains active. Neither is a runtime pass yet.

### Domain-Neutral UI Integration Path

A typed operation already has an exact instance, operation, generation and cancellation lease. A framework-owned cancellation action can follow `dispatch_tool_run_action`'s immediate routing before the typed worker queue, validating the addressed owner/generation before signalling the existing lease. The renderer already subscribes to `TypedOperationUiProgress` dirty scopes. An `ArtifactView` operation-status projection plus a shared accessible status/control renderer would let Raster display active export beside the Layers actions without inventing a plugin-global registry. Checkpoint counters may be observed generically, but arbitrary checkpoint state must not be assumed to be JSON progress. Preserve the private segmented download/ACK ownership path. Confirm that host cancellation can interleave with ongoing operation draining before calling the visible Cancel control complete. This is a researched integration path, not implemented behavior.

## Visible Operation Controls — Implementation in Progress

Created the domain-neutral `plugin/⏳️operation-progress` schema, neutral fixtures, Rust/TypeScript identity validation and localized status captions. Cancellation arguments are exact 16-digit lowercase hexadecimal operation/generation IDs, with no client-supplied instance authority. AJV validates the same neutral cases; `Intl.NumberFormat` checks full-width counter text. The observed TypeScript red was a missing implementation module (97 existing tests passed); the green run now passes **114 tests**.

The framework now projects checkpoint counters into `ArtifactView::operations()`, coalesces refresh scopes, and routes a framework Cancel action immediately to the matching local operation lease. It checks instance, actor and generation. Raster renders shared accessible status, indeterminate progress and Cancel controls above the Layers tree. Native integration coverage now reads the rendered control identity, checks EN/DE output, rejects stale generation, invokes Cancel, and checks that no PNG or history edit is published. This implementation is **not yet native or live verified**; completion/retirement refresh behavior and host interleaving still require verification.

Native export run 4 failed at test compilation because the new tests used the internal async crate without a direct dependency. Added `semio-framework-async` as a test-only workspace dependency. Production progress controls were authored while that run was active, so its result does not validate the new framework changes.

### Retirement Refresh and Host Cancellation Review

Corrected the shared helper implementation to retain the framework's generic space member parameter and expose helpers to the parent module. Added a coalesced retirement refresh witness, included it in pending/runnable checks, and drained it during close. This ensures an operation disappearing from the registry still owes a final UI refresh. Native export run 5 and the focused framework `operation_progress` test target are now active; no native pass is claimed.

The renderer's `drainTypedOperations` advances one serialized continuation, releases ingress authority, and yields between turns. Its source explicitly admits new commands through the same serialization boundary, which supports the intended Cancel interleaving. This is source evidence only; a mounted host test and live acceptance are still required. `consumeTypedOperationEffects` currently throws for a Fault result page, and generic lease cancellation currently publishes such a page. An explicit user cancellation therefore still needs a distinct graceful terminal outcome; suppressing arbitrary worker failures would be incorrect. This remains an open end-user requirement.

### Explicit Cancellation Outcome

Added a schema and four neutral cases for cancellation outcome classification: an explicit user stop without a worker failure emits an empty Terminal page; worker faults and other authority cancellations remain Fault pages. The immediate Cancel handler records user intent on the exact mounted operation before signalling its lease. The existing ACK and retirement path remains responsible for disposal. This avoids changing transport lane codes or discarding actual failures.

The TypeScript test first failed on the missing classification export (97 existing tests passed). After implementation, **118 tests pass, 0 fail**, including AJV checking both expected and incorrect terminal classifications. Native tests now also assert that the visible Cancel control produces a Terminal page, no Fault/download, and a final refresh after the last slot retires. Native builds began before this last implementation change; they must be evaluated against the source they actually compiled and rerun as necessary. Native and browser verification remain open.

A subsequent call-site audit found that the live render method did not actually call the authored progress projection helpers. Added the missing snapshot capture and `ArtifactView::with_operations` binding to the live render branch; override snapshots remain independent. Extended the visible-control test to reject a foreign actor and instance, accept repeated cancellation, and render the Cancelling state. The earlier description of UI integration was implementation intent, not a verified runtime result.

### Active Verification Checkpoint

The TypeScript cancellation green run is terminal: 118 pass, 0 fail. Native export run 5 (session 37190), framework operation-progress run 1 (session 38312), and activation 13 (session 18283) were still active at the last check. The framework native test target was compiling `semio_framework_plugin`; Raster was still in its native build stage. Recent shared-plugin compiler output contained no new error diagnostics, which is not a completed test or activation result. Continue these existing runs before launching replacements. The new framework neutral-test census is three; the Raster visible-control law additionally verifies final retirement refresh and graceful cancellation.

Framework progress run 1 subsequently failed at test compilation. Four existing typed-operation test constructors lacked the new progress and cancellation fields; all four now initialize them. Narrow additional compile repairs were needed in existing shared tests: the editable document fixture path had one extra parent segment; UiMap observations used removed `get` instead of its owned cursor; two builder `unwrap` calls required unavailable Debug implementations on their retained error owners. Updated those call sites without changing their assertions or production APIs. Framework progress run 2 (session 15188) is active. Raster run 5 and activation 13 remain active; neither was restarted.

## Native Export Run 5

The complete Raster run executed **310 tests: 307 passed, 3 failed, 0 skipped**. The PNG command download/ACK/retirement test, visible EN/DE progress control, stale/foreign cancellation authority rejection, repeated Cancel, graceful Terminal without Fault/download, final progress removal, and unchanged undo history all passed. These are native app/rendered-tree results, not live browser acceptance.

Both retained media tests failed at admission because Raster had not registered `build_snapshot_disposer`. Added the missing hook and a disposer that atomically relinquishes its Arc alias with `Arc::into_inner`; if it receives the final value, it delegates to Raster's existing bounded owned-snapshot retirement. It does not wait for uniqueness while retaining an alias that the live store/cache also owns. The editable archive fixture failure and correction are recorded separately. Native run 6 (session 84390) now checks these repairs with backtraces enabled. Framework progress run 2 is terminal with exit 0; its exact census is recorded from its log below.

Framework focused native verification: **3 passed, 852 filtered/skipped**, exit 0 (`raster-operation-progress-native-2.log`). These verify neutral cancellation identities, localized full-width counters and graceful-versus-fault outcome classification. The shared test harness compiled after the repairs, but this filtered run does not claim that all 855 framework tests executed.

Added a direct disposer ownership regression while run 6 was building dependencies: two aliases of a populated Raster snapshot retire independently, zero-item grants preserve ownership, every nonzero turn stays within one item/16 KiB, and the final alias must dispose nested owned maps without blocking on the earlier alias. The expected complete Raster census is now **311**; inspect run 6's actual census rather than assuming this late-added test executed.

Run 6 is terminal, exit 1, before test execution: the subsequently added layer-selection helper mixed JSON and DSL value types. That conversion is corrected, as recorded in the duplication notes. Run 7 (session 29929, `raster-export-native-7.log`) is now active and includes the export/archive repairs plus the selection laws; the current expected census is 313. Activation 13 remains live. Do not poll the terminal native run 6 again or count its compilation as runtime verification.

## Native Run 7 and Follow-up

Run 7 (handle 29929, terminal exit 1) executed **313 tests: 310 passed, 3 failed, zero skipped**. The snapshot disposer law and superseded-document media refusal now pass, as do the PNG download/progress/cancel laws. Remaining media test failure was polling after explicit cancellation: the framework immediately transfers that handle into its close registry, and subsequent polling is invalid. The test now asserts retired-handle refusal, unchanged history, and bounded final app closure with completed/cancelled owners. This corrected test is pending run 8; no production media contract was changed.

Other failures: archive fixture compared JSON integer tokens against schema-produced floating tokens before reaching archive reload; layer deletion left stale selection because Raster declared Flat membership. Both are addressed in source and awaiting run 8. Run 8 uses the existing full native target with no cache and no fail-fast.

Run 8 ended in shared kernel compilation before testing the Raster repairs. The remaining narrow borrowed Option projection was corrected; run 9 is pending. Raster TypeScript rerun `raster-selection-follow-ts-green-2.log` passed **124 tests**, zero failures (handle 43386 terminal exit 0). Activation 13 remains live at the last poll; its source predates current changes.

Run 9 also ended before Raster tests, with one ordered-map progress type mismatch. The exact type was corrected; run 10 (`raster-export-native-10.log`) is active. Native workflow repairs remain unverified until a completed run reaches them.

Latest checkpoint: run 10 is terminal before Raster tests because shared WGPU draw types lack an Authored material match arm. The last executed Raster census remains **310 passed / 3 failed / 313 total** from run 7. The topology, fixture-number and media-cancel test repairs have not yet reached execution. Shared frame math is independently green (126 TS, 36 native), Raster TS is 124 green. Activation 13 is live but sampled waiting on Cargo's prebuild lock; not verified successful.
