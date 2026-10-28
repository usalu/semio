# Native Mutation Category Audit

{
  "other": 204,
  "scalar-row": 99,
  "scalar-row-cross-field": 3
}

- `change-fault-severity` scalar-row: `faults.severity` f64
- `change-surface-wind-exposed` scalar-row: `surfaces.wind_exposed` bool
- `change-surface-sun-exposed` scalar-row: `surfaces.sun_exposed` bool
- `change-fenestration-fin-offset` scalar-row: `fenestrations.fin_offset_m` f64
- `change-fenestration-overhang-offset` scalar-row: `fenestrations.overhang_offset_m` f64
- `change-fenestration-fin-depth` scalar-row: `fenestrations.fin_depth_m` f64
- `change-fenestration-overhang-depth` scalar-row: `fenestrations.overhang_depth_m` f64
- `change-infiltration-flow-per-exterior-area` scalar-row: `infiltrations.flow_per_exterior_area_m3_s_m2` f64
- `change-infiltration-design-flow-ach` scalar-row: `infiltrations.design_flow_ach` f64
- `change-fenestration-height` scalar-row: `fenestrations.height_m` f64
- `change-fenestration-sill-height` scalar-row: `fenestrations.sill_height_m` f64
- `change-fenestration-area` scalar-row: `fenestrations.area_m2` f64
- `change-infiltration-discharge-coefficient` scalar-row: `infiltrations.discharge_coefficient` f64
- `change-infiltration-effective-leakage-area` scalar-row: `infiltrations.effective_leakage_area_m2` f64
- `change-infiltration-temperature-term-coefficient` scalar-row: `infiltrations.temperature_term_coefficient` f64
- `change-infiltration-stack-height` scalar-row: `infiltrations.stack_height_m` f64
- `change-infiltration-constant-term-coefficient` scalar-row: `infiltrations.constant_term_coefficient` f64
- `change-infiltration-velocity-term-coefficient` scalar-row: `infiltrations.velocity_term_coefficient` f64
- `change-infiltration-velocity-squared-term-coefficient` scalar-row: `infiltrations.velocity_squared_term_coefficient` f64
- `change-glazing-material-thickness` scalar-row: `glazing_materials.thickness_m` f64
- `change-glazing-material-solar-transmittance` scalar-row: `glazing_materials.solar_transmittance` f64
- `change-glazing-material-visible-transmittance` scalar-row: `glazing_materials.visible_transmittance` f64
- `change-gas-material-thickness` scalar-row: `gas_materials.thickness_m` f64
- `change-glazing-material-conductivity` scalar-row: `glazing_materials.conductivity_w_m_k` f64
- `change-lighting-gain-watts-per-area` scalar-row: `lighting.watts_per_area` f64
- `change-lighting-gain-visible-fraction` scalar-row: `lighting.visible_fraction` f64
- `change-lighting-gain-radiant-fraction` scalar-row: `lighting.radiant_fraction` f64
- `change-lighting-gain-return-air-fraction` scalar-row: `lighting.return_air_fraction` f64
- `change-plant-loop-design-flow` scalar-row: `plant_loops.design_flow_kg_s` f64
- `change-plant-loop-supply-temperature` scalar-row: `plant_loops.supply_temperature_c` f64
- `change-plant-loop-return-temperature` scalar-row: `plant_loops.return_temperature_c` f64
- `change-shw-system-heater-capacity` scalar-row: `shw_systems.heater_capacity_w` f64
- `change-shw-system-setpoint` scalar-row: `shw_systems.setpoint_c` f64
- `change-shw-system-storage-volume` scalar-row: `shw_systems.storage_volume_m3` f64
- `change-battery-max-charge` scalar-row: `battery_storage.max_charge_w` f64
- `change-battery-max-discharge` scalar-row: `battery_storage.max_discharge_w` f64
- `change-battery-round-trip-efficiency` scalar-row: `battery_storage.round_trip_efficiency` f64
- `change-battery-capacity` scalar-row: `battery_storage.capacity_kwh` f64
- `change-refrigeration-system-design-load` scalar-row: `refrigeration_systems.design_load_w` f64
- `change-refrigeration-system-case-count` scalar-row: `refrigeration_systems.case_count` u32
- `change-ideal-loads-system-min-cooling-supply-air-temp` scalar-row: `ideal_loads.min_cooling_supply_air_temp_c` f64
- `change-ideal-loads-system-outdoor-air-per-area` scalar-row: `ideal_loads.outdoor_air_per_area_m3_s_m2` f64
- `change-ideal-loads-system-outdoor-air-per-person` scalar-row: `ideal_loads.outdoor_air_per_person_m3_s` f64
- `change-ideal-loads-system-max-heating-supply-air-temp` scalar-row: `ideal_loads.max_heating_supply_air_temp_c` f64
- `change-outdoor-air-system-economizer-enabled` scalar-row: `outdoor_air_systems.economizer_enabled` bool
- `change-outdoor-air-system-min-oa-flow` scalar-row: `outdoor_air_systems.min_oa_flow_m3_s` f64
- `change-mechanical-ventilation-fan-total-efficiency` scalar-row: `mechanical_ventilations.fan_total_efficiency` f64
- `change-mechanical-ventilation-design-flow` scalar-row: `mechanical_ventilations.design_flow_m3_s` f64
- `change-mechanical-ventilation-fan-delta-pressure` scalar-row: `mechanical_ventilations.fan_delta_pressure_pa` f64
- `change-water-system-peak-flow` scalar-row: `water_systems.peak_flow_l_s` f64
- `change-water-system-fixture-count` scalar-row: `water_systems.fixture_count` u32
- `change-equipment-gain-watts-per-area` scalar-row: `equipment.watts_per_area` f64
- `change-equipment-gain-radiant-fraction` scalar-row: `equipment.radiant_fraction` f64
- `change-equipment-gain-latent-fraction` scalar-row: `equipment.latent_fraction` f64
- `change-air-loop-supply-node` scalar-row: `air_loops.supply_node_id` u32
- `change-air-loop-design-supply-air-flow` scalar-row: `air_loops.design_supply_air_flow_m3_s` f64
- `change-air-loop-return-node` scalar-row: `air_loops.return_node_id` u32
- `change-daylight-zone-window-transmittance` scalar-row: `daylight_zones.window_transmittance` f64
- `change-daylight-zone-glare-limit` scalar-row: `daylight_zones.glare_limit` f64
- `change-daylight-zone-illuminance-target` scalar-row: `daylight_zones.illuminance_target_lux` f64
- `change-humidistat-dehumidifying-throttle-range` scalar-row: `humidistats.dehumidifying_throttle_range` f64
- `change-humidistat-humidifying-throttle-range` scalar-row: `humidistats.humidifying_throttle_range` f64
- `change-surface-multiplier` scalar-row: `surfaces.multiplier` u32
- `change-material-conductivity` scalar-row: `materials.conductivity_w_m_k` f64
- `change-material-specific-heat` scalar-row: `materials.specific_heat_j_kg_k` f64
- `change-material-thickness` scalar-row: `materials.thickness_m` f64
- `change-material-density` scalar-row: `materials.density_kg_m3` f64
- `change-thermostat-heating-throttle-range` scalar-row: `thermostats.heating_throttle_range_k` f64
- `change-thermostat-cooling-throttle-range` scalar-row: `thermostats.cooling_throttle_range_k` f64
- `change-pv-system-area` scalar-row: `pv_systems.area_m2` f64
- `change-pv-system-faiman-heat-loss-constant` scalar-row-cross-field: `pv_systems.faiman_heat_loss_constant_w_m2k` f64 constraints ['if existing.faiman_heat_loss_constant_w_m2k==payload.new_faiman_heat_loss_constant_w_m2k{return protocol::MutationOutcome::empty().warning("mutation.no-op","PV system already carries this authored thermal value.");}']
- `change-pv-system-inverter-efficiency` scalar-row: `pv_systems.inverter_efficiency` f64
- `change-pv-system-module-efficiency` scalar-row: `pv_systems.module_efficiency` f64
- `change-pv-system-temperature-coefficient` scalar-row-cross-field: `pv_systems.temperature_coefficient` f64 constraints ['if existing.temperature_coefficient==payload.new_temperature_coefficient{return protocol::MutationOutcome::empty().warning("mutation.no-op","PV system already carries this authored thermal value.");}']
- `change-pv-system-tilt` scalar-row: `pv_systems.tilt_deg` f64
- `change-pv-system-azimuth` scalar-row: `pv_systems.azimuth_deg` f64
- `change-pv-system-faiman-heat-loss-wind` scalar-row-cross-field: `pv_systems.faiman_heat_loss_wind_w_s_m3k` f64 constraints ['if existing.faiman_heat_loss_wind_w_s_m3k==payload.new_faiman_heat_loss_wind_w_s_m3k{return protocol::MutationOutcome::empty().warning("mutation.no-op","PV system already carries this authored thermal value.");}']
- `change-pv-system-dc-capacity` scalar-row: `pv_systems.dc_capacity_w` f64
- `change-material-solar-absorptance` scalar-row: `materials.solar_absorptance` f64
- `change-material-thermal-absorptance` scalar-row: `materials.thermal_absorptance` f64
- `change-material-visible-absorptance` scalar-row: `materials.visible_absorptance` f64
- `change-solar-thermal-system-tilt` scalar-row: `solar_thermal_systems.tilt_deg` f64
- `change-solar-thermal-system-efficiency` scalar-row: `solar_thermal_systems.efficiency` f64
- `change-solar-thermal-system-azimuth` scalar-row: `solar_thermal_systems.azimuth_deg` f64
- `change-solar-thermal-system-collector-area` scalar-row: `solar_thermal_systems.collector_area_m2` f64
- `change-solar-thermal-system-storage-volume` scalar-row: `solar_thermal_systems.storage_volume_m3` f64
- `change-people-gain-sensible-fraction` scalar-row: `people.sensible_fraction` f64
- `change-people-gain-radiant-fraction` scalar-row: `people.radiant_fraction` f64
- `change-people-gain-people-per-area` scalar-row: `people.people_per_area` f64
- `change-people-gain-latent-fraction` scalar-row: `people.latent_fraction` f64
- `change-zone-equipment-cooling-capacity` scalar-row: `zone_equipment.cooling_capacity_w` f64
- `change-zone-equipment-heating-capacity` scalar-row: `zone_equipment.heating_capacity_w` f64
- `change-fenestration-vlt` scalar-row: `fenestrations.vlt` f64
- `change-fenestration-shgc` scalar-row: `fenestrations.shgc` f64
- `change-fenestration-u-value` scalar-row: `fenestrations.u_value_w_m2k` f64
- `change-fenestration-frame-conductance` scalar-row: `fenestrations.frame_conductance_w_k` f64
- `change-fenestration-divider-conductance` scalar-row: `fenestrations.divider_conductance_w_k` f64
- `change-zone-conditioned` scalar-row: `zones.conditioned` bool
- `change-zone-floor-area-participation` scalar-row: `zones.part_of_total_floor_area` bool
- `change-zone-volume` scalar-row: `zones.volume_m3` f64
- `change-zone-multiplier` scalar-row: `zones.multiplier` u32
- `change-space-floor-area` scalar-row: `spaces.floor_area_m2` f64


