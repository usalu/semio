# Energy Complete Native Role Blueprint

The repair boundary is the existing `construct(record, native, maximum)` in the Energy native module. Pass full copied `SqliteDatabaseLimits` instead of only `maximum`, then finish borrowed semantic admission before either typed pack construction or Model construction. Outer optional fields must distinguish required `FieldValue::Absent` from a missing field. Existing shape/parser/backing controls remain authoritative. Do not obtain a typed Model merely to count its SQL cells.

For typed projection, convert `Projection` emitter/helper parameters and all three adjacent projection modules to the existing `RowWriter` interface and expose one authored `visit_rows`. Both SQL creation and native encode preflight call it. Helpers `emit`/`edge` can use inline bounded cell storage or paid frontier allocation; their existing uncharged `Vec::with_capacity` must not become a new admission shadow. No tree frontier is needed for this fixed domain: iterate actual collection slices and nested arrays/lists directly with paid row checkpoints. Complete schema table/column/UTF-8 admission must precede typed/native work.

Borrowed model navigation uses `DslValue::Object` named fields and `Array` lists. Require the actual declared type and explicit optional Null semantics. Primitive UInt widths must match u8/u16/u32 declarations; Bool and Int/Float categories follow the existing derive's controlled conversion, without copying string/value owners. Borrowed string byte cost is UTF-8 length, not character count. Unit enums project their authored scalar strings; externally tagged enums require exact selected payload: OutsideBoundary Interzone has one EntityId; OutdoorAirReset has four named float fields and adds its separate row. Do not charge JSON syntax or tag-object backing as SQL values. Ground arrays each have exactly twelve floats; daily hourly_values exactly24, weekly daily_schedule_ids exactly7, vertex arrays exactly3, annual holiday tuples exactly3 with widths u16/u8/u8. Missing fields/list/optional wrappers must refuse before typed construction, not silently act as empty.

Every nullable optional ID pays zero when Null and eight when present. Every entity/relationship row pays identity+parent+ordinal (24), every key singleton pays identity (8). Binary64 cells pay bits8 plus exact class length and query8 except NaN query Null0. Thus per projected float is22 finite,32 either Infinity,11 NaN. Preserve duplicate semantic domain IDs, literal children/links, unresolved domain references and existing SQL ownership rules; a domain simulation-validation pass would wrongly reject existing accepted fixture semantics.

The complete original fixture can be reused: `completeModel` covers all37 model collections and five schedule collections; its binary64 declaration pins173 fields. The original law independently constructs Model through serde, and independent Bun SQLite edits all173 actual REAL companion fields across every literal IEEE word. Add a closed extent companion by exporting that same actual complete owner through the existing Source provider, querying all69 physical tables in Bun SQLite, and summing each actual cell via `CASE typeof`: integer/real8, text byte length (`length(CAST(cell AS BLOB))`), blob length, null0. Count all rows. Validate integrity/FKs and copy exact per-table tuples into a constant neutral schema. Repeat each word by the existing physical IEEE edit path; keep NaN query NULL and actual class strings. This is independent of the native visitor. No populated costs have been measured in this audit and no guessed totals are supplied. Preserve the original large-name/four-phase cancellation and one-short file/row obligations.

Independent Bun SQLite loaded the actual authored DDL and read all 69 tables through PRAGMA table_info. This is a schema-role measurement, not a populated specimen or Native qualification. Full closed table/column details are retained in [the role input](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/energy-native-complete-authored-cell-role-map.json).

