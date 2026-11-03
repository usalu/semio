"""📚️ blue-prints adapter: single Eurocode formula classes of EN 1992-1-1:2004, EN 1993-1-1:2005 and EN 1995-1-1:2004 evaluated as black boxes; SI in and out, the library works in N, mm and MPa."""

import importlib

LIBRARY = "blue-prints"
ROOT = "blueprints.codes.eurocode."
TO_LIBRARY = {"1": 1.0, "n": 1.0, "m": 1.0e3, "m2": 1.0e6, "m3": 1.0e9, "m4": 1.0e12, "pa": 1.0e-6, "nm": 1.0e3, "hz": 1.0, "im": 1.0e-3}
EC2 = "en_1992_1_1_2004.chapter_"
EC3 = "en_1993_1_1_2005.chapter_6_ultimate_limit_state.formula_"


def build(path, name, units, quantity, out_unit):
    """🧩️ A check running class `name` of `path` on the inputs named like its parameters; `units` maps parameter to SI unit, the result is `quantity` in `out_unit`."""

    def check(i):
        module = importlib.import_module(ROOT + path)
        value = float(getattr(module, name)(**{k: i[k] * TO_LIBRARY[u] for k, u in units.items()}))
        return {quantity: value / TO_LIBRARY[out_unit]}

    return check


def shear_resistance_c(i):
    """✂️ Eq. 6.2a/6.3N V_Rd,c = max((C_Rd,c k (100 rho_l f_ck)^(1/3) + k1 sigma_cp) b_w d, (v_min + k1 sigma_cp) b_w d) with k, rho_l and v_min from the library formulas."""
    module = importlib.import_module(ROOT + EC2 + "6_ultimate_limit_state.formula_6_2")
    low = importlib.import_module(ROOT + EC2 + "6_ultimate_limit_state.formula_6_3n")
    fck, d, bw = i["f_ck"] / 1e6, i["d"] * 1e3, i["b_w"] * 1e3
    k = float(module.Form6Dot2aSub1ThicknessFactor(d=d))
    rho = float(module.Form6Dot2aSub2RebarRatio(a_sl=i["A_sl"] * 1e6, b_w=bw, d=d))
    vmin = float(low.Form6Dot3nShearCapacityWithoutRebar(k=k, f_ck=fck))
    return {"V_Rd_c": float(module.Form6Dot2ShearResistance(c_rd_c=i["C_Rdc"], k=k, rho_l=rho, f_ck=fck, k_1=i["k1"], sigma_cp=i["sigma_cp"] / 1e6, b_w=bw, d=d, v_min=vmin)), "k": k, "rho_l": rho, "v_min": vmin * 1e6}


def strain_difference(i):
    """🧵️ Eq. 7.9 strain difference eps_sm - eps_cm; alpha_e = E_s / E_cm is formed from the stiffnesses."""
    module = importlib.import_module(ROOT + EC2 + "7_serviceability_limit_state.formula_7_9")
    ae = i["e_s"] / i["e_cm"]
    return {"alpha_e": ae, "epsilon_sm_minus_epsilon_cm": float(module.Form7Dot9EpsilonSmMinusEpsilonCm(sigma_s=i["sigma_s"] / 1e6, k_t=i["k_t"], f_ct_eff=i["f_ct_eff"] / 1e6, rho_p_eff=i["rho_p_eff"], e_s=i["e_s"] / 1e6, e_cm=i["e_cm"] / 1e6))}


F3 = EC3


EC2_6 = EC2 + "6_ultimate_limit_state."
EC2_8 = EC2 + "8_detailing_of_reinforcement_and_prestressing_tendons."
EC5_7 = "en_1995_1_1_2004.chapter_7_serviceability_limit_states."
TIMBER = "blueprints.codes.eurocode.en_338_2016.chapter_5_classification_of_structural_timber."
TO_LIBRARY |= {"deg": 1.0, "kgm3": 1.0}


