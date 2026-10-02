CREATE TABLE en1992_document (
 id INTEGER PRIMARY KEY CHECK(id=1), annex TEXT NOT NULL CHECK(annex IN ('En','De')), title TEXT NOT NULL, design_working_life_years REAL, delta_c_dev REAL, cement_type TEXT NOT NULL,
 design_working_life_years_ieee754_bits INTEGER NOT NULL, design_working_life_years_ieee754_class TEXT NOT NULL,
 delta_c_dev_ieee754_bits INTEGER NOT NULL, delta_c_dev_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_concrete_grade (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1992_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, name TEXT NOT NULL, f_ck REAL,
 f_ck_ieee754_bits INTEGER NOT NULL, f_ck_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_reinforcement_grade (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1992_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, name TEXT NOT NULL, f_yk REAL, e_s REAL, ductility TEXT NOT NULL CHECK(ductility IN ('a','b','c')), k REAL, eps_uk REAL,
 f_yk_ieee754_bits INTEGER NOT NULL, f_yk_ieee754_class TEXT NOT NULL,
 e_s_ieee754_bits INTEGER NOT NULL, e_s_ieee754_class TEXT NOT NULL,
 k_ieee754_bits INTEGER NOT NULL, k_ieee754_class TEXT NOT NULL,
 eps_uk_ieee754_bits INTEGER NOT NULL, eps_uk_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_prestress_steel (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1992_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, name TEXT NOT NULL, f_pk REAL, f_p0_1k REAL,
 f_pk_ieee754_bits INTEGER NOT NULL, f_pk_ieee754_class TEXT NOT NULL,
 f_p0_1k_ieee754_bits INTEGER NOT NULL, f_p0_1k_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_member (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1992_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, label_en TEXT NOT NULL, label_de TEXT NOT NULL,
 kind TEXT NOT NULL CHECK(kind IN ('Beam','Slab','Column','Wall','FlatSlab','RibbedSlab','TensionMember','Bridge','LiquidRetaining')),
 concrete_grade_id TEXT NOT NULL, reinforcement_grade_id TEXT NOT NULL, prestress_steel_id TEXT NOT NULL,
 exposure TEXT NOT NULL CHECK(exposure IN ('X0','Xc1','Xc2','Xc3','Xc4','Xd1','Xd2','Xd3','Xs1','Xs2','Xs3','Xf1','Xf2','Xf3','Xf4','Xa1','Xa2','Xa3')),
 width REAL, height REAL, effective_depth REAL, cover REAL, span REAL,
 support TEXT NOT NULL CHECK(support IN ('SimplySupported','Continuous','Cantilever','Fixed')), buckling_length REAL,
 use_fem INTEGER NOT NULL CHECK(use_fem IN (0,1)), udl REAL, deflection_sensitive INTEGER NOT NULL CHECK(deflection_sensitive IN (0,1)), tightness TEXT CHECK(tightness IN ('Tc0','Tc1','Tc2')),
 hd_over_h REAL, liquid_sigma_s REAL, liquid_rho_p_eff REAL, liquid_f_ct_eff REAL, liquid_s_r_max REAL, bridge_sigma_c REAL, bridge_delta_sigma_s REAL,
 width_ieee754_bits INTEGER NOT NULL, width_ieee754_class TEXT NOT NULL,
 height_ieee754_bits INTEGER NOT NULL, height_ieee754_class TEXT NOT NULL,
 effective_depth_ieee754_bits INTEGER NOT NULL, effective_depth_ieee754_class TEXT NOT NULL,
 cover_ieee754_bits INTEGER NOT NULL, cover_ieee754_class TEXT NOT NULL,
 span_ieee754_bits INTEGER NOT NULL, span_ieee754_class TEXT NOT NULL,
 buckling_length_ieee754_bits INTEGER NOT NULL, buckling_length_ieee754_class TEXT NOT NULL,
 udl_ieee754_bits INTEGER NOT NULL, udl_ieee754_class TEXT NOT NULL,
 hd_over_h_ieee754_bits INTEGER NOT NULL, hd_over_h_ieee754_class TEXT NOT NULL,
 liquid_sigma_s_ieee754_bits INTEGER NOT NULL, liquid_sigma_s_ieee754_class TEXT NOT NULL,
 liquid_rho_p_eff_ieee754_bits INTEGER NOT NULL, liquid_rho_p_eff_ieee754_class TEXT NOT NULL,
 liquid_f_ct_eff_ieee754_bits INTEGER NOT NULL, liquid_f_ct_eff_ieee754_class TEXT NOT NULL,
 liquid_s_r_max_ieee754_bits INTEGER NOT NULL, liquid_s_r_max_ieee754_class TEXT NOT NULL,
 bridge_sigma_c_ieee754_bits INTEGER NOT NULL, bridge_sigma_c_ieee754_class TEXT NOT NULL,
 bridge_delta_sigma_s_ieee754_bits INTEGER NOT NULL, bridge_delta_sigma_s_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_bar_layer (
 id INTEGER PRIMARY KEY, member_id INTEGER NOT NULL REFERENCES en1992_member(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL,
 diameter REAL, count INTEGER NOT NULL CHECK(count BETWEEN 0 AND 4294967295), position TEXT NOT NULL, anchorage_length REAL, lap_length REAL, bond_condition TEXT NOT NULL, aggregate_size REAL,
 diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL,
 anchorage_length_ieee754_bits INTEGER NOT NULL, anchorage_length_ieee754_class TEXT NOT NULL,
 lap_length_ieee754_bits INTEGER NOT NULL, lap_length_ieee754_class TEXT NOT NULL,
 aggregate_size_ieee754_bits INTEGER NOT NULL, aggregate_size_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_stirrups (
 id INTEGER PRIMARY KEY REFERENCES en1992_member(id), diameter REAL, spacing REAL, legs INTEGER NOT NULL CHECK(legs BETWEEN 0 AND 4294967295),
 diameter_ieee754_bits INTEGER NOT NULL, diameter_ieee754_class TEXT NOT NULL,
 spacing_ieee754_bits INTEGER NOT NULL, spacing_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_punching (
 id INTEGER PRIMARY KEY REFERENCES en1992_member(id), column_width REAL, column_depth REAL, column_position TEXT NOT NULL, asw REAL,
 column_width_ieee754_bits INTEGER NOT NULL, column_width_ieee754_class TEXT NOT NULL,
 column_depth_ieee754_bits INTEGER NOT NULL, column_depth_ieee754_class TEXT NOT NULL,
 asw_ieee754_bits INTEGER NOT NULL, asw_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_prestress (
 id INTEGER PRIMARY KEY REFERENCES en1992_member(id), force REAL, area REAL, eccentricity REAL, loss_ratio REAL,
 force_ieee754_bits INTEGER NOT NULL, force_ieee754_class TEXT NOT NULL,
 area_ieee754_bits INTEGER NOT NULL, area_ieee754_class TEXT NOT NULL,
 eccentricity_ieee754_bits INTEGER NOT NULL, eccentricity_ieee754_class TEXT NOT NULL,
 loss_ratio_ieee754_bits INTEGER NOT NULL, loss_ratio_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_fire (
 id INTEGER PRIMARY KEY REFERENCES en1992_member(id), rating TEXT NOT NULL CHECK(rating IN ('R30','R60','R90','R120')), axis_distance REAL, column_method TEXT NOT NULL, slab_system TEXT NOT NULL,
 axis_distance_ieee754_bits INTEGER NOT NULL, axis_distance_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_member_action (
 id INTEGER PRIMARY KEY, member_id INTEGER NOT NULL REFERENCES en1992_member(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, point_force REAL, m_k REAL, n_k REAL, v_k REAL, t_k REAL, v_k_punch REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 point_force_ieee754_bits INTEGER NOT NULL, point_force_ieee754_class TEXT NOT NULL,
 m_k_ieee754_bits INTEGER NOT NULL, m_k_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_k_ieee754_bits INTEGER NOT NULL, v_k_ieee754_class TEXT NOT NULL,
 t_k_ieee754_bits INTEGER NOT NULL, t_k_ieee754_class TEXT NOT NULL,
 v_k_punch_ieee754_bits INTEGER NOT NULL, v_k_punch_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_anchor (
 id INTEGER PRIMARY KEY, document_id INTEGER NOT NULL REFERENCES en1992_document(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, h_ef REAL,
 cracked INTEGER NOT NULL CHECK(cracked IN (0,1)), f_uk REAL, f_yk REAL, a_s REAL, d REAL, c1 REAL, f_ck REAL,
 h_ef_ieee754_bits INTEGER NOT NULL, h_ef_ieee754_class TEXT NOT NULL,
 f_uk_ieee754_bits INTEGER NOT NULL, f_uk_ieee754_class TEXT NOT NULL,
 f_yk_ieee754_bits INTEGER NOT NULL, f_yk_ieee754_class TEXT NOT NULL,
 a_s_ieee754_bits INTEGER NOT NULL, a_s_ieee754_class TEXT NOT NULL,
 d_ieee754_bits INTEGER NOT NULL, d_ieee754_class TEXT NOT NULL,
 c1_ieee754_bits INTEGER NOT NULL, c1_ieee754_class TEXT NOT NULL,
 f_ck_ieee754_bits INTEGER NOT NULL, f_ck_ieee754_class TEXT NOT NULL
);
CREATE TABLE en1992_anchor_action (
 id INTEGER PRIMARY KEY, anchor_id INTEGER NOT NULL REFERENCES en1992_anchor(id), ordinal INTEGER NOT NULL CHECK(ordinal>=0), logical_id TEXT NOT NULL, kind TEXT NOT NULL, category TEXT NOT NULL, source TEXT NOT NULL,
 g_k_line REAL, q_k_line REAL, point_force REAL, m_k REAL, n_k REAL, v_k REAL, t_k REAL, v_k_punch REAL,
 g_k_line_ieee754_bits INTEGER NOT NULL, g_k_line_ieee754_class TEXT NOT NULL,
 q_k_line_ieee754_bits INTEGER NOT NULL, q_k_line_ieee754_class TEXT NOT NULL,
 point_force_ieee754_bits INTEGER NOT NULL, point_force_ieee754_class TEXT NOT NULL,
 m_k_ieee754_bits INTEGER NOT NULL, m_k_ieee754_class TEXT NOT NULL,
 n_k_ieee754_bits INTEGER NOT NULL, n_k_ieee754_class TEXT NOT NULL,
 v_k_ieee754_bits INTEGER NOT NULL, v_k_ieee754_class TEXT NOT NULL,
 t_k_ieee754_bits INTEGER NOT NULL, t_k_ieee754_class TEXT NOT NULL,
 v_k_punch_ieee754_bits INTEGER NOT NULL, v_k_punch_ieee754_class TEXT NOT NULL
);
