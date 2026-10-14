# Media Two-Stage Test Migration Plan

Read-only proposed test-only migration, 2026-10-13. No builds/edits.

## Actual Types And Custody

Installed first stage is `owner.receive_nested::<Media, Result<Option<MediaArtifact>,MediaError>>`. Exporter refusal is actual **MediaError**, not MediaArtifactError. Current four fixtures carry MediaArtifactError::Export(MediaError::Payload(...)); move its inner MediaError into the actual first-stage output. Do not clone port/message or flatten to ValueError text.

Test helper should return `Result<MediaFaultProjection, MediaValueError>` for its fault-only scenario. On first-stage `Ok(Err(error))`, place `Some(MediaArtifactError::Export(error))` in the caller's original Option, then call existing `receive_media_artifact_fault(source,owner)`. The Option loan remains alive for stage2 refusal. On first-stage outer Err, return exact original ValueError; error payload now lives in native retained output when exporter ran, so do not fabricate a caller source or try to recover error by consuming recipient. For unexpected successful MediaArtifact output, preserve typed ownership or fail outside measured operation with explicit cleanup; this helper's callback contract should be closed to intended exporter-fault cases.

Stage1 output move price and close/publication are real installed semantics. Subsequent fault projection is existing actual source-retaining producer; helper is test scaffolding mirroring those stages, not new production codec/legacy compatibility.

## Denied Currency Expectations

Items0/capacity0/depth0 fail actual frame pre-admission: exporterCalls0, zero births/releases, caller source pointer retained.

Copy0/release0 can create original native frame, invoke exporter once and own returned MediaError in output; publication/header or frame close refuses. Expect outer error, caller source None when consumed, native recipient present and source capacity retained. No requirement exporterCalls0/heap0 at these later frontiers. Compare allocator actual births/releases to full original body receipt, not guessed frame bytes. Successful callback call count must be asserted against actual stage reached, with neutral fixture cases authored separately.

If stage1 succeeded but stage2 rejected, original Option again holds MediaArtifactError until fault projection moves it; assert source Some or original recipient, exact surviving pointer, no detached error. These states must not be collapsed into a boolean claiming all errors remain in caller Option.

## Preserve Original Grants Through Cleanup

Keep immutable denied owner grant and full body receipt. Do not replace owner with a new owner over the same native controller to reset progress/grant and retry consumed input. Denied copy/release may have spent real capacity/items and retained source; same trial must close that original native recipient with separately authored finite close grant.

Snapshot owner body record_progress checks original body grant. Recording funded cleanup releases into a body owner with original release0 will exceed that grant; instead retain body receipt, end owner loan, call original native.close_retirement_recipient(closeGrant), aggregate close receipts separately under close policy, preserving native cumulative ledger. Zero close plus funded step receipts/actual System bytes must conserve body frame births + original source String capacities + all cleanup births. Remaining body grant is not funded close policy when denied axis0.

After original failed trial terminal cleanup, success trial needs fresh independently owned source/error and fresh original controller/recipient under authored success grant. This is a separate law input, not a reset of failed ownership. If source never moved prebirth, it can be reused with a separately scoped successful trial, but preserve original failure receipt and explicit distinction.

Cancellation trials keep same original observer Cell false through denied funded close, then rearm that exact Cell. Existing fault projections close via output.close_step(owner) under successful original grant; body and accepted fault-output cleanup receipts must remain cumulative. Callback cuts may shift because actual two-stage producer invokes different observer events; author cuts from independent declared frontier, not assumed old helper ordinal.

No weakening of installed semantics: the deleted helper's all-axis prebirth behavior is absent from current production. Revised neutral laws should qualify true frame/output/projection stages and all five currency refusals rather than preserve fictitious earlier callback denial. Exact pointer and source backing conservation remain mandatory.
