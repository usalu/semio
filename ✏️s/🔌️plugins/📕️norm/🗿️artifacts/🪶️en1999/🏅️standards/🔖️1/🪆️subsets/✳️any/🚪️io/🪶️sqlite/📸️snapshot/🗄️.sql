CREATE TABLE en1999_document (id INTEGER PRIMARY KEY CHECK(id=1), annex TEXT NOT NULL CHECK(annex IN ('En','De')), reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3')));
CREATE TABLE en1999_material (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, designation TEXT NOT NULL, product_form TEXT NOT NULL CHECK(product_form IN ('sheet','extrudedProfile','extrudedTube','extrudedBar','drawnTube','forging')));
CREATE TABLE en1999_section (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL CHECK(kind IN ('extrudedI','channel','angle','rhs','box','chs','free')),
 height REAL, width REAL, flange_thickness REAL, web_thickness REAL, outer_diameter REAL,
 height_ieee754_bits INTEGER NOT NULL, height_ieee754_class TEXT NOT NULL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 flange_thickness_ieee754_bits INTEGER NOT NULL, flange_thickness_ieee754_class TEXT NOT NULL,
 web_thickness_ieee754_bits INTEGER NOT NULL, web_thickness_ieee754_class TEXT NOT NULL,
 outer_diameter_ieee754_bits INTEGER NOT NULL, outer_diameter_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_plate_element (
 id INTEGER PRIMARY KEY, section_id INTEGER NOT NULL REFERENCES en1999_section(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL,
 width REAL, thickness REAL, outstand INTEGER NOT NULL CHECK(outstand IN(0,1)), welded INTEGER NOT NULL CHECK(welded IN(0,1)), weld_position REAL, weld_process TEXT NOT NULL CHECK(weld_process IN ('mig','tig')), weld_t_junction INTEGER NOT NULL CHECK(weld_t_junction IN(0,1)), weld_direction TEXT NOT NULL CHECK(weld_direction IN ('longitudinal','transverse')), edge_stiffener_depth REAL, edge_stiffener_thickness REAL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL,
 weld_position_ieee754_bits INTEGER NOT NULL, weld_position_ieee754_class TEXT NOT NULL,
 edge_stiffener_depth_ieee754_bits INTEGER NOT NULL, edge_stiffener_depth_ieee754_class TEXT NOT NULL,
 edge_stiffener_thickness_ieee754_bits INTEGER NOT NULL, edge_stiffener_thickness_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_member (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, section_reference TEXT NOT NULL, material_reference TEXT NOT NULL,
 length REAL, support TEXT NOT NULL CHECK(support IN('simplySupported','continuous','cantilever')), buckling_length_y REAL, buckling_length_z REAL, buckling_length_t REAL, ltb_length REAL, c1 REAL, restrained_ltb INTEGER NOT NULL CHECK(restrained_ltb IN(0,1)), bearing_length REAL, transverse_weld INTEGER NOT NULL CHECK(transverse_weld IN(0,1)), transverse_weld_offset REAL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL,
 buckling_length_y_ieee754_bits INTEGER NOT NULL, buckling_length_y_ieee754_class TEXT NOT NULL,
 buckling_length_z_ieee754_bits INTEGER NOT NULL, buckling_length_z_ieee754_class TEXT NOT NULL,
 buckling_length_t_ieee754_bits INTEGER NOT NULL, buckling_length_t_ieee754_class TEXT NOT NULL,
 ltb_length_ieee754_bits INTEGER NOT NULL, ltb_length_ieee754_class TEXT NOT NULL,
 c1_ieee754_bits INTEGER NOT NULL, c1_ieee754_class TEXT NOT NULL,
 bearing_length_ieee754_bits INTEGER NOT NULL, bearing_length_ieee754_class TEXT NOT NULL,
 transverse_weld_offset_ieee754_bits INTEGER NOT NULL, transverse_weld_offset_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_member_action (
 id INTEGER PRIMARY KEY, member_id INTEGER NOT NULL REFERENCES en1999_member(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL, torsion_k REAL, local_load_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL,
 torsion_k_ieee754_bits INTEGER NOT NULL, torsion_k_ieee754_class TEXT NOT NULL,
 local_load_k_ieee754_bits INTEGER NOT NULL, local_load_k_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_connection (id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, member_reference TEXT NOT NULL, material_reference TEXT NOT NULL, kind TEXT NOT NULL, load_eccentricity REAL, hole_stagger REAL, force_method TEXT NOT NULL CHECK(force_method IN ('elastic','instantaneousCentre')), load_eccentricity_ieee754_bits INTEGER NOT NULL, load_eccentricity_ieee754_class TEXT NOT NULL, hole_stagger_ieee754_bits INTEGER NOT NULL, hole_stagger_ieee754_class TEXT NOT NULL);
CREATE TABLE en1999_connection_action (
 id INTEGER PRIMARY KEY, connection_id INTEGER NOT NULL REFERENCES en1999_connection(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL, torsion_k REAL, local_load_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL,
 torsion_k_ieee754_bits INTEGER NOT NULL, torsion_k_ieee754_class TEXT NOT NULL,
 local_load_k_ieee754_bits INTEGER NOT NULL, local_load_k_ieee754_class TEXT NOT NULL
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
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL, torsion_k REAL, local_load_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL,
 torsion_k_ieee754_bits INTEGER NOT NULL, torsion_k_ieee754_class TEXT NOT NULL,
 local_load_k_ieee754_bits INTEGER NOT NULL, local_load_k_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_shell (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, material_reference TEXT NOT NULL, radius REAL, thickness REAL, length REAL, welded INTEGER NOT NULL CHECK(welded IN(0,1)), fabrication_class TEXT NOT NULL CHECK(fabrication_class IN ('a','b','c')),
 radius_ieee754_bits INTEGER NOT NULL, radius_ieee754_class TEXT NOT NULL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL,
 length_ieee754_bits INTEGER NOT NULL, length_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_shell_action (
 id INTEGER PRIMARY KEY, shell_id INTEGER NOT NULL REFERENCES en1999_shell(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, n_k REAL, v_y_k REAL, v_z_k REAL, m_y_k REAL, m_z_k REAL, torsion_k REAL, local_load_k REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_y_k_ieee754_bits INTEGER NOT NULL, v_y_k_ieee754_class TEXT NOT NULL,
 v_z_k_ieee754_bits INTEGER NOT NULL, v_z_k_ieee754_class TEXT NOT NULL,
 m_y_k_ieee754_bits INTEGER NOT NULL, m_y_k_ieee754_class TEXT NOT NULL,
 m_z_k_ieee754_bits INTEGER NOT NULL, m_z_k_ieee754_class TEXT NOT NULL,
 torsion_k_ieee754_bits INTEGER NOT NULL, torsion_k_ieee754_class TEXT NOT NULL,
 local_load_k_ieee754_bits INTEGER NOT NULL, local_load_k_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_placement (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, x REAL, y REAL, z REAL, yaw REAL,
 x_ieee754_bits INTEGER NOT NULL, x_ieee754_class TEXT NOT NULL,
 y_ieee754_bits INTEGER NOT NULL, y_ieee754_class TEXT NOT NULL,
 z_ieee754_bits INTEGER NOT NULL, z_ieee754_class TEXT NOT NULL,
 yaw_ieee754_bits INTEGER NOT NULL, yaw_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_weld_run (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, member_reference TEXT NOT NULL, element_reference TEXT NOT NULL, run_start REAL, run_length REAL, across_offset REAL, throat REAL, process TEXT NOT NULL, t_junction INTEGER NOT NULL CHECK(t_junction IN(0,1)), direction TEXT NOT NULL, haz_extent REAL,
 run_start_ieee754_bits INTEGER NOT NULL, run_start_ieee754_class TEXT NOT NULL,
 run_length_ieee754_bits INTEGER NOT NULL, run_length_ieee754_class TEXT NOT NULL,
 across_offset_ieee754_bits INTEGER NOT NULL, across_offset_ieee754_class TEXT NOT NULL,
 throat_ieee754_bits INTEGER NOT NULL, throat_ieee754_class TEXT NOT NULL,
 haz_extent_ieee754_bits INTEGER NOT NULL, haz_extent_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_section_vertex (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, section_reference TEXT NOT NULL, ring TEXT NOT NULL CHECK(ring IN ('outer','hole')), element_reference TEXT NOT NULL, y REAL, z REAL,
 y_ieee754_bits INTEGER NOT NULL, y_ieee754_class TEXT NOT NULL,
 z_ieee754_bits INTEGER NOT NULL, z_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_fatigue_category (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, member_reference TEXT NOT NULL, element_reference TEXT NOT NULL, detail_category TEXT NOT NULL, detail_type TEXT NOT NULL CHECK(detail_type IN ('plain','buttWeld','filletWeld','bolted','riveted','other')), location_x REAL, m1 REAL, m2 REAL,
 location_x_ieee754_bits INTEGER NOT NULL, location_x_ieee754_class TEXT NOT NULL,
 m1_ieee754_bits INTEGER NOT NULL, m1_ieee754_class TEXT NOT NULL,
 m2_ieee754_bits INTEGER NOT NULL, m2_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_patch_load (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, subject_reference TEXT NOT NULL, position TEXT NOT NULL CHECK(position IN ('end','interior')), x REAL, bearing_length REAL, web_height REAL, web_angle REAL, corner_radius REAL, load REAL,
 x_ieee754_bits INTEGER NOT NULL, x_ieee754_class TEXT NOT NULL,
 bearing_length_ieee754_bits INTEGER NOT NULL, bearing_length_ieee754_class TEXT NOT NULL,
 web_height_ieee754_bits INTEGER NOT NULL, web_height_ieee754_class TEXT NOT NULL,
 web_angle_ieee754_bits INTEGER NOT NULL, web_angle_ieee754_class TEXT NOT NULL,
 corner_radius_ieee754_bits INTEGER NOT NULL, corner_radius_ieee754_class TEXT NOT NULL,
 load_ieee754_bits INTEGER NOT NULL, load_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_ring_stiffener (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, shell_reference TEXT NOT NULL, x REAL, depth REAL, thickness REAL, internal INTEGER NOT NULL CHECK(internal IN(0,1)),
 x_ieee754_bits INTEGER NOT NULL, x_ieee754_class TEXT NOT NULL,
 depth_ieee754_bits INTEGER NOT NULL, depth_ieee754_class TEXT NOT NULL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1999_transverse_stiffener (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1999_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, girder_reference TEXT NOT NULL, position REAL, width REAL, thickness REAL, sides TEXT NOT NULL CHECK(sides IN ('one','two')),
 position_ieee754_bits INTEGER NOT NULL, position_ieee754_class TEXT NOT NULL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 thickness_ieee754_bits INTEGER NOT NULL, thickness_ieee754_class TEXT NOT NULL
);
