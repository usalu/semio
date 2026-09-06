# Energy Engine API and BESTEST-Validation Feasibility

Explored read-only. All paths relative to repo root `/Users/ueli/Documents/semio`. The
engine lives at `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/<module>/🦀️.rs` (50
modules, mounted flat into crate `semio-s-plugin-energy` via
`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs:40-191` — `pub mod <name>;` + `pub use
<name>::*;` per module, each `#[path = "…/<emoji-module>/🦀️.rs"]`-mounted from the real
location under `🔨️modules/⚡️simulation/⚙️engine/`).

## 1. Public engine API

### Model construction

`Model` (`⚙️engine/🔋️model/🦀️.rs:766-807`) is a flat, plain-`Vec`-based aggregate — there is
**no builder pattern**; you construct it as a struct literal (see the cheat-sheet in §7). It
derives `Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive,
FromValueDerive`, so `Model::default()` plus `..Default::default()` struct-update syntax works
for every field you don't care about.

`EntityId` (model.rs:14) is a plain `pub struct EntityId(pub u32)` — mint one yourself with
`EntityId::new(id)` or `EntityId(id)` (model.rs:29-32). There is no allocator/registry; IDs are
just user-chosen `u32`s that must be unique and cross-referenced correctly by hand (validation
below checks referential integrity but does not mint or dedupe IDs).

**`FixedTable<K, V>` / `admit(capacity)`** (model.rs:36-150) is **not used by `Model` itself**
(Model's fields are plain `Vec<T>`). It's the engine's *runtime* backing structure for
simulation state and results — a boxed slice of `Option<(K, V)>` slots that is allocated exactly
once via `admit(observed_capacity)` (model.rs:116-130: fails if already admitted, or if
`try_reserve_exact` fails) and thereafter only supports sorted binary-search `insert` (fails
with `FixedTableError::Unordered`/`Overflow` if you insert out of order or beyond capacity — no
post-admission growth, ever). It's used for: `SimulationModel.zones`/`.surfaces` (kernel.rs:142-143),
`PrecomputedModel.zone_geometry`/`.surfaces`/`.fenestrations`/`.default_setpoints`/index tables
(precompute.rs:68-79), `MeterTable.meters` (meters.rs:72), `TimeSeriesTable.series` (output.rs:126),
and internal dedup tables + the weather array in `sim.rs` (lines 1501-1504, 1772). This is the
repo's CQRS/"no CRUD, bounded-allocation" convention applied to simulation numerics — every
timestep loop is careful never to reallocate.

### Validation

`Model::validate(&self) -> Result<(), Diagnostics>` **exists only at
`⚙️engine/🔋️model/🦀️.rs:812`, gated `#[cfg(test)]`** — it is **not compiled into production
builds** and cannot be called from outside the crate's own test module (it's also `pub(crate)`).
It checks: at least one zone: zone volume > 0 and unique names; space/surface/fenestration
zone/construction/surface references resolve; surfaces have ≥3 vertices; interzone pairs resolve;
constructions have ≥1 layer and all layer materials resolve; material thickness/conductivity > 0;
thermostat/ideal-loads/humidistat/zone-equipment/mechanical-ventilation/air-loop/daylight-zone
zone references resolve; adjacency-pair surfaces resolve (model.rs:812-934). **There is currently
no validation entry point reachable from a real simulation run** — `Engine::run`/`Engine::job`
never call `Model::validate`. Separately, `crate::error::{Error, Diagnostics, Severity}`
(error.rs) is a real, always-available (non-test-gated) diagnostics type used throughout the
engine (`Results.diagnostics: Diagnostics`, results.rs:63) and by several *component-level*
validators that ARE always compiled: `curves::validate_curve`/`validate_lookup_table`
(curves.rs:119,153), `hvac_topo::validate_topology`/`FluidNode::validate`/`Branch::validate`
(hvac_topo.rs:172,225,237). There is no single "validate the whole `Model`" call you can make in
a production build today — you'd need to either lift `Model::validate` out of `#[cfg(test)]`
(schema-first design in CLAUDE.md would say this should not be test-gated) or build your own
BESTEST harness's own pre-flight checks.

### Run entry points

Three layers, all in `⚙️engine/🧪️sim/🦀️.rs`:

- **`Engine::run(model: Model, config: SimulationConfig) -> Result<Results, Error>`**
  (sim.rs:4597-4642) — the batch/oracle-friendly entry point. Internally calls
  `Engine::job` then drives `EnergyJob::step` in a loop until a terminal `StepOutcome`
  (`Complete`/`Fault`/`Cancelled`), acking preview/checkpoint packets as they arrive, then calls
  `job.take_results()`. **This is what a BESTEST harness should call.**