## Installed Authored Categories

- 103 numeric/boolean row leaves, with actual JSON Schema bounds.
- 11 immutable fixed-enum row leaves, with explicit finite choices and paid sealed storage.
- 31 entity/schedule reference row leaves, with original retained foreign collection searches.
- 2 model identity leaves.

The initial three cross-field flags were false positives caused by whitespace in no-op comparisons; those leaves have scalar-only validation. All 145 row recipe payloads compile through the recipe AST validator. Full production macro/type checking is still pending the shared Window compile frontier. Remaining 159 authored leaves need native producers before the outer root/factory switch.


## Native Text Rows and Shared Publication

The native row publication owner now performs the same actual `PersistentSet` and retained sealed-branch adoption for scalar and string categories. The semantic leaf owner shares diagnostics, inverse factory ownership, cancellation, and physical retirement. String names scan Unicode whitespace and compare original bytes incrementally; duplicate checks traverse actual persistent rows and exclude only the addressed identifier. Original old text is prepared through `RetainedFieldPreparation<String>` for the inverse. Gas and glazing names preserve their authored policy permitting duplicate names; the other thirteen row-name recipes enforce uniqueness.

Fifteen authored row-name schemas and actual payloads now declare native producer recipes. Current declared coverage is 160 row recipes (103 fixed scalars, 11 enums, 31 references, 15 names) plus two model identity leaves: 162 of 306. These declarations are not a claim that the aggregate or active editor has switched to the native root. The remaining 144 operations, aggregate dispatch, factory registration, and end-to-end editor checks remain required.