| Table | Width | Ordered column roles |
|---|---:|---|
| energy_document | 27 | id:INTEGER?, schema:TEXT!, name:TEXT!, version:TEXT!, latitude_deg:REAL?, longitude_deg:REAL?, elevation_m:REAL?, time_zone_hours:REAL?, north_axis_deg:REAL?, deep_ground_c:REAL?, start_month:INTEGER!, start_day:INTEGER!, end_month:INTEGER!, end_day:INTEGER!, year:INTEGER!, latitude_deg_ieee754_bits:INTEGER!, latitude_deg_ieee754_class:TEXT!, longitude_deg_ieee754_bits:INTEGER!, longitude_deg_ieee754_class:TEXT!, elevation_m_ieee754_bits:INTEGER!, elevation_m_ieee754_class:TEXT!, time_zone_hours_ieee754_bits:INTEGER!, time_zone_hours_ieee754_class:TEXT!, north_axis_deg_ieee754_bits:INTEGER!, north_axis_deg_ieee754_class:TEXT!, deep_ground_c_ieee754_bits:INTEGER!, deep_ground_c_ieee754_class:TEXT! |
| energy_structure_child | 6 | id:INTEGER?, child_id:TEXT!, artifact_id:TEXT!, artifact_kind:TEXT!, standard:TEXT!, subset:TEXT! |
| energy_zones_child | 6 | id:INTEGER?, child_id:TEXT!, artifact_id:TEXT!, artifact_kind:TEXT!, standard:TEXT!, subset:TEXT! |
| energy_referenced_model_link | 12 | id:INTEGER?, artifact_id:TEXT!, artifact_kind:TEXT!, standard:TEXT!, subset:TEXT!, role:TEXT!, pin_kind:TEXT!, checkpoint_id:TEXT?, blob_hash:TEXT?, blob_size_high:INTEGER?, blob_size_low:INTEGER?, blob_media_type:TEXT? |
| energy_weather_link | 12 | id:INTEGER?, artifact_id:TEXT!, artifact_kind:TEXT!, standard:TEXT!, subset:TEXT!, role:TEXT!, pin_kind:TEXT!, checkpoint_id:TEXT?, blob_hash:TEXT?, blob_size_high:INTEGER?, blob_size_low:INTEGER?, blob_media_type:TEXT? |
| energy_ground_month | 9 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, building_surface_c:REAL?, shallow_c:REAL?, building_surface_c_ieee754_bits:INTEGER!, building_surface_c_ieee754_class:TEXT!, shallow_c_ieee754_bits:INTEGER!, shallow_c_ieee754_class:TEXT! |
| energy_zone | 11 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, volume_m3:REAL?, multiplier:INTEGER!, conditioned:INTEGER!, part_of_total_floor_area:INTEGER!, volume_m3_ieee754_bits:INTEGER!, volume_m3_ieee754_class:TEXT! |
| energy_space | 9 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, zone_id:INTEGER!, floor_area_m2:REAL?, floor_area_m2_ieee754_bits:INTEGER!, floor_area_m2_ieee754_class:TEXT! |
| energy_surface | 13 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, zone_id:INTEGER!, class:TEXT!, construction_id:INTEGER!, outside_boundary_kind:TEXT!, interzone_partner:INTEGER?, sun_exposed:INTEGER!, wind_exposed:INTEGER!, multiplier:INTEGER! |
| energy_surface_vertex | 12 | id:INTEGER?, surface_id:INTEGER!, ordinal:INTEGER!, x_m:REAL?, y_m:REAL?, z_m:REAL?, x_m_ieee754_bits:INTEGER!, x_m_ieee754_class:TEXT!, y_m_ieee754_bits:INTEGER!, y_m_ieee754_class:TEXT!, z_m_ieee754_bits:INTEGER!, z_m_ieee754_class:TEXT! |
| energy_fenestration | 43 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, surface_id:INTEGER!, u_value_w_m2k:REAL?, shgc:REAL?, vlt:REAL?, area_m2:REAL?, height_m:REAL?, sill_height_m:REAL?, frame_conductance_w_k:REAL?, divider_conductance_w_k:REAL?, overhang_depth_m:REAL?, overhang_offset_m:REAL?, fin_depth_m:REAL?, fin_offset_m:REAL?, glazing_construction_id:INTEGER?, u_value_w_m2k_ieee754_bits:INTEGER!, u_value_w_m2k_ieee754_class:TEXT!, shgc_ieee754_bits:INTEGER!, shgc_ieee754_class:TEXT!, vlt_ieee754_bits:INTEGER!, vlt_ieee754_class:TEXT!, area_m2_ieee754_bits:INTEGER!, area_m2_ieee754_class:TEXT!, height_m_ieee754_bits:INTEGER!, height_m_ieee754_class:TEXT!, sill_height_m_ieee754_bits:INTEGER!, sill_height_m_ieee754_class:TEXT!, frame_conductance_w_k_ieee754_bits:INTEGER!, frame_conductance_w_k_ieee754_class:TEXT!, divider_conductance_w_k_ieee754_bits:INTEGER!, divider_conductance_w_k_ieee754_class:TEXT!, overhang_depth_m_ieee754_bits:INTEGER!, overhang_depth_m_ieee754_class:TEXT!, overhang_offset_m_ieee754_bits:INTEGER!, overhang_offset_m_ieee754_class:TEXT!, fin_depth_m_ieee754_bits:INTEGER!, fin_depth_m_ieee754_class:TEXT!, fin_offset_m_ieee754_bits:INTEGER!, fin_offset_m_ieee754_class:TEXT! |
| energy_fenestration_vertex | 12 | id:INTEGER?, fenestration_id:INTEGER!, ordinal:INTEGER!, x_m:REAL?, y_m:REAL?, z_m:REAL?, x_m_ieee754_bits:INTEGER!, x_m_ieee754_class:TEXT!, y_m_ieee754_bits:INTEGER!, y_m_ieee754_class:TEXT!, z_m_ieee754_bits:INTEGER!, z_m_ieee754_class:TEXT! |
| energy_material | 27 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, roughness:TEXT!, thickness_m:REAL?, conductivity_w_m_k:REAL?, density_kg_m3:REAL?, specific_heat_j_kg_k:REAL?, thermal_absorptance:REAL?, solar_absorptance:REAL?, visible_absorptance:REAL?, thickness_m_ieee754_bits:INTEGER!, thickness_m_ieee754_class:TEXT!, conductivity_w_m_k_ieee754_bits:INTEGER!, conductivity_w_m_k_ieee754_class:TEXT!, density_kg_m3_ieee754_bits:INTEGER!, density_kg_m3_ieee754_class:TEXT!, specific_heat_j_kg_k_ieee754_bits:INTEGER!, specific_heat_j_kg_k_ieee754_class:TEXT!, thermal_absorptance_ieee754_bits:INTEGER!, thermal_absorptance_ieee754_class:TEXT!, solar_absorptance_ieee754_bits:INTEGER!, solar_absorptance_ieee754_class:TEXT!, visible_absorptance_ieee754_bits:INTEGER!, visible_absorptance_ieee754_class:TEXT! |
| energy_glazing_material | 38 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, thickness_m:REAL?, conductivity_w_m_k:REAL?, solar_transmittance:REAL?, solar_reflectance_front:REAL?, solar_reflectance_back:REAL?, visible_transmittance:REAL?, visible_reflectance_front:REAL?, visible_reflectance_back:REAL?, infrared_transmittance:REAL?, infrared_emissivity_front:REAL?, infrared_emissivity_back:REAL?, thickness_m_ieee754_bits:INTEGER!, thickness_m_ieee754_class:TEXT!, conductivity_w_m_k_ieee754_bits:INTEGER!, conductivity_w_m_k_ieee754_class:TEXT!, solar_transmittance_ieee754_bits:INTEGER!, solar_transmittance_ieee754_class:TEXT!, solar_reflectance_front_ieee754_bits:INTEGER!, solar_reflectance_front_ieee754_class:TEXT!, solar_reflectance_back_ieee754_bits:INTEGER!, solar_reflectance_back_ieee754_class:TEXT!, visible_transmittance_ieee754_bits:INTEGER!, visible_transmittance_ieee754_class:TEXT!, visible_reflectance_front_ieee754_bits:INTEGER!, visible_reflectance_front_ieee754_class:TEXT!, visible_reflectance_back_ieee754_bits:INTEGER!, visible_reflectance_back_ieee754_class:TEXT!, infrared_transmittance_ieee754_bits:INTEGER!, infrared_transmittance_ieee754_class:TEXT!, infrared_emissivity_front_ieee754_bits:INTEGER!, infrared_emissivity_front_ieee754_class:TEXT!, infrared_emissivity_back_ieee754_bits:INTEGER!, infrared_emissivity_back_ieee754_class:TEXT! |
| energy_gas_material | 9 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, thickness_m:REAL?, gas:TEXT!, thickness_m_ieee754_bits:INTEGER!, thickness_m_ieee754_class:TEXT! |
| energy_construction | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT! |
| energy_construction_layer | 4 | id:INTEGER?, construction_id:INTEGER!, ordinal:INTEGER!, material_id:INTEGER! |
| energy_people | 19 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, schedule_id:INTEGER!, activity_schedule_id:INTEGER!, people_per_area:REAL?, sensible_fraction:REAL?, latent_fraction:REAL?, radiant_fraction:REAL?, people_per_area_ieee754_bits:INTEGER!, people_per_area_ieee754_class:TEXT!, sensible_fraction_ieee754_bits:INTEGER!, sensible_fraction_ieee754_class:TEXT!, latent_fraction_ieee754_bits:INTEGER!, latent_fraction_ieee754_class:TEXT!, radiant_fraction_ieee754_bits:INTEGER!, radiant_fraction_ieee754_class:TEXT! |
| energy_lighting | 18 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, schedule_id:INTEGER!, watts_per_area:REAL?, radiant_fraction:REAL?, visible_fraction:REAL?, return_air_fraction:REAL?, watts_per_area_ieee754_bits:INTEGER!, watts_per_area_ieee754_class:TEXT!, radiant_fraction_ieee754_bits:INTEGER!, radiant_fraction_ieee754_class:TEXT!, visible_fraction_ieee754_bits:INTEGER!, visible_fraction_ieee754_class:TEXT!, return_air_fraction_ieee754_bits:INTEGER!, return_air_fraction_ieee754_class:TEXT! |
| energy_equipment | 15 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, schedule_id:INTEGER!, watts_per_area:REAL?, radiant_fraction:REAL?, latent_fraction:REAL?, watts_per_area_ieee754_bits:INTEGER!, watts_per_area_ieee754_class:TEXT!, radiant_fraction_ieee754_bits:INTEGER!, radiant_fraction_ieee754_class:TEXT!, latent_fraction_ieee754_bits:INTEGER!, latent_fraction_ieee754_class:TEXT! |
| energy_thermostat | 13 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, heating_setpoint_schedule_id:INTEGER!, cooling_setpoint_schedule_id:INTEGER!, heating_throttle_range_k:REAL?, cooling_throttle_range_k:REAL?, heating_throttle_range_k_ieee754_bits:INTEGER!, heating_throttle_range_k_ieee754_class:TEXT!, cooling_throttle_range_k_ieee754_bits:INTEGER!, cooling_throttle_range_k_ieee754_class:TEXT! |
| energy_humidistat | 13 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, humidifying_setpoint_schedule_id:INTEGER!, dehumidifying_setpoint_schedule_id:INTEGER!, humidifying_throttle_range:REAL?, dehumidifying_throttle_range:REAL?, humidifying_throttle_range_ieee754_bits:INTEGER!, humidifying_throttle_range_ieee754_class:TEXT!, dehumidifying_throttle_range_ieee754_bits:INTEGER!, dehumidifying_throttle_range_ieee754_class:TEXT! |
| energy_setpoint_manager | 7 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, kind:TEXT!, schedule_id:INTEGER? |
| energy_setpoint_outdoor_reset | 13 | id:INTEGER?, low_outdoor_c:REAL?, high_outdoor_c:REAL?, low_setpoint_c:REAL?, high_setpoint_c:REAL?, low_outdoor_c_ieee754_bits:INTEGER!, low_outdoor_c_ieee754_class:TEXT!, high_outdoor_c_ieee754_bits:INTEGER!, high_outdoor_c_ieee754_class:TEXT!, low_setpoint_c_ieee754_bits:INTEGER!, low_setpoint_c_ieee754_class:TEXT!, high_setpoint_c_ieee754_bits:INTEGER!, high_setpoint_c_ieee754_class:TEXT! |
| energy_ideal_loads | 23 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, max_heating_supply_air_temp_c:REAL?, min_cooling_supply_air_temp_c:REAL?, max_heating_capacity_w:REAL?, max_cooling_capacity_w:REAL?, outdoor_air_per_person_m3_s:REAL?, outdoor_air_per_area_m3_s_m2:REAL?, max_heating_supply_air_temp_c_ieee754_bits:INTEGER!, max_heating_supply_air_temp_c_ieee754_class:TEXT!, min_cooling_supply_air_temp_c_ieee754_bits:INTEGER!, min_cooling_supply_air_temp_c_ieee754_class:TEXT!, max_heating_capacity_w_ieee754_bits:INTEGER?, max_heating_capacity_w_ieee754_class:TEXT?, max_cooling_capacity_w_ieee754_bits:INTEGER?, max_cooling_capacity_w_ieee754_class:TEXT?, outdoor_air_per_person_m3_s_ieee754_bits:INTEGER!, outdoor_air_per_person_m3_s_ieee754_class:TEXT!, outdoor_air_per_area_m3_s_m2_ieee754_bits:INTEGER!, outdoor_air_per_area_m3_s_m2_ieee754_class:TEXT! |
| energy_zone_equipment | 13 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, equipment_type:TEXT!, priority:INTEGER!, heating_capacity_w:REAL?, cooling_capacity_w:REAL?, heating_capacity_w_ieee754_bits:INTEGER!, heating_capacity_w_ieee754_class:TEXT!, cooling_capacity_w_ieee754_bits:INTEGER!, cooling_capacity_w_ieee754_class:TEXT! |
| energy_air_loop | 10 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, supply_node_id:INTEGER!, return_node_id:INTEGER!, design_supply_air_flow_m3_s:REAL?, design_supply_air_flow_m3_s_ieee754_bits:INTEGER!, design_supply_air_flow_m3_s_ieee754_class:TEXT! |
| energy_air_loop_terminal | 4 | id:INTEGER?, air_loop_id:INTEGER!, ordinal:INTEGER!, zone_id:INTEGER! |
| energy_plant_loop | 15 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, loop_type:TEXT!, supply_temperature_c:REAL?, return_temperature_c:REAL?, design_flow_kg_s:REAL?, supply_temperature_c_ieee754_bits:INTEGER!, supply_temperature_c_ieee754_class:TEXT!, return_temperature_c_ieee754_bits:INTEGER!, return_temperature_c_ieee754_class:TEXT!, design_flow_kg_s_ieee754_bits:INTEGER!, design_flow_kg_s_ieee754_class:TEXT! |
| energy_plant_loop_equipment | 4 | id:INTEGER?, plant_loop_id:INTEGER!, ordinal:INTEGER!, equipment_id:INTEGER! |
| energy_outdoor_air_system | 9 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, air_loop_id:INTEGER!, min_oa_flow_m3_s:REAL?, economizer_enabled:INTEGER!, min_oa_flow_m3_s_ieee754_bits:INTEGER!, min_oa_flow_m3_s_ieee754_class:TEXT! |
| energy_infiltration | 34 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, schedule_id:INTEGER!, method:TEXT!, design_flow_ach:REAL?, flow_per_exterior_area_m3_s_m2:REAL?, effective_leakage_area_m2:REAL?, discharge_coefficient:REAL?, stack_height_m:REAL?, constant_term_coefficient:REAL?, temperature_term_coefficient:REAL?, velocity_term_coefficient:REAL?, velocity_squared_term_coefficient:REAL?, design_flow_ach_ieee754_bits:INTEGER!, design_flow_ach_ieee754_class:TEXT!, flow_per_exterior_area_m3_s_m2_ieee754_bits:INTEGER!, flow_per_exterior_area_m3_s_m2_ieee754_class:TEXT!, effective_leakage_area_m2_ieee754_bits:INTEGER!, effective_leakage_area_m2_ieee754_class:TEXT!, discharge_coefficient_ieee754_bits:INTEGER!, discharge_coefficient_ieee754_class:TEXT!, stack_height_m_ieee754_bits:INTEGER!, stack_height_m_ieee754_class:TEXT!, constant_term_coefficient_ieee754_bits:INTEGER!, constant_term_coefficient_ieee754_class:TEXT!, temperature_term_coefficient_ieee754_bits:INTEGER!, temperature_term_coefficient_ieee754_class:TEXT!, velocity_term_coefficient_ieee754_bits:INTEGER!, velocity_term_coefficient_ieee754_class:TEXT!, velocity_squared_term_coefficient_ieee754_bits:INTEGER!, velocity_squared_term_coefficient_ieee754_class:TEXT! |
| energy_mechanical_ventilation | 15 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, schedule_id:INTEGER!, design_flow_m3_s:REAL?, fan_total_efficiency:REAL?, fan_delta_pressure_pa:REAL?, design_flow_m3_s_ieee754_bits:INTEGER!, design_flow_m3_s_ieee754_class:TEXT!, fan_total_efficiency_ieee754_bits:INTEGER!, fan_total_efficiency_ieee754_class:TEXT!, fan_delta_pressure_pa_ieee754_bits:INTEGER!, fan_delta_pressure_pa_ieee754_class:TEXT! |
| energy_shading_surface | 6 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT!, transmittance_schedule_id:INTEGER? |
| energy_shading_vertex | 12 | id:INTEGER?, shading_surface_id:INTEGER!, ordinal:INTEGER!, x_m:REAL?, y_m:REAL?, z_m:REAL?, x_m_ieee754_bits:INTEGER!, x_m_ieee754_class:TEXT!, y_m_ieee754_bits:INTEGER!, y_m_ieee754_class:TEXT!, z_m_ieee754_bits:INTEGER!, z_m_ieee754_class:TEXT! |
| energy_space_list | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT! |
| energy_space_list_member | 4 | id:INTEGER?, space_list_id:INTEGER!, ordinal:INTEGER!, space_id:INTEGER! |
| energy_thermal_enclosure | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT! |
| energy_thermal_enclosure_zone | 4 | id:INTEGER?, thermal_enclosure_id:INTEGER!, ordinal:INTEGER!, zone_id:INTEGER! |
| energy_adjacency_pair | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, surface_a_id:INTEGER!, surface_b_id:INTEGER! |
| energy_airflow_network | 2 | id:INTEGER?, outdoor_node_id:INTEGER! |
| energy_airflow_zone_node | 5 | id:INTEGER?, network_id:INTEGER!, ordinal:INTEGER!, zone_id:INTEGER!, node_id:INTEGER! |
| energy_airflow_link | 4 | id:INTEGER?, network_id:INTEGER!, ordinal:INTEGER!, link_id:INTEGER! |
| energy_electrical_load_center | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, name:TEXT! |
| energy_electrical_generator | 4 | id:INTEGER?, load_center_id:INTEGER!, ordinal:INTEGER!, generator_id:INTEGER! |
| energy_electrical_pv | 4 | id:INTEGER?, load_center_id:INTEGER!, ordinal:INTEGER!, pv_id:INTEGER! |
| energy_electrical_battery | 4 | id:INTEGER?, load_center_id:INTEGER!, ordinal:INTEGER!, battery_id:INTEGER! |
| energy_pv_system | 22 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, dc_capacity_w:REAL?, area_m2:REAL?, tilt_deg:REAL?, azimuth_deg:REAL?, module_efficiency:REAL?, inverter_efficiency:REAL?, dc_capacity_w_ieee754_bits:INTEGER!, dc_capacity_w_ieee754_class:TEXT!, area_m2_ieee754_bits:INTEGER!, area_m2_ieee754_class:TEXT!, tilt_deg_ieee754_bits:INTEGER!, tilt_deg_ieee754_class:TEXT!, azimuth_deg_ieee754_bits:INTEGER!, azimuth_deg_ieee754_class:TEXT!, module_efficiency_ieee754_bits:INTEGER!, module_efficiency_ieee754_class:TEXT!, inverter_efficiency_ieee754_bits:INTEGER!, inverter_efficiency_ieee754_class:TEXT! |
| energy_battery | 16 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, capacity_kwh:REAL?, max_charge_w:REAL?, max_discharge_w:REAL?, round_trip_efficiency:REAL?, capacity_kwh_ieee754_bits:INTEGER!, capacity_kwh_ieee754_class:TEXT!, max_charge_w_ieee754_bits:INTEGER!, max_charge_w_ieee754_class:TEXT!, max_discharge_w_ieee754_bits:INTEGER!, max_discharge_w_ieee754_class:TEXT!, round_trip_efficiency_ieee754_bits:INTEGER!, round_trip_efficiency_ieee754_class:TEXT! |
| energy_shw_system | 14 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, heater_capacity_w:REAL?, storage_volume_m3:REAL?, setpoint_c:REAL?, schedule_id:INTEGER!, heater_capacity_w_ieee754_bits:INTEGER!, heater_capacity_w_ieee754_class:TEXT!, storage_volume_m3_ieee754_bits:INTEGER!, storage_volume_m3_ieee754_class:TEXT!, setpoint_c_ieee754_bits:INTEGER!, setpoint_c_ieee754_class:TEXT! |
| energy_solar_thermal | 19 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, collector_area_m2:REAL?, efficiency:REAL?, storage_volume_m3:REAL?, tilt_deg:REAL?, azimuth_deg:REAL?, collector_area_m2_ieee754_bits:INTEGER!, collector_area_m2_ieee754_class:TEXT!, efficiency_ieee754_bits:INTEGER!, efficiency_ieee754_class:TEXT!, storage_volume_m3_ieee754_bits:INTEGER!, storage_volume_m3_ieee754_class:TEXT!, tilt_deg_ieee754_bits:INTEGER!, tilt_deg_ieee754_class:TEXT!, azimuth_deg_ieee754_bits:INTEGER!, azimuth_deg_ieee754_class:TEXT! |
| energy_refrigeration | 9 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, case_count:INTEGER!, design_load_w:REAL?, defrost_schedule_id:INTEGER!, design_load_w_ieee754_bits:INTEGER!, design_load_w_ieee754_class:TEXT! |
| energy_water_system | 9 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, fixture_count:INTEGER!, peak_flow_l_s:REAL?, schedule_id:INTEGER!, peak_flow_l_s_ieee754_bits:INTEGER!, peak_flow_l_s_ieee754_class:TEXT! |
| energy_fault | 10 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, target_equipment_id:INTEGER!, fault_type:TEXT!, severity:REAL?, start_schedule_id:INTEGER!, severity_ieee754_bits:INTEGER!, severity_ieee754_class:TEXT! |
| energy_output_variable | 6 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, name:TEXT!, key:TEXT!, reporting_frequency:TEXT! |
| energy_sizing_object | 7 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, sizing_type:TEXT!, design_day_type:TEXT! |
| energy_daylight_zone | 14 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, entity_id:INTEGER!, zone_id:INTEGER!, illuminance_target_lux:REAL?, glare_limit:REAL?, window_transmittance:REAL?, illuminance_target_lux_ieee754_bits:INTEGER!, illuminance_target_lux_ieee754_class:TEXT!, glare_limit_ieee754_bits:INTEGER!, glare_limit_ieee754_class:TEXT!, window_transmittance_ieee754_bits:INTEGER!, window_transmittance_ieee754_class:TEXT! |
| energy_room_air_model | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, zone_id:INTEGER!, model:TEXT! |
| energy_constant_schedule | 7 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, schedule_id:INTEGER!, value:REAL?, value_ieee754_bits:INTEGER!, value_ieee754_class:TEXT! |
| energy_daily_schedule | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, schedule_id:INTEGER!, interpolation:TEXT! |
| energy_daily_hour | 6 | id:INTEGER?, schedule_id:INTEGER!, ordinal:INTEGER!, value:REAL?, value_ieee754_bits:INTEGER!, value_ieee754_class:TEXT! |
| energy_daily_limits | 7 | id:INTEGER?, min:REAL?, max:REAL?, min_ieee754_bits:INTEGER!, min_ieee754_class:TEXT!, max_ieee754_bits:INTEGER!, max_ieee754_class:TEXT! |
| energy_weekly_schedule | 4 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, schedule_id:INTEGER! |
| energy_weekly_day | 4 | id:INTEGER?, schedule_id:INTEGER!, ordinal:INTEGER!, daily_schedule_id:INTEGER! |
| energy_annual_schedule | 6 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, schedule_id:INTEGER!, default_daily_schedule_id:INTEGER!, holiday_daily_schedule_id:INTEGER? |
| energy_annual_rule | 8 | id:INTEGER?, schedule_id:INTEGER!, ordinal:INTEGER!, start_month:INTEGER!, start_day:INTEGER!, end_month:INTEGER!, end_day:INTEGER!, daily_schedule_id:INTEGER! |
| energy_annual_holiday | 6 | id:INTEGER?, schedule_id:INTEGER!, ordinal:INTEGER!, year:INTEGER!, month:INTEGER!, day:INTEGER! |
| energy_time_series_schedule | 5 | id:INTEGER?, document_id:INTEGER!, ordinal:INTEGER!, schedule_id:INTEGER!, timestep_seconds:INTEGER! |
| energy_time_series_value | 6 | id:INTEGER?, schedule_id:INTEGER!, ordinal:INTEGER!, value:REAL?, value_ieee754_bits:INTEGER!, value_ieee754_class:TEXT! |

