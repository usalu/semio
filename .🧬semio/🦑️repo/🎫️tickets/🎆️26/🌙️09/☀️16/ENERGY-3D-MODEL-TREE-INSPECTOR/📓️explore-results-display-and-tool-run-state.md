# Explore: results display (fem2d/fem3d) and reading a finished tool run's results (energy)

Goal recap: make the 🔋️energy editor paint walls by transmission loss after `mod+enter` finishes the
`energySimulation` ToolRun. This note is exhaustive on facts found; see §4 for the concrete recipe.

## 0. Headline findings

1. **fem2d/fem3d never use a ToolRun for results.** Both solve synchronously inside `render()`
   (fem2d caches per-revision in a thread-local; fem3d re-solves every render, "no cache, no
   `RunAnalysis` operation"). Their colour-ramp/legend pattern is directly reusable, but their
   dirty-scope story (an explicit config-mutating command) does **not** apply to an async run.
2. **The energy simulation is a genuine async `ToolRun`** (`EnergySimulationRunJob: InteractiveJob`).
   Its window (`⚡️simulation`) today shows only `run.state`/`run.identity`/settings — it never reads
   `ToolRunView.payload` and explicitly disclaims keeping "run state of its own".
3. **The numerical `Results` (meters, time series, per-surface physics) are computed and then
   thrown away for a read-only run.** `EnergyJobAuthority::take_results() -> Option<Results>` exists,
   and `StepOutcome::Complete(CommitCandidate { state, output })` carries the final `output`
   (`RetainedJobPayload`, stream `CommitOutput`) — but the **framework** driver
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs:1571-1577`) closes/frees that
   candidate's payload pages (`close_step_outcome`) **without ever reading them**, for every tool run,
   mutating or not. Nothing plugs `EnergyJob`'s numeric `Results` into anything a window can see.
4. **The only channel a window can read after Finalized is `ToolRunView.payload: Option<Arc<[u8]>>`**
   — the bytes the plugin itself put into the *tick* via `ToolRunTickWriter::payload(Vec<u8>)`
   (`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:1283`). Energy's run job never calls `.payload(...)` today,
   so `ToolRunView.payload` is always `None` for `energySimulation`.
5. **Per-tick refresh of a plugin window is opt-in and energy has not opted in.**
   `ToolRunDefinition.windows: Vec<String>` ("window kind ids whose bodies render this run's state
   … every tick refreshes them next to the ToolRun panel, and no other window",
   `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:1899-1903`) feeds `ToolRunLedger::dirty_scope()`
   (`⏯️tool-run/🦀️.rs` in the OS plugin module, lines 889-893). Energy's
   `energy_simulation_run_definition()` sets `windows: Vec::new()` (line 196 of the simulation-session
   file) — so **no plugin window, including a future walls window, is refreshed by a tick today.**
6. **There is no 3D window in the energy editor at all yet.** `grep` for `world3d_scene`/
   `world_3d_surface`/`World3dScene` across `✏️s/🔌️plugins/🔋️energy` returns nothing. The only editor
   windows are `🌳️structure` (tree), `📊️zones` (table) and `⚡️simulation` (tree/BlockList). "Walls" are
   surfaces (`crate::model::Surface { vertices_m: Vec<[f64;3]>, .. }`) with no geometry window today.
7. **W3-1's G1 is resolved in code; G2's mechanism landed but energy hasn't used it.** The wave doc
   (`.../INTERACTIVE-TOOLS-VISIBLE-PROCESS/📓️wave-W3-1.md` §7) lists G1 ("add `progress` to
   `ToolRunView`") and G2 ("tick dirty_scope covers only the panel and trace windows") as open. Today's
   `ToolRunView` already carries `progress: ToolRunProgress` and `payload: Option<Arc<[u8]>>` (G1 done),
   and `dirty_scope()` already unions `entry.window_bodies` — populated from
   `ToolRunDefinition.windows` — with the panel body key (G2's mechanism done). What's still true is
   that **energy's own `ToolRunDefinition` declares no windows**, so the mechanism is unused by energy.

---

## 1. The fem result-display pipeline, end to end

### 1.1 Field/mode selector → command → window-config store

Panel: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📊️results/🦀️.rs`
- Builds `select`/`slider`/`number`/`button` rows tagged with `{field, value, windowId}` (see
  `control_args`, lines ~44-51). Every control names the **field** it owns in `{field, value}` because
  "the host merges a control's own scalar under the single key `value`" (module doc, top of file).
- `windowId` is carried explicitly on every control because "a panel projection carries no `window_id`
  of its own" — the puzzle3d Settings-panel rule, reused verbatim.
- Actions dispatched: `DISPLAY_ACTION = "setResultDisplay"`, `PLAYBACK_ACTION = "setResultAnimation"`,
  `ANALYSIS_ACTION = "setAnalysisSettings"`.

Command: `.../✏️editor/🎮️commands/👁️set-result-display/🦀️.rs`
- `SetResultDisplay { source_id, mode, mode_index, field, value, window_id }` (dsl record).
- `handle_window` resolves the addressed window via
  `results::config::addressed_window_id(cfg, view, payload.window_id.as_deref())`, reads
  `results::config::current(cfg)`, applies either the named `field` (`apply_field`) or the whole
  triple, then emits:
  ```rust
  Emit {
      window_config_mutations: vec![results::config::addressed_to(&window_id, next)],
      ui_scope: UiDirtyScope::Partial {
          window_bodies: vec![results::BODY_KEY.to_owned()],
          panel_bodies: vec![crate::editor::fem2d::panels::results::BODY_KEY.to_owned()],
          utilities: false, tools: false, engagements: false, measures: false, labels: false,
      },
      ..Default::default()
  }
  ```
  This is the **whole re-render trigger**: an explicit `UiDirtyScope::Partial` naming exactly the
  results window body key and the results panel body key. No ToolRun, no tick — the mutation is
  synchronous and the dirty scope is authored by the command itself.

### 1.2 Where the choice is stored

`.../🎭️modes/✏️edit/🪟️windows/📊️results/🎚️config/🦀️.rs`:
- `Fem2dResultsWindowConfig { camera, result_source_id: Option<String>, result_mode: ResultMode,
  result_mode_index: u32, animation: Fem2dResultsAnimation }` — a **whole-record window config**,
  registered as a `WindowConfigOwner` (`Fem2dResultsWindowConfigOwner`, `MAXIMUM_PUBLICATION_BYTES:
  16_384`), with DSL/pack codecs and a `Snapshot`-only mutation (`Fem2dResultsWindowConfigMutation`).
- `current`/`captured` read the config already bound to the render's `ConfigView` (per-window-instance
  partition, captured by `WindowConfigOwnerRegistry::capture`); `addressed_window_id` is the guard that
  refuses to let a control retune a pane other than the one it captured (or, for a panel, other than
  the *focused* pane).
