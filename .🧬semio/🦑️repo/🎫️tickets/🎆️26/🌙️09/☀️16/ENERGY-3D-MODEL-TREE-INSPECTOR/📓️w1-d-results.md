# W1-D — per-surface simulation results, kernel → colour map

Lane W1-D of `26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR`. Scope: an `EntityId`-keyed per-surface energy
accumulator in the engine, its publication on `Results` and through the tool-run tick payload, the
editor-side decode/colour/legend module, and a `set-result-field` selector on the config store.

Everything below is implemented and green natively + wasm32-wasip2. Nothing in this report is
"written but not run" — every command in §6 was executed and its output is quoted honestly.

---

## 1. Engine accumulator

**New types** — `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️.rs`:

- `SurfaceEnergy { conduction_loss_j, conduction_gain_j, solar_transmitted_j, solar_absorbed_j }` —
  the live joule accumulator, with `accumulate_conduction(flux_w, dt_s)` (splits the one signed
  quantity into its two unsigned halves so a colour map never carries a signed field),
  `accumulate_solar_transmitted`, `accumulate_solar_absorbed`, `summary(id)`.
- `SurfaceEnergySummary { id: EntityId, conduction_loss_kwh, conduction_gain_kwh,
  solar_transmitted_kwh, solar_absorbed_kwh }` — the published kWh row.
- `SurfaceEnergyTable { opaque: FixedTable<EntityId, SurfaceEnergy>, windows: FixedTable<EntityId,
  SurfaceEnergy> }` — admitted exactly once, like `MeterTable`'s backing, never grown afterwards.
  `admit(opaque, windows)`, `insert_opaque/insert_window`, `opaque_mut/window_mut`, `pop` (one bounded
  retirement step), `len/is_empty`, `summaries()`, `get(id)`.

  **Why two `FixedTable`s and not one.** `FixedTable::insert` only accepts keys in ascending order
  (`Err(index) if index == self.len`). The model's opaque surfaces are ascending among themselves and
  its fenestrations are ascending among themselves, but the two sequences interleave arbitrarily, so a
  single table would need a merge pass threaded through the bounded initialization cursor. Every call
  site already knows which side of the envelope it is on (`EnclosureFace::Opaque` vs `::Window`), so
  the split costs nothing at the call site and lets admission piggyback on the exact loops that
  already fill `SimulationModel::surfaces`/`::windows`. `summaries()` chains both, so the published
  shape is a single flat `EntityId`-keyed list.

**State + admission** — `🌰️kernel/🦀️.rs` `SimulationModel` gained `pub(crate) per_surface:
SurfaceEnergyTable`; `🧪️sim/🦀️.rs` `EnergyJobStage::InitializeSurfaces` admits it at backing stage 2
(`model.surfaces.len()`, `model.fenestrations.len()`) and inserts a zeroed row beside every
`SurfaceState` (stage 3) and every `WindowState` (stage 5, which I had to un-inline from its `&&`
short-circuit so both inserts run under the same `pre` guard). `close_step` drains it with
`state.per_surface.pop()` before releasing the state, one row per step.

**Run-period gate.** Every accumulate site is wrapped in `if state.warmup_complete`. That flag is set
in `🧪️sim/🦀️.rs` exactly once, immediately before `EnergyJobStage::StartRun`, so warmup days can never
contribute. Law: `sim::tests::warmup_never_contributes_to_the_per_surface_totals`.

**Accumulate sites** (all in `🌰️kernel/🦀️.rs`):

| quantity | site | expression |
|---|---|---|
| `solar_absorbed` (opaque, outside face) | `unit_face`, `OutsideBoundary::OutdoorAir` arm | `outside_solar_absorptance · solar_w_m2 · area_m2` — the same product that drives `q_outside` |
| `solar_transmitted` + `solar_absorbed` (window) | `unit_window_solar`, after `incident` | diffuse `incident.diffuse_w_m2()·diffuse_transmittance·area` **plus** beam `incident.beam_w_m2·beam_transmittance(cos)·area`; absorbed = Σ panes of the front-absorptance product × area |
| `solar_absorbed` (inside face, diffuse redistribution) | `unit_window_solar` back-face loop | `part · inside_solar_absorptance` (opaque) / `part · Σ diffuse_back_absorptance` (window) |
| `solar_absorbed` (inside face, beam patch) | `unit_beam_patch` both back arms | `absorbed_w · inside_solar_absorptance` / `arriving_w · Σ beam_back_absorptance` |
| `conduction_loss` / `conduction_gain` | `unit_commit`, per face, before the chain substitution | `area_m2 · h_inside · (T_air_settled − T_inside_face)` |