## Exact authored emitter authorities

Each following expression is the actual authored projection role, not a guessed census. `emit` and `edge` add identity, parent and ordinal (24 integer bytes); `insert_key` adds identity (8 bytes). Float helpers append exact word and class companions to every declared REAL. Null optional cells cost zero.

[🪶️sqlite/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs)

```rust
 insert_key_ieee754(&mut p,"energy_document",1,&[Cell::Text(&s.schema),Cell::Text(&m.name),Cell::Text(&m.version),Cell::Real(m.site.latitude_deg),Cell::Real(m.site.longitude_deg),Cell::Real(m.site.elevation_m),Cell::Real(m.site.time_zone_hours),Cell::Real(m.site.north_axis_deg),Cell::Real(m.ground_temperature.deep_c),i(u32::from(m.run_period.start_month)),i(u32::from(m.run_period.start_day)),i(u32::from(m.run_period.end_month)),i(u32::from(m.run_period.end_day)),i(u32::from(m.run_period.year))],DOCUMENT)?;
 literal_child(&mut p,"energy_structure_child",&s.structure)?;literal_child(&mut p,"energy_zones_child",&s.zones)?;
 if let Some(link)=&s.referenced_model{literal_link(&mut p,"energy_referenced_model_link",link)?;}if let Some(link)=&s.weather_link{literal_link(&mut p,"energy_weather_link",link)?;}
 for n in 0..12{emit(&mut p,"energy_ground_month",n,&[Cell::Real(m.ground_temperature.building_surface_c[n]),Cell::Real(m.ground_temperature.shallow_c[n])],MONTH)?;}
```

