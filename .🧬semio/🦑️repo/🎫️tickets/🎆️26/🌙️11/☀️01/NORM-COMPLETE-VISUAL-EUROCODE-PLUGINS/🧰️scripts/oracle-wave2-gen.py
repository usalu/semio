"""🧪️ Wave-2 third-party oracle case files (`third-party-extended*.json`) for EN 1990 to EN 1999 with placeholder expectations; bless with the harness afterwards (reference values come from the libraries or from independent formulations, never from the Rust).

Usage: /home/deck/semio/.venv/bin/python -I oracle-wave2-gen.py
"""

import json
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
ART = ROOT / "🗿️artifacts"
LEAF = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"
DIRS = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1996": "🪨️en1996", "en1997": "🌍️en1997", "en1998": "🫨️en1998", "en1999": "🪶️en1999"}
SCHEMA = ROOT / "🔮️oracles/🏃️harness/🧬️schema/🔣️.json"
SC, CP, SP, BP, GH, SY, PN, AN, NU = ("norm-eurocode-structuralcodes", "structuralcodes"), ("norm-eurocode-concreteproperties", "concreteproperties"), ("norm-eurocode-sectionproperties", "sectionproperties"), ("norm-eurocode-blue-prints", "blueprints"), ("norm-eurocode-groundhog", "groundhog"), ("norm-eurocode-sympy", "sympy"), ("norm-eurocode-pynite", "pynite"), ("norm-eurocode-anastruct", "anastruct"), ("norm-eurocode-scipy-numerics", "numerical")
EQ, NUM, SPREAD, EXACT = "eurocode-quantities-v1", "numerical-v1", "method-spread-v1", "table-exact-v1"
STD = {"en1990": "EN 1990", "en1991": "EN 1991-1-1", "en1992": "EN 1992-1-1", "en1993": "EN 1993-1-1", "en1994": "EN 1994-1-1", "en1995": "EN 1995-1-1", "en1996": "EN 1996-1-1", "en1997": "EN 1997-1", "en1998": "EN 1998-1", "en1999": "EN 1999-1-1"}


def ref(lib, independent=False):
    """🔗️ A reference to adapter `lib`; independent formulations are labelled as such, never as third-party."""
    return {"oracle": lib[0], "adapter": lib[1], "kind": "independent-formulation" if independent else "third-party-library"}


def case(art, slug, check, clause, inputs, expected, refs, edition="2004", standard=None, table=None, annex=None):
    """🧾️ One case `art.slug` with placeholder expectations `{name: (unit, profile)}`."""
    citation = {"standard": standard or STD[art], "edition": edition, "clause": clause}
    if table:
        citation["table"] = table
    if annex:
        citation["annex"] = annex
    return {"id": f"{art}.{slug}", "check": check, "citation": citation, "inputs": inputs, "expected": {k: {"value": 0, "unit": u, "profile": p} for k, (u, p) in expected.items()}, "references": refs}


def write(art, name, cases):
    """💾️ Write `name`.json for `art` with a relative `$schema`."""
    folder = ART / DIRS[art] / LEAF
    folder.mkdir(parents=True, exist_ok=True)
    rel = Path(*([".."] * len(folder.relative_to(ROOT).parts))) / SCHEMA.relative_to(ROOT)
    document = {"$schema": str(rel), "schemaVersion": 1, "artifact": art, "annex": "de", "cases": cases}
    (folder / f"{name}.json").write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def q(unit, profile=EQ, *names):
    """📏️ Expectation map for `names` with one unit and profile."""
    return {n: (unit, profile) for n in names}


T_BEAM = {"fck": 30e6, "fyk": 500e6, "gamma_c": 1.5, "gamma_s": 1.15, "alpha_cc": 0.85, "b_f": 1.2, "h_f": 0.15, "b_w": 0.3, "h": 0.6, "d": 0.55, "As": 0.002}
C = "en1992"


