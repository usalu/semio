# 📓 WSX Inference Execution Path (read-only audit)

Scope: the framework path an artifact editor in the plugin wasm guest must use to (a) run its own registered inference service incrementally with budgets, cancellation, resume state and progress, (b) surface progress and cancel in the tool-run status pill and panel, and (c) feed results into a window render each frame. Target: generation3d `geometry` (AD2, AD5 of `📓️brep-mesh-widget-set-2026-10-07.md`).

Method: static reading only. Nothing was built, run or tested. Every API cited below was read at the stated line, or confirmed by grep for the line number. Claims not confirmed at runtime are marked **unverified**.

Abbreviations (repo-relative, prefix omitted below):

- `OSMOD` = `🧰️framework/🛍️products/💻️os/🔨️modules`
- `TR` = `OSMOD/🔌️plugin/⏯️tool-run/🦀️.rs` (plugin tool-run ledger, `ArtifactApp` hooks live in `OSMOD/🔌️plugin/🦀️.rs`)
- `FTR` = `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs` (framework tool-run types: `ToolRunTick`, `ToolRunDefinition`, state, events, actions)
- `INF` = `OSMOD/💡️inference/🦀️.rs` (`InferredField`, `InferenceCache`, `infer_field`)
- `IJOB` = `OSMOD/🔌️plugin/⚛️reactor/💼️jobs/💡️infer/🦀️.rs` (`semio.infer` job)
- `GEN` = `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `PREV` = `GEN/🧵️preview-eval/🦀️.rs`, `PREVRUN` = `GEN/🧵️preview-eval/⏯️tool-run/🦀️.rs`
- `GEDIT` = `GEN/✏️editor/🦀️.rs`

---

## 1. Answer in one paragraph

The correct path is the **tool-run run job**, not the `semio.infer` job. The generation3d app declares its geometry service with `.inference_services([...])`, keeps one `InferenceCache` inside its retained instance operation owner, and implements `ArtifactApp::build_tool_run_job` (or `build_retargetable_tool_run_job`) to return an `InteractiveJob`. That job holds the `ArtifactInstanceOperationOwnerHandle`, and on each `step(cx)` it calls the registered `ArtifactInferenceService::infer_with_context` under `owner.with_mut`, passing `cx.fuel_remaining()` as the step's `work_units` and the previous cursor as `previous_state`. It reports `ToolRunTick`s through `StepOutcome::PreviewReady` (progress, steps, counters, small payload), checkpoints through `StepOutcome::CheckpointReady`, and observes the framework Abort through `cx.is_cancelled()`. Windows read the owner's retained values in `render_with_request_context` every time the tool-run marks them dirty. Three framework gaps block a clean implementation (section 6): the `InferredField` driver is not budgeted or resumable, contextual services cannot be reached from the plugin gateway, and the interactive `semio.infer` result loses its completeness flag.

---

## 2. Verified framework facts

### 2.1 Executable inference service (`OSMOD/🔌️plugin/🦀️.rs`)

- `ArtifactInferenceServiceMetadata` (`:1498-1512`) carries owner, artifact kind and schema, inference schema, versions, and `payload: Option<ArtifactInferencePayloadContract>`. The contract has `progress_unit`, `input_schema`, `output_schema`, `artifact_binding`, `commit` (`:1518-1525`).
- Executable shape: `ArtifactInference = for<'a> fn(&ArtifactInferenceExecutionRequest<'a>) -> Result<ArtifactInferenceExecution, ArtifactInferenceExecutionError>` (`:1630`). A **contextual** variant takes a second `&dyn std::any::Any` argument (`:1635`, `:1657` `new_contextual`).
- Request (`:1584-1592`): `policy`, `budgets: &WireArtifactInferenceBudget`, `cancellation_id`, `previous_state: Option<&[u8]>`, `requested_cache_mode`, `canonical_payload`, `dependencies`.
- Result (`:1620-1627`): `canonical_payload`, `diagnostics`, `validity`, `quality`, `complete: bool`, `actual_cache_mode`. There is **no field for work units used** (see gap G6).
- Budget (`:2010-2014`): `allocation_bytes`, `work_units`, `recursion_depth`. The wire validator (`:2133-2176`) only checks `1 + dependencies <= work_units` and request bytes `<= allocation_bytes`. The function itself must honor `budgets.work_units`.
- Resume: incremental mode requires `previous_state` (`:2144-2147`). A service returns `complete: false` plus its cursor in `canonical_payload`. The caller re-invokes with `previous_state` set. Resume is therefore explicit state threading, not a framework job checkpoint, on this path.
- Methods: `infer` (`:1673`, errors `artifact-inference.context-required` for contextual services, `:1676`), `infer_with_context` (`:1681`, shared `&dyn Any`, not `&mut`), `executable_identity` (`:1666`).
- Registry: process-global `OnceLock<RwLock<ArtifactInferenceServiceRegistry>>` (`:1775-1779`). Lookup `artifact_inference_service(kind, schema)` (`:1813`) returns a `Copy` handle. Registration `register_artifact_inference_services` (`:1804`) after `preflight_artifact_inference_services` (`:1786`). Declaration wiring: `inference_services` builder (`:3292`), assembly preflight (`:41554`) and registration (`:41654`).
- Cancellation registry: `cancel_artifact_inference` (`:2104`) only sees ids begun by `begin_artifact_inference` (`:2117`), which is called **only** on the wire path (`:2251`). Direct in-run calls are not cancellable through this registry; they must observe `StepContext::is_cancelled` (see 2.5).

### 2.2 Wire entry points and where the context comes from

- `wire_artifact_infer(request)` (`:2214`) uses the global registry and no context. Calls `wire_artifact_infer_from_context(..., None)` (`:2229-2245`).
- For a contextual service with no supplied context, the gateway calls `crate::plugin_runtime::with_extension_inference_context` (`:2240-2241`, body `:47765-47774`). That reads the **extension bundle's** resource owner (`:47772`, `owner.inference_context()`). A plugin artifact has no extension bundle, so the callback receives `None` and the service fails with `artifact-inference.context-required` (`:1676`).
- The only production caller of `wire_artifact_infer_with_context` is the extension retirement owner (`OSMOD/🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs:114-115`). The brep flow extension is the one production contextual service (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/🦀️.rs:36` `new_contextual`, `…/📐️brep/🦀️.rs:2162` `infer_with_context(&request, &self.context)`).
- Plugin-side entry: `plugin_wire_artifact_infer` (`:42702-42706`) calls `plugin.wire_artifact_infer` (`:37322-37323`), which is `wire_artifact_infer_from(self.runtime.inference_services(), …)` (context `None`). Same failure as above.
- The `semio.infer` job calls the global `crate::app::wire_artifact_infer` (`IJOB:233`) when no ActionBus factory exists (`IJOB:212-222`).

