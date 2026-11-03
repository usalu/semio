SUMMARY = "EN 1998 kernel formulas that no snapshot entity reaches"
MISSING = "mass"
ENTRIES = [
    ("C98-01", "EN 1998-1", "4.3.3.2.2", "Lateral force base shear from the design spectrum", "Fundamentschubkraft aus dem Bemessungsspektrum", ["a_g", "s_soil", "t_b", "t_c", "t_d", "q_factor", "t_1", "mass", "lambda_c", "v_rd"], "base_shear_design", "base_shear_from_design_spectrum"),
    ("C98-02", "EN 1998-1", "3.2.2.2", "Elastic response force with damping correction", "Elastische Antwortkraft mit Dämpfungskorrektur", ["a_g", "s_soil", "t_b", "t_c", "t_d", "t_1", "mass", "xi_percent", "v_rd"], "elastic_force", "elastic_response_force"),
    ("C98-03", "EN 1998-1", "3.2.2.3", "Vertical seismic force from the vertical spectrum", "Vertikale Erdbebenkraft aus dem Vertikalspektrum", ["a_g", "avg_ratio", "t_v", "mass", "f_rd_v"], "vertical_force", "vertical_seismic_force"),
    ("C98-04", "EN 1998-1", "4.3.3.3", "Modal response spectrum analysis of a three-storey shear building", "Antwortspektrenverfahren eines dreigeschossigen Schubbalkens", ["m_1", "m_2", "m_3", "k_1", "k_2", "k_3", "a_g", "s_soil", "t_b", "t_c", "t_d", "q_factor", "xi_percent", "v_rd"], "modal_analysis", "modal_response_spectrum"),
    ("C98-05", "EN 1998-1", "3.1.2", "Ground type from the shear wave velocity of three layers", "Baugrundklasse aus der Scherwellengeschwindigkeit dreier Schichten", ["h_1", "h_2", "h_3", "vs_1", "vs_2", "vs_3", "vs_min"], "ground_velocity", "ground_type_from_layers"),
    ("C98-06", "EN 1998-5", "4.1.4", "Liquefaction safety factor of a sand layer", "Sicherheit gegen Bodenverflüssigung einer Sandschicht", ["n1_60", "fines_percent", "a_max", "sigma_v", "sigma_v_eff", "z_depth", "magnitude", "f_l_min"], "liquefaction", "liquefaction_safety"),
    ("C98-07", "EN 1998-5", "5.4.2", "Pile bending moment from inertial loading", "Pfahlbiegemoment aus Trägheitsbeanspruchung", ["e_pile", "d_pile", "l_pile", "k_h", "head_shear", "head_fixity", "z_check", "m_rd"], "pile_inertial", "pile_inertial_moment"),
    ("C98-08", "EN 1998-1", "4.4.2.5", "Diaphragm chord force at a section", "Gurtkraft einer Deckenscheibe in einem Schnitt", ["f_storey", "gamma_d", "span_dia", "depth_dia", "x_pos", "chord_area", "chord_fd"], "diaphragm_chord", "diaphragm_chord_force"),
]
