# Remaining App Configuration Ownership Inventory

This source inventory records the remaining editor app-config fields after the OS/utility cleanup. Names alone do not establish violations: each candidate requires a producer/consumer trace. The active migration lane covers Jack and nine audited display-camera families. A later independent audit must also trace Puzzle2D/3D/5D camera/LOD partitions, brush candidate results and generation-job status; the initial audit called Puzzle2D camera window-local without following its reconstruction from app config.

Confirmed Puzzle2D trace: `set-camera` updates `Puzzle2dActionCtx.scene.runtime`; `Puzzle2dPlayApp::scene_for` reconstructs the scene from one `Puzzle2dConfig`; the retained reducer returns configuration from that scene. `Puzzle2dConfig` declares cameraX/Y/Zoom, lodModeByPane, engagementInputByPane, brush candidate buffers and fill-job lifecycle fields. Exact ownership remains open until that reducer/publication trace is independently completed.

## 🎬️sequence / 🎬️sequence

`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`lastRunJson`, `orientation`, `camera`

## 🌍️gis / 🏔️gisterrain

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`cameraJson`

## 🌍️gis / 🗺️gismap

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`layerVisibility`, `cameraJson`, `renderMode`, `vectorStyle`, `lodMode`, `layerStrokeScale`

## 💡️reasoning / 🔌️wires

`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`dragNodeId`, `dragLastX`, `dragLastY`

## 💠️lowpoly / 💠️lowpoly

`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`activeObjectId`, `selectionMode`, `selectionIds`, `selectionTargetsMesh`, `selectionTargetsVertex`, `selectionTargetsEdge`, `selectionTargetsFace`, `selectionKeys`, `paintUtility`, `activePaintLayer`, `selectionMethod`, `selectionModeDefault`, `selectedObjectIds`, `hoveredObjectId`, `hoveredTargetObjectId`, `hoveredTargetMode`, `hoveredTargetId`, `utilityParamsJson`, `paintColorR`, `paintColorG`, `paintColorB`, `paintColorA`, `worldCameraPosition`, `worldCameraTarget`, `worldCameraFov`, `engagementInput`, `showEdges`, `sunEnabled`, `sunAzimuth`, `sunElevation`, `sunIntensity`, `sunColor`

## 🏗️fem / ◻️2d

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`resultSourceId`, `resultMode`, `resultModeIndex`, `camera`

## ➗️mathematical / ➗️equation

`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`camera`

## 🌀️procedural / 🌀️generation2d

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`camera`, `showMode`, `selectedGenerationId`, `generationPreviewText`

## 🎥️shooting / 🎥️shooting

`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`defaultShotFormat`, `defaultShotShape`, `defaultAssetFormat`, `selectedShotIds`, `centerModel`, `fitRevision`, `cameraDraftLabel`, `camera`

## 🏗️fem / 🧊️3d

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`resultSourceId`, `resultMode`, `resultModeIndex`, `camera`

## 🌿️vcs / 🌿️vcs

`✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`



## 🌀️procedural / 🧊️generation3d

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`lodMode`, `showMode`, `camera`, `previewCamera`, `sunJson`, `selectedGenerationId`, `generationPreviewText`, `previewEvalText`

## 📖️playbook / 📖️playbook

`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`contributionsJson`

## 🏛️architect / 🏛️program

`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`activeRegister`, `searchQuery`, `searchHistoryJson`, `activeReportJson`, `lastResultJson`, `lastAnalysisJson`, `adjacencyKindFilter`, `graphCameraX`, `graphCameraY`, `graphCameraZoom`

## 🧩️puzzle / ◻️2d

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`cameraX`, `cameraY`, `cameraZoom`, `lodModeByPane`, `engagementInputByPane`, `brushCandidateIndex`, `brushCandidates`, `brushCandidateSourceHandleId`, `fillCount`, `fillJobOperation`, `fillJobGeneration`, `fillJobSeed`, `fillJobBaseRevision`, `fillJobCheckpointSequence`, `fillJobAcceptedCount`, `fillJobSearchCount`, `fillJobStage`, `fillJobLifecycle`, `fillJobFaultCode`, `gridSnapEnabled`, `gridFactor`, `suggestionOffset`, `nodeKindWeights`, `handleKindWeights`, `exampleLoadGeneration`, `exampleLoadId`

## 🖍️draw / 🖍️drawing

`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`engagementInput`, `camera`, `tracePointerGeneration`, `tracePointerCompletedWork`, `tracePointerPendingWork`

