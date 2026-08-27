# CAD, Space Engine, and Shooting Route Audit

## Method and status

Static source audit only; Cargo, Nx, compiler, and official verifier were not run. `B` below means a route is structurally bounded once wire/payload admission is capped; `R` means it currently iterates, parses, decodes, serializes, renders, or carries an unbounded batch and needs an operation-owned reducer with a persisted microcursor. A current no-op is called out separately: its present CPU cost is not proof of its intended semantics.

Baseline ledger: CAD 40, Space engine 40, Shooting 39 = **119** rows. Exact owner files are:

- `CadPlayApp`: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `SpaceApp`: `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs`
- `ShootingPlayApp`: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`

None of these three packets is in the official 53-row scan-then-monolith ledger.

## CAD — source delta and invalid coverage claim

The official 40-row remaining ledger is stale: current source marks 39 routes `Migrated` (`setContributions` plus 38 of the 40 baseline IDs); only `setLocale` and `setTerminology` remain unmarked. However, this is **not 39 routes of actual factory coverage**:

- `bounded_first_step_tool_proofs!` lists those same 39 IDs at component lines 1162–1176.
- The registered `CadHostConfigurationJobFactory` owns only `CAD_HOST_CONFIGURATION_TOOL_IDS = ["setContributions"]` (lines 1062–1063), and `build_tool_job` returns `None` for every other ID (line 1187 onward).
- Therefore the 38 remaining annotated CAD IDs have a proof/registration **factory gap**. The macro's advertised `BoundedFirstStepCommandJobFactory` is not a registered factory for them.
- The official canonical `import-media` row remains `CadPlayApp / failClosedPendingFactory`; `importCadFile` is additionally annotated but has no matching factory. No official process-global record overlaps CAD.

### CAD action classification and executor boundaries

| Boundary | IDs | Required treatment |
|---|---|---|
| CAD-B1: single-item/config/gesture | `addNode`, `addObject`, `deleteObject`, `duplicateObject`, `patchObject`, `renameNode`, `patchCadPlayReference`, `engagementAbort`, `engagementInput`, `engagementPossibleSelect`, `engagementRepeatLast`, `engagementSubmit`, `focusModelDefinition`, `setActiveExample`, `worldPointerDown`, `worldPointerMove`, `setActiveUtility`, `setCamera`, `setDislocateOption`, `setLocale`, `setNodeSelection`, `setProjection`, `setProjectionParam`, `setReferenceSelection`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `setTerminology`, `toggleSun`, `loadRawRequest`, `setContributions` | B. One typed command/fixed fixture/request effect. `setContributions` is the only route current factory code actually owns. Several object/transform-adjacent routes are documented no-ops pending child dispatch; that does not authorize an unbounded implementation later. |
| CAD-R1: selected/batch model mutation | `patchSelection`, `translateSelection`, `rotateSelection`, `scaleSelection`, `applyTransformation` | R. The first four accept or derive an arbitrary selection; `applyTransformation` delegates model-derived mutations. Persist `{operation, generation, selection/transform cursor, base revision}` and emit bounded mutation pages. |
| CAD-R2: import/export codec | `importCadFile`, `saveSelected`, `saveInPlay`, `saveCurrent` | R. `importCadFile` parses arbitrary JSON/data and can reset a full snapshot; saves clone/export a document/model. The file-open `loadRawRequest` stays B; its returned payload starts R2. Canonical `import-media` must use the same operation-owned payload path, not a process store. |

## Space engine — no job proof, real global blind spot

All 40 baseline IDs remain without `action_interactive_job(Migrated)`, a bounded-first-step proof, a `ToolJobFactory`, or `build_tool_job` implementation in the current engine component.

The official importer ledger has no `SpaceApp` entry because its route is named `importMedia`, not the canonical artifact `import-media`. That is not a clearance: `importMediaPayload` base64-decodes and imports arbitrary bytes, while `importSpacePackPayload` decodes a full pack. Both need resumable operation state.

The official process-global ledger also has no Space record and no Space exemption. Current source nevertheless declares `static REGISTRY: OnceLock<Arc<Mutex<HashMap<...>>>>` in `shared_presence_peers` (lines 190–191). It is a cross-app mutable presence cache outside the verifier's reported candidate set. It must either be moved behind host/presence operation ownership or receive a deliberate static-exemption decision; it cannot be used for import/export checkpoints.

### Space action classification and executor boundaries

| Boundary | IDs | Required treatment |
|---|---|---|
| Space-B1: one-command graph/config/shell request | `addParameter`, `bindParameterField`, `closeFocusedInstance`, `compiledDagEngagementInput`, `compiledDagEngagementSubmit`, `connectMediaPorts`, `disconnectMediaEdge`, `goHome`, `importMedia`, `importSpacePack`, `moveMediaNode`, `navigateVirtualFileSystemNode`, `nodeGraphViewport`, `openInstance`, `openSpace`, `patchParameter`, `presenceHeartbeat`, `removeAppInstance`, `removeParameter`, `renameAppInstance`, `setActiveExample`, `setActivePanelTab`, `spawnApp`, `unbindParameterField`, `workflowEngagementInput`, `workflowEngagementSubmit` | B after bounded text/record admission. `importMedia` and `importSpacePack` only request file selection; they must not decode or retain file bytes. |
| Space-R1: selection/batch graph edits | `copyAppInstance`, `deleteSelection`, `duplicateAppInstance`, `pasteAppInstance`, `patchAppInstances`, `patchMediaNodes`, `reorganizeWorkflow` | R. Each stores or emits an unbounded list. `reorganizeWorkflow` explicitly maps every graph node on empty selection; duplicate/paste performs a graph lookup per source ID. Persist the source-ID list or index and cursor, emit pages, and respect cancellation. |
| Space-R2: opaque graph document batch | `nodeGraphEdit`, `setAppRegistrations` | R. `nodeGraphEdit` parses an unbounded JSON operations array and can apply a whole fixture; `setAppRegistrations` forwards arbitrary JSON into host registration. Use typed bounded pages / a cursor rather than one opaque JSON reducer. |
| Space-R3: media/pack codec and export | `exportMedia`, `exportStudioDsl`, `exportStudioPack`, `importMediaPayload`, `importSpacePackPayload` | R. These materialize embedded documents, transform media, encode/decode base64, or serialize a whole space pack. Persist only opaque operation-owned handles/checkpoints; never retain bytes in `REGISTRY`. |

## Shooting — no migration proof, importer and global overlap

All 39 baseline IDs are still unmarked: no migrated registration, tool proof, factory, factory registration, or `build_tool_job` is present in `ShootingPlayApp`'s component. The official ledger additionally records:

- Canonical `import-media`: `ShootingPlayApp`, `failClosedPendingFactory`.
- Process global: `SHOOTING_EMBLEM_SCRATCH`, `thread_local RefCell<HashMap<String, SemioImageSnapshot>>`, classified `child-content-scratch` at `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️component.rs:518`.

That scratch store must not be reused as importer payload retention or export/render job state.

### Shooting action classification and executor boundaries

| Boundary | IDs | Required treatment |
|---|---|---|
| Shooting-B1: singleton settings/selection/gesture/request | `addAsset`, `addShot`, `loadRequest`, `loadSavedCamera`, `saveCamera`, `setActiveAsset`, `setActiveExample`, `setActiveShot`, `setActiveShotFormat`, `setActiveShotLabel`, `setActiveShotShape`, `setActiveUtility`, `setAmbientIntensity`, `setCamera`, `setCameraDraftLabel`, `setCenterModel`, `setLocale`, `setMaterialRoughness`, `setShadowEnabled`, `setShotCamera`, `setShotSelection`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `toggleSun`, `worldPointerDown`, `worldPointerMove`, `importAssetRequest`, `resetFixture` | B, subject to fixed argument and fixed-fixture caps. `importAssetRequest` is only a file-open effect. |
| Shooting-R1: selected/batch asset and shot mutations | `patchAssets`, `patchShots`, `translateSelection`, `rotateSelection`, `scaleSelection` | R. `patchAssets`/`patchShots` explicitly map ID vectors; transforms forward arbitrary `asset_ids` to aggregate mutations. Persist IDs + cursor and produce bounded mutation pages. |
| Shooting-R2: file/document/image work | `importAsset`, `importSnapshotJson`, `saveDownload`, `exportActiveShot`, `exportAllShots` | R. Imports carry arbitrary data/snapshots; download serializes a document; export creates an item per shot and render request. `exportActiveShot` is a one-shot route but its rendering/materialization still needs a bounded operation boundary; `exportAllShots` additionally needs a shot cursor. |

## Recommended dispatch order

1. **CAD correctness packet**: make factory registration exactly match the claimed tool list before adding coverage; either register B1 routes with real bounded work or remove their false migrated/proof declarations. Split R1/R2 into typed resumable factories. This packet is source-disjoint from the other two.
2. **Space operation-kernel packet**: introduce B1, R1, R2, and R3 factory families; first retire direct full-pack/media work and adjudicate the hidden presence `REGISTRY`.
3. **Shooting media packet**: create B1 and R1/R2 factories, close canonical `import-media`, and keep `SHOOTING_EMBLEM_SCRATCH` limited to its child-content role or eliminate it.

These packets cover all **119** baseline rows; the route totals are CAD **31 B + 9 R**, Space **26 B + 14 R**, and Shooting **29 B + 10 R**.