Conclusion: a plugin-owned **contextual** service cannot be executed through any gateway today. It can only be executed by code that already holds the context, which is the path this design uses (section 4).

### 2.3 Plugin hooks (`OSMOD/🔌️plugin/🦀️.rs`, `ArtifactApp` trait)

- `mounted_job_maintenance_step(instance_id, max_items, max_bytes)` (`:14648`), static, default `Complete`.
- `register_tool_job_factories(&mut ArtifactToolFactoryRegistry<Self>)` (`:14670`), default empty.
- `build_tool_run_job(ToolRunJobRequest<Self>) -> Result<Option<ToolRunJob>, Fault>` (`:14674`). **Associated function, no `self`.** The job must reach instance state through `request.instance_owner`.
- `build_retargetable_tool_run_job(...) -> Result<Option<Box<dyn ToolRunRetargetableJob<Config>>>, Fault>` (`:14679`). Asked first; falls back to `build_tool_run_job` (`TR:1703-1706`).
- `build_instance_operation_owner()` (generation3d: `GEDIT:1965-1967`, returns `Generation3dInstanceOperationOwner::new()`). Puzzle precedent at `✏️s/🔌️plugins/🧩️puzzle/…/✏️editor/🦀️.rs:7341-7344`.
- `ArtifactInstanceOperationOwner: Any + Send` (`:16679`): `as_any_mut`, `maintenance_step`, `close_step`.
- `ArtifactInstanceOperationOwnerHandle` (`:16708`): `Arc<Mutex<Box<dyn ArtifactInstanceOperationOwner>>>`. `with_mut::<T, R>` (`:16721-16725`) uses `try_lock`. Busy yields `interactive-job.instance-owner-busy` (`:16722`); a type mismatch yields `interactive-job.instance-owner-type` (`:16723`).

### 2.4 Tool-run ledger (`TR`, framework types in `FTR`)

Framework types:

- `ToolRunIdentity { id, generation, base_revision }` (`FTR:78-82`).
- `ToolRunState { Starting, Running, Paused, Complete, Finalizing, Finalized, Aborting, Aborted, Faulted }` (`FTR:115-125`), labels via `ToolRunLabel` (`FTR:163`).
- `ToolRunSlot { run, generation, state }` (`FTR:180`).
- `ToolRunEvent`: `Start`, `JobAdmitted`, `Pause`, `Resume`, `Step`, `JobComplete`, `JobFault`, `SettingsChanged`, `BaseChanged`, `Finalize`, `PublicationComplete`, `RevalidationConflicts`, `StoreRejected`, `Abort{publishing}`, `AbortComplete`, `Dismiss`, `Closed` (`FTR:188-205`).
- `ToolRunTick { identity, sequence, progress: Option<ToolRunProgress>, steps, trace, append_ops, append_entities, retract_to, payload: Option<Vec<u8>> }` (`FTR:1076-1088`). `payload` is "the latest intermediate result", read back by windows as `ToolRunView::payload` (`FTR:1085-1087`, `TR:201-208`).
- `ToolRunDefinition` (`FTR:1901`): `mutating`, `rebase: ToolRunRebasePolicy` (`Revalidate | Restart | Freeze`, `FTR:1797-1801`), `reconfigure` (`Resume | Restart`, `FTR:1807-1810`), `unit`, `stages`, `counters`, `reasons`, `trace`, `run_job: JobKindId` (`FTR:1910`), optional `revalidate_job`, `settings` reads, `windows: Vec<String>` (window kinds refreshed on every tick, `FTR` doc above the field), `member`. **`run_job` is never read by the ledger** (grep of `TR` and `FTR`): the job is chosen only by the app hook. See gap G5.
- Reserved action ids (`FTR:2063`): start, pause, resume, step, abort, finalize, dismiss. Chords (`FTR:2065-2080`): start `mod+enter`, pause/resume `mod+alt+enter`, step `mod+alt+arrowright`, abort `mod+.`, finalize `mod+shift+enter`, dismiss `escape`.
- Size ceiling per tick: `TOOL_RUN_TICK_BYTES_MAX = 262_144` (`FTR:42`), enforced at `TR:2069-2072`.

Plugin ledger (`TR`):

- `ToolRunJobRequest<A>` (`TR:91-108`): `tool_id`, `definition`, `purpose: Run | Revalidate`, `identity`, `snapshot` (`Run` → `entry.base`, `TR:1686-1687`), `config`, `window_id`, `window_config`, `checkpoint: Option<&[u8]>`, `provisional`, `instance_owner: ArtifactInstanceOperationOwnerHandle`, `port: ToolRunJobPort`, `trace_keys`, `entity_marks`, `children`, `member_ops`.
- `ToolRunRetargetableJob<C>` (`TR:137-143`): `rebind(identity)`, `reconfigure(identity, config) -> bool`. `false` closes and rebuilds from checkpoint.
- `ToolRunJobPort` (`TR:150-196`): host effects queued by the job plus a wait/wake handle for jobs waiting on host round trips.
- `build_tool_run_job_slot` (`TR:1680-1707`): retargetable first, then plain.
- Step loop `step_tool_run_job` (`TR:2036-2110`):
  - Fuel: `INTERACTIVE_LANE_FUEL` normally, `1` when Paused (`TR:2043`). Wall budget `INTERACTIVE_LANE_WALL_US` (`StepBudget::from_duration`, `TR:2044`). Constants: `🧰️framework/🔨️modules/🧵️job/🦀️.rs:394-395` (`1_000` µs and `2_000_000` fuel).
  - `drive_step` runs one `InteractiveJob::step` under a watchdog (`🧵️job/🦀️.rs` doc above `drive_step`, ~`:1318`). The hard ceiling is `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 ms, re-exported `🧵️job/🦀️.rs:51`). Sustained overruns quarantine the job.
  - `PreviewReady(payload)` must decode as `ToolRunTick` (`TR:2068-2075`), then `apply_tick` (`TR:2075`).
  - `CheckpointReady(cp)` stores `entry.checkpoint` (`TR:2095-2096`), which a rebuilt job receives as `request.checkpoint` (`TR:1693`).
  - `Complete` → `complete_tool_run_job` (`TR:2100-2107`). `Cancelled`/`Fault` → fault or abort paths (`TR:2109` onward).

### 2.5 Job trait contract (`🧵️job/🦀️.rs`)

- `InteractiveJob` (`:1303`): `step(&mut self, cx: &mut StepContext) -> StepOutcome`, `begin_close`, `close_step(max_items, max_bytes)`, `terminal_is_empty`.
- `StepContext` (`:974`): `fuel_remaining()` (`:1036`), `consume_fuel(units)` (`:1042`), `should_yield()` (`:1050`), `is_cancelled()` (`:1059`, must be checked on entry and after each unit), `payload_from_bytes(stream, bytes)` (`:1104`).
- `StepOutcome` (`:1225-1232`): `Yield`, `PreviewReady(RetainedJobPayload)`, `CheckpointReady(Checkpoint)`, `Complete(CommitCandidate)`, `Cancelled`, `Fault(JobFault)`.
- `BoundedJob` (used by `semio.infer`, `IJOB:212`) is a different, fuel-priced state machine. Tool runs use `InteractiveJob`.

### 2.6 Existing run-job users

| User | Where | What it shows |
|---|---|---|
| generation3d `previewEval` run | `PREVRUN:449-645` (`PreviewEvalRunJob`), `GEDIT:2591-2597` (`build_tool_run_job`), `PREV:134-144` (definition from `🔣️.json`) | Interactive job that holds `ArtifactInstanceOperationOwnerHandle`, calls `owner.with_mut` per step, uses `port.dispatch` hop effects, `StepOutcome::PreviewReady(encoded ToolRunTick)` and `ToolRunTickWriter`. Interacts with `FlowEvalSession`, which is the code to replace. |
| puzzle3d fill run | `✏️s/🔌️plugins/🧩️puzzle/…/🧊️3d/…/✏️editor/🦀️.rs:7324-7334` (`build_tool_run_job`, `build_retargetable_tool_run_job` → `fill_tool::build_run_job`), `…/⏳️precompute/🪣️fill/🦀️.rs:2519-2527` | Best precedent for a **resumable, checkpointed, retargetable** run. One unit of fuel = one candidate verdict. `CheckpointReady` after each placement (`:2078`). `resume` with a new count, and silent replay on rebuild. |
| gismap, print chart | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs:174-200, 348`; `🧰️framework/🛍️products/📓️print/🦀️.rs:47, 60-68` | Pure services, whole-snapshot, no progress. The services exist for the gateway only. |
| gltf inference leaves | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs:106-117, 121-191, 194-209, 212-216` | Declaration with `.inferences` and `.inference_services`. Pure `ArtifactInference` fns that decode `canonical_payload` and run `Inference::infer` once. No budgets used, no resume. |
| flow brep geometry (contextual) | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/💡️inferences/📐️geometry/🦀️.rs:25-47` (contract and `new_contextual` at `:36`), `…/📐️brep/🦀️.rs:2162` | Only production contextual service. Its context is the extension owner. This is the template for a contextual service with a payload contract. |

