# Runtime Integration Audit

2026-10-28. Read-only source inspection; no heavy command or source edit.

## Command and Publication Roster

No active BatchOnlyPendingRewrite entry remains in the authored layout app declaration. The occurrences are explanatory comments, including stale pointer test comments. All 20 standard layout commands are Migrated, present in LAYOUT_RETAINED_TOOL_IDS, the bounded proof catalogue and the publication contracts. All six exports are Migrated and have their separate LayoutExportJobFactory proof and HostOnly contracts. Media export also has its own HostOnly contract. No missing retained publication roster was identified.

The lane split is consistent with the work code: addPage publishes Artifact plus WindowConfig; canvasDrop has Artifact, WindowConfig and WindowTransient; transforms have Artifact plus WindowTransient; pointer handlers and engagement submit publish host actions; camera and active-page changes use WindowConfig. FocusPreflightIssue also emits a selection action through the framework while its optional page update uses WindowConfig. Do not change classifications or lane declarations speculatively while waiting for actual test failures.

## Import Boundary

The reserved import-media job accepts only fields:in and rejects any other port before decoding. The layout I/O declaration advertises JSON/SVG/PDF/STEP/IDML/DXF/DWG format support separately from that typed data-input port; fields:in is form.dictionary. Thus invoking reserved import-media with an SVG/PDF document is the wrong test route. Natural artifact reopening uses the framework artifact:native route, and interchange formats must use converter/opening services. This audit did not find a new missing reserved import builder.

## Concrete Reopening Route

The shell already implements four user actions with exact IDs: exportArtifactDocument, importArtifactDocument, saveArtifactFile and openArtifactFile. The input funnel handles them before dispatching guest actions, via documentTransferRef. These are framework routes, distinct from layout exportPdf/exportSvg/exportPng.

- exportArtifactDocument downloads a .semio-archive containing the document archive through readAppDocumentArchive and encodeDocumentArchiveBytes.
- importArtifactDocument opens a picker accepting .semio-archive and the declared archive MIME type, decodes the pack, creates a fresh same-app instance, loads it through loadAppDocumentArchive, and focuses the new spawned instance. Register Playwright filechooser before clicking; provide the previously downloaded archive through setFiles. Assert exact marker/frame IDs in the newly focused instance.
- saveArtifactFile calls exportAppMedia(instanceId, artifact:native), checks its descriptor against naturalFileFormatV1 and downloads the bytes. openArtifactFile picks that format, reads in bounded cancellable chunks, creates a fresh instance and invokes importAppMedia with artifact:native plus binary format_kind. Use this only when the selected app provides a native file format.

Look for action.exportArtifactDocument/action.importArtifactDocument/action.saveArtifactFile/action.openArtifactFile in the resolved action rail or framework command palette; availability must be observed in the actual app. These zero-argument framework actions may execute on row click. A guest layout importer refusing artifact:native is not enough to conclude this framework natural-file route fails, because framework native codecs own that route.

Layout supplies its own Artifact tab, so workbench-left-tabs returns those plugin tabs early and omits the fallback artifact.openWith sections. Do not wait for fallback Open With rows in this layout tab. Existing role-switch/switchPluginApp documentSource path transfers the current archive and sets documentInitialized=true, preserving the document while changing editor/viewer. It requires the same plugin and a current source session.

## Demo Boot Versus Persistence

Initial example dispatch is enabled only for non-host sessions that are not documentInitialized. It selects the registered/default example and sends buildActiveExampleAction; selecting playground.navbar.fixture explicitly also dispatches the example. A new uninitialized playground instance is therefore a demonstration seed, not evidence of durable reopening. The concrete archive download/import workflow above proves persistence independently of playground automatic seeding. Keep a same-context reload check as a separate persistence witness.

One source-level readiness defect is actionable if it explains a live false-ready result: useInitialExampleReadiness ends with .then(finish, finish), so rejected example loading OR rejected view publication marks the instance ready just like success. The callback has no failure state. A narrow repair would retain failed readiness and publish the rejection through the owning shell fault path, with a test asserting rejected start/publish cannot become ready. It is shared shell infrastructure; do not change it without a failing layout boot witness or coordinated owner coverage.

## Next Task-Scoped Checks

1. Run current retained tests rather than rewriting complete command rosters.
2. Extend live probe with exportArtifactDocument/importArtifactDocument archive round-trip using an exact added marker, then verify SVG/PNG/PDF independently. This uses an implemented shell path and avoids inventing a JSON save action.
3. If the live app is marked ready while missing the demo, capture the rejected example/publication and use it to drive the readiness fix above.
4. Preserve framework download/transfer notices as assertions: no-document, exporter/loader-unavailable, import-unreadable and file-open-failed must fail the round-trip even though the input funnel initially returns applied.