Validation: actual-source native document harness 23/23 passed, including sparse path/unchanged branch identity, denied item/copy/capacity/release/depth currencies, paid original inverse, and intermediate cancellation. Macro tests 5/5 pass against all 160 authored recipes. An independent Bun SQLite oracle passes the same neutral name fixture (six cases; 14 assertions). The actual generated-payload product check is running against the shared SDK graph; source parsing alone is not treated as product compilation.


## Enclosing Branches, Singleton Fields and Owned Values

Typed branch-output owners now move a prepared child through the actual enclosing branch before producing the sparse native diff. Schedules adopt through `ModelOutputState`; site, ground and run-period children adopt through `ModelIdentityState`. The shared semantic leaf supports the actual zero-target singleton declaration without fabricating a row identifier. Run-period changes preserve range checks, no-op ordering and original calendar interval validation.

Current authored coverage: 178 schema recipes plus two model identity leaves, 180 of 306. The additions comprise fifteen names, four nested schedule scalar/reference/enum leaves, eleven singleton site/ground/run leaves and three owned vector replacements. Remaining: 126 leaves, native aggregate/root switch, installed original factory and actual full-editor verification.

The native row-field machinery for strings and vectors is shared through a typed owned-value path and retained validation contract. A wide time-series fixture contains 1200 values: original copy grants stay at 4096 while the independent capacity grant funds its genuine 9600-byte backing allocation. Denials and cancellations preserve the actual source and close each prepared owner. Missing-reference diagnostics now use the authored original foreign identifier or row identifier, as declared explicitly by each reference recipe.