### 2.7 Instance-owned state and cache

- generation3d owner: `Generation3dInstanceOperationOwner { eval_session: Option<FlowEvalSession>, run_link: PreviewEvalRunLink, gumball, closing }` (`GEDIT:179-186`), constructor `new()` (`:189-191`), `ArtifactInstanceOperationOwner` impl (`:241-…`), `PreviewEvalRunOwner` impl (`:234-239`). This is where the instance keeps one retained session across turns.
- Renders receive the same owner: `render_with_request_context(owner, body_key, doc, cfg, view_state, transient, interaction)` (`GEDIT:2608-2615`). It calls `owner.with_mut::<Generation3dInstanceOperationOwner, _>` (`:2624-2633`) and maps any fault to `PluginAssemblyError` (`:2634`).
- `InferenceCache` (`INF:150-156`): `config`, `entries: HashMap<DepHash, CacheEntry>`, `lru`, `used_bytes`, `stats`. Constructor `pub async fn new(config)` (`INF:167`). Config: `InferenceCacheConfig { enabled, budget_bytes (16 MiB default), persistence, record_stats }` (`INF:119-131`). `enabled` only matters if a caller passes `Some(cache)`.
- `InferenceCache::get` and `insert` are **private** (`INF:182`, `INF:200`). A plugin cannot read a value by `DepHash`. Values are reachable only through the `infer_field` driver's return value.
- Cache values are encoded as text through `crate::os_io::text::inferences::{encode, decode}` (`INF:249`). Large geometry values pay a text round trip on every store and load.
- Production users of `InferenceCache`: **none** outside tests. Every production `infer_field` call passes `None` (prior audit section 1.4, confirmed by grep: `infer_field_after_diff` and `InferenceCache::new` appear only in `INF` and test files).

### 2.8 Window-retained state

- `WindowTransientOwner` (`✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs:132-144`): state `Generation3dPreviewWindowTransient { preview_eval_text: Option<String> }` (`:11-13`), mutation `set-preview-eval` (`:46-49`), admission `preflight` by footprint (`:120-125`), `transfer` (`:127-130`).
- Read in render through `TransientView::window::<T>()` (`GEDIT:2617-2620`).
- Ceiling: `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES = 1_048_576` (`🏪️store/🦀️.rs:16988`), admissibility `is_admissible` (`:17017`), `for_ephemeral_item` (`:17040`).
- Conclusion: a window transient is mutation-fed, history-free, and capped at 1 MiB per item. It is the right place for small per-window selection or mode state. It is the wrong place for tessellated meshes (see gap N8).

### 2.9 The `semio.infer` job (`IJOB`)

- `job_infer` (`:212-222`): if `ActionBus::production()` contains `semio.infer/<inference_schema>` (`:217-218`), it admits `InteractiveInferenceJob`, else a `TwoPhaseBoundedJob` calling the global wire (`:221`, `:233`).
- Interactive path: `dispatch_wire` into a worker session (`:383-387`), budget `fuel_per_step = work_units.clamp(1, USER_VISIBLE_LANE_FUEL)`, `step_budget_us = USER_VISIBLE_LANE_WALL_US` (`:391-400`). Progress from `Yield`/`PreviewReady` (`:428-536`). Resume state only through `checkpoint()` (`:644-646`).
- Result: `encode_result` (`:672-712`) rebuilds the wire result with `complete: true` (`:707`), `validity: "valid"` (`:705`), `quality: "exact"` (`:706`), `diagnostics: Vec::new()` (`:703`), `actual_cache_mode = requested` (`:708`). Partial results from a service are reported as complete. See gap G3.
- Host entry: `OSMOD/🔌️plugin/🖥️host/🦀️.rs:5967-5968` (`run_job_on_worker("semio.infer", …)`).
- MCP entry: `OSMOD/🌉️mcp/🏠️workspace/🦀️.rs:1702` (`route.router.infer`).

