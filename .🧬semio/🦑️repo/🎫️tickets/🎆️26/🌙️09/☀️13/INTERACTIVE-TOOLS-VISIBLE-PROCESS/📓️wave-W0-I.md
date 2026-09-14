# ⏯️ Wave W0-I: Tool Run API Gaps

Lane W0-I of `📋️tool-run-contract.md` (§2.1, §2.2, §2.5, §2.7, §3.3). It closes the ten API gaps W1-B (§7), W3-F (open item 2) and W2-A (open items 1 and 2) reported. Status: **landed, green** (see §4 for the one run still owed at the time of writing, if any).

Abbreviations:

- `R` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`
- `P` = `🔌️plugin/🦀️.rs`
- `M` = `🧰️framework/🔨️modules/⏯️tool-run/`
- `E3` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`
- `EL` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`
- `T` = this ticket folder

Logs are in `T/🗑️generated/W0-I/`.

## 1. What Changed, per Gap

### 1.1 Base Change Rebinds the Running Job (gap 1)

- `watch_tool_run_generations` (`R:1220`), for any non-`restart` rebase:
  - a retargetable job gets `rebind(identity)` in place;
  - any other job is closed through `retarget_current_job` (`R:857`) and rebuilt next turn under the new identity, from its checkpoint.
- So no tick of the old identity is ever produced. Before, the job kept running and every one of its ticks was dropped as stale.
- `restart` keeps its close, discard and rebuild behaviour.

### 1.2 Run State in the Command Context (gap 2)

- `ArtifactOwnedToolJobContext` (`P:13898`, `P:13905`) gains two methods, and its constructor is unchanged:
  - `with_tool_run(Option<ToolRunView>) -> Self`;
  - `tool_run() -> Option<&ToolRunView>`.
- The retained command admission binds `self.tool_runs.view()` (`P:25763`).
- The run view is ephemeral local state, so it is not part of the context's identity digest.
- **Puzzle 3d:**
  - `Puzzle3dActionCtx.tool_run` and `Puzzle3dActionPrologue.tool_run` were added, plus `Puzzle3dWindowCommandWork::with_tool_run`.
  - The `engagementAbort` arm binds `request.context.tool_run()`.
  - `🎮️commands/🛑️engagement-abort` dispatches `toolRunAbort` with `fill_tool::live_fill_run(ctx.tool_run)`'s identity. It no longer reads the renderer-echoed cursor.

### 1.3 Chords Resolve the Live Run (gap 3)

- `apply_tool_run_action` (`R:1002`): an action whose args name no identity (neither `runId` nor `generation`) targets the live slot's `(run, generation)` at dispatch time. Args of `None`, `{}` and `{windowId}` all count as naming no identity.
- An identity that is named is checked as is:
  - a stale one is still `toolRun.stale`;
  - a partial one (only `generation`) is never completed from the live run.

### 1.4 Trace Keys and Placements in the Request (gap 4)

- `ToolRunJobRequest` gains:
  - `trace_keys: ToolRunTraceKeys` (`R:80`), one allocator per run shared by every job. It observes every upserted key in `apply_tick`, and `allocate(n)` hands out keys above all of them.
  - `entity_marks: &[(u32, u64)]` (`R:81`): the provisional length at the end of the appending tick, plus the entity.
- **Puzzle 3d revalidate:**
  - `fill_run_placements(provisional, lane, first_key)` takes its keys from `request.trace_keys.allocate(placements)`.
  - `FILL_REVALIDATE_KEY_BASE` (`1 << 63`) is deleted.

### 1.5 Settings Changes Only from Declared Reads (gap 5), Schema-First

`ToolRunDefinition.settings: ToolRunSettingsReads { config: [pointer], windowConfig: { windowKindId: [pointer] } }` (`M/🦀️.rs:1784`, `:1841`, schema `$defs.ToolRunSettingsReads` / `ToolRunSettingsPointer`).

- **Pointers.** Pointers are RFC 6901 JSON Pointers into the `ToValue` document of the app config and of window config partitions.
  - `validate()` refuses a malformed pointer with `InvalidSettingsPointer`.
  - `tool_run_pointer_tokens` and `tool_run_pointer_value` (`M:1806`, `:1814`) resolve them.
- **Watch** (`R:1220`).
  - A config generation change, or a window-config publication, is only a trigger.
  - The driver re-reads the declared values (`tool_run_settings_values`, `R:1138`) and fires `settingsChanged` only if they differ.
  - A camera move therefore never replays a run.
- **Empty or absent declaration.** It reads no settings: serde default, and it is skipped on the wire.
- **Why this option and not a `tool_run_settings_digest` hook:**
  - The declaration lives in the manifest, which is the run's source of record.
  - It is validated by the same JSON Schema and fixtures in Rust and TS.
  - It can be inspected without running app code.
  - A digest hook would be code-first, opaque to the schema, and could silently omit a field.

### 1.6 Read-Only Ledger Inspection for Tests (gap 6)

- `VcsArtifactApp::tool_run_ledger() -> &ToolRunLedger<A>`, under `#[cfg(feature = "artifact-app-testing")]` (`P:20082`).
- The ledger gained read-only accessors: `checkpoint()`, `trace_keys()`, `entity_marks()` and `window()` (`R:656`–`:671`).