- fem3d's twin file is byte-for-byte analogous (`Fem3dResultsWindowConfig`), same
  `WindowConfigOwner`/`Snapshot` pattern.

### 1.3 Render reads config → solves → colours → legend

fem2d window: `.../🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`
- `render(doc, display, camera, window, interaction, operation, window_instance_id, active_utility)`
  dispatches on `display.mode` (`Static`/`Modal(n)`/`Buckling(n)`) built from
  `ResultMode::display(index)`.
- **Results cache** (thread-local, keyed by `(app_instance_id, canonical_base_revision)` off
  `AppRenderOperationContext`): `Fem2dResultsCache { statics: Option<HashMap<String,StaticResult>>,
  modes: Vec<(ModeKey, ModeValues)> }`, evicting the whole entry on a different key and capping modes
  at `RESULTS_CACHE_MODES = 8`. `with_static_results`/`with_mode_values` solve at most once per
  revision; `results_solve_count()`/`reset_results_cache()` exist purely so a law can assert the cache
  actually avoided a re-solve.
- **Static view** (`static_layers`, ~line 300+): undeformed structure, deformed-shape polyline scaled
  by `doc.analysis.deformation_scale * amplitude`, reaction labels (de-overlapped by
  `place_reaction_label`), beam moment diagrams, and:
  - **Stress contour**: `crate::fem2d_engine::mesh_preview::fem2d_nodal_von_mises(doc, case_id)` gives
    a nodal-averaged von-Mises map; `fem2d_region_mesh_triangles(doc)` gives mesh triangles with
    per-vertex node ids. For each triangle, min/max across its three nodal values plus the **global**
    min/max across all valued triangles bound `n_bands = VON_MISES_BANDS.len()` (8) equal-width
    thresholds; each band is produced by **Sutherland-Hodgman clipping** the triangle's
    linearly-interpolated scalar field against the two threshold half-planes (`clip_by_value`,
    `interpolate_at_value`) — this is genuine marching-triangle banding, not a per-vertex gradient.
  - Legend: `von_mises_legend_layers(min, max)` — 8 triangle-pair swatches (`filled_triangle_layer`
    ×2 per band) plus two text layers for the numeric min/max, placed near the canvas origin (Canvas2d
    layers, not a separate UI element).
