# Exhaustive Semantic Mutation Vocabulary — `s.energy.model`

All paths below are relative to the repo root `/Users/ueli/Documents/semio`. The typed model lives at
`✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs` (1155 lines, hereafter `model.rs`).

## 0. Headline findings (read this first)

1. **`replace-model` is exactly the pattern the mutation taxonomy bans.** `📓️taxonomy.md:57-62` and
   `📓️derivation-rules.md:38-42` (ticket `26/08/12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL`) state
   flatly: *"Only `set-snapshot` (whole-document replacement) is banned... whole-document replace is
   not expressible as an in-history mutation at all; it goes through `ArtifactStore::reset`."*
   `replace-model` (`…/🧬️mutations/♻️replace-model/🦀️.rs:10-14`, payload `{ new_model_json: String }`)
   *is* a whole-document replace of the artifact's only real content (`model: crate::model::Model`).
   Its own diff comment admits it (`…/🔺️diff/🦀️.rs:5-8`): *"falls back to `Model::default()` if
   `payload.new_model_json` doesn't parse... never touches `schema` or `results_json`"* — i.e. malformed
   input silently degrades to an empty model, an outcome the taxonomy explicitly forbids for a real
   mutation. The oracle file itself documents the same boundary: `…/🔮️oracle/🔣️.json`'s rationale
   says its Python second-implementation *"deliberately REFUSES a non-empty `newModelJson`, because no
   schema in this repository states the `model` member's field layout,"* and the Rust adapter marks
   `replace-model` `UNOBSERVABLE`. **This ticket's exhaustive vocabulary is the fix for that gap, not
   an addition alongside it.**
2. **Model already stores a typed `Model`, not JSON text.** `EnergyModelSnapshot`
   (`…/🧬️schema/📸️snapshot/🦀️.rs:22`) and `EnergyModelArtifact`
   (`…/🧬️schema/🦀️.rs:39-57`) both hold `pub model: crate::model::Model` directly (`Model` derives
   `ToValueDerive`/`FromValueDerive`, `model.rs:765`). `replace-model`'s `String` payload is a needless
   round-trip through `serde_json` — the artifact has never needed opaque JSON since the
   `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` migration.
3. **Schedules referenced by `ScheduleId` are not part of the persisted document at all.** Every
   gain/HVAC/infiltration/fault entity in `Model` carries a `ScheduleId` (e.g. `PeopleGain.schedule_id`,
   `model.rs:373`), but the actual schedule value tables (`ConstantSchedule`, `DailySchedule`,
   `WeeklySchedule`, `AnnualSchedule`, `TimeSeriesSchedule` — all in
   `…/⚙️engine/🗓️schedule/🦀️.rs:24-71`, aggregated as `ScheduleSet`, `…/🗓️schedule/🦀️.rs:77-83`) live
   only on `kernel::SimulationConfig.schedules` (`…/⚙️engine/🌰️kernel/🦀️.rs:63`), which is **not** a
   field of `Model` and **not** a field of `EnergyModelSnapshot`/`EnergyModelArtifact`. The only thing
   that ever populates a `SimulationConfig` today is `EnergySimulationConfigProjection`
   (`…/🧵️simulation-session/🦀️.rs:30-56`), an ephemeral per-run "Start Simulation" command projection
   that carries `zone_timestep_minutes`/`warmup_days`/run-period fields only — **it has no schedule
   fields either**, so `SimulationConfig::default()`'s empty `ScheduleSet` is literally the only
   schedule data that has ever existed for this plugin. **There is currently no way, anywhere in this
   codebase, to persist an actual schedule value.** An "exhaustive" catalog that includes
   `create-daily-schedule` etc. requires first deciding where schedule data lives — see §5.1.
4. **`RunPeriod`/weather are not `Model` fields either**, contrary to what the task brief's own file
   list implied. `RunPeriod` (`…/⚙️engine/📅️calendar/🦀️.rs:61-66`) and `EpwWeather`
   (`…/⚙️engine/📍️site/🦀️.rs:89`) are fields of `kernel::SimulationConfig`
   (`…/🌰️kernel/🦀️.rs:53-65`: `run_period_start_month`..`run_period_end_day`, `weather:
   Option<EpwWeather>`), a sibling *engine input*, not part of `Model`/the artifact. `model.rs`'s own
   `Site` struct (`model.rs:238-246`) only carries lat/long/elevation/timezone/north-axis — no run
   period, no weather file reference. See §5.2 for the proposed fix.
5. **`EntityId(u32)` (`model.rs:14`) has no id-minting convention anywhere in this codebase for u32
   ids**, and it is a bare, untagged wrapper — nothing stops a `Zone` and a `Material` from sharing the
   numeric value 5 (`model.rs`'s own tests use disjoint values 1/10/20/30 by convention only, never
   enforced). The closest repo precedent for *numeric* id minting is
   `Part21Document::next_id()` (`…/🗄️stdio/…/📐️part21/🦀️.rs:277-279`: `self.instances.iter().map(|i|
   i.id).max().unwrap_or(0) + 1`), a per-collection max+1 scan; the closest for *string* ids is
   `block3d`'s/`puzzle`'s/`fem`'s `next_id(existing, prefix)` (`…/🧱️block/…/🧬️schema/🦀️.rs:292`, `…/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs:46-40`), which finds the smallest unused
   `"{prefix}{n}"` string, called from the **editor/app-surface layer**, never inside a mutation's own
   `diff`/`inverse`. Every `create-*` payload examined (`CreateStream.stream.id`,
   `CreateVortex.vortex.id`, `CreateLoadCase.load_case.id`) takes a caller-resolved id as an ordinary
   field — no mutation mints its own id. Energy has neither helper. See §5.3.