- **`Engine::job(model, config) -> Result<EnergyJob, EnergyAdmissionRejected>`** (sim.rs:4592-4594)
  — creates the persistent `EnergyJob` used by interactive hosts (the editor's simulation
  session) and by `Engine::run`'s own batch adapter. Wraps `EnergyJob::new` /
  `EnergyJob::admit(operation, model, config, bounds: EnergyNumericalBounds)` (sim.rs:1897,1901).
- **`EnergyJob` implements `InteractiveJob`** (sim.rs:3165) with `step(&mut self, context:
  &mut StepContext<'_>) -> StepOutcome` as the cooperative, budgeted, cancellable stepping
  primitive — this is the repo's CQRS/interactive-job pattern (bounded fuel, checkpoint/resume,
  cancellation, no unbounded work per call), not a plain "run to completion" function. A BESTEST
  driver that just wants a final `Results` should use `Engine::run`; a driver that wants
  progress/cancellation should replicate `Engine::run`'s loop itself.

`SimulationConfig` (kernel.rs:53-83):
```rust
pub struct SimulationConfig {
    pub environment: SimulationEnvironment,   // WeatherRunPeriod | HeatingDesignDay | CoolingDesignDay | CustomDesignPeriod
    pub zone_timestep_minutes: u32,           // default 60
    pub system_timestep_minutes: u32,         // default 60
    pub warmup_days: u32,                     // default 7
    pub run_period_start_month: u8, pub run_period_start_day: u8,
    pub run_period_end_month: u8,   pub run_period_end_day: u8,     // default 1/1 .. 12/31
    pub tolerances: ConvergenceTolerances,     // temp 0.01K, humidity 1e-5, mass flow 1e-4, energy 1W, max_iter 20
    pub schedules: ScheduleSet,
    pub weather: Option<crate::site::EpwWeather>,  // None => synthetic sinusoidal weather (sim.rs:4652-4676)
}
```
`zone_timestep_minutes`/`system_timestep_minutes` are only validated (1..=60) by the mounted
session's config projection (`EnergySimulationConfigProjection::validate`, simulation-session
file line 60-68), not by `SimulationConfig` itself — sub-hourly timesteps below 60 min are
plumbed through but I did not find them exercised end-to-end in `kernel.rs`'s main loop beyond
being stored; treat sub-hourly as unverified for BESTEST purposes (BESTEST itself uses 1-hour
reporting anyway).

### Reading results

`Results` (`⚙️engine/🧾️results/🦀️.rs:56-65`):
```rust
pub struct Results {
    pub time_series: TimeSeriesTable,      // output.rs — string-keyed hourly series
    pub meters: MeterTable,                // meters.rs — string-keyed energy-use meters
    pub summaries: SummaryTables,          // annual_energy/monthly_energy/peak_loads/comfort rows
    pub sizing: SizingTables,              // zone_loads/equipment SizingResult rows
    pub environmental: EnvironmentalMetrics,  // site/source energy kWh, CO2 kg
    pub resilience: ResilienceMetrics,        // hours above/below thresholds, unmet hours
    pub diagnostics: Diagnostics,
    pub run_metadata: RunMetadata,         // model_name/version, weather_location, timesteps, warmup_days, elapsed_ms
}
```

**Time series and meters are NOT a fixed enum of named EnergyPlus-style output variables** —
they are generic `String`-keyed registries (`TimeSeries`/`Meter` structs, output.rs:31-137,
meters.rs:31-86) populated ad hoc by the aggregation pass in `sim.rs`. What is actually produced
today, per zone (`sim.rs:2639-2765`, confirmed by the `AggregateZoneStage` enum
`{Temperature, Heating, Cooling, Fan, Complete}`, sim.rs:1610-1616):

| Semio key (as produced) | What it is | Nearest EnergyPlus output variable |
|---|---|---|
| `"Zone Air Temperature [<ZoneName>]"` (sim.rs:2647, confirmed by test at 5191/5225) | hourly zone air temp, °C | `Zone Mean Air Temperature` |
| `"<ZoneName> Heating"` meter, `FuelType::Electricity`/`EndUse::Heating` (sim.rs:2708-2721) | zone heating energy | `Zone Ideal Loads Supply Air Total Heating Energy` |
| `"<ZoneName> Cooling"` meter, `EndUse::Cooling` (sim.rs:2726-2739) | zone cooling energy | `Zone Ideal Loads Supply Air Total Cooling Energy` |
| `"<ZoneName> Fans"` meter, `EndUse::Fans` (sim.rs:2744-2757) | zone fan electricity | `Fan Electricity Energy` |
| `"Facility Heating"` meter (sim.rs:2773-2789) | facility total heating | `Facility Total Heating Energy` |
| `"Facility PV"` meter, `EndUse::Generators` (sim.rs:2793-2799) | facility PV generation | `Generator Produced DC Electricity Energy` |
| `SummaryTables.annual_energy` rows: `"Electricity"`, `"Natural Gas"`, `"Energy Use Intensity"`, `"Annual Energy Cost"`, `"LCCA Present Value"` (sim.rs:2592-2616) | annual summary | — (custom rollups, no direct E+ equivalent name) |

