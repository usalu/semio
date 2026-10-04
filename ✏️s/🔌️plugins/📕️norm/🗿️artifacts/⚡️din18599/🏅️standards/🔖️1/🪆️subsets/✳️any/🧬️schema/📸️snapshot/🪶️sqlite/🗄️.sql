CREATE TABLE din18599_document (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  building_category TEXT NOT NULL CHECK (building_category IN ('Residential','NonResidential')),
  attachment TEXT NOT NULL CHECK (attachment IN ('Detached','SemiDetached','EndTerrace','MidTerrace')),
  use_class TEXT NOT NULL CHECK (use_class IN ('Residential','Office','School')),
  method TEXT NOT NULL CHECK (method IN ('DetailedMonthly','Tabular')),
  net_floor_area_m2 REAL, heated_volume_m3 REAL, geg_qp_factor REAL, delta_u_wb_w_m2k REAL,
  automation_class TEXT NOT NULL CHECK (automation_class IN ('A','B','C','D')),
  net_floor_area_m2_ieee754_bits INTEGER NOT NULL, net_floor_area_m2_ieee754_class TEXT NOT NULL,
  heated_volume_m3_ieee754_bits INTEGER NOT NULL, heated_volume_m3_ieee754_class TEXT NOT NULL,
  geg_qp_factor_ieee754_bits INTEGER NOT NULL, geg_qp_factor_ieee754_class TEXT NOT NULL,
  delta_u_wb_w_m2k_ieee754_bits INTEGER NOT NULL, delta_u_wb_w_m2k_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_climate_child (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  child_id TEXT NOT NULL, artifact_id TEXT NOT NULL,
  artifact_kind TEXT NOT NULL, standard TEXT NOT NULL, subset TEXT NOT NULL
);
CREATE TABLE din18599_zone (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL,
  usage_profile TEXT NOT NULL CHECK (usage_profile IN ('WFH','Office','School')),
  area_m2 REAL, volume_m3 REAL, theta_i_heat_c REAL, theta_i_cool_c REAL,
  occupants INTEGER NOT NULL CHECK (occupants BETWEEN 0 AND 4294967295),
  internal_gains_w_m2 REAL, lighting_power_w_m2 REAL,
  area_m2_ieee754_bits INTEGER NOT NULL, area_m2_ieee754_class TEXT NOT NULL,
  volume_m3_ieee754_bits INTEGER NOT NULL, volume_m3_ieee754_class TEXT NOT NULL,
  theta_i_heat_c_ieee754_bits INTEGER NOT NULL, theta_i_heat_c_ieee754_class TEXT NOT NULL,
  theta_i_cool_c_ieee754_bits INTEGER NOT NULL, theta_i_cool_c_ieee754_class TEXT NOT NULL,
  internal_gains_w_m2_ieee754_bits INTEGER NOT NULL, internal_gains_w_m2_ieee754_class TEXT NOT NULL,
  lighting_power_w_m2_ieee754_bits INTEGER NOT NULL, lighting_power_w_m2_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_element (
  id INTEGER PRIMARY KEY,
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),
  logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL,
  kind TEXT NOT NULL CHECK (kind IN ('Wall','Roof','Floor','Door','Window')),
  zone_logical_id TEXT NOT NULL,
  area_m2 REAL, u_value_w_m2k REAL, orientation_deg REAL, tilt_deg REAL, g_value REAL, fc REAL,
  adjacency TEXT NOT NULL CHECK (adjacency IN ('Outdoor','Ground','Unheated','Heated')),
  area_m2_ieee754_bits INTEGER NOT NULL, area_m2_ieee754_class TEXT NOT NULL,
  u_value_w_m2k_ieee754_bits INTEGER NOT NULL, u_value_w_m2k_ieee754_class TEXT NOT NULL,
  orientation_deg_ieee754_bits INTEGER NOT NULL, orientation_deg_ieee754_class TEXT NOT NULL,
  tilt_deg_ieee754_bits INTEGER NOT NULL, tilt_deg_ieee754_class TEXT NOT NULL,
  g_value_ieee754_bits INTEGER NOT NULL, g_value_ieee754_class TEXT NOT NULL,
  fc_ieee754_bits INTEGER NOT NULL, fc_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_heating (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  generation_efficiency REAL, distribution_efficiency REAL, storage_efficiency REAL, transfer_efficiency REAL,
  energy_carrier TEXT NOT NULL,
  generation_efficiency_ieee754_bits INTEGER NOT NULL, generation_efficiency_ieee754_class TEXT NOT NULL,
  distribution_efficiency_ieee754_bits INTEGER NOT NULL, distribution_efficiency_ieee754_class TEXT NOT NULL,
  storage_efficiency_ieee754_bits INTEGER NOT NULL, storage_efficiency_ieee754_class TEXT NOT NULL,
  transfer_efficiency_ieee754_bits INTEGER NOT NULL, transfer_efficiency_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_dhw (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  specific_demand_kwh_person_a REAL, storage_loss_kwh_a REAL, distribution_loss_kwh_a REAL,
  energy_carrier TEXT NOT NULL,
  specific_demand_kwh_person_a_ieee754_bits INTEGER NOT NULL, specific_demand_kwh_person_a_ieee754_class TEXT NOT NULL,
  storage_loss_kwh_a_ieee754_bits INTEGER NOT NULL, storage_loss_kwh_a_ieee754_class TEXT NOT NULL,
  distribution_loss_kwh_a_ieee754_bits INTEGER NOT NULL, distribution_loss_kwh_a_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_ventilation (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  airflow_m3_h REAL, heat_recovery_eta REAL, fan_power_w REAL,
  airflow_m3_h_ieee754_bits INTEGER NOT NULL, airflow_m3_h_ieee754_class TEXT NOT NULL,
  heat_recovery_eta_ieee754_bits INTEGER NOT NULL, heat_recovery_eta_ieee754_class TEXT NOT NULL,
  fan_power_w_ieee754_bits INTEGER NOT NULL, fan_power_w_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_cooling (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id)
);
CREATE TABLE din18599_cooling_plant (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  cooling_id INTEGER NOT NULL REFERENCES din18599_cooling(id),
  eer REAL, energy_carrier TEXT NOT NULL,
  eer_ieee754_bits INTEGER NOT NULL, eer_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_lighting (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  control_factor REAL,
  control_factor_ieee754_bits INTEGER NOT NULL, control_factor_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_renewables (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  pv_area_m2 REAL, pv_efficiency REAL, solar_thermal_kwh_a REAL,
  pv_area_m2_ieee754_bits INTEGER NOT NULL, pv_area_m2_ieee754_class TEXT NOT NULL,
  pv_efficiency_ieee754_bits INTEGER NOT NULL, pv_efficiency_ieee754_class TEXT NOT NULL,
  solar_thermal_kwh_a_ieee754_bits INTEGER NOT NULL, solar_thermal_kwh_a_ieee754_class TEXT NOT NULL
);
CREATE TABLE din18599_climate_month (
  id INTEGER PRIMARY KEY CHECK (id BETWEEN 1 AND 12),
  document_id INTEGER NOT NULL REFERENCES din18599_document(id),
  theta_e_c REAL, g_h_w_m2 REAL,
  theta_e_c_ieee754_bits INTEGER NOT NULL, theta_e_c_ieee754_class TEXT NOT NULL,
  g_h_w_m2_ieee754_bits INTEGER NOT NULL, g_h_w_m2_ieee754_class TEXT NOT NULL
);
