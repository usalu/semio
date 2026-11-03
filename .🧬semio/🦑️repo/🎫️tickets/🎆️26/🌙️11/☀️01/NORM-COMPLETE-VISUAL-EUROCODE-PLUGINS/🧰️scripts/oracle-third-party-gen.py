"""🧪️ Generates the third-party oracle case files for the Eurocode artifacts with placeholder expectations; run the harness with `--bless` afterwards (reference values then come from the libraries, never from the Rust).

Usage: /home/deck/semio/.venv/bin/python -I oracle-third-party-gen.py
"""

import json
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
ART = ROOT / "🗿️artifacts"
LEAF = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"
DIRS = {"en1990": "⚖️en1990", "en1992": "🏛️en1992", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1997": "🌍️en1997"}
SCHEMA = ROOT / "🔮️oracles/🏃️harness/🧬️schema/🔣️.json"
SC, CP, SP, BP, GH, SY, PN, AN, NU = ("norm-eurocode-structuralcodes", "structuralcodes"), ("norm-eurocode-concreteproperties", "concreteproperties"), ("norm-eurocode-sectionproperties", "sectionproperties"), ("norm-eurocode-blue-prints", "blueprints"), ("norm-eurocode-groundhog", "groundhog"), ("norm-eurocode-sympy", "sympy"), ("norm-eurocode-pynite", "pynite"), ("norm-eurocode-anastruct", "anastruct"), ("norm-eurocode-scipy-numerics", "numerical")
EQ, NUM, SPREAD = "eurocode-quantities-v1", "numerical-v1", "method-spread-v1"


def ref(lib, kind="third-party-library", check=None):
    """🔗️ A reference to adapter `lib`; `check` overrides the case check id."""
    out = {"oracle": lib[0], "adapter": lib[1], "kind": kind}
    if check:
        out["check"] = check
    return out


def case(cid, check, clause, standard, edition, inputs, expected, refs, table=None, annex=None):
    """🧾️ One case with placeholder expectations `{name: (unit, profile)}`."""
    citation = {"standard": standard, "edition": edition, "clause": clause}
    if table:
        citation["table"] = table
    if annex:
        citation["annex"] = annex
    return {"id": cid, "check": check, "citation": citation, "inputs": inputs, "expected": {k: {"value": 0, "unit": u, "profile": p} for k, (u, p) in expected.items()}, "references": refs}


def write(artifact, name, cases):
    """💾️ Write `name`.json for `artifact` with a relative `$schema`."""
    folder = ART / DIRS[artifact] / LEAF
    folder.mkdir(parents=True, exist_ok=True)
    rel = Path(*([".."] * len(folder.relative_to(ROOT).parts))) / SCHEMA.relative_to(ROOT)
    document = {"$schema": str(rel), "schemaVersion": 1, "artifact": artifact, "annex": "de", "cases": cases}
    (folder / f"{name}.json").write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


E1992, E1993, E1997 = "EN 1992-1-1", "EN 1993-1-1", "EN 1997-1"
BEAM = {"fck": 30e6, "fyk": 500e6, "b": 0.3, "h": 0.5, "d": 0.45, "As": 0.0012, "gamma_c": 1.5, "gamma_s": 1.15, "alpha_cc": 0.85}
LAWS = {k: ("Pa", EQ) for k in ("fcm", "fctm", "fctk_5", "fctk_95", "Ecm", "fcd")} | {k: ("1", EQ) for k in ("eps_c1", "eps_cu1", "eps_c2", "eps_cu2", "n")}


def en1992():
    cases = []
    for fck in (30e6, 60e6, 90e6):
        cases.append(case(f"en1992.3.1.laws-c{int(fck / 1e6)}", "en1992.3.1.concrete-laws", "3.1.3 Table 3.1; 3.1.7; 3.15", E1992, "2004", {"fck": fck, "alpha_cc": 0.85, "gamma_c": 1.5}, LAWS, [ref(SC)], table="3.1"))
    cases.append(case("en1992.3.15.fcd-c35-de", "en1992.3.15.fcd", "3.1.6 Eq. 3.15", E1992, "2004", {"f_ck": 35e6, "alpha_cc": 0.85, "gamma_c": 1.5}, {"f_cd": ("Pa", EQ)}, [ref(SC), ref(BP)], annex="de"))
    flex = {"M_Rd": ("N*m", NUM), "x": ("m", NUM)}
    cases.append(case("en1992.6.1.flexure-c30-as1200", "en1992.6.1.flexure-rect", "6.1; 3.1.7(1)", E1992, "2004", BEAM, flex, [ref(SC), ref(CP), ref(NU, "independent-formulation")], annex="de"))
    cases.append(case("en1992.6.1.flexure-c30-as3500-overreinforced", "en1992.6.1.flexure-rect", "6.1; 3.1.7(1); 3.2.7", E1992, "2004", BEAM | {"As": 0.0035}, {"M_Rd": ("N*m", NUM)}, [ref(SC), ref(CP)], annex="de"))
    hard = {"k_hardening": 1.08, "eps_uk": 0.05}
    cases.append(case("en1992.6.1.flexure-c30-as1200-hardening", "en1992.6.1.flexure-rect", "6.1; 3.2.7 Figure 3.8 inclined branch k = 1.08", E1992, "2004", BEAM | hard, {"M_Rd": ("N*m", NUM)}, [ref(SC), ref(CP)], annex="de"))
    cases.append(case("en1992.6.1.flexure-c30-as3500-hardening", "en1992.6.1.flexure-rect", "6.1; 3.2.7 Figure 3.8 inclined branch k = 1.08", E1992, "2004", BEAM | hard | {"As": 0.0035}, {"M_Rd": ("N*m", NUM)}, [ref(SC), ref(CP)], annex="de"))
    cases.append(case("en1992.6.1.nm-interaction-c30-hardening", "en1992.6.1.nm-interaction", "6.1; Figure 6.1; 3.2.7 k = 1.08", E1992, "2004", BEAM | hard | {"N_Ed": [0.0, 500e3, 1000e3, 2000e3, -200e3]}, {f"M_Rd_{k}": ("N*m", NUM) for k in range(5)}, [ref(SC), ref(CP)], annex="de"))
    cases.append(case("en1992.6.1.flexure-c60-as2000", "en1992.6.1.flexure-rect", "6.1; 3.1.7(1); Table 3.1", E1992, "2004", BEAM | {"fck": 60e6, "As": 0.002}, {"M_Rd": ("N*m", NUM)}, [ref(SC), ref(CP)], annex="de"))
    cases.append(case("en1992.6.1.nm-interaction-c30", "en1992.6.1.nm-interaction", "6.1; Figure 6.1", E1992, "2004", BEAM | {"N_Ed": [0.0, 500e3, 1000e3, 2000e3, -200e3]}, {f"M_Rd_{k}": ("N*m", NUM) for k in range(5)}, [ref(SC), ref(CP)], annex="de"))
    cases.append(case("en1992.7.2.cracked-c30", "en1992.7.2.cracked-section", "7.1; 7.2(2)", E1992, "2004", BEAM | {"M_Ed": 100e3}, {"x_cr": ("m", NUM), "I_cr": ("m^4", NUM), "M_cr": ("N*m", NUM), "sigma_c": ("Pa", NUM), "sigma_s": ("Pa", NUM)}, [ref(CP)], annex="de"))
    for tag, extra in (("en", {"C_Rdc": 0.12, "k1": 0.15}), ("de", {"C_Rdc": 0.1, "k1": 0.12})):
        for name, over in (("base", {}), ("normal-force", {"sigma_cp": 1.5e6}), ("thin-slab", {"d": 0.15, "b_w": 1.0, "A_sl": 0.0015}), ("rho-capped", {"A_sl": 0.004})):
            inputs = {"f_ck": 30e6, "d": 0.45, "b_w": 0.3, "A_sl": 0.0012, "sigma_cp": 0.0, "alpha_cc": 0.85, "gamma_c": 1.5} | extra | over
            cases.append(case(f"en1992.6.2.vrdc-{name}-{tag}-constants", "en1992.6.2.shear-vrdc", "6.2.2(1) Eq. 6.2a, 6.2b, 6.3N", E1992, "2004", inputs, {"V_Rd_c": ("N", EQ)}, [ref(SC), ref(BP)], annex=tag))
    for cot in (1.0, 2.194, 2.5):
        cases.append(case(f"en1992.6.2.links-cot{cot}", "en1992.6.2.shear-links", "6.2.3 Eq. 6.8, 6.9; 6.2.2 Eq. 6.5", E1992, "2004", {"f_ck": 30e6, "b": 0.3, "z": 0.405, "d": 0.45, "Ac": 0.15, "cot_theta": cot, "Asw": 0.000157, "s": 0.2, "fyk": 500e6, "gamma_s": 1.15, "alpha_cc": 1.0, "gamma_c": 1.5}, {"V_Rd_max": ("N", EQ), "V_Rd_s": ("N", EQ), "V_Ed_max_unreinforced": ("N", EQ)}, [ref(SC)], annex="en"))
    strain = {"sigma_s": 250e6, "e_s": 200e9, "e_cm": 32.8e9, "rho_p_eff": 0.02, "k_t": 0.4, "f_ct_eff": 2.9e6}
    cases.append(case("en1992.7.9.strain-difference", "en1992.7.9.strain-difference", "7.3.4 Eq. 7.9", E1992, "2004", strain, {"epsilon_sm_minus_epsilon_cm": ("1", EQ), "alpha_e": ("1", EQ)}, [ref(SC), ref(BP)]))
    cases.append(case("en1992.7.11.sr-max", "en1992.7.11.sr-max", "7.3.4 Eq. 7.11", E1992, "2004", {"c": 0.03, "diam": 0.016, "rho_p_eff": 0.02, "k_1": 0.8, "k_2": 0.5, "k_3": 3.4, "k_4": 0.425}, {"sr_max": ("m", EQ)}, [ref(SC), ref(BP)]))
    cases.append(case("en1992.7.8.crack-width", "en1992.7.8.crack-width", "7.3.4 Eq. 7.8", E1992, "2004", {"s_r_max": 0.238, "epsilon_sm_minus_epsilon_cm": 9.246e-4}, {"w_k": ("m", EQ)}, [ref(SC), ref(BP)]))
    single = [
        ("6.6n.nu", "en1992.6.6n.nu", "6.2.2 Eq. 6.6N", {"f_ck": 30e6}, {"nu": ("1", EQ)}),
        ("6.30.t-rd-max", "en1992.6.30.t-rd-max", "6.3.2 Eq. 6.30", {"nu": 0.528, "alpha_cw": 1.0, "f_cd": 20e6, "a_k": 0.083789, "t_ef_i": 0.09375, "theta": 45.0}, {"T_Rd_max": ("N*m", EQ)}),
        ("6.38.punching-stress", "en1992.6.38.punching-stress", "6.4.3 Eq. 6.38", {"beta": 1.15, "v_ed": 500e3, "u_i": 3.2, "d": 0.25}, {"v_Ed": ("Pa", EQ)}),
        ("5.13n.lambda-lim", "en1992.5.13n.lambda-lim", "5.8.3.1 Eq. 5.13N", {"a": 0.7, "b": 1.1, "c": 0.7, "n_ed": 1.2e6, "a_c": 0.16, "f_cd": 17e6}, {"lambda_lim": ("1", EQ)}),
        ("5.14.slenderness", "en1992.5.14.slenderness", "5.8.3.2 Eq. 5.14", {"l_0": 4.2, "i": 0.115}, {"lambda": ("1", EQ)}),
        ("5.34.curvature", "en1992.5.34.curvature", "5.8.8.3 Eq. 5.34", {"k_r": 1.0, "k_phi": 1.2, "f_yd": 434.78e6, "e_s": 200e9, "d": 0.35}, {"curvature": ("rad/m", EQ)}),
        ("7.16.span-depth", "en1992.7.16.span-depth", "7.4.2 Eq. 7.16a", {"capital_k": 1.3, "f_ck": 30e6, "rho": 0.005, "rho_0": 0.005477, "rho_prime": 0.0}, {"l_over_d": ("1", EQ)}),
        ("8.3.anchorage-rqd", "en1992.8.3.anchorage-rqd", "8.4.3 Eq. 8.3", {"diameter": 0.016, "sigma_sd": 434.78e6, "f_bd": 2.7e6}, {"l_b_rqd": ("m", EQ)}),
    ]
    for cid, check, clause, inputs, expected in single:
        cases.append(case(f"en1992.{cid}", check, clause, E1992, "2004", inputs, expected, [ref(BP)]))
    cases.append(case("en1992.7.16.span-depth-compression", "en1992.7.16.span-depth", "7.4.2 Eq. 7.16b", E1992, "2004", {"capital_k": 1.3, "f_ck": 30e6, "rho": 0.02, "rho_0": 0.005477, "rho_prime": 0.005}, {"l_over_d": ("1", EQ)}, [ref(BP)]))
    write("en1992", "third-party", cases)
    beam = {"E": 33e9, "A": 0.15, "I": 0.3 * 0.5**3 / 12}
    write("en1992", "third-party-mechanics", [case("en1992.mechanics.propped-cantilever-udl", "mechanics.beam-continuous", "5.1.3 load arrangement; linear elastic analysis 5.4", E1992, "2004", beam | {"spans": [6.0], "w": [10e3], "fixed_start": True}, {"R_0": ("N", NUM), "R_1": ("N", NUM), "M_hog_0": ("N*m", NUM), "M_sag_0": ("N*m", NUM), "delta_0": ("m", NUM)}, [ref(PN), ref(AN)])])


I_SECTIONS = {"heb240": (0.24, 0.24, 0.017, 0.010, 0.021), "heb260": (0.26, 0.26, 0.0175, 0.010, 0.024), "heb280": (0.28, 0.28, 0.018, 0.0105, 0.024), "heb300": (0.30, 0.30, 0.019, 0.011, 0.027), "ipe300": (0.30, 0.15, 0.0107, 0.0071, 0.015)}
SECTION = ("m^2", NUM), ("m^4", NUM)


def en1993():
    cases = []
    gross = lambda names: {n: (u, NUM) for n, u in names}
    common = [("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("It", "m^4"), ("Wel_y", "m^3"), ("Wel_z", "m^3"), ("Wpl_y", "m^3"), ("Wpl_z", "m^3")]
    for name, (h, b, tf, tw, r) in I_SECTIONS.items():
        cases.append(case(f"en1993.section.{name}", "en1993.section.i-rolled", "5.5 section classification inputs; 6.2.5; 6.3.2.2 section constants", E1993, "2005", {"h": h, "b": b, "tf": tf, "tw": tw, "r": r}, gross(common + [("Iw", "m^6")]), [ref(SP)]))
    cases.append(case("en1993.section.rhs-200x100x8", "en1993.section.rhs", "6.2 section constants", E1993, "2005", {"h": 0.2, "b": 0.1, "t": 0.008, "r_out": 0.016}, gross(common), [ref(SP)]))
    cases.append(case("en1993.section.chs-139.7x5", "en1993.section.chs", "6.2 section constants", E1993, "2005", {"d": 0.1397, "t": 0.005}, gross(common), [ref(SP), ref(NU, "independent-formulation")]))
    cases.append(case("en1993.section.channel-pfc200", "en1993.section.channel", "6.2 section constants", E1993, "2005", {"h": 0.2, "b": 0.075, "tf": 0.0115, "tw": 0.0085, "r": 0.0115}, gross(common[:4] + common[6:] + [("Iw", "m^6"), ("y_sc", "m")]), [ref(SP)]))
    cases.append(case("en1993.section.angle-l100x10", "en1993.section.angle", "6.2 section constants", E1993, "2005", {"h": 0.1, "b": 0.1, "t": 0.01, "r_root": 0.012, "r_toe": 0.006}, gross([("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("I_major", "m^4"), ("I_minor", "m^4"), ("It", "m^4"), ("Wpl_y", "m^3")]), [ref(SP)]))
    for c1 in (1.0, 1.127):
        cases.append(case(f"en1993.6.3.2.mcr-ipe300-l6-c1-{c1}", "en1993.6.3.2.mcr-i", "6.3.2.2; NCCI SN003 Eq. (1)", E1993, "2005", {"h": 0.3, "b": 0.15, "tf": 0.0107, "tw": 0.0071, "r": 0.015, "E": 210e9, "G": 81e9, "L": 6.0, "C1": c1}, {"M_cr": ("N*m", NUM), "Iz": ("m^4", NUM), "It": ("m^4", NUM), "Iw": ("m^6", NUM)}, [ref(SP)]))
    heb = {"a": 0.0106, "b1": 0.24, "b2": 0.24, "hw": 0.206, "r1": 0.021, "r2": 0.021, "tf1": 0.017, "tf2": 0.017, "tw": 0.010, "eta": 1.0}
    single = [
        ("6.2.npl-heb240", "en1993.6.2.npl", "6.2.4 Eq. 6.6", {"a": 0.0106, "f_y": 355e6, "gamma_m0": 1.0}, {"N_pl_Rd": ("N", EQ)}),
        ("6.2.mc-rd-heb240", "en1993.6.2.mc-rd-class12", "6.2.5 Eq. 6.13", {"w_pl": 1.053e-3, "f_y": 355e6, "gamma_m0": 1.0}, {"M_c_Rd": ("N*m", EQ)}),
        ("6.2.vpl-rd-heb240", "en1993.6.2.vpl-rd", "6.2.6 Eq. 6.18", {"a_v": 3.324e-3, "f_y": 355e6, "gamma_m0": 1.0}, {"V_pl_Rd": ("N", EQ)}),
        ("6.2.av-rolled-i-heb240", "en1993.6.2.av-rolled-i", "6.2.6(3)a", heb, {"A_v": ("m^2", EQ)}),
        ("6.2.av-rhs-200x100x8", "en1993.6.2.av-rhs-depth", "6.2.6(3)f", {"a": 0.0043781, "b": 0.1, "h": 0.2}, {"A_v": ("m^2", EQ)}),
        ("6.2.av-chs-139.7x5", "en1993.6.2.av-chs", "6.2.6(3)g", {"a": 0.002115}, {"A_v": ("m^2", EQ)}),
        ("6.2.av-channel-pfc200", "en1993.6.2.av-channel", "6.2.6(3)b", {"a": 0.0032866, "b": 0.075, "tf": 0.0115, "tw": 0.0085, "r": 0.0115}, {"A_v": ("m^2", EQ)}),
        ("6.2.my-v-rd-heb240", "en1993.6.2.my-v-rd", "6.2.8 Eq. 6.30", {"w_pl_y": 1.053e-3, "rho": 0.3, "h_w": 0.206, "t_w": 0.010, "f_y": 355e6, "gamma_m0": 1.0, "m_y_c_rd": 373.8e3}, {"M_y_V_Rd": ("N*m", EQ)}),
        ("6.2.my-n-rd-heb240", "en1993.6.2.my-n-rd", "6.2.9.1 Eq. 6.36", {"mpl_y_rd": 373.8e3, "a": 0.3, "n": 0.2}, {"M_N_y_Rd": ("N*m", EQ)}),
        ("6.3.mb-rd-heb240", "en1993.6.3.mb-rd", "6.3.2.1 Eq. 6.55", {"chi_lt": 0.8, "w_y": 1.053e-3, "f_y": 355e6, "gamma_m1": 1.0}, {"M_b_Rd": ("N*m", EQ)}),
        ("5.2.alpha-cr", "en1993.5.2.alpha-cr", "5.2.1 Eq. 5.2", {"h_ed": 50e3, "v_ed": 1500e3, "h": 4.0, "delta_h_ed": 0.012}, {"alpha_cr": ("1", EQ)}),
    ]
    for cid, check, clause, inputs, expected in single:
        cases.append(case(f"en1993.{cid}", check, clause, E1993, "2005", inputs, expected, [ref(BP)]))
    write("en1993", "third-party", cases)
    frame = {"E": 210e9, "A": 5.38e-3, "I_col": 8.356e-5, "I_beam": 1.6712e-4, "h": 4.0, "span": 6.0, "H": 10e3}
    write("en1993", "third-party-mechanics", [
        case("en1993.mechanics.portal-sway", "mechanics.portal-sway", "5.2 global analysis; 5.3.2 imperfections (first order sway)", E1993, "2005", frame, {"sway": ("m", NUM), "M_base_left": ("N*m", NUM), "M_base_right": ("N*m", NUM), "H_base_left": ("N", NUM)}, [ref(PN), ref(AN)]),
        case("en1993.mechanics.ss-beam-udl", "mechanics.beam-continuous", "7.2 deflections; linear elastic analysis", E1993, "2005", {"E": 210e9, "A": 5.38e-3, "I": 8.356e-5, "spans": [6.0], "w": [10e3]}, {"R_0": ("N", NUM), "M_sag_0": ("N*m", NUM), "delta_0": ("m", NUM)}, [ref(PN), ref(AN)]),
    ])


def en1997():
    cases = []
    for phi in (20.0, 30.0, 35.0):
        cases.append(case(f"en1997.D.4.bearing-factors-phi{int(phi)}", "en1997.D.4.bearing-factors", "Annex D.4 (D.2-D.4)", E1997, "2004", {"phi": phi}, {"Nq": ("1", EQ), "Nc": ("1", EQ), "Ng_ec7": ("1", EQ)}, [ref(GH)]))
    for phi, delta in ((30.0, 0.0), (30.0, 20.0), (35.0, 23.3333)):
        cases.append(case(f"en1997.9.earth-pressure-phi{int(phi)}-delta{int(delta)}", "en1997.9.earth-pressure", "9.5 and Annex C (limit earth pressure coefficients)", E1997, "2004", {"phi": phi, "delta": delta}, ({"Ka_rankine": ("1", EQ), "Kp_rankine": ("1", EQ)} | ({"Ka_coulomb": ("1", EQ), "Kp_coulomb": ("1", SPREAD)} if delta else {})), [ref(GH)], table="C"))
    for qc, sv in ((10e6, 100e3), (5e6, 50e3), (20e6, 200e3)):
        cases.append(case(f"en1997.2.cpt-phi-qc{int(qc / 1e6)}-sv{int(sv / 1e3)}", "en1997.2.cpt-friction-angle", "2.4.3 and 10.2 derived values from field tests (Kulhawy and Mayne 1990)", E1997, "2004", {"qc": qc, "sigma_v0_eff": sv}, {"phi_deg": ("deg", EQ)}, [ref(GH)]))
    cases.append(case("en1997.2.cpt-su-nk15", "en1997.2.cpt-undrained-strength", "2.4.3 derived values from field tests (Rad and Lunne 1988)", E1997, "2004", {"q_net": 1.5e6, "N_k": 15.0}, {"s_u": ("Pa", EQ)}, [ref(GH)]))
    cases.append(case("en1997.2.cpt-behaviour-index", "en1997.2.cpt-behaviour-index", "2.4.3 derived values from field tests (Robertson and Wride 1998)", E1997, "2004", {"qt": 10e6, "fs": 100e3, "sigma_v0": 150e3, "sigma_v0_eff": 100e3}, {"Ic": ("1", SPREAD)}, [ref(GH)]))
    write("en1997", "third-party", cases)


def mechanics():
    two = {"E": 210e9, "A": 5.38e-3, "I": 8.356e-5, "spans": [6.0, 6.0]}
    support = {"R_0": ("N", NUM), "R_1": ("N", NUM), "R_2": ("N", NUM), "M_sag_0": ("N*m", NUM), "M_hog_0": ("N*m", NUM)}
    for art, std, ed, E, A, I in (("en1994", "EN 1994-1-1", "2004", 210e9, 5.38e-3, 8.356e-5), ("en1995", "EN 1995-1-1", "2004", 11e9, 0.03, 0.1 * 0.3**3 / 12)):
        base = two | {"E": E, "A": A, "I": I}
        cases = [
            case(f"{art}.mechanics.two-span-udl", "mechanics.beam-continuous", "5.1.3 load arrangements; 5.4 linear elastic analysis", std, ed, base | {"w": [10e3, 10e3]}, support | {"delta_0": ("m", NUM)}, [ref(PN), ref(AN)]),
            case(f"{art}.mechanics.two-span-pattern-first-span", "mechanics.beam-continuous", "5.1.3 load arrangements (alternate spans)", std, ed, base | {"w": [10e3, 0.0]}, support, [ref(PN), ref(AN)]),
            case(f"{art}.mechanics.two-span-midspan-forces", "mechanics.beam-continuous", "5.1.3 load arrangements (point loads at midspan)", std, ed, base | {"w": [0.0, 0.0], "P_mid": [20e3, 20e3]}, support, [ref(PN), ref(AN)]),
            case(f"{art}.mechanics.ss-beam-deflection", "mechanics.beam-continuous", "7.2 deflections", std, ed, {"E": E, "A": A, "I": I, "spans": [4.5], "w": [10e3]}, {"R_0": ("N", NUM), "M_sag_0": ("N*m", NUM), "delta_0": ("m", NUM)}, [ref(PN), ref(AN)]),
        ]
        write(art, "third-party-mechanics", cases)
    write("en1995", "third-party", [case("en1995.7.5.natural-frequency", "en1995.7.5.natural-frequency", "7.3.3 Eq. 7.5", "EN 1995-1-1", "2004", {"length": 4.5, "ei_l": 1.2e5, "m": 150.0}, {"f_1": ("Hz", EQ)}, [ref(BP)])])


def main():
    en1992()
    en1993()
    en1997()
    mechanics()


main()