[🪶️sqlite/🏘️envelope/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🏘️envelope/🦀️.rs)

```rust
 for(n,v)in m.zones.iter().enumerate(){emit(p,"energy_zone",n,&[i(v.id.0),Cell::Text(&v.name),Cell::Real(v.volume_m3),i(v.multiplier),b(v.conditioned),b(v.part_of_total_floor_area)],ZONE)?;}
 for(n,v)in m.spaces.iter().enumerate(){emit(p,"energy_space",n,&[i(v.id.0),Cell::Text(&v.name),i(v.zone_id.0),Cell::Real(v.floor_area_m2)],SPACE)?;}
 for(n,v)in m.surfaces.iter().enumerate(){let(kind,partner)=match v.outside_boundary_condition{OutsideBoundary::OutdoorAir=>("OutdoorAir",Cell::Null),OutsideBoundary::Ground=>("Ground",Cell::Null),OutsideBoundary::OtherSideTemperature=>("OtherSideTemperature",Cell::Null),OutsideBoundary::Adiabatic=>("Adiabatic",Cell::Null),OutsideBoundary::Interzone(id)=>("Interzone",i(id.0))};let parent=emit(p,"energy_surface",n,&[i(v.id.0),Cell::Text(&v.name),i(v.zone_id.0),Cell::Text(surface_class(v.class)),i(v.construction_id.0),Cell::Text(kind),partner,b(v.sun_exposed),b(v.wind_exposed),i(v.multiplier)],&[])?;vertices(p,"energy_surface_vertex",parent,&v.vertices_m)?;}
 for(n,v)in m.fenestrations.iter().enumerate(){let parent=emit(p,"energy_fenestration",n,&[i(v.id.0),Cell::Text(&v.name),i(v.surface_id.0),Cell::Real(v.u_value_w_m2k),Cell::Real(v.shgc),Cell::Real(v.vlt),Cell::Real(v.area_m2),Cell::Real(v.height_m),Cell::Real(v.sill_height_m),Cell::Real(v.frame_conductance_w_k),Cell::Real(v.divider_conductance_w_k),Cell::Real(v.overhang_depth_m),Cell::Real(v.overhang_offset_m),Cell::Real(v.fin_depth_m),Cell::Real(v.fin_offset_m),optional_id(v.glazing_construction_id)],FENESTRATION)?;vertices(p,"energy_fenestration_vertex",parent,&v.vertices_m)?;}
 for(n,v)in m.materials.iter().enumerate(){emit(p,"energy_material",n,&[i(v.id.0),Cell::Text(&v.name),Cell::Text(roughness(v.roughness)),Cell::Real(v.thickness_m),Cell::Real(v.conductivity_w_m_k),Cell::Real(v.density_kg_m3),Cell::Real(v.specific_heat_j_kg_k),Cell::Real(v.thermal_absorptance),Cell::Real(v.solar_absorptance),Cell::Real(v.visible_absorptance)],MATERIAL)?;}
 for(n,v)in m.glazing_materials.iter().enumerate(){emit(p,"energy_glazing_material",n,&[i(v.id.0),Cell::Text(&v.name),Cell::Real(v.thickness_m),Cell::Real(v.conductivity_w_m_k),Cell::Real(v.solar_transmittance),Cell::Real(v.solar_reflectance_front),Cell::Real(v.solar_reflectance_back),Cell::Real(v.visible_transmittance),Cell::Real(v.visible_reflectance_front),Cell::Real(v.visible_reflectance_back),Cell::Real(v.infrared_transmittance),Cell::Real(v.infrared_emissivity_front),Cell::Real(v.infrared_emissivity_back)],GLAZING)?;}
 for(n,v)in m.gas_materials.iter().enumerate(){emit(p,"energy_gas_material",n,&[i(v.id.0),Cell::Text(&v.name),Cell::Real(v.thickness_m),Cell::Text(gas(v.gas))],GAS)?;}
 for(n,v)in m.constructions.iter().enumerate(){let parent=emit(p,"energy_construction",n,&[i(v.id.0),Cell::Text(&v.name)],&[])?;ids(p,"energy_construction_layer",parent,&v.layer_material_ids)?;}
 for(n,v)in m.shading_surfaces.iter().enumerate(){let parent=emit(p,"energy_shading_surface",n,&[i(v.id.0),Cell::Text(&v.name),optional_schedule(v.transmittance_schedule_id)],&[])?;vertices(p,"energy_shading_vertex",parent,&v.vertices_m)?;}
 for(n,v)in m.space_lists.iter().enumerate(){let parent=emit(p,"energy_space_list",n,&[i(v.id.0),Cell::Text(&v.name)],&[])?;ids(p,"energy_space_list_member",parent,&v.space_ids)?;}
 for(n,v)in m.thermal_enclosures.iter().enumerate(){let parent=emit(p,"energy_thermal_enclosure",n,&[i(v.id.0),Cell::Text(&v.name)],&[])?;ids(p,"energy_thermal_enclosure_zone",parent,&v.zone_ids)?;}
 for(n,v)in m.adjacency_pairs.iter().enumerate(){emit(p,"energy_adjacency_pair",n,&[i(v.surface_a_id.0),i(v.surface_b_id.0)],&[])?;}Ok(())
```