### 2.10 Shell inference port

- `ShellCommand::SetDocumentInferencePort` / `ClearDocumentInferencePort` (`OSMOD/🖥️shell/🦀️.rs:322-328`) store an `InferencePortStatus` per document (`🖥️shell/🧬️schema/🦀️.rs:615`). It is a registration status for the agent or MCP path. It carries no results to windows.

---

## 3. Prior audit (`📓️wsx-inference-system-2026-10-07.md`) checked

| Prior claim | Status |
|---|---|
| Every production `infer_field` call passes `None`; `infer_field_after_diff` has no production callers (D6) | Confirmed by grep. |
| generation3d `topology` is descriptor-only, no `.inference_services` (4.2) | Confirmed: `GEN/🦀️.rs:150` has `.inferences(...)` only. |
| Only five executable services in the tree (3.? / section 1.3) | Not re-counted. Not needed for the design. |
| Contextual services are reachable from the gateway | **Contradicted** by section 2.2 above. Only the extension owner supplies context. |
| `semio.infer` interactive path forwards results faithfully | **Contradicted** by section 2.9 above. Completeness metadata is hard-coded. |
| CAD tessellates in-process without budgets (3.4) | Not re-read in this pass. Precedent cited from prior audit, **unverified** here. |

---

## 4. Recommended design

### 4.1 Decision

Use the tool-run run job as the execution host. Reasons:

1. It already gives progress (`ToolRunProgress`, counters, step ring), cancel (Abort, Pause, Step), resume (`CheckpointReady` to `request.checkpoint`), and window refresh (`ToolRunDefinition.windows`), with the chord contract the user already has.
2. It already holds the instance owner (`ToolRunJobRequest.instance_owner`, `TR:102`), so it can call a contextual service with the owner's cache.
3. The `semio.infer` route needs an `ArtifactOwnedToolJobFactory` (`ArtifactToolFactoryRegistry::register`, `:16804-16854`, Migrated classification, publication contracts), runs on a worker without the window contract, and loses result metadata (G3). Keep it for MCP and agent access only, after G2 and G3 are fixed.

### 4.2 Types (generation3d, proposed)

- `GeometryCursor { plan_hash: [u8; 32], next: u32 }`, encoded into `previous_state` and `CheckpointReady`. Fixed size, same pattern as the puzzle fill checkpoint (`🧬️schema/🦀️.rs:790`, 68 bytes).
- `GeometryHandle { dep: DepHash, quality: String, fault: Option<WidgetFault> }`. This is the `InferredField::Value`. It must stay small, because the cache text-encodes it (`INF:249`). Do not put meshes or shapes in the value.
- `GeometryState` (owned by `Generation3dInstanceOperationOwner`, behind `RefCell` because the contextual service receives `&dyn Any`):
  - `cache: InferenceCache` (enabled, `budget_bytes` sized for geometry, `record_stats: true`).
  - `base: Option<(u64 revision, Vec<u8> canonical, Generation3dSnapshot decoded)>`. The canonical snapshot bytes are encoded once per revision. The decoded snapshot is reused across steps, so a step never decodes the whole document.
  - `handles: BTreeMap<WidgetId, GeometryHandle>`.
  - `payloads: BTreeMap<DepHash, GeometryPayload>` (typed shape and mesh values, owner-side, not cache-side).
  - `meshes: BTreeMap<DepHash, TessellatedMesh>` (tessellation stage output, keyed by DepHash, AD5).
- `GeometryRunJob<O: GeometryRunOwner>` replaces `PreviewEvalRunJob` (`PREVRUN:449`). Holds `owner: ArtifactInstanceOperationOwnerHandle`, `port: ToolRunJobPort`, `identity`, `service: ArtifactInferenceService` (resolved at admission via `artifact_inference_service(kind, schema)`, `OSMOD` `:1813`, so the exact registered executable runs), `cursor: Option<GeometryCursor>`, `writer: ToolRunTickWriter`, `stage`, `closing`, `released`.
- Contextual service `geometry_inference_service() -> ArtifactInferenceService` built with `new_contextual`, metadata `owner: "procedural-generation3d"`, `artifact_kind: "s.procedural.generation3d"`, `inference_schema: "s.procedural.generation3d.geometry"`, `payload: Some(ArtifactInferencePayloadContract { progress_unit: "widget-step", input_schema, output_schema, artifact_binding: None, commit: None })` (pattern: flow brep `…/📐️geometry/🦀️.rs:25-47`). Function `infer_geometry_contextual(request, context)`:
  1. `context.downcast_ref::<Generation3dInstanceOperationOwner>()` (refuse otherwise with `generation3d.geometry.context`).
  2. Refresh `base` if `request.canonical_payload` changed (compare hash, decode once).
  3. Run one bounded stepper call with `budget = request.budgets.work_units` (see gap G1).
  4. Return `ArtifactInferenceExecution { canonical_payload: encoded GeometryCursor, complete, validity, quality, actual_cache_mode: request.requested_cache_mode.clone(), diagnostics }`.