6. **`FixedTable<K,V>` (`model.rs:39-234`) is dead weight for this ticket** — it is used only by the
   simulation runtime (`…/🌰️kernel`, `…/🧠️precompute`, `…/🧪️sim`, `…/📤️output`, `…/🧮️meters`, per
   `grep -rl FixedTable`), never by `Model` itself, which uses plain `Vec<T>` for every collection
   (`model.rs:770-806`). No admission/no-growth constraint applies to mutating `Model` — this removes
   a hazard the task brief's phrasing ("FixedTable admission semantics that may make in-place edits
   impossible") worried about.

## 1. `Model` field inventory (model.rs:763-807)

`Model` (`model.rs:766-807`) is `#[derive(..., ToValueDerive, FromValueDerive)]`, no `FixedTable`, no
custom serialization — every field below is a plain `Vec<T>`/`Option<T>`/scalar.

| Field | Type | Entity key fields (line) |
|---|---|---|
| `name` | `String` | — (model identity) |
| `version` | `String` | — |
| `site` | `Site` (240-246) | `latitude_deg,longitude_deg,elevation_m,time_zone_hours,north_axis_deg` (all `f64`) |
| `zones` | `Vec<Zone>` (252-259) | `id,name,volume_m3,multiplier:u32,conditioned:bool,part_of_total_floor_area:bool` |
| `spaces` | `Vec<Space>` (262-268) | `id,name,zone_id,floor_area_m2` |
| `surfaces` | `Vec<Surface>` (286-298) | `id,name,zone_id,class:SurfaceClass,vertices_m:Vec<[f64;3]>,construction_id,outside_boundary_condition:OutsideBoundary,sun_exposed:bool,wind_exposed:bool,multiplier:u32` |
| `fenestrations` | `Vec<Fenestration>` (311-322) | `id,name,surface_id,u_value_w_m2k,shgc,vlt,area_m2,frame_conductance_w_k,divider_conductance_w_k` |
| `materials` | `Vec<Material>` (327-338) | `id,name,thickness_m,conductivity_w_m_k,density_kg_m3,specific_heat_j_kg_k,thermal_absorptance,solar_absorptance,visible_absorptance` |
| `constructions` | `Vec<Construction>` (341-346) | `id,name,layer_material_ids:Vec<EntityId>` (ORDERED — outside→inside) |
| `people` | `Vec<PeopleGain>` (369-379) | `id,zone_id,schedule_id,activity_schedule_id,people_per_area,sensible_fraction,latent_fraction,radiant_fraction` |
| `lighting` | `Vec<LightingGain>` (381-391) | `id,zone_id,schedule_id,watts_per_area,radiant_fraction,visible_fraction,return_air_fraction` |
| `equipment` | `Vec<EquipmentGain>` (393-402) | `id,zone_id,schedule_id,watts_per_area,radiant_fraction,latent_fraction` |
| `thermostats` | `Vec<Thermostat>` (406-415) | `id,zone_id,heating_setpoint_schedule_id,cooling_setpoint_schedule_id,heating_throttle_range_k,cooling_throttle_range_k` |
| `humidistats` | `Vec<Humidistat>` (430-439) | `id,zone_id,humidifying_setpoint_schedule_id,dehumidifying_setpoint_schedule_id,humidifying_throttle_range,dehumidifying_throttle_range` |
| `setpoint_managers` | `Vec<SetpointManager>` (450-457) | `id,name,kind:SetpointManagerKind (442-448, tagged enum incl. OutdoorAirReset{4×f64}),schedule_id:Option<ScheduleId>` |
| `ideal_loads` | `Vec<IdealLoadsSystem>` (417-428) | `id,zone_id,max_heating_supply_air_temp_c,min_cooling_supply_air_temp_c,max_heating_capacity_w:Option<f64>,max_cooling_capacity_w:Option<f64>,outdoor_air_per_person_m3_s,outdoor_air_per_area_m3_s_m2` |
| `zone_equipment` | `Vec<ZoneEquipmentAssignment>` (459-468) | `id,zone_id,equipment_type:ZoneEquipmentType (470-481, 8-way enum),priority:u8,heating_capacity_w,cooling_capacity_w` |
| `air_loops` | `Vec<ModelAirLoop>` (483-492) | `id,name,supply_node_id:u32,return_node_id:u32,design_supply_air_flow_m3_s,terminal_zone_ids:Vec<EntityId>` (unordered set) |
| `plant_loops` | `Vec<PlantLoopConfig>` (494-504) | `id,name,loop_type:PlantLoopType (506-512),supply_temperature_c,return_temperature_c,design_flow_kg_s,equipment_ids:Vec<EntityId>` (⚠️ dangling — no plant-equipment collection exists, §5.4) |
| `outdoor_air_systems` | `Vec<OutdoorAirSystem>` (514-521) | `id,air_loop_id,min_oa_flow_m3_s,economizer_enabled:bool` |
| `infiltrations` | `Vec<Infiltration>` (750-760) | `id,zone_id,schedule_id,flow_per_exterior_area_m3_s_m2,constant_term_coefficient,temperature_term_coefficient,velocity_term_coefficient,velocity_squared_term_coefficient` |
| `mechanical_ventilations` | `Vec<MechanicalVentilation>` (555-564) | `id,zone_id,schedule_id,design_flow_m3_s,fan_total_efficiency,fan_delta_pressure_pa` |
| `shading_surfaces` | `Vec<ShadingSurface>` (523-530) | `id,name,vertices_m:Vec<[f64;3]>,transmittance_schedule_id:Option<ScheduleId>` |
| `space_lists` | `Vec<SpaceList>` (532-538) | `id,name,space_ids:Vec<EntityId>` (unordered set) |
| `thermal_enclosures` | `Vec<ThermalEnclosure>` (540-546) | `id,name,zone_ids:Vec<EntityId>` (unordered set) |
| `adjacency_pairs` | `Vec<AdjacencyPair>` (548-553) | `surface_a_id,surface_b_id` — **no own id** (edge collection, §5.5) |
| `airflow_network` | `Option<AirflowNetworkDefinition>` (566-572) | `zone_node_ids:Vec<(EntityId,u32)>,outdoor_node_id:u32,link_ids:Vec<u32>` — singleton |
| `electrical_load_centers` | `Vec<ElectricalLoadCenter>` (574-582) | `id,name,generator_ids/pv_ids/battery_ids:Vec<EntityId>` (⚠️ `generator_ids` dangling — no `Generator` type exists in `Model` at all) |
| `pv_systems` | `Vec<PvSystemAssignment>` (584-594) | `id,dc_capacity_w,area_m2,tilt_deg,azimuth_deg,module_efficiency,inverter_efficiency` — no name |
| `battery_storage` | `Vec<BatteryAssignment>` (596-604) | `id,capacity_kwh,max_charge_w,max_discharge_w,round_trip_efficiency` — no name |
| `shw_systems` | `Vec<ShwSystemConfig>` (606-614) | `id,heater_capacity_w,storage_volume_m3,setpoint_c,schedule_id` |
| `solar_thermal_systems` | `Vec<SolarThermalConfig>` (616-625) | `id,collector_area_m2,efficiency,storage_volume_m3,tilt_deg,azimuth_deg` |
| `refrigeration_systems` | `Vec<RefrigerationConfig>` (627-634) | `id,case_count,design_load_w,defrost_schedule_id` |
| `water_systems` | `Vec<WaterSystemConfig>` (636-643) | `id,fixture_count,peak_flow_l_s,schedule_id` |
| `faults` | `Vec<FaultDefinition>` (645-653) | `id,target_equipment_id:EntityId (⚠️ untyped/undiscriminated, §5.4),fault_type:FaultType (655-663),severity,start_schedule_id` |
| `output_variables` | `Vec<OutputVariableSpec>` (665-671) | `name,key,reporting_frequency:OutputReportFrequency (673-681)` — **no id** (§5.5) |
| `sizing_objects` | `Vec<SizingObject>` (683-690) | `id,zone_id,sizing_type:SizingType (692-698),design_day_type:DesignDayType (700-705)` |
| `daylight_zones` | `Vec<DaylightZoneConfig>` (707-715) | `id,zone_id,illuminance_target_lux,glare_limit,window_transmittance` |
| `room_air_models` | `Vec<RoomAirModelAssignment>` (717-722) | `zone_id,model:RoomAirModelType (724-731)` — **no id, `zone_id` doubles as the key** (§5.5) |
| `ground_temperature` | `GroundTemperatureConfig` (733-745) | `building_surface_c:[f64;12],shallow_c:[f64;12],deep_c:f64` — singleton facet |