### 1.7 Large-Graph Re-Send Flicker (gap 7, W3-F)

- A retract refold now carries `boundary: false` (`ToolRunRefold`, `R:331`).
  - It folds provisional ops, including those appended after the retract.
  - It swaps the overlay only once the job reaches a boundary: `CheckpointReady`, `Complete`, its close, or no job at all (`settle_refold`, `R:415`).
- Until then the previous overlay stays rendered.
- A refold that is only waiting for its boundary does not block job steps (`refold_tool_run`, `R:1338`), and it counts as no work once caught up, so a port-waiting job never spins.
- Base-change and finalize refolds swap as soon as they are folded (`boundary: true`).

### 1.8 `reconfigure: resume` Retargets in Place (gap 8)

- **Job protocol:** `pub trait ToolRunRetargetableJob<C>: InteractiveJob + Send { fn rebind(&mut self, ToolRunIdentity); fn reconfigure(&mut self, ToolRunIdentity, Arc<C>) -> bool }` (`R:111`).
  - It is built through the new `ArtifactApp::build_retargetable_tool_run_job` hook, which defaults to `None`.
  - The hook is mirrored on `ArtifactEditor` and `ViewerApp` and forwarded by `EditorApp` and `ViewerApp` (`P:11148`, `:29534`, `:30019`, `:30301`, `:30654`).
  - The driver asks this hook first and falls back to `build_tool_run_job`.
- **Driver:**
  - A mutating retargetable job stays resident while its run is `complete` (`R:1472`); it is not work.
  - A settings change calls `reconfigure`. `false`, or a plain job, means close and rebuild from the checkpoint.
  - Finalize closes a resident run job before revalidation (`R:1533`).
- **Puzzle 3d fill:**
  - `Puzzle3dFillToolRunJob` implements the trait. `reconfigure` retargets `FillRunJob` in place (`FillRunJob::retarget`) when only `fillCount` moved.
  - A changed overlap budget or weights declines, which leads to rebuild and restart.
  - `FillRunJob::rebind` and `FillRevalidateJob::rebind` are production API again.
  - `E3/🦀️.rs` builds fill through `build_retargetable_tool_run_job`. `build_tool_run_job` keeps W2-C's brush arm.
  - The fill declares `settings.config = RUN_SETTINGS_CONFIG` = `["/fillCount", "/overlapBudget", "/objectKindWeights", "/vortexKindWeights"]`.
- This removes W1-B deviation 1 ("count change = rebuild + replay") for fill. Replay remains only for weight or overlap changes and for plain jobs.

### 1.9 Starting Window and Its Config in the Request (gap 9, W2-A)