- Output: `crate::app_surface::canvas_2d_surface(BODY_KEY, &Canvas2dScene { camera_x, camera_y, zoom,
  layers_json, snapshot: None, tool_run_trace: None, lanes: Vec::new() })` — the layers are raw
  Canvas2d JSON (`fill.color: [r,g,b,a]`), assembled through `semio_framework_plugin::scene_surface`.

fem3d window: `.../🗿️artifacts/🧊️3d/.../🪟️windows/📊️results/🦀️.rs`
- Same dispatch shape (`config_result_display` projects `ResultDisplay` from
  `Fem3dResultsWindowConfig`), but **no results cache** — `fem3d_solve_all`/`fem3d_modal_mode_values`/
  `fem3d_buckling_mode_values` run fresh every render (module doc: "v0 design ... no cache").
- Colouring is **per-vertex mesh colour**, not banded polygons:
  `fem3d_solid_mesh_entries` (`.../✏️editor/🦀️.rs:627-680`) builds each `FemSolid`'s boundary-triangle
  mesh with a `colors: [r,g,b, r,g,b, ...]` array (one RGB triple per emitted vertex, duplicated per
  triangle for flat shading). `vertex_color(idx)` looks up `nodal_stress.get(node_id)` and calls
  `von_mises_color(value, min, max)` → `hex_to_rgb01`. `(min, max)` are the global min/max across ALL
  solids' nodal values (or `(0.0, 1.0)` with a neutral grey `(0.78, 0.78, 0.8)` fallback when
  `nodal_stress` is `None`). No legend layer exists for fem3d — the doc comment for
  `fem3d_solid_mesh_entries` names the React renderer's `PaintTexturedMesh` as the vertex-colour
  consumer.
- Output: `crate::app_surface::world_3d_surface(FEM3D_BODY_RESULTS, &world3d_scene(camera_json,
  meshes_json, instances_json, selection_json, &WorldSunConfig::default()))`.

### 1.4 Colour-ramp code and framework colour helpers

Shared helper module: `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs` (used by **both** fem2d_ui and
fem3d_ui — "non-constitutional... shared code used by ≥2 apps"):
```rust
pub const VON_MISES_BANDS: [&str; 8] =
    ["#1d4ed8","#2563eb","#0ea5e9","#22c55e","#eab308","#f97316","#ef4444","#b91c1c"]; // blue→green→yellow→red
pub const MODE_SHAPE_AMPLITUDE_RATIO: f64 = 0.1;
pub fn hex_to_rgb01(hex: &str) -> (f64, f64, f64)
pub fn von_mises_color(value: f64, min: f64, max: f64) -> &'static str  // clamp+round-to-nearest-band
pub fn normalize_mode_shape(disp_map: &mut HashMap<String,[f64;6]>)     // unit-peak normalize
pub struct ResultDisplay { source_id: Option<String>, mode: DisplayMode }
pub enum DisplayMode { Static, Modal(usize), Buckling(usize) }
pub fn result_display_action_args() -> Vec<ActionArgDef>  // shared "sourceId/mode/modeIndex" arg decl
```
**There is no framework-level (Rust, plugin-callable) colour-ramp/palette helper.** I checked
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette` — it holds only `🎨️.css` and a `.cs` (C#) file, i.e.
CSS custom properties and a C# binding, neither reachable from a wasm plugin's Rust code. fem hand-rolled
its own 8-stop hex ramp and hex→rgb01 conversion in `app-surface` precisely because no such Rust helper
exists to reuse. **Energy would face the same gap** — it will need its own ramp constant (or literally
reuse `crate::app_surface::{VON_MISES_BANDS, von_mises_color, hex_to_rgb01}` by depending on the fem
engine crate, or — more likely — copy the same 8 lines, since cross-plugin deps between fem and energy
are not established).

---

## 2. What the energy run publishes at Finalized, and how a window can read it

### 2.1 The run job and what it actually ticks

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs`
(1293 lines):
- `ENERGY_SIMULATION_RUN_JOB_KIND = "energy.simulation.run"`,
  `ENERGY_SIMULATION_RUN_SCHEMA = "energy.simulation.run.v1"` (lines 18-19).