**Not currently emitted as time series/meters** (would need to be added for full BESTEST
comparison): peak heating/cooling load (only `Meter.peak_demand_w`/`peak_demand_hour` fields
exist per-meter, meters.rs:46-56, not surfaced as a distinct summary row beyond
`SummaryTables.peak_loads: Vec<SummaryRow>` which nothing currently populates in the aggregation
pass I read); surface/window solar gain series; infiltration/ventilation load series; zone
humidity ratio series; surface inside/outside temperatures; unmet-hours per BESTEST's usual
convention (only `ResilienceMetrics.unmet_heating_hours`/`unmet_cooling_hours` exist,
metrics.rs:47-52, computed from the zone temperature history — needs checking whether it's wired
into the finalization pass for every run). `SizingTables` exists but nothing in `sim.rs`'s
aggregation pass populates `zone_loads`/`equipment` today (design-day sizing path is a separate,
less-explored code path in `sizing.rs`, 395 lines, 4 tests).

`Diagnostics`/`Error`/`Severity` (error.rs, 124 lines) — real, simple, always-compiled: `Severity
{Fatal, Severe, Warning, RecurringWarning}`, `Error{severity, message, context}`,
`Diagnostics{messages: Vec<Error>}` with `push`/`has_fatal`/`merge`.

## 2. Weather / EPW ingestion

`⚙️engine/📍️site/🦀️.rs` (310 lines) delegates EPW **text decoding** entirely to stdio's real,
lossless EPW codec — this is a genuine bridge, not a stub (site.rs:1-7 docstring, confirmed):
```rust
use semio_s_plugin_stdio::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwRecord;
use semio_s_plugin_stdio::artifacts::epw::EpwSnapshot;
```
- `EpwWeather::parse(content: &str) -> Result<Self, Error>` (site.rs:102-105) calls
  `semio_s_plugin_stdio::…::io::decode_epw(content)` then `Self::from_snapshot`.
- `EpwWeather::from_snapshot(snapshot: &EpwSnapshot) -> Result<Self, Error>` (site.rs:110-123)
  reads `LOCATION` fields (lat/long/elevation/time-zone/city) and maps every `EpwRecord` through
  `WeatherRecord::try_from` (site.rs:55-83), which **hard-errors on any malformed numeric field**
  (no silent defaulting — `parse_epw_field`, site.rs:51-53) and converts EPW's 1..24 hour-ending
  convention to 0..23 (site.rs:68).
- `WeatherRecord` (site.rs:20-37) carries dry-bulb, dew-point, RH, pressure, wind speed/direction,
  direct-normal + diffuse-horizontal + horizontal-infrared irradiance, precipitation, snow depth
  — the standard EPW hourly fields needed for BESTEST. `humidity_ratio()`/`air_density()` helpers
  exist (site.rs:40-46).
- `EpwWeather::interpolate(hour_index: f64)` (site.rs:130-144) does linear sub-hourly
  interpolation of a subset of fields (temp, dewpoint, RH, wind, DNI, DHI) — others copied
  verbatim from the floor record via `..a`.

**Design days**: `DesignDay`/`DesignDayKind`/`DesignDayHumidity` (site.rs:150-188) — a simple
sinusoidal daily profile (`hourly_dry_bulb`, site.rs:178-187), used for `HeatingDesignDay`/
`CoolingDesignDay` `SimulationConfig.environment` modes. **Solar position**: `solar_position(lat,
lon, day_of_year, hour_solar)` (site.rs:201-211) is a simplified SPA-style calculation (not the
full NREL SPA algorithm — declination via a coarse sinusoid, no atmospheric refraction
correction). **Ground temperature**: `GroundTemperatureModel::{Monthly, Shallow, Deep}`
(site.rs:224-244) — analytical models only, no real soil heat-conduction solve (relevant to
BESTEST cases with a floor slab, e.g. 600-series has a floor but BESTEST's simplified floor
convention typically treats it as adiabatic/insulated in the "ground contact" cases, so this may
be adequate — needs a case-by-case check against the BESTEST spec's own floor boundary
convention). **Sky temperature**: Brunt-type correlation `sky_temperature_k` (site.rs:214-218).