- **Request fields:**
  - `ToolRunJobRequest.window_id: Option<&str>` is `toolRunStart`'s `windowId`, else the dispatching view's `window_id` or `focused_window_id` (`R:74`).
  - `window_config: Option<WindowConfigSnapshot>` is that window's current snapshot at build time.
- **Settings reads.** A declared `windowConfig` kind that matches the starting window's kind reads only that window's partition. Any other declared kind reads every partition of its kind.
- **Registry, additive (foreign):** `WindowConfigOwnerRegistry::snapshot(kind, window)` and `::pointer_values(kind, pointers)`.
- **Call site for puzzle 2d** to move `fillCount` and `suggestionOffset` back to window config:
  - `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3643` (`build_tool_run_job`, `Run` arm). Read `request.window_config.as_ref().and_then(|window| window.get::<the 2d window config owner>())` in place of `request.config.fill_count` and `PUZZLE2D_DEFAULT_SUGGESTION_OFFSET`.
  - Then change `run_definition()` (`…/◻️2d/…/🛠️tools/🪣️fill/🦀️.rs:31`) to declare `settings.window_config = { "<2d window kind>": ["/fillCount", "/suggestionOffset"] }` instead of `config: ["/fillCount"]`.
  - No other 2d edit was made.

### 1.10 Board2d `toolRunTrace` Lane (gap 10, W2-A)

- **Rust scene** (`🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`):
  - `Board2dScene` gains `tool_run_trace: Option<String>` and `lanes: Vec<SceneLaneRef>`, in the pack wire, serde and value forms.
  - Added `Board2dSceneLane { ToolRunTrace }` and `BOARD2D_SCENE_LANE_{KEY_PREFIX,NAMES,FIELDS,BODY_KEYS,OPTIONAL}`, with body key `framework.scene.board2d.toolRunTrace`.
  - Added `split_lanes` and `merge_lane`.
- **Fixture** `🖱️ui/🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json`:
  - the lane declaration and round trip;
  - `traceShapes`: the footprint rule, catalog, shapes, malformed cases and a pinned `placementLane`.
- **TS mirror** (`🎬️scene/🟦️.ts`): `Board2dScene.{toolRunTrace,lanes}`, `BOARD2D_SCENE_LANE_KEY_PREFIX`, `BOARD2D_SCENE_LANES`, `board2dSceneLaneForBodyKey` and `board2dSceneFromLanes`.
- **wgpu reconcile:** a Board2d arm in `🔀️reconcile/🦀️.rs` (`merge_scene_lanes`).
- **Render-plan validator:** `board2d.toolRunTrace` is checked (`EL/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`).
- **React Interpreter:** `PagedSurfaceView` and `SurfaceView` page `board-2d` surfaces with `BOARD2D_SCENE_LANES`.
- **Plugin injection** (`R:272`, `:283`): `tool_run_scene_surface` and `inject_tool_run_trace_lane_into` handle `board-2d@1`.
- **React host:**
  - New `EL/🖥️Board2dHost/⏯️tool-run-trace/🟦️.tsx`: `BOARD2D_TOOL_RUN_TRACE_KIND_SIZE`, `board2dToolRunTraceShapes` and `board2dToolRunTracePathForShape`.
  - `Board2dHost` mounts `ToolRunTrace2dLayer` with the board camera, the kind footprints and `useToolRunTraceCursorEcho(windowInstanceId)`.
- **wgpu store and paint** (`♾️infinite/🌍️world/⏯️tool-run-trace/🦀️.rs`, additive): `ToolRunTraceLayer::placement_draws`, `ToolRunTrace2dDraw`, `ToolRunTraceShape2d`, `BOARD2D_TOOL_RUN_TRACE_KIND_SIZE` and `board2d_tool_run_trace_shapes`.
- **wgpu host** (`EL/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`):
  - `EngineSurface` gains `board_trace`, `board_trace_shapes` and `board_trace_palette`. `BoardSyncCache` gains `tool_run_trace_window_id`, closed like its other strings.
  - `sync_board_tool_run_trace` applies the lane.
  - `append_board_tool_run_trace` paints the footprints over `build_vector_scene()` at the host's live camera, with the newest `testing` record outlined.
  - `board2d_tool_run_trace_cursors()` is merged into the Shell's `live_view_state` (`🐚️Shell`).

