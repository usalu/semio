const num = { type: "number" };
const int = { type: "integer", minimum: 0 };
const bool = { type: "boolean" };
const str = { type: "string" };
const nullable = (schema: object) => ({ oneOf: [{ type: "null" }, schema] });
const vertex = { type: "array", items: num, minItems: 3, maxItems: 3 };
const ring = { type: "array", items: vertex, minItems: 3 };
const empty = { type: "array", maxItems: 0 };
const record = (properties: Record<string, object>) => ({ type: "object", additionalProperties: false, required: Object.keys(properties), properties });
const list = (item: object) => ({ type: "array", items: item });
const classes = ["ExteriorWall", "InteriorWall", "Roof", "Ceiling", "Floor", "Interzone", "Adiabatic", "Ground"];
const boundary = { oneOf: [{ type: "string", enum: ["OutdoorAir", "Ground", "OtherSideTemperature", "Adiabatic"] }, record({ Interzone: int })] };
const month12 = { type: "array", items: num, minItems: 12, maxItems: 12 };
const model = record({
  name: str, version: str,
  site: record({ latitude_deg: num, longitude_deg: num, elevation_m: num, time_zone_hours: num, north_axis_deg: num }),
  zones: list(record({ id: int, name: str, volume_m3: { type: "number", exclusiveMinimum: 0 }, multiplier: { type: "integer", minimum: 1 }, conditioned: bool, part_of_total_floor_area: bool })),
  spaces: list(record({ id: int, name: str, zone_id: int, floor_area_m2: { type: "number", minimum: 0 } })),
  surfaces: list(record({ id: int, name: str, zone_id: int, class: { type: "string", enum: classes }, vertices_m: ring, construction_id: int, outside_boundary_condition: boundary, sun_exposed: bool, wind_exposed: bool, multiplier: { type: "integer", minimum: 1 } })),
  fenestrations: list(record({ id: int, name: str, surface_id: int, u_value_w_m2k: { type: "number", exclusiveMinimum: 0 }, shgc: { type: "number", minimum: 0, maximum: 1 }, vlt: { type: "number", minimum: 0, maximum: 1 }, area_m2: { type: "number", exclusiveMinimum: 0 }, height_m: { type: "number", exclusiveMinimum: 0 }, sill_height_m: num, frame_conductance_w_k: num, divider_conductance_w_k: num, overhang_depth_m: num, overhang_offset_m: num, fin_depth_m: num, fin_offset_m: num, glazing_construction_id: nullable(int), vertices_m: ring })),
  materials: list(record({ id: int, name: str, roughness: { type: "string", enum: ["VeryRough", "Rough", "MediumRough", "MediumSmooth", "Smooth", "VerySmooth"] }, thickness_m: { type: "number", exclusiveMinimum: 0 }, conductivity_w_m_k: { type: "number", exclusiveMinimum: 0 }, density_kg_m3: { type: "number", minimum: 0 }, specific_heat_j_kg_k: { type: "number", minimum: 0 }, thermal_absorptance: { type: "number", minimum: 0, maximum: 1 }, solar_absorptance: { type: "number", minimum: 0, maximum: 1 }, visible_absorptance: { type: "number", minimum: 0, maximum: 1 } })),
  glazing_materials: empty, gas_materials: empty,
  constructions: list({ ...record({ id: int, name: str, layer_material_ids: { type: "array", items: int, minItems: 1 } }) }),
  people: list(record({ id: int, zone_id: int, schedule_id: int, activity_schedule_id: int, people_per_area: { type: "number", minimum: 0 }, sensible_fraction: num, latent_fraction: num, radiant_fraction: num })),
  lighting: list(record({ id: int, zone_id: int, schedule_id: int, watts_per_area: { type: "number", minimum: 0 }, radiant_fraction: num, visible_fraction: num, return_air_fraction: num })),
  equipment: list(record({ id: int, zone_id: int, schedule_id: int, watts_per_area: { type: "number", minimum: 0 }, radiant_fraction: num, latent_fraction: num })),
  thermostats: list(record({ id: int, zone_id: int, heating_setpoint_schedule_id: int, cooling_setpoint_schedule_id: int, heating_throttle_range_k: num, cooling_throttle_range_k: num })),
  humidistats: empty, setpoint_managers: empty,
  ideal_loads: list(record({ id: int, zone_id: int, max_heating_supply_air_temp_c: num, min_cooling_supply_air_temp_c: num, max_heating_capacity_w: nullable(num), max_cooling_capacity_w: nullable(num), outdoor_air_per_person_m3_s: num, outdoor_air_per_area_m3_s_m2: { type: "number", minimum: 0 } })),
  zone_equipment: empty, air_loops: empty, plant_loops: empty, outdoor_air_systems: empty, infiltrations: empty, mechanical_ventilations: empty, shading_surfaces: empty, space_lists: empty,
  thermal_enclosures: list(record({ id: int, name: str, zone_ids: list(int) })),
  adjacency_pairs: list(record({ surface_a_id: int, surface_b_id: int })),
  airflow_network: { type: "null" },
  electrical_load_centers: empty, pv_systems: empty, battery_storage: empty, shw_systems: empty, solar_thermal_systems: empty, refrigeration_systems: empty, water_systems: empty, faults: empty, output_variables: empty, sizing_objects: empty, daylight_zones: empty, room_air_models: empty,
  ground_temperature: record({ building_surface_c: month12, shallow_c: month12, deep_c: num }),
  run_period: record({ start_month: int, start_day: int, end_month: int, end_day: int, year: int }),
  schedules: record({
    constants: list(record({ id: int, value: num })),
    daily: list(record({ id: int, hourly_values: { type: "array", items: num, minItems: 24, maxItems: 24 }, interpolation: { type: "string", enum: ["Continuous", "Discrete"] }, limits: nullable(record({ min: num, max: num })) })),
    weekly: list(record({ id: int, daily_schedule_ids: { type: "array", items: int, minItems: 7, maxItems: 7 } })),
    annual: empty, time_series: empty,
  }),
});
const child = (artifactId: string, subset: string) => record({ childId: { const: artifactId }, target: record({ artifactId: { const: artifactId }, dialect: record({ artifactKind: { const: "s.stdio.semio" }, standard: { const: "v1" }, subset: { const: subset } }) }) });
const schema = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: "https://json.schemas.assets.semio-tech.com/s/bim/model/export/energy.json",
  title: "BimEnergyExport",
  description: "The snapshot of s.energy.model@1 as the BIM export writes it: the typed `model` member field for field as the energy engine declares it, and the two derived child handles.",
  ...record({ schema: { const: "energy.model" }, model, structure: child("energy-value", "value"), zones: child("energy-table", "table"), referencedModel: { type: "null" }, weatherLink: { type: "null" } }),
};
await Bun.write(process.argv[2], JSON.stringify(schema, null, 2) + "\n");
