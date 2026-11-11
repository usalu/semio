# Verify wave 2

## instance-render: check --all-targets 0 errors; test: 0 tests, ok (render2-check4.log, render2-test1.log)
- projections: impl block now bounds `X: InstanceExtension` (import added).
- tool-machine: `set_tool_clock_ms` is gated on `feature = "artifact-app-testing"` only (a dependent crate's `cfg(test)` does not enable the tool-machine field); five retained-work closures annotated `Ok::<_,OriginalTurnPrimary>`.
- reserved-jobs: the async `build_artifact_reserved_action_job` / `build_artifact_reserved_media_job` are removed (they had no step context). Replaced by stepped members of the render bundle driving the resident `ReservedPreparation` (its `wire_construction` admission, retired by `close_slot`/`close_step`):
  - `advance_reserved_action(action,args,decoded_items,meta,maximum,cx)` (action wire, interaction read, snapshot read, wire admission, request assembly, builder) -> `true` once the job is built.
  - `handoff_reserved_producer(&mut output,cx)` -> moves (job, completion, wire admission) out under one funded step.
- jobs `ReservedPreparation::advance_action_wire`: returned early on `raw_wire.is_some()` before `sourceClose`, so `action_returned` was never set; now done only when both are set.

## Callers migrated (uncompiled)
- `dispatch_import_media(port,&mut Option<Media>,meta,cx) -> Result<Option<InvocationResult>,OriginalTurnFailure>`: `None` = pending, re-poll next funded step. It keeps the producer permit in the resident `media.pending_reserved_producer_permit` (retired by the docs lease ladder), drives `advance_reserved_media`, then `handoff_reserved_producer`, then `dispatch_reserved_original_producer`; the preparation stays in `pending.reserved_preparation` (retired by the existing `owns_birth`/`close_slot` ladder).
- One-shot callers (`import_media`, `consume_media` x2) go through `import_media_funded`, which re-polls while the single step stays funded and otherwise returns a fault.
- Clipboard route (`dispatch_framework_reserved_action`) uses the same resident permit (`CommandCtx.media`), `advance_reserved_action` and handoff, returning `Ok(None)` while pending.

## Downstream: blocked
`check -p semio-plugin-instance-app -p semio-plugin-surface-fixture -p semio-plugin --all-targets` (render2-down1..3.log) is refused by source admission before compiling: `semio-framework-job-context` `📦️owner/🦀️.rs` 3702 bytes vs 3442 remaining in the crate boundary (another agent is editing that crate, `🛬️owned/🦀️.rs` modified 21:38). infinite-board-normal-fill, raster, writer, text-splice, window-transient not run (same dependency). app-owned-media-error not run.

## Update
Builds now pass the source gate but stop in `semio-framework-job-native-turn` (job-context split in progress: `OriginalSourceCloseContext`/`OriginalSourceCloseProducer` missing, `settle_original_work` missing; log render2-callers1.log). The migrated callers, instance-app and the remaining downstream targets are not yet compiled.

## Update 2: one-shot media API migrated (uncompiled)
`import_media_funded` removed. Pending-aware trait shapes (`PluginApp`, `⚛️reactor/🔌️app`):
- `import_media(&mut self, port, media:&mut Option<Media>, meta, cx) -> Result<Option<InvocationResult>, OriginalTurnFailure>`
- `consume_media(&mut self, port, artifact:&mut Option<MediaArtifact>, cx) -> Result<Option<MediaConsumption>, MediaArtifactOperationFailure>`
`None` = pending, re-polled next funded turn. Implementors: default trait bodies and `VcsArtifactApp` (the only overrides). The instance's intrinsic/natural branches decode/take the media only when `reserved_media_due()` (header staged, source not yet donated), restoring natural bytes if not donated.
Callers: runtime `plugin_consume_media` (new `MediaImport::Pending`, replaces `Admitting`; restores resident `data` when pending; `Original` failures propagate, `Domain` refuses), exchange `arm_media_in`; test callers (16 files, ~45 sites) now use new fixture drivers `semio_plugin::artifact_app_laws::{import_media, consume_media}` which re-poll while the fixture scope stays funded.
Not migrated (already stale before, no `cx`): `target.consume_media` in layout, `PluginApp::*_media` in cad and plugin-runtime-contract parts 5/6.
Known risk: a donation poll refused for funding drops the locally decoded `Media`.
Compile status: still blocked in `semio-framework-job-native-turn` by the job-context split (render2-callers2.log).

## Update 3
- Data-loss risk fixed without a second resident slot: `reserved_media_due(cx)` now also requires the donation to be funded this step (shared `media_donation_funded`, the same check `prepare_reserved_media_input` uses) and that the wire receiver is vacant, so the media is decoded/taken only on a poll whose donation cannot be refused. Nothing decoded is ever left unowned. The natural-file restore code was removed. No refused-then-funded test was added: it needs a RenderCtx plus an unfunded step context fixture, which cannot be built or run while the job-context split is uncompilable.
- Stale call sites now thread a fixture scope `cx` through the drivers: layout `intrinsic` (also matches `MediaArtifactOperationFailure::Domain`), cad `geometry-media-and-owners` (`import_media`), runtime-contract parts 5 and 6 (`consume_media`, foreign-schema refusal destructured from `Domain`). Other stale calls in those same tests (no-cx `produce_media`, `close_registered_fixture_app`) are untouched.
- Compile: still blocked, now in `semio-framework-job-context` itself (`📦️return/🎟️place` unresolved `crate::source`, render2-callers3.log), the split in progress.

## Update 4 (after the split compiled; render2-q1.log)
`check -p semio-plugin-instance-dispatch -p ...commands -p ...app --all-targets` now compiles up to dispatch, which had never been migrated: 19 errors, none in my edits.
Fixed (mechanical, uncompiled): five `Ok::<_,OriginalTurnPrimary>` annotations (producers:57, host-events:134, emit/original:49, runtime-command/close:4, host-event-routing:22); decoder identity `(NativeAllocationIdentity,u64)` in `SourceScope`, instance root `original_command_scope`/`original_host_event_scope`, typed-command `original_decoder`; removed `retired_key` (typed-command:70); `(JsonBorrowError,_)` destructuring (typed-command:176, producers:289).
Remaining in dispatch:
- `mount_original_reserved_owners` (producers:265..272) and typed-command:247..288 build `MountedTypedCommandFullOperation` the old one-shot way: missing `reserved_construction`, `captured_child_content` must be a `SharedOriginalStorage<ChildContentView>` (stepped `capture_child_content`).
- typed-command:318, 388: `close_step`/`step` now take `&mut RetainedAllocationEffect`, not a grant.
Design consequence for my media/clipboard work: `dispatch_reserved_original_producer` still ends in that old one-shot mount. The stepped path (`admit_original_reserved` phase 3 in `advance_original_reserved_media`, taking the resident permit, `prepared.job`, `prepared.completion`) is the new mounting; `dispatch_import_media` and the clipboard route should end in it instead of `mount_original_reserved_owners`. Not done.
Not run: refused-then-funded test, remaining no-cx calls, check/test of commands, instance-app, facade, surface-fixture, infinite-board-normal-fill, editors (all downstream of dispatch).