### 4.3 Call sites and hooks

| # | Site | Change |
|---|---|---|
| C1 | `GEN/🦀️.rs:150` | Add `.inference_services([geometry_inference_service()])` beside `.inferences(...)`. Preflight and registration run at `OSMOD/🔌️plugin/🦀️.rs:41554` and `:41654`. |
| C2 | `GEDIT:179-191` | Add `geometry: RefCell<GeometryState>` to `Generation3dInstanceOperationOwner`. Build the cache in `new()` with `InferenceCache::new(InferenceCacheConfig { enabled: true, .. }).await`, resolved with `semio_framework_async::poll::resolve_ready` (pattern `PREV:143`). |
| C3 | `GEDIT:1965-1967` | Unchanged call. `new()` now also builds the geometry state. |
| C4 | `GEDIT:2591-2597` | Replace `PreviewEvalRunJob::new(...)` with `GeometryRunJob::new(request.instance_owner, request.port, request.identity, request.checkpoint, request.snapshot)`. Keep the `request.tool_id == PREVIEW_EVAL_TOOL_ID` gate and the `Run` purpose gate. Consider `build_retargetable_tool_run_job` (`OSMOD:14679`) for settings and base changes (C6). |
| C5 | `PREVRUN:552-600` (`step`) | New step: `if closing || cx.is_cancelled() { Cancelled }`; `owner.with_mut::<O>(|held| service.infer_with_context(&req, &*held))` with `budgets.work_units = cx.fuel_remaining().clamp(1, STEP)` and `previous_state = cursor`. Then `consume_fuel`, `PreviewReady(tick)` and `CheckpointReady(cursor)`. On busy (`interactive-job.instance-owner-busy`, `OSMOD:16722`) return `Yield`, as today (`PREVRUN:579`). |
| C6 | `TR:1703-1706` (framework hook) | Use `build_retargetable_tool_run_job` so a base or config change retargets with `rebind`/`reconfigure` (`TR:137-143`). Otherwise `rebase: restart` rebuilds from the warm owner cache, which is still incremental. Start with plain jobs. |
| C7 | `GEDIT:2608-2635` (`render_with_request_context`) | Replace the `preview_eval_text`/flow-session read with a read of `owner.geometry.handles` for `preview` widgets and `owner.geometry.meshes` by DepHash. Keep `owner.with_mut` and the window transient for selection and mode only. On a busy owner, render the last retained values, not a fault (gap N7). |
| C8 | `PREV:134-144`, `PREVRUN` record `🔣️.json` | Keep tool id `previewEval`, `mutating: false`, `rebase: restart`, `reconfigure: restart`, the `windows` list. Rename stages to `geometry` and `tessellate` only if the renderer is updated together. Keep counter ids and reason codes stable (section 5). |
| C9 | `GEN` widget compute | Each widget kind computes inside the stepper. A kernel call that cannot finish within the 8 ms ceiling must itself be an `InteractiveJob` nested in the run job, as in the puzzle fill (`…/⏳️precompute/🪣️fill/🦀️.rs`). Do not call one-shot kernel operations from the stepper. |

### 4.4 Data flow

```
user: Mod+Enter (toolRunStart toolId=previewEval windowId=<preview>)
  |
  v
ToolRunLedger (TR)  Start -> Starting -> admit_tool_run_job       [TR:1962, :2007]
  |  build_tool_run_job_slot -> ArtifactApp hook                     [TR:1680-1707]
  |     request.instance_owner  (Generation3dInstanceOperationOwner handle)
  |     request.snapshot        (base, Run)  request.checkpoint (resume)
  v
GeometryRunJob (InteractiveJob), stepped by drive_step               [TR:2050-2061]
  |  fuel = INTERACTIVE_LANE_FUEL (1 when Paused), wall 1 ms           [TR:2043-2044]
  |  step(cx):
  |    cx.is_cancelled()  -> StepOutcome::Cancelled   (Abort, mod+.)  [StepContext:1059]
  |    owner.with_mut(|held| service.infer_with_context(req, &*held))  [OSMOD:16721, :1681]
  |        -> infer_geometry_contextual
  |             owner.geometry.cache  + plan + cursor
  |             for each widget in order (budgeted by work_units):
  |                DepHash = root/chain(kind, params, incoming wiring)
  |                cache hit  -> handle reused
  |                miss       -> compute kind (pure) -> payloads/meshes in owner
  |             returns ArtifactInferenceExecution{complete, cursor}
  |    consume_fuel(granted)
  |    PreviewReady(ToolRunTick{progress{stage, completed, total, counters}, steps})   [TR:2068-2075]
  |    CheckpointReady(GeometryCursor)  -> entry.checkpoint                 [TR:2095-2096]
  |    Complete when complete -> complete_tool_run_job                      [TR:2100-2107]
  v
ToolRunView{state, progress, payload} -> ArtifactView::tool_run()
  |  windows in ToolRunDefinition.windows refresh on every tick             [FTR:1920]
  v
render_with_request_context (GEDIT:2608) each refresh:
  owner.with_mut -> handles(preview widgets) + meshes(DepHash) -> scene
  window transient (<= 1 MiB) : selection and mode only                   [store:16988]

Mod+. (Abort) -> ToolRunEvent::Abort -> job.begin_close -> closing=true
  -> next step returns Cancelled -> close_step releases owner link         [PREVRUN:602-640]
Framework closes the job -> rebuild from entry.checkpoint (request.checkpoint)
  -> cursor restored; owner cache still warm, so only dirty DepHashes recompute
Document edit -> rebase: restart -> new run, new base revision -> base refreshed once
```

