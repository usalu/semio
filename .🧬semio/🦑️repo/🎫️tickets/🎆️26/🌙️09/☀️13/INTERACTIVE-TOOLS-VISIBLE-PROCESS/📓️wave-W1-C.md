# 🧹️ Wave W1-C: React host cleanup

Lane W1-C of `📋️tool-run-contract.md` (§3.6 tried-ring row, §4, §5 W1-C row, wave-1 "also required").

**Status: landed, green.**

Paths below are relative to the repo root.

- `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`
- `WH` = `E/🧱️elements/🌐️World3dHost/🟦️.tsx`

## 1. What changed

### 1.1 Deleted from `WH` (no fill code is left in the host)

A case-insensitive `rg fill` over `WH` now only matches paint fields such as `fill:` and `hover-interactive-fill`.

**Types and wire caps**

- `WorldFillBuildRecord`, and the `WorldInteractionRecord.fillBuild` field.
- `WorldFillTriedRecord` and `WorldFillDiagnosticRecord`.
- `WorldBrushPreviewRecord.fillBuildPreview`.
- `WORLD_FILL_STATUS_LABEL_MAX_BYTES`, `WORLD_FILL_COLOR_MAX_BYTES`, `WORLD_FILL_PREVIEW_JSON_MAX_BYTES`, `WORLD_FILL_TRIED_MAX`.
- The key sets `WORLD_FILL_{ROOT,DIAGNOSTIC,GHOST,TRIED}_KEYS`, `WORLD_BRUSH_VERDICTS` and `censusAllowedOwnKeys`.

**`fillBuildPreview` parsing**

- `parseWorldBrushPreview` (`WH:1582`) is now a plain JSON object parse. It lost the fill census, the 16 KiB cap and the tried-ring validation.
- `WorldBrushVerdict` (`WH:319`) is narrowed to what the brush publishes: `"testing" | "free" | "collision"`. The `rejected` and `accepted` verdicts were fill-only.

**Fill identity ref and fill mode**

- `latestFillIdentityRef` and the `fillDiagnostic` freshness fold.
- `fillMode`. `visibleBrushPreview` is now just `brushPreview`, and `volumeLayersInteractive` no longer checks fill.

**Ghost rendering**

- `FillTriedGhosts`, `WORLD_FILL_TRIED_FAINTEST`, and `BrushPreviewGhost`'s `opacityScale` prop, which only the tried ring used.
- `FillDiagnosticOverlay` and its `data-fill-*` probe surface. The ToolRun panel and `data-tool-run-*` replace them.

**Fill tick loop** (the `fillBuildTick` command is deleted by W1-B)

- `worldFillBuildShouldTick` and `worldFillBuildHostTickAllowed`.
- The `fillBuildTick` interval effect and `leftoverArmedToolId`.
- The now-unused imports `isolatedJobDriveIsActive` and `takeIsolatedJobUiPoll`.

**Picking and leftover overlay**

- `leftoverOverlayArmedUtilityV1`, which mapped `activeToolId === "fill"` to `"fill"`. `mergeWorldInteractionWithLeftoverV1` now reads `leftover?.activeUtility` (`WH:1417`).
- `worldInstancePickBlocked` (`WH:1622`) no longer lists `"fill"`. It blocks `brush`, `volumeBrush` and `surfaceBrush`.

**Comments**

- Comments that named fill (projection fit, the suggestion tick backlog, the blockPick prop) were made tool-neutral.

**Kept for the brush hover ghost until wave 2C**

- `WorldBrushPreviewRecord`, `WorldBrushVerdict` and `parseWorldBrushPreview`.
- `retainWorldBrushPreviewJsonV1`, `brushObjectPlacementArgs` and `brushPreviewGhostMeshUrl`.
- `brushGhostPaint` and `BrushPreviewGhost`.
- The `MeshStyleKind` `"danger"` arm, `MESH_STYLE_PAINT.danger` (`WH:397`) and `resolveMeshStyle`'s `danger` input. `brushGhostPaint` paints a `collision` brush ghost with it. Its comment now names the brush, not fill.