Validation before canonical payload integration: actual-source harness 28/28 passes, schema compiler 5/5 passes, independent Ajv validates all 178 recipes and bounds (847 assertions). Independent SQLite fixtures pass for names, nested schedule changes, calendar intervals and owned vector changes/inverses. The next harness build references all 178 actual canonical authored payload files and actual OS-command defining traits. Existing whole-value reference impl bodies were separated into neighboring reference modules; production domain roots still include them. No payload source is copied into the ticket, and the harness creates no replacement contract traits. Product compilation remains required.


## Actual Source Authority and Structural Row Progress

The native source harness now includes all 178 canonical payload source files directly, including their genuine DSL MutationLeaf macro and actual native recipe derive. This passes 29 tests, replacing the earlier AST-only coverage claim. The fixed, unit-only ZoneEquipmentType enum now accurately declares Copy.

The authored repository taxonomy omitted nine physical leaves (four photovoltaic fields and five glazing optical fields), and their descriptors retained old flat owner addresses. All nine physical owners and descriptors were corrected; the real mutationAuthorityProjection generator rebuilt the source authority table from the actual repository taxonomy. All 306 physical mutation descriptors now match registered source owners. The nested ✏s workspace declares explicit ancestor source authority in package metadata. The defining authority test preserves the existing unpaired nested workspace refusal and separately checks successful, invalid-path and missing-target delegation. Its independent Ajv/picomatch oracle passes.

Five glazing optical scalar recipes bring declared native coverage to 185/306 (183 schema recipes plus two identity leaves). These five canonical payloads are included in the next actual source harness run. The existing independent schema oracle passes 3 tests and 877 assertions across the recipe and structural fixtures.

Ranked row publication now uses the defining PersistentSplice cursor for insertions/removals, retaining the actual original authored row, page source and displaced branch roots. Its focused source test passes five insertion/removal cases, 13 cancellation cutoffs and every denied funding axis with exact physical heap/receipt agreement. This establishes a structural foundation; it does not claim create/delete semantic leaves are installed. A bounded actual-original identity/rank scan is under test for creation duplicate and authored index rules.


## Current Native Coverage and Actual Receiving Proof

Native semantic coverage is 189 of 306: 110 scalar row leaves, 12 fixed enum row leaves, 32 foreign-reference row leaves, 15 name row leaves, 11 singleton scalar fields, 3 owned vector row leaves, 4 create row leaves, and the 2 model identity leaves. The earlier numeric subtotal in the progress notes was stale; this count is read from all actual authored payload schemas. The remaining 117 semantics, the aggregate retained dispatch, final native root switch and installed Energy preparation factory remain required work.

The four full create leaves are CreateZone, CreatePvSystem, CreateBattery and CreateSolarThermalSystem. Each actual canonical source is exercised through its retained semantic leaf, native typed payload receiving, sparse central application, original Delete inverse construction, original cancellation, all denied funding axes and exact per-turn physical birth/release receipts. Combined invalid name and invalid rank preserve the authored validation ordering. The independent JSON Schema and SQL oracle checks all four actual creation fixtures. It found missing creation bounds for battery and solar thermal; these now match their actual authored change-field bounds. Source suite: 36 tests passed before receiving coverage extension; all independent creation/recipe tests passed (5 tests, 1291 assertions).

All 306 canonical payload source files now declare the genuine NativeReceive implementation. Every canonical source is directly included by the source harness, and an explicit NativeReceive<DslValue> contract assertion checks all 306 actual types. The complete source suite passed 37 tests. This establishes original receiving and real source authority for the full payload vocabulary; it does not claim all 306 semantic leaves are implemented. The actual mutation aggregate also now declares NativeReceive for its true tagged vocabulary, awaiting product graph compilation.

The all-payload check exposed a defining NativeReceive macro bug for zero-field records (DisconnectReferencedModel and UnbindWeatherFile). Its field ordinal and original child reference now have explicit correct types. No invented field or alternate receiver was added. The defining original empty-record test uses neutral JSON vectors, a serde map/object oracle, twelve cancellation cutoffs and physical receipt checks. The entire defining native-owned receiving library passes all 20 tests. The native Energy schema compiler passes all 7 tests.

Every cold MutationKind/ColdMutationKind body now resides in an explicit reference sibling beside its actual payload, included by the actual owning domain barrel. This permits the full real payload receiving vocabulary to compile independently of the numerical reference snapshot. The existing classic product behavior remains active while the complete native semantic root is finished; the reference source is not a native fallback.