`Model::validate()` (`model.rs:812-934`, `#[cfg(test)]` only — not run in production) cross-checks
zone/surface/construction/material ids plus thermostat/ideal-loads/humidistat/zone-equipment/
mechanical-ventilation/air-loop/daylight-zone/adjacency-pair references. It does **not** check
`plant_loops.equipment_ids`, `electrical_load_centers.*_ids`, `faults.target_equipment_id`,
`setpoint_managers.schedule_id`, any gain/HVAC `ScheduleId`, `shading_surfaces.transmittance_schedule_id`,
`space_lists.space_ids`, `thermal_enclosures.zone_ids`, `outdoor_air_systems.air_loop_id`,
`sizing_objects.zone_id`, or `room_air_models.zone_id` — i.e. roughly half the cross-references in the
model are unvalidated today, which matters for deciding cascade-vs-refuse behavior in §4.

## 2. Mutation semantics rules (ticket `26/08/12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL`)

Full verb table: `📓️taxonomy.md:29-55`. Key points used below:

- **`create`/`delete`** (id-keyed): full payload / `id`; inverse `delete`↔`create`; delete captures
  cascade (`taxonomy.md:31-32`).
- **`change`**: one scalar field, addr + `new_<field>`; inverse is `change` with the old value
  (`taxonomy.md:37`).
- **`update`**: one **inseparable** multi-field facet, ALL fields required every time — reserved for
  facets "never meaningfully set one-field-at-a-time" (`derivation-rules.md:23-24`, e.g. `Site`,
  `GroundTemperatureConfig`).
- **`add`/`remove`**: set-like `Vec` membership, owner addr + member (`taxonomy.md:35`,
  `derivation-rules.md:27`).
- **`reorder`**: only if list order is user-meaningful (`derivation-rules.md:28-29`) — true for
  `Construction.layer_material_ids` (physical layer order), false for every other `Vec<EntityId>` set
  field in `Model` (`terminal_zone_ids`, `equipment_ids`, `generator_ids`/`pv_ids`/`battery_ids`,
  `space_ids`, `zone_ids`), which get `add`/`remove` only, no `reorder`.
- **`replace`**: whole-value swap of a large structured sub-payload, e.g. geometry
  (`taxonomy.md:45`) — used here for `vertices_m` and the `SetpointManagerKind` tagged union.
- **`connect`/`disconnect`**: relationship/edge collections (`taxonomy.md:47`,
  `derivation-rules.md:33-34`) — used for `adjacency_pairs` and the artifact's `referenced_model` link.
- **Rule 6, verbatim** (`derivation-rules.md:38-42`): *"Document-level whole-content replace: NONE
  inside the mutation enum... it goes through the store's non-history `reset` path."* This is what
  kills `replace-model` as designed (§0.1, §4.9).
- **Addressing** (`taxonomy.md:95-106`): id-keyed by default; inverse always computed from `base`
  (pre-state); missing target ⇒ `inverse` returns `Vec::new()`.