[🪶️sqlite/⚙️systems/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/⚙️systems/🦀️.rs)

```rust
 for(n,v)in m.people.iter().enumerate(){emit(p,"energy_people",n,&[i(v.id.0),i(v.zone_id.0),i(v.schedule_id.0),i(v.activity_schedule_id.0),Cell::Real(v.people_per_area),Cell::Real(v.sensible_fraction),Cell::Real(v.latent_fraction),Cell::Real(v.radiant_fraction)],PEOPLE)?;}
 for(n,v)in m.lighting.iter().enumerate(){emit(p,"energy_lighting",n,&[i(v.id.0),i(v.zone_id.0),i(v.schedule_id.0),Cell::Real(v.watts_per_area),Cell::Real(v.radiant_fraction),Cell::Real(v.visible_fraction),Cell::Real(v.return_air_fraction)],LIGHTING)?;}
 for(n,v)in m.equipment.iter().enumerate(){emit(p,"energy_equipment",n,&[i(v.id.0),i(v.zone_id.0),i(v.schedule_id.0),Cell::Real(v.watts_per_area),Cell::Real(v.radiant_fraction),Cell::Real(v.latent_fraction)],EQUIPMENT)?;}
 for(n,v)in m.thermostats.iter().enumerate(){emit(p,"energy_thermostat",n,&[i(v.id.0),i(v.zone_id.0),i(v.heating_setpoint_schedule_id.0),i(v.cooling_setpoint_schedule_id.0),Cell::Real(v.heating_throttle_range_k),Cell::Real(v.cooling_throttle_range_k)],THERMOSTAT)?;}
 for(n,v)in m.humidistats.iter().enumerate(){emit(p,"energy_humidistat",n,&[i(v.id.0),i(v.zone_id.0),i(v.humidifying_setpoint_schedule_id.0),i(v.dehumidifying_setpoint_schedule_id.0),Cell::Real(v.humidifying_throttle_range),Cell::Real(v.dehumidifying_throttle_range)],THERMOSTAT)?;}
 for(n,v)in m.setpoint_managers.iter().enumerate(){let kind=match &v.kind{SetpointManagerKind::Scheduled=>"Scheduled",SetpointManagerKind::OutdoorAirReset{..}=>"OutdoorAirReset",SetpointManagerKind::WarmestZone=>"WarmestZone",SetpointManagerKind::ColdestZone=>"ColdestZone"};let id=emit(p,"energy_setpoint_manager",n,&[i(v.id.0),Cell::Text(&v.name),Cell::Text(kind),optional_schedule(v.schedule_id)],&[])?;if let SetpointManagerKind::OutdoorAirReset{low_outdoor_c,high_outdoor_c,low_setpoint_c,high_setpoint_c}=v.kind{insert_key_ieee754(p,"energy_setpoint_outdoor_reset",id,&[Cell::Real(low_outdoor_c),Cell::Real(high_outdoor_c),Cell::Real(low_setpoint_c),Cell::Real(high_setpoint_c)],RESET)?;}}
 for(n,v)in m.ideal_loads.iter().enumerate(){emit(p,"energy_ideal_loads",n,&[i(v.id.0),i(v.zone_id.0),Cell::Real(v.max_heating_supply_air_temp_c),Cell::Real(v.min_cooling_supply_air_temp_c),optional_float(v.max_heating_capacity_w),optional_float(v.max_cooling_capacity_w),Cell::Real(v.outdoor_air_per_person_m3_s),Cell::Real(v.outdoor_air_per_area_m3_s_m2)],IDEAL)?;}
 for(n,v)in m.zone_equipment.iter().enumerate(){emit(p,"energy_zone_equipment",n,&[i(v.id.0),i(v.zone_id.0),Cell::Text(zone_equipment(&v.equipment_type)),i(u32::from(v.priority)),Cell::Real(v.heating_capacity_w),Cell::Real(v.cooling_capacity_w)],ZONE_EQUIPMENT)?;}
 for(n,v)in m.air_loops.iter().enumerate(){let id=emit(p,"energy_air_loop",n,&[i(v.id.0),Cell::Text(&v.name),i(v.supply_node_id),i(v.return_node_id),Cell::Real(v.design_supply_air_flow_m3_s)],AIR_LOOP)?;ids(p,"energy_air_loop_terminal",id,&v.terminal_zone_ids)?;}
 for(n,v)in m.plant_loops.iter().enumerate(){let kind=match v.loop_type{PlantLoopType::Heating=>"Heating",PlantLoopType::Cooling=>"Cooling",PlantLoopType::Condenser=>"Condenser"};let id=emit(p,"energy_plant_loop",n,&[i(v.id.0),Cell::Text(&v.name),Cell::Text(kind),Cell::Real(v.supply_temperature_c),Cell::Real(v.return_temperature_c),Cell::Real(v.design_flow_kg_s)],PLANT)?;ids(p,"energy_plant_loop_equipment",id,&v.equipment_ids)?;}
 for(n,v)in m.outdoor_air_systems.iter().enumerate(){emit(p,"energy_outdoor_air_system",n,&[i(v.id.0),i(v.air_loop_id.0),Cell::Real(v.min_oa_flow_m3_s),b(v.economizer_enabled)],OUTDOOR)?;}
 for(n,v)in m.infiltrations.iter().enumerate(){let method=match v.method{crate::air_exchange::InfiltrationMethod::ScheduledAch=>"ScheduledAch",crate::air_exchange::InfiltrationMethod::PerExteriorArea=>"PerExteriorArea",crate::air_exchange::InfiltrationMethod::EffectiveLeakageArea=>"EffectiveLeakageArea",crate::air_exchange::InfiltrationMethod::WindAndStack=>"WindAndStack"};emit(p,"energy_infiltration",n,&[i(v.id.0),i(v.zone_id.0),i(v.schedule_id.0),Cell::Text(method),Cell::Real(v.design_flow_ach),Cell::Real(v.flow_per_exterior_area_m3_s_m2),Cell::Real(v.effective_leakage_area_m2),Cell::Real(v.discharge_coefficient),Cell::Real(v.stack_height_m),Cell::Real(v.constant_term_coefficient),Cell::Real(v.temperature_term_coefficient),Cell::Real(v.velocity_term_coefficient),Cell::Real(v.velocity_squared_term_coefficient)],INFILTRATION)?;}
 for(n,v)in m.mechanical_ventilations.iter().enumerate(){emit(p,"energy_mechanical_ventilation",n,&[i(v.id.0),i(v.zone_id.0),i(v.schedule_id.0),Cell::Real(v.design_flow_m3_s),Cell::Real(v.fan_total_efficiency),Cell::Real(v.fan_delta_pressure_pa)],EQUIPMENT)?;}
 if let Some(v)=&m.airflow_network{p.insert_key("energy_airflow_network",1,&[i(v.outdoor_node_id)])?;for(n,(zone,node))in v.zone_node_ids.iter().enumerate(){edge(p,"energy_airflow_zone_node",1,n,&[i(zone.0),i(*node)],&[])?;}for(n,link)in v.link_ids.iter().enumerate(){edge(p,"energy_airflow_link",1,n,&[i(*link)],&[])?;}}
 for(n,v)in m.electrical_load_centers.iter().enumerate(){let id=emit(p,"energy_electrical_load_center",n,&[i(v.id.0),Cell::Text(&v.name)],&[])?;ids(p,"energy_electrical_generator",id,&v.generator_ids)?;ids(p,"energy_electrical_pv",id,&v.pv_ids)?;ids(p,"energy_electrical_battery",id,&v.battery_ids)?;}
 for(n,v)in m.pv_systems.iter().enumerate(){emit(p,"energy_pv_system",n,&[i(v.id.0),Cell::Real(v.dc_capacity_w),Cell::Real(v.area_m2),Cell::Real(v.tilt_deg),Cell::Real(v.azimuth_deg),Cell::Real(v.module_efficiency),Cell::Real(v.inverter_efficiency)],PV)?;}
 for(n,v)in m.battery_storage.iter().enumerate(){emit(p,"energy_battery",n,&[i(v.id.0),Cell::Real(v.capacity_kwh),Cell::Real(v.max_charge_w),Cell::Real(v.max_discharge_w),Cell::Real(v.round_trip_efficiency)],BATTERY)?;}
 for(n,v)in m.shw_systems.iter().enumerate(){emit(p,"energy_shw_system",n,&[i(v.id.0),Cell::Real(v.heater_capacity_w),Cell::Real(v.storage_volume_m3),Cell::Real(v.setpoint_c),i(v.schedule_id.0)],SHW)?;}
 for(n,v)in m.solar_thermal_systems.iter().enumerate(){emit(p,"energy_solar_thermal",n,&[i(v.id.0),Cell::Real(v.collector_area_m2),Cell::Real(v.efficiency),Cell::Real(v.storage_volume_m3),Cell::Real(v.tilt_deg),Cell::Real(v.azimuth_deg)],SOLAR)?;}
 for(n,v)in m.refrigeration_systems.iter().enumerate(){emit(p,"energy_refrigeration",n,&[i(v.id.0),i(v.case_count),Cell::Real(v.design_load_w),i(v.defrost_schedule_id.0)],REFRIGERATION)?;}
 for(n,v)in m.water_systems.iter().enumerate(){emit(p,"energy_water_system",n,&[i(v.id.0),i(v.fixture_count),Cell::Real(v.peak_flow_l_s),i(v.schedule_id.0)],REFRIGERATION)?;}
 for(n,v)in m.faults.iter().enumerate(){emit(p,"energy_fault",n,&[i(v.id.0),i(v.target_equipment_id.0),Cell::Text(fault(v.fault_type)),Cell::Real(v.severity),i(v.start_schedule_id.0)],FAULT)?;}
 for(n,v)in m.output_variables.iter().enumerate(){emit(p,"energy_output_variable",n,&[Cell::Text(&v.name),Cell::Text(&v.key),Cell::Text(frequency(v.reporting_frequency))],&[])?;}
 for(n,v)in m.sizing_objects.iter().enumerate(){let kind=match v.sizing_type{SizingType::Heating=>"Heating",SizingType::Cooling=>"Cooling",SizingType::OutdoorAir=>"OutdoorAir"};let day=match v.design_day_type{DesignDayType::Heating=>"Heating",DesignDayType::Cooling=>"Cooling"};emit(p,"energy_sizing_object",n,&[i(v.id.0),i(v.zone_id.0),Cell::Text(kind),Cell::Text(day)],&[])?;}
 for(n,v)in m.daylight_zones.iter().enumerate(){emit(p,"energy_daylight_zone",n,&[i(v.id.0),i(v.zone_id.0),Cell::Real(v.illuminance_target_lux),Cell::Real(v.glare_limit),Cell::Real(v.window_transmittance)],DAYLIGHT)?;}
 for(n,v)in m.room_air_models.iter().enumerate(){emit(p,"energy_room_air_model",n,&[i(v.zone_id.0),Cell::Text(room_air(v.model))],&[])?;}Ok(())
```