### 4.5 Progress and cancel UI contract to preserve

- Start, pause, resume, step, abort, finalize, dismiss actions and chords: `FTR:2063-2080`. Pause is a single-step mode with fuel 1 (`TR:2039, :2043`).
- `ToolRunState` labels and the `ToolRunView` the pill reads: `FTR:115-163`, `TR:201-216`. `ToolRunProgress { stage, completed, total, counters, units_per_second, conflicts, steps }` (`PREVRUN:510-531` shows how the current job fills it).
- Declared record ids the renderer and tests assert equal to the job's enums (`PREV` doc and `🧪️tests`): stage index and id, counter index and id (`PreviewEvalRunCounter`, `PREV:48-74`), reason codes (`PreviewEvalRunReason`, `PREV:76-125`), `ToolRunVerdict` per reason. Keep codes 0 to 11 stable.
- Cancel latency: bounded by one step, which is at most the 1 ms wall budget plus the 8 ms watchdog ceiling (`🧵️job/🦀️.rs:394`, `INTERACTIVE_STEP_CEILING_US`). A sustained overrun quarantines the job, so each widget compute must fit.
- Retry on busy: a busy owner returns `Yield`, not a fault (`PREVRUN:579`). Keep this for the run job.

---

## 5. Framework gaps and minimal domain-neutral additions

Ordered by how strictly they block the design.

**G1 (required). The `InferredField` driver cannot run budgeted or resumable.** `infer_field` (`INF:254-284`) walks the whole plan with no fuel and no cursor. `compute` is infallible (`INF:112`).

Minimal addition next to `infer_field` in `INF`:
- `pub struct InferenceCursor<K> { plan: Vec<InferenceStep<K>>, next: usize, hashes: BTreeMap<K, DepHash> }`. Values stay in the caller's map.
- `pub fn infer_field_step<P, F>(snapshot: &P, cache: Option<&mut InferenceCache>, cursor: &mut InferenceCursor<F::Key>, values: &mut BTreeMap<F::Key, F::Value>, fuel: usize) -> bool` with the body of the loop at `INF:259-281`, stopping after `fuel` computes.
- `infer_field` becomes `infer_field_step` with `fuel = usize::MAX`. The existing cache-transparency tests stay unchanged.

**G2 (required for MCP and agent parity, not for the UI path). Contextual services cannot be executed through any gateway by a plugin.** `wire_artifact_infer_from_context` supplies context only from the extension bundle (`OSMOD:2240-2241`, `:47765-47774`). Minimal addition: `plugin_wire_artifact_infer` (`OSMOD:42702-42706`) runs `wire_artifact_infer_with_context(request, owner)` inside `ArtifactInstanceOperationOwnerHandle::with_mut::<O>` (`OSMOD:16721`). The owner type comes from the app's own `ArtifactApp`. This is domain-neutral, because the handle is already generic.

**G3 (defect). The interactive `semio.infer` result hard-codes completeness.** `encode_result` (`IJOB:672-712`) sets `complete: true`, `validity: "valid"`, `quality: "exact"`, empty diagnostics, and `actual_cache_mode` from the request. A partial geometry step is reported as complete, and `previous_state` is never forwarded. Minimal fix: have the worker output carry the full `ArtifactInferenceExecution` metadata and decode it in `encode_result`, instead of rebuilding the result from the request.

**G4 (constraint, not a gap). Mesh-scale data cannot ride a tick or a window transient.** Tick payload cap 256 KiB (`FTR:42`, enforced `TR:2069-2072`). Window transient cap 1 MiB per item (`store:16988`, `is_admissible` `store:17017`). Preview cap 1 MiB (`IJOB:13`). AD5's "retained per window keyed by DepHash" must be **instance-owned** (section 4.2 `meshes`), with the window keeping only small state. This is a design constraint, not a framework change.

