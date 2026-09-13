# ⏯️ Wave W0-E: trace lane and renderers

Lane W0-E of `📋️tool-run-contract.md` (§3.2, §4, §5 W0-E row, §6 items 3 and 7).

**Status: landed, green.**

Paths below are relative to the repo root.

- `S` = `🧰️framework/🔨️modules/🖱️ui/🎬️scene/`
- `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`
- `W` = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/`

## 1. What changed

### 1.1 Scene lanes (crate `semio-framework-ui-scene`)

**World3d lane** (`S/🎬️scenes/🦀️.rs`)

- New field `World3dScene.tool_run_trace: Option<String>` (`:400`), serialized as `toolRunTrace`. It is threaded through the pack wire, `base()`, `ToValue` and `FromValue`.
- New lane `World3dSceneLane::ToolRunTrace` (`:657`):
  - body key `framework.scene.world3d.toolRunTrace`, optional;
  - added to `WORLD3D_SCENE_LANE_NAMES`, `FIELDS`, `BODY_KEYS` and `OPTIONAL` (all arrays are now length 20), plus `ALL`, `take` and `put`.
- The payload is an opaque base64url string, so the crate gains no dependency on tool-run.

**Canvas2d lane** (`S/🎬️scenes/🦀️.rs:170-330`)

- `Canvas2dScene` gains `tool_run_trace: Option<String>` and `lanes: Vec<SceneLaneRef>`.
- New constructor `Canvas2dScene::base(camera_x, camera_y, zoom, layers_json)`.
- New `Canvas2dScenePack` wire and a `SceneDoc` impl with `split_lanes`/`merge_lane`.
- New `Canvas2dSceneLane { ToolRunTrace }` and the `CANVAS2D_SCENE_LANE_{KEY_PREFIX,NAMES,FIELDS,BODY_KEYS,OPTIONAL}` constants. The body key is `framework.scene.canvas2d.toolRunTrace`.

**Renames** (no compatibility layer)

- `World3dSceneLaneRef` → `SceneLaneRef` (`:430`).
- `world3d_scene_lane_hash` → `scene_lane_hash` (`:863`).
- Both are now shared by the two scenes.

**TypeScript mirror** (`S/🟦️.ts`)

- `World3dScene.toolRunTrace` and `Canvas2dScene.{toolRunTrace,lanes}`.
- `SceneLaneRef` and the generic `SceneLane<S>`.
- `sceneFromLanes` (`:491`); `world3dSceneFromLanes` is now a wrapper around it.
- The `toolRunTrace` entry in `WORLD3D_SCENE_LANES`.
- `CANVAS2D_SCENE_LANE_KEY_PREFIX`, `CANVAS2D_SCENE_LANES`, `canvas2dSceneLaneForBodyKey` and `canvas2dSceneFromLanes`.

**Fixtures**

- `S/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json` gains the lane row.
- New `S/🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json`. Its round-trip payload is the base64url of `⏯️tool-run/🧫️fixtures/📼️trace-pages.json` `deltas[0]`.
- The typed scene catalog `🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json` gains `toolRunTrace` for both records.

**Tests**

- `S/🧪️tests/🔬️scenes-unit/🦀️.rs` gains three tests:
  - `world3d_tool_run_trace_rides_its_own_optional_lane_and_merges_back`
  - `canvas2d_scene_lanes_mirror_the_language_neutral_declaration`
  - `canvas2d_scene_splits_into_the_declared_lanes_and_merges_back`
- `🔬️scenes-value-round-trip` also checks the value and pack codecs with a trace lane present.

### 1.2 Lane consumers

**wgpu reconcile** (`🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:357`)

- `merge_world3d_lanes` became the generic `merge_scene_lanes<T: SceneDoc>`.
- It now merges lanes for both world-3d and canvas-2d surfaces.

**React Interpreter** (`E/🗣️Interpreter/🟦️.tsx`)

- `world3dSurfaceLaneTexts` became `surfaceLaneTexts<S>(…, lanes)` (`:581`).
- `PagedSurfaceView` picks `CANVAS2D_SCENE_LANES` or `WORLD3D_SCENE_LANES` by surface kind (`:1492`).
- `SurfaceView` now pages canvas-2d surfaces too (`:1557`).
- The test registration passes world and canvas lane-text adapters.
- `E/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx` gains a canvas-2d carrier test.

### 1.3 base64url codec (`🧰️framework/🔨️modules/🚪️io/🔤️base64/`)

The contract requires base64url, and no first-party codec for it existed.

- Rust `base64_url_encode` and `base64_url_decode` (`🦀️.rs:65,129`): RFC 4648 §5, unpadded, strict.
- TS `base64UrlEncode` and `base64UrlDecode` (`🟦️.ts:107,133`).
- New fixture `🧫️fixtures/🔣️rfc4648-base64url-vectors.json` (vectors plus rejected inputs).
- Rust tests use the fixture and a third-party `base64` `URL_SAFE_NO_PAD` oracle. The TS test uses the Node `Buffer` `base64url` oracle.

### 1.4 wgpu world trace layer (crate `semio-framework-os-infinite`)

**New module `W/⏯️tool-run-trace/🦀️.rs`**

- `ToolRunTraceLayer` is the keyed per-window store.
- Batches are per `(family, mesh/shape index, verdict)` and swap-remove records, so upsert and retire are O(1).
- `apply_lane` skips a lane text it has already applied (digest check) and keeps the old records when a lane is malformed.
- `apply_delta` is idempotent:
  - it clears on `clear` or on a run/generation change;
  - it skips pages below the cursor.
- Draws are one `SceneDraw3d` per `(mesh, verdict)`:
  - verdict colors come from the theme (`progress`, `success`, `warning`, `error`);
  - `testing` uses the `toolRun.testingOpacity` token;
  - records fade by age down to the floor token;
  - the newest `testing` record draws `selected` (highlight outline);
  - visibility toggles are applied.
- `tool_run_provisional_color` gives the success tone at the provisional opacity token.
- `TOOL_RUN_TRACE_LANE_BYTES_MAX` is exported for the render-plan validator.

**Mount in `W/🦀️.rs`**

- New `World3dState` fields `tool_run_trace`, `tool_run_trace_visibility` and `provisional_instance_ids` (`:1412-1418`).
- `sync_world3d_tool_run_trace` (`:10063`) applies the lane and maps mesh index → mesh key using `meshes_json` order.
- `world3d_tool_run_trace_cursor` (`:10081`).
- `append_tool_run_trace_draws` (`:10088`) adds translucent draws and falls back to a placeholder box for unknown meshes.
- Provisional instances are partitioned out of the opaque draws and repainted with the provisional color (`:10214`).
- The module is declared with `#[path] pub mod tool_run_trace` (`:10378`).