- `energy_simulation_run_definition()` (line 183): `mutating: false`, `rebase: Restart`,
  `reconfigure: Restart`, `trace: None`, `settings: {config: [zoneTimestepMinutes,
  systemTimestepMinutes, warmupDays]}`, **`windows: Vec::new()`** (default via `..Default` not shown
  but structurally empty — confirmed no `windows:` field is set, so it defaults to `Vec::new()`, and
  the wave doc explicitly says the same window never got wired for progress).
- `EnergySimulationRunJob::step` (line ~1195, `impl InteractiveJob`): each call runs a phase loop
  (`Census → Capture → Simulate`), then:
  ```rust
  let state = if matches!(settled, Some(StepOutcome::Complete(_))) { Complete } else { Running };
  self.writer.progress(self.cursor.progress(self.identity, state, self.stage()));
  let payload = self.writer.finish().and_then(|tick| tick.encode().ok())
      .and_then(|bytes| cx.payload_from_bytes(JobPayloadStream::Preview, &bytes).ok());
  match payload {
      Some(payload) => { self.settled = settled; StepOutcome::PreviewReady(payload) }
      None => { /* close settled outcome unread */ fault_outcome() }
  }
  ```
  The bytes handed to `PreviewReady` are the **encoded `ToolRunTick`** (progress + step ring only —
  `self.writer` is a plain `ToolRunTickWriter`; `.payload(...)` is **never called** anywhere in this
  file). The actual settled `StepOutcome::Complete(candidate)` (the numerical result) is stashed in
  `self.settled` and returned **verbatim, unread**, on the *next* `step()` call (`if let Some(outcome)
  = self.settled.take() { return outcome; }`).

### 2.2 Where the numerical Results live, and where they die

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs`:
- `EnergyJobAuthority::take_results(&mut self) -> Option<Results>` (line 2137) — the accessor for the
  canonical typed result.
- `Results` struct — `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️.rs:56-63`:
  ```rust
  pub struct Results {
      pub time_series: TimeSeriesTable, pub meters: MeterTable, pub summaries: SummaryTables,
      pub sizing: SizingTables, pub environmental: EnvironmentalMetrics,
      pub resilience: ResilienceMetrics, pub diagnostics: Diagnostics, pub run_metadata: RunMetadata,
  }
  ```
  **No field here is per-surface.** `MeterTable` (`🧮️meters/🦀️.rs:69-71`) is
  `FixedTable<String, Meter>` keyed by an end-use meter name (fuel × end-use), not by surface id.
- `EnergyJob::step` (`🧪️sim/🦀️.rs:3766-3772`) settles with:
  ```rust
  StepOutcome::Complete(CommitCandidate {
      state: RetainedJobPayload::empty(JobPayloadStream::CommitState),
      output: <the EncodeOutput-stage-written packet.payload>,   // the wire-encoded Results/output
  })
  ```
- **The framework tool-run driver discards this unconditionally**, for every tool (mutating or not):
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs:1571-1577`:
  ```rust
  semio_framework_job::StepOutcome::Complete(_) => {
      close_step_outcome(&mut outcome);   // frees the candidate's payload pages, reads nothing
      entry.settle_refold();
      match purpose { Some(Revalidate) => self.complete_tool_run_revalidation(), _ => self.complete_tool_run_job(run, generation) }
      terminal = true;
  }
  ```
  `close_step_outcome` (line 351-353) just loops `outcome.close_step(1, JOB_PAYLOAD_PAGE_BYTES)` until
  the pages are released — it never extracts `candidate.state`/`candidate.output` into anything. For a
  `mutating: false` run there is no document publication to carry it either (`complete_tool_run_job`'s
  own doc-comment: "a run that declares `mutating: false` authors no provisional edit at all, so there
  is nothing for a finalize to publish"). **Conclusion: today the fully-solved `Results` (and any
  per-surface physics inside it) is computed, wire-encoded, and thrown away. Nothing reads it.**