**Example EPW location**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw`
— **Hannover, Niedersachsen, DEU** (`LOCATION,Hannover,Niedersachsen,DEU,semio-fixture,10238,52.37,9.74,1.0,55.0`
— 52.37°N, 9.74°E, elevation 55 m). File is **only 32 lines total**: 8 EPW header lines + **24
data records = exactly one day** (2026-01-15, all 24 hours), not a full annual TMY file. This is
a unit-test fixture, not something you can run a full annual BESTEST case against as-is — a real
BESTEST run needs Denver TMY3 (per the ASHRAE 140 spec) or another full-year EPW file supplied
separately. The energy engine's own `site.rs` test suite (site.rs:263-271) copies the same
8-header + 1-record shape verbatim from this file.

## 3. Physics coverage inventory

No `todo!()`/`unimplemented!()`/`// stub` markers found anywhere in the 50-module engine tree
(broad grep across all `.rs` files, non-test code). Every module below has at least one real
(non-trivial) computation, not just data structs — but several are simplified/analytical rather
than full multi-node solvers. One-line summary per module (from each file's own header
docstring, cross-checked against source for the physics-critical ones):

| Module | What it models | Coverage verdict |
|---|---|---|
| `envelope` | Opaque conduction, convection, surface heat balance | **Real but simplified**: convection is McAdams-type (exterior) / adaptive natural-convection correlation (interior) — reasonable. Conduction is a **first-order (single-RC, one history term) CTF approximation** (`ConductionState`, envelope.rs:56-79), not EnergyPlus's real multi-term CTF nor finite-difference. Exterior/interior surface temps solved via Newton-Raphson / fixed-point iteration (envelope.rs:111-134) — real iterative solves, not constants. |
| `fenestration` | Glazing U/SHGC, frame/divider, condensation | Real center-of-glass + frame/divider area-weighted U-value and SHGC (layer-stack product of transmittances) — **no angular (incidence-angle) SHGC/transmittance variation**, no multi-layer optical ray-tracing (WINDOW-style), no gas-fill convection correlations beyond a fixed `gap_resistance_m2k_w` input. |
| `air_exchange` | Infiltration (4 methods incl. wind/stack), ventilation, interzone mixing | Real — `ScheduledAch`/`PerExteriorArea`/`EffectiveLeakageArea`/`WindAndStack` methods mirror EnergyPlus's `Infiltration:DesignFlowRate` coefficient model closely. BESTEST 600-series (constant 0.5 ACH) maps directly onto `ScheduledAch`. |
| `zone_air` | Zone sensible/latent air balance, integration | **Real, notably good**: BDF3 (third-order backward difference, `bdf3_next_value`, zone_air.rs:103-113) plus an analytical steady-state fallback — this is close to EnergyPlus's own third-order backward-difference zone-temperature-update scheme. |
| `ideal_hvac` | Ideal-loads air system (E+'s `ZoneHVAC:IdealLoadsAirSystem` analogue) | Real: capacity-limited heating/cooling, OA sizing (per-person + per-area), economizer modes (differential dry-bulb/enthalpy/fixed-dry-bulb), humidity control modes. This is the HVAC system BESTEST cases specify — good coverage. |
| `controls` | Thermostats, humidistats, zone load prediction, equipment priority | Real proportional-throttle-range thermostat model; equipment priority queue. |
| `solar` | Beam incidence, shading (overhang), interior solar distribution | Real incidence-angle geometry; shading is a **single flat overhang factor only** (no side fins, no adjacent-building shading, no full sun/shadow polygon clipping) — sufficient for BESTEST 610 (overhang) but not 620/650 (fins) unless those cases can be approximated with the same one-parameter overhang model. Interior solar distribution has 3 hard-coded modes (DirectToFloor/UniformOnSurfaces/SplitFlux-40%-to-floor) — **no view-factor-based distribution** as EnergyPlus's default `FullInteriorAndExterior` uses. |
| `site` | Weather/EPW, design days, solar position, ground temp | Real EPW bridge (see §2); solar position is simplified SPA; ground temperature is analytical only (Monthly/Shallow/Deep), no soil conduction solve. |
| `kernel` | Calendar, multi-rate loop, warmup, predictor-corrector coupling | Real orchestration layer wiring all of the above together per-timestep; `SimulationModel` state uses `FixedTable` for zones/surfaces. |
| `precompute` | Geometry, CTF coefficients, solar factors, zone topology precompute | Real — one-time precomputation pass fed by the model, feeding the kernel. |
| `sizing` | Zone/equipment sizing from design-day calcs | Present (395 lines, 4 tests) but not confirmed wired into the main `Engine::run` aggregation pass populating `Results.sizing` — needs separate verification for autosizing BESTEST cases (most BESTEST cases specify hard equipment capacities/unlimited ideal loads, so this may not block anything). |
| `hvac_topo` | HVAC fluid node/branch/splitter/mixer topology + validation | Real; not needed for ideal-loads-only BESTEST cases. |
| `plant`, `air_system`, `zone_hvac`, `coils`, `fans`, `terminal`, `dispatch` | Full HVAC-plant/air-system stack (pumps, boilers, chillers, VAV/CAV, coils, fan curves, dispatch strategies) | Real, substantial (each has 3-8 tests) — irrelevant to ideal-loads BESTEST cases but relevant if the validation later moves to plant-level cases. |
| `daylight`, `comfort`, `room_air`, `iaq`, `airflow_network`, `evaporative`, `humidity_eq`, `heat_recovery`, `refrigeration`, `electrical`, `shw`, `solar_thermal`, `water`, `faults`, `economics` | Secondary systems (daylighting/glare, PMV/PPD comfort, stratified room-air, CO2/IAQ, pressure-driven AFN, evap cooling, humidifiers, HRV/ERV, refrigeration racks, PV/battery/grid, service hot water, solar thermal collectors, misc water systems, equipment faults, tariffs/LCC) | Real implementations, all out of scope for the core ASHRAE 140 BESTEST cases (600/600FF/900/610/620/630/640/650/900FF/960), relevant only if validation later extends to PV/HVAC/comfort-specific test suites. |
| `gains`, `schedule`, `calendar`, `curves`, `metrics`, `output`, `meters`, `model`, `results`, `error`, `num`, `props`, `material`, `geometry`, `units` | Internal-gains decomposition, schedules, calendar/DST, performance curves, environmental/resilience metrics, output/meter registries, the model schema, results schema, error taxonomy, numerical solvers (Newton-Raphson etc.), psychrometrics, construction thermal properties, surface geometry, SI constants | Real, all straightforward and exercised by unit tests. |

**BESTEST-specific verdict**: the 600-series cases (600, 600FF, 900, 900FF — lightweight/heavyweight,
free-float or ideal-loads, no shading) look **feasible now** given real conduction (albeit
simplified 1st-order CTF — may cause deviation vs E+ for the 900-series heavyweight-mass cases,
where multi-term CTF/finite-difference matters most), real infiltration, real BDF3 zone-air
integration, real ideal-loads HVAC, and real EPW ingestion. **Case 610 (south overhang shading)**
is feasible via the single-overhang shading factor. **Cases 620/640/650 (window
orientation/fins/night-setback/setback-recovery thermostat)** are feasible for orientation and
setback (schedule-driven thermostat setpoints already exist via `ThermostatSpec`/`ScheduleSet`)
but **fins are not modeled** (only overhang) — would need a workaround or an out-of-reach call.
**Case 630 (higher window-to-wall + shading)** similarly limited by the shading model. **Cases
960/195-series (sunspace / analytical solution verification)** are **out of reach** without
additional interior-solar-distribution and interzone/sunspace radiative-coupling work — the
current interior solar distribution is a fixed floor/uniform/split-flux heuristic, not a proper
enclosure radiation model, and case 960 needs a two-zone sunspace with real interzone heat/solar
transfer. **Ground-coupled cases** are limited by the analytical (non-conductive) ground
temperature model.

## 4. Existing tests / validation

**262 `#[test]` functions across all 50 engine modules** (`grep -c` per file, summed via
`grep -ro '#\[test\]' … | wc -l`). Top counts: `sim.rs` 32, `model.rs` 22, `hvac_topo.rs` 12,
`curves.rs`/`kernel.rs` 8, `plant.rs` 7, most others 3-6, three modules (`results.rs`,
`metrics.rs`, `units.rs`) have exactly 1. **None of these are BESTEST-style or
EnergyPlus-comparison tests** — they are unit tests of individual physics functions (e.g.
`ctf_flux_sign_correct`, `steady_flux_cold_outside` in envelope.rs) and, in `sim.rs`
specifically, an extensive set of **wire-protocol / numerical-admission conformance tests**
(`p7c1_*`, `p7c2_*` — see below), not thermal-physics validation.

`✏️s/🔌️plugins/🔋️energy/🪨️tests/🧮️p7c1-energy-numerical-laws.json`,
`🦠️p7c2-energy-retained-wire-mutations.json`, `🔗️p7c2-energy-retained-wire-laws.json` are
**cross-language conformance-oracle specs for the CQRS/interactive-job admission and wire
protocol**, not BESTEST/physics data. Confirmed consumer: **not `📜️script.ts`** (grepped root
`📜️script.ts`, all 35,342 lines — zero hits for `p7c1`/`p7c2`/`retained-wire`; the task's
"around line 26214" pointer no longer matches current content, likely stale from concurrent
edits). The actual consumer is `sim.rs` itself via `include_str!`:
```rust
// sim.rs:5855-5856
fn p7c1_language_agnostic_law_fixture_matches_reference_parser() {
    let source = include_str!("../../../../🪨️tests/🧮️p7c1-energy-numerical-laws.json");
```
This is the CLAUDE.md-mandated "language-agnostic test with a third-party-comparable oracle" —
but for the *engine's own admission-capacity/wire-format laws* (fixed-capacity dimensions like
`zones`, `surfaces`, `weatherRecords`, `timesteps`, `meters`; `FixedTable`'s
one-time-admission/no-grow semantics; the `EnergyWirePacket`/checkpoint/preview/commit/fault
channel protocol with magic bytes `"SMENERGY"`, header/checkpoint/preview byte layouts, and
generation-tagged leasing), **not for building-physics correctness**. `p7c1-energy-numerical-laws.json`
declares admission dimensions and "laws" like `sampleBacking: reserveBeforeInsert`,
`tableBacking: oneTimeAdmittedFixedBoxedSlots`; `p7c2-energy-retained-wire-laws.json` declares
the wire header/checkpoint/preview byte layout and channel capacities/policies;
`p7c2-energy-retained-wire-mutations.json` is a mutation-testing catalog (byte-flip a field,
assert a specific rejection reason). **Conclusion: there is currently no EnergyPlus/BESTEST
oracle or comparison harness anywhere in the repo for this engine** — a new one must be built
from scratch, most likely as a new subdirectory under this ticket or a dedicated
`✏️s/🔌️plugins/🔋️energy/🪨️tests/` fixture set with expected E+ output values, run through
`Engine::run` and compared in a new test/binary.