- **Forbidden**: `SetSnapshot`, bare `Set<WholeObject>` variants, `Vec`-arg pseudo-bulk on a singular
  verb, index addressing where a stable id exists (`taxonomy.md:108-114`).

## 3. Precedent: how `📸️remodel` structures its 34 kinds

(The task said 35; `RemodelingMutation::kinds().len()` is asserted `== 34` at
`…/🧬️mutations/🦀️.rs:326`, and `KINDS.len()` at `…/🦀️.rs:512-548` lists exactly 34 slugs.)

Every mutation is a `🧬️mutations/<emoji><slug>/` directory with a fixed triad
(`derivation-rules.md:49-63`): `🦀️.rs` (payload struct + `impl protocol::MutationKind`, delegating to
siblings), `🔺️diff/🦀️.rs` (real diff logic), `↩️inverse/🦀️.rs` (real inverse logic), plus
`.ts`/`.graphql`/`.proto`/`🧬️.schema.json` mirrors and a `🔣️.json` descriptor
(`schemaVersion,owner,semanticKind,displayName,emoji,aggregateVariant,payloadSchema,textOpcode,
binaryTag,invertibility,diffParticipation,outcomeClasses,composition,requiredLanguageSurfaces`). The
aggregate `RemodelingMutation` enum (`…/🦀️.rs:22-61`) is a flat `#[derive(dsl::Mutations)]` enum, one
tuple variant per kind, dispatch fully generated (no hand-written `apply`/`diff` on the enum).

Confirmed families and one worked cascade example:
- `create-stream`/`delete-stream` (id-keyed, `String` id chosen by the CALLER —
  `🌱create-stream/🧬️.schema.json` requires `stream.id: string`, never minted by the mutation).
- `delete-stream`'s diff (`🪓delete-stream/🔺️diff/🦀️.rs:8-30`) is a **real cascade**: removes the
  stream, then also strips every GCP observation addressing it, emitting an `info`-level
  `mutation.cascade` message — the exact pattern to reuse for e.g. `delete-construction` (§4.3) and
  `delete-schedule` (§5.1) cascading into every referencing entity.
- `delete-stream`'s inverse (`↩️inverse/🦀️.rs:8-13`) reconstructs a `create-stream` from the captured
  BASE record, or `Vec::new()` if the target was already missing.
- `change-stream-sync` (scalar `change`), `add-stream-frame`/`remove-stream-frame` (ordered-list
  member, index-addressed with the BASE/FINAL asymmetry from `taxonomy.md:100-102`),
  `update-ingest-params` (whole-facet `update`, 8 of these — one per `ReconstructionParams` sub-struct),
  `replace-job`/`replace-sparse`/… (`replace`, engine-owned large blobs), `commit-reconstruction`
  (closing domain verb).
- `outcomeClasses` observed: `["applied"]` (unconditional), `["error","applied"]` (missing-target
  refusal, e.g. `delete-stream`); `invertibility` observed: `"explicit-mutation"` (normal) vs energy's
  own `"self"` (only used by the banned whole-swap `replace-model`).

The current `♻️replace-model` (`…/🗿️artifacts/🔋️model/…/🧬️mutations/♻️replace-model/`) is the ONLY
kind in energy's aggregate `EnergyModelMutation` (`…/🧬️mutations/🦀️.rs:14`), `KINDS = ["replace-model"]`
(`♻️replace-model/🦀️.rs:32`), matching the oracle catalog `…/🔮️oracle/🔣️.json`'s
`mutationCatalogs[0].kinds == ["replace-model"]`.

## 4. Proposed catalog

Design choice made explicitly (flagged, not silently assumed): **one `change-<entity>-<field>` per
remaining independent scalar**, matching `remodel`'s and `block3d`'s own precedent (`block3d` mints a
separate `change-object-kind-label`/`change-object-kind-icon`/`change-object-kind-unit` rather than one
bundled `update-object-kind`). `update-*` is reserved strictly for the two genuinely inseparable
document-root facets (`Site`, `GroundTemperatureConfig`) per `derivation-rules.md:23-24`. This is a
judgment call — bundling per-entity scalars into `update-<entity>` facets would roughly halve the count
in §4.13; it is rejected here because none of `Model`'s per-entity scalar groups are validated together
the way `ReconstructionParams`'s 8 sub-structs are, and every sibling exemplar found treats independent
physical properties as independently mutable.

Shared shape for every group below unless noted: `invertibility: "explicit-mutation"`,
`diffParticipation: "detect"`, `composition: "atomic"`. `create-*`: `outcomeClasses: ["applied"]`
(or `["applied","warning"]` if duplicate ids are treated as no-op rather than error — recommend
**error** on duplicate id, matching `derivation-rules.md`'s "reject and rewrite" spirit for silent
degradation). `delete-*`/`change-*`/`add-*`/`remove-*` targeting a specific id:
`outcomeClasses: ["error","applied"]` (missing-target ⇒ `mutation.target-missing` error, exactly
`delete-stream`'s pattern). Inverse: `create`↔`delete`, `change` is its own inverse (old value from
`base`), `add`↔`remove`.

### 4.1 Model root (name/version/site/ground-temperature/airflow-network)

| Kind | Verb | Payload | Cascade | Fixture |
|---|---|---|---|---|
| `rename-model` | rename | `{new_name: String}` | none | `🏛️renames-bestest-600-case-600-to-case-600fc` |
| `change-model-version` | change | `{new_version: String}` | none | `🏛️bumps-bestest-600-to-schema-version-2` |
| `update-site` | update | `{site: Site}` (all 5 fields) | none | `🏛️relocates-bestest-600-to-denver-colorado` |
| `update-ground-temperature` | update | `{ground_temperature: GroundTemperatureConfig}` | none | `🏛️sets-monthly-ground-temperatures-for-denver` |
| `replace-airflow-network` | replace | `{airflow_network: Option<AirflowNetworkDefinition>}` (nullable ⇒ covers set AND clear) | none (network is engine-consumed only) | `🏛️replaces-the-airflow-network-topology-for-the-envelope-model` |