[🪶️sqlite/🗓️schedules/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🗓️schedules/🦀️.rs)

```rust
 for(n,v)in s.constants.iter().enumerate(){emit(p,"energy_constant_schedule",n,&[i(v.id.0),Cell::Real(v.value)],VALUE)?;}
 for(n,v)in s.daily.iter().enumerate(){let mode=match v.interpolation{ScheduleInterpolation::Continuous=>"Continuous",ScheduleInterpolation::Discrete=>"Discrete"};let id=emit(p,"energy_daily_schedule",n,&[i(v.id.0),Cell::Text(mode)],&[])?;for(hour,value)in v.hourly_values.iter().enumerate(){edge(p,"energy_daily_hour",id,hour,&[Cell::Real(*value)],HOUR)?;}if let Some(limits)=v.limits{insert_key_ieee754(p,"energy_daily_limits",id,&[Cell::Real(limits.min),Cell::Real(limits.max)],LIMITS)?;}}
 for(n,v)in s.weekly.iter().enumerate(){let id=emit(p,"energy_weekly_schedule",n,&[i(v.id.0)],&[])?;for(day,daily)in v.daily_schedule_ids.iter().enumerate(){edge(p,"energy_weekly_day",id,day,&[i(daily.0)],&[])?;}}
 for(n,v)in s.annual.iter().enumerate(){let id=emit(p,"energy_annual_schedule",n,&[i(v.id.0),i(v.default_daily_schedule_id.0),optional_schedule(v.holiday_daily_schedule_id)],&[])?;for(n,rule)in v.rules.iter().enumerate(){edge(p,"energy_annual_rule",id,n,&[i(u32::from(rule.start_month)),i(u32::from(rule.start_day)),i(u32::from(rule.end_month)),i(u32::from(rule.end_day)),i(rule.daily_schedule_id.0)],&[])?;}for(n,(year,month,day))in v.holiday_dates.iter().enumerate(){edge(p,"energy_annual_holiday",id,n,&[i(u32::from(*year)),i(u32::from(*month)),i(u32::from(*day))],&[])?;}}
 for(n,v)in s.time_series.iter().enumerate(){let id=emit(p,"energy_time_series_schedule",n,&[i(v.id.0),i(v.timestep_seconds)],&[])?;for(n,value)in v.values.iter().enumerate(){edge(p,"energy_time_series_value",id,n,&[Cell::Real(*value)],HOUR)?;}}Ok(())
```