## 2. Public API as Landed

```rust
// semio-framework-tool-run (M)
pub struct ToolRunSettingsReads { pub config: Vec<String>, pub window_config: BTreeMap<String, Vec<String>> } // Default; is_empty(), pointers()
pub struct ToolRunDefinition { …, pub settings: ToolRunSettingsReads }               // serde/value default, skipped when empty
pub enum ToolRunDefinitionError { …, InvalidSettingsPointer(String) }
pub fn tool_run_pointer_tokens(pointer: &str) -> Option<Vec<String>>;
pub fn tool_run_pointer_value<'a>(document: &'a dsl::DslValue, pointer: &str) -> Option<&'a dsl::DslValue>;
// semio-framework (manifest): re-exports ToolRunSettingsReads, ToolRunTraceCursor

// semio-framework-plugin
pub struct ToolRunJobRequest<'a, A> { …, pub window_id: Option<&'a str>, pub window_config: Option<WindowConfigSnapshot>, pub trace_keys: ToolRunTraceKeys, pub entity_marks: &'a [(u32, u64)] }
#[derive(Clone, Debug, Default)] pub struct ToolRunTraceKeys;   // next() -> u64, allocate(count: u64) -> Range<u64>
pub trait ToolRunRetargetableJob<C>: InteractiveJob + Send { fn rebind(&mut self, identity: ToolRunIdentity); fn reconfigure(&mut self, identity: ToolRunIdentity, config: Arc<C>) -> bool; }
// ArtifactApp / ArtifactEditor / ViewerApp (default Ok(None); EditorApp<E>, ViewerApp<V> forward)
fn build_retargetable_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<Self::Config>>>, Fault>;
impl<A: ArtifactApp> ToolRunLedger<A> { pub fn checkpoint(&self) -> Option<&[u8]>; pub fn trace_keys(&self) -> Option<&ToolRunTraceKeys>; pub fn entity_marks(&self) -> &[(u32, u64)]; pub fn window(&self) -> Option<(&str, &str)>; }
impl<A, M> VcsArtifactApp<A, M> { #[cfg(feature = "artifact-app-testing")] pub fn tool_run_ledger(&self) -> &ToolRunLedger<A>; }
impl<A> ArtifactOwnedToolJobContext<A> { pub fn with_tool_run(self, tool_run: Option<ToolRunView>) -> Self; pub fn tool_run(&self) -> Option<&ToolRunView>; }
impl WindowConfigOwnerRegistry { pub fn snapshot(&self, window_kind_id: &str, window_id: &str) -> Option<WindowConfigSnapshot>; pub fn pointer_values(&self, window_kind_id: &str, pointers: &[String]) -> Vec<(String, Vec<Option<protocol::DslValue>>)>; }

// semio-framework-ui-scene
pub struct Board2dScene { …, pub tool_run_trace: Option<String>, pub lanes: Vec<SceneLaneRef> }
pub enum Board2dSceneLane { ToolRunTrace }   // ALL, name, field, body_key, optional, from_body_key, from_name, take, put
pub const BOARD2D_SCENE_LANE_KEY_PREFIX: &str; pub const BOARD2D_SCENE_LANE_NAMES/FIELDS/BODY_KEYS: [&str; 1]; pub const BOARD2D_SCENE_LANE_OPTIONAL: [bool; 1];

// semio-framework-os-infinite (world::tool_run_trace)
pub const BOARD2D_TOOL_RUN_TRACE_KIND_SIZE: f64 = 96.0;
pub enum ToolRunTraceShape2d { Circle { radius: f64 }, Rectangle { width: f64, height: f64 } }
pub struct ToolRunTrace2dDraw { pub shape: u32, pub position: [f32; 2], pub rotation: f32, pub color: Rgba, pub newest: bool }
pub fn board2d_tool_run_trace_shapes(glyph_catalogs_json: &str) -> Vec<ToolRunTraceShape2d>;
impl ToolRunTraceLayer { pub fn placement_draws(&self, palette: &ToolRunTracePalette, visibility: ToolRunTraceVisibility) -> Vec<ToolRunTrace2dDraw>; }

// semio-framework-os-renderer-wgpu (engine_canvas)
pub fn board2d_tool_run_trace_cursors() -> HashMap<String, semio_framework::ToolRunTraceCursor>;
pub fn sync_board2d_scene(scene: &UiComponentSceneNode, window_id: &str, bounds: Rect, theme: &Theme) -> bool; // gained `theme`
```