`airflow_network` is a document-root `Option<T>` singleton (not a collection), so it gets one `replace`
kind rather than create/delete — matches `taxonomy.md:45`'s "whole-value swap of a large structured
sub-payload," and the nullable payload lets one kind both attach and detach it (no separate
`clear-airflow-network` needed).

### 4.2 Zones & spaces

| Entity | Kinds |
|---|---|
| `Zone` (id,name,volume_m3,multiplier,conditioned,part_of_total_floor_area) | `create-zone`, `delete-zone` (cascade: refuse if any space/surface/thermostat/… still references it — see below), `rename-zone`, `change-zone-volume`, `change-zone-multiplier`, `change-zone-conditioned`, `change-zone-floor-area-participation` — **7** |
| `Space` (id,name,zone_id,floor_area_m2) | `create-space`, `delete-space`, `rename-space`, `change-space-floor-area`, `change-space-zone` (reassign owner — see §5.6 on whether this is `change` or a hierarchy `move`) — **5** |

`delete-zone` cascade decision (flag, not resolved): `Zone` is referenced by ~15 other collections
(`spaces.zone_id`, `surfaces.zone_id`, every gain/HVAC/infiltration/mechanical-ventilation/sizing/
daylight/room-air-model `zone_id`, `air_loops.terminal_zone_ids`, `thermal_enclosures.zone_ids`).
`delete-stream`'s cascade style (strip references, emit `mutation.cascade` info) does not scale cleanly
here — cascading a zone delete would need to also delete every space/surface IN that zone (which
themselves cascade to fenestrations, adjacency pairs...). Recommend **refuse `delete-zone` while any
space or surface still references it** (`mutation.target-in-use` error naming the blockers), same as a
foreign-key `RESTRICT`, and require `delete-space`/`delete-surface` first — cheaper to reason about and
matches the fact that `Model::validate()` already treats these as hard errors, not warnings.

### 4.3 Geometry & envelope

| Entity | Kinds |
|---|---|
| `Surface` (id,name,zone_id,class,vertices_m,construction_id,outside_boundary_condition,sun_exposed,wind_exposed,multiplier) | `create-surface`, `delete-surface` (cascade: strip fenestrations + adjacency pairs referencing it, `delete-stream`-style), `rename-surface`, `change-surface-zone`, `change-surface-class`, `replace-surface-vertices`, `change-surface-construction`, `change-surface-boundary-condition`, `change-surface-sun-exposed`, `change-surface-wind-exposed`, `change-surface-multiplier` — **11** |
| `Fenestration` (id,name,surface_id,u_value_w_m2k,shgc,vlt,area_m2,frame_conductance_w_k,divider_conductance_w_k) | `create-fenestration`, `delete-fenestration`, `rename-fenestration`, `change-fenestration-surface`, `change-fenestration-u-value`, `change-fenestration-shgc`, `change-fenestration-vlt`, `change-fenestration-area`, `change-fenestration-frame-conductance`, `change-fenestration-divider-conductance` — **10** |
| `ShadingSurface` (id,name,vertices_m,transmittance_schedule_id) | `create-shading-surface`, `delete-shading-surface`, `rename-shading-surface`, `replace-shading-surface-vertices`, `change-shading-surface-transmittance-schedule` — **5** |
| `AdjacencyPair` (surface_a_id,surface_b_id, **no id**) | `connect-surfaces{surface_a_id,surface_b_id}`, `disconnect-surfaces{surface_a_id,surface_b_id}` (address by the pair itself — §5.5) — **2** |

`replace-surface-vertices`/`replace-shading-surface-vertices` payload is `{id, vertices_m:
Vec<[f64;3]>}` — a real geometry array, not a JSON string (unlike the current `replace-model`); minimum
3 vertices per `Model::validate()`'s own rule (`model.rs:846-848`), though that check is
`#[cfg(test)]`-only today (§5.7).

### 4.4 Constructions & materials

| Entity | Kinds |
|---|---|
| `Material` (id,name,thickness_m,conductivity_w_m_k,density_kg_m3,specific_heat_j_kg_k,thermal_absorptance,solar_absorptance,visible_absorptance) | `create-material`, `delete-material` (refuse if any construction still lists it as a layer), `rename-material`, `change-material-thickness`, `change-material-conductivity`, `change-material-density`, `change-material-specific-heat`, `change-material-thermal-absorptance`, `change-material-solar-absorptance`, `change-material-visible-absorptance` — **10** |
| `Construction` (id,name,layer_material_ids ORDERED) | `create-construction`, `delete-construction` (refuse if any surface still references it), `rename-construction`, `add-construction-layer{id,material_id,index}`, `remove-construction-layer{id,index}`, `reorder-construction-layers{id,from,to}` — **6** |

`layer_material_ids` is the ONE genuinely ordered `Vec<EntityId>` in `Model` where order carries
physical meaning (heat flows outside→inside through the listed layer sequence) — the only field in the
whole model that earns `reorder` per `derivation-rules.md:28-29`.

### 4.5 Internal gains

