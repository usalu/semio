SUMMARY = "EN 1994 catalogue formulas that no snapshot entity reaches"
MISSING = "p_ed"
ENTRIES = [
    ("C94-01", "EN 1994-1-1", "6.6.4.2", "Headed stud resistance with profiled sheeting", "Dübeltragfähigkeit mit Profilblech", ["d_stud", "h_sc", "fu_stud", "fck", "gamma_v", "h_p", "b_0", "t_sheet", "n_r", "ribs", "p_ed"], "stud_sheeting", "stud_with_profiled_sheeting"),
    ("C94-02", "EN 1993-1-5", "6", "Patch loading of a web with bending interaction", "Lasteinleitung in den Steg mit Biegeinteraktion", ["h_w", "t_w", "b_f", "t_f", "fyw", "fyf", "s_s", "a_panel", "gamma_m1", "f_ed", "m_ed", "m_rd"], "patch_loading", "patch_loading_with_bending"),
    ("C94-03", "EN 1994-1-1", "6.7.4", "Load introduction into a composite column", "Lasteinleitung in eine Verbundstütze", ["kind", "n_intro", "perimeter", "d_min", "l_col", "p_rd"], "load_introduction", "column_load_introduction"),
    ("C94-04", "EN 1994-1-1", "7.3.1", "Deflection of a simply supported composite beam", "Durchbiegung eines Verbundträgers als Einfeldträger", ["l_span", "g_line", "q_line", "i_long", "i_short", "span_ratio"], "deflection_simple_span", "deflection_simple_span"),
    ("C94-05", "EN 1994-1-1", "7.3.1", "Deflection of a two-span composite beam", "Durchbiegung eines zweifeldrigen Verbundträgers", ["l_span", "g_line", "q_line", "i_long", "i_short", "span_ratio"], "deflection_two_span", "deflection_two_span"),
    ("C94-06", "EN 1994-1-2", "3.2", "Steel strength at elevated temperature", "Stahlfestigkeit bei erhöhter Temperatur", ["theta_a", "fy", "sigma_ed"], "steel_at_temperature", "steel_strength_in_fire"),
    ("C94-07", "EN 1994-1-1", "6.2.1.2", "Sagging plastic moment resistance with full shear connection", "Plastische Momententragfähigkeit im Feld bei vollständiger Verdübelung", ["sec_h", "sec_b", "sec_tf", "sec_tw", "sec_a", "sec_iy", "fyd", "ys", "b_eff", "h_c", "fck", "gamma_c", "m_ed"], "sagging_plastic", "sagging_plastic_moment"),
    ("C94-08", "EN 1994-1-1", "6.2.1.2", "Hogging plastic moment resistance", "Plastische Momententragfähigkeit über der Stütze", ["sec_h", "sec_b", "sec_tf", "sec_tw", "sec_a", "sec_iy", "fyd", "ys", "as_rebar", "fsd", "y_r", "m_ed"], "hogging_plastic", "hogging_plastic_moment"),
]