**Dependency and taxonomy**

- The crate adds a `semio-framework-tool-run` dependency. `os-infinite` sits above `ui`, so there is no cycle.
- `⏯️tool-run-trace` is registered in `taxonomy.json` under `members-of-members-of-members-of-modules`.

**Tests** (`W/⏯️tool-run-trace/🧪️tests/🔬️unit/🦀️.rs`)

- Fixture residency cases delivered through real lane text, including a cold resend.
- Redelivery, stale lanes and malformed lanes.
- Randomized clear, retire, eviction and compaction runs compared against the Rust `ToolRunTraceStore`.
- The draw-count test.
- Fade and provisional tokens.

### 1.5 Render-plan validator

In `E/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:62-94`:

- New `RenderPlanLimits.max_tool_run_trace_bytes`, defaulting to `TOOL_RUN_TRACE_LANE_BYTES_MAX` (16 pages, base64url-sized).
- `Default` now returns the `RENDER_PLAN_LIMITS` constant.
- `check_tool_run_trace` runs for both `world3d.toolRunTrace` and `canvas2d.toolRunTrace`.

New test `validate_component_scene_bounds_the_tool_run_trace_lane_on_both_scene_kinds`.

### 1.6 React 3d trace (`E/🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx`, new)

**Store**

- `ToolRunTraceRecordStore`: base64url → `decodeToolRunTraceDelta` from `⏯️tool-run/🟦️.ts`. It is the same algorithm as the Rust layer.
- Batches keep dense typed columns: `Float32Array` stamps and 4×4 matrices. Each batch tracks a dirty upload range.