There were no other fill-specific `MeshStyleKind` arms. `provisional` is W0-E's generic token and stays.

### 1.2 Trace layer mounted for every World3d window

- **Store hook.** `World3dHost` owns one store per window: `const toolRunTrace = useToolRunTraceStore(scene?.toolRunTrace)` (`WH:5128`), declared before the empty-scene return.
- **Probe attributes.** The host div spreads `toolRunTraceDataAttributes(toolRunTrace.store)` (`WH:6651`), which publishes `data-tool-run-{run,generation,page,records,testing,success,warning,danger}`.
- **Mount point.** `<WorldToolRunTrace lane={scene.toolRunTrace} meshes={meshes} trace={toolRunTrace} />` sits in the R3F scene beside the brush ghost (`WH:6847`).
- **`WorldToolRunTrace`** (`WH:2130`, new):
  - `geometryForMesh(index)` resolves `meshesJson[index]`. Inline `data` goes through `geometryFromMesh`, and those geometries are disposed on change. A GLB `url` mesh is used once it has loaded; until then the geometry is `null`, which draws the layer's unit box.
  - GLBs are loaded only for meshes that a live `instance3d` batch names.
  - It mounts `ToolRunTraceLayer` with the host-owned store.
- **`ToolRunTraceGlbGeometry`** (`WH:2108`, new):
  - Loads the GLB with `useLoader(GLTFLoader)` and welds it into one indexed `BufferGeometry` via the existing `extractGlbCollisionMesh`. That geometry already carries the GLB frame rotation, then gets vertex normals.
  - Hands the geometry to the layer and disposes it on unmount.

### 1.3 Provisional mesh style

- The instance view already paints `provisional` when either `useToolRunProvisional(instance)` (the `ToolRunProvisionalIdsContext`) or `instance.provisional === true` is set. W0-E landed that; this lane left it unchanged.
- **Defined prop path.** W0-D exposes the set only in Rust, as `ArtifactView::tool_run().provisional_entities: BTreeSet<u64>`. It reaches no scene or TS surface yet, so this lane defines the path:
  - **Primary.** The producer of `World3dScene.instancesJson` stamps `"provisional": true` on every instance record whose entity id is in `view.tool_run()?.provisional_entities`. The same applies to `instancesDeltaJson.changed[]`. `WorldInstanceRecord.provisional` already exists, and `advanceWorldInstanceResidency` carries record fields through unchanged.
  - **Secondary.** An outer shell may provide `ToolRunProvisionalIdsContext` with the ids as strings (`String(entity)`). They match `instance.id` or `instance.interactionId`.
  - On wgpu, the same set fills `World3dState.provisional_instance_ids`.
- The new mount test covers this: a `provisional: true` instance paints exactly one dashed `LineDashedMaterial` outline.

## 2. Public API as landed

```ts
// E/🧱️elements/🌐️World3dHost/🟦️.tsx
export function parseWorldBrushPreview(brushPreviewJson: string | undefined): WorldBrushPreviewRecord | null; // plain object parse
export function worldInstancePickBlocked(activeUtility: string | undefined): boolean;                         // brush | volumeBrush | surfaceBrush
export function mergeWorldInteractionWithLeftoverV1(base: WorldInteractionRecord, leftover: LeftoverWorldSelectionOverlayV1 | null): WorldInteractionRecord;
// removed exports: worldFillBuildShouldTick, worldFillBuildHostTickAllowed, leftoverOverlayArmedUtilityV1
// internal: function WorldToolRunTrace({ lane, meshes, trace }), function ToolRunTraceGlbGeometry({ url, onGeometry })
```

The re-exports in `E/🎯️targets/⚛️react/🟦️.tsx` (import and export lines of the World3dHost region) dropped `worldFillBuildShouldTick` and `worldFillBuildHostTickAllowed`.

## 3. Tests

### 3.1 New language-neutral fixture

`E/🧱️elements/🌐️World3dHost/🧫️fixtures/⏯️tool-run-trace-mount.json` holds:

- ledger identity and capacity;
- two inline meshes, a quad and a triangle;
- one committed instance and one provisional instance;
- two steps of trace ops, covering all four verdicts across both meshes, one `entity` subject, a verdict change `testing → success`, a retire and a new `testing` record.

Each step lists the expected live `(mesh, verdict, count)` batches and the `data-tool-run-*` counters.

### 3.2 New suite

`E/🧱️elements/🌐️World3dHost/🧪️tests/🧩️component/🟦️.tsx` has one test: `mounts one instanced mesh per (mesh, verdict) from the scene's toolRunTrace lane and publishes its counters`.

- **Mount.** `World3dHost` is mounted for real in jsdom. The WebGL seam is mocked as in `🖱️world3d-interaction`, and `useFrame` callbacks are run by the test.
- **Delivery.** The TS `ToolRunTraceStore` ledger is the producer. After each step, `base64url(encodeToolRunTraceDelta(deltaAfter(cursor)))` is rerendered as `toolRunTrace`, and the cursor is echoed.
- **Recording.** `three`'s `InstancedMesh` and `LineDashedMaterial` are subclassed to record every live mesh and every dashed outline.
- **Assertions.**
  - Every live instanced mesh carries exactly one `(mesh, verdict)`.
  - Every instance matrix equals the three.js `Matrix4.compose` oracle of a resident ledger record.
  - Each batch geometry has the vertex count of `meshesJson[mesh]`.
  - The live batch counts equal the fixture.
  - The DOM counters equal the fixture, and `data-tool-run-page` equals `delta.next`.
  - Provisional outlines equal `provisionalOutlines`.
- **Registration.** The suite is registered in `E/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` as `elementSuite("🌐️World3dHost", "🧩️component", "tsx")`.
- **Mutation check.** Making `geometryForMesh` ignore the index (`inline[0]`) failed the test with `a batch must draw through meshesJson[mesh]: expected 4 to be 3`. The change was reverted.

### 3.3 Updated existing World3dHost tests (`E/🧪️tests/🔬️engine-contract/🟦️.ts`)

**Deleted**

- `admits only the fixed localized fill diagnostic schema`.
- `parses the tried-candidate ring and the per-candidate verdicts, and refuses an unbounded one`.
- `reads tested · locked / requested, the verdict and the stall reason off the fill overlay probe attributes`.
- `renders the %s fill label with visible and ARIA parity`.
- `an unstarted fill plan … still schedules fillBuildTick while fill is armed`.
- `skips fillBuildTick while an Isolated job is driving unless a UI poll is due`.

**Rewritten**

- `blocks instance picking for brush and volume brush engagements but not move` now also covers `surfaceBrush` and no longer covers `fill`.
- `leftover fill tool overlays guest select so fillBuildTick still arms` became `carries an armed mode tool id across hover leftovers without overriding the published utility`.
- `every self-gating world lane dispatches through the awaitable twin` lost its `fillBuildTick` sites.
- The background-tick comment in `gates on exactly what run returns` is now tool-neutral.
- The imports of the removed symbols were dropped.

### 3.4 Runs

All runs were in the foreground. Logs are in `T/🗑️generated/W1-C/`.

| Command (cwd `E/🎯️targets/⚛️react/📦️packages/🟦️typescript`) | Result |
|---|---|
| `bun ./📜️script.ts test long "🌐️World3dHost/🧪️tests"` | 1 file, 1 test passed (`host-mount-test-1.txt`). jsdom logs a `HTMLCanvasElement.getContext` "not implemented" error from `WorldTerrainLayer`; it is noise, and the test still passes. |
| same, with the mutation applied | 1 failed, as intended (`host-mount-mutation-1.txt`) |
| `bun ./📜️script.ts test long "🌐️World3dHost" "world3d-interaction" "world3d-instance-delta" "world3d-pick-bounds"` | 5 files, 42 tests passed (`world3d-suites-1.txt`). This includes W0-E's trace suite. |
| `bun ./📜️script.ts test long "engine-contract"` | 1 file, 605 tests passed (`engine-contract-1.txt`) |
| `bunx tsc --noEmit -p tsconfig.json` | 0 new errors (`react-tsc-1.txt`). `WH` still has the same 4 unrelated errors as W0-E's `react-tsc-2.txt`: `leftoverSelectIdsMustNameHoverPickV1`, two `addEventListener` overloads and a module-level `addEventListener`. Engine-contract has 11, as before. The react index has none. |
| `bunx tsc --noEmit -p T/🗑️generated/W1-C/tsconfig.host-test.json` (extends the package tsconfig, includes the new suite and `WH`) | The suite has 0 errors; `--listFilesOnly` confirms it is included. `WH` has only the same 4 errors (`host-test-tsc-1.txt`). |