Typed original construction has authored field recipes for 31 of 38 creation payloads. The structural row source preserves independently funded scalar/sealed fields and genuine PersistentSplice page paths. A separate owned output form and admitted rank context are being tested for actual typed create-payload reconstruction from original row fields, to support delete inverses without any sealed temporary or full model copy.

### Native Unrestricted Deletion

The actual native delete category now preserves the selected original row projection and its original list rank through sparse `PersistentSplice` removal, central branch application, typed Create reconstruction, and cancellation. Twenty-two authored unrestricted Delete recipes install this category; thirteen referenced Delete leaves and three special inverse constructors remain outside it. No restriction is inferred or bypassed: the recipe explicitly declares `none`, and other guard policies fail macro compilation until implemented.

The real Solar deletion leaf passes first/last/missing cases, exact typed inverse fields and rank, fifteen cancellation cutoffs per case, all denied grant currencies, 4096-byte copy limits, and precise physical heap receipts. All 306 actual canonical payload sources, including the 22 deletion derives and their real inverse producers, compile in the source-referenced harness. The independent SQLite event oracle validates removal, original inverse rank, and restoration; Ajv strictly validates all 22 recipes against their actual Create field recipe and key declaration. The macro deletion test was first red for an unsupported category and then passed after the real implementation.

Installed native semantic category coverage is 211 of 306; 95 leaves remain. This is domain-source validation, not a completed product/editor compile or active root switch. Generated evidence: `energy-native-deletion-test.log`, `energy-native-delete-oracle.log`, and `energy-native-delete-recipes.log`.

### Native Reference-Protected Deletion

Six additional original Delete leaves now install actual typed referencing-row policies: PV, Battery, Space, Material, Construction, and AirLoop. The shared guard uses the actual `PersistentListRead` and retains each original row while inspecting wide sealed reference arrays one key per admitted turn. PV's actual 1200-key physical regression passes: a matching final key refuses removal, unmatched first/last deletes preserve actual Create inverse rank, and every cancellation prefix and denied grant currency preserves original ownership and exact physical receipts. Native installed semantics are 217/306; the new Zone restriction is pending its own runtime proof.

Construction deletion had omitted fenestration glazing references. Both the authored reference diff and the native policy now protect that real relationship. The native policy composes the actual Surface and Fenestration scans, closing each genuine child before starting the next. Its actual canonical leaf passes empty/first/last/missing, glazing-reference refusal, typed inverse, denied grants, and every cancellation prefix. The independent SQLite oracle covers the same referenced and unreferenced events. Defining macro tests passed 9/9 before the new Zone compilation test was added.

Evidence: `energy-native-deletion-reference-test.log`, `energy-native-deletion-combined-test.log`, `energy-native-delete-recipes.log`, `energy-native-derive-test.log`. Full source suite previously passed 43/43 before these two added runtime proofs. The most recent complete editor check still stops at `time-travel/store-state:316` because the real `commit_finished_replay` boundary retains cold Clone/FromValue bounds; lifecycle owns that shared repair. All mounted PagedBytes compile errors are resolved in this check.

### Zone Deletion and Original Root References

Zone deletion now installs all seventeen declared relationships, including fourteen direct row keys, thermal-enclosure and air-loop reference arrays, and the optional airflow-network tuple source. Each row scan uses the actual native page reader; the optional root source keeps genuine document/deleted-row bindings and reads one original tuple key per admitted turn. Separately funded `AllRowReferences` children close before the next source begins. The actual canonical Zone leaf passes positive first/last/missing, refusal on the last of 1200 network tuples, all fifteen cancellation cutoffs, all denied currencies, 4096-byte copy bounds, and exact physical receipts. Installed semantics now cover 218/306, including 29/38 Delete leaves.

The first wide receiving fixture exhausted its explicitly admitted 16 MiB lifetime quota before deletion. Diagnosis showed exact equality: 16,777,209 bytes on the original native decoder ownership ledger and 16,777,209 independently observed physical births. There was no duplicated or fabricated charge. The one wide fixture now declares 64 MiB as its original receiving ownership maximum before creating its decoder; production policies and per-turn copy grants are unchanged. Every receiving turn now additionally asserts ledger equality to cumulative physical births. No caller/control is renewed after admission.

Evidence: `energy-native-deletion-zone-test.log` passes; independent recipe/SQL/Ajv oracle passes six tests and 1469 assertions. Latest source-referenced full suite will include 46 tests; the previous full suite passed 43 before these three new deletion proofs.

### Next Creation Boundary

The next bulk boundary is a retained creation validator composing the existing real typed constructors with declared scalar/vector foreign keys and bounded vector rules. It must retain original payload/document bindings, drive each actual foreign collection lookup under the original grant, close each original lookup before continuing, and preserve the exact failing reference source for diagnostics. Creation failures currently support only stateless target callbacks, so stateful failing-reference metadata must be exposed explicitly from the retained core rather than replacing the authored foreign-key target with the new row id. Twenty-seven additional straightforward constructor semantics can share that pipeline; seven special constructors still need actual presence/enum/schedule field transforms. This paragraph records planned work, not verified coverage.