**Instancing**

- `createToolRunTraceInstancedMesh`: one `THREE.InstancedMesh` per `(mesh, verdict)`. The age fade runs in the shader through a per-instance `instanceStamp` attribute and a `uNewestStamp` uniform, so fading is O(1) per frame.
- `syncToolRunTraceInstancedMesh` uploads only the dirty range.

**Component and hooks**

- `ToolRunTraceLayer` props: `lane`, `geometryForMesh`, `visibility`, `reducedMotion`, `onCursor` and an optional host-owned `store`.
- The newest `testing` record gets an outline in the `secondary` highlight token. The outline pulses, and stays static under reduced motion.
- `useToolRunTraceStore` and `usePrefersReducedMotion`.
- `toolRunTraceDataAttributes` returns `data-tool-run-{run,generation,page,records,testing,success,warning,danger}`.

**Tokens**

- `TOOL_RUN_TRACE_VERDICT_PAINT`:

  | Verdict | Token | Opacity |
  |---|---|---|
  | `testing` | `semanticVar("accent")`, the progress fill | `testingOpacity` |
  | `success` | `tokenVar("success")` | 1 |
  | `warning` | `tokenVar("warning")` | 1 |
  | `danger` | `tokenVar("danger")` | 1 |

- `TOOL_RUN_PROVISIONAL_PAINT`.

**Provisional hooks** (exposed for W1-C)

- `ToolRunProvisionalIdsContext`
- `useToolRunProvisional`
- `ToolRunProvisionalOutline`: a dashed outline that breathes over the dash period token and stays static under reduced motion.

### 1.7 React 2d twin (`E/📐️Canvas2dHost/⏯️tool-run-trace/🟦️.tsx`, new)

The real host directory is `📐️Canvas2dHost`.

- `paintToolRunTrace2d` sets one fill style per `(shape, verdict)` batch and fills one path per placement at the host camera transform. It applies the fade and highlight, and returns a paint report.
- `ToolRunTrace2dLayer` is an overlay canvas that carries the `data-tool-run-*` counters.
- `resolveToolRunTrace2dPalette` resolves the verdict colors once.

### 1.8 Theme token `provisional` (`🖱️ui/🎨️styling/`)

**New tokens.** `🔣️.json` `metrics.toolRun` adds:

- `testingOpacity`
- `fadeFloorOpacity`
- `fadeRecords`
- `testingOutlineWidth`
- `testingPulseMs`
- `provisionalOpacity`
- `provisionalOutlineWidth`
- `provisionalDash`
- `provisionalDashPeriodMs`

**Generator.** `📽️projection/🟦️.ts` now emits Rust metric modules in snake_case (`pub mod ${toSnakeCase(section)}`), so the new block becomes `ui_styling::metrics::tool_run`. The existing lowercase sections are unchanged. The outputs were regenerated.

**Where it is used**

- wgpu: `ToolRunTracePalette` and `tool_run_provisional_color`.
- React: `STYLING_METRICS.toolRun`.

**World3dHost mesh style** (`E/🌐️World3dHost/🟦️.tsx`)

- `MeshStyleKind` gains `"provisional"`; its paint is the success token at `provisionalOpacity` (`:497,519`).
- `resolveMeshStyle` ranks `provisional` after `danger` and before `celebrated` (`:571`).
- `WorldInstanceRecord.provisional?: boolean` (`:180`).
- In the instance view, `provisional` comes from the context or the record (`:2736`). A provisional instance is not pickable and draws the dashed outline.

### 1.9 Test registration

`E/../🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` gains two element suites: `🌐️World3dHost/⏯️tool-run-trace` and `📐️Canvas2dHost/⏯️tool-run-trace`.

New suites:

- `E/🌐️World3dHost/⏯️tool-run-trace/🧪️tests/🧩️component/🟦️.ts` (6 tests)
- `E/📐️Canvas2dHost/⏯️tool-run-trace/🧪️tests/🧩️component/🟦️.ts` (1 test)