def en1990():
    cases = []
    for pf, tag in ((1.0e-3, "1e-3"), (1.0e-4, "1e-4"), (7.23e-5, "7.23e-5"), (1.0e-6, "1e-6")):
        cases.append(case("en1990", f"C.beta-pf-{tag}", "en1990.C.beta", "Annex C Table C2 reliability index", {"pf": pf}, {"beta": ("1", EQ)}, [ref(SY), ref(NU, True)], annex=None))
    pair = {"mu_R": 300.0, "sigma_R": 30.0, "mu_E": 150.0, "sigma_E": 30.0}
    form = q("1", NUM, "beta", "alpha_R", "alpha_E") | {"R_d": ("1", NUM), "E_d": ("1", NUM), "P_f": ("1", NUM)}
    cases.append(case("en1990", "C.form-equal-scatter", "en1990.C.form-linear", "Annex C.6 and C.7 FORM sensitivity factors", pair, form, [ref(SY, True), ref(NU, True)]))
    cases.append(case("en1990", "C.form-resistance-scatter", "en1990.C.form-linear", "Annex C.6 and C.7 FORM sensitivity factors", {"mu_R": 420.0, "sigma_R": 42.0, "mu_E": 200.0, "sigma_E": 36.0}, form, [ref(SY, True), ref(NU, True)]))
    common = {"G_k": [50.0e3, 20.0e3], "Q_k": [30.0e3, 20.0e3, 10.0e3], "psi_0": [0.7, 0.7, 0.6], "psi_1": [0.5, 0.5, 0.2], "psi_2": [0.3, 0.3, 0.0], "gamma_G": 1.35, "gamma_Q": 1.5, "xi": 0.85}
    out = {k: ("N", EQ) for k in ("E_6_10", "E_6_10a", "E_6_10b", "E_d", "E_char", "E_freq", "E_qp")}
    cases.append(case("en1990", "6.4.combinations-three-variable", "en1990.6.4.combinations", "6.4.3.2 Eq. 6.10, 6.10a, 6.10b, 6.14b, 6.15b, 6.16b", common, out, [ref(NU, True)], annex="en"))
    cases.append(case("en1990", "6.4.combinations-single-variable", "en1990.6.4.combinations", "6.4.3.2 Eq. 6.10, 6.10a, 6.10b, 6.14b, 6.15b, 6.16b", common | {"Q_k": [40.0e3], "psi_0": [0.7], "psi_1": [0.5], "psi_2": [0.3]}, out, [ref(NU, True)], annex="en"))
    cases.append(case("en1990", "6.4.combinations-ten-percent-permanent", "en1990.6.4.combinations", "6.4.3.2 Eq. 6.10b governing when the permanent share is large", common | {"G_k": [200.0e3], "Q_k": [60.0e3, 15.0e3], "psi_0": [0.7, 0.5], "psi_1": [0.5, 0.2], "psi_2": [0.3, 0.0]}, out, [ref(NU, True)], annex="en"))
    write("en1990", "third-party-extended", cases)
    envelope = {"E": 33e9, "A": 0.15, "I": 0.003125, "spans": [6.0, 6.0], "g": 20e3, "q": 15e3, "gamma_G": 1.35, "gamma_Q": 1.5, "gamma_G_inf": 1.0}
    expected = {"M_sag_0": ("N*m", NUM), "M_sag_1": ("N*m", NUM), "M_hog_support": ("N*m", NUM), "R_max_support": ("N", NUM), "R_min_0": ("N", NUM)}
    write("en1990", "third-party-mechanics-extended", [case("en1990", "mechanics.two-span-load-arrangement-envelope", "mechanics.beam-envelope", "6.4.3.2 load arrangements and 6.4.3.2 Eq. 6.10 partial factors; EN 1990 Annex A1.2.2 Table A1.2(B)", envelope, expected, [ref(PN), ref(AN)]), case("en1990", "mechanics.unequal-spans-envelope", "mechanics.beam-envelope", "6.4.3.2 load arrangements", envelope | {"spans": [5.0, 7.5], "g": 12e3, "q": 25e3}, expected, [ref(PN), ref(AN)])])


def en1991():
    janssen = q("Pa", NUM, "p_hf", "p_vf", "p_wf", "p_ho") | {"z_o": ("m", NUM)}
    cases = [
        case("en1991", f"4.janssen-{slug}", "en1991.4.janssen-pressure", "EN 1991-4 5.2.1.2 Eq. 5.3, 5.4 (slice equilibrium)", {"gamma": g, "a_m": a, "mu": mu, "K": k, "depth": z}, janssen, [ref(NU, True), ref(SY, True)], standard="EN 1991-4")
        for slug, g, a, mu, k, z in (("wheat-deep", 9000.0, 1.0, 0.4, 0.5, 6.0), ("cement-shallow", 16000.0, 0.75, 0.5, 0.45, 2.0), ("deep-asymptote", 8000.0, 1.2, 0.35, 0.55, 30.0))
    ]
    write("en1991", "third-party-extended", cases)
    base = {"E": 33e9, "A": 0.15, "I": 0.003125, "Q": 300e3, "s": 1.2, "step": 0.3}
    tandem = {"M_max": ("N*m", NUM), "R_max": ("N", NUM)}
    write("en1991", "third-party-mechanics-extended", [case("en1991", f"mechanics.lm1-tandem-span-{str(span).replace('.', '_')}", "mechanics.lm1-tandem", "EN 1991-2 4.3.2 Load Model 1 tandem system TS (two axles of Q_1k at 1.2 m)", base | {"span": span}, tandem, [ref(PN), ref(AN), ref(NU, True)], standard="EN 1991-2", edition="2003") for span in (6.0, 9.6, 15.6)])