The engine's own plugin-level `🧪️oracle/🔣️.json` (`✏️s/🔌️plugins/🔋️energy/🧪️oracle/🔣️.json`) is
just repo test-platform boilerplate declaring which crate hosts test adapters
(`semio-s-plugin-stdio-test-oracle`) — `"oracles": []`, `"noOracleDecisions": []` — no real
oracle registered at the plugin level (oracle decisions are made per-subset under
`🗿️artifacts/…/🪆️subsets/✳️any/🔣️oracle.json`, not explored in depth here).

Two fixture-generator functions exist and are directly useful for a BESTEST harness:
`sim::test_model_single_zone()` (sim.rs:4701-4744, literally named **"BESTEST Single Zone"**
in its `model.name` field) and `sim::test_model_full_topology()` (sim.rs:4747+, adds
thermostats, a plant loop, PV, daylight zones) — both are `#[cfg(test)]`-free `pub fn`s (usable
from outside the crate/in a harness binary), giving a ready-made starting point for a BESTEST 600
model (see §7).

## 5. Simulation session (editor-driven run)

Defined at `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs`
(2,833 lines), mounted into the crate as `crate::energy_simulation_session` via
`#[path = "…"] pub mod energy_simulation_session;` at
`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs:511-512`. This is the artifact-neutral,
mounted, worker-driven session backing the `StartSimulation`/cancel/retry/discard/adopt actions
exposed by the editor's simulation window
(`…/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`, 115 lines).