```ts
export type ToolRunSettingsReads = { config?: Array<string>, windowConfig?: { [key in string]?: Array<string> } };  // generated manifest; ToolRunDefinition.settings?
export type Board2dScene = { …; readonly toolRunTrace?: string; readonly lanes?: readonly SceneLaneRef[] };
export const BOARD2D_SCENE_LANE_KEY_PREFIX; export const BOARD2D_SCENE_LANES; export function board2dSceneLaneForBodyKey(bodyKey); export function board2dSceneFromLanes(spine, laneTexts);
export const BOARD2D_TOOL_RUN_TRACE_KIND_SIZE = 96; export type Board2dToolRunTraceShape;
export function board2dToolRunTraceShapes(glyphCatalogsJson: string): readonly Board2dToolRunTraceShape[];
export function board2dToolRunTracePathForShape(shapes, createPath?): (shape: number) => Path2D | null;
```

### Breaking changes (precise)

1. **`ToolRunDefinition` gained the field `settings`.** Every Rust struct literal needs `settings: …`.
   - The sites are energy, remodel, puzzle 2d, puzzle 3d, layout-run and the plugin app-builder test.
   - Energy, layout-run and puzzle 2d/3d had already been given `ToolRunSettingsReads::default()` by a peer sweep. This lane set the real declarations for 2d, 3d and remodel.
   - JSON declarations, such as generation3d's `🧵️preview-eval/🔣️.json` and the manifest fixtures, keep working: the field defaults to empty.
   - **Behaviour:** a run whose definition declares nothing is never reconfigured by any settings publication. This matters for the generation3d preview eval, which reads no settings.
2. **`Board2dScene` gained `tool_run_trace` and `lanes`.** Literals need both. The sites are puzzle 2d `puzzle2d_board_scene` and 5d `puzzle5d_board_scene`, both fixed.
3. **`engine_canvas::sync_board2d_scene` gained `theme: &Theme`.** Its only caller is `sync_engine_scene`.
4. **Semantics:** a mutating run built through `build_retargetable_tool_run_job` keeps its job resident while `complete`. Plain jobs behave as before.

