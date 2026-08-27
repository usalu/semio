# Remaining App Command Cohorts

## Source Snapshot, Not Runtime Coverage

The coordinator grouped the exact `remainingCommands` array from `📊️member-mounted-source-full-census-r5-2026-08-27.json`, rather than estimating from filenames or summary counts. The source snapshot has 773 command rows, 351 source-admitted rows, 315 BatchOnly rows, two forbidden rows, and 269 remaining live registrations. These are overlapping projections, not disjoint categories to sum into a percentage. BatchOnly is not an interactive implementation and does not satisfy the user's all-app completion requirement.

| Plugin / Artifact | Remaining live registrations |
| --- | ---: |
| 📸️remodel / 📸️remodel | 39 |
| 🎥️shooting / 🎥️shooting | 37 |
| 🧩️puzzle / ◻2d | 37 |
| 📋️forms / 📋️forms | 29 |
| 🖍️draw / 🖍️draw | 26 |
| 🌀️procedural / 🧊️procedural3d | 23 |
| 🧱️block / 🧊️3d | 23 |
| 🏛️architect / 🏛️program | 21 |
| 🖨️raster / 🖨️raster | 16 |
| 🪐️space / 🪐️space | 14 |
| 💠️lowpoly / 💠️lowpoly | 4 |
| Total | 269 |

## Exact Registered Commands

### 📸️remodel / 📸️remodel

`addGcp`, `addStream`, `advanceReconstruction`, `calibrateCameras`, `cancelReconstruction`, `clearDense`, `clearGeoProducts`, `clearMeshResult`, `clearResult`, `clearSparse`, `clearTracks`, `editCalibration`, `exportQcReport`, `importFramePayload`, `importVideoBytesPayload`, `importVideoDone`, `importVideoFramePayload`, `placeGcpObservation`, `removeGcp`, `removeStream`, `resetPlaceholderMesh`, `retryStage`, `runReconstruction`, `runStage`, `setActiveUtility`, `setCamera`, `setDenseParams`, `setFeatureParams`, `setFrameCursor`, `setGeoParams`, `setIngestParams`, `setLayerVisibility`, `setLocale`, `setMatchParams`, `setMeshParams`, `setMotionParams`, `setReportTable`, `setSfmParams`, `setStreamSync`.

Source reasons: 39 × no explicit factory, bounded-first-step proof, or fail-closed disposition.

### 🎥️shooting / 🎥️shooting

`addAsset`, `addShot`, `exportActiveShot`, `exportAllShots`, `importAsset`, `importSnapshotJson`, `loadSavedCamera`, `patchAssets`, `patchShots`, `resetFixture`, `rotateSelection`, `saveCamera`, `saveDownload`, `scaleSelection`, `setActiveAsset`, `setActiveExample`, `setActiveShot`, `setActiveShotFormat`, `setActiveShotLabel`, `setActiveShotShape`, `setActiveUtility`, `setAmbientIntensity`, `setCamera`, `setCameraDraftLabel`, `setCenterModel`, `setLocale`, `setMaterialRoughness`, `setShadowEnabled`, `setShotCamera`, `setShotSelection`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `toggleSun`, `translateSelection`, `worldPointerDown`, `worldPointerMove`.

Source reasons: 37 × no explicit factory, bounded-first-step proof, or fail-closed disposition.

### 🧩️puzzle / ◻2d

`applyBoardEvents`, `brushCancelSlot`, `brushCommitSlot`, `brushCycleCandidate`, `brushFillSessionAdopt`, `brushFillSessionBegin`, `brushFillSessionCancel`, `brushFillSessionClear`, `brushFillSessionDiscard`, `brushFillSessionRetry`, `brushFillSessionStep`, `brushOpenSlot`, `brushSetCandidateIndex`, `deleteSelection`, `duplicateSelection`, `engagementAbort`, `engagementControlSelect`, `engagementInput`, `engagementSubmit`, `focusSelection`, `lodScaleJson`, `patchInspectorNodes`, `redrawHandles`, `reorganize`, `selectSameKind`, `setActiveExampleStep`, `setBrushKindWeights`, `setBrushNodeSize`, `setCamera`, `setFillCount`, `setGridFactor`, `setGridSnapEnabled`, `setLocale`, `setLodModeForPane`, `setSelectionFlag`, `setSuggestionOffset`, `setTerminology`.

Source reasons: 37 × no explicit factory, bounded-first-step proof, or fail-closed disposition.

### 📋️forms / 📋️forms

`addQuestion`, `addQuestionOption`, `addStep`, `addVectorField`, `dropQuestionKind`, `exportFixture`, `moveQuestion`, `moveStep`, `nextStep`, `patchQuestionOptions`, `patchQuestions`, `patchStep`, `patchVectorField`, `previousStep`, `removeQuestion`, `removeQuestionOption`, `removeStep`, `removeVectorField`, `resetTry`, `setActiveExample`, `setContributions`, `setLocale`, `setSpecJson`, `setTryValue`, `setTryValues`, `setTryValueStep`, `submit`, `updateForm`, `updatePlaybook`.

Source reasons: 27 × no explicit factory, bounded-first-step proof, or fail-closed disposition; 2 × classification proof exists but no exact registered app-owned retained reducer factory and builder.

### 🖍️draw / 🖍️draw