**Deviation worth flagging.** The task named `unit_face` as the conduction site. I accumulate
conduction in `unit_commit` instead, one stage later in the same timestep, and I believe this is the
right call: `unit_face` only *assembles* the implicit matrix row — its `inside_c` is the PREVIOUS
timestep's face temperature and its `air_c` the pre-solve air temperature. `unit_commit` runs after
`unit_solve`/`unit_settle` and has the settled inside-face temperature (`state.solver.solution[index]`
or the mean-radiant reduction) and the settled air temperature (`work.free_temp_c`). That is the flux
the zone air balance actually saw, which is what makes the meter identity in §5 exact rather than
approximate. Solar stays in `unit_face`/`unit_window_solar`/`unit_beam_patch` as specified.

**Publication.** `Results` gained `pub per_surface: Vec<SurfaceEnergySummary>`, filled at
`ResultBuildStage::Assemble` from `self.state` (still resident there — the state is only released by
`close_step`'s retirement). `close_results_step` pops one row per step.

**Live accessor.** `EnergyJobAuthority::per_surface_energy(&self) -> Option<&SurfaceEnergyTable>`
(`🧪️sim/🦀️.rs`, next to `take_results`). `EnergyJob` derefs to the authority, so the run job reads it
as `job.per_surface_energy()` while the run is live.

## 2. Tick payload

`🧵️simulation-session/🦀️.rs`, new `//#region 🧱️SurfacePayload`:

```
"ESF1" (4 B) · row count u32 LE (4 B) · per row 20 B: id u32, 4 × f32 (loss, gain, transmitted, absorbed)
```

- `ENERGY_SURFACE_PAYLOAD_MAGIC` / `_HEADER_BYTES` (8) / `_ROW_BYTES` (20) / `_MAXIMUM_ROWS` (8 192),
  `energy_surface_payload_bytes(rows)`, `encode_surface_energy_payload(rows)`.
- Hand-rolled, not `pack::encode_json_value` — that builds a whole `.spk` container for what is a
  fixed-width array (memory note "pack::encode_json_value Is Heavyweight").
- **Budget.** 2 000 surfaces = 40 008 B, ~15 % of `TOOL_RUN_TICK_BYTES_MAX` (262 144). The 8 192-row
  ceiling is 163 848 B, leaving ~96 KiB for the progress record and the step ring. Over the ceiling
  the encoder publishes its first 8 192 rows rather than failing the tick.
- **When.** `EnergyRunCursor` gained `surface_payload_due`, set in `publish_tier`; `InteractiveJob::
  step` calls `writer.payload(...)` before `writer.progress(...)`/`writer.finish()` when a tier
  boundary fired **or** the run settled `Complete`.

**`windows`.** `energy_simulation_run_definition()` now sets
`windows: vec![ENERGY_MODEL_3D_WINDOW_KIND_ID.into(), ENERGY_SIMULATION_WINDOW_KIND_ID.into()]`
(`"energy.model.3d"`, `"energy.simulation"`). Both are literals declared in the session file rather
than imports of lane B's `…::windows::model::WINDOW_KIND_ID`: this module is the artifact-root run
contract and the editor mounts *through* it (the simulation window reads
`ENERGY_SIMULATION_RUN_SCHEMA`), so importing the other way would make the layering circular for two
string constants. The test
`the_run_refreshes_exactly_the_three_d_model_and_simulation_window_bodies` is the drift gate — it
asserts the literals equal the windows' own constants and the schema's own `x-semio-toolRun.windows`.
That key was added to `✏️editor/🧵️simulation-session/🔣️.json` as a one-line insertion (the file is
NOT reformatted).

## 3. Editor-side results module

New `✏️editor/📊️results/🦀️.rs` + `📊️results/🧪️tests/🔬️unit/🦀️.rs`, mounted in the artifact crate root as
`editor::model::results`.

- `SurfaceEnergyMap(pub HashMap<u32, SurfaceEnergy>)` with `SurfaceEnergy { …_kwh × 4 }` and
  `SurfaceEnergy::field(ResultField) -> f64`.
- `decode_run_payload(&[u8]) -> Option<SurfaceEnergyMap>` — total: wrong magic, short header, a body
  shorter than its own row count, or an allocation refusal all return `None`.
- `surface_energy_from_run(Option<&ToolRunView>) -> Option<SurfaceEnergyMap>` — filters
  `run.tool_id == tools::simulation::TOOL_ID`, then decodes `run.payload`.
- `ResultField { ConductionLoss | ConductionGain | SolarTransmitted | SolarAbsorbed }` with
  `ALL`, `id()` (`conductionLoss|conductionGain|solarTransmitted|solarAbsorbed`), `from_id`, `label`,
  `Default = ConductionLoss`.
- `surface_colors(&SurfaceEnergyMap, ResultField) -> (HashMap<u32, [f64; 3]>, f64, f64)` — global
  min/max across the map (fem3d's pattern), 8-stop ramp, `[r,g,b]` in 0..1. Empty map → empty colours
  and `(0.0, 0.0)`.
- `legend_caption(field, min, max) -> String` → `"Conduction loss · 0.0 – 412.3 kWh"`.
- `SURFACE_ENERGY_BANDS` (the eight hexes copied from fem's `VON_MISES_BANDS`, read low→high so a
  cold blue wall loses little and a red one loses a lot), `SURFACE_ENERGY_NEUTRAL` (fem3d's grey
  fallback), `hex_to_rgb01`, `band_color` (same clamp-and-round arithmetic as `von_mises_color`).
  Copied, not imported: fem and energy have no cross-plugin dependency, and the framework exposes no
  Rust colour helper to a wasm plugin (explore report §1.4).
- `result_field(&EnergyModelConfig) -> ResultField`.

## 4. Result-field selector

- `EnergyModelConfig` (`✏️editor/🎚️config/🦀️.rs`) gained `pub result_field: String` (default
  `"conductionLoss"`, also exported as `DEFAULT_RESULT_FIELD`). `is_valid` now also requires the
  string to name a field this build can colour by. **The struct is no longer `Copy`** (owned
  `String`), so `is_valid`/`simulation_template` take `&self` and the three by-value call sites became
  by-reference: `simulation::render(run, settings: &EnergyModelConfig, …)`, `settings_nodes(&…)`, and
  `EnergyModelEditor::render`'s `simulation::render(doc.tool_run(), cfg.snapshot, …)`.
- `🧬️schema/🔣️.json` gained `resultField` (string + `enum` of the four ids + default +
  `x-semio-state: config`) and its `required` entry. Surgical insert; no reformat.
- New mutation leaf `🧬️schema/🧬️mutations/🎨️change-result-field/` (`🦀️.rs`, `🔣️.json` with
  `binaryTag: 1`, `🧬️schema/🔣️.json`), variant `EnergyModelConfigMutation::ChangeResultField`.
  `ChangeResultField::config(base)` replaces only `resultField` and keeps everything else.
- `ChangeSimulationSettings` was **not** whole-record-safe once the config grew a field it does not
  own: its `diff` replaced the entire snapshot and would have silently reset `resultField`. It now
  has `config_over(base)` (keeps the base's `resultField`) used by `diff`/`inverse`; the old
  `config()` is kept as the standalone validity probe the command already calls. Law:
  `changing_the_run_settings_keeps_whatever_result_field_the_base_had`.
- `🧫️fixtures/🔁️mutations.json` grew from 2 to 6 vectors (a settings change over a non-default
  colour field, two valid recolours, one invalid field id).
- Window action: `SET_RESULT_FIELD_ACTION_ID = "set-result-field"` and `result_field_action()` in
  `🪟️windows/⚡️simulation/🦀️.rs` — `ActionKind::View`, `InteractiveJobClassification::Migrated`, one
  required `select` arg `field` whose options are exactly `ResultField::ALL`, EN/DE labels. The
  window's settings tree now renders `Surfaces coloured by: …` / `Flächen eingefärbt nach: …`.
- Editor plumbing (`✏️editor/🦀️.rs`): `EnergyModelEditorCommand::SetResultField { field }`
  (`#[dsl(key = "set-result-field")]`), `action_id()` arm, args-bridge arm (accepts `field` or the
  host-merged `value`), reduce arm → `ChangeResultField` + `UiDirtyScope::Partial { window_bodies:
  ["energy.model.3d", "energy.simulation"] }`, and registration in
  `ENERGY_MODEL_RETAINED_TOOL_IDS`, `ArtifactToolPublicationContract { lanes: [Config] }` and the
  `bounded_first_step_tool_proofs!` roster.
- **`resultField` is deliberately NOT in `ENERGY_SIMULATION_RUN_SETTINGS`**, so recolouring never trips
  the run's `reconfigure: Restart` policy. Law asserts that pointer's absence explicitly.

## 5. Wiring for the 3d window — already done by the coordinator

The exact call (now live at `✏️editor/🦀️.rs:2007-2017`, `model_window::BODY_KEY` arm):

```rust
let energy = crate::editor::model::results::surface_energy_from_run(doc.tool_run());
let field  = crate::editor::model::results::result_field(cfg.snapshot);
let painted = energy.as_ref().map(|map| crate::editor::model::results::surface_colors(map, field));
let caption = painted.as_ref().map(|(_, min, max)| crate::editor::model::results::legend_caption(field, *min, *max));
model_window::render_with_camera(&crate::energy_model(doc.snapshot), interaction,
    painted.as_ref().map(|(colors, _, _)| colors), caption.as_deref(), model_window::config::current(cfg))?
```

I did not touch lane B's window file. Note for whoever paints: the colour map is keyed by
`EntityId.0` (`u32`), which is the same id lane B's meshes/instances are named by (`40–45`, `50–51`
for BESTEST 600), and a surface missing from the map should take `SURFACE_ENERGY_NEUTRAL`.

## 6. Gates — commands run and their actual output

```
cd /Users/ueli/Documents/semio && cargo check -p semio-s-artifact-energy-model --lib --tests
  → Finished `dev` profile ... 2 lib warnings, 13 lib-test warnings, 0 errors
     (both warning counts are peers' pre-existing unused imports / unnecessary qualifications)

cargo test -p semio-s-artifact-energy-model --lib -- sim::tests results energy_simulation_session editor::model::config windows::simulation
  → 68 passed; 4 failed  — the 3 declared pre-existing reds plus
    sim::tests::energy_job_previews_checkpoints_and_commits_bounded_steps

cargo test -p semio-s-artifact-energy-model --lib -- sim::tests
  → 32 passed; 3 failed  — ONLY the 3 declared pre-existing reds.
cargo test -p semio-s-artifact-energy-model --lib -- sim::tests::energy_job_previews_checkpoints_and_commits_bounded_steps
  → 1 passed.
  So that 4th failure is an in-process ordering effect (the faulting p7c2 test leaves the shared
  Energy abandonment registry dirty for a later test in the same binary), not a new red of mine — it
  does not reproduce in the full run either.

cargo test -p semio-s-artifact-energy-model --lib
  → test result: FAILED. 6222 passed; 3 failed; 2 ignored; finished in 60.50s
    failures: sim::tests::p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one
              sim::tests::p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total
              sim::tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology
    == exactly the three declared pre-existing reds. No new reds.

cargo check -p semio-s-plugin-energy --lib --target wasm32-wasip2
  → Finished `dev` profile, 0 errors (`-v` confirms `Fresh semio-s-plugin-energy`, i.e. the plugin
     crate itself really compiled for wasm32-wasip2, not just its dependency).
```

**Checkpoint wire encoding.** `SimulationModel` grew one field, so its `ToValue`/`Serialize` shape
changed — but the commit/preview/checkpoint *wire* encoders walk named fields of `Results`
(`step_commit_census` / the `EncodeOutput` fragment writer) and were not extended, so the emitted
bytes are unchanged. The three p7c2/p7c1 reds fail with exactly the same messages as the coordinator's
baseline; none of them regressed further.

## 7. Tests added

Engine — `⚙️engine/🧪️sim/🧪️tests/🔬️unit/🦀️.rs`, new `//#region 🧱️PerSurfaceEnergy` (3 tests):

1. `per_surface_conduction_losses_close_the_zone_air_balance_against_the_heating_meter` —
   `test_model_single_zone()` plus the one thermostat the fixture lacks (20 °C heating / 27 °C cooling
   constant schedules), `HeatingDesignDay`, 3 warmup days, one run day. Asserts the single row is
   keyed by `EntityId(30)`, that `conduction_loss_kwh > 0`, and that
   `Σ(loss − gain)` matches the zone heating meter.

   **Tolerance and its physics.** The zone air node balances to
   `Q_hvac = Σ_faces area·h_in·(T_air − T_face) + Q_outdoor_air + Q_internal + dU_air/dt`.
   This fixture zeroes every right-hand term but the first: no people/lights/equipment, no
   infiltration object and an outdoor-air rate that is per-*person* with zero people, and a heating
   design day whose direct and diffuse irradiance are both 0 at −10 °C. The air-storage term vanishes
   too, and that is the key point: at −10 °C the zone needs heat in all 24 hours, so the thermostat
   pins the air at its setpoint for the whole run period. The wall's own transient charge is not
   dropped either — it lives inside `T_face`, which is exactly what the accumulated product carries.
   So the identity is not approximate: the meter's `required_w` is read off the same settled linear
   system whose envelope term `unit_commit` integrates face by face. I measured it before choosing a
   band: **7.246355136318226 kWh vs 7.246355136318228 kWh, two ULP apart.** The assertion is therefore
   `relative ≤ 1e-6` — floating-point slack, not physics slack. My first guess had been 10 %; I
   tightened it after measuring rather than leaving a band wide enough to hide a real regression.

2. `a_window_row_carries_transmitted_solar_on_a_sunny_design_day` — BESTEST case 600 (the glazed
   case) on a `CoolingDesignDay` (35 °C ⇒ 800 W/m² DNI, 150 W/m² diffuse, hours 8–17). Asserts every
   fenestration gets its own row, that Σ `solar_transmitted_kwh > 0` and Σ `solar_absorbed_kwh > 0`
   over the window rows, and that at least one sun-exposed opaque surface absorbs solar too.

3. `warmup_never_contributes_to_the_per_surface_totals` — the same model with 3 vs 7 warmup days;
   four extra −10 °C warmup days would roughly double the total if warmup leaked in, so the law
   asserts the two totals agree within 5 %.

Editor results module — `✏️editor/📊️results/🧪️tests/🔬️unit/🦀️.rs` (8 tests): payload round-trip to
single precision (including an id above 2³¹), the 2 000-surface byte law, encoder truncation at the
row ceiling, malformed/absent payloads (`empty`, short header, wrong magic, truncated body, `None`
run, empty map), colour endpoints on the two extremes plus out-of-range clamping, empty-map and
flat-map safety, every field's column + wire id, config selection with unknown-string fallback, and
the legend caption's exact text.

Session — `🧵️simulation-session/🧪️tests/🔬️unit/🦀️.rs`, new `//#region 🧱️SurfacePayload` (3 tests):
the window-id/schema drift gate, a full fixture run asserting some tick carries a payload, that the
number of carrying ticks never exceeds `tiers + 1`, that the row count equals surfaces + fenestrations
and that the decoded map matches `Results::per_surface` row by row, and a real
`ToolRunTickWriter` law encoding 2 000 rows + 8 steps + progress and asserting the encoded tick is
under half of `TOOL_RUN_TICK_BYTES_MAX` and round-trips through `ToolRunTick::decode`.

Config — `✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`: two new tests (recolour keeps the run settings,
the engine template and the run-settings pointer set; a settings change keeps the base's colour
field), the fixture-driven vector test now covers 6 vectors, the pack/DSL round-trip carries the new
field, and `defaults_and_ranges_follow_the_schema_source_of_record` was extended to probe an
enumerated *string* property outside its `enum` (it previously assumed every property had a numeric
`maximum` and would have panicked on `resultField`).

Window — `🪟️windows/⚡️simulation/🧪️tests/🔬️unit/🦀️.rs`: the existing action-list assertion was updated
for the new verb, and one new test asserts the `select` offers exactly `ResultField::ALL`, that the
action is `ActionKind::View`, and that the window renders the current field in both languages.

## 8. Files touched

Engine:
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🧪️tests/🔬️unit/🦀️.rs`

Session / run contract:
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs`
- `…/🧵️simulation-session/🧪️tests/🔬️unit/🦀️.rs`
- `…/✏️editor/🧵️simulation-session/🔣️.json` (one added `windows` line)

Editor:
- `…/✏️editor/📊️results/🦀️.rs` *(new)*
- `…/✏️editor/📊️results/🧪️tests/🔬️unit/🦀️.rs` *(new)*
- `…/✏️editor/🎚️config/🦀️.rs`
- `…/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `…/✏️editor/🎚️config/🧬️schema/🧬️mutations/🦀️.rs`
- `…/✏️editor/🎚️config/🧬️schema/🧬️mutations/⏱️change-simulation-settings/🦀️.rs`
- `…/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎨️change-result-field/🦀️.rs` *(new)*
- `…/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎨️change-result-field/🔣️.json` *(new)*
- `…/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎨️change-result-field/🧬️schema/🔣️.json` *(new)*
- `…/✏️editor/🎚️config/🧫️fixtures/🔁️mutations.json`
- `…/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs`
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🧪️tests/🔬️unit/🦀️.rs`
- `…/✏️editor/🦀️.rs` (command variant, bridge, reduce, roster, contracts, proofs, `UiDirtyScope`
  import, and the `cfg.snapshot` by-reference render call)
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs` (mounts `editor::model::results`)

## 9. What is still owed / known limits

1. **`solar_absorbed_j` mixes two physically different quantities on an opaque surface**: the
   outside-face absorbed solar (which mostly re-radiates to sky, only a fraction conducts inward) and
   the inside-face share of solar another window transmitted onto it. Both are "solar this face
   keeps", but a user reading the legend as "solar gain to the zone" will over-read an exterior wall.
   If that matters, split it into `solar_absorbed_outside_j` / `solar_absorbed_inside_j` — the wire
   format has room (bump the magic to `ESF2` and add two f32s).
2. **`ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS = 8 192` truncates silently.** A model with more envelope
   faces publishes its first 8 192 rows and the rest colour neutral. No diagnostic step is emitted.
3. **`Results::per_surface` still dies with the job** for a batch/oracle caller only —
   `EnergyJobAuthority::take_results()` returns it, but the framework tool-run driver still discards
   the settled `Complete` candidate unread. The tick payload is the only path to a window; that is by
   design here, not an oversight, but it means the *final* map a window shows is the one published on
   the completing tick, not the one inside the discarded candidate. They are the same data (asserted
   in `a_tier_boundary_and_the_completing_tick_carry_the_per_surface_map`).
4. **The 3d window has no legend swatch strip**, only the caption string `legend_caption` returns.
   fem2d's 8-swatch Canvas2d legend does not port to World3d; if a real ramp strip is wanted it needs
   a UI element lane B owns.
5. **No `set-result-field` control is rendered anywhere yet** — the action is declared, classified,
   registered and reducible, and the simulation window renders the *current* value as a tree leaf, but
   no panel emits a `select` bound to it. Lane C's panels are the natural home.
6. **The coordinator's wave-2 wiring is already in place** (§5) and compiles; I did not re-verify it
   in the browser. Lane E's probes should confirm that a finished run actually recolours.
7. **Three pre-existing reds remain red** (`p7c1_weather_owner…`, `p7c2_preview_typed_view…`,
   `p7c2_restored_commit_bytes…`). I did not touch them. `p7c2_restored_commit_bytes` prints
   `[DEBUG] reserve rejected commits 0 pages 2 / 1 …`, i.e. the commit reservation exceeds
   `numerical_census.pages`; that census is derived from the *model* working set, not from `Results`,
   so it is not my accumulator — but somebody should own it, and the `eprintln!("[DEBUG] …")`
   scaffolding all over `🧪️sim/🦀️.rs`'s fault paths (flagged in the explore report's closing note) is
   still there.

---

## 10. Recolour law

Follow-up to the coordinator's browser evidence (`🗑️generated/energy-results-w5/timeline.txt`): the
scene recoloured while the run ticked (t=22 s, meshes 2833→4777 B, vertex-colour histogram
`[0,0,0,12,0,12,24,0]` → `[0,0,48,0,0,0,0,0]`) and snapped back **byte-identical** to the pre-run scene
at `Finalized` (t=32 s). Two defects, both now fixed and both covered by one app-level law.

### 10.1 The law

`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`,
region `//#region 🎨️RecolourLaw`:
`a_finalized_simulation_run_recolours_the_three_d_model_window`.

It drives the whole chain through the route the shell uses — the registry-backed
`VcsArtifactApp<EditorApp<EnergyModelEditor>, SemioMembers>` from the existing `simulation_app()`
harness, `toolRunStart` for `energySimulation`, pump to `finalized`, then
`app.render(model_window::BODY_KEY, None, &ViewModel::default())` projected to text — and asserts:

- **(a)** the projected 3d body contains `kWh` and names one of the four published fields, *after*
  finalization, and that the finalized body is not byte-identical to the pre-run body;
- **(b)** at least two distinct ramp stops are painted into the scene's per-vertex colour arrays,
  after finalize **and** while the run was still `running`/`starting`;
- **(c)** at least one `UiDirtyScope` the app owed during the run covered `energy.model.3d`, read the
  way the shell reads them (`PluginApp::take_typed_operation_ui_scope`, drained every driver turn —
  the ledger only pushes a new scope when the outbox is empty, so a test that never drains sees one),
  plus the static half: every id in `ToolRunDefinition::windows` is a window kind this editor
  registers, because the driver builds `entry.window_bodies` with
  `definition.windows.iter().filter_map(|id| registry.window_body_key(id))` and **silently filters
  away** an id nothing declares.

Two helpers were needed and are worth naming: `drain_ui_scopes` (above) and `painted_bands`, which
reads the ramp stops out of the projected scene. The first version of `painted_bands` matched each
channel at one fixed precision and was quietly wrong — the mesh lane prints `0.839,0.812,0.769`, i.e.
three decimals, so `format!("{:.5}")[..5]` misses `0.9216`→`0.922`. It now accepts any of the
precisions the projection may have written, or the law would have been vacuous.

### 10.2 Defect 1 — the finalize path cleared the payload (framework)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs`. `close_tool_run_publication`, on
`ToolRunEffect::ReleaseProvisional` (line ~1781), called `discard_provisional()`, whose last line is
`entry.payload = None`. That method has two kinds of caller and they want opposite things:

- genuine discards — abort, job fault, rebase-restart, reconfigure-restart — where the run's
  intermediate result is invalidated and the windows must stop showing it;
- the **post-finalize release**, where the provisional ops have just been published successfully.

`payload` is not provisional work. It is the run's own result (`ToolRunTick::payload`'s own doc: "the
latest intermediate result of the run … the run's windows read it back through
`ToolRunView::payload`"), and a `mutating: false` run publishes nothing else at all. Clearing it on
finalize is what made a finished read-only run's windows revert the instant it reached `Finalized`.

Minimal generic fix: `discard_provisional()` is now `release_provisional(false)`, the finalize path
calls `release_provisional(true)`, and the split is documented on both. Nothing else changed.

**Blast radius, checked rather than assumed.** `entry.payload` has exactly three touch points in the
whole framework — the `ToolRunView` builder (line 444), `apply_tick` (547) and `release_provisional`
(982) — and `ToolRunView::payload` is read in exactly ONE place in every `✏️s/` plugin combined:
`✏️editor/📊️results/🦀️.rs:167` (`surface_energy_from_run`). So the change is provably inert for every
other plugin; it can only ever make a payload outlive a finalize that previously erased it.
`cargo check -p semio-framework-os-kernel --lib --tests` is clean. The os-kernel `--lib` suite aborts
the whole test binary in `os_spr::protocol_laws::tests::frame_corpus_round_trip_panics_for_a_lossy_codec`
("thread caused non-unwinding panic"), which is pre-existing, unrelated to `⏯️tool-run`, and prevents
running that binary to completion; the `🔌️plugin` module has no unit tests in it at all — its laws
live in the artifact crates' `artifact_app_laws` harnesses, which is where this law lives.

**The law discriminates it.** With the fix reverted (`release_provisional(false)`) the law fails with
exactly the reported symptom:

```
a_finalized_simulation_run_recolours_the_three_d_model_window ... FAILED
the FINALIZED run left no legend caption in the 3d body — `ToolRunView::payload` was cleared on
finalize, or the body never re-rendered. Body: {…"colors":[0.839,0.812,0.769,…]…}
```

— the family palette, no ramp stop anywhere. Restored, it passes.

### 10.3 Defect 2 — one ramp bucket

Measured the real numbers first rather than guessing at the mapping. BESTEST 600 under the app's own
fixture scenario (`🧵️simulation-session/🧫️fixtures/🔣️.json` run period + settings), per surface:

| id | surface | conduction loss kWh | conduction gain kWh | solar transmitted | solar absorbed |
|---|---|---|---|---|---|
| 40 | South wall | 0.2439 | 1.2149 | 0 | 28.008 |
| 41 | East wall | 0.4478 | 2.5469 | 0 | 21.915 |
| 42 | North wall | 0.6069 | 2.3674 | 0 | 9.097 |
| 43 | West wall | 0.4478 | 2.5491 | 0 | 21.723 |
| 44 | Roof | 1.4207 | 3.9919 | 0 | 67.511 |
| 45 | Floor | 0.5157 | 11.1071 | 0 | 14.273 |
| 50/51 | South windows | 0.7449 | 0.4347 | 18.924 | 5.489 |

So the data DOES have spread — seven distinct loss values — and linear banding over them would have
given five distinct bands, not one. The single bucket therefore was **not** the mapping: it was the
payload. Two things caused it, both fixed:

1. **The only intermediate payload was all zeros.** Tier boundaries were the sole republish trigger,
   and the first (and for a design-day run the only) one fires at `EnergyJobStage::StartRun` —
   *before* a single run timestep has been integrated. min == max == 0, so
   `band_color(0.0, 0.0, 0.0)` put all 48 vertices on stop 0 and froze there for the rest of the run.
   Exactly the recorded histogram.
   - `SurfaceEnergyTable::has_energy()` (new, `⚙️engine/🧾️results/🦀️.rs`) now gates publication: the
     run holds the payload back until something has actually been integrated.
   - `surface_colors` returns an **empty** colour map when the selected field is identically zero
     everywhere, so the window keeps its neutral family palette instead of painting a fake uniform
     result. This is a real case, not a corner: on a cooling design day BESTEST 600's conduction
     *loss* is `0.0000` on all eight faces (measured), and on a heating one the *gain* column is.
     The caption still prints the honest `0.0 – 0.0 kWh`.
   - `ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL = 16` republishes the map every 16 computed timesteps on
     top of tier boundaries and completion, so a long run recolours continuously rather than holding
     whatever the last tier carried (~40 KB per refresh at 2 000 surfaces).

2. **The mapping could not survive a dominant face even when the data was good.** An envelope's
   conduction is dominated by area × U, so one face (a ground floor, a large glazed wall) routinely
   carries several times what the others do, and fem3d's linear `(v−min)/(max−min)` then collapses
   every other face onto the coldest band. `surface_colors` now bands by **rank among the distinct
   values present**, spread evenly over the eight stops — the smallest distinct value always takes
   stop 0, the largest always stop 7, ties share a band, and values closer than `span·1e-6` are one
   tie so float noise never splits one. That guarantees the property the coordinator asked for:
   distinct values get distinct bands. On the table above it yields seven bands where linear yielded
   five. The trade-off is explicit and lives in the doc comment: **the ramp encodes the order of the
   faces, the caption encodes the magnitudes.**

New unit laws in `✏️editor/📊️results/🧪️tests/🔬️unit/🦀️.rs`:
`one_dominant_face_no_longer_collapses_every_other_face_onto_the_coldest_band` (which first asserts
that the linear ramp really does collapse that case, so the premise cannot rot),
`the_rank_ramp_stays_monotonic_and_ties_share_a_band`,
`more_surfaces_than_bands_still_span_the_whole_ramp_monotonically` (40 surfaces → all eight bands,
never running backwards), and
`a_field_that_is_zero_everywhere_colours_nothing_rather_than_faking_a_flat_result`.

### 10.4 Files changed in this follow-up

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` — `discard_provisional` /
  `release_provisional` split; the finalize path keeps the payload.
- `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧾️results/🦀️.rs` — `SurfaceEnergyTable::has_energy`.
- `…/🧵️simulation-session/🦀️.rs` — `ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL`,
  `EnergySimulationRunJob::ticks_since_surface_payload`, the `has_energy` publication gate.
- `…/✏️editor/📊️results/🦀️.rs` — rank banding (`surface_colors`, `rank_band`), the all-zero-field guard.
- `…/✏️editor/📊️results/🧪️tests/🔬️unit/🦀️.rs` — four new laws, one updated.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the recolour law and its two helpers.
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🧪️tests/🔬️unit/🦀️.rs` — my two action laws now read
  `window_shared_action_definitions()` instead of the window kind's own `actions`, which lane W2-C
  emptied while moving the three verbs app-level.

### 10.5 Gates re-run

```
cargo check -p semio-s-artifact-energy-model --lib --tests        → clean
cargo check -p semio-framework-os-kernel  --lib --tests           → clean
cargo check -p semio-s-plugin-energy --lib --target wasm32-wasip2 → Finished, 0 errors
cargo test  -p semio-s-artifact-energy-model --lib -- results editor::model::config
            windows::simulation energy_simulation_session recolour → 43 passed; 0 failed
cargo test  -p semio-s-artifact-energy-model --lib                 → 6290 passed; 3 failed; 2 ignored
    failures: the same three declared pre-existing reds
      sim::tests::p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one
      sim::tests::p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total
      sim::tests::p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology
```

### 10.6 Honest limits of this law

- It discriminates **defect 1** cleanly (proved by reverting the framework fix: the law fails with the
  reported symptom). It does **not** discriminate the tick-interval republish on this fixture: with
  `ENERGY_SURFACE_PAYLOAD_TICK_INTERVAL` disabled the law still passes, because the fixture run is
  short enough that a tier boundary lands after real data has accumulated and before the job settles.
  The interval is justified by the browser run (30 s, many timesteps between tiers), not by this law.
  A law that pinned it would need a fixture with a long tier-free stretch.
- `painted_bands` reads colours out of the projected body text, not out of a decoded `World3dScene`
  (the app-level render returns a `ComponentTree` projected to a string, and the mesh lane arrives
  chunked across `dataAttributes` keys). It can therefore under-count bands if the projection ever
  changes how it prints floats — it would fail closed, not pass vacuously, but it is a text match.
- The law asserts "≥ 2 distinct bands", not the exact band per surface. Pinning the exact assignment
  would re-assert `rank_band`'s arithmetic, which its own unit laws already own.
- The browser has **not** been re-probed since these fixes — lane E's timeline should be re-run to
  confirm the scene now stays coloured through `Finalized` and spreads across bands during the run.