def en1992():
    cases = []
    creep = {"h_0": ("m", NUM), "phi_RH": ("1", EQ), "phi_0": ("1", EQ), "phi": ("1", EQ), "eps_cd_0": ("1", EQ), "eps_cd": ("1", EQ), "eps_ca": ("1", EQ), "eps_cs": ("1", EQ)}
    for slug, fck, rh, ac, u, cement, t0 in (("c30-rh50", 30e6, 50.0, 0.15, 1.6, 1, 28.0), ("c50-rh80-rapid", 50e6, 80.0, 0.12, 1.2, 2, 7.0), ("c25-rh70-slow", 25e6, 70.0, 0.2, 1.8, 0, 90.0)):
        cases.append(case(C, f"3.1.creep-shrinkage-{slug}", "en1992.3.1.creep-shrinkage", "3.1.4 and Annex B Eq. B.1-B.13; 3.1.4(6) Eq. 3.8-3.12", {"fck": fck, "RH": rh, "Ac": ac, "u": u, "cement_class": cement, "t0": t0, "t": 18250.0, "t_s": 3.0}, creep, [ref(SC)], table="B"))
    cases.append(case(C, "3.6.creep-deformation", "en1992.3.6.creep-deformation", "3.1.4(2) Eq. 3.6", {"phi_inf_t0": 2.4, "sigma_c": 10e6, "e_c": 33e9}, q("1", EQ, "epsilon_cc"), [ref(BP)]))
    for fck in (60e6, 90e6):
        cases.append(case(C, f"3.21.eta-lambda-c{int(fck / 1e6)}", "en1992.3.21.eta", "3.1.7(3) Eq. 3.21, 3.22", {"f_ck": fck}, q("1", EQ, "eta"), [ref(BP)]))
        cases.append(case(C, f"3.19.lambda-c{int(fck / 1e6)}", "en1992.3.19.lambda", "3.1.7(3) Eq. 3.19, 3.20", {"f_ck": fck}, q("1", EQ, "lambda"), [ref(BP)]))
    cases.append(case(C, "4.1.c-nom", "en1992.4.1.c-nom", "4.4.1.1(2) Eq. 4.1", {"c_min": 0.025, "delta_c_dev": 0.01}, q("m", EQ, "c_nom"), [ref(BP)]))
    cases.append(case(C, "4.2.c-min", "en1992.4.2.c-min", "4.4.1.2(2) Eq. 4.2", {"c_min_b": 0.02, "c_min_dur": 0.025, "delta_c_dur_gamma": 0.0, "delta_c_dur_st": 0.0, "delta_c_dur_add": 0.0}, q("m", EQ, "c_min"), [ref(BP)]))
    for slug, over in (("beam-axial-free", {"h": 0.5, "b": 0.3, "N_Ed": 0.0}), ("deep-beam-compression", {"h": 0.9, "b": 0.4, "N_Ed": -300e3})):
        cases.append(case(C, f"7.3.min-reinforcement-{slug}", "en1992.7.3.min-reinforcement", "7.3.2(2) Eq. 7.1; Table 7.2 k; k_c of rectangular sections", {"f_ct_eff": 2.9e6, "sigma_s": 300e6} | over, {"k_c": ("1", EQ), "k": ("1", EQ), "A_s_min": ("m^2", EQ)}, [ref(SC)]))
    for slug, over in (("shallow-axis", {"x": 0.15}), ("deep-axis", {"x": 0.3})):
        cases.append(case(C, f"7.3.effective-tension-area-{slug}", "en1992.7.3.effective-tension-area", "7.3.2(3) h_c,ef; Eq. 7.10", {"h": 0.5, "d": 0.45, "b": 0.3, "As": 0.0012} | over, {"h_c_ef": ("m", EQ), "rho_p_eff": ("1", EQ)}, [ref(SC)]))
    cases.append(case(C, "7.14.sr-max-far", "en1992.7.14.sr-max-far", "7.3.4(3) Eq. 7.14", {"h": 0.5, "x": 0.15}, q("m", EQ, "sr_max"), [ref(SC)]))
    cases.append(case(C, "6.2.shear-link-limit", "en1992.6.2.shear-link-limit", "6.2.3(3) Eq. 6.12", {"f_ck": 30e6, "alpha_cc": 1.0, "gamma_c": 1.5, "b_w": 0.3, "s": 0.2, "f_ywd": 434.78e6, "Ac": 0.15}, q("m^2", EQ, "A_sw_max"), [ref(SC)]))
    for slug, over, refs in (("flange-compressed", {}, [ref(SC), ref(CP), ref(NU, True)]), ("web-compressed", {"As": 0.006}, [ref(SC), ref(CP), ref(NU, True)]), ("c50-web", {"fck": 50e6, "As": 0.004}, [ref(SC), ref(CP), ref(NU, True)])):
        cases.append(case(C, f"6.1.flexure-t-{slug}", "en1992.6.1.flexure-t", "6.1; 3.1.7(1); T-section with effective flange width 5.3.2.1", T_BEAM | over, {"M_Rd": ("N*m", NUM)}, refs, annex="de"))
    cases.append(case(C, "6.8.vrds", "en1992.6.8.vrds", "6.2.3(3) Eq. 6.8", {"a_sw": 157e-6, "s": 0.2, "z": 0.405, "f_ywd": 434.78e6, "theta": 45.0}, q("N", EQ, "V_Rd_s"), [ref(BP)]))
    cases.append(case(C, "6.9.vrdmax", "en1992.6.9.vrdmax", "6.2.3(3) Eq. 6.9", {"b_w": 0.3, "z": 0.405, "f_cd": 17e6, "nu_1": 0.528, "alpha_cw": 1.0, "theta": 45.0}, q("N", EQ, "V_Rd_max"), [ref(BP)]))
    for fck in (30e6, 70e6):
        cases.append(case(C, f"6.10.nu1-c{int(fck / 1e6)}", "en1992.6.10.nu1", "6.2.3(3) Eq. 6.10a/b N", {"f_ck": fck}, q("1", EQ, "nu_1"), [ref(BP)]))
    for sigma in (2e6, 6e6, 15e6):
        cases.append(case(C, f"6.11.alpha-cw-sigma{int(sigma / 1e6)}", "en1992.6.11.alpha-cw", "6.2.3(3) Eq. 6.11a/b/c N", {"sigma_cp": sigma, "f_cd": 17e6}, q("1", EQ, "alpha_cw"), [ref(BP)]))
    cases.append(case(C, "6.26.tau-wall", "en1992.6.26.tau-wall", "6.3.2(1) Eq. 6.26", {"t_ed": 50e3, "a_k": 0.083789}, q("N/m", EQ, "tau_t_t_ef"), [ref(BP)]))
    cases.append(case(C, "6.28.asl-torsion", "en1992.6.28.asl-torsion", "6.3.2(3) Eq. 6.28", {"u_k": 1.2, "f_yd": 434.78e6, "t_ed": 50e3, "a_k": 0.083789, "theta": 45.0}, q("m^2", EQ, "sum_A_sl"), [ref(BP)]))
    cases.append(case(C, "6.47.punching-vrdc", "en1992.6.47.punching-vrdc", "6.4.4(1) Eq. 6.47", {"c_rd_c": 0.12, "k": 1.89, "rho_l": 0.01, "f_ck": 30e6, "k_1": 0.1, "sigma_cp": 0.0, "v_min": 0.5e6}, q("Pa", EQ, "v_Rd_c"), [ref(BP)]))
    cases.append(case(C, "6.51.punching-eccentric", "en1992.6.51.punching-eccentric", "6.4.3(3) Eq. 6.51", {"v_ed_red": 500e3, "u": 3.2, "d": 0.25, "k": 0.6, "m_ed": 60e3, "w": 1.2}, q("Pa", EQ, "v_Ed"), [ref(BP)]))
    cases.append(case(C, "6.43.beta-rectangular", "en1992.6.43.beta-rectangular", "6.4.3(6) Eq. 6.43", {"ey": 0.1, "ez": 0.05, "by": 0.4, "bz": 0.4}, q("1", EQ, "beta"), [ref(BP)]))
    cases.append(case(C, "6.39.beta-moment", "en1992.6.39.beta-moment", "6.4.3(3) Eq. 6.39", {"k": 0.6, "m_ed": 60e3, "v_ed": 500e3, "u_1": 3.2, "w_1": 1.2}, q("1", EQ, "beta"), [ref(BP)]))
    cases.append(case(C, "8.2.fbd", "en1992.8.2.fbd", "8.4.2(2) Eq. 8.2", {"eta_1": 1.0, "eta_2": 1.0, "f_ctd": 2.0e6}, q("Pa", EQ, "f_bd"), [ref(BP)]))
    for slug, over in (("alpha-product-above-limit", {"alpha_2": 0.9, "alpha_3": 0.95, "alpha_5": 0.9}), ("alpha-product-capped", {"alpha_2": 0.7, "alpha_3": 0.8, "alpha_5": 0.8})):
        cases.append(case(C, f"8.4.anchorage-design-{slug}", "en1992.8.4.anchorage-design", "8.4.4(1) Eq. 8.4, 8.5, 8.6", {"alpha_1": 1.0, "alpha_4": 1.0, "l_b_rqd": 0.6, "diameter": 0.016} | over, q("1", EQ, "product_alpha_235") | q("m", EQ, "l_b_min", "l_bd"), [ref(BP)]))
    cases.append(case(C, "8.10.lap-length", "en1992.8.10.lap-length", "8.7.3(1) Eq. 8.10", {"alpha_1": 1.0, "alpha_2": 0.9, "alpha_3": 1.0, "alpha_5": 1.0, "alpha_6": 1.5, "l_b_rqd": 0.6, "l_0_min": 0.3}, q("m", EQ, "l_0"), [ref(BP)]))
    cases.append(case(C, "8.14.bundle-diameter", "en1992.8.14.bundle-diameter", "8.9.1(2) Eq. 8.14", {"diameter": 0.016, "n_b": 2}, q("m", EQ, "diameter_n"), [ref(BP)]))
    write(C, "third-party-extended", cases)