### 2.3 The one channel that *does* survive to a window: `ToolRunView.payload`

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs:194-201`:
```rust
pub struct ToolRunView {
    pub tool_id: String, pub identity: ToolRunIdentity, pub state: ToolRunState,
    pub provisional_entities: Arc<BTreeSet<u64>>,
    pub progress: semio_framework_tool_run::ToolRunProgress,
    pub payload: Option<Arc<[u8]>>,
}
```
- Filled from the entry's `payload: Option<Arc<[u8]>>` field (line 410, `ToolRunEntry`), set in
  `apply_tick` (line ~546-547): `if let Some(payload) = tick.payload.take() { self.payload =
  Some(payload.into()); }`. Only cleared by `discard_provisional` (abort/rebase-discard, line ~968);
  **survives Complete → Finalizing → Finalized**.
- `tick.payload` is `ToolRunTick.payload: Option<Vec<u8>>`
  (`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:1066-1078`), documented explicitly: *"Opaque plugin bytes
  the run's windows read back through `ToolRunView::payload` — the latest intermediate result of the
  run (a live readout, a partial solution); the newest tick carrying one wins."*
- Set by the plugin via `ToolRunTickWriter::payload(&mut self, payload: Vec<u8>)` (line 1283): *"Sets
  the pending tick's plugin payload; a later call before `finish` replaces it."*
- **Size limit:** the whole encoded `ToolRunTick` (progress + steps + trace + payload, field 9 =
  `Bytes64(payload)`) must stay under `TOOL_RUN_TICK_BYTES_MAX = 262_144` bytes (256 KiB) —
  `ToolRunTick::encode()` rejects over that (`ToolRunCodecError::Limit("tick bytes")`,
  `⏯️tool-run/🦀️.rs:32,1100-1103`). The tick is then wrapped as a job `PreviewReady` payload via
  `RetainedJobPayload`, paged at `JOB_PAYLOAD_PAGE_BYTES = 16 * 1024` (16 KiB/page,
  `🧰️framework/🔨️modules/🧵️job/🦀️.rs:397`) — a 256 KiB tick spans up to 16 pages, which is fine
  (paged), but a plugin that wants to stay comfortably inside one or two pages should keep its own
  payload well under the 256 KiB tick ceiling (leave room for progress/steps too).
- **Read side for an app window:** `ArtifactView::tool_run() -> Option<&ToolRunView>`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7810` and the mirrored
  `ArtifactOwnedToolJobRequest`-adjacent accessor at line 14068), bound during render via
  `with_render_context(...).with_tool_run(tool_run)`. This is exactly what
  `.../windows/⚡️simulation/🦀️.rs`'s doc comment refers to ("the state of the energy simulation tool
  run as the framework ledger reports it (`ArtifactView::tool_run`)") and what
  `.../✏️editor/🦀️.rs`'s render call already passes: `simulation::render(doc.tool_run(), *cfg.snapshot,
  model, view_state.locale)` (per `📓️wave-W3-1.md` §1, "Render:" line). **A window checks
  `run.tool_id == tools::simulation::TOOL_ID` (as `run_nodes` already does) and, if present, would
  decode `run.payload` itself** — there is no framework-side decode; `output_schema` on
  `WindowKindDefinition` (energy's simulation window sets it to `ENERGY_SIMULATION_RUN_SCHEMA`) is pure
  metadata, not an automatic decoder (confirmed: `output_schema` is only ever stored/forwarded as
  `Option<String>`, never matched against in the framework).

### 2.4 The shortest path to a per-surface map

No per-surface accumulator exists anywhere in the engine today. The zone timestep solve computes
per-surface exterior driving terms transiently (`🌰️kernel/🦀️.rs:1391-1406`, `h_outside`/`q_outside` per
surface index feeding the implicit conduction matrix) and per-surface absorbed solar
(`🌰️kernel/🦀️.rs:1283-1345`, `inside_absorbed_w_m2[..]`/`zone_diffuse_w`) but these live in per-timestep
solver scratch arrays indexed by a local surface index, not by `EntityId`, and are **not** summed across
the run or attached to `Results`.

Shortest path (three additions, in order of dependency):
1. **Accumulate.** In the kernel's per-timestep zone/surface loop, add a `HashMap<EntityId, (f64,f64)>`
   (or a `Vec` aligned to `PrecomputedModel`'s surface list) accumulating
   `conduction_loss_kwh += q_outside_w * area_m2 * dt_s / 3_600_000.0` and
   `solar_gain_kwh += absorbed_w * dt_s / 3_600_000.0` per surface, each timestep.
2. **Publish into `Results`.** Add a field to `Results` (`🧾️results/🦀️.rs:56-63`), e.g.
   `pub per_surface: Vec<SurfaceEnergyRow>` with `SurfaceEnergyRow { surface_id: EntityId,
   conduction_loss_kwh: f64, solar_gain_kwh: f64 }` — mirroring `SummaryRow`'s shape. This makes the
   data reachable via `EnergyJobAuthority::take_results()` for anything **inside** the numerical engine
   (batch callers, the EnergyPlus oracle), but — per §2.2/2.3 — that alone does **not** reach a window,
   because the framework discards the `Complete` candidate's payload.
3. **Surface it through the tick, not the discarded candidate.** In
   `EnergySimulationRunJob::simulate`/`step` (simulation-session `🦀️.rs`), on the tier-publish edge (or
   on every tick — budget permitting under the 256 KiB cap) encode a compact `surface_id →
   (conduction_loss_kwh, solar_gain_kwh)` map (e.g. as a small bincode/`Value` blob, or the project's
   own `pack`/DSL record codec — see memory note "pack::encode_json_value Is Heavyweight": prefer
   `encode/decode_record_body` over the full `.spk` container for this) and call
   `self.writer.payload(bytes)` before `self.writer.finish()`. This is the only field that survives to
   `ToolRunView.payload` and is readable from a window via `doc.tool_run()`.

`ENERGY_SIMULATION_RUN_SCHEMA` (`"energy.simulation.run.v1"`) is the natural schema id to bump/extend for
this new payload shape (it is already wired as `output_schema` on the simulation window and as the
`ToolRunDefinition`'s implicit output contract) — but note again that bumping the constant is only
documentation; the decode logic itself must live in whichever window(s) read `run.payload`.

---

## 3. Re-render triggers: fem2d vs. energy, and the G1/G2 gaps today

### 3.1 fem2d (synchronous, command-driven)

fem2d's results window is re-rendered because **the command that changes its config says so**:
`SetResultDisplay`/`SetResultAnimation` emit `UiDirtyScope::Partial { window_bodies:
[results::BODY_KEY], panel_bodies: [panels::results::BODY_KEY], .. }` (§1.1). There is no polling, no
tick, no async job — every result (static/modal/buckling) is solved fresh (or from the thread-local
cache) the moment the window body is asked to render again, which happens exactly when the dirty scope
names it. This pattern has **nothing to do with ToolRun dirty-scope plumbing** because fem has no
ToolRun for results at all.

### 3.2 energy (async ToolRun, tick-driven) — G1/G2 status today

- **G1 ("add `progress` to `ToolRunView`")** — **done**. Today's `ToolRunView` (§2.3) already carries
  `progress: ToolRunProgress` (not optional) and `payload: Option<Arc<[u8]>>`. The wave doc's remark
  "with it the energy window can restore its per-tier tree" is not acted on: the current
  `.../windows/⚡️simulation/🦀️.rs` still deliberately shows only `run.state`/`run.identity` (via
  `run_nodes`) and explicitly defers the tier readout to "the framework panel's step log" (module doc,
  and `result_text()`'s comments). Nothing prevents the window from also reading `run.progress.counters`
  today (e.g. `FacilityElectricityWh`) — it simply doesn't.
- **G2 ("tick `dirty_scope()` covers only the panel and trace windows")** — **the mechanism is now
  built, but energy has not opted in.** `ToolRunDefinition.windows: Vec<String>`
  (`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:1899-1903`, doc: "Window kind ids whose bodies render this
  run's state (`ArtifactView::tool_run()`): every tick refreshes them next to the ToolRun panel, and no
  other window") is consumed at run-entry construction
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs:1186-1192`,
  `window_bodies = definition.windows.iter().filter_map(|id| registry.window_body_key(id))...`) and
  unioned into `ToolRunLedger::dirty_scope()` (lines 889-893) alongside the trace windows and the
  framework panel body key. **Energy's `energy_simulation_run_definition()` sets `windows:
  Vec::new()`** (simulation-session `🦀️.rs`, the struct literal at line ~183-198 has no `windows:`
  override, defaulting empty) — so **no plugin window body, including `energy.simulation`'s own
  `BODY_KEY = "energy.simulation"`, is ever named by a tick's dirty scope.** Any future walls/3D window
  would need its `WINDOW_KIND_ID` added to this list to redraw on each tick automatically; otherwise it
  only redraws on the next full/document-dirty refresh (window focus, a config mutation elsewhere,
  etc.) — see `mark_tool_run_ui_dirty`/`self.tool_runs.document_dirty` branch at line ~1313 of the same
  file.
- **G3 (panel not mounted as a tab)** — out of scope for painting walls, not re-verified here.
- The **provisional/trace "visible process" layers** (§4.1 of `📋️tool-run-contract.md`) do not apply
  well to energy: layer 1 (provisional entities styled in the live document) only matters for
  `mutating: true` runs with document-visible provisional ops; energy's run is `mutating: false` and
  never touches the document (`finalize publishes nothing` — confirmed by W3-1 §0/§2). Layer 2 (the
  `toolRunTrace` World3d scene lane) is for spatial candidate feedback (`trace: ToolRunTraceKind`);
  energy's run declares `trace: ToolRunTraceKind::None`. **Neither applies — the opaque tick `payload`
  channel (§2.3/§2.4) is the only available path for per-surface colouring.**

---

## 4. Concrete minimal recipe for energy

Files to touch/add (paths abbreviated from
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any` = `A`):