| Entity | Kinds |
|---|---|
| `PeopleGain` (id,zone_id,schedule_id,activity_schedule_id,people_per_area,sensible_fraction,latent_fraction,radiant_fraction) | create/delete, `change-{zone,schedule,activity-schedule,people-per-area,sensible-fraction,latent-fraction,radiant-fraction}` — **9** |
| `LightingGain` (id,zone_id,schedule_id,watts_per_area,radiant_fraction,visible_fraction,return_air_fraction) | create/delete, `change-{zone,schedule,watts-per-area,radiant-fraction,visible-fraction,return-air-fraction}` — **8** |
| `EquipmentGain` (id,zone_id,schedule_id,watts_per_area,radiant_fraction,latent_fraction) | create/delete, `change-{zone,schedule,watts-per-area,radiant-fraction,latent-fraction}` — **7** |
| `Infiltration` (id,zone_id,schedule_id,flow_per_exterior_area_m3_s_m2,constant/temperature/velocity/velocity_squared_term_coefficient) | create/delete, `change-{zone,schedule,flow-per-exterior-area,constant-term-coefficient,temperature-term-coefficient,velocity-term-coefficient,velocity-squared-term-coefficient}` — **9** |
| `MechanicalVentilation` (id,zone_id,schedule_id,design_flow_m3_s,fan_total_efficiency,fan_delta_pressure_pa) | create/delete, `change-{zone,schedule,design-flow,fan-total-efficiency,fan-delta-pressure}` — **7** |

None of these have a `name` field — no `rename-*` kind for any of them.

### 4.6 HVAC (zone-level)

| Entity | Kinds |
|---|---|
| `Thermostat` | create/delete, `change-{zone,heating-setpoint-schedule,cooling-setpoint-schedule,heating-throttle-range,cooling-throttle-range}` — **7** |
| `Humidistat` | create/delete, `change-{zone,humidifying-setpoint-schedule,dehumidifying-setpoint-schedule,humidifying-throttle-range,dehumidifying-throttle-range}` — **7** |
| `IdealLoadsSystem` | create/delete, `change-{zone,max-heating-supply-air-temp,min-cooling-supply-air-temp,max-heating-capacity,max-cooling-capacity,outdoor-air-per-person,outdoor-air-per-area}` — **9** |
| `ZoneEquipmentAssignment` (equipment_type is an 8-way enum, treated as one scalar) | create/delete, `change-{zone,equipment-type,priority,heating-capacity,cooling-capacity}` — **7** |
| `DaylightZoneConfig` | create/delete, `change-{zone,illuminance-target,glare-limit,window-transmittance}` — **6** |
| `SizingObject` | create/delete, `change-{zone,sizing-type,design-day-type}` — **5** |
| `RoomAirModelAssignment` (no id — `zone_id` IS the key, §5.5) | `create-room-air-model-assignment{zone_id,model}`, `delete-room-air-model-assignment{zone_id}`, `change-room-air-model{zone_id,new_model}` — **3** |

### 4.7 HVAC (loop-level) & plant

| Entity | Kinds |
|---|---|
| `SetpointManager` (id,name,kind:tagged-union,schedule_id) | `create-setpoint-manager`, `delete-setpoint-manager`, `rename-setpoint-manager`, `replace-setpoint-manager-kind{id,new_kind}` (tagged union ⇒ `replace`, not `change`, since the payload shape differs per variant), `change-setpoint-manager-schedule` — **5** |
| `ModelAirLoop` (id,name,supply_node_id,return_node_id,design_supply_air_flow_m3_s,terminal_zone_ids) | create/delete, rename, `change-{supply-node-id,return-node-id,design-supply-air-flow}`, `add-air-loop-terminal-zone{id,zone_id}`, `remove-air-loop-terminal-zone{id,zone_id}` — **8** |
| `PlantLoopConfig` (id,name,loop_type,supply/return_temperature_c,design_flow_kg_s,equipment_ids ⚠️) | create/delete, rename, `change-{loop-type,supply-temperature,return-temperature,design-flow}`, `add-plant-loop-equipment{id,equipment_id}`, `remove-plant-loop-equipment{id,equipment_id}` — **9** |
| `OutdoorAirSystem` (id,air_loop_id,min_oa_flow_m3_s,economizer_enabled) | create/delete, `change-{air-loop,min-oa-flow,economizer-enabled}` — **5** |

`add-plant-loop-equipment`/`remove-plant-loop-equipment` are **speculative** — `equipment_ids` has no
backing collection to draw a real id from (§5.4); until a plant-equipment catalog exists these two
kinds cannot be given a real create/delete counterpart to validate against.

### 4.8 Electrical, renewables & other systems

| Entity | Kinds |
|---|---|
| `ElectricalLoadCenter` (id,name,generator_ids ⚠️,pv_ids,battery_ids) | create/delete, rename, `add-/remove-electrical-load-center-generator` (⚠️ same dangling-reference problem as plant equipment), `add-/remove-electrical-load-center-pv{id,pv_id}`, `add-/remove-electrical-load-center-battery{id,battery_id}` — **9** |
| `PvSystemAssignment` (no name) | create/delete, `change-{dc-capacity,area,tilt,azimuth,module-efficiency,inverter-efficiency}` — **7** |
| `BatteryAssignment` (no name) | create/delete, `change-{capacity,max-charge,max-discharge,round-trip-efficiency}` — **5** |
| `ShwSystemConfig` | create/delete, `change-{heater-capacity,storage-volume,setpoint,schedule}` — **5** |
| `SolarThermalConfig` | create/delete, `change-{collector-area,efficiency,storage-volume,tilt,azimuth}` — **6** |
| `RefrigerationConfig` | create/delete, `change-{case-count,design-load,defrost-schedule}` — **4** |
| `WaterSystemConfig` | create/delete, `change-{fixture-count,peak-flow,schedule}` — **4** |
| `FaultDefinition` (target_equipment_id ⚠️ untyped) | create/delete, `change-{target-equipment,fault-type,severity,start-schedule}` — **5** |

`pv_ids`/`battery_ids` on `ElectricalLoadCenter` at least reference real collections
(`pv_systems`/`battery_storage`) — those two `add`/`remove` pairs are NOT speculative, unlike
`generator_ids`.

### 4.9 Grouping collections