## 2. Public API as landed

```rust
// semio-framework-ui-scene
pub struct World3dScene { …, pub tool_run_trace: Option<String>, … }
pub enum World3dSceneLane { …, Status, ToolRunTrace }            // ALL: [Self; 20]
pub struct SceneLaneRef { pub lane: String, pub bytes: u32, pub hash: String }
pub fn scene_lane_hash(payload: &str) -> String;
pub struct Canvas2dScene { pub camera_x: f64, pub camera_y: f64, pub zoom: f64, pub layers_json: String, pub snapshot: Option<Canvas2dSnapshotLease>, pub tool_run_trace: Option<String>, pub lanes: Vec<SceneLaneRef> }
impl Canvas2dScene { pub fn base(camera_x: f64, camera_y: f64, zoom: f64, layers_json: String) -> Self }
pub enum Canvas2dSceneLane { ToolRunTrace }                      // name/field/body_key/optional/from_body_key/from_name/take/put
pub const CANVAS2D_SCENE_LANE_KEY_PREFIX: &str;                  // + _NAMES/_FIELDS/_BODY_KEYS: [&str; 1], _OPTIONAL: [bool; 1]

// semio-framework-io-base64
pub fn base64_url_encode(bytes: impl AsRef<[u8]>) -> String;
pub fn base64_url_decode(encoded: impl AsRef<[u8]>) -> Result<Vec<u8>, Base64Error>;

// semio-framework-os-infinite: infinite_world::world::tool_run_trace
pub const TOOL_RUN_TRACE_LANE_BYTES_MAX: usize;
pub struct ToolRunTraceBatchKey { pub family: ToolRunTraceFamily, pub index: u32, pub verdict: u8 }   // of(subject, verdict), verdict()
pub enum ToolRunTraceFamily { Instance3d, Placement2d, Entity }
pub struct ToolRunTraceBatch { pub keys: Vec<u64>, pub subjects: Vec<ToolRunTraceSubject>, pub stamps: Vec<u64> }
pub enum ToolRunTraceLaneFault { Base64, Codec(ToolRunCodecError) }
pub struct ToolRunTraceLayerApply { pub cleared: bool, pub pages: u32, pub ops: u32 }
pub struct ToolRunTraceLayer;   // apply_lane(Option<&str>) -> Result<ToolRunTraceLayerApply, ToolRunTraceLaneFault>, apply_delta(&ToolRunTraceDelta),
                                // cursor() -> Option<ToolRunTraceCursor>, len, is_empty, count(verdict), contains(key), batches(), newest_testing(),
                                // draws(&ToolRunTracePalette, ToolRunTraceVisibility) -> Vec<ToolRunTraceDraw>
pub fn tool_run_trace_fade(age: u64) -> f32;
pub struct ToolRunTracePalette { pub testing: Rgba, pub success: Rgba, pub warning: Rgba, pub danger: Rgba }  // from_theme(&Theme), color(verdict)
pub struct ToolRunTraceVisibility { pub testing: bool, pub accepted: bool, pub rejected: bool }        // Default all true, shows(verdict)
pub struct ToolRunTraceDraw { pub mesh: u32, pub verdict: ToolRunVerdict, pub instances: Vec<Instance3d> }
pub fn tool_run_provisional_color(theme: &Theme) -> [f32; 4];
// infinite_world::world
pub struct World3dState { …, pub tool_run_trace: ToolRunTraceLayer, pub tool_run_trace_visibility: ToolRunTraceVisibility, pub provisional_instance_ids: HashSet<String> }
pub fn world3d_tool_run_trace_cursor(state: &World3dState) -> Option<ToolRunTraceCursor>;

// renderer wgpu interpreter
pub struct RenderPlanLimits { …, pub max_tool_run_trace_bytes: usize }
```