def en1993():
    cases = []
    E = "en1993"
    gross = lambda names: {n: (u, NUM) for n, u in names}
    cases.append(case(E, "section.tee-t150x150", "en1993.section.tee", "6.2 section constants (T-section)", {"h": 0.15, "b": 0.15, "tf": 0.0107, "tw": 0.0071, "r": 0.012}, gross([("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("It", "m^4"), ("Wpl_y", "m^3"), ("Wpl_z", "m^3")]), [ref(SP)], edition="2005"))
    for slug, over in (("plate-girder", {}), ("heavy-bottom-flange", {"b_t": 0.25, "b_b": 0.5, "t_ft": 0.016, "t_fb": 0.04})):
        mono = {"h": 0.8, "b_t": 0.3, "b_b": 0.4, "t_ft": 0.02, "t_fb": 0.03, "tw": 0.012, "r": 0.0} | over
        cases.append(case(E, f"section.mono-i-{slug}", "en1993.section.mono-i", "5.5 and 6.2 section constants of welded mono-symmetric I-sections", mono, gross([("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("It", "m^4"), ("Iw", "m^6"), ("Wpl_y", "m^3"), ("Wpl_z", "m^3")]), [ref(SP)], edition="2005"))
    cases.append(case(E, "section.ehs-300x200x10", "en1993.section.ehs", "6.2 section constants (elliptical hollow section)", {"d_x": 0.3, "d_y": 0.2, "t": 0.01}, gross([("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("It", "m^4")]), [ref(SP)], edition="2005"))
    for h, b, t, tag in ((0.2, 0.1, 0.008, "200x100x8"), (0.3, 0.3, 0.012, "300x300x12")):
        cases.append(case(E, f"section.rhs-sharp-{tag}", "en1993.section.rhs-sharp", "6.2 section constants of a sharp-cornered rectangular hollow section (difference of two rectangles)", {"h": h, "b": b, "t": t, "r_out": 0.0}, gross([("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("Wel_y", "m^3"), ("Wel_z", "m^3"), ("Wpl_y", "m^3"), ("Wpl_z", "m^3")]), [ref(SP), ref(NU, True)], edition="2005"))
    singles = [
        ("6.7.nu-rd", "en1993.6.7.nu-rd", "6.2.3 Eq. 6.7", {"a_net": 0.008, "f_u": 490e6, "gamma_m2": 1.25}, q("N", EQ, "N_u_Rd")),
        ("6.10.nc-rd", "en1993.6.10.nc-rd", "6.2.4 Eq. 6.10", {"a": 0.0106, "f_y": 355e6, "gamma_m0": 1.0}, q("N", EQ, "N_c_Rd")),
        ("6.14.mc-rd-class3", "en1993.6.14.mc-rd-class3", "6.2.5 Eq. 6.14", {"w_el_min": 0.001, "f_y": 355e6, "gamma_m0": 1.0}, q("N*m", EQ, "M_c_Rd")),
        ("6.32.mnrd-rectangular", "en1993.6.32.mnrd-rectangular", "6.2.9.1 Eq. 6.32", {"m_pl_rd": 100e3, "n_ed": 300e3, "n_pl_rd": 1000e3}, q("N*m", EQ, "M_N_Rd")),
        ("6.39.aw-hollow", "en1993.6.39.aw-hollow", "6.2.9.1 Eq. 6.39 a_w for hollow sections", {"a": 0.0043781, "b": 0.1, "t": 0.008}, q("1", EQ, "a_w")),
        ("6.39.mnyrd-hollow", "en1993.6.39.mnyrd-hollow", "6.2.9.1 Eq. 6.39", {"mpl_y_rd": 100e3, "n": 0.3, "a_w": 0.4}, q("N*m", EQ, "M_N_y_Rd")),
        ("6.21.tau-i", "en1993.6.21.tau-i", "6.2.6(4) Eq. 6.21", {"v_ed": 200e3, "a_w": 0.002, "a_f": 0.005}, q("Pa", EQ, "tau_Ed")),
    ]
    for slug, check, clause, inputs, expected in singles:
        cases.append(case(E, slug, check, clause, inputs, expected, [ref(BP)], edition="2005"))
    write(E, "third-party-extended", cases)


def en1994():
    E = "en1994"
    base = {"E_a": 210e9, "h": 0.3, "b": 0.15, "tf": 0.0107, "tw": 0.0071, "r": 0.0, "b_eff": 2.0, "h_c": 0.12}
    elastic = {"n": ("1", EQ), "A_eff": ("m^2", NUM), "z_na": ("m", NUM), "I_eff": ("m^4", NUM)}
    cases = [case(E, "5.4.composite-elastic-ipe300-short-term", "en1994.5.4.composite-elastic", "5.4.1.2 effective width and 5.4.2.2 modular ratio n = E_a / E_cm; transformed section", base | {"E_c": 33e9}, elastic, [ref(SP), ref(NU, True)]),
             case(E, "5.4.composite-elastic-ipe300-long-term", "en1994.5.4.composite-elastic", "5.4.2.2 modular ratio n_L = n (1 + psi_L phi_t) with E_c,eff = E_cm / (1 + psi_L phi_t)", base | {"E_c": 11e9}, elastic, [ref(SP), ref(NU, True)]),
             case(E, "5.4.composite-elastic-ipe300-rounded", "en1994.5.4.composite-elastic", "5.4.2.2 transformed section with the rolled root radius", base | {"E_c": 33e9, "r": 0.015}, elastic, [ref(SP)])]
    plastic = {"z_pna": ("m", NUM), "M_pl_Rd": ("N*m", NUM)}
    for slug, over in (("neutral-axis-in-slab", {}), ("neutral-axis-in-flange", {"h_c": 0.07, "b_eff": 1.0}), ("neutral-axis-in-web", {"h_c": 0.08, "b_eff": 0.5})):
        cases.append(case(E, f"6.2.composite-plastic-{slug}", "en1994.6.2.composite-plastic", "6.2.1.2 plastic resistance moment of a composite section with full shear connection under sagging bending", {"h": 0.3, "b": 0.15, "tf": 0.0107, "tw": 0.0071, "h_c": 0.12, "b_eff": 2.0, "f_y": 355e6, "gamma_a": 1.0, "f_ck": 30e6, "gamma_c": 1.5} | over, plastic, [ref(NU, True)]))
    write(E, "third-party-extended", cases)
    envelope = {"E": 210e9, "A": 5.38e-3, "I": 8.356e-5, "spans": [6.0, 6.0], "g": 20e3, "q": 15e3, "gamma_G": 1.35, "gamma_Q": 1.5, "gamma_G_inf": 1.0}
    expected = {"M_sag_0": ("N*m", NUM), "M_sag_1": ("N*m", NUM), "M_hog_support": ("N*m", NUM), "R_max_support": ("N", NUM), "R_min_0": ("N", NUM)}
    write(E, "third-party-mechanics-extended", [case(E, "mechanics.two-span-composite-envelope", "mechanics.beam-envelope", "5.4.2.3 and 5.4.2.4 global analysis of continuous composite beams, load arrangements of EN 1990 6.4.3.2", envelope, expected, [ref(PN), ref(AN)])])


def en1995():
    E = "en1995"
    cases = []
    quantities = lambda names: {n: ("Pa" if n.startswith(("f_", "e_", "g_")) else "kg/m^3", EXACT) for n in names}
    soft = ("f_m_k", "f_t_0_k", "f_t_90_k", "f_c_0_k", "f_c_90_k", "f_v_k", "g_mean", "e_m_90_mean", "rho_k", "rho_mean", "e_m_0_mean", "e_m_0_k")
    for name in ("C14", "C16", "C18", "C20", "C22", "C24", "C27", "C30", "C35", "C40", "C45", "C50"):
        cases.append(case(E, f"338.strength-class-{name.lower()}", "en1995.338.strength-class", "EN 338 Table 1 softwood bending strength classes", {"strength_class": name}, quantities(soft), [ref(BP)], edition="2016", standard="EN 338", table="1"))
    for name in ("T14", "T22", "T30"):
        cases.append(case(E, f"338.strength-class-{name.lower()}", "en1995.338.strength-class", "EN 338 Table 2 softwood tension strength classes", {"strength_class": name}, quantities(("f_m_k", "f_t_0_k", "f_t_90_k", "f_c_0_k", "f_c_90_k", "f_v_k", "g_mean", "rho_k", "rho_mean", "e_t_0_mean", "e_t_0_k")), [ref(BP)], edition="2016", standard="EN 338", table="2"))
    for name in ("D24", "D40", "D70"):
        cases.append(case(E, f"338.strength-class-{name.lower()}", "en1995.338.strength-class", "EN 338 Table 3 hardwood strength classes", {"strength_class": name}, quantities(soft), [ref(BP)], edition="2016", standard="EN 338", table="3"))
    write(E, "third-party-extended", cases)
    envelope = {"E": 11e9, "A": 0.03, "I": 0.1 * 0.3**3 / 12, "spans": [4.5, 4.5], "g": 1.5e3, "q": 2.5e3, "gamma_G": 1.35, "gamma_Q": 1.5, "gamma_G_inf": 1.0}
    expected = {"M_sag_0": ("N*m", NUM), "M_sag_1": ("N*m", NUM), "M_hog_support": ("N*m", NUM), "R_max_support": ("N", NUM), "R_min_0": ("N", NUM)}
    write(E, "third-party-mechanics-extended", [case(E, "mechanics.two-span-timber-envelope", "mechanics.beam-envelope", "6.1 and 2.4.1 with EN 1990 6.4.3.2 load arrangements", envelope, expected, [ref(PN), ref(AN)])])


def en1996():
    E = "en1996"
    cases = []
    for slug, over in (("x9-catalogue", {}), ("heavier-reinforcement", {"As": 1500e-6}), ("lower-strength-masonry", {"f_d": 3.0e6, "As": 400e-6})):
        cases.append(case(E, f"6.6.1.flexure-reinforced-{slug}", "en1996.6.6.1.flexure-reinforced", "6.6.1 reinforced masonry members subjected to bending; rectangular stress block", {"f_d": 5.957e6, "f_yd": 435e6, "b": 1.0, "h": 0.35, "d": 0.3, "As": 600e-6} | over, {"M_Rd": ("N*m", NUM), "x": ("m", NUM)}, [ref(CP), ref(NU, True)]))
    write(E, "third-party-extended", cases)


def en1997():
    E = "en1997"
    cases = []
    stress = {"sigma_z": ("Pa", NUM)}
    for slug, inputs in (("strip-edge", {"shape": "strip", "q": 100e3, "z": 2.0, "width": 2.0, "x": 0.0}), ("strip-outside", {"shape": "strip", "q": 100e3, "z": 3.0, "width": 2.0, "x": 1.5}), ("rectangle-corner", {"shape": "rectangle", "q": 100e3, "z": 2.0, "width": 2.0, "length": 4.0}), ("rectangle-corner-deep", {"shape": "rectangle", "q": 150e3, "z": 6.0, "width": 3.0, "length": 3.0}), ("circle-axis", {"shape": "circle", "q": 100e3, "z": 2.0, "radius": 1.0})):
        cases.append(case(E, f"F.boussinesq-{slug}", "en1997.F.boussinesq", "Annex F.2 stress increase below a loaded area (Boussinesq elastic half-space)", inputs, stress, [ref(GH), ref(NU, True)]))
    for slug, inputs in (("saturated-clay", {"S_r": 1.0, "e": 0.8, "G_s": 2.65, "gamma_w": 10e3}), ("partly-saturated-sand", {"S_r": 0.6, "e": 0.55, "G_s": 2.67, "gamma_w": 10e3})):
        cases.append(case(E, f"2.phase-relations-{slug}", "en1997.2.phase-relations", "2.4.3 derived values; unit weight from the phase relations of EN ISO 14688-2", inputs, {"gamma": ("N/m^3", EQ), "gamma_eff": ("N/m^3", EQ)}, [ref(GH)]))
    write(E, "third-party-extended", cases)


def en1998():
    E = "en1998"
    cases = []
    spectrum = {"a_g": 1.5, "S": 1.15, "T_B": 0.15, "T_C": 0.5, "T_D": 2.0, "q": 3.0, "beta": 0.2}
    for slug, period in (("rising-branch", 0.08), ("plateau", 0.3), ("constant-velocity", 1.0), ("constant-displacement", 3.0), ("lower-bound", 4.5)):
        cases.append(case(E, f"3.2.2.5.design-spectrum-{slug}", "en1998.3.2.2.5.design-spectrum", "3.2.2.5(4) Eq. 3.13-3.16; 3.2.2.2 Eq. 3.2-3.5", spectrum | {"T": period}, {"S_d": ("m/s^2", EQ), "S_e": ("m/s^2", EQ)}, [ref(SY, True)], annex="en"))
    modal = {"T_1": ("s", NUM), "T_2": ("s", NUM), "T_3": ("s", NUM)}
    cases.append(case(E, "4.3.3.shear-building-three-storey", "en1998.4.3.shear-building-modal", "4.3.3.1 modal response spectrum analysis; 4.3.3.2.2 Eq. 4.6 vs eigenvalue periods", {"m": [100e3, 100e3, 80e3], "k": [60e6, 50e6, 40e6]}, modal | {"Gamma_1": ("1", NUM), "m_eff_1_fraction": ("1", NUM)}, [ref(NU, True)]))
    cases.append(case(E, "4.3.3.shear-building-three-storey-periods", "en1998.4.3.shear-building-modal", "4.3.3.1 modal response spectrum analysis periods", {"m": [100e3, 100e3, 80e3], "k": [60e6, 50e6, 40e6]}, modal, [ref(NU, True), ref(SY, True)]))
    cases.append(case(E, "4.3.3.shear-building-five-storey-periods", "en1998.4.3.shear-building-modal", "4.3.3.1 modal response spectrum analysis periods", {"m": [250e3, 240e3, 240e3, 230e3, 200e3], "k": [180e6, 170e6, 160e6, 150e6, 120e6]}, {f"T_{j}": ("s", NUM) for j in range(1, 6)}, [ref(NU, True), ref(SY, True)]))
    frame = {"E": 210e9, "A": 5.38e-3, "I_col": 8.356e-5, "I_beam": 8.356e-3, "h": [3.5, 3.2, 3.2], "span": 6.0, "F": [40e3, 80e3, 120e3]}
    expected = {f"u_{j}": ("m", NUM) for j in (1, 2, 3)} | {f"drift_{j}": ("m", NUM) for j in (1, 2, 3)} | {"V_base": ("N", NUM), "M_base_left": ("N*m", NUM), "M_base_right": ("N*m", NUM)}
    write(E, "third-party-extended", cases)
    write(E, "third-party-mechanics-extended", [case(E, "mechanics.lateral-force-frame-three-storey", "mechanics.lateral-frame", "4.3.3.2.3 Eq. 4.11 lateral forces on a planar frame; 4.4.3.2 interstorey drift", frame, expected, [ref(PN), ref(AN)]), case(E, "mechanics.lateral-force-frame-two-storey", "mechanics.lateral-frame", "4.3.3.2.3 Eq. 4.11 lateral forces on a planar frame; 4.4.3.2 interstorey drift", frame | {"h": [4.0, 3.0], "F": [60e3, 90e3], "span": 7.5}, {k: v for k, v in expected.items() if not k.endswith("3")}, [ref(PN), ref(AN)])])


def en1999():
    E = "en1999"
    cases = []
    names = lambda *items: {n: (u, NUM) for n, u in items}
    base = [("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("It", "m^4")]
    cases.append(case(E, "section.i-extruded-200x100", "en1999.section.i", "6.1 section properties of extruded aluminium I-sections", {"h": 0.2, "b_t": 0.1, "b_b": 0.1, "t_ft": 0.01, "t_fb": 0.01, "tw": 0.006, "r": 0.0}, names(*base, ("Iw", "m^6"), ("Wpl_y", "m^3")), [ref(SP)]))
    cases.append(case(E, "section.rhs-extruded-sharp-120x80", "en1999.section.rhs-sharp", "6.1 section properties of extruded hollow sections (sharp corners)", {"h": 0.12, "b": 0.08, "t": 0.005, "r_out": 0.0}, names(("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("Wel_y", "m^3"), ("Wel_z", "m^3"), ("Wpl_y", "m^3"), ("Wpl_z", "m^3")), [ref(SP), ref(NU, True)]))
    cases.append(case(E, "section.chs-extruded-100x5", "en1999.section.chs", "6.1 section properties of extruded tubes", {"d": 0.1, "t": 0.005}, names(*base, ("Wel_y", "m^3"), ("Wpl_y", "m^3")), [ref(SP), ref(NU, True)]))
    cases.append(case(E, "section.channel-extruded-100", "en1999.section.channel", "6.1 section properties of extruded channels", {"h": 0.1, "b": 0.05, "tf": 0.007, "tw": 0.005, "r": 0.004}, names(("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("It", "m^4"), ("Iw", "m^6"), ("Wpl_y", "m^3"), ("y_sc", "m")), [ref(SP)]))
    cases.append(case(E, "section.tee-extruded-60", "en1999.section.tee", "6.1 section properties of extruded T-sections", {"h": 0.06, "b": 0.06, "tf": 0.006, "tw": 0.005, "r": 0.003}, names(("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("It", "m^4"), ("Wpl_y", "m^3")), [ref(SP)]))
    cases.append(case(E, "section.angle-extruded-60x6", "en1999.section.angle", "6.1 section properties of extruded angles", {"h": 0.06, "b": 0.06, "t": 0.006, "r_root": 0.006, "r_toe": 0.003}, names(("A", "m^2"), ("Iy", "m^4"), ("Iz", "m^4"), ("I_major", "m^4"), ("I_minor", "m^4"), ("It", "m^4")), [ref(SP)]))
    cases.append(case(E, "section.ehs-extruded-120x80", "en1999.section.ehs", "6.1 section properties of extruded elliptical tubes", {"d_x": 0.12, "d_y": 0.08, "t": 0.005}, names(*base), [ref(SP)]))
    write(E, "third-party-extended", cases)


def main():
    for build in (en1990, en1991, en1992, en1993, en1994, en1995, en1996, en1997, en1998, en1999):
        build()


main()