| Entity | Kinds |
|---|---|
| `SpaceList` (id,name,space_ids unordered) | create/delete, rename, `add-space-list-member{id,space_id}`, `remove-space-list-member{id,space_id}` — **5** |
| `ThermalEnclosure` (id,name,zone_ids unordered) | create/delete, rename, `add-thermal-enclosure-zone{id,zone_id}`, `remove-thermal-enclosure-zone{id,zone_id}` — **5** |

### 4.10 Outputs

| Entity | Kinds |
|---|---|
| `OutputVariableSpec` (name,key,reporting_frequency — **no id**) | `add-output-variable{spec}`, `remove-output-variable{name,key}` (composite natural key — §5.5) — **2** |

### 4.11 `replace-model` disposition

Per §0.1/§2's rule 6, the clean-long-term answer (CLAUDE.md: "no backward compatibility... aim for
clean long term solution... must not be pragmatic") is: **remove `replace-model` from
`EnergyModelMutation` entirely.** Whole-model load (open a `.energy` file, paste-over, load an example)
moves to `ArtifactStore::reset`, exactly like every other composed artifact already does per this
ticket's own governing rule — it stops being a `Mutation` at all, so the "honest degradation to
`Model::default()` on parse failure" hazard (§0.1) disappears (a `reset` is not required to be
partial-tolerant the way an in-history mutation must be). This also resolves the standing oracle gap
(§0.1): with `replace-model` gone, `energy-model-1-mutate`'s Python second implementation gets replaced
by real per-kind coverage instead of one un-observable no-op vector.