## Native Creation Coverage Update

Verified native semantic coverage is 243 of 306 actions: 183 scalar, enum, reference, name and singleton actions; two identity actions; 29 creation actions; and 29 deletion actions. The next nine creation actions are SpaceList, Space, Construction, ThermalEnclosure, ShadingSurface, Material, AirLoop, ElectricalLoadCenter and PlantLoop. All actual typed sources compile. The ShadingSurface proof covers optional schedule references, authored row diagnostic targets, Unicode blank names, rank ordering, a 1200-point polygon, denied grant axes, sparse central apply, actual inverse ownership and cancellation prefixes. Independent Ajv and SQLite tests validate all 29 recipes and the seven creation law fixtures. No product build or aggregate native dispatch claim follows from this coverage count.

The next two schedule creation actions require actual cross-kind identity scans across the five retained schedule collections before insertion rank or scalar validation. Their metadata and retained precheck are authored; focused tests are running. The active numerical snapshot export has not yet switched to the native root. IDF import must publish genuine bounded semantic row operations under the protocol's 256 KiB message limit; private sparse branch headers are internal apply artifacts and must never become an opaque whole-model transport mutation.

### Schedule Creation Validation

The full actual-source native harness passes 56 tests. Native create coverage is now 31 actions, bringing verified installed semantics to 245/306. Constant and time-series creators perform funded typed searches across all five original schedule collections before rank/value validation, retain each search through paid close, and publish through the actual nested schedule branch. Both law fixtures exercise cross-kind collisions in all five collections, invalid ranks, actual inverse handoff, denied currencies and 15 cancellation cutoffs. The time-series law also carries 1200 original values. Macro tests pass 11/11; independent Ajv/SQLite tests pass 7/7 with 1758 assertions across 31 recipes and nine law fixtures.

### Optional Facet Validation

Verified installed semantics reached 250/306: the actual-source harness passes 61 tests, including heating/cooling capacity limits and three optional schedule references. The scalar owner retains original transformed fixed input custody, validates after genuine target lookup, and preserves exact original optional inverses. The wide inline scalar frame exceeded the 4096-byte copy bound; its actual row projection now receives a separate funded box and exact paid release. All prior scalar, creation, deletion and source lifecycle laws still pass. Optional references preserve the authored collection roster: annual holiday profiles require daily schedules, while shading and setpoint managers allow the five schedule types. `None` does not trigger a foreign-key search. Native derive tests pass 12/12 and independent Ajv/SQLite optional event tests pass 2/2.

CreateIdealLoadsSystem and DeleteIdealLoadsSystem are the next pending validation pair. The defining row-birth schema now declares each optional numeric field's actual value/presence source; its immutable typed field is born through the existing paid RowBirthPreparation. Deletion additionally refuses actual retained Fault references, and its inverse restores exact optional capacity values and the original rank. Pending compilation/testing is not counted above.

### Ideal Loads Pair and Product Compile Frontier

Verified installed semantics reached 252/306. The full actual-source native suite passes 64/64; derive tests pass 12/12 and combined schema/SQLite category oracles pass 11/11 with 1930 assertions. Both optional capacities are born and reconstructed from exact original typed value/presence fields. Ideal-loads deletion now refuses a live Fault target, consistently in the reference semantic implementation and native path; refused reference inverses are empty. The latest real editor cargo check reaches four shared plugin-instance-stores errors (two paged payload signatures and two incidental cold capability bounds); the parent/results agents own that frontier. The active Energy root/aggregate/native factory integration remains pending.

The next weekly creation proof uses an authored seven-element fixed array and a separately funded original array-to-vector inverse backing. This adds no whole-document copy and does not change receiving policy or the 4096-byte copy frontier. It is not included in the 252 count until its typed runtime and independent oracles pass.

### Fixed Schedules and Empty Creation Sources

The actual-source suite passed 68/68 after weekly and daily creation, including the separately funded original fixed-array-to-vector inverse backing and paired daily bounds. Verified coverage at that point was 254/306. The independent schema/SQLite fixed-schedule oracle passed 8/8 with 1875 assertions.

Annual and fenestration creation now declare explicit empty-vector sources and `separate-intents` recreation. All existing exact recreation recipes declare `exact` explicitly. The derive regression rejects declaring an empty creation source as an exact recreation; no lossy `RowRecreation` implementation is emitted. Annual's focused runtime proof passes and the independent creation/schema oracles pass 8/8 with 1974 assertions. The full 70-test rerun currently has 12 close-denial regressions after shared bulk Vec retirement changes: a grant one copy byte below a prior quoted frontier is being treated as a smaller adaptive body and makes positive smaller-fragment progress. Genesis, owned vectors and previous row-birth laws expose the same issue. This is reported to the parent; tests have not been relaxed. New glazing bind/clear native facet tests are pending, with explicit missing-on-empty clear semantics and exact cross-intent inverses.