```ts
// 🎬️scene/🟦️.ts (@semio-tech/framework)
export type SceneLaneRef; export type SceneLane<S>; export type World3dSceneLane = SceneLane<World3dScene>;
export function sceneFromLanes<S extends object>(spine: S, laneTexts: ReadonlyMap<string, string>, lanes: readonly SceneLane<S>[]): S;
export const CANVAS2D_SCENE_LANE_KEY_PREFIX, CANVAS2D_SCENE_LANES; export function canvas2dSceneLaneForBodyKey(bodyKey); export function canvas2dSceneFromLanes(spine, laneTexts);
// 🔤️base64/🟦️.ts
export function base64UrlEncode(bytes: Uint8Array): string; export function base64UrlDecode(encoded: string): Uint8Array;
// 🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx
export class ToolRunTraceRecordStore { version; cursor; size; newestStamp; newestTesting; batches(); has(key); count(verdict, family?); applyLane(lane); applyDelta(delta) }
export type ToolRunTraceBatch; ToolRunTraceFamily; ToolRunTraceStoreApply; ToolRunTraceVisibility; ToolRunTraceLayerProps;
export const TOOL_RUN_TRACE_METRICS, TOOL_RUN_TRACE_VERDICT_PAINT, TOOL_RUN_TRACE_HIGHLIGHT_PAINT, TOOL_RUN_PROVISIONAL_PAINT, TOOL_RUN_TRACE_VISIBLE_ALL, ToolRunProvisionalIdsContext;
export function toolRunTraceMatrix, toolRunTraceFade, toolRunTraceDataAttributes, useToolRunTraceStore, toolRunTraceShows, usePrefersReducedMotion,
  createToolRunTraceInstancedMesh, syncToolRunTraceInstancedMesh, toolRunTraceCapacity, useToolRunProvisional;
export function ToolRunTraceLayer(props: ToolRunTraceLayerProps); export function ToolRunProvisionalOutline({ geometry, reducedMotion? });
// 📐️Canvas2dHost/⏯️tool-run-trace/🟦️.tsx
export function paintToolRunTrace2d(ctx, store, camera, viewport, pathForShape, palette, visibility?, pulse?): ToolRunTrace2dPaintReport;
export function resolveToolRunTrace2dPalette(): ToolRunTrace2dPalette; export function ToolRunTrace2dLayer(props: ToolRunTrace2dLayerProps);
// World3dHost
resolveMeshStyle({ …, provisional?: boolean }); WorldInstanceRecord.provisional?: boolean
```

## 3. Tests run (foreground; logs in `T/🗑️generated/W0-E/`)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-ui-scene` | 123 passed, 0 failed (`scene-test-2.txt`) |
| `cargo check -p semio-framework-ui-scene --target wasm32-wasip2` and `--target wasm32-unknown-unknown` | Both finished. A temporary `[DEBUG]` probe produced a warning on both targets, which proves type-checking reached the crate. The probe was removed. |
| `cargo test -p semio-framework-io-base64` | 6 passed (RFC vectors plus `URL_SAFE_NO_PAD` oracle) |
| `cargo test -p semio-framework-os-infinite --lib tool_run_trace` | 5 passed (`infinite-test-1.txt`). The crate emitted warnings, all pre-existing, none in lane files. |
| `cargo test -p semio-framework-os-renderer-wgpu --lib render_plan` | 13 run: 9 passed, including all 5 `validate_*` tests (the new trace-lane test among them); 4 image tests failed (`renderer-wgpu-test-1.txt`); see the note below |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- ui_node_wire_format reconcile` | 39 passed (`ui-test-wgpu.txt`) |
| `cargo test -p semio-framework-ui-styling` and `bun ./📜️script.ts check-generated` (styling) | 1 passed; generated artifacts fresh |
| `bun ./📜️script.ts test long "tool-run-trace"` (react package) | 2 files, 7 tests passed: 6 in the 3d suite including the provisional-rank test, 1 in the 2d suite (`react-trace-test-2.txt`) |
| `bun ./📜️script.ts test long "🗣️Interpreter/🟦️.tsx"` | 97 passed, including the new canvas-2d carrier test (`react-interpreter-test-1.txt`) |
| `bunx tsc --noEmit -p tsconfig.json` (react package), plus a temporary tsconfig covering the new trace dirs | 0 errors in any lane file. The remaining errors are pre-existing (`react-tsc-2.txt`, `trace-tsc-1.txt`). |
| `cargo check --keep-going -p lowpoly, reasoning-wires, animate-presentation, draw-drawing, layout-layout, remodel-remodeling --tests` | Finished (`plugins-check-2.txt`) |
| `cargo check -p fem-2d -p procedural-generation2d --features …/component-app-assembly --tests` | Finished (`plugins-check-4.txt`) |
| `cargo check -p semio-framework-plugin --features artifact-app-testing --tests` | Finished (`plugin-check-tests.txt`) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests` | Finished (`puzzle3d-check-tests.txt`) |