If the dev instead wants to KEEP an in-history "paste a whole model" gesture (e.g. for a
scripting/import affordance), the task's fallback — a typed `Model` payload instead of `newModelJson:
String` — is a one-line schema change (`pub struct ReplaceModel { pub new_model: crate::model::Model
}`, since `Model` already derives `ToValue`/`FromValue`), but it should be renamed away from `replace`
(reserved for a *targeted* large sub-payload per `taxonomy.md:45`) to a clearly-marked domain verb
(e.g. `import-model`) with an explicit taxonomy-exception comment, matching
`derivation-rules.md:38-42`'s own suggested alternative of "several batched semantic mutations under
one `Edit`" if the real gesture turns out to be composable from the granular kinds above instead.

### 4.12 New `Model` fields this catalog implies (recommended, not yet in `model.rs`)

To make "run/outputs" and "schedules" genuinely part of the model document (§0.3, §0.4) rather than
ephemeral session state:

- `run_period: RunPeriod` (moved from `SimulationConfig` into `Model`) → `update-run-period{run_period}`
  (facet — `start_month/start_day/end_month/end_day/year` are one inseparable EnergyPlus-style
  `RunPeriod` object, matches the `update` exception).
- `weather_link: Option<store::ArtifactLink>` (mirrors `referenced_model`'s existing link-slot pattern,
  `…/🧬️schema/🦀️.rs:50-53`, pointing at a `🌦️epw` stdio artifact instead of embedding `EpwWeather`
  inline) → `bind-weather-file{link}` / `unbind-weather-file` (domain verbs, parameterization
  attach/detach per `taxonomy.md:48`).
- `referenced_model` ALREADY exists on the snapshot (`…/📸️snapshot/🦀️.rs:26`) but has **no mutation at
  all** today — propose `connect-referenced-model{link}` / `disconnect-referenced-model` (`taxonomy.md:47`'s
  `connect`/`disconnect` pair for a relationship-to-another-artifact).

These three add **6** kinds to the model-root group in §4.1 (10 → 16 there).

### 4.13 Final count

| Group | Kinds |
|---|---|
| Model root (§4.1, incl. §4.12's 6) | 16 |
| Zones & spaces (§4.2) | 12 |
| Geometry & envelope (§4.3) | 28 |
| Constructions & materials (§4.4) | 16 |
| Internal gains (§4.5) | 40 |
| HVAC zone-level (§4.6) | 42 |
| HVAC loop-level & plant (§4.7) | 27 |
| Electrical/renewables/other systems (§4.8) | 45 |
| Grouping collections (§4.9) | 10 |
| Outputs (§4.10) | 2 |
| **Total** | **238** |

This is ~7× `remodel`'s 34 — proportionate, not inflated: `Model` has 3.5× as many distinct entity
collections (36 vs `remodel`'s ~10) and averages more independent scalar physical properties per entity
(a `Material` alone has 7 independently-editable scalars). Bundling every entity's remaining scalars
into one `update-<entity>` facet (rejected above, but a real alternative) would collapse the 238 to
roughly **120** (one `create`/`delete`/`rename` + one `update` per entity, keeping `add`/`remove`/
`reorder`/`replace` where the field is a collection or large payload, not a scalar) — worth a second
opinion from the dev before committing to per-field granularity at this scale.

## 5. Ambiguities & hazards

### 5.1 Where do schedule value definitions live?
`ScheduleId` is referenced ~15 times across `Model` but `ConstantSchedule`/`DailySchedule`/
`WeeklySchedule`/`AnnualSchedule`/`TimeSeriesSchedule` (`…/🗓️schedule/🦀️.rs:24-71`) are not stored
anywhere reachable from the artifact (§0.3). Before any `create-*-schedule` kind can be designed,
someone must decide: (a) add a `schedules: ScheduleSet`-shaped set of `Vec` fields directly to `Model`
(cleanest, matches this ticket's "no legacy, get everything working" mandate, and gives every
`ScheduleId` a real referent to validate against), or (b) keep schedules in a separate artifact/config
entirely and leave every `ScheduleId` in this catalog as an unvalidated opaque handle forever. Not
resolved here — flagged as the single biggest open design question this ticket surfaces.

### 5.2 `RunPeriod`/weather currently belong to `SimulationConfig`, not `Model`
§4.12 proposes moving them into `Model`. This is an architecture change touching
`…/⚙️engine/🌰️kernel/🦀️.rs:53-65`, `…/🧵️simulation-session/🦀️.rs:30-83` (the "Start Simulation"
command projection would need to stop carrying its own copy once `Model` owns the canonical value —
or the projection becomes a per-run OVERRIDE of the model's stored run period, which is itself a
design decision with no obvious default). Flagging, not resolving.

### 5.3 `EntityId(u32)` minting has no established convention
Every `create-*` mutation payload in this catalog needs the caller to supply a concrete, non-colliding
`EntityId` — no mutation may mint its own id (repo-wide convention, §0.5). Recommended fix: add an
editor/app-surface-layer helper analogous to `Part21Document::next_id()`
(`…/📐️part21/🦀️.rs:277-279`, `max(existing) + 1`) **scoped per collection**, e.g.
`fn next_zone_id(model: &Model) -> EntityId { EntityId(model.zones.iter().map(|z| z.id.0).max().unwrap_or(0) + 1) }`
— NOT a single global counter, because `EntityId` carries no type tag and different collections'
id spaces are never compared against each other (a `Zone` and a `Material` sharing the numeric value 5
is harmless today; `model.rs`'s own test fixtures already rely on disjoint-by-convention values 1/10/
20/30, `model.rs:981-994`). This helper does not exist yet anywhere in the energy plugin.

### 5.4 Dangling/untyped `EntityId` references with no owning collection
- `PlantLoopConfig.equipment_ids: Vec<EntityId>` (`model.rs:503`) — no `Chiller`/`Boiler`/`Pump`/
  central-plant-equipment struct exists in `Model` at all. `add-plant-loop-equipment`/
  `remove-plant-loop-equipment` (§4.7) are speculative until such a collection is added.
- `ElectricalLoadCenter.generator_ids: Vec<EntityId>` (`model.rs:579`) — same problem, no `Generator`
  type in `Model`. (`pv_ids`/`battery_ids` on the same struct ARE backed by real collections.)
- `FaultDefinition.target_equipment_id: EntityId` (`model.rs:649`) — untyped/undiscriminated: could
  point at a `zone_equipment` entry, a nonexistent plant/central-equipment entry, or nothing at all,
  and `Model::validate()` never checks it. `change-fault-target-equipment` (§4.8) cannot be given a
  real referential-integrity check until this is resolved (e.g. a tagged `TargetEquipmentRef` enum
  naming which collection it points into).

### 5.5 Collections with no stable id
- `AdjacencyPair` (`model.rs:550-553`): no id field; `connect-surfaces`/`disconnect-surfaces` (§4.3)
  must address by the `(surface_a_id, surface_b_id)` pair itself. Two independent pairs could name the
  same unordered surface set twice (nothing prevents a duplicate `AdjacencyPair`) — `connect-surfaces`
  should probably refuse (or no-op) if the pair already exists; not specified in `Model::validate()`
  today.
- `OutputVariableSpec` (`model.rs:667-671`): no id; `add-output-variable`/`remove-output-variable`
  (§4.10) use `(name, key)` as a natural composite key, but nothing enforces that pair's uniqueness
  either — two identical `(name, key)` entries could coexist, making `remove-output-variable` ambiguous
  about which one it targets (recommend: remove ALL matches, or require the composite key be unique and
  make `add-output-variable` refuse a duplicate).
- `RoomAirModelAssignment` (`model.rs:719-722`): no id; `zone_id` doubles as both the address AND a
  payload field, an unusual shape versus every other id-keyed entity in `Model` (§4.6). Functionally a
  keyed-map (`zone_id → RoomAirModelType`) rather than a genuine collection — `create`/`delete` here
  really mean "does this zone have an override, or does it fall back to the engine default
  (`RoomAirModelType::WellMixed`, inferred from `…/🛏️room_air/🦀️.rs` — not confirmed in this pass)."

### 5.6 `Space.zone_id` / hierarchy-vs-scalar ambiguity
`derivation-rules.md` rule 5 (`derivation-rules.md:35-37`) mandates `move-to-<container>` for
"hierarchy (parent_id / nesting field)" — the exact trap the taxonomy's own preamble warns about
(`taxonomy.md:10-15`). `Space.zone_id` (`model.rs:266`) is structurally a foreign key, not a recursive
parent/child nesting (zones don't nest inside zones), so §4.2 defaults it to a plain `change-space-zone`
scalar reassignment under the general axis — but this is exactly the shape of trap the taxonomy warns
readers about, so it deserves a second look before implementation. Same question applies to every other
bare `zone_id`/`surface_id`/`air_loop_id` foreign key across the catalog (all treated as `change-*`
here, consistently).

### 5.7 `Model::validate()` is test-only
`model.rs:811` — `#[cfg(test)] pub(crate) fn validate(&self)`. None of its checks (vertex count ≥ 3,
positive volume/thickness/conductivity, non-empty construction layers, duplicate zone names, ~10
referential-integrity checks) run in production today. Every cascade/refuse behavior proposed in §4
assumes these become REAL, always-on checks inside each mutation's `diff` (mirroring `delete-stream`'s
handcrafted `mutation.target-missing` error), not a reuse of `validate()` as currently scoped.

### 5.8 Sync vs. async `MutationKind`
`remodel`'s `impl protocol::MutationKind` methods are `async fn diff/inverse/label/target`
(`…/🪓delete-stream/🦀️.rs:26-35`); `block3d`'s and energy's own `replace-model` are plain, synchronous
`fn` (`…/🌀create-vortex/🦀️.rs:25`, `…/♻️replace-model/🦀️.rs:20-30`). Not investigated further in this
pass (out of scope), but worth checking against `protocol::MutationKind`'s trait definition before
authoring 238 new leaves, so the whole new catalog picks one convention consistently rather than mixing
sync and async across kinds.