## 3. Tests Run (exact commands, foreground)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-plugin --features artifact-app-testing -j 4 --lib -- tool_run` | **27 passed, 0 failed** (`plugin-test-4.txt`): the 19 prior laws plus 8 new laws (list below) |
| `cargo test -p semio-framework-tool-run -j 4` | **24 passed** (`tool-run-test-final.txt`), new: `settings_pointers_resolve_like_the_rfc_6901_oracle`; the definition round trip covers `settings` |
| `bun test ./🧪️tests/🧩️conformance/🟦️.ts` (in `M`) | 18 pass; ajv validates the new `settings` and `settingsPointers` schema and fixture (`tool-run-ts-conformance-1.txt`) |
| `cargo check -p semio-framework-plugin --features component-guest --target wasm32-wasip2 -j 4` | Finished; the plugin lib reached its 28 warnings (`plugin-wasip2-final.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | Finished; plugin (26), os-infinite and renderer (46) reached warnings, none in edited files (`renderer-wasm32-unknown-final.txt`) |
| `cargo check -p semio-framework-plugin --target wasm32-unknown-unknown` | Pre-existing stop in `semio-framework-ui` (`web_sys` unresolved when built standalone, as W3-F noted); hence the renderer check above (`plugin-wasm32-unknown-1.txt`) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- fill --test-threads=4` | **49 passed, 0 failed** (`test-3d-fill-t4-1.txt`) |
| same with `--test-threads=1` | **49 passed, 0 failed**, 269 s (`test-3d-fill-t1-1.txt`) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --lib` plus a `[DEBUG]` probe fn | reached `w0i_debug_probe` never used, so the crate is type-checked; the probe was removed and the file compared byte-identical (`check-3d-probe.txt`) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 --target wasm32-wasip2` | Finished, 95 warnings (`check-3d-wasip2-1.txt`) |
| `cargo test -p semio-framework-ui-scene -j 4` | **125 passed**, new: `board2d_scene_lanes_mirror_the_language_neutral_declaration` and `board2d_scene_splits_into_the_declared_lanes_and_merges_back` (`ui-scene-test-1.txt`) |
| `cargo test -p semio-framework-os-infinite --lib -j 4 -- tool_run` | **6 passed**, new: `board2d_placement_draws_follow_the_board_lane_contract` (`infinite-test-1.txt`) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-os-renderer-wgpu --lib -j 4 -- a_board_window_paints_its_tool_run_trace_lane tiled_map_and_board_windows validate_component_scene_bounds_the_tool_run_trace_lane --test-threads=1` | **3 passed**, new: `a_board_window_paints_its_tool_run_trace_lane_and_echoes_the_cursor` (`renderer-test-board-trace-2.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests -j 4` | Finished (`renderer-native-check-2.txt`) |
| `bun ./📜️script.ts test long "tool-run-trace"` (react package) | 3 files, **10 passed**, new board suite (2 tests) (`react-trace-test-1.txt`) |
| `bun ./📜️script.ts test long "🗣️Interpreter/🟦️.tsx"` | **98 passed**, new board-2d carrier test (`react-interpreter-test-1.txt`) |
| `bunx tsc --noEmit -p tsconfig.json` (react package) | no error in edited files; 2 pre-existing Interpreter errors at `:275`, `:1716` (`react-tsc-1.txt`) |
| `cargo test -j 4 --features typegen exports_typescript_bindings` (in `🧰️framework/📦️packages/🦀️rust`) | passed: the projection renders byte-identical to the committed generated manifest TS (`framework-typegen-check-2.txt`) |
| `cargo check -j 4 --lib --tests -p semio-s-artifact-energy-model -p semio-framework-graph-layout-run -p semio-s-artifact-remodel-remodeling` | Finished (`foreign-crates-check-2.txt`) |
| `cargo check … -p semio-s-artifact-puzzle-2d --features component-app-assembly …` | lib and lib-test reached warnings (`foreign-crates-check-1.txt`). The generation3d `example-geometry` test target fails without its features; that failure is pre-existing and unrelated. |

**New plugin laws** (`🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs`, fixture `🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json` sections `rebind`, `chord`, `settingsReads`, `retarget`, `compact`, `traceKeys`, `startWindow`, `boardLane`, `windowSettingsReads`):

1. `tool_run_base_change_rebinds_the_running_job_so_ticks_never_carry_a_stale_identity`: covers both a plain job (rebuilt from its checkpoint) and a retargetable one (rebound, never rebuilt).
2. `tool_run_chord_actions_without_an_identity_resolve_the_live_run_and_stale_identities_still_no_op`.
3. `tool_run_settings_changed_fires_only_for_the_declared_settings_reads`:
   - a republished equal target and a window-config publication (camera) keep generation 0 and the job;
   - a changed target reconfigures;
   - an undeclared run never reconfigures.
4. `tool_run_reconfigure_resume_retargets_a_retargetable_job_in_place`: raise, lower and rebase-then-raise all run without a rebuild (`resumedFrom` stays 0), and the job stays resident while `complete`.
5. `tool_run_retract_keeps_the_previous_overlay_until_the_job_reaches_its_checkpoint`: `count=5` stays rendered across three compaction pages, and `count=3` appears after the checkpoint.
6. `tool_run_requests_carry_trace_key_allocation_entity_marks_and_the_starting_window`.
7. `tool_run_board_scene_render_carries_the_trace_lane`.
8. `tool_run_window_settings_reads_follow_the_starting_window_only`: the request carries the window config snapshot, another window of the same kind is ignored, and the starting window reconfigures.

**Mutation checks** (each file restored and verified byte-identical; script `T/🐍️w0i-mutation-check.py`)

| Mutation | What went red |
|---|---|
| No rebind on base change | The rebind law. The stale job appended until the toy provisional cap and panicked (SIGABRT on unwind) (`mutation-no-rebind.txt`) |
| No chord resolution | The chord law |
| Settings compare ignored | The settings law |
| Refold always at boundary | The overlay law |
| In-place reconfigure replaced by close | The retarget law |
| Board schema dropped from the injection list | The board lane law (`mutation-no-board-injection.txt`) |
| Starting-window filter dropped | The window-settings law (`mutation-window-settings-all-partitions.txt`) |
| TS footprint rule changed | Both board footprint tests (`react-board-mutation.txt`) |

**Flake fixed during the lane.** The rebind law once rebased before the plain job's first checkpoint. It now waits for `ledger.checkpoint()` (`plugin-test-2.txt`).

## 4. Commands to Register in launch.json

- `cargo test -p semio-framework-plugin --features artifact-app-testing -- tool_run`
- `bun nx run @semio-tech/framework-tool-run-rs:test`
- `cargo test -p semio-framework-ui-scene`
- `cargo test -p semio-framework-os-infinite --lib -- tool_run`
- `cargo test -p semio-framework-os-renderer-wgpu --lib -- a_board_window_paints_its_tool_run_trace_lane validate_component_scene_bounds_the_tool_run_trace_lane`
- `bun ./📜️script.ts test long "tool-run-trace"` (react package)
- `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown`

## 5. Deviations, with Reasons

1. **The retarget hooks are a second builder (`build_retargetable_tool_run_job`) and trait, not new methods on `ToolRunJob`.** Changing `ToolRunJob = Box<dyn InteractiveJob + Send>` would have broken every concurrent lane's job. The additive builder keeps plain jobs untouched and gives typed, synchronous hooks.
2. **Retract refold boundary.**
   - W3-F proposed deferring until "the next tick without `retractTo`". A compaction's later pages are exactly such ticks, so that rule would still flicker.
   - The boundary is instead the job's next `CheckpointReady` or `Complete`, or its close. W3-F's layout run checkpoints at the end of every compaction.
   - A job that retracts and never checkpoints keeps the previous overlay until it completes or closes.
3. **Only mutating retargetable jobs stay resident in `complete`.** Read-only runs still finalize themselves, which is the existing law.
4. **`ArtifactOwnedToolJobContext::tool_run` is not part of the context identity digest.** It is ephemeral local state. The action resolves or checks the identity at dispatch, so a queued command with an older run view only no-ops (stale).
5. **The Board2d wgpu trace layer is dropped whole at engine-surface retirement.** Its columns are plain data, so the drop costs O(batches) frees. Its window id string is closed through the existing bounded `close_board_sync`.
6. **The Board2d footprint is `96 × scale`,** the kind size the puzzle 2d fill captures kinds at. The rule is pinned in the board lane fixture and implemented twice (Rust and TS).
   - The React overlay reads the scene camera, so it lags a live pan by one scene publication.
   - The wgpu overlay reads the host's live camera.

## 6. Foreign Edits

| Area | Files |
|---|---|
| tool-run module (`M`, additive) | `🦀️.rs` (settings reads, pointer resolver), `🧬️schema/🔣️.json`, `🧫️fixtures/⚖️lifecycle-law.json`, `🧪️tests/🔬️unit/🦀️.rs`; input script `T/🐍️w0i-settings-reads-schema.py` |
| manifest | `🛂️manifest/🦀️.rs` (re-exports `ToolRunSettingsReads`, `ToolRunTraceCursor`), `🛂️manifest/🟦️.ts`, `🤖️generated/🪪️manifest/🟦️.ts`, `🧬️schema/📽️projection/🦀️.rs`, `🧪️tests/🔬️app-label/🦀️.rs` (type count 194 → 195) |
| plugin (outside the tool-run regions) | `🪟️window/🎚️config/🦀️.rs` (`snapshot`, `pointer_values`), `P` (hook on four traits, context `tool_run`, `tool_run_ledger`), `🧪️tests/🔬️app-app-builder/🦀️.rs` (`settings`) |
| ui-scene / ui | `🎬️scene/🎬️scenes/🦀️.rs`, `🎬️scene/🟦️.ts`, `🎬️scene/🧪️tests/🔬️scenes-unit/🦀️.rs`, new `🎬️scene/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json`, `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` |
| os-infinite | `🌍️world/⏯️tool-run-trace/🦀️.rs`, `…/🧪️tests/🔬️unit/🦀️.rs` |
| renderer | `EL/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, `EL/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`, `EL/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`live_view_state`), `EL/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, `EL/🗣️Interpreter/🧪️tests/🔬️wgpu-render-plan-validator/🦀️.rs`, `EL/🗣️Interpreter/🟦️.tsx`, `EL/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx`, `EL/🖥️Board2dHost/🟦️.tsx`, new `EL/🖥️Board2dHost/⏯️tool-run-trace/{🟦️.tsx,🧪️tests/🧩️component/🟦️.ts}`, `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (suite registration) |
| puzzle 2d (W2-A, finished) | `…/🛠️tools/🪣️fill/🦀️.rs` (`settings.config = ["/fillCount"]`; a peer's default had silently disabled count reconfigure), `…/🎭️modes/✏️edit/🦀️.rs` (Board2dScene literal) |
| puzzle 5d (W2-B) | `…/🪟️windows/◻️2d/🦀️.rs` (Board2dScene literal) |
| remodel | `…/🧵️reconstruction-session/🦀️.rs` (`settings: ToolRunSettingsReads::default()`, import) |
| puzzle 3d (owned follow-up) | `E3/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` (settings, retargetable job, trace-key allocation), `E3/⏳️precompute/🪣️fill/🦀️.rs` (`retarget`, `rebind`, `fill_run_placements(.., first_key)`, `FILL_REVALIDATE_KEY_BASE` deleted), `…/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`, `E3/🦀️.rs` (retargetable builder, `Puzzle3dActionCtx.tool_run`, prologue field, `with_tool_run`, abort arm), `E3/🎮️commands/🛑️engagement-abort/🦀️.rs` |

## 7. Open Items

1. **Puzzle 2d (owner W2-A or successor).** Move `fillCount` and `suggestionOffset` back to window config at the call site in §1.9, and declare them under `settings.window_config`.
2. **Generation3d preview eval** declares no settings reads, so no settings publication restarts it now. If a window-config field feeds the evaluation, its owner should declare it in `🧵️preview-eval/🔣️.json` `definition.settings`.
3. **Remodel and energy.** Energy declares its three simulation fields. Remodel reads no config; its "a camera orbit must not lose progress" note is now structurally true.
4. **React Board2d overlay camera.** The overlay follows the scene camera rather than the live session camera during a pan. A follow-up can feed `session.cameraJson()` per animation frame.
5. **Layout-run consumers** (W3 lanes) should declare `settings` on top of `layout_run_definition(..)` and can adopt `ToolRunRetargetableJob` to retarget iteration settings in place.