Key mechanics:
- **Model capture**: `ModelCapture`/`CaptureCensus` (lines 251-853) walk the open document's
  `Model` snapshot field-by-field in small bounded steps (one lane/substage per `step_one` call)
  to build an owned copy for admission — a strictly incremental, allocation-bounded copy (never
  reads/allocates more than one field or one vector element per call), consistent with the
  repo's "no unbounded work per step" convention.
- **Admission**: `MountedState::admit_job` (lines 962-1002) calls either
  `EnergyRestoreJob::admit(...)` (checkpoint-token resume path) or `EnergyJob::admit(...)`
  (fresh run), both against `EnergyNumericalBounds::default()`; rejection sets
  `EnergySimulationStatus::Faulted` and stores the rejection for a later `retry_rejected()` call
  (lines 1004-1036).
- **Quality tiers**: `EnergyQualityTier` progression tracked via `EnergyTierProjection` per tier
  index (`quality_tier_index`, lines 229-236): **SteadyStateEstimate(0) → DesignDay(1) →
  CoarseTimestep(2) → Final(3)** — the editor UI (simulation window `🦀️.rs`) renders all four
  tiers' progress (`timestep`/`total_timesteps`/`facility_electricity_kwh`) plus overall status.
- **Status enum**: `EnergySimulationStatus {Idle, Admitting, Queued, Running, Cancelled, Faulted,
  FinalReady, Adopted, Closing}` (lines 127-137) drives both the UI's accessible live-region text
  (aria-live=polite, role=status) and the worker's own step logic.
- **Progress/cancellation**: `worker_step(&mut self, budget: JobBudget) -> JobStep`
  (lines 1135+) checks `self.closing || self.cancel.is_cancelled_now()` first (→ `Cancelled`),
  respects a fuel/deadline `StepBudget` capped at 7ms per call (line 1144), and steps either a
  `restore` job or the main `job`, collecting preview/checkpoint/commit/fault wire packets one at
  a time via `collect_channels_one` (lines 1071-1125) — checkpoints set `checkpoint_ready`,
  commits set `final_ready` + `FinalReady` status, faults set `fault_ready` + `Faulted` status.
