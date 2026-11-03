CREATE TABLE en1997_document (
 id INTEGER PRIMARY KEY CHECK(id=1), structure_id TEXT NOT NULL, geotechnical_category INTEGER NOT NULL CHECK(geotechnical_category BETWEEN 0 AND 255), design_situation TEXT NOT NULL, design_approach TEXT NOT NULL, annex TEXT NOT NULL CHECK(annex IN ('En','De')),
 groundwater_level REAL, investigation_depth REAL,
 groundwater_level_ieee754_bits INTEGER NOT NULL, groundwater_level_ieee754_class TEXT NOT NULL,
 investigation_depth_ieee754_bits INTEGER NOT NULL, investigation_depth_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_soil_layer (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, soil_type TEXT NOT NULL CHECK(soil_type IN ('sand','gravel','clay','silt','fill')),
 depth_top REAL, depth_bottom REAL, gamma REAL, gamma_prime REAL, phi_prime_deg REAL, cohesion_effective REAL, cohesion_undrained REAL, oedometric_modulus REAL, poisson_ratio REAL, cpt_qc REAL, spt_n REAL, cpt_nkt REAL,
 depth_top_ieee754_bits INTEGER NOT NULL, depth_top_ieee754_class TEXT NOT NULL,
 depth_bottom_ieee754_bits INTEGER NOT NULL, depth_bottom_ieee754_class TEXT NOT NULL,
 gamma_ieee754_bits INTEGER NOT NULL, gamma_ieee754_class TEXT NOT NULL,
 gamma_prime_ieee754_bits INTEGER NOT NULL, gamma_prime_ieee754_class TEXT NOT NULL,
 phi_prime_deg_ieee754_bits INTEGER NOT NULL, phi_prime_deg_ieee754_class TEXT NOT NULL,
 cohesion_effective_ieee754_bits INTEGER NOT NULL, cohesion_effective_ieee754_class TEXT NOT NULL,
 cohesion_undrained_ieee754_bits INTEGER NOT NULL, cohesion_undrained_ieee754_class TEXT NOT NULL,
 oedometric_modulus_ieee754_bits INTEGER NOT NULL, oedometric_modulus_ieee754_class TEXT NOT NULL,
 poisson_ratio_ieee754_bits INTEGER NOT NULL, poisson_ratio_ieee754_class TEXT NOT NULL,
 cpt_qc_ieee754_bits INTEGER NOT NULL, cpt_qc_ieee754_class TEXT NOT NULL,
 spt_n_ieee754_bits INTEGER NOT NULL, spt_n_ieee754_class TEXT NOT NULL,
 cpt_nkt_ieee754_bits INTEGER NOT NULL, cpt_nkt_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_footing (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL,
 width REAL, length REAL, embedment REAL, base_inclination_deg REAL, settlement_limit REAL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL,
 embedment_ieee754_bits INTEGER NOT NULL, embedment_ieee754_class TEXT NOT NULL,
 base_inclination_deg_ieee754_bits INTEGER NOT NULL, base_inclination_deg_ieee754_class TEXT NOT NULL,
 settlement_limit_ieee754_bits INTEGER NOT NULL, settlement_limit_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_foundation_load_case (
 id INTEGER PRIMARY KEY, footing_id INTEGER NOT NULL REFERENCES en1997_footing(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, design_situation TEXT NOT NULL CHECK(design_situation IN ('persistent','transient','accidental')),
 vertical_permanent REAL, vertical_variable REAL, horizontal_permanent REAL, horizontal_variable REAL, moment_permanent REAL, moment_variable REAL,
 vertical_permanent_ieee754_bits INTEGER NOT NULL, vertical_permanent_ieee754_class TEXT NOT NULL,
 vertical_variable_ieee754_bits INTEGER NOT NULL, vertical_variable_ieee754_class TEXT NOT NULL,
 horizontal_permanent_ieee754_bits INTEGER NOT NULL, horizontal_permanent_ieee754_class TEXT NOT NULL,
 horizontal_variable_ieee754_bits INTEGER NOT NULL, horizontal_variable_ieee754_class TEXT NOT NULL,
 moment_permanent_ieee754_bits INTEGER NOT NULL, moment_permanent_ieee754_class TEXT NOT NULL,
 moment_variable_ieee754_bits INTEGER NOT NULL, moment_variable_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_pile (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, pile_type TEXT NOT NULL CHECK(pile_type IN ('bored','driven','cfa')),
 diameter REAL, length REAL, count INTEGER NOT NULL CHECK(count BETWEEN 0 AND 4294967295), alpha_s REAL, unit_shaft_resistance REAL, unit_base_resistance REAL, compression_permanent REAL, compression_variable REAL, tension_permanent REAL, tension_variable REAL, bending_stiffness_ei REAL, horizontal_subgrade_modulus REAL, lateral_head_load REAL, elastic_modulus REAL, settling_thickness REAL, negative_friction_beta REAL, dynamic_test_count REAL, dynamic_resistance_mean REAL, dynamic_resistance_min REAL, hammer_weight REAL, drop_height REAL, set_per_blow REAL, elastic_compression REAL, hammer_efficiency REAL, concrete_class TEXT NOT NULL CHECK(concrete_class IN ('notGiven','c1215','c1620','c2025','c2530','c3037','c3545','c4050','c4555','c5060')),
 diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL,
 alpha_s_ieee754_bits INTEGER NOT NULL, alpha_s_ieee754_class TEXT NOT NULL,
 unit_shaft_resistance_ieee754_bits INTEGER NOT NULL, unit_shaft_resistance_ieee754_class TEXT NOT NULL,
 unit_base_resistance_ieee754_bits INTEGER NOT NULL, unit_base_resistance_ieee754_class TEXT NOT NULL,
 compression_permanent_ieee754_bits INTEGER NOT NULL, compression_permanent_ieee754_class TEXT NOT NULL,
 compression_variable_ieee754_bits INTEGER NOT NULL, compression_variable_ieee754_class TEXT NOT NULL,
 tension_permanent_ieee754_bits INTEGER NOT NULL, tension_permanent_ieee754_class TEXT NOT NULL,
 tension_variable_ieee754_bits INTEGER NOT NULL, tension_variable_ieee754_class TEXT NOT NULL,
 bending_stiffness_ei_ieee754_bits INTEGER NOT NULL, bending_stiffness_ei_ieee754_class TEXT NOT NULL,
 horizontal_subgrade_modulus_ieee754_bits INTEGER NOT NULL, horizontal_subgrade_modulus_ieee754_class TEXT NOT NULL,
 lateral_head_load_ieee754_bits INTEGER NOT NULL, lateral_head_load_ieee754_class TEXT NOT NULL,
 elastic_modulus_ieee754_bits INTEGER NOT NULL, elastic_modulus_ieee754_class TEXT NOT NULL,
 settling_thickness_ieee754_bits INTEGER NOT NULL, settling_thickness_ieee754_class TEXT NOT NULL,
 negative_friction_beta_ieee754_bits INTEGER NOT NULL, negative_friction_beta_ieee754_class TEXT NOT NULL,
 dynamic_test_count_ieee754_bits INTEGER NOT NULL, dynamic_test_count_ieee754_class TEXT NOT NULL,
 dynamic_resistance_mean_ieee754_bits INTEGER NOT NULL, dynamic_resistance_mean_ieee754_class TEXT NOT NULL,
 dynamic_resistance_min_ieee754_bits INTEGER NOT NULL, dynamic_resistance_min_ieee754_class TEXT NOT NULL,
 hammer_weight_ieee754_bits INTEGER NOT NULL, hammer_weight_ieee754_class TEXT NOT NULL,
 drop_height_ieee754_bits INTEGER NOT NULL, drop_height_ieee754_class TEXT NOT NULL,
 set_per_blow_ieee754_bits INTEGER NOT NULL, set_per_blow_ieee754_class TEXT NOT NULL,
 elastic_compression_ieee754_bits INTEGER NOT NULL, elastic_compression_ieee754_class TEXT NOT NULL,
 hammer_efficiency_ieee754_bits INTEGER NOT NULL, hammer_efficiency_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_pile_test_profile (
 id INTEGER PRIMARY KEY, pile_id INTEGER NOT NULL REFERENCES en1997_pile(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, shaft_resistance REAL, base_resistance REAL,
 shaft_resistance_ieee754_bits INTEGER NOT NULL, shaft_resistance_ieee754_class TEXT NOT NULL,
 base_resistance_ieee754_bits INTEGER NOT NULL, base_resistance_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_retaining_wall (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL,
 height REAL, embedment REAL, base_width REAL, stem_thickness REAL, backfill_phi_deg REAL, backfill_gamma REAL, wall_friction_deg REAL, earth_pressure_mode TEXT NOT NULL CHECK(earth_pressure_mode IN ('active','atRest','increasedActive')), wall_movement TEXT NOT NULL CHECK(wall_movement IN ('free','rigid','propped')), ocr REAL, concrete_gamma REAL, surcharge REAL, vertical_permanent REAL, horizontal_permanent REAL, seismic_kh REAL, seismic_kv REAL, reinforcement_spacing REAL, reinforcement_width REAL, reinforcement_friction REAL,
 height_ieee754_bits INTEGER NOT NULL, height_ieee754_class TEXT NOT NULL,
 embedment_ieee754_bits INTEGER NOT NULL, embedment_ieee754_class TEXT NOT NULL,
 base_width_ieee754_bits INTEGER NOT NULL, base_width_ieee754_class TEXT NOT NULL,
 stem_thickness_ieee754_bits INTEGER NOT NULL, stem_thickness_ieee754_class TEXT NOT NULL,
 backfill_phi_deg_ieee754_bits INTEGER NOT NULL, backfill_phi_deg_ieee754_class TEXT NOT NULL,
 backfill_gamma_ieee754_bits INTEGER NOT NULL, backfill_gamma_ieee754_class TEXT NOT NULL,
 wall_friction_deg_ieee754_bits INTEGER NOT NULL, wall_friction_deg_ieee754_class TEXT NOT NULL,
 ocr_ieee754_bits INTEGER NOT NULL, ocr_ieee754_class TEXT NOT NULL,
 concrete_gamma_ieee754_bits INTEGER NOT NULL, concrete_gamma_ieee754_class TEXT NOT NULL,
 surcharge_ieee754_bits INTEGER NOT NULL, surcharge_ieee754_class TEXT NOT NULL,
 vertical_permanent_ieee754_bits INTEGER NOT NULL, vertical_permanent_ieee754_class TEXT NOT NULL,
 horizontal_permanent_ieee754_bits INTEGER NOT NULL, horizontal_permanent_ieee754_class TEXT NOT NULL,
 seismic_kh_ieee754_bits INTEGER NOT NULL, seismic_kh_ieee754_class TEXT NOT NULL,
 seismic_kv_ieee754_bits INTEGER NOT NULL, seismic_kv_ieee754_class TEXT NOT NULL,
 reinforcement_spacing_ieee754_bits INTEGER NOT NULL, reinforcement_spacing_ieee754_class TEXT NOT NULL,
 reinforcement_width_ieee754_bits INTEGER NOT NULL, reinforcement_width_ieee754_class TEXT NOT NULL,
 reinforcement_friction_ieee754_bits INTEGER NOT NULL, reinforcement_friction_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_slope (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, angle_deg REAL, height REAL, length REAL, governing_layer_id TEXT NOT NULL, circle_centre_x REAL, circle_centre_z REAL, circle_radius REAL, width REAL,
 angle_deg_ieee754_bits INTEGER NOT NULL, angle_deg_ieee754_class TEXT NOT NULL,
 height_ieee754_bits INTEGER NOT NULL, height_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL,
 circle_centre_x_ieee754_bits INTEGER NOT NULL, circle_centre_x_ieee754_class TEXT NOT NULL,
 circle_centre_z_ieee754_bits INTEGER NOT NULL, circle_centre_z_ieee754_class TEXT NOT NULL,
 circle_radius_ieee754_bits INTEGER NOT NULL, circle_radius_ieee754_class TEXT NOT NULL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_uplift_case (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, permanent_stabilizing REAL, permanent_destabilizing REAL, variable_destabilizing REAL, pore_pressure REAL, total_stress REAL,
 permanent_stabilizing_ieee754_bits INTEGER NOT NULL, permanent_stabilizing_ieee754_class TEXT NOT NULL,
 permanent_destabilizing_ieee754_bits INTEGER NOT NULL, permanent_destabilizing_ieee754_class TEXT NOT NULL,
 variable_destabilizing_ieee754_bits INTEGER NOT NULL, variable_destabilizing_ieee754_class TEXT NOT NULL,
 pore_pressure_ieee754_bits INTEGER NOT NULL, pore_pressure_ieee754_class TEXT NOT NULL,
 total_stress_ieee754_bits INTEGER NOT NULL, total_stress_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_anchor (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, wall_id TEXT NOT NULL, tendon TEXT NOT NULL CHECK(tendon IN ('strand','bar')), head_depth REAL, inclination_deg REAL, free_length REAL, fixed_length REAL, fixed_diameter REAL, bond_stress REAL, design_force REAL, tendon_yield_force REAL, min_free_length REAL, test_factor REAL,
 head_depth_ieee754_bits INTEGER NOT NULL, head_depth_ieee754_class TEXT NOT NULL,
 inclination_deg_ieee754_bits INTEGER NOT NULL, inclination_deg_ieee754_class TEXT NOT NULL,
 free_length_ieee754_bits INTEGER NOT NULL, free_length_ieee754_class TEXT NOT NULL,
 fixed_length_ieee754_bits INTEGER NOT NULL, fixed_length_ieee754_class TEXT NOT NULL,
 fixed_diameter_ieee754_bits INTEGER NOT NULL, fixed_diameter_ieee754_class TEXT NOT NULL,
 bond_stress_ieee754_bits INTEGER NOT NULL, bond_stress_ieee754_class TEXT NOT NULL,
 design_force_ieee754_bits INTEGER NOT NULL, design_force_ieee754_class TEXT NOT NULL,
 tendon_yield_force_ieee754_bits INTEGER NOT NULL, tendon_yield_force_ieee754_class TEXT NOT NULL,
 min_free_length_ieee754_bits INTEGER NOT NULL, min_free_length_ieee754_class TEXT NOT NULL,
 test_factor_ieee754_bits INTEGER NOT NULL, test_factor_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_anchor_test (
 id INTEGER PRIMARY KEY, anchor_id INTEGER NOT NULL REFERENCES en1997_anchor(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, pull_out_resistance REAL,
 pull_out_resistance_ieee754_bits INTEGER NOT NULL, pull_out_resistance_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_embedded_wall (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('freeEarth','cantilever')), retained_height REAL, embedment REAL, prop_depth REAL, thickness REAL, backfill_gamma REAL, phi_deg REAL, delta_active_deg REAL, delta_passive_deg REAL, surcharge REAL, undrained_strength REAL,
 retained_height_ieee754_bits INTEGER NOT NULL, retained_height_ieee754_class TEXT NOT NULL,
 embedment_ieee754_bits INTEGER NOT NULL, embedment_ieee754_class TEXT NOT NULL,
 prop_depth_ieee754_bits INTEGER NOT NULL, prop_depth_ieee754_class TEXT NOT NULL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL,
 backfill_gamma_ieee754_bits INTEGER NOT NULL, backfill_gamma_ieee754_class TEXT NOT NULL,
 phi_deg_ieee754_bits INTEGER NOT NULL, phi_deg_ieee754_class TEXT NOT NULL,
 delta_active_deg_ieee754_bits INTEGER NOT NULL, delta_active_deg_ieee754_class TEXT NOT NULL,
 delta_passive_deg_ieee754_bits INTEGER NOT NULL, delta_passive_deg_ieee754_class TEXT NOT NULL,
 surcharge_ieee754_bits INTEGER NOT NULL, surcharge_ieee754_class TEXT NOT NULL,
 undrained_strength_ieee754_bits INTEGER NOT NULL, undrained_strength_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_pile_group (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, pile_id TEXT NOT NULL, rows INTEGER NOT NULL CHECK(rows BETWEEN 0 AND 4294967295), columns INTEGER NOT NULL CHECK(columns BETWEEN 0 AND 4294967295), spacing REAL, cap_vertical REAL, cap_moment_x REAL, cap_moment_y REAL, single_resistance REAL, shaft_cohesion_sum REAL, base_cohesion REAL,
 spacing_ieee754_bits INTEGER NOT NULL, spacing_ieee754_class TEXT NOT NULL,
 cap_vertical_ieee754_bits INTEGER NOT NULL, cap_vertical_ieee754_class TEXT NOT NULL,
 cap_moment_x_ieee754_bits INTEGER NOT NULL, cap_moment_x_ieee754_class TEXT NOT NULL,
 cap_moment_y_ieee754_bits INTEGER NOT NULL, cap_moment_y_ieee754_class TEXT NOT NULL,
 single_resistance_ieee754_bits INTEGER NOT NULL, single_resistance_ieee754_class TEXT NOT NULL,
 shaft_cohesion_sum_ieee754_bits INTEGER NOT NULL, shaft_cohesion_sum_ieee754_class TEXT NOT NULL,
 base_cohesion_ieee754_bits INTEGER NOT NULL, base_cohesion_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_lab_test (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, layer_id TEXT NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('oedometer','triaxial','directShear')), void_ratio REAL, preload_stress REAL, drainage_height REAL, consolidation_time REAL, compression_index REAL, swelling_index REAL, stress_increment REAL, secondary_compression_index REAL, design_life REAL,
 void_ratio_ieee754_bits INTEGER NOT NULL, void_ratio_ieee754_class TEXT NOT NULL,
 preload_stress_ieee754_bits INTEGER NOT NULL, preload_stress_ieee754_class TEXT NOT NULL,
 drainage_height_ieee754_bits INTEGER NOT NULL, drainage_height_ieee754_class TEXT NOT NULL,
 consolidation_time_ieee754_bits INTEGER NOT NULL, consolidation_time_ieee754_class TEXT NOT NULL,
 compression_index_ieee754_bits INTEGER NOT NULL, compression_index_ieee754_class TEXT NOT NULL,
 swelling_index_ieee754_bits INTEGER NOT NULL, swelling_index_ieee754_class TEXT NOT NULL,
 stress_increment_ieee754_bits INTEGER NOT NULL, stress_increment_ieee754_class TEXT NOT NULL,
 secondary_compression_index_ieee754_bits INTEGER NOT NULL, secondary_compression_index_ieee754_class TEXT NOT NULL,
 design_life_ieee754_bits INTEGER NOT NULL, design_life_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_lab_point (
 id INTEGER PRIMARY KEY, lab_test_id INTEGER NOT NULL REFERENCES en1997_lab_test(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, normal_stress REAL, shear_stress REAL,
 normal_stress_ieee754_bits INTEGER NOT NULL, normal_stress_ieee754_class TEXT NOT NULL,
 shear_stress_ieee754_bits INTEGER NOT NULL, shear_stress_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_layer_water (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, layer_id TEXT NOT NULL, piezometric_depth REAL, permeability REAL, head_difference REAL, flow_length REAL, well_radius REAL, well_water_depth REAL, influence_radius REAL, well_capacity REAL,
 piezometric_depth_ieee754_bits INTEGER NOT NULL, piezometric_depth_ieee754_class TEXT NOT NULL,
 permeability_ieee754_bits INTEGER NOT NULL, permeability_ieee754_class TEXT NOT NULL,
 head_difference_ieee754_bits INTEGER NOT NULL, head_difference_ieee754_class TEXT NOT NULL,
 flow_length_ieee754_bits INTEGER NOT NULL, flow_length_ieee754_class TEXT NOT NULL,
 well_radius_ieee754_bits INTEGER NOT NULL, well_radius_ieee754_class TEXT NOT NULL,
 well_water_depth_ieee754_bits INTEGER NOT NULL, well_water_depth_ieee754_class TEXT NOT NULL,
 influence_radius_ieee754_bits INTEGER NOT NULL, influence_radius_ieee754_class TEXT NOT NULL,
 well_capacity_ieee754_bits INTEGER NOT NULL, well_capacity_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_placement (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, x REAL, y REAL, z REAL, yaw REAL,
 x_ieee754_bits INTEGER NOT NULL, x_ieee754_class TEXT NOT NULL,
 y_ieee754_bits INTEGER NOT NULL, y_ieee754_class TEXT NOT NULL,
 z_ieee754_bits INTEGER NOT NULL, z_ieee754_class TEXT NOT NULL,
 yaw_ieee754_bits INTEGER NOT NULL, yaw_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_case (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL, entry_id TEXT NOT NULL
);
CREATE TABLE en1997_case_number (
 id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1997_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value REAL,
 value_ieee754_bits INTEGER NOT NULL, value_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_case_text (
 id INTEGER PRIMARY KEY, case_id INTEGER NOT NULL REFERENCES en1997_case(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, value TEXT NOT NULL
);
CREATE TABLE en1997_investigation (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, position_x REAL, position_y REAL, ground_level REAL, depth REAL, kind TEXT NOT NULL CHECK(kind IN ('borehole','cpt','spt')),
 position_x_ieee754_bits INTEGER NOT NULL, position_x_ieee754_class TEXT NOT NULL,
 position_y_ieee754_bits INTEGER NOT NULL, position_y_ieee754_class TEXT NOT NULL,
 ground_level_ieee754_bits INTEGER NOT NULL, ground_level_ieee754_class TEXT NOT NULL,
 depth_ieee754_bits INTEGER NOT NULL, depth_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_investigation_reading (
 id INTEGER PRIMARY KEY, investigation_id INTEGER NOT NULL REFERENCES en1997_investigation(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, depth REAL, measurement REAL,
 depth_ieee754_bits INTEGER NOT NULL, depth_ieee754_class TEXT NOT NULL,
 measurement_ieee754_bits INTEGER NOT NULL, measurement_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_characteristic_basis (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, layer_id TEXT NOT NULL, parameter TEXT NOT NULL CHECK(parameter IN ('undrainedStrength','frictionAngle')), basis TEXT NOT NULL CHECK(basis IN ('declared','derived'))
);