1. **Engine: accumulate per-surface energy.**
   `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs` — extend the zone timestep
   loop (~lines 1283-1406) to accumulate `conduction_loss_kwh`/`solar_gain_kwh` per `EntityId` (§2.4.1).
   Add `SurfaceEnergyRow`/`Results::per_surface` in
   `.../⚙️engine/🧾️results/🦀️.rs` (§2.4.2). Unit test: `sim::tests` — assert a known BESTEST case's
   per-surface conduction sums to the facility heating/cooling meter within tolerance.

2. **Run job: encode a compact surface map into the tick payload.**
   `A/🧵️simulation-session/🦀️.rs` — in `EnergySimulationRunJob::simulate`/`step`, after
   `cursor.observe(...)` on a tier boundary (or at minimum once at `Complete`), build
   `Vec<(EntityId, f32, f32)>` (surface id, conduction kWh, solar kWh — `f32` halves the byte cost)
   from the numerical job's live per-surface accumulator (expose a `EnergyJobAuthority` accessor
   parallel to `cursor()`, e.g. `per_surface_energy(&self) -> &[SurfaceEnergyRow]`), encode it (small
   fixed-width binary, no need for the heavyweight `pack::encode_json_value` — see memory note), and
   call `self.writer.payload(bytes)` before `self.writer.finish()`. Keep the encoded blob well under
   ~100 KiB so it comfortably fits inside `TOOL_RUN_TICK_BYTES_MAX` alongside progress/steps.
   Unit test: a fixture run's last tick decodes to the same per-surface totals as `Results::per_surface`
   from a batch `Engine::run` (mirrors the existing "batch parity oracle" pattern from W3-1 §3.1).