- **What the UI can read back**: `EnergySimulationProjection` (lines 158-173) — status, request/
  operation/generation identity, per-tier progress array (`tiers: [Option<EnergyTierProjection>; 4]`),
  `checkpoint_ready`/`final_ready`/`fault_ready`/`adopted` flags. The final `Results` themselves
  are retrieved via the underlying `EnergyJob::take_results()` (sim.rs:2084), not shown directly
  in the projection I read (would need to trace the `Adopt` action further, not done here — file
  continues past line 1173/2833, not fully read).

## 6. Model ↔ structure/zones bridge

`energy_structure_from_model`/`energy_model_from_structure` live at
`✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs:115-125` (390-line file):
```rust
pub fn energy_structure_from_model(model: &crate::model::Model) -> SemioValueSnapshot {
    let value = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
    // wrapped via semio_value_from_json → SemioValueSnapshot
}
pub fn energy_model_from_structure(structure: &SemioValueSnapshot) -> crate::model::Model {
    serde_json::from_value(json_from_semio_value(&structure.root)).unwrap_or_default()
}
```
This is the sole bridge from the typed Rust `Model` to the generic
`s.stdio.semio.value` (`SemioValue`) tree that the editor's "structure"/"zones" tree windows
render (`…/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/🦀️.rs` and
`…/📊️zones/🦀️.rs`) — it round-trips through `serde_json::Value` as an intermediate, with a
hand-written `semio_value_from_json`/`json_from_semio_value` converter (model.rs:61-109) doing
the `serde_json::Value` ↔ `SemioValue` mapping. The **Cargo.toml comment
(`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/Cargo.toml:36-46`) explicitly marks this as an
architecturally pinned exception**, not tech debt: `Model` derives `Serialize`/`Deserialize`
transitively across all 40 fields and the whole 50-module engine tree, so switching the bridge
to the repo's `ToValue`/`FromValue` (`DslValue`) path instead of `serde_json::Value` is now
*possible* (since `DslValue::Number` gained `UInt`/`Int`/`Float` fidelity per ticket
`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`) but is a real, unstarted,
out-of-scope rewrite — `serde`/`serde_json` stay as workspace deps for this one file only
(Cargo.toml:47-48).

**Demo example** (`📚️examples/🎬️demo/🦀️.rs`, `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs`)
is a **7-line placeholder**: `pub const ID: &str = "demo"; pub const LABEL_EN: &str = "Demo";` —
**there is no actual building model behind the demo example yet** (no zones/surfaces/materials
populated). If you need a real reference building to demo against, you must build one — the
`test_model_single_zone`/`test_model_full_topology` fixtures in `sim.rs` (§4 above) are the
closest thing that exists today.

## 7. API cheat-sheet: BESTEST 600 in ~40 lines

`sim::test_model_single_zone()` (sim.rs:4701-4744) is already named "BESTEST Single Zone" and is
most of the way there — reproduced/adapted below (real signatures, not invented):

```rust
use semio_s_plugin_energy::{
    Model, EntityId, Zone, Material, Construction, Surface, SurfaceClass, OutsideBoundary,
    IdealLoadsSystem, Site, SimulationConfig, Engine,
};

let model = Model {
    name: "BESTEST 600".into(),
    version: "1.0".into(),
    site: Site { latitude_deg: 39.83, longitude_deg: -104.65, elevation_m: 1650.0, time_zone_hours: -7.0, north_axis_deg: 0.0 },
    zones: vec![Zone { id: EntityId(1), name: "Zone1".into(), volume_m3: 129.6, multiplier: 1, conditioned: true, part_of_total_floor_area: true }],
    materials: vec![
        Material { id: EntityId(10), name: "Wall Insulation".into(), thickness_m: 0.066, conductivity_w_m_k: 0.04, density_kg_m3: 12.0, specific_heat_j_kg_k: 840.0, thermal_absorptance: 0.9, solar_absorptance: 0.6, visible_absorptance: 0.6 },
        // ... plywood, floor insulation, etc. — one Material per BESTEST construction layer
    ],
    constructions: vec![
        Construction { id: EntityId(20), name: "Exterior Wall".into(), layer_material_ids: vec![EntityId(10) /* , ... */] },
    ],
    surfaces: vec![
        Surface { id: EntityId(30), name: "South Wall".into(), zone_id: EntityId(1), class: SurfaceClass::ExteriorWall,
            vertices_m: vec![[0.0,0.0,0.0], [8.0,0.0,0.0], [8.0,0.0,2.7], [0.0,0.0,2.7]],
            construction_id: EntityId(20), outside_boundary_condition: OutsideBoundary::OutdoorAir,
            sun_exposed: true, wind_exposed: true, multiplier: 1 },
        // ... 3 more walls, roof, floor — BESTEST 600 is an 8m x 6m x 2.7m box
    ],
    ideal_loads: vec![
        IdealLoadsSystem { id: EntityId(40), zone_id: EntityId(1), max_heating_supply_air_temp_c: 50.0,
            min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None,
            outdoor_air_per_person_m3_s: 0.0, outdoor_air_per_area_m3_s_m2: 0.0 },
    ],
    // south windows: two Fenestration entries on the south wall surface, 12m² total (BESTEST 600 spec)
    // infiltration: one Infiltration { method-equivalent via flow_per_exterior_area_m3_s_m2, ScheduledAch 0.5 ACH }
    ..Default::default()
};

let config = SimulationConfig {
    weather: Some(semio_s_plugin_energy::EpwWeather::parse(&std::fs::read_to_string("denver.epw")?)?),
    warmup_days: 7,
    run_period_start_month: 1, run_period_start_day: 1,
    run_period_end_month: 12, run_period_end_day: 31,
    ..Default::default()
};

let results = Engine::run(model, config)?;   // -> Results { time_series, meters, summaries, ... }
let zone_temp = results.time_series.get("Zone Air Temperature [Zone1]");   // hourly °C series
let heating_kwh = results.meters.get("Zone1 Heating").map(|m| m.energy_kwh());
```