## Exact named model and schedule field types

The outer record uses IDs 0 schema Text, 1 model Value(DslValue), 2 structure Record, 3 zones Record, 4 referenced_model Absent or Record, 5 weather_link Absent or Record. Inside model, derived ToValue/FromValue use named snake_case object fields, not native record IDs. Actual declarations follow; newtype EntityId/ScheduleId are unsigned32 semantic words.

[✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs)

```rust
pub struct Site {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
    pub elevation_m: f64,
    pub time_zone_hours: f64,
    pub north_axis_deg: f64,
}

pub struct Zone {
    pub id: EntityId,
    pub name: String,
    pub volume_m3: f64,
    pub multiplier: u32,
    pub conditioned: bool,
    pub part_of_total_floor_area: bool,
}

pub struct Space {
    pub id: EntityId,
    pub name: String,
    pub zone_id: EntityId,
    pub floor_area_m2: f64,
}

pub enum SurfaceClass {
    ExteriorWall,
    InteriorWall,
    Roof,
    Ceiling,
    Floor,
    Interzone,
    Adiabatic,
    Ground,
}

pub struct Surface {
    pub id: EntityId,
    pub name: String,
    pub zone_id: EntityId,
    pub class: SurfaceClass,
    pub vertices_m: Vec<[f64; 3]>,
    pub construction_id: EntityId,
    pub outside_boundary_condition: OutsideBoundary,
    pub sun_exposed: bool,
    pub wind_exposed: bool,
    pub multiplier: u32,
}

pub enum OutsideBoundary {
    OutdoorAir,
    Ground,
    OtherSideTemperature,
    Adiabatic,
    Interzone(EntityId),
}

pub enum OutsideBoundaryKind {
    OutdoorAir,
    Ground,
    OtherSideTemperature,
    Adiabatic,
    Interzone,
}

pub struct Fenestration {
    pub id: EntityId,
    pub name: String,
    pub surface_id: EntityId,
    pub u_value_w_m2k: f64,
    pub shgc: f64,
    pub vlt: f64,
    pub area_m2: f64,
    pub height_m: f64,
    pub sill_height_m: f64,
    pub frame_conductance_w_k: f64,
    pub divider_conductance_w_k: f64,
    pub overhang_depth_m: f64,
    pub overhang_offset_m: f64,
    pub fin_depth_m: f64,
    pub fin_offset_m: f64,
    pub glazing_construction_id: Option<EntityId>,
    #[serde(default)]
    #[value(default)]
    pub vertices_m: Vec<[f64; 3]>,
}

pub enum SurfaceRoughness {
    VeryRough,
    Rough,
    MediumRough,
    MediumSmooth,
    Smooth,
    VerySmooth,
}

pub struct Material {
    pub id: EntityId,
    pub name: String,
    pub roughness: SurfaceRoughness,
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub density_kg_m3: f64,
    pub specific_heat_j_kg_k: f64,
    pub thermal_absorptance: f64,
    pub solar_absorptance: f64,
    pub visible_absorptance: f64,
}

pub struct GlazingMaterial {
    pub id: EntityId,
    pub name: String,
    pub thickness_m: f64,
    pub conductivity_w_m_k: f64,
    pub solar_transmittance: f64,
    pub solar_reflectance_front: f64,
    pub solar_reflectance_back: f64,
    pub visible_transmittance: f64,
    pub visible_reflectance_front: f64,
    pub visible_reflectance_back: f64,
    pub infrared_transmittance: f64,
    pub infrared_emissivity_front: f64,
    pub infrared_emissivity_back: f64,
}

pub enum GasKind {
    Air,
    Argon,
    Krypton,
    Xenon,
}

pub struct GasMaterial {
    pub id: EntityId,
    pub name: String,
    pub thickness_m: f64,
    pub gas: GasKind,
}

pub struct Construction {
    pub id: EntityId,
    pub name: String,
    pub layer_material_ids: Vec<EntityId>,
}

pub struct PeopleGain {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub activity_schedule_id: ScheduleId,
    pub people_per_area: f64,
    pub sensible_fraction: f64,
    pub latent_fraction: f64,
    pub radiant_fraction: f64,
}

pub struct LightingGain {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub visible_fraction: f64,
    pub return_air_fraction: f64,
}

pub struct EquipmentGain {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub watts_per_area: f64,
    pub radiant_fraction: f64,
    pub latent_fraction: f64,
}

pub struct Thermostat {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub heating_setpoint_schedule_id: ScheduleId,
    pub cooling_setpoint_schedule_id: ScheduleId,
    pub heating_throttle_range_k: f64,
    pub cooling_throttle_range_k: f64,
}

pub struct IdealLoadsSystem {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub max_heating_supply_air_temp_c: f64,
    pub min_cooling_supply_air_temp_c: f64,
    pub max_heating_capacity_w: Option<f64>,
    pub max_cooling_capacity_w: Option<f64>,
    pub outdoor_air_per_person_m3_s: f64,
    pub outdoor_air_per_area_m3_s_m2: f64,
}

pub struct Humidistat {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub humidifying_setpoint_schedule_id: ScheduleId,
    pub dehumidifying_setpoint_schedule_id: ScheduleId,
    pub humidifying_throttle_range: f64,
    pub dehumidifying_throttle_range: f64,
}

pub enum SetpointManagerKind {
    Scheduled,
    OutdoorAirReset { low_outdoor_c: f64, high_outdoor_c: f64, low_setpoint_c: f64, high_setpoint_c: f64 },
    WarmestZone,
    ColdestZone,
}

pub struct SetpointManager {
    pub id: EntityId,
    pub name: String,
    pub kind: SetpointManagerKind,
    pub schedule_id: Option<ScheduleId>,
}

pub struct ZoneEquipmentAssignment {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub equipment_type: ZoneEquipmentType,
    pub priority: u8,
    pub heating_capacity_w: f64,
    pub cooling_capacity_w: f64,
}

pub enum ZoneEquipmentType {
    Baseboard,
    Radiant,
    FanCoil,
    Ptac,
    VrfTerminal,
    Erv,
    UnitHeater,
    WaterToAirHp,
}

pub struct ModelAirLoop {
    pub id: EntityId,
    pub name: String,
    pub supply_node_id: u32,
    pub return_node_id: u32,
    pub design_supply_air_flow_m3_s: f64,
    pub terminal_zone_ids: Vec<EntityId>,
}

pub struct PlantLoopConfig {
    pub id: EntityId,
    pub name: String,
    pub loop_type: PlantLoopType,
    pub supply_temperature_c: f64,
    pub return_temperature_c: f64,
    pub design_flow_kg_s: f64,
    pub equipment_ids: Vec<EntityId>,
}

pub enum PlantLoopType {
    Heating,
    Cooling,
    Condenser,
}

pub struct OutdoorAirSystem {
    pub id: EntityId,
    pub air_loop_id: EntityId,
    pub min_oa_flow_m3_s: f64,
    pub economizer_enabled: bool,
}

pub struct ShadingSurface {
    pub id: EntityId,
    pub name: String,
    pub vertices_m: Vec<[f64; 3]>,
    pub transmittance_schedule_id: Option<ScheduleId>,
}

pub struct SpaceList {
    pub id: EntityId,
    pub name: String,
    pub space_ids: Vec<EntityId>,
}

pub struct ThermalEnclosure {
    pub id: EntityId,
    pub name: String,
    pub zone_ids: Vec<EntityId>,
}

pub struct AdjacencyPair {
    pub surface_a_id: EntityId,
    pub surface_b_id: EntityId,
}

pub struct MechanicalVentilation {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub design_flow_m3_s: f64,
    pub fan_total_efficiency: f64,
    pub fan_delta_pressure_pa: f64,
}

pub struct AirflowNetworkDefinition {
    pub zone_node_ids: Vec<(EntityId, u32)>,
    pub outdoor_node_id: u32,
    pub link_ids: Vec<u32>,
}

pub struct ElectricalLoadCenter {
    pub id: EntityId,
    pub name: String,
    pub generator_ids: Vec<EntityId>,
    pub pv_ids: Vec<EntityId>,
    pub battery_ids: Vec<EntityId>,
}

pub struct PvSystemAssignment {
    pub id: EntityId,
    pub dc_capacity_w: f64,
    pub area_m2: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub module_efficiency: f64,
    pub inverter_efficiency: f64,
}

pub struct BatteryAssignment {
    pub id: EntityId,
    pub capacity_kwh: f64,
    pub max_charge_w: f64,
    pub max_discharge_w: f64,
    pub round_trip_efficiency: f64,
}

pub struct ShwSystemConfig {
    pub id: EntityId,
    pub heater_capacity_w: f64,
    pub storage_volume_m3: f64,
    pub setpoint_c: f64,
    pub schedule_id: ScheduleId,
}

pub struct SolarThermalConfig {
    pub id: EntityId,
    pub collector_area_m2: f64,
    pub efficiency: f64,
    pub storage_volume_m3: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
}

pub struct RefrigerationConfig {
    pub id: EntityId,
    pub case_count: u32,
    pub design_load_w: f64,
    pub defrost_schedule_id: ScheduleId,
}

pub struct WaterSystemConfig {
    pub id: EntityId,
    pub fixture_count: u32,
    pub peak_flow_l_s: f64,
    pub schedule_id: ScheduleId,
}

pub struct FaultDefinition {
    pub id: EntityId,
    pub target_equipment_id: EntityId,
    pub fault_type: FaultType,
    pub severity: f64,
    pub start_schedule_id: ScheduleId,
}

pub enum FaultType {
    SensorBias,
    CoilFouling,
    DamperStuck,
    ChillerFouling,
    BoilerEfficiencyDegradation,
}

pub struct OutputVariableSpec {
    pub name: String,
    pub key: String,
    pub reporting_frequency: OutputReportFrequency,
}

pub enum OutputReportFrequency {
    Timestep,
    Hourly,
    Daily,
    Monthly,
    RunPeriod,
}

pub struct SizingObject {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub sizing_type: SizingType,
    pub design_day_type: DesignDayType,
}

pub enum SizingType {
    Heating,
    Cooling,
    OutdoorAir,
}

pub enum DesignDayType {
    Heating,
    Cooling,
}

pub struct DaylightZoneConfig {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub illuminance_target_lux: f64,
    pub glare_limit: f64,
    pub window_transmittance: f64,
}

pub struct RoomAirModelAssignment {
    pub zone_id: EntityId,
    pub model: RoomAirModelType,
}

pub enum RoomAirModelType {
    WellMixed,
    OneNodeDisplacement,
    TwoNodeBuoyancy,
    UnderFloorAirDistribution,
}

pub struct GroundTemperatureConfig {
    pub building_surface_c: [f64; 12],
    pub shallow_c: [f64; 12],
    pub deep_c: f64,
}

pub struct Infiltration {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub method: crate::air_exchange::InfiltrationMethod,
    pub design_flow_ach: f64,
    pub flow_per_exterior_area_m3_s_m2: f64,
    pub effective_leakage_area_m2: f64,
    pub discharge_coefficient: f64,
    pub stack_height_m: f64,
    pub constant_term_coefficient: f64,
    pub temperature_term_coefficient: f64,
    pub velocity_term_coefficient: f64,
    pub velocity_squared_term_coefficient: f64,
}

pub struct Model {
    pub name: String,
    pub version: String,
    pub site: Site,
    pub zones: Vec<Zone>,
    pub spaces: Vec<Space>,
    pub surfaces: Vec<Surface>,
    pub fenestrations: Vec<Fenestration>,
    pub materials: Vec<Material>,
    pub glazing_materials: Vec<GlazingMaterial>,
    pub gas_materials: Vec<GasMaterial>,
    pub constructions: Vec<Construction>,
    pub people: Vec<PeopleGain>,
    pub lighting: Vec<LightingGain>,
    pub equipment: Vec<EquipmentGain>,
    pub thermostats: Vec<Thermostat>,
    pub humidistats: Vec<Humidistat>,
    pub setpoint_managers: Vec<SetpointManager>,
    pub ideal_loads: Vec<IdealLoadsSystem>,
    pub zone_equipment: Vec<ZoneEquipmentAssignment>,
    pub air_loops: Vec<ModelAirLoop>,
    pub plant_loops: Vec<PlantLoopConfig>,
    pub outdoor_air_systems: Vec<OutdoorAirSystem>,
    pub infiltrations: Vec<Infiltration>,
    pub mechanical_ventilations: Vec<MechanicalVentilation>,
    pub shading_surfaces: Vec<ShadingSurface>,
    pub space_lists: Vec<SpaceList>,
    pub thermal_enclosures: Vec<ThermalEnclosure>,
    pub adjacency_pairs: Vec<AdjacencyPair>,
    pub airflow_network: Option<AirflowNetworkDefinition>,
    pub electrical_load_centers: Vec<ElectricalLoadCenter>,
    pub pv_systems: Vec<PvSystemAssignment>,
    pub battery_storage: Vec<BatteryAssignment>,
    pub shw_systems: Vec<ShwSystemConfig>,
    pub solar_thermal_systems: Vec<SolarThermalConfig>,
    pub refrigeration_systems: Vec<RefrigerationConfig>,
    pub water_systems: Vec<WaterSystemConfig>,
    pub faults: Vec<FaultDefinition>,
    pub output_variables: Vec<OutputVariableSpec>,
    pub sizing_objects: Vec<SizingObject>,
    pub daylight_zones: Vec<DaylightZoneConfig>,
    pub room_air_models: Vec<RoomAirModelAssignment>,
    pub ground_temperature: GroundTemperatureConfig,
    pub run_period: crate::calendar::RunPeriod,
    pub schedules: crate::schedule::ScheduleSet,
}
```

[✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🗓️schedule/🦀️.rs)

```rust
pub enum ScheduleInterpolation {
    Continuous,
    Discrete,
}

pub struct ScheduleLimits {
    pub min: f64,
    pub max: f64,
}

pub struct ConstantSchedule {
    pub id: ScheduleId,
    pub value: f64,
}

pub struct DailySchedule {
    pub id: ScheduleId,
    pub hourly_values: [f64; 24],
    pub interpolation: ScheduleInterpolation,
    pub limits: Option<ScheduleLimits>,
}

pub struct WeeklySchedule {
    pub id: ScheduleId,
    pub daily_schedule_ids: [ScheduleId; 7],
}

pub struct CompactScheduleRule {
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub daily_schedule_id: ScheduleId,
}

pub struct AnnualSchedule {
    pub id: ScheduleId,
    pub rules: Vec<CompactScheduleRule>,
    pub default_daily_schedule_id: ScheduleId,
    pub holiday_daily_schedule_id: Option<ScheduleId>,
    pub holiday_dates: Vec<(u16, u8, u8)>,
}

pub struct TimeSeriesSchedule {
    pub id: ScheduleId,
    pub values: Vec<f64>,
    pub timestep_seconds: u32,
}

pub struct ScheduleSet {
    pub constants: Vec<ConstantSchedule>,
    pub daily: Vec<DailySchedule>,
    pub weekly: Vec<WeeklySchedule>,
    pub annual: Vec<AnnualSchedule>,
    pub time_series: Vec<TimeSeriesSchedule>,
}

pub struct ScheduleContext {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub day_of_week: u8,
    pub timestep_index: u32,
    pub is_dst: bool,
}
```

[✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📅️calendar/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📅️calendar/🦀️.rs)

```rust
pub struct SimDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

pub struct RunPeriod {
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub year: u16,
}

pub struct RunPeriodHours {
    current: SimDate,
    end: SimDate,
    hour: u8,
    index: u32,
    finished: bool,
}

pub struct DstRule {
    pub start_month: u8,
    pub start_week: u8,
    pub end_month: u8,
    pub end_week: u8,
    pub shift_hours: f64,
}
```