## 4. Commands to register in launch.json

- `bun ./📜️script.ts test long "🌐️World3dHost/🧪️tests"` in `E/🎯️targets/⚛️react/📦️packages/🟦️typescript`. This is the focused form. The suite is also part of `bun nx run @semio-tech/framework-renderer-react:test-long`.

## 5. Deviations from the contract, with reasons

1. **Provisional set source.** W0-D has not exposed the provisional entity set outside Rust, so this lane defines the path (§1.3) and mounts no provider of its own. A provider fed from nothing would be dead code, and a TS-only scene field would break schema-first.
2. **GLB geometry for trace batches.** The contract only says "InstancedMesh per mesh×verdict". A GLB-backed mesh draws the layer's unit box until the GLB loads. The welded geometry is not subdivided per GLB material.
3. **`worldInstancePickBlocked` no longer blocks during fill.** Under the tool-run contract, provisional instances are excluded from picking by style (`instancePickEnabled && !provisional`). Blocking every pick while a tool id is armed would be fill-specific host code.
4. **Legend toggles and cursor echo are not wired.** No window-config visibility source and no view-state `toolRunTraceCursor` echo exist on the React side yet. The layer uses `TOOL_RUN_TRACE_VISIBLE_ALL`, and the cursor is observable through `data-tool-run-page`.

## 6. Foreign edits

| File | Change |
|---|---|
| `E/🧪️tests/🔬️engine-contract/🟦️.ts` | The World3dHost tests listed in §3.3 |
| `E/🎯️targets/⚛️react/🟦️.tsx` | Two removed names on the World3dHost import and export lines |
| `E/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | One `elementSuite` line |

## 7. Open items

**Producers (W0-D / W1-B)**

- Stamp `"provisional": true` on instance records from `ArtifactView::tool_run().provisional_entities`, or provide `ToolRunProvisionalIdsContext` from the shell.
- Fill `World3dState.provisional_instance_ids` on wgpu.

**Stale fill references outside `WH` (not owned here)**

- `E/🧱️elements/🛠️ShellHelpers/🟦️.tsx:296` still lists `"fillBuildTick"` among background tick actions, and `:2874` has a doc comment naming it.
- `E/🧱️elements/🏛️ShellHost/🟦️.tsx:6553` tests `action.action !== "fillBuildTick"`.
- The doc comment at `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts:923` names `fillBuildTick`.
- All of these should go together with W1-B's command deletion.

**W1-D (root `📜️script.ts`)**

- The predicates at `📜️script.ts:9733-9739` and `:9827-9837` read `FillDiagnosticOverlay`, `data-fill-*`, `WORLD_FILL_*_KEYS` and the fill parser out of `WH`. They were already out of sync with the file before this lane; they now fail by construction and must be deleted or replaced by the `FILL_TRIED_RING` absence predicate.
- `:9882` reads `fillBuildPreview.candidatePage` from a puzzle fixture.

**Rust twin**

- A Rust twin of `⏯️tool-run-trace-mount.json` over `ToolRunTraceLayer::draws` would pin the same batch counts on wgpu. The fixture is language-neutral for that purpose.

**Later lanes**

- Legend toggles as window config, persisted local-only.
- React view-state echo of `toolRunTraceCursor`.
- Drawing `entity` subjects by mapping entity id to `instance.id` or `interactionId`.