def timber_class(i):
    """🪵️ EN 338:2016 Table 1 (softwood C), Table 2 (softwood T) and Table 3 (hardwood D) characteristic values by strength class name; strengths and moduli in Pa, densities in kg/m3."""
    name = i["strength_class"]
    table, member = {"C": ("table_1", "Table1StrengthClassesSoftwoodBending"), "T": ("table_2", "Table2StrengthClassesSoftwoodTension"), "D": ("table_3", "Table3StrengthClassesHardwoodBending")}[name[0]]
    module = importlib.import_module(TIMBER + table)
    enum = next(c for n, c in vars(module).items() if isinstance(c, type) and n.endswith(("Bending", "Tension", "Class")) and not n.startswith("Table") and name in c._value2member_map_)
    row = getattr(module, member)(enum(name))
    out = {k: float(getattr(row, k)) * 1.0e6 for k in ("f_m_k", "f_t_0_k", "f_t_90_k", "f_c_0_k", "f_c_90_k", "f_v_k", "g_mean", "e_m_90_mean") if hasattr(row, k)}
    out |= {"rho_k": float(row.rho_k), "rho_mean": float(row.rho_mean)}
    out |= {k: float(getattr(row, k)) * 1.0e6 for k in ("e_m_0_mean", "e_m_0_k", "e_t_0_mean", "e_t_0_k") if hasattr(row, k)}
    return out


def anchorage_design(i):
    """⚓️ Eq. 8.4 design anchorage length l_bd = alpha_1 alpha_2 alpha_3 alpha_4 alpha_5 l_b,rqd >= l_b,min with the product alpha_2 alpha_3 alpha_5 limited to 0.7 (Eq. 8.5) from the library."""
    m4 = importlib.import_module(ROOT + EC2_8 + "formula_8_4")
    m5 = importlib.import_module(ROOT + EC2_8 + "formula_8_5")
    m6 = importlib.import_module(ROOT + EC2_8 + "formula_8_6")
    product = float(m5.Form8Dot5ProductAlphas235(alpha_2=i["alpha_2"], alpha_3=i["alpha_3"], alpha_5=i["alpha_5"]))
    lb_min = float(m6.Form8Dot6MinimumTensionAnchorage(l_b_rqd=i["l_b_rqd"] * 1e3, diameter=i["diameter"] * 1e3))
    value = float(m4.Form8Dot4DesignAnchorageLength(alpha_1=i["alpha_1"], alpha_2=i["alpha_2"], alpha_3=i["alpha_3"], alpha_4=i["alpha_4"], alpha_5=i["alpha_5"], l_b_rqd=i["l_b_rqd"] * 1e3, l_b_min=lb_min, min_product_alpha_2_3_5=product))
    return {"product_alpha_235": product, "l_b_min": lb_min / 1e3, "l_bd": value / 1e3}