`addLayer`, `canvasCommitDraft`, `canvasDoubleClick`, `canvasEscape`, `canvasPointerDown`, `canvasPointerMove`, `canvasPointerUp`, `combineBoolean`, `commitDocument`, `deleteLayer`, `dropLayerKind`, `duplicateLayer`, `engagementInput`, `engagementSubmit`, `moveLayer`, `patchLayer`, `patchLayers`, `setActiveExample`, `setActiveUtility`, `setCamera`, `setCameraZoom`, `setFixtureJson`, `setLocale`, `setSelectedOpacity`, `setSnapshot`, `toggleLayerVisible`.

Source reasons: 20 × no explicit factory, bounded-first-step proof, or fail-closed disposition; 6 × app-owned retained reducer lacks an exact publishable completion-lane contract or preparation owner.

### 🌀️procedural / 🧊️procedural3d

`addWidget`, `deleteSelection`, `flowEvalTick`, `graphPointerDown`, `moveMediaNode`, `nodeGraphViewport`, `patchFlowWidgets`, `removeWidget`, `reorganize`, `rotateSelection`, `scaleSelection`, `setActiveExample`, `setActiveUtility`, `setCamera`, `setLocale`, `setLodMode`, `setShowMode`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `toggleSun`, `translateSelection`, `worldPointerDown`.

Source reasons: 23 × classification proof exists but no exact registered app-owned retained reducer factory and builder.

### 🧱️block / 🧊️3d

`addRepresentation`, `addVortex`, `addVortexKind`, `edit`, `patchObjectKind`, `patchRepresentation`, `removeRepresentation`, `removeVortex`, `removeVortexKind`, `setActiveExample`, `setActiveRepresentation`, `setActiveUtility`, `setBrushFlip`, `setBrushRadius`, `setBrushVortexKind`, `setCamera`, `setWindowArrangement`, `setWindowRepresentations`, `setWindowSpacing`, `toggleWindowRepresentation`, `worldSurfaceHover`, `worldSurfaceLeave`, `worldSurfacePlace`.

Source reasons: 23 × no explicit factory, bounded-first-step proof, or fail-closed disposition.

### 🏛️architect / 🏛️program

`addElement`, `addRegisterItem`, `applyTemplate`, `exportProgram`, `exportRegistersCsv`, `importProgram`, `importProgramRequest`, `importRegistersCsv`, `nodeGraphEdit`, `nodeGraphViewport`, `patchRegisterItem`, `removeElement`, `removeRegisterItem`, `runAnalysis`, `runReport`, `runValidation`, `search`, `selectRegister`, `setAdjacencyField`, `setAdjacencyFilter`, `setAdjacencyKind`.

Source reasons: 21 × no explicit factory, bounded-first-step proof, or fail-closed disposition.

### 🖨️raster / 🖨️raster

`addLayer`, `deleteLayer`, `dropLayerKind`, `duplicateLayer`, `moveLayer`, `patchLayer`, `patchLayers`, `setActiveUtility`, `setBrushOpacity`, `setBrushSize`, `setCamera`, `setCameraZoom`, `setCompositeViewport`, `setLayerVisible`, `setLocale`, `toggleLayerVisible`.

Source reasons: 16 × no explicit factory, bounded-first-step proof, or fail-closed disposition.

### 🪐️space / 🪐️space

`copyInviteLink`, `createArtifact`, `deleteArtifact`, `foldDirectoryEvents`, `inviteMember`, `openArtifact`, `openArtifactWith`, `presenceHeartbeat`, `removeMember`, `renameArtifact`, `requestDeleteArtifact`, `requestInviteMember`, `setVisibility`, `touchArtifact`.

Source reasons: 14 × no explicit factory, bounded-first-step proof, or fail-closed disposition.

### 💠️lowpoly / 💠️lowpoly

`paintStrokeBegin`, `paintStrokeEnd`, `setActiveUtility`, `transformBegin`.

Source reasons: 4 × app-owned retained reducer lacks an exact publishable completion-lane contract or preparation owner.

## Shared Gates Still Preceding Admission

- typed command preparation lacks a fixed-width event-maintained immutable child-content root and no-default terminal-witnessed old-root retirement authority.
- Jack `.spr`/`.ops` envelope caller lacks the shared retained edit decoder, exact child retirement, fixed-page ingress, initializer recovery, cancellation, or exact completion acknowledgement.
- Trinity Rewrite envelope caller lacks the Jack-owned fixed-page operation store, generation handle, bounded progress/cancel/close, exact rejected-page handback, or completion acknowledgement.
- child snapshot retirement domain cohorts or callsites do not match the exact machine-readable owner inventory.
- instance close still permits implicit nested payload destruction or lacks saturation-safe bounded cleanup job ownership.
- bounded command worker does not enforce decoded, work, step-time, and output contract limits.
- 16 process-global payload store candidate(s) require operation-owned state or an explicit static exemption.
- 6 app-owned retained route(s) lack an exact nonempty publication-lane contract.
- 4 app-owned retained route(s) declare Store publication lanes without their exact app-owned preparation authority.
- 35 app-owned import-media route(s) remain fail-closed pending explicit resumable factories.
- 269 live command registration(s) remain fail-closed; see remainingCommands ledger.

After current ownership/close/renderer foundations, executor waves must cover these concrete app families and all fail-closed imports, then re-run the source census and actual registered/native/Wasm/browser gates. No blanket classification change or generic whole-operation adapter can count as implementation. The source census alone cannot establish clean activation; the real CAD constructor found a command-vs-action declaration mismatch despite an admitted source proof.

Read-only peer checkpoint: HEAD `a8d1caf41f`, 2026-08-27T11:04:49+02:00; disk 102 GiB available at this audit. No cleanup or git mutation was performed.