**G5 (metadata drift). `ToolRunDefinition.run_job` is unused.** The ledger never reads it (`FTR:1910`); the job is chosen by the hook (`TR:1703-1706`). Either remove it from the record or document it as informational. No runtime effect.

**G6 (minimal addition). The service cannot report work consumed.** `ArtifactInferenceExecution` (`OSMOD:1620-1627`) has no `work_units` field, so the job cannot charge fuel from the actual work. Minimal addition: `pub work_units: u64` on `ArtifactInferenceExecution`, validated against `budgets.work_units` in `validate_inference_execution` (`OSMOD:2178-2198`). Without it, the job charges the granted budget for every step, which is safe but over-charges.

**G7 (framework-level cancel nuance). In-run calls are not in the cancellation registry.** `begin_artifact_inference` runs only on the wire path (`OSMOD:2251`). `cancel_artifact_inference` therefore cannot cancel a direct in-run call. The run job must rely on `cx.is_cancelled()` (`StepContext:1059`). No change required. The design must not depend on the registry for UI cancel.

**G8 (cache API). `InferenceCache` has no keyed read.** `get` and `insert` are private (`INF:182`, `:200`). The design avoids needing a keyed read by keeping typed payloads in the owner (section 4.2). If a plugin ever needs cache reads by DepHash, expose `pub fn get`.

---

## 6. Other findings and risks (new in this pass)

- **N1. Contextual service unreachable from the gateway.** Not in the prior audit. See G2.
- **N2. Interactive `semio.infer` misreports completeness.** See G3.
- **N3. `InferenceCache` is opaque to plugins.** See G8.
- **N4. The `ArtifactInferenceExecution` contract has no work accounting.** See G6.
- **N5. `InferredField::Value` as specified in AD2 would put `WidgetEvaluation` with meshes into a text-encoded cache** (`INF:249`). Use a small handle, as in 4.2.
- **N6. `run_job` is dead metadata.** See G5.
- **N7. Render can fault while a job holds the owner lock (unverified at runtime).** `with_mut` uses `try_lock` (`OSMOD:16722`). Render maps any fault to `PluginAssemblyError` (`GEDIT:2634`). A job step that holds the lock during render would fail the body. The step holds the lock only for one bounded call, and the comment at `OSMOD:16727-16732` says the same race is already accepted. Still, render should fall back to the last retained values on busy instead of failing. **Needs a runtime test.**
- **N8. AD5 says "retained per window keyed by DepHash".** A window transient is capped at 1 MiB per item (`store:16988`), so tessellated meshes must stay in the instance owner.
- **N9. Single widget compute must stay under the watchdog.** A kernel call that exceeds 8 ms cannot be cut inside the stepper and risks quarantine. Heavy kernel operations need their own resumable `InteractiveJob` (C9). This is the main schedule risk for AD3 and AD4.
- **N10. Driver order sensitivity.** `infer_field` silently drops parents missing from `hashes` (`INF:260, :268, :275`, prior D2). A resumable driver must not inherit this. Make a missing parent a loud error.

---

## 7. Test plan for this path (not run)

Language-agnostic fixtures, then third-party oracles, as AGENTS requires:

1. Budget invariance: the same final `handles` and `meshes` for step sizes 1, 7 and "run to completion" (G1).
2. Cancel mid-run: no partial cache insert and no partial `handles` entry (the stepper commits per widget).
3. Resume: a run rebuilt from `entry.checkpoint` equals a fresh run.
4. Warm versus cold: identical outputs, and a second run on an unchanged document computes zero widgets.
5. Dirty propagation: changing one widget recomputes only its descendants (matches the existing cache laws in `INF` tests).
6. Abort: `Abort` during `Running` and during `Paused` ends `Aborted`, releases the owner link (`PREVRUN:602-640`), and leaves the owner reusable.
7. Busy owner: a render during a step returns the last retained values, not a fault (N7).
8. Third-party oracle for geometry values (three.js or manifold-3d, as in the wave-3 plan of the brep-mesh note).

Each test needs a logged runtime confirmation (AGENTS: no claim without console or runtime evidence).

---

## 8. Open items (unverified)

- No build, test or app run (read-only audit).
- Whether the render path actually runs while a run step holds the owner lock on another thread (N7).
- The exact fuel the framework grants per drive under load (`INTERACTIVE_LANE_FUEL = 2_000_000` at `🧵️job/🦀️.rs:395` is a ceiling, not a measured step size).
- CAD in-process tessellation precedent (`CADINF/🦀️.rs:630, 647`) was taken from the prior audit and not re-read here.
- The MCP route's inference call (`OSMOD/🌉️mcp/🏠️workspace/🦀️.rs:1702`) was read only at that line. The surrounding route was not traced.