Gaps vs. the real BESTEST 600 spec this cheat-sheet glosses over: no `Fenestration` entries
shown (real API exists — `Fenestration { id, name, surface_id, u_value_w_m2k, shgc, vlt,
area_m2, frame_conductance_w_k, divider_conductance_w_k }`, model.rs:312-327); no `Infiltration`
entry shown (real API — model.rs:751-760, `flow_per_exterior_area_m3_s_m2` +
constant/temperature/velocity/velocity² coefficients, matching `InfiltrationSpec` in
air_exchange.rs); thermostat setpoints need a `Thermostat` entry + a `ScheduleSet` schedule for
the always-20°C-heating/27°C-cooling BESTEST setpoints (case 600 is not free-float). `EpwWeather`
needs a real annual Denver TMY3-derived EPW (the bundled `example.epw` is 1 day only, §2).

## 8. Sources consulted (file:line index)

- `⚙️engine/🔋️model/🦀️.rs` — EntityId:14, FixedTable:39-150, Model:766-807, validate:812(`#[cfg(test)]`)
- `⚙️engine/🚨️error/🦀️.rs` — Severity/Error/Diagnostics, whole file (124 lines)
- `⚙️engine/🧪️sim/🦀️.rs` — Engine::job/run:4590-4643, EnergyJob::new/admit:1897/1901, fixtures:4701-4747+, p7c1/p7c2 tests:4788-5855+, aggregation:2639-2799
- `⚙️engine/🌰️kernel/🦀️.rs` — SimulationConfig:53-83, SimulationModel/ZoneState/SurfaceState:113-156
- `⚙️engine/🧾️results/🦀️.rs` — Results:56-65, RunMetadata:69-76
- `⚙️engine/📤️output/🦀️.rs`, `⚙️engine/🧮️meters/🦀️.rs` — TimeSeries/Meter registries
- `⚙️engine/📍️site/🦀️.rs` — EPW bridge:1-146, DesignDay:150-188, solar position/ground temp:191-252
- `⚙️engine/🏢️envelope/🦀️.rs`, `🪟️fenestration/🦀️.rs`, `🔄️air_exchange/🦀️.rs`, `🏠️zone_air/🦀️.rs`, `💯️ideal_hvac/🦀️.rs`, `🎛️controls/🦀️.rs`, `☀️solar/🦀️.rs` — physics coverage detail
- `🗿️artifacts/🔋️model/🦀️.rs` — bridge:33-169; `📦️packages/🦀️rust/Cargo.toml:36-49` — pinned-exception comment
- `🗿️artifacts/…/✳️any/🧵️simulation-session/🦀️.rs` — session mechanics (read lines 1-1173 of 2833)
- `🗿️artifacts/…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs` — editor UI, whole file (115 lines)
- `🗿️artifacts/…/✳️any/📚️examples/🎬️demo/🦀️.rs` — demo placeholder, whole file (7 lines)
- `🪨️tests/🧮️p7c1-energy-numerical-laws.json`, `🦠️p7c2-…mutations.json`, `🔗️p7c2-…laws.json` — consumed by `sim.rs:5855-5856` (`include_str!`), not by root `📜️script.ts`
- `🧪️oracle/🔣️.json` — plugin-level test-platform boilerplate, no real oracle registered
- EPW example: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🧪️example/🌦️.epw` — Hannover, DE, 1 day (24 records)
- Test counts: `grep -ro '#\[test\]' ⚙️engine | wc -l` → 262
