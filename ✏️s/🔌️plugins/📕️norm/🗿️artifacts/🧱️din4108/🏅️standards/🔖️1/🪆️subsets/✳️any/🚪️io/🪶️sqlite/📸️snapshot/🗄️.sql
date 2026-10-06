CREATE TABLE din4108_document (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  climate_zone TEXT NOT NULL CHECK (climate_zone IN ('Zone1','Zone2','Zone3','Zone4')),
  usage TEXT NOT NULL, t_int_c REAL, rh_int REAL,
  has_mechanical_ventilation INTEGER NOT NULL CHECK (has_mechanical_ventilation IN (0,1)),
  airtightness_n50 REAL,
  bb2_details_conform INTEGER NOT NULL CHECK (bb2_details_conform IN (0,1)),
  t_int_c_ieee754_bits INTEGER NOT NULL, t_int_c_ieee754_class TEXT NOT NULL,
  rh_int_ieee754_bits INTEGER NOT NULL, rh_int_ieee754_class TEXT NOT NULL,
  airtightness_n50_ieee754_bits INTEGER NOT NULL, airtightness_n50_ieee754_class TEXT NOT NULL
);
CREATE TABLE din4108_zone (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES din4108_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, floor_area_m2 REAL,
  heaviness TEXT NOT NULL, night_ventilation TEXT NOT NULL,
  floor_area_m2_ieee754_bits INTEGER NOT NULL, floor_area_m2_ieee754_class TEXT NOT NULL
);
CREATE TABLE din4108_window (
  id INTEGER PRIMARY KEY,
  zone_id INTEGER NOT NULL REFERENCES din4108_zone(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, orientation TEXT NOT NULL,
  inclination_deg REAL, area_m2 REAL, g_value REAL, shading_fc REAL,
  inclination_deg_ieee754_bits INTEGER NOT NULL, inclination_deg_ieee754_class TEXT NOT NULL,
  area_m2_ieee754_bits INTEGER NOT NULL, area_m2_ieee754_class TEXT NOT NULL,
  g_value_ieee754_bits INTEGER NOT NULL, g_value_ieee754_class TEXT NOT NULL,
  shading_fc_ieee754_bits INTEGER NOT NULL, shading_fc_ieee754_class TEXT NOT NULL
);
CREATE TABLE din4108_element (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES din4108_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, kind TEXT NOT NULL, zone_logical_id TEXT NOT NULL,
  orientation_deg REAL, inclination_deg REAL, adjacent TEXT NOT NULL,
  area_m2 REAL, delta_u_g REAL, delta_u_f REAL, delta_u_r REAL,
  orientation_deg_ieee754_bits INTEGER NOT NULL, orientation_deg_ieee754_class TEXT NOT NULL,
  inclination_deg_ieee754_bits INTEGER NOT NULL, inclination_deg_ieee754_class TEXT NOT NULL,
  area_m2_ieee754_bits INTEGER NOT NULL, area_m2_ieee754_class TEXT NOT NULL,
  delta_u_g_ieee754_bits INTEGER NOT NULL, delta_u_g_ieee754_class TEXT NOT NULL,
  delta_u_f_ieee754_bits INTEGER NOT NULL, delta_u_f_ieee754_class TEXT NOT NULL,
  delta_u_r_ieee754_bits INTEGER NOT NULL, delta_u_r_ieee754_class TEXT NOT NULL
);
CREATE TABLE din4108_layer (
  id INTEGER PRIMARY KEY,
  element_id INTEGER NOT NULL REFERENCES din4108_element(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, material_logical_id TEXT NOT NULL,
  thickness_m REAL, lambda REAL, mu REAL, density REAL,
  application_type TEXT NOT NULL, compressive_class TEXT NOT NULL,
  water_class TEXT NOT NULL, tensile_class TEXT NOT NULL, acoustic_class TEXT NOT NULL,
  thickness_m_ieee754_bits INTEGER NOT NULL, thickness_m_ieee754_class TEXT NOT NULL,
  lambda_ieee754_bits INTEGER NOT NULL, lambda_ieee754_class TEXT NOT NULL,
  mu_ieee754_bits INTEGER NOT NULL, mu_ieee754_class TEXT NOT NULL,
  density_ieee754_bits INTEGER NOT NULL, density_ieee754_class TEXT NOT NULL
);
CREATE TABLE din4108_segment (
  id INTEGER PRIMARY KEY,
  layer_id INTEGER NOT NULL REFERENCES din4108_layer(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, material_logical_id TEXT NOT NULL,
  fraction REAL, lambda REAL, mu REAL, density REAL,
  fraction_ieee754_bits INTEGER NOT NULL, fraction_ieee754_class TEXT NOT NULL,
  lambda_ieee754_bits INTEGER NOT NULL, lambda_ieee754_class TEXT NOT NULL,
  mu_ieee754_bits INTEGER NOT NULL, mu_ieee754_class TEXT NOT NULL,
  density_ieee754_bits INTEGER NOT NULL, density_ieee754_class TEXT NOT NULL
);
CREATE TABLE din4108_thermal_bridge (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES din4108_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, psi REAL, length_m REAL, bb2_type TEXT NOT NULL,
  psi_ieee754_bits INTEGER NOT NULL, psi_ieee754_class TEXT NOT NULL,
  length_m_ieee754_bits INTEGER NOT NULL, length_m_ieee754_class TEXT NOT NULL
);