**Renderer-wgpu run.** The same run also has 4 failing `resolve_ui_image_*` / `apply_ui_image_bytes_*` tests. They share a process-global image queue: a URL assert mismatch between parallel tests, then a `WorldAssetFetchOwner` drop panic. Nothing this lane touched.

**Mathematical equation crate.** Its `--tests` build fails, but none of the errors are in the lane's file. The failures are `EquationCamera` not found in `✏️editor/🦀️.rs` and the `PluginApp` bound in `window-config-ownership`. Its lib code, including the edited geometry window, compiled inside that run. A retried lib-only check hit shared build-dir churn: another process deleted artifacts mid-build.

**Mutation checks** (both restored byte-identical, verified with `cmp`)

- Rust: removing the swap-remove index fix-up made the randomized lockstep test fail.
- TS: removing the dirty mark on swap made the three.js InstancedMesh oracle test fail.

**TDD note.** The fixtures and tests were written together with the implementation, not strictly first; the two mutation checks show the tests are not vacuous.

## 4. Commands to register in launch.json

- `bun nx run @semio-tech/ui-scene-rs:test` (existing target; now covers the trace lane)
- `bun nx run semio-framework-os-infinite:test` (includes `world::tool_run_trace`)
- `bun nx run @semio-tech/framework-renderer-react:test-long`. The two new element suites run at level `long`. Focused form: `bun ./📜️script.ts test long tool-run-trace` in `…/🎯️targets/⚛️react/📦️packages/🟦️typescript`.
- `bun nx run @semio-tech/ui-styling-tokens:generate` (after `metrics.toolRun` changes)

## 5. Deviations from the contract, with reasons

1. **Canvas2d lane location.** The contract points at `🎬️scene/🖼️canvas2d-snapshot/`, but that directory is the fixed-page snapshot store. `Canvas2dScene` and the lane mechanism live in `🎬️scenes/🦀️.rs`, so the twin lane went there. `canvas2d-snapshot` is unchanged.
2. **Shared lane ref.** `World3dSceneLaneRef` and `world3d_scene_lane_hash` were renamed to `SceneLaneRef` and `scene_lane_hash`. Both scenes share them now, and there are no aliases. Keeping world-named types on the canvas-2d scene would have been wrong.
3. **base64url.** The contract names base64url, but no first-party codec existed. It was added to the `🚪️io/🔤️base64` module (Rust and TS, with fixture and oracles) instead of hand-rolling it in the renderers.
4. **Theme token shape.** "Theme token `provisional`" landed as the `metrics.toolRun` token block plus the existing `success` color. There is no new chrome paint.
   - The wgpu `Theme` already carries `success`, `progress`, `warning` and `error`. The layer composes color from those and opacity from the `ui_styling::metrics::tool_run` constants. The wgpu theme file was not touched.
   - React uses `tokenVar("success")` together with `STYLING_METRICS.toolRun`.