3. **Register the run definition's `windows`.** `A/🧵️simulation-session/🦀️.rs`,
   `energy_simulation_run_definition()` — set `windows: vec![new_walls_window::WINDOW_KIND_ID.into()]`
   (and/or the existing `simulation::WINDOW_KIND_ID` if it should also live-refresh) so `dirty_scope()`
   marks the walls window's body dirty on every tick (§3.2). Law: a running-run law asserting
   `ToolRunLedger::dirty_scope().window_bodies` contains the new window's body key.

4. **New editor window: `walls` (or reuse `structure`'s slot) as a `SurfaceKind::World3d` window.**
   New directory `A/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️walls/🦀️.rs`, modeled directly on fem3d's
   results window (§1.3):
   - `WindowKindDefinition` registered via the editor manifest's `.window_kind(WINDOW_KIND_ID, label,
     BODY_KEY, SurfaceKind::World3d, icon_id)`, the same call fem3d uses
     (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/.../✏️editor/🦀️.rs:1156-1157`).
   - `render(doc: &ArtifactView<EnergyModelSnapshot>, ..)` reads `doc.tool_run()`, filters
     `run.tool_id == tools::simulation::TOOL_ID`, decodes `run.payload` (step 2's format) into a
     `HashMap<EntityId, (f32,f32)>` (empty map / neutral grey when `None`, exactly like fem3d's
     `nodal_stress: Option<&HashMap<..>>` fallback in `vertex_color`).
   - Build one mesh per `Surface` from `vertices_m: Vec<[f64;3]>` (triangulate the polygon — a wall is
     planar, so a simple fan triangulation from vertex 0 suffices), colour **per-vertex** with a
     reused 8-stop ramp (copy `VON_MISES_BANDS`/`hex_to_rgb01`/`von_mises_color` from
     `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs`, or re-derive locally — no framework Rust
     helper exists to share, §1.4) keyed by that surface's `conduction_loss_kwh` (min/max taken across
     all surfaces present in the payload, matching fem3d's global-min/max pattern).
   - Emit via `world3d_scene(camera_json, meshes_json, instances_json, selection_json,
     &WorldSunConfig::default())`, one mesh+instance pair per surface (mirroring
     `fem3d_solid_mesh_entries`).
   - Legend: either a Canvas2d-style overlay is not available in World3d — follow fem3d's own choice of
     **no legend layer** (delegate a min/max readout to a caption/text node placed via
     `with_caption`, the same helper fem3d's results window already uses for "Case: {case_id}"), or add
     a small fixed HUD text block listing `{min:.1} / {max:.1} kWh`. fem2d's swatch-legend pattern
     (Canvas2d only) does not port directly to World3d without a new UI element.
   - Result-field selector: a `set-result-field` `ActionKind::View` action (config mutation on a new
     `EnergyWallsWindowConfig` window-config store, same `WindowConfigOwner` shape as
     `Fem2dResultsWindowConfigOwner`, §1.2) choosing `conductionLoss` vs `solarGain`; its command emits
     `UiDirtyScope::Partial { window_bodies: [walls::BODY_KEY], .. }` exactly like `SetResultDisplay`
     (§1.1) for the *manual* re-render path, independent of the *tick-driven* path from step 3.

5. **Tests to add**, mirroring the existing suites:
   - `A/🧵️simulation-session/🧪️tests/🔬️unit/🦀️.rs`: tick payload round-trip (encode/decode the
     per-surface map), tick-byte-budget law (`ToolRunTick::encode` stays under
     `TOOL_RUN_TICK_BYTES_MAX` for a model with N surfaces — pick a realistic N and assert headroom).
   - New `.../🪟️windows/🧱️walls/🧪️tests/🔬️unit/🦀️.rs`: renders with `run: None` (neutral grey, no
     crash — mirrors fem3d's `None` fallback), renders with a fixture payload (colours match
     `von_mises_color` applied to the known per-surface values), and a `set-result-field` config
     round-trip test mirroring `.../🪟️windows/📊️results/🎚️config/🧪️tests/🔬️unit/🦀️.rs`.
   - A `ToolRunLedger::dirty_scope` law: after registering `windows: [walls::WINDOW_KIND_ID]`, a
     running tick's dirty scope contains the walls body key (framework-side test, or an energy app-level
     integration test through `handle_action`/`toolRunStart`→tick→assert dirty scope, mirroring W3-1's
     `a_completed_simulation_run_finalizes_without_changing_the_document_or_history` style).

### Notable oddity spotted in passing

`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs` around lines 3707-3738 has several
`eprintln!("[DEBUG] begin_fault line ...")` calls left in the `EncodeOutput` fault paths — looks like
debug scaffolding that should probably be removed, unrelated to this ticket's scope.