## 🪐️space / 🏠️home

`✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`activePanelTab`, `directoryJson`, `directorySessionBindingSha256`, `directoryAuthorizationGeneration`, `directoryReceiptSha256`, `clientId`, `clientName`

## 🕸️dag / 🕸️dag

`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`cameraX`, `cameraY`, `cameraZoom`

## 🧩️puzzle / 🖐️5d

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`camera2d`, `camera3d`, `selection`, `selectionMethod`, `hoveredPartId`, `fillCount`, `brushCandidateIndex`, `overlapBudget`, `lodMode`, `suggestionOffset`, `gridSnapEnabled`, `gridFactor`, `engagementInputByWindow`, `objectKindWeights`, `vortexKindWeights`, `sun`

## 🎞️animate / 🎬️presentation

`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`engagementInput`

## 🧱️block / ◻️2d

`✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`



## ✒️writer / ✒️writer

`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`editorSelection`, `formatSignal`, `lintSignal`, `revision`, `editorSettings`, `engagementInput`, `camera`

## 🧩️puzzle / 🧊️3d

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`selection`, `selectionMethod`, `hoveredObjectId`, `hoveredVortexFullId`, `suggestionMenu`, `overlapBudget`, `fillCount`, `fillApplyGeneration`, `fillAppliedCount`, `fillCheckpoint`, `brushCandidateIndex`, `objectKindWeights`, `vortexKindWeights`, `lodAutomatic`, `lodDepthVariable`, `gridVisible`, `lodManual`, `gridSnapEnabled`, `gridSpacing`, `selectableKinds`, `hoveredKindId`, `engagementInput`, `selectionModeDefault`, `proximityRadius`, `chunkSize`, `voxelDims`, `transformMove`, `transformRotate`, `vortexShow`, `vortexDirection`, `sun`, `camera`, `windowOptions`, `activeToolId`, `windowIds`

## 🧱️block / 🖐️5d

`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`



## 📋️forms / 📋️forms

`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`currentStepIndex`, `tryValues`, `contributionsJson`

## 📏️layout / 📏️layout

`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`activePageId`, `selectedIds`, `hoveredId`, `dropPreview`, `engagementInput`, `camera`, `previewCamera`

## 🧱️block / 🧊️3d

`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`activeRepresentationId`, `wantedTags`, `windows`, `brushVortexKindId`, `brushRadius`, `brushFlip`, `camera`

## 🗒️note / 🗒️note

`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`engagementInput`, `camera`

## 📐️cad / 📐️cad

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`selectedObjectIds`, `selectedNodeIds`, `selectionMethod`, `hoveredObjectId`, `hoveredTarget`, `activeObjectId`, `componentSelection`, `engagementInput`, `engagementStep`, `activeExampleId`, `selectedReferenceModelDefinitionId`, `selectedReferenceId`, `selectedPrimitiveId`, `selectedPrimitiveKind`, `engagementPane`, `engagementSessionJson`, `engagementPreviewOperationJson`, `engagementPreviewGeneration`, `lastFinalizedInteractionId`, `sun`, `camera`, `cameraBuilding`, `cameraEnergy`, `cameraStructureClassic`, `dislocateShape`, `dislocateBuilding`, `dislocateEnergy`, `dislocateStructureClassic`, `contributionsJson`

## 🪵️sourcing / 🗂️curation

`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`filters`, `contributionsJson`

## 📜️imperative / 📜️procedure

`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`runOutputJson`, `contributionsJson`

## 🏭️process / 🧊️process3d

`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`engagementInput`, `cameraPosition`, `cameraTarget`, `cameraFov`, `sunEnabled`, `sunAzimuth`, `sunElevation`, `sunIntensity`, `sunColor`, `contributionsJson`

## 🌊️flow / 🌊️flow

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`previewOffNodeIds`, `camera`, `lodMode`, `proximityDistance`, `gridVisible`, `gridSnapEnabled`, `gridFactor`, `catalogueSectionsJson`, `automationEnabledJson`, `contributionsJson`, `generationJson`, `duplicateWidgetProgressJson`

## 🖨️raster / 🖨️raster

`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`brushSize`, `brushOpacity`, `compositeViewport`, `camera`

## 📸️remodel / 📸️remodeling

`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`camera`, `layers`, `frameCursor`, `reportTable`

## 🔱️trinity / 🔌️jack

`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`

`camera`, `jackQuery`, `lodModeByWindow`