### Exact Original Close Policy and Glazing Inverses

Verified native semantic coverage is 258/306: 183 core actions, two identity actions, 36 constructors, 30 deletions and seven optional facets. The actual-source suite passes 75/75. Annual and fenestration constructors preserve existing nonempty rules, holidays and polygons; their authored empty creation sources do not claim exact row recreation. Glazing bind and clear preserve the original optional construction identity in their cross-intent inverse, including clearing an originally absent link and refusing clear on an empty slot. Four original glazing runtime tests pass; independent Ajv/SQLite category and optional-facet tests passed 10/10 with 1895 assertions.

The twelve prior retirement regressions are repaired without changing the denial assertions. Native Genesis, RowBirth, OwnedRow, CreateRow and shared RowLeaf owners capture the actual body only on their first fully funded source advance. That usize write is included in both the copy quote and receipt. Later source advances refuse a foreign body; close queries derive from the original admitted policy. Every close preflights the same complete quote before effects and forwards the selected copy cap. Active generic retirement owners additionally use the defining exclusive `RetainedCloneClose::select(body)` loan, preserving the exact real owner and preventing a denied copy budget from silently choosing smaller work. The new language-neutral body fixture, native physical-allocation law and independent SQLite policy oracle pass.

Daily schedule paired-limits native migration is verified as action 259. Its schema explicitly declares the lower and upper original nullable fields; the typed transform preserves complete absence and ordered finite intervals, keeps target lookup before invalid-input refusal, and reconstructs both inverse bounds. Independent schema/SQLite validation passes 11/11 with 1918 assertions across categories and facets. Its focused native runtime proof passes with 75 other actual-source tests compiled; installed coverage is now 259/306.

### Native Membership Category

The full actual-source suite passes 90/90 and native installed semantics reach 273/306. The fourteen new membership actions share one original retained producer: SpaceList member, Construction layer, ThermalEnclosure zone, AirLoop terminal, ElectricalLoadCenter PV/battery and PlantLoop equipment insertion/removal. It retains the genuine authored payload and selected row, searches only the declared real foreign collection, scans one original vector item per funded turn, reserves the exact replacement backing, and copies one item per bounded turn. The actual sealed field, row, persistent page path and owning branch publish through the existing native stages. The selected original item and ordinal survive for the opposite-intent inverse. No temporary numerical Model or whole-model clone is created.

Recipes preserve each authored diagnostic priority: SpaceList/Enclosure/LoadCenter check insertion rank before no-op duplication; AirLoop and PlantLoop refuse duplicates before rank checks; Construction permits repeated layer materials and removes by ordinal. PlantLoop requires a positive equipment id without inventing an unavailable foreign equipment registry. All fourteen original native laws pass, including independent per-currency denial, exact physical allocation/release receipts and cancellation prefixes. Independent Ajv and SQLite tests pass with 186 assertions across all fourteen recipes and their event/inverse fixtures. Two additional 1200-layer original insertion/removal laws are authored; the fresh run is temporarily blocked by the parent-owned VCS checkpoint module declaring `repin_facts` before its Rust source exists. They are not included in the 90 result yet.


### Construction Permutation, Joint Optics and Surface Boundary

The full actual-source suite reached 99 passing tests; its sole failure was the independently owned wide root-link receiving frontier requesting 4097 copy bytes. The defining receiver now reserves its actual wrapper overhead and is being rerun. This run proves all earlier categories plus the duplicate-preserving 1200-layer construction permutation and paired infrared-emissivity facet. Verified action coverage is 278/306 before the four root-link leaves and two surface additions are counted.

Construction permutation owns two bounded key vectors and drives the first-party heap-sort cursor one compare/swap/advance at a time. Comparing the sorted original keys retains duplicate multiplicities; the unchanged original vector remains the inverse source. Its two native lifecycle tests pass, including every positive denied axis and physical cancellation receipts. The independent SQLite multiset oracle passes.

The optical pair reads and replaces both real scalar columns through one sparse original row path. Its inverse retains both distinct original values. No tuple reference or replacement state field was invented. Its native lifecycle test and Ajv/SQLite two-column oracle pass.

Surface-boundary schema now names the actual discriminator/partner transformation. The native scalar path validates their union, forbids a self-partner, and searches the original surface roster before producing the sparse sealed boundary replacement. Its independent schema/SQL foreign-key oracle passes (13 assertions); native lifecycle validation is running. Surface creation now authors the same union birth and exact recreation, original zone/construction/partner rules, polygon fragments, multiplicity and rank. The combined independent recipe/create oracle passes 8 tests with 2056 assertions over 37 creation recipes and 10 facets. These pending native surface tests are not yet counted as verified action coverage.


