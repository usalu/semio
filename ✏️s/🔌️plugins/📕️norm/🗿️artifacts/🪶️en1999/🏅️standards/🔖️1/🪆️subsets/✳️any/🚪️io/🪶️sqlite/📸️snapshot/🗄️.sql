CREATE TABLE en1999_document (id INTEGER PRIMARY KEY CHECK(id=1), annex TEXT NOT NULL CHECK(annex IN ('En','De')));
CREATE TABLE en1999_material (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, designation TEXT NOT NULL);
CREATE TABLE en1999_section (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL,
 height REAL, width REAL, flange_thickness REAL, web_thickness REAL, outer_diameter REAL,
 height_ieee754_bits INTEGER NOT NULL, height_ieee754_class TEXT NOT NULL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 flange_thickness_ieee754_bits INTEGER NOT NULL, flange_thickness_ieee754_class TEXT NOT NULL,
 web_thickness_ieee754_bits INTEGER NOT NULL, web_thickness_ieee754_class TEXT NOT NULL,
 outer_diameter_ieee754_bits INTEGER NOT NULL, outer_diameter_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_plate_element (
 id INTEGER PRIMARY KEY, section_id INTEGER NOT NULL REFERENCES en1999_section(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL,
 width REAL, thickness REAL, outstand INTEGER NOT NULL CHECK(outstand IN(0,1)), welded INTEGER NOT NULL CHECK(welded IN(0,1)), weld_position REAL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL,
 weld_position_ieee754_bits INTEGER NOT NULL, weld_position_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_member (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, section_reference TEXT NOT NULL, material_reference TEXT NOT NULL,
 length REAL, support TEXT NOT NULL CHECK(support IN('simplySupported','continuous','cantilever')), buckling_length_y REAL, buckling_length_z REAL, buckling_length_t REAL, ltb_length REAL, c1 REAL, restrained_ltb INTEGER NOT NULL CHECK(restrained_ltb IN(0,1)),
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL,
 buckling_length_y_ieee754_bits INTEGER NOT NULL, buckling_length_y_ieee754_class TEXT NOT NULL,
 buckling_length_z_ieee754_bits INTEGER NOT NULL, buckling_length_z_ieee754_class TEXT NOT NULL,
 buckling_length_t_ieee754_bits INTEGER NOT NULL, buckling_length_t_ieee754_class TEXT NOT NULL,
 ltb_length_ieee754_bits INTEGER NOT NULL, ltb_length_ieee754_class TEXT NOT NULL,
 c1_ieee754_bits INTEGER NOT NULL, c1_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_member_action (
 id INTEGER PRIMARY KEY, member_id INTEGER NOT NULL REFERENCES en1999_member(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_connection (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, member_reference TEXT NOT NULL, material_reference TEXT NOT NULL, kind TEXT NOT NULL);
CREATE TABLE en1999_connection_action (
 id INTEGER PRIMARY KEY, connection_id INTEGER NOT NULL REFERENCES en1999_connection(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_bolt_group (
 id INTEGER PRIMARY KEY REFERENCES en1999_connection(id), material TEXT NOT NULL, diameter REAL,
 rows_count INTEGER NOT NULL CHECK(rows_count BETWEEN 0 AND 4294967295), bolts_per_row INTEGER NOT NULL CHECK(bolts_per_row BETWEEN 0 AND 4294967295), edge_distance REAL, pitch REAL, gauge REAL, plate_thickness REAL,
 diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL,
 edge_distance_ieee754_bits INTEGER NOT NULL, edge_distance_ieee754_class TEXT NOT NULL,
 pitch_ieee754_bits INTEGER NOT NULL, pitch_ieee754_class TEXT NOT NULL,
 gauge_ieee754_bits INTEGER NOT NULL, gauge_ieee754_class TEXT NOT NULL,
 plate_thickness_ieee754_bits INTEGER NOT NULL, plate_thickness_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_weld_group (
 id INTEGER PRIMARY KEY REFERENCES en1999_connection(id), filler_alloy TEXT NOT NULL, throat REAL, length REAL, beta_w REAL, haz_extent REAL,
 throat_ieee754_bits INTEGER NOT NULL, throat_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL,
 beta_w_ieee754_bits INTEGER NOT NULL, beta_w_ieee754_class TEXT NOT NULL,
 haz_extent_ieee754_bits INTEGER NOT NULL, haz_extent_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_fire_scenario (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, member_reference TEXT NOT NULL, theta_a REAL, duration_s REAL,
 theta_a_ieee754_bits INTEGER NOT NULL, theta_a_ieee754_class TEXT NOT NULL,
 duration_s_ieee754_bits INTEGER NOT NULL, duration_s_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_fatigue_detail (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, member_reference TEXT NOT NULL, detail_category TEXT NOT NULL,
 delta_sigma_c REAL, delta_sigma_ed REAL, n_cycles REAL, m1 REAL, m2 REAL,
 delta_sigma_c_ieee754_bits INTEGER NOT NULL, delta_sigma_c_ieee754_class TEXT NOT NULL,
 delta_sigma_ed_ieee754_bits INTEGER NOT NULL, delta_sigma_ed_ieee754_class TEXT NOT NULL,
 n_cycles_ieee754_bits INTEGER NOT NULL, n_cycles_ieee754_class TEXT NOT NULL,
 m1_ieee754_bits INTEGER NOT NULL, m1_ieee754_class TEXT NOT NULL,
 m2_ieee754_bits INTEGER NOT NULL, m2_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_cold_formed_sheet (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, material_reference TEXT NOT NULL, thickness REAL, width REAL, span REAL, welded INTEGER NOT NULL CHECK(welded IN(0,1)),
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 span_ieee754_bits INTEGER NOT NULL, span_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_sheet_action (
 id INTEGER PRIMARY KEY, sheet_id INTEGER NOT NULL REFERENCES en1999_cold_formed_sheet(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_shell (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, material_reference TEXT NOT NULL, radius REAL, thickness REAL, length REAL,
 radius_ieee754_bits INTEGER NOT NULL, radius_ieee754_class TEXT NOT NULL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_shell_action (
 id INTEGER PRIMARY KEY, shell_id INTEGER NOT NULL REFERENCES en1999_shell(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL
);
