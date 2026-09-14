# ⚡️ Wave W3-1: energy simulation as a ToolRun

Lane W3-1 of `📋️tool-run-contract.md` (§2, §3.2, §3.6, §3.7, §4, §5 wave 3 item 1).

**Status: landed.**

- The simulation is the `energySimulation` tool's read-only framework ToolRun.
- The five plugin verbs, `EnergySimulationStatus`, the plugin session registry and the plugin chords are gone.
- All new laws pass: 42 tests, listed in §3.
- The W1-D predicates report 0 energy findings.
- Native and `wasm32-wasip2` checks reach warnings on both energy crates.
- The EnergyPlus oracle **runs again but is red**, for pre-existing physics reasons (§3.3).

Paths: `E` = `✏️s/🔌️plugins/🔋️energy`, `A` = `E/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. Logs are in `T/🗑️generated/W3-1/`.

## 0. What `AdoptSimulation` wrote to the document: nothing

**Source trace (pre-change code):**

1. `reduce` sent every session verb, adopt included, to `simulation_session::record_event`. It returned `Emit { description, ..Default }` with no artifact, config or draft mutations.
2. The factory declared the publication lane `HostOnly`.
3. `apply_event_one(Adopt)` only set `MountedState.adopt_requested`.
4. `maintenance_one` then acknowledged the numerical commit lease (`job.ack_commit_packet`) and set `projection.adopted` / `Adopted`.
5. `maintenance_step` copied that into a process-local `AdoptedProjectionAuthority`. It was shown only while the render identity (same app instance and document revision) still matched. `mounted_job_maintenance_step` has no store access at all.

**Runtime confirmation.** A temporary test ran `start-energy-simulation` and then `adopt-energy-simulation` through `handle_action` on a registry-backed app (`baseline-adopt-trace.txt`). Result:

- 0 mutations and no `history_patch`;
- `document_pack` pack/spr and `history_snapshot` byte-identical before and after.

The fixture records this as `lifecycle.adoptBeforeToolRun`.

**Consequence:** `mutating: false`. Finalize publishes nothing; "adopt" is the framework's `finalized` state.

## 1. What changed

### Run vocabulary and run job: `A/🧵️simulation-session/🦀️.rs` (rewritten, 1272 lines)

**Deleted:**
- the process-global `Registry`, `MountedState` and event log;
- the `EnergySimulation{Status,Projection,EventKind,RequestIdentity,ConfigProjection}` types and `EnergyTierProjection`;
- the process bridge (`BoundedJob` factory, `ENERGY_SIMULATION_JOB_KIND`, `initialize`, recovery registry);
- the adopted authority, `reconcile`/`with_projection`/`session_settings` and all close/maintenance hooks.

**Kept:** the bounded `CaptureCensus`/`ModelCapture`. They now also copy `run_period` and all five schedule families (lanes 40–45, `CAPTURE_LAST_LANE`). Before, a run silently simulated the default full year, because the old code read those fields from the live snapshot instead of the capture.

**New (lines 18–205):**
- constants `ENERGY_SIMULATION_RUN_JOB_KIND`, `ENERGY_SIMULATION_RUN_SCHEMA`, `ENERGY_SIMULATION_RUN_SETTINGS`;
- enums `EnergySimulationRunStage`, `EnergySimulationRunCounter`, `EnergySimulationRunReason`;
- `energy_simulation_run_definition()` and `simulation_config_for()`.

**New `EnergySimulationRunJob` (line 1009, `impl InteractiveJob` at 1162):**
- Phases are census, capture, admission of the numerical `EnergyJob`, then inline simulation.
- Each numerical step gets its own inner `StepContext` under the caller's deadline, because a context grants one payload page.
- Restore checkpoints are retired, since the run restarts instead of restoring.
- One unit of fuel is one computed timestep (warmup or run). A tick is flushed only when at least one timestep was computed or the run settled; the settled outcome follows on the next call.
- A step is published when each quality tier ends.
- Refusals and faults publish a `danger` step before `Fault`.
- The bounded close retires the settled outcome, the numerical job, the rejected admission, the capture and the base `Arc`.

### Schema-first sources and fixtures

- `A/✏️editor/🧵️simulation-session/🔣️.json` is now the run definition source of record: `x-semio-toolRun` with policies, stages, counters, reasons (code, verdict, args, EN/DE) and settings pointers. It replaces the event schema.
- `…/🧫️fixtures/🔣️.json` is now the language-agnostic run law:
  - scenario: BESTEST 600, Jan 1, warmup 1 day;
  - expected: 24 + 24 timesteps, tiers `steadyStateEstimate → coarseTimestep → final`, fuel unit `timestep` with 48 ticks, plus the lifecycle expectations.

### Editor config (new, replaces the registry settings): `A/✏️editor/🎚️config/`

- `🦀️.rs`: `EnergyModelConfig { zone_timestep_minutes, system_timestep_minutes, warmup_days }`, with `is_valid()`, `simulation_template()`, DSL/pack codecs and `impl_whole_record_config!`.
- `🧬️schema/🧬️mutations/🦀️.rs` is the `EnergyModelConfigMutation` aggregate. `⏱️change-simulation-settings/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}` is its only leaf.
- `🧬️schema/🔣️.json`, `🧫️fixtures/🔁️mutations.json` and `🧪️tests/🔬️unit/🦀️.rs` hold the schema, the mutation vectors and the tests.
- The old placeholder `📌️.empty.md` was deleted.

### Tool: `A/✏️editor/🎭️modes/✏️edit/🛠️tools/⚡️simulation/🦀️.rs` (new)

- `TOOL_ID = "energySimulation"`.
- `definition() -> ToolDefinition { run: Some(energy_simulation_run_definition()), .. }` with no `keys`.
- It is listed in the edit mode's `tools` (`…/✏️edit/🦀️.rs`) and mounted in `E/🗿️artifacts/🔋️model/🦀️.rs` (`editor::model::config`, `modes::edit::tools`).

### Editor: `A/✏️editor/🦀️.rs`

**Removed:**
- the `start/cancel/retry/discard/adopt-energy-simulation` ids, commands, bridge arms, proofs and publication contracts;
- `is_session_command`/`session_event`;
- `pending_effects`, `mounted_job_*`;
- the `mod+enter`, `mod+.` and `mod+shift+enter` keybindings.

**Renamed:** `configure-energy-simulation` → `set-simulation-settings` (`SetSimulationSettings`). It validates and emits one config mutation on the `Config` lane.

**Added:**
- `.tool(tools::simulation::definition())` (line 1316);
- `build_tool_run_job` (line 1201): `Run` purpose only; `EnergySimulationRunJob::new(identity, snapshot, config.simulation_template())`;
- `build_config_store_one_item_preparation_factory` (line 1195), using the framework's bounded config preparation;
- store owners, disposers and retirement factories (line 1147). Without them `load_document_pack` and close panicked in `Drop`: a pre-existing defect that made the existing dispatch tests red.

**Fixed:**
- the document preparation footprint `work_items` 1 → 2 (line 954). A point-invertible edit costs two rows, so every document verb failed the store's fold contract.
- `load_document_effect` detaches its envelope with `.into_owners()`. The same fix went into the artifact root `energy_model_load_document_effect`: `setActiveExample` panicked in `Drop`.

**Render:** `simulation::render(doc.tool_run(), *cfg.snapshot, model, view_state.locale)`.

### Windows

**Editor simulation window** (`A/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`, rewritten):
- actions `set-simulation-settings` and `set-run-period`;
- a live region built from `ToolRunView` (framework state label, busy, run and generation, result meaning);
- the settings tree;
- keyboard help rendered from `ToolRunAction::chord()` and the framework labels.
- The per-tier tree could not be kept (gap G1). The tier readout now lives in the framework panel's step log, with the reason templates above.

**Viewer simulation window**: the process-local adopted projection is gone. It renders the run period and states that simulations never change the document.

### Other files

- `E/🦀️.rs`: the `initialize()` call was removed.
- `A/✏️editor/🟦️.ts` and `…/⚡️simulation/🟦️.ts`: the TS twins now carry the tool id, the config type, the roster and the actions. The event and tier projection types were removed.
- Both energy `Cargo.toml`s gained `semio-framework-tool-run = { workspace = true }`. The plugin crate needs it because its `dyn_enum_close!` expansion names tool-run types.

### Engine: `E/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs`

- **New accessor:** `pub struct EnergyJobCursor` (line 415) and `EnergyJobAuthority::cursor()` (line 2029).
- **Bug fix in `step_aggregate_facility`** (line 2818). Cursor 2 left a half-built meter work item mounted after hour 0. Hour 1 then tried to insert a third facility meter into the full table, raised `backing_rejected`, and faulted **every** batch and streamed run on its second hour. This was pre-existing: `sim::tests::engine_*`, `run_period_honors_calendar`, `full_topology_e2e` and `energy_conservation_*` were all red, and they pass now.

## 2. Public API as landed

```rust
// crate::energy_simulation_session
pub const ENERGY_SIMULATION_RUN_JOB_KIND: &str = "energy.simulation.run";
pub const ENERGY_SIMULATION_RUN_SCHEMA: &str = "energy.simulation.run.v1";
pub const ENERGY_SIMULATION_RUN_SETTINGS: [&str; 3] = ["/zoneTimestepMinutes", "/systemTimestepMinutes", "/warmupDays"];
pub enum EnergySimulationRunStage { Capture, Prepare, Warmup, Run, Finalize, Encode }      // ALL, index, id, label, of(EnergyJobStage)
pub enum EnergySimulationRunCounter { WarmupTimesteps, RunTimesteps, TiersPublished, FacilityElectricityWh } // ALL, index, id, label
pub enum EnergySimulationRunReason { SteadyStateEstimate, DesignDay, CoarseTimestep, Final, WarmupConverged, CaptureRejected, AdmissionRejected, SimulationFaulted } // ALL, code, from_code, id, verdict, template, of_tier
pub fn energy_simulation_run_definition() -> ToolRunDefinition;
pub fn simulation_config_for(template: &SimulationConfig, model: &Model) -> SimulationConfig;
pub struct EnergySimulationRunJob;
impl EnergySimulationRunJob { pub fn new(identity: ToolRunIdentity, snapshot: Arc<EnergyModelSnapshot>, template: SimulationConfig) -> Self }
impl InteractiveJob for EnergySimulationRunJob;
// crate::editor::model::config
pub struct EnergyModelConfig { pub zone_timestep_minutes: u32, pub system_timestep_minutes: u32, pub warmup_days: u32 } // Default 60/60/7; is_valid; simulation_template
pub enum EnergyModelConfigMutation { ChangeSimulationSettings(ChangeSimulationSettings) }
pub struct ChangeSimulationSettings { pub zone_timestep_minutes: u32, pub system_timestep_minutes: u32, pub warmup_days: u32 } // config()
// crate::editor::model::modes::edit::tools::simulation
pub const TOOL_ID: &str = "energySimulation"; pub fn definition() -> ToolDefinition;
// crate::editor::model::modes::edit::windows::simulation
pub const SET_SETTINGS_ACTION_ID: &str = "set-simulation-settings";
pub fn render(run: Option<&ToolRunView>, settings: EnergyModelConfig, model: &Model, locale: Locale) -> BuiltNode;
// engine
pub struct EnergyJobCursor { pub stage, pub tier, pub warmup_hour: u32, pub warmup_hours: u32, pub timestep: u32, pub total_timesteps: u32, pub facility_electricity_kwh: f64 }
impl EnergyJobAuthority { pub fn cursor(&self) -> EnergyJobCursor }
```

### Run definition

| Field | Value | Reason |
|---|---|---|
| `mutating` | `false` | Adopt never wrote the document (§0) |
| `rebase` | `restart` | Results must describe the head model. The job captures the base, so any edit invalidates every tier |
| `reconfigure` | `restart` | The timestep and warmup settings change the numerical trajectory. There is no prefix property to resume from |
| `trace` | `none` | No spatial candidates exist |
| `revalidateJob` | none | — |
| `settings.config` | the three config pointers | `reconfigure` fires only for these fields |

**Stages:** capture, prepare, warmup, run, finalize, encode. **Counters:** warmup timesteps, run timesteps, tiers published, facility electricity (Wh, live). **Unit:** timesteps / Zeitschritte.

**Progress:** `completed` = computed timesteps; `total` = warmup computed + total run timesteps, known from the run stage on. **State:** `running`; the last tick is `complete`.

## 3. Tests run (foreground; logs in `T/🗑️generated/W3-1/`)

`RUST_MIN_STACK=134217728` was set on every test command.

| Command | Result |
|---|---|
| `cargo test -p semio-s-artifact-energy-model --lib -- energy_simulation_session:: sim::tests::engine sim::tests::run_period sim::tests::full_topology …` | 18 run, 16 pass (`test-8.txt`). The 2 failures are the design-day tests (§3.3) |
| `cargo test -p semio-s-artifact-energy-model --lib -- editor::model::component::tests --test-threads=4` | **20 passed, 1 failed** (`test-editor-4.txt`). The failure was the finalize law, which asserted a manual finalize. Rewritten for W0-H's automatic read-only finalize, it passes 1/1 (`test-finalize.txt`) |
| `cargo test -p semio-s-artifact-energy-model --lib -- artifact_root_tests energy_simulation_session editor::model::config tools::simulation windows::simulation sim::tests::engine sim::tests::run_period sim::tests::full_topology` | **29 passed, 0 failed** (`test-scoped-final.txt`) |
| `cargo test -p semio-s-artifact-energy-model --lib -- --test-threads=8 --skip p7c2_restored_commit_bytes…` | 940 passed, 4 992 failed (`test-lib-full.txt`); breakdown in §3.2 |
| `cargo check -p semio-s-artifact-energy-model -p semio-s-plugin-energy --lib --tests` | Finished (`check-native-final.txt`) |
| `cargo check … --lib` and `… --lib --target wasm32-wasip2`, each with a temporary `[DEBUG]` dead fn in both crates | Both targets Finished with the probe warning reported in **both** crates (`check-native-probe.txt`, `check-wasip2-probe.txt`). Probes removed |
| `bun T/🐍️w3-1-energy-policy-probe.ts` (new, kept) | amend 0, local-lifecycle 0, legacy-trace 0, declaration 0, reserved-action 0 (`policy-probe-2.txt`) |
| Planted-violation check | Renaming the row's tool gives 1 declaration finding; planting `"adopt-energy-simulation"` in the session gives 7 lifecycle findings, so the predicate is not vacuous |
| `cargo test -p semio-s-plugin-energy --lib -- plugin::surface_tests` | 2 passed, 1 failed: `new_viewer_builds_a_runnable_energy_model_viewer_app`, a pre-existing viewer `Drop` without store owners |

### 3.1 Required laws: all green

- **Run → complete → finalize** (`a_completed_simulation_run_finalizes_without_changing_the_document_or_history`):
  - a real BESTEST 600 run completes 48/48 timesteps;
  - the window shows the accepted result and the panel step log contains `Final: … kWh`;
  - a second finalize is rejected;
  - document pack/spr and history are byte-identical.
- **Abort** (`aborting_a_simulation_run_leaves_the_document_byte_identical`): abort after the first timesteps reaches `aborted`, with the document and history byte-identical.
- **Fuel unit = one timestep:**
  - job law `the_pause_step_fuel_unit_is_one_computed_timestep`: fuel 1 gives 48 ticks, each advancing `completed` by exactly 1, with the tier order, the stage sequence and the counters from the fixture;
  - app law `a_paused_simulation_run_steps_exactly_one_timestep_per_step`: `toolRunPause`, then three `toolRunStep`, each +1.
- **Reconfigure** (`changing_the_simulation_settings_restarts_a_live_run_in_its_next_generation`): `set-simulation-settings` on a paused run moves it to the next generation, and the next step restarts at `completed = 1`.
- **Batch parity oracle** (`run_job_results_and_final_readout_agree_with_the_batch_engine_oracle`):
  - the streamed run's meters, summaries and time series equal `Engine::run`, the exact path the EnergyPlus comparison uses;
  - the final step reports the facility meter.
- **Other laws:**
  - `cancellation_mid_run_settles_cancelled_and_close_retires_every_owner`;
  - `a_model_beyond_numerical_admission_publishes_a_danger_step_before_the_fault`;
  - `the_bounded_capture_reproduces_every_model_field_including_run_period_and_schedules`;
  - `run_definition_matches_the_schema_source_of_record`;
  - the config mutation vectors against the serde oracle;
  - the tool declaration;
  - `the_simulation_is_driven_by_the_framework_tool_run_actions_and_chords_only`: `mod+enter`/`mod+.`/`mod+shift+enter` bind `toolRunStart`/`toolRunAbort`/`toolRunFinalize`, and the five verbs are gone from roster, windows and the command bridge.

### 3.2 Pre-existing reds (none in files this lane changed, unless noted)

- **4 975** `standards::v1::subsets::…` mutation-conformance cases fail with "committed before-snapshot does not decode: expected a string, found Null". This is snapshot fixture/schema drift.
- `examples::demo` ×2 fail with "missing model line".
- `bestest::tests::committed_example_assets_match_the_builders` and `dispatch::tests::uniform_with_no_capacity_returns_empty` fail.
- The `structure`/`zones` windows fail (×5) on `ui.fixed-capacity` tree siblings and untranslated arg names.
- `viewer_declares_no_dispatchable_action` fails: the viewer's structure/zones kits declare actions.
- Seven `sim::tests::p7c*`/`energy_job_previews…` laws fail. They pin the old fuel chronology or checkpoint shape of the numerical job. They were already red: the facility fault ended every run at hour 1.
- `p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology` runs for more than 20 minutes and was skipped.

### 3.3 Third-party oracle (EnergyPlus / ANSI/ASHRAE 140)

`cargo test … --lib -- --ignored bestest_cases_compared_with_energyplus` now **runs** in 14 s, where before the fix it faulted at hour 1. It **fails on physics** (`test-bestest-oracle.txt`). Examples:
- case 920: cooling 5 969 kWh vs 2 741 kWh from EnergyPlus;
- case 940: cooling 8 953 kWh vs 2 434 kWh.

The design-day tests (`ashrae_140_case600_base`, `hvac_bestest_heating_day`) fault at `EncodeOutput` with `OutputFault::BackingRejected`, a commit reservation mismatch for design-day environments.

Both are engine defects outside the tool-run contract. The streamed run is proven bit-identical to the batch engine (§3.1), so the ToolRun conversion itself cannot move the oracle.

## 4. Commands to register in launch.json

- `cargo test -p semio-s-artifact-energy-model --lib -- energy_simulation_session editor::model::config tools::simulation windows::simulation editor::model::component`: the W3-1 laws (the app-level run tests take about 10 min at opt-level 0 under load).
- `cargo check -p semio-s-artifact-energy-model -p semio-s-plugin-energy --lib --target wasm32-wasip2`
- `bun .🧬semio/…/INTERACTIVE-TOOLS-VISIBLE-PROCESS/🐍️w3-1-energy-policy-probe.ts`: the energy row of the W1-D predicates.

## 5. Deviations, with reasons

1. **Settings moved into a real config store** (`EnergyModelConfig`) instead of the plugin registry. The driver can only observe config-store or window-config generations (§3.3, §3.7.5), and the contract forbids plugin-owned registries.
2. **The simulation window no longer draws the per-tier tree.** `ToolRunView` exposes only state and identity, and plugin window bodies are not in the tick dirty scope (G1, G2). The live tier readout is the framework panel's step log, with EN/DE reason templates carrying `[kWh, timesteps, total]`, plus the live `facilityElectricityWh` counter.
3. **Finalize is automatic.** W0-H landed an automatic finalize for `mutating: false` runs during this lane, so the finalize law asserts that `finalized` is reached with zero document change and that a manual finalize is rejected.
4. **Fuel unit.** Preparation, capture and finalization compute no timestep. They yield without ticks and fold into the adjacent unit, so one step is always exactly one timestep. The final settle tick advances `completed` by 0.
5. **Out-of-scope fixes in owned files**, needed to reach runtime evidence:
   - the engine facility-meter fault;
   - the preparation footprint (`work_items` 2);
   - the envelope detach (`into_owners`);
   - the editor store owners, disposers and retirement factories.
6. **The EnergyPlus oracle stays red** for physics reasons (§3.3), not green as briefed.

## 6. Foreign edits

None. Everything is under `✏️s/🔌️plugins/🔋️energy/**`. One script was added in `T`: `🐍️w3-1-energy-policy-probe.ts`.

## 7. Open items and gaps

- **G1 (W0-H).** Add `progress: ToolRunProgress` (or at least steps and counters) to `ToolRunView`, so plugin windows can draw run results from framework state. With it the energy window can restore its per-tier tree.
- **G2 (W0-H).** A tick's `dirty_scope()` covers only the ToolRun panel and trace windows. Plugin bodies that read `doc.tool_run()` refresh only on document-dirty events.
- **G3 (W0-H or framework).** Nothing mounts the `FRAMEWORK_TOOL_RUN_BODY_KEY` panel as a panel tab. It needs to be injected for apps with `app_declares_tool_run`, the way the history tab is, or the step log and buttons are unreachable in the shell.
- **G4.** With `rebase: restart`, a base change while the run is `complete` does not re-run: W0-D open item 5. It is moot now that read-only runs finalize immediately.
- **Energy engine owners:**
  - EnergyPlus disagreement (case 920/940 cooling, 2–4×);
  - design-day `EncodeOutput` reservation fault;
  - the red `p7c*` numerical chronology laws;
  - the 4 975 snapshot-decode conformance failures;
  - the structure/zones window reds.
- **`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json`** (foreign) still embeds the deleted `semio.energy.simulation-event.v1` fixture text. Its owner should regenerate it.
- **W1-D row.** The energy row could add `✏️editor/🎭️modes/✏️edit/🛠️tools/⚡️simulation` to its `scope`; the predicates are already green without it.
- **Launch.** The browser/runtime proof for energy (§6.7) is not done; W1-E-style verification is owed.