5. **Provisional outline animation.** Core three.js `LineDashedMaterial` has no dash offset, so "animated" is an opacity breath over `provisionalDashPeriodMs`; the dashes themselves stay static. It is static under reduced motion. wgpu paints provisional as a static translucent success tone, with no outline, because the instance pipeline has no dashed-outline pass.
6. **Placement and entity subjects.** `placement2d` subjects draw only on 2d and `instance3d` only on 3d. `entity` subjects are counted in the store and in the `data-tool-run-*` counters but not drawn: highlighting by entity needs the entity → instance mapping W1-C owns.
7. **Store stamps (React).** The React store keeps stamps in a `Float32Array`. They are exact up to 2^24 upserts per run, which is 16× the 1 M residency cap, but a run that re-upserts a key more than 16 M times would lose fade precision.
8. **Legend, toggles and cursor echo are exposed, not wired.** This covers `ToolRunTraceVisibility`, the `onCursor` prop, `world3d_tool_run_trace_cursor` and `ToolRunProvisionalIdsContext`. Plugin and window-config wiring and the view-state echo belong to later lanes, as the brief says.

## 6. Foreign edits (smallest possible, all compile- or test-verified)

- `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`: generic `merge_scene_lanes` plus the canvas-2d arm.
- `E/🗣️Interpreter/🟦️.tsx`: generic lane texts, canvas-2d paging, test dependency adapters.
- `E/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx`: parametrized `surfaceWithLanes`, canvas-2d test.
- `E/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` and `🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs`. The brief assigns these to this lane.
- `E/🌐️World3dHost/🟦️.tsx`: the provisional mesh style (brief-assigned; W1-C owns the file in wave 1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (`P`): two re-export lines renamed to `SceneLaneRef` and `scene_lane_hash`.
- `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` and the puzzle-3d `🧪️tests/🔬️example-switch/🦀️.rs`: hash rename.
- `World3dScene` full struct literals gained `tool_run_trace: None` in:
  - `🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs`
  - `…/🔬️wgpu-render-plan-validator/🦀️.rs`
  - `W/🧪️tests/🔬️unit/🦀️.rs`
- `Canvas2dScene` literals gained `tool_run_trace: None, lanes: Vec::new()`. This was a span-keyed script edit of 23 sites:
  - framework tests: `ui-node-wire-format`, `🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d`;
  - plugins: lowpoly uv, reasoning wires (editor and viewer), generation2d preview (×3), draw canvas (×2), fem 2d results (×3), fem 2d model (×3), animate tile-editor (×2), layout preview/blueprint (×3), mathematical geometry, remodel frames.
- `🔌️plugins`-independent files: `🚪️io/🔤️base64/**`, `🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json`, `taxonomy.json` (one member name), `♾️infinite/📦️packages/🦀️rust/Cargo.toml` (tool-run dependency), and the react vitest config (two suites).

## 7. Open items

- **W0-D (ledger).** Publish `base64_url_encode(ToolRunTraceDelta::encode())` into `World3dScene.tool_run_trace` or `Canvas2dScene.tool_run_trace`, within `TOOL_RUN_TRACE_LANE_BYTES_MAX`. Read the renderer's echoed cursor back. The React hook reports it through `onCursor`; wgpu exposes `world3d_tool_run_trace_cursor`. The view-state echo plumbing itself is not in this lane.
- **W1-C.**
  - Mount `ToolRunTraceLayer` in `WH` with `geometryForMesh` and a host-owned store, and spread `toolRunTraceDataAttributes` on the world DOM wrapper.
  - Provide `ToolRunProvisionalIdsContext` from the framework's provisional entity set, or stamp `provisional: true` on instance records.
  - On wgpu, fill `World3dState.provisional_instance_ids`.
  - Wire the legend toggles to window config (persisted local-only).
- **W2-A.** Mount `ToolRunTrace2dLayer` in `📐️Canvas2dHost` with a `pathForShape`.
- **Pick exclusion on wgpu.** `provisional_instance_ids` is only painted; excluding those instances from wgpu picking belongs with the interaction-topology exclusion (§4.1).
- **Coordinator.** The tracked bundle `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js` embeds the typed scene catalog. It needs its normal regeneration to pick up `toolRunTrace` (it is generated; not hand-edited).
- **Pre-existing reds seen, not caused here.** The 4 wgpu `resolve_ui_image_*` tests (shared global queue under parallel runs); the mathematical-equation `--tests` compile errors; the World3dHost/Interpreter tsc errors listed in `react-tsc-2.txt`.
