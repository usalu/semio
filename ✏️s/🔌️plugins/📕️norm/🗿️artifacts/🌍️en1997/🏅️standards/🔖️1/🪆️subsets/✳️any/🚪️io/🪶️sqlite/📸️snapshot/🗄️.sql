CREATE TABLE en1997_document (
 id INTEGER PRIMARY KEY CHECK(id=1), structure_id TEXT NOT NULL, geotechnical_category INTEGER NOT NULL CHECK(geotechnical_category BETWEEN 0 AND 255), design_situation TEXT NOT NULL, design_approach TEXT NOT NULL, annex TEXT NOT NULL CHECK(annex IN ('En','De')),
 groundwater_level REAL, investigation_depth REAL,
 groundwater_level_ieee754_bits INTEGER NOT NULL, groundwater_level_ieee754_class TEXT NOT NULL,
 investigation_depth_ieee754_bits INTEGER NOT NULL, investigation_depth_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_soil_layer (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, soil_type TEXT NOT NULL,
 depth_top REAL, depth_bottom REAL, gamma REAL, gamma_prime REAL, phi_prime_deg REAL, cohesion_effective REAL, cohesion_undrained REAL, oedometric_modulus REAL, poisson_ratio REAL, cpt_qc REAL, spt_n REAL,
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
 spt_n_ieee754_bits INTEGER NOT NULL, spt_n_ieee754_class TEXT NOT NULL
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
 id INTEGER PRIMARY KEY, footing_id INTEGER NOT NULL REFERENCES en1997_footing(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, design_situation TEXT NOT NULL,
 vertical_permanent REAL, vertical_variable REAL, horizontal_permanent REAL, horizontal_variable REAL, moment_permanent REAL, moment_variable REAL,
 vertical_permanent_ieee754_bits INTEGER NOT NULL, vertical_permanent_ieee754_class TEXT NOT NULL,
 vertical_variable_ieee754_bits INTEGER NOT NULL, vertical_variable_ieee754_class TEXT NOT NULL,
 horizontal_permanent_ieee754_bits INTEGER NOT NULL, horizontal_permanent_ieee754_class TEXT NOT NULL,
 horizontal_variable_ieee754_bits INTEGER NOT NULL, horizontal_variable_ieee754_class TEXT NOT NULL,
 moment_permanent_ieee754_bits INTEGER NOT NULL, moment_permanent_ieee754_class TEXT NOT NULL,
 moment_variable_ieee754_bits INTEGER NOT NULL, moment_variable_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_pile (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, pile_type TEXT NOT NULL,
 diameter REAL, length REAL, count INTEGER NOT NULL CHECK(count BETWEEN 0 AND 4294967295), alpha_s REAL, unit_shaft_resistance REAL, unit_base_resistance REAL, compression_permanent REAL, compression_variable REAL, tension_permanent REAL, tension_variable REAL,
 diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL,
 alpha_s_ieee754_bits INTEGER NOT NULL, alpha_s_ieee754_class TEXT NOT NULL,
 unit_shaft_resistance_ieee754_bits INTEGER NOT NULL, unit_shaft_resistance_ieee754_class TEXT NOT NULL,
 unit_base_resistance_ieee754_bits INTEGER NOT NULL, unit_base_resistance_ieee754_class TEXT NOT NULL,
 compression_permanent_ieee754_bits INTEGER NOT NULL, compression_permanent_ieee754_class TEXT NOT NULL,
 compression_variable_ieee754_bits INTEGER NOT NULL, compression_variable_ieee754_class TEXT NOT NULL,
 tension_permanent_ieee754_bits INTEGER NOT NULL, tension_permanent_ieee754_class TEXT NOT NULL,
 tension_variable_ieee754_bits INTEGER NOT NULL, tension_variable_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_pile_test_profile (
 id INTEGER PRIMARY KEY, pile_id INTEGER NOT NULL REFERENCES en1997_pile(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, shaft_resistance REAL, base_resistance REAL,
 shaft_resistance_ieee754_bits INTEGER NOT NULL, shaft_resistance_ieee754_class TEXT NOT NULL,
 base_resistance_ieee754_bits INTEGER NOT NULL, base_resistance_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_retaining_wall (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL,
 height REAL, embedment REAL, base_width REAL, stem_thickness REAL, backfill_phi_deg REAL, backfill_gamma REAL, wall_friction_deg REAL, earth_pressure_mode TEXT NOT NULL, wall_movement TEXT NOT NULL, ocr REAL, concrete_gamma REAL, surcharge REAL, vertical_permanent REAL, horizontal_permanent REAL,
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
 horizontal_permanent_ieee754_bits INTEGER NOT NULL, horizontal_permanent_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_slope (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, angle_deg REAL, height REAL, length REAL, governing_layer_id TEXT NOT NULL,
 angle_deg_ieee754_bits INTEGER NOT NULL, angle_deg_ieee754_class TEXT NOT NULL,
 height_ieee754_bits INTEGER NOT NULL, height_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1997_uplift_case (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1997_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, permanent_stabilizing REAL, permanent_destabilizing REAL, variable_destabilizing REAL, pore_pressure REAL, total_stress REAL,
 permanent_stabilizing_ieee754_bits INTEGER NOT NULL, permanent_stabilizing_ieee754_class TEXT NOT NULL,
 permanent_destabilizing_ieee754_bits INTEGER NOT NULL, permanent_destabilizing_ieee754_class TEXT NOT NULL,
 variable_destabilizing_ieee754_bits INTEGER NOT NULL, variable_destabilizing_ieee754_class TEXT NOT NULL,
 pore_pressure_ieee754_bits INTEGER NOT NULL, pore_pressure_ieee754_class TEXT NOT NULL,
 total_stress_ieee754_bits INTEGER NOT NULL, total_stress_ieee754_class TEXT NOT NULL
);
