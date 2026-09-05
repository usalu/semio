# React Shell Rendering Path Investigation

**Date:** 2026-09-05  
**Ticket:** 26/09/03/PROCEDURAL-3D-END-TO-END  
**Focus:** generation3d app windows (NodeGraph + World3dHost)

---

## 1. Empty State Conditions & Non-Empty Data Shapes

### NodeGraphHost
**Empty Condition:** Line 1145 in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`
```typescript
if (!scene) return <div className="semio-node-graph-empty">{emptySceneLabel}</div>;
```

**Required Data Shape** (type `NodeGraphScene`):
- `nodes` (array): `readonly NodeGraphNodeRecord[]` — node definitions with id, instanceId, label, position (x, y), dimensions (width, height), ports (inputs/outputs with id, label, resourceKind)
- `edges` (array): `readonly NodeGraphEdgeRecord[]` — edge definitions with id, source/target node ids, source/target port ids
- `viewport`: `NodeGraphViewport` — camera {x, y, zoom}
- `editable`: boolean
- `findItems`: `readonly NodeGraphFindItem[]` — searchable items
- `presencePeersJson`: JSON string of `PresencePeer[]`
- `capabilitiesJson`: string (e.g., `'{"engine":"flow"}'` for flow variant)
- `fixtureJson`: JSON string (when set, routes to `FlowGraphCanvasHost`)
- `statusJson`: optional status data
- `highlighted`: optional array of node ids for highlighting

### World3dHost
**Empty Condition:** Line 5055 in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
```typescript
if (!scene) return <div className="semio-world-3d-empty">{emptySceneLabel}</div>;
```

**Required Data Shape** (type `World3dScene`):
- `cameraJson`: JSON string of `WorldCameraRecord` {position: [x, y, z], target: [x, y, z], zoom, projection}
- `meshesJson`: JSON string of `WorldMeshRecord[]` — mesh definitions with id, data (positions, normals, indices, colors, uvs), or url
- `instancesJson`: JSON string of `WorldInstanceRecord[]` — rendered instances with id, meshId, position/rotation/scale, selected/hovered/highlighted/disabled flags, objectKind, revealIndex
- `selectionJson`: JSON string of `WorldSelectionRecord` — selection state with ids, hoveredId, targets (mesh/vertex/edge/face), transformation mode, gumball config
- `interactionJson`: JSON string of `WorldInteractionRecord` — interaction state with activeUtility, vortexes, suggestion menu, fill build progress, reveal cutoffs
- `lodJson`: JSON string of `WorldLodRecord` — LOD settings
- `vortexesJson`: JSON string of `WorldVortexRecord[]` — interaction points
- `attractionsJson`: JSON string of `WorldAttractionRecord[]` — visual guides
- `targetVolumesJson`: JSON string of `WorldTargetVolumeRecord[]` — interaction volumes
- `referencesJson`: JSON string of `WorldReferenceRecord[]` — background references
- `brushPreviewJson`: JSON string of `WorldBrushPreviewRecord` — brush preview
- `pointCloudsJson`: JSON string of `WorldPointCloudLayerRecord[]` — point clouds
- `environmentJson`: JSON string of `WorldEnvironmentRecord` — lighting/materials
- `frameJson`: JSON string of `WorldFrameRecord` — frame settings
- `fitJson`: JSON string of `WorldFitRecord` — auto-fit flags
- `domainId`: interaction domain (when non-empty, enables plugin-owned actions)
- `domainGranularityId`: granularity level for interaction

---

## 2. Data Flow: Plugin → ShellHost → Render Props

### Flow Path

1. **Plugin Effect Loop** → `applyHostEffects` (line 3129 in ShellHost)
   - Plugin returns `requestedEffects` in action responses or `refreshUi` responses

2. **RefreshUI Request/Response Cycle** (line 2683 in ShellHost)
   - `plugin.refreshUi(instanceId, request)` is called with UiRefreshRequest
   - Returns `PluginUiRefreshResponse` containing:
     - `windows`: array of `PluginUiRefreshSectionResponse` (BuiltNode trees)
     - `panels`: array of `PluginUiRefreshSectionResponse`
     - `requestedEffects`: optional array of Effects to execute
     - `engagements`, `measures`, `tools`, `labels` sections

3. **UI Response Cache** (line 2794 in ShellHost)
   - `applyUiRefreshResponseToCache(cache, response)` stores windows/panels as BuiltNode structures
   - Call: `applyUiRefreshResponseToCache` from `../🛠️ShellHelpers/🟦️.tsx` line 3776

4. **Window Rendering** → React Components
   - `InterpretedUiNode` (from `🗣️Interpreter/🟦️.tsx`) interprets BuiltNode trees
   - For each window with `componentScene` type and `nodeGraph` or `world3d` component:
     - Extracts the scene data from the BuiltNode props
     - Renders `<NodeGraphHost>` or `<World3dHost>` with `{ node: { nodeGraph: scene } }` or `{ node: { world3d: scene } }`

5. **Surface Scene Access in Hosts**
   - `NodeGraphHost` reads: `const scene = node.nodeGraph;` (line 1109)
   - `World3dHost` reads: `const scene = node.world3d;` (line 3868)
   - Scene data flows directly from the BuiltNode props constructed by the plugin

### Pending Effects → Scene Updates

**Key Line in RefreshUI loop (line 2796 in ShellHost):**
```typescript
if (response.requestedEffects?.length) await applyHostEffects(response.requestedEffects, nextSession);
```

- `refreshUi` response can trigger `requestedEffects` which update the session state
- Next `refreshUi` cycle includes updated surface scenes in the window BuiltNodes
- Surface data (nodeGraph/world3d) is included in each window's UI node props, re-rendered on refresh

---

## 3. Temporary Debug Hooks

**Search for `[DEBUG]` in TS/TSX files across the renderer:**

### ShellHost (🏛️ShellHost/🟦️.tsx)

| Line | Context | Code |
|------|---------|------|
| 710 | tutorial blob asset | `console.warn("[DEBUG] tutorial blob asset src not resolvable in this scope", src.hash);` |
| 1084 | extension invocation | `console.log("[DEBUG] extension invocation completed", { extensionId, capability, instanceId, req, status: "ok" in outcome ? "ok" : "fault" });` |
| 1239 | history snapshot fail | `console.error("[DEBUG] history snapshot failed", error);` |
| **1909** | **establishPrimarySession catch** | **`if (bytes) console.error("[DEBUG] boot fault text", new TextDecoder().decode(Uint8Array.from(bytes)));`** ← MUST REMOVE |
| 1959 | hot-swap event | `console.log(`[DEBUG] hot-swap ${pluginId}`, hotSwapEvent);` |
| 1981 | hot-swap dropped instances | `[DEBUG] hot-swap ${pluginId} dropped ${dropped.length} spawned instance(s)` |
| 2007 | hot-swap rollback | `console.warn(`[DEBUG] hot-swap rolled back for ${pluginId}`, error);` |
| 2029 | host plugin refusal | `console.warn(`[DEBUG] refusing to uninstall the host/primary plugin: ${pluginId}`);` |
| 2033 | active session plugin refusal | `console.warn(`[DEBUG] refusing to uninstall the active session's plugin: ${pluginId}`);` |
| 2095 | space extension ledger dispatch | `console.log("[DEBUG] space extension ledger op dispatched", { action, args });` |
| 2097 | space extension ledger skip | `console.warn("[DEBUG] space extension ledger op skipped", action, error instanceof Error ? error.message : String(error));` |
| 2125 | extension store install ok | `console.log("[DEBUG] extension store install ok", result);` |
| 2127 | extension store unavailable | `console.warn("[DEBUG] extension store unavailable or install failed; falling back to catalog id heuristic", error instanceof Error ? error.message : String(error));` |
| 2134 | installExtension resolve error | `console.warn("[DEBUG] installExtension could not resolve moduleUrl", resolveError);` |
| 2204 | install from file ok | `console.log("[DEBUG] extension store install from file ok", { file: file.name, ...result });` |
| 2206 | installExtensionFromFile fail | `console.warn("[DEBUG] installExtensionFromFile failed", error instanceof Error ? error.message : String(error));` |
| 2307 | setExtensionEnabled | `console.log("[DEBUG] setExtensionEnabled", { extensionId, enabled });` |
| 3006 | render fail | `console.error("[DEBUG] render failed", renderError);` |
| 3026 | spawned render fail | `console.error("[DEBUG] spawned render failed", renderError);` |
| 3089 | spawned program sync fail | `console.error("[DEBUG] spawned program document sync failed", syncError);` |
| 3184 | loadDocument pack/spr | `console.log("[DEBUG] loadDocument pack/spr for instance", baseSession.instanceId, "pack", packBytes.length, "spr", sprBytes.length);` |
| 3325 | invokeExtension dispatch fail | `console.error("[DEBUG] invokeExtension dispatch failed", { extensionId, capability, req, error });` |
| 3355 | openPluginInstance focused spawned | `console.log("[DEBUG] openPluginInstance focused spawned app", { ... });` |
| 3409 | applyShellUri reentrancy | `console.error(`[DEBUG] applyShellUri: reentrant call blocked at depth ${applyShellUriDepthRef.current}, ...` |
| 3478 | applyShellUri openSpace | `console.log("[DEBUG] applyShellUri openSpace", spaceId);` |
| 3503 | shell uri apply fail | `console.error("[DEBUG] shell uri apply failed", uriError);` |
| 3690 | recovery diagnostics | `console.log("[DEBUG] recovery diagnostics", { pluginId, supervisor: ... });` |
| 3784 | setActiveUtility fail | `console.error("[DEBUG] setActiveUtility failed", utilityError);` |
| 3811 | setActiveTool fail | `console.error("[DEBUG] setActiveTool failed", toolError);` |
| 3916 | undeclared action | `console.warn("[DEBUG] skipping undeclared action", action.action, targetSession.app.id);` |
| 3948 | action fail | `console.error("[DEBUG] action failed", action.action, action.args, actionError);` |
| 4055 | tutorial sandbox snapshot fail | `console.error("[DEBUG] tutorial sandbox snapshot failed", snapshotError);` |
| 4064 | tutorial base document load fail | `console.error("[DEBUG] tutorial base document load failed", loadError);` |
| 4080 | tutorial sandbox restore fail | `console.error("[DEBUG] tutorial sandbox restore failed", restoreError);` |
| 4206 | tutorial rebuild | `console.log("[DEBUG] tutorial rebuild", { atMs: clamped });` |
| 4283 | tutorial recording validation fail | `console.error("[DEBUG] tutorial recording validation failed", validationError);` |
| 4285 | tutorial recording | `console.log("[DEBUG] tutorial recording", json);` |
| 4672 | AppRouter.build fail | `console.error("[DEBUG] AppRouter.build failed", buildError);` |
| 4732 | setDefaultApp fail | `console.error("[DEBUG] setDefaultApp failed", commandError);` |
| 4743 | clearDefaultApp fail | `console.error("[DEBUG] clearDefaultApp failed", commandError);` |
| 4760 | setMergePolicy fail | `console.error("[DEBUG] setMergePolicy failed", commandError);` |
| 4781 | resolveConflict fail | `console.error("[DEBUG] resolveConflict failed", commandError);` |
| 4801 | openArtifactWithAppRef not found | `console.error(`[DEBUG] openArtifactWithAppRef: ${target.pluginId}/${target.appId} not found after install`);` |
| 4804 | openArtifact fail | `console.error("[DEBUG] openArtifact failed", commandError);` |
| 4896 | readConflicts fail | `console.error("[DEBUG] readConflicts failed", commandError);` |
| 5672 | touchSpaceIndexArtifact fail | `console.error("[DEBUG] touchSpaceIndexArtifact failed", touchError);` |

### NodeGraphHost (🕸️NodeGraph/🟦️.tsx)

| Line | Context | Code |
|------|---------|------|
| 640 | WasmGraphSurface sync fail | `console.warn("[DEBUG] WasmGraphSurface sync failed", error instanceof Error ? error.message : String(error));` |
| 652 | WasmGraphSurface ready sync fail | `console.warn("[DEBUG] WasmGraphSurface ready sync failed", error instanceof Error ? error.message : String(error));` |

### World3dHost (🌐️World3dHost/🟦️.tsx)

| Line | Context | Code |
|------|---------|------|
| 3926 | world3d viewport reattached | `console.log("[DEBUG] world3d viewport reattached to scene camera", { surfaceId: node.surfaceId, sceneCameraJson });` |
| 4106 | world3d viewport detached | `console.log("[DEBUG] world3d viewport detached from shared scene camera", { surfaceId: node.surfaceId });` |

---

## 4. Storybook Stories

### NodeGraphHost Stories
**File:** `.storybook/stories/framework/hosts/NodeGraphHost.stories.tsx`

| Story | Fixture Data | Route |
|-------|--------------|-------|
| **Workflow** | Two nodes connected via one edge; WASM DAG engine (`GraphSession`); `wasm: ["node-graph"]` | `WasmGraphSurface` (real dag-engine) |
| **FlowGraph** | `capabilitiesJson: '{"engine":"flow"}'` + `fixtureJson` (flow fixture with slider/neuron/preview); `wasm: ["flow"]` | `FlowGraphCanvasHost` (real flow-engine) |

**Fixture Details:**
- `WORKFLOW_SCENE`: nodes with ports (inputs/outputs), edges, viewport, editable=true, findItems
- `FLOW_FIXTURE_JSON`: schema="flow.fixture", widgets (inputSlider, neuron, outputPreview), synapses

### World3dHost Stories
**File:** `.storybook/stories/framework/hosts/World3dHost.stories.tsx`

| Story | Fixture Data | Route |
|-------|--------------|-------|
| **MinimalViewport** | Empty meshes/instances, camera={}, interaction={activeUtility:"select"}; no WASM | `WorldCanvas` (pure r3f, no terrain layer) |
| **TerrainViewport** | Same as Minimal + `terrainJson` (tile template, project origin, exaggeration); `wasm: ["terrain"]` | `WorldTerrainLayer` (real `TerrainSession`) |

---

## 5. Test Coverage & Execution

### Playwright E2E Tests

**File:** `.storybook/framework-hosts-wasm.spec.ts` (lines 41–51, 89–96)

| Test | Story ID | Assertion |
|------|----------|-----------|
| NodeGraphHost workflow | `🛠️framework🔌️hosts-nodegraphhost--workflow` | `.semio-node-graph-host` visible, no errors |
| NodeGraphHost flow graph | `🛠️framework🔌️hosts-nodegraphhost--flow-graph` | `.semio-node-graph-host` visible, no errors |
| World3dHost minimal | `🛠️framework🔌️hosts-world3dhost--minimal-viewport` | Clean boot, no WASM needed |
| World3dHost terrain | `🛠️framework🔌️hosts-world3dhost--terrain-viewport` | Clean boot, terrain layer loads |

**Run Commands:**
```bash
# Run Storybook build + playwright tests
nx run @semio-tech/framework-renderer-react:playwright

# Or run storybook dev + tests separately  
nx run storybook:dev
nx run storybook:test  # (if target exists)
```

### Unit/Integration Tests

**File:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🔬️index.test.ts`

- Tests `NodeGraphHost` and `World3dHost` with fixture data matching Storybook stories
- Exercises scene data parsing, viewport updates, and host rendering conditions
- Search target: look for test names containing "accepts" or "scene" around line 3676+

---

## Summary

**NodeGraphHost Rendering:**
- Empty when `!scene`; renders "ui.host.emptyScene" label
- Non-empty requires JSON-serialized nodes/edges + viewport
- Data source: BuiltNode props from `refreshUi` response windows
- Two engine paths: DAG (WasmGraphSurface) or Flow (FlowGraphCanvasHost)

**World3dHost Rendering:**
- Empty when `!scene`; renders "ui.host.emptyScene" label  
- Non-empty requires meshes/instances + camera JSON + interaction state
- Data source: BuiltNode props from `refreshUi` response windows
- Optional terrain layer loads separate TerrainSession WASM engine

**Data Flow:**
- Plugin → `requestedEffects` → `applyHostEffects` → `refreshUi` → window BuiltNodes → host scene props

**[DEBUG] Hooks to Remove:**
- **Line 1909 (ShellHost):** `[DEBUG] boot fault text` in establishPrimarySession catch — CRITICAL
- Lines 640, 652 (NodeGraphHost): WasmGraphSurface sync debug logs
- Lines 3926, 4106 (World3dHost): viewport attachment debug logs
- All others are informational and can stay or be removed per review

**Test Targets:**
- Storybook: `.storybook/framework-hosts-wasm.spec.ts` (playwright)
- Unit tests: `index.test.ts` in react package (covers both hosts)