### Surface, Setpoint and Composite Pair Follow-up

Surface boundary and creation native lifecycle tests pass (2/2). Complete root-link native core and four actual generated leaves pass after the defining optional receiver reserved its genuine phase overhead. The combined native suite then reached 110 passing tests and two setpoint failures: one expected fixture encoded integer JSON literals for typed floating fields, and the fixed-field transform wrapper recomputed an adaptive close from the denied grant. The fixture now records the authored floating representation; the wrapper now captures the actual first-funded body, refuses changes, preflights that same selected quote and caps delegated copy. Verification is queued. Verified action coverage is 285/306, including the passing setpoint creation but excluding the two repaired setpoint tests until they pass.

The setpoint payload discriminator is now the actual four-arm unit enum from its existing JSON Schema. The same four wire strings remain authoritative. The numerical control-law union consists entirely of fixed copied fields and now exposes exact typed decomposition/reconstruction. Native row birth, sparse replacement and deletion recreation retain the actual tag, all four limits, optional schedule presence and original insertion rank. The independent creation/recipe oracle passes 8 tests with 2104 assertions over 38 creation recipes, 23 unrestricted deletes and 11 facets; the dedicated SQL control-law oracle passes 19 assertions.

Adjacency connect/disconnect now author a distinct two-reference collection path. It uses actual undirected duplicate identity, retained original surface searches, the actual ordered pair orientation, explicit/default insertion rank, ranked persistent splice and exact inverse orientation/rank. No entity identifier was fabricated for a pair. Its independent schema/SQL composite-key oracle passes 29 assertions. Native lifecycle tests are authored and queued.

Fenestration polygon replacement now authors a bounded native validation owner. It reads one original finite point per turn, retains the actual host surface projection, accumulates one original host edge per turn with the defining numerical engine's Newell helper, normalizes using the existing engine implementation, then checks one original aperture point per turn against the authored tolerance. The native owned-field path retains the complete original vector inverse and immutable sibling fields. Fixtures cover an empty rectangle-derived polygon, unchanged/changed polygons, invalid cardinality, typed nonfinite input, missing target precedence, off-plane coordinates, exact tolerance edges, 1200 original host edges and a 1200-point input. The independent SQL cross-product oracle passes; the actual native run is queued and is not counted yet.

Six annual/weekly schedule edits and five schedule deletions belong to the results agent. Central schedule recipe registration and the actual source-homed test includes are installed. The public root type switch, aggregate preparation registration and actual editor product compilation remain required after all native action paths pass. Remaining unregistered non-schedule actions are surface/fenestration deletion, optional airflow network replacement and output variable add/remove.

### Native Geometry And Public Model Integration

The source-referenced native document suite now passes 119 tests. Complete aperture deletion emits genuine polygon/creation inverse intents, and the 1,200-point aperture plane test passes under the original retained decoder capacity policy. Together with the six schedule and two output-variable actions this establishes 299 of 306 authored native leaves. Seven remain under final verification: five schedule deletions, surface cascade and the network singleton. The schedule reference matrix and wide annual inverse already pass; a small expected fixture payload was corrected to its actual schema.

The surface cascade uses original immutable splice roots for all three geometry collections, retains its original geometry authority for inverse production, and emits complete typed inverses one at a time. Its independent SQLite/Ajv oracle passes 20 assertions, including full original rank/field restoration and interzone restriction. The first native runtime check found a genuine missing ancestor depth in two source-admission quotes; those exact quotes are repaired and the focused physical test is running.

The public Snapshot now exports the native persistent document. The numerical reference Snapshot is explicitly scoped under reference; native editing has no whole-model clone facade. The public diff and aggregate point to the native central retained applier, and the editor factory uses PagedOneItemPreparationFactory with MutationApplyEdit. Product compilation remains in progress; this source switch does not establish a working mounted editor by itself.

### All 306 Native Actions Verified

The authoritative fresh original-source suite passes 123 of 123 selected tests (four unrelated tests filtered by its original_ name selector). This includes all 306 canonical action sources, the surface three-collection cascade, aperture polygons and multi-inverse reconstruction, the five schedule deletion reference matrices and 1,200-rule annual inverse, both output-variable actions and the 1,200-pair network singleton. The source suite checks actual original receiving, sparse central adoption, inverse ownership, all positive currency denials and paid cancelled prefixes.

The public native aggregate additionally delegates original borrowed target fragments to each authored leaf. Identity operations explicitly declare document scope; entity and composite targets preserve their leaf-specific IDs. Product compilation and mounted verification remain required and are in progress.