CHECKS = {
    "en1992.3.15.fcd": build(EC2 + "3_materials.formula_3_15", "Form3Dot15DesignValueCompressiveStrength", {"alpha_cc": "1", "f_ck": "pa", "gamma_c": "1"}, "f_cd", "pa"),
    "en1992.3.17.sigma-c": build(EC2 + "3_materials.formula_3_17", "Form3Dot17CompressiveStressConcrete", {"f_cd": "pa", "epsilon_c": "1", "epsilon_c2": "1", "n": "1"}, "sigma_c", "pa"),
    "en1992.5.13n.lambda-lim": build(EC2 + "5_structural_analysis.formula_5_13n", "Form5Dot13nSlendernessCriterionIsolatedMembers", {"a": "1", "b": "1", "c": "1", "n_ed": "n", "a_c": "m2", "f_cd": "pa"}, "lambda_lim", "1"),
    "en1992.5.14.slenderness": build(EC2 + "5_structural_analysis.formula_5_14", "Form5Dot14SlendernessRatio", {"l_0": "m", "i": "m"}, "lambda", "1"),
    "en1992.5.34.curvature": build(EC2 + "5_structural_analysis.formula_5_34", "Form5Dot34Curvature", {"k_r": "1", "k_phi": "1", "f_yd": "pa", "e_s": "pa", "d": "m"}, "curvature", "im"),
    "en1992.6.2.shear-vrdc": shear_resistance_c,
    "en1992.6.6n.nu": build(EC2 + "6_ultimate_limit_state.formula_6_6n", "Form6Dot6nStrengthReductionFactor", {"f_ck": "pa"}, "nu", "1"),
    "en1992.6.30.t-rd-max": build(EC2 + "6_ultimate_limit_state.formula_6_30", "Form6Dot30DesignTorsionalResistanceMoment", {"nu": "1", "alpha_cw": "1", "f_cd": "pa", "a_k": "m2", "t_ef_i": "m", "theta": "1"}, "T_Rd_max", "nm"),
    "en1992.6.38.punching-stress": build(EC2 + "6_ultimate_limit_state.formula_6_38", "Form6Dot38MaxShearStress", {"beta": "1", "v_ed": "n", "u_i": "m", "d": "m"}, "v_Ed", "pa"),
    "en1992.7.9.strain-difference": strain_difference,
    "en1992.7.8.crack-width": build(EC2 + "7_serviceability_limit_state.formula_7_8", "Form7Dot8CrackWidth", {"s_r_max": "m", "epsilon_sm_minus_epsilon_cm": "1"}, "w_k", "m"),
    "en1992.7.11.sr-max": build(EC2 + "7_serviceability_limit_state.formula_7_11", "Form7Dot11MaximumCrackSpacing", {"k_3": "1", "c": "m", "k_1": "1", "k_2": "1", "k_4": "1", "diam": "m", "rho_p_eff": "1"}, "sr_max", "m"),
    "en1992.7.16.span-depth": build(EC2 + "7_serviceability_limit_state.formula_7_16_ab", "Form7Dot16abSpanDepthRatio", {"capital_k": "1", "f_ck": "pa", "rho": "1", "rho_0": "1", "rho_prime": "1"}, "l_over_d", "1"),
    "en1992.8.3.anchorage-rqd": build(EC2 + "8_detailing_of_reinforcement_and_prestressing_tendons.formula_8_3", "Form8Dot3RequiredAnchorageLength", {"diameter": "m", "sigma_sd": "pa", "f_bd": "pa"}, "l_b_rqd", "m"),
    "en1993.6.2.npl": build(F3 + "6_6", "Form6Dot6DesignPlasticResistanceGrossCrossSection", {"a": "m2", "f_y": "pa", "gamma_m0": "1"}, "N_pl_Rd", "n"),
    "en1993.6.2.mc-rd-class12": build(F3 + "6_13", "Form6Dot13MCRdClass1And2", {"w_pl": "m3", "f_y": "pa", "gamma_m0": "1"}, "M_c_Rd", "nm"),
    "en1993.6.2.vpl-rd": build(F3 + "6_18", "Form6Dot18DesignPlasticShearResistance", {"a_v": "m2", "f_y": "pa", "gamma_m0": "1"}, "V_pl_Rd", "n"),
    "en1993.6.2.av-rolled-i": build(F3 + "6_18_sub_av", "Form6Dot18SubARolledIandHSection", {"a": "m2", "b1": "m", "b2": "m", "hw": "m", "r1": "m", "r2": "m", "tf1": "m", "tf2": "m", "tw": "m", "eta": "1"}, "A_v", "m2"),
    "en1993.6.2.av-rhs-depth": build(F3 + "6_18_sub_av", "Form6Dot18SubF1RolledRectangularHollowSectionDepth", {"a": "m2", "b": "m", "h": "m"}, "A_v", "m2"),
    "en1993.6.2.av-chs": build(F3 + "6_18_sub_av", "Form6Dot18SubGCircularHollowSection", {"a": "m2"}, "A_v", "m2"),
    "en1993.6.2.av-channel": build(F3 + "6_18_sub_av", "Form6Dot18SubBRolledChannelSection", {"a": "m2", "b": "m", "tf": "m", "tw": "m", "r": "m"}, "A_v", "m2"),
    "en1993.6.2.my-v-rd": build(F3 + "6_30", "Form6Dot30ReducedPlasticResistanceMoment", {"w_pl_y": "m3", "rho": "1", "h_w": "m", "t_w": "m", "f_y": "pa", "gamma_m0": "1", "m_y_c_rd": "nm"}, "M_y_V_Rd", "nm"),
    "en1993.6.2.my-n-rd": build(F3 + "6_36", "Form6Dot36MomentReduction", {"mpl_y_rd": "nm", "a": "1", "n": "1"}, "M_N_y_Rd", "nm"),
    "en1993.6.3.mb-rd": build(F3 + "6_55", "Form6Dot55DesignBucklingResistanceMoment", {"chi_lt": "1", "w_y": "m3", "f_y": "pa", "gamma_m1": "1"}, "M_b_Rd", "nm"),
    "en1993.5.2.alpha-cr": build("en_1993_1_1_2005.chapter_5_structural_analysis.formula_5_2", "Form5Dot2ElasticCriticalBucklingFactor", {"h_ed": "n", "v_ed": "n", "h": "m", "delta_h_ed": "m"}, "alpha_cr", "1"),
    "en1995.7.5.natural-frequency": build("en_1995_1_1_2004.chapter_7_serviceability_limit_states.formula_7_5", "Form7Dot5NaturalFrequency", {"length": "1", "ei_l": "1", "m": "1"}, "f_1", "hz"),
    "en1992.3.6.creep-deformation": build(EC2 + "3_materials.formula_3_6", "Form3Dot6CreepDeformationOfConcrete", {"phi_inf_t0": "1", "sigma_c": "pa", "e_c": "pa"}, "epsilon_cc", "1"),
    "en1992.3.21.eta": build(EC2 + "3_materials.formula_3_21_22", "Form3Dot21And22EffectiveStrength", {"f_ck": "pa"}, "eta", "1"),
    "en1992.3.19.lambda": build(EC2 + "3_materials.formula_3_19_20", "Form3Dot19And20EffectivePressureZoneHeight", {"f_ck": "pa"}, "lambda", "1"),
    "en1992.4.1.c-nom": build(EC2 + "4_durability_and_cover.formula_4_1", "Form4Dot1NominalConcreteCover", {"c_min": "m", "delta_c_dev": "m"}, "c_nom", "m"),
    "en1992.4.2.c-min": build(EC2 + "4_durability_and_cover.formula_4_2", "Form4Dot2MinimumConcreteCover", {"c_min_b": "m", "c_min_dur": "m", "delta_c_dur_gamma": "m", "delta_c_dur_st": "m", "delta_c_dur_add": "m"}, "c_min", "m"),
    "en1992.6.8.vrds": build(EC2_6 + "formula_6_8", "Form6Dot8ShearResistance", {"a_sw": "m2", "s": "m", "z": "m", "f_ywd": "pa", "theta": "deg"}, "V_Rd_s", "n"),
    "en1992.6.9.vrdmax": build(EC2_6 + "formula_6_9", "Form6Dot9MaximumShearResistance", {"b_w": "m", "z": "m", "f_cd": "pa", "nu_1": "1", "alpha_cw": "1", "theta": "deg"}, "V_Rd_max", "n"),
    "en1992.6.10.nu1": build(EC2_6 + "formula_6_10abn", "Form6Dot10abnStrengthReductionFactor", {"f_ck": "pa"}, "nu_1", "1"),
    "en1992.6.11.alpha-cw": build(EC2_6 + "formula_6_11abcn", "Form6Dot11abcnCompressionChordCoefficient", {"sigma_cp": "pa", "f_cd": "pa"}, "alpha_cw", "1"),
    "en1992.6.26.tau-wall": build(EC2_6 + "formula_6_26", "Form6Dot26ShearStressInWall", {"t_ed": "nm", "a_k": "m2"}, "tau_t_t_ef", "im"),
    "en1992.6.28.asl-torsion": build(EC2_6 + "formula_6_28", "Form6Dot28RequiredCrossSectionalArea", {"u_k": "m", "f_yd": "pa", "t_ed": "nm", "a_k": "m2", "theta": "deg"}, "sum_A_sl", "m2"),
    "en1992.6.47.punching-vrdc": build(EC2_6 + "formula_6_47", "Form6Dot47PunchingShearResistance", {"c_rd_c": "1", "k": "1", "rho_l": "1", "f_ck": "pa", "k_1": "1", "sigma_cp": "pa", "v_min": "pa"}, "v_Rd_c", "pa"),
    "en1992.6.51.punching-eccentric": build(EC2_6 + "formula_6_51", "Form6Dot51AppliedPunchingShearStressEccentricLoading", {"v_ed_red": "n", "u": "m", "d": "m", "k": "1", "m_ed": "nm", "w": "m2"}, "v_Ed", "pa"),
    "en1992.6.43.beta-rectangular": build(EC2_6 + "formula_6_43", "Form6Dot43BetaRectangular", {"ey": "m", "ez": "m", "by": "m", "bz": "m"}, "beta", "1"),
    "en1992.6.39.beta-moment": build(EC2_6 + "formula_6_39", "Form6Dot39BetaCoefficient", {"k": "1", "m_ed": "nm", "v_ed": "n", "u_1": "m", "w_1": "m2"}, "beta", "1"),
    "en1992.8.2.fbd": build(EC2_8 + "formula_8_2", "Form8Dot2UltimateBondStress", {"eta_1": "1", "eta_2": "1", "f_ctd": "pa"}, "f_bd", "pa"),
    "en1992.8.4.anchorage-design": anchorage_design,
    "en1992.8.10.lap-length": build(EC2_8 + "formula_8_10", "Form8Dot10DesignLapLength", {"alpha_1": "1", "alpha_2": "1", "alpha_3": "1", "alpha_5": "1", "alpha_6": "1", "l_b_rqd": "m", "l_0_min": "m"}, "l_0", "m"),
    "en1992.8.14.bundle-diameter": build(EC2_8 + "formula_8_14", "Form8Dot14EquivalentDiameterBundledBars", {"diameter": "m", "n_b": "1"}, "diameter_n", "m"),
    "en1993.6.7.nu-rd": build(F3 + "6_7", "Form6Dot7DesignUltimateResistanceNetCrossSection", {"a_net": "m2", "f_u": "pa", "gamma_m2": "1"}, "N_u_Rd", "n"),
    "en1993.6.10.nc-rd": build(F3 + "6_10", "Form6Dot10NcRdClass1And2And3", {"a": "m2", "f_y": "pa", "gamma_m0": "1"}, "N_c_Rd", "n"),
    "en1993.6.14.mc-rd-class3": build(F3 + "6_14", "Form6Dot14MCRdClass3", {"w_el_min": "m3", "f_y": "pa", "gamma_m0": "1"}, "M_c_Rd", "nm"),
    "en1993.6.32.mnrd-rectangular": build(F3 + "6_32", "Form6Dot32MNrdRectangular", {"m_pl_rd": "nm", "n_ed": "n", "n_pl_rd": "n"}, "M_N_Rd", "nm"),
    "en1993.6.39.mnyrd-hollow": build(F3 + "6_39", "Form6Dot39ReducedBendingMomentResistance", {"mpl_y_rd": "nm", "n": "1", "a_w": "1"}, "M_N_y_Rd", "nm"),
    "en1993.6.39.aw-hollow": build(F3 + "6_39aw", "Form6Dot39awHollowSections", {"a": "m2", "b": "m", "t": "m"}, "a_w", "1"),
    "en1993.6.21.tau-i": build(F3 + "6_21", "Form6Dot21ShearStressIOrHSection", {"v_ed": "n", "a_w": "m2", "a_f": "m2"}, "tau_Ed", "pa"),
    "en1995.338.strength-class": timber_class,
}
