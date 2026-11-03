"""🧪️ Oracle v2 case files (`third-party-v2-extended.json`, `third-party-mechanics-v2-extended.json`) for the clauses that had no third-party check after wave 2 and for the wave-2 artifact features; placeholder expectations, bless with the harness (`oracle-wave2-v2-bless.sh`).

Usage: /home/deck/semio/.venv/bin/python -I oracle-wave2-v2-gen.py
"""

import json
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
ART = ROOT / "🗿️artifacts"
LEAF = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"
DIRS = {"en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1996": "🪨️en1996", "en1997": "🌍️en1997", "en1998": "🫨️en1998", "en1999": "🪶️en1999"}
SCHEMA = ROOT / "🔮️oracles/🏃️harness/🧬️schema/🔣️.json"
SC, SP, GH, SY, PN, AN, NU, PB = ("norm-eurocode-structuralcodes", "structuralcodes"), ("norm-eurocode-sectionproperties", "sectionproperties"), ("norm-eurocode-groundhog", "groundhog"), ("norm-eurocode-sympy", "sympy"), ("norm-eurocode-pynite", "pynite"), ("norm-eurocode-anastruct", "anastruct"), ("norm-eurocode-scipy-numerics", "numerical"), ("norm-eurocode-published-tables", "published")
EQ, NUM, SPREAD, EXACT = "eurocode-quantities-v1", "numerical-v1", "method-spread-v1", "table-exact-v1"
STD = {"en1990": "EN 1990", "en1991": "EN 1991-1-1", "en1992": "EN 1992-1-1", "en1993": "EN 1993-1-1", "en1994": "EN 1994-1-1", "en1996": "EN 1996-1-1", "en1997": "EN 1997-1", "en1998": "EN 1998-1", "en1999": "EN 1999-1-1"}


def ref(lib, independent=False):
    """🔗️ A reference to adapter `lib`; formulations written from the standard (no library computes them) are labelled independent."""
    return {"oracle": lib[0], "adapter": lib[1], "kind": "independent-formulation" if independent else "third-party-library"}


def case(art, slug, check, clause, inputs, expected, refs, edition="2004", standard=None, table=None, annex=None):
    """🧾️ One case `art.slug` with placeholder expectations `{name: (unit, profile[, tolerance])}`."""
    citation = {"standard": standard or STD[art], "edition": edition, "clause": clause}
    if table:
        citation["table"] = table
    if annex:
        citation["annex"] = annex
    exp = {}
    for k, v in expected.items():
        entry = {"value": 0, "unit": v[0], "profile": v[1]}
        if len(v) > 2:
            entry["tolerance"] = v[2]
        exp[k] = entry
    return {"id": f"{art}.{slug}", "check": check, "citation": citation, "inputs": inputs, "expected": exp, "references": refs}


def write(art, name, cases):
    """💾️ Write `name`.json for `art` with a relative `$schema`."""
    folder = ART / DIRS[art] / LEAF
    folder.mkdir(parents=True, exist_ok=True)
    rel = Path(*([".."] * len(folder.relative_to(ROOT).parts))) / SCHEMA.relative_to(ROOT)
    document = {"$schema": str(rel), "schemaVersion": 1, "artifact": art, "annex": "de", "cases": cases}
    (folder / f"{name}.json").write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def q(unit, profile, *names):
    """📏️ Expectation map for `names` with one unit and profile."""
    return {n: (unit, profile) for n in names}


def en1990():
    cases = []
    for n in (5, 10, 30):
        cases.append(case("en1990", f"D.7.characteristic-from-tests-n{n}", "en1990.D.7.characteristic-from-tests", "Annex D.7.2 Table D1 k_n (5 % fractile, V_x unknown: Student prediction factor; known: 1.645 sqrt(1 + 1/n))", {"n": n, "mean": 100.0, "s": 12.0, "sigma": 12.0}, {"k_n_unknown": ("1", NUM), "k_n_known": ("1", NUM), "X_k": ("1", EQ), "X_k_known": ("1", EQ)}, [ref(NU), ref(PB, True)], annex="en", table="D1"))
    spectrum = {"detail_category": 71e6, "stress_range": [120e6, 90e6, 60e6, 40e6, 20e6], "cycles": [1e4, 1e5, 5e5, 2e6, 1e7]}
    cases.append(case("en1990", "fatigue.miner-two-slope", "en1990.fatigue.miner", "Palmgren-Miner sum with the EN 1993-1-9 Figure 7.1 S-N curve of detail category 71 (m = 3 / m = 5)", spectrum, q("1", EQ, "D") | q("Pa", EQ, "dsigma_D", "dsigma_L"), [ref(NU, True)], standard="EN 1993-1-9"))
    cases.append(case("en1990", "fatigue.miner-below-limit", "en1990.fatigue.miner", "Palmgren-Miner sum, all bins below the cut-off dsigma_L give no damage", spectrum | {"stress_range": [20e6, 15e6, 10e6, 8e6, 5e6]}, q("1", EQ, "D") | q("Pa", EQ, "dsigma_D", "dsigma_L"), [ref(NU, True)], standard="EN 1993-1-9"))
    for slug, over in (("moderate", {"mu_R": 300.0, "sigma_R": 30.0, "mu_E": 150.0, "sigma_E": 30.0}), ("high-cov-action", {"mu_R": 420.0, "sigma_R": 42.0, "mu_E": 200.0, "sigma_E": 60.0})):
        cases.append(case("en1990", f"C.form-lognormal-gumbel-{slug}", "en1990.C.form-nonnormal", "Annex C.5 to C.7 FORM with non-normal variables: R lognormal, E Gumbel (maxima), Rackwitz-Fiessler transformation against the iso-probabilistic design point; beta_exact from the exact P_f integral", over, {"beta": ("1", NUM), "R_d": ("1", NUM), "E_d": ("1", NUM)}, [ref(NU, True), {"oracle": NU[0], "adapter": NU[1], "kind": "independent-formulation", "check": "en1990.C.form-nonnormal-rf"}], annex=None))
        cases.append(case("en1990", f"C.form-vs-exact-{slug}", "en1990.C.form-nonnormal", "Annex C: FORM beta against the exact reliability index of the same limit state (tolerance reflects the FORM linearisation)", over, {"beta_exact": ("1", SPREAD, 0.05)}, [ref(NU, True)], annex=None))
    write("en1990", "third-party-v2-extended", cases)


def en1991():
    cases = []
    for slug, v_b, z, z0, z_min in (("terrain-ii-10m", 25.0, 10.0, 0.05, 2.0), ("terrain-iii-40m", 27.5, 40.0, 0.3, 5.0), ("terrain-iv-below-zmin", 24.0, 6.0, 1.0, 10.0)):
        cases.append(case("en1991", f"4.wind-peak-pressure-{slug}", "en1991.4.wind-peak-pressure", "EN 1991-1-4 4.3.2 Eq. 4.3 to 4.5, 4.4 Eq. 4.7 and 4.5 Eq. 4.8", {"v_b": v_b, "z": z, "z0": z0, "z_min": z_min, "rho": 1.25, "c_o": 1.0, "k_I": 1.0}, q("1", EQ, "I_v", "c_r") | {"v_m": ("m/s", EQ), "q_p": ("Pa", EQ)}, [ref(SY, True), ref(NU, True)], standard="EN 1991-1-4", edition="2005", annex="en"))
    for slug, zone, hd, area in (("zone-a", "A", 1.0, 10.0), ("zone-b-small-area", "B", 1.0, 1.0), ("zone-d-low-building", "D", 0.1, 10.0), ("zone-d-interpolated", "D", 0.6, 2.5), ("zone-e-tall", "E", 3.0, 10.0)):
        cases.append(case("en1991", f"7.wind-cpe-{slug}", "en1991.7.wind-cpe", "EN 1991-1-4 7.2.2 Table 7.1 vertical walls, interpolation 7.1.2(2)", {"zone": zone, "h_d": hd, "area": area}, q("1", EQ, "c_pe_10", "c_pe_1", "c_pe"), [ref(NU, True), ref(SY, True)], standard="EN 1991-1-4", edition="2005", annex="en", table="7.1"))
    for alpha in (0.0, 20.0, 30.0, 45.0, 60.0, 75.0):
        cases.append(case("en1991", f"snow.shape-mono-pitch-{int(alpha)}deg", "en1991.snow.shape-mono-pitch", "EN 1991-1-3 5.3.2 Table 5.2 and Eq. 5.1", {"alpha": alpha, "C_e": 1.0, "C_t": 1.0, "s_k": 1100.0}, q("1", EQ, "mu_1", "mu_2") | {"s": ("Pa", EQ)}, [ref(NU, True), ref(SY, True)], standard="EN 1991-1-3", edition="2003", annex="en", table="5.2"))
    for slug, over in (("tall-parapet", {"h": 1.5, "b1": 8.0, "b2": 12.0, "s_k": 1100.0}), ("low-parapet-limited", {"h": 0.5, "b1": 2.0, "b2": 3.0, "s_k": 1000.0}), ("minimum", {"h": 0.3, "b1": 1.0, "b2": 1.0, "s_k": 1500.0})):
        cases.append(case("en1991", f"snow.drift-{slug}", "en1991.snow.drift", "EN 1991-1-3 5.3.6 and Annex B (obstructions): mu_w, l_s", {"gamma": 2000.0} | over, q("1", EQ, "mu_w") | {"l_s": ("m", EQ), "s_peak": ("Pa", EQ)}, [ref(NU, True), ref(SY, True)], standard="EN 1991-1-3", edition="2003", annex="en"))
    para = {"O": 0.04, "b": 1500.0, "q_td": 300e6, "t_lim": 1200.0}
    out = {"Gamma": ("1", EQ), "t_max": ("s", EQ), "theta_max": ("K", EQ), "theta_g": ("K", EQ)}
    for slug, over in (("heating", {"t": 1800.0}), ("cooling-mid", {"t": 10800.0}), ("cooling-short", {"O": 0.02, "b": 2000.0, "q_td": 200e6, "t": 28800.0}), ("cooling-long-fuel", {"O": 0.1, "b": 500.0, "q_td": 1000e6, "t": 7560.0})):
        cases.append(case("en1991", f"fire.parametric-{slug}", "en1991.fire.parametric-curve", "EN 1991-1-2 Annex A Eq. A.1 to A.12, ventilation-controlled (t_max > t_lim)", para | over, out, [ref(NU, True), ref(SY, True)], standard="EN 1991-1-2", edition="2002", annex="en"))
    for t in (5, 30, 60, 90, 120, 180):
        cases.append(case("en1991", f"fire.iso834-{t}min", "en1991.fire.iso834", "EN 1991-1-2 3.2.1 Eq. 3.4 standard temperature-time curve", {"t_min": t}, {"theta_g": ("K", NUM)}, [ref(NU, True), ref(SY, True), ref(PB, True)], standard="EN 1991-1-2", edition="2002", annex="en"))
    write("en1991", "third-party-v2-extended", cases)


def en1992():
    cases = []
    for slug, n in (("tension-both", {"n_x": 200e3, "n_y": 100e3, "n_xy": 80e3}), ("compression-y", {"n_x": 300e3, "n_y": -150e3, "n_xy": 120e3}), ("compression-x", {"n_x": -100e3, "n_y": 200e3, "n_xy": 100e3})):
        cases.append(case("en1992", f"F.membrane-reinforcement-{slug}", "en1992.F.membrane-reinforcement", "Annex F (informative) in-plane stress reinforcement, tension positive", n, q("N/m", NUM, "f_x", "f_y"), [ref(NU, True), ref(SY, True)]))
    cases.append(case("en1992", "J.3.corbel-truss", "en1992.J.3.corbel-truss", "Annex J.3 and 6.5 corbel strut-and-tie (three-bar truss)", {"F": 500e3, "a": 0.25, "z": 0.4, "H": 50e3}, {"F_t": ("N", EQ), "C": ("N", EQ), "theta": ("1", EQ)}, [ref(AN), ref(SY, True), ref(NU, True)], annex="en"))
    cases.append(case("en1992", "J.3.corbel-truss-no-horizontal", "en1992.J.3.corbel-truss", "Annex J.3 and 6.5 corbel strut-and-tie (three-bar truss)", {"F": 300e3, "a": 0.15, "z": 0.35, "H": 0.0}, {"F_t": ("N", EQ), "C": ("N", EQ), "theta": ("1", EQ)}, [ref(AN), ref(SY, True), ref(NU, True)], annex="en"))
    for slug, over in (("c30-rh50", {"f_cm": 38e6, "phi_RH": 1.4, "beta_H": 500.0}), ("c50-rh80", {"f_cm": 58e6, "phi_RH": 1.1, "beta_H": 700.0})):
        cases.append(case("en1992", f"KK.age-adjusted-modulus-{slug}", "en1992.KK.age-adjusted-modulus", "EN 1992-2 Annex KK age-adjusted effective modulus E_c,adj = E / (1 + chi phi), creep function of EN 1992-1-1 Annex B.1", {"E": 33e9, "t0": 28.0, "t": 18250.0} | over, {"phi": ("1", EQ), "chi": ("1", SPREAD), "E_adj": ("Pa", SPREAD)}, [ref(NU, True)], standard="EN 1992-2", edition="2005", annex="en"))
    write("en1992", "third-party-v2-extended", cases)


def en1993():
    cases = []
    frame = {"E": 210e9, "A": 0.02, "I_col": 8e-5, "I_beam": 1.6e-4, "h": 4.0, "span": 6.0, "H": 50e3, "V": 2.0e6}
    cases.append(case("en1993", "5.2.alpha-cr-storey-portal", "en1993.5.2.alpha-cr-storey", "5.2.1(4)B Eq. 5.2 alpha_cr = H/V h/delta_H with the first-order drift", frame, {"alpha_cr_formula": ("1", NUM), "sway": ("m", NUM)}, [ref(PN), ref(AN), ref(NU, True)]))
    for slug, over in (("stiff-beam", {"I_beam": 8e-4}), ("flexible-beam", {"I_beam": 6e-5})):
        cases.append(case("en1993", f"5.2.alpha-cr-eigen-{slug}", "en1993.5.2.alpha-cr-eigen", "5.2.1(3) alpha_cr as the lowest eigenvalue of the geometric stiffness problem (scipy) against the second-order amplification delta_II / delta_I = 1 / (1 - 1 / alpha_cr) (PyNite P-Delta)", frame | over, {"alpha_cr_eigen": ("1", SPREAD)}, [ref(NU, True), ref(PN)]))
        cases.append(case("en1993", f"5.2.alpha-cr-formula-vs-eigen-{slug}", "en1993.5.2.alpha-cr-storey", "5.2.1(4)B Eq. 5.2 against the eigenvalue: both quantities at one frame", frame | over, {"alpha_cr_formula": ("1", NUM)}, [ref(PN), ref(AN), ref(NU, True)]))
    for slug, over in (("h6-m2", {"h": 6.0, "m": 2.0}), ("h3-m5", {"h": 3.0, "m": 5.0}), ("h20-m1", {"h": 20.0, "m": 1.0})):
        cases.append(case("en1993", f"5.3.2.sway-imperfection-{slug}", "en1993.5.3.2.sway-imperfection", "5.3.2(3) Eq. 5.5 phi = phi_0 alpha_h alpha_m, equivalent horizontal force H = phi V", {"V": 1.2e6} | over, q("1", EQ, "alpha_h", "phi") | {"H_eq": ("N", EQ)}, [ref(NU, True), ref(SY, True)]))
    for slug, over in (("square-eccentric", {"b": 0.2, "d": 0.2, "F": 100e3, "e": 0.15}), ("slender-eccentric", {"b": 0.1, "d": 0.3, "F": 80e3, "e": 0.2})):
        cases.append(case("en1993", f"1-8.4.5.weld-group-box-{slug}", "en1993.1-8.4.5.weld-group-box", "EN 1993-1-8 4.5.3.2 directional/simplified method, elastic vector method for a box weld group (per unit throat)", over, {"L_w": ("m", NUM), "I_p": ("m^3", NUM), "f_max": ("N/m", NUM)}, [ref(SP), ref(NU, True), ref(SY, True)], standard="EN 1993-1-8", edition="2005"))
    write("en1993", "third-party-v2-extended", cases)


def en1994():
    cases = []
    stud = {"d": 0.019, "h_sc": 0.1, "f_u": 450e6, "f_ck": 30e6, "E_cm": 33e9, "gamma_V": 1.25, "n_r": 1, "b_0": 0.1, "h_p": 0.06, "k_t_max": 0.75}
    out = {"alpha": ("1", EQ), "P_Rd_steel": ("N", EQ), "P_Rd_concrete": ("N", EQ), "P_Rd": ("N", EQ), "k_t": ("1", EQ), "P_Rd_sheeting": ("N", EQ)}
    for slug, over in (("h-d-5.3-steel-governs", {}), ("h-d-3.5-concrete-governs", {"d": 0.02, "h_sc": 0.07, "f_ck": 25e6, "E_cm": 31e9, "f_u": 500e6}), ("two-studs-per-rib", {"n_r": 2, "b_0": 0.12, "h_p": 0.05, "h_sc": 0.095, "k_t_max": 0.6})):
        cases.append(case("en1994", f"6.6.3.stud-resistance-{slug}", "en1994.6.6.3.stud-resistance", "6.6.3.1 Eq. 6.18, 6.19 and 6.6.4.2 Eq. 6.23", stud | over, out, [ref(NU, True), ref(SY, True)]))
    ltb = {"h": 0.3, "b": 0.3, "tf": 0.014, "tw": 0.0085, "E": 210e9, "G": 81e9, "L": 10.0, "f_o": 355e6, "gamma_M1": 1.0, "alpha_LT": 0.49, "lambda_0": 0.2, "k_s": 5.0e4}
    out_ltb = {"Iz": ("m^4", SPREAD), "It": ("m^4", SPREAD), "Iw": ("m^6", SPREAD), "M_cr": ("N*m", SPREAD), "lambda_LT": ("1", SPREAD), "chi_LT": ("1", SPREAD), "M_b_Rd": ("N*m", SPREAD)}
    for slug, over in (("inverted-u-ks", {}), ("inverted-u-stiff", {"k_s": 4.0e5, "L": 14.0}), ("no-restraint", {"k_s": 0.0})):
        cases.append(case("en1994", f"6.4.ltb-restrained-{slug}", "en1994.6.4.ltb-restrained", "6.4.2 continuous inverted-U frame model: M_cr from the sinusoidal LTB mode with continuous torsional restraint k_s, chi_LT of EN 1993-1-1 6.3.2.2", ltb | over, out_ltb, [ref(SP), ref(SY, True), ref(NU, True)]))
    mk = {"E_a": 210e9, "E_c": 33e9, "h": 0.3, "b": 0.15, "tf": 0.0107, "tw": 0.0071, "tw_eff": 0.0071, "r": 0.0, "b_eff": 0.3, "h_c": 0.06, "y_s": 0.06}
    cases.append(case("en1994", "6.2.1.4.mkappa-elastic-0.001", "en1994.6.2.1.4.mkappa-elastic", "6.2.1.4 moment-curvature relation, elastic branch M = E_a I_eff kappa (slab ordinates compressed, linear concrete)", mk | {"kappa": 1.0e-3, "tw": 0.0071}, {"M": ("N*m", NUM)}, [ref(SP), ref(NU, True)]))
    write("en1994", "third-party-v2-extended", cases)


def en1996():
    cases = []
    for slug, over in (("lambda-0.41", {"h_ef": 2.6, "t": 0.2, "e_mk": 0.02}), ("eccentric", {"h_ef": 2.6, "t": 0.2, "e_mk": 0.05}), ("slender", {"h_ef": 4.0, "t": 0.1, "e_mk": 0.01})):
        cases.append(case("en1996", f"G.phi-m-{slug}", "en1996.G.phi-m", "Annex G.1 reduction factor Phi_m at mid-height", {"f_k": 5e6, "E": 5e9} | over, q("1", EQ, "lambda", "A_1", "u", "Phi_m"), [ref(SY, True), ref(NU, True)], annex="en"))
    for slug, over in (("concentric", {"e": 0.2, "N_Ed": 300e3}), ("eccentric-partial-contact", {"e": 0.9, "N_Ed": 300e3}), ("limited-by-0.065fb", {"e": 0.2, "N_Ed": 1.2e6})):
        cases.append(case("en1996", f"6.2.shear-wall-{slug}", "en1996.6.2.shear-wall", "3.6.2 Eq. 3.5 f_vk and 6.2 Eq. 6.18 V_Rd = f_vd t l_c with the compressed length of a no-tension wall", {"l": 3.0, "t": 0.2, "f_vk0": 0.2e6, "f_b": 10e6, "gamma_M": 1.5} | over, {"l_c": ("m", EQ), "sigma_d": ("Pa", EQ), "f_vk": ("Pa", EQ), "V_Rd": ("N", EQ)}, [ref(NU, True), ref(SY, True)], annex="en"))
    write("en1996", "third-party-v2-extended", cases)


def en1997():
    cases = []
    base = {"phi": 30.0, "c": 0.0, "B": 2.0, "L": 3.0, "gamma": 18e3, "q": 18e3, "V": 1.0e6, "H": 0.15e6, "alpha": 0.0}
    out = {k: ("1", EQ) for k in ("N_q", "N_c", "N_gamma", "s_q", "s_gamma", "s_c", "i_q", "i_gamma", "i_c", "b_q")} | {"q_ult": ("Pa", EQ), "R_k": ("N", EQ)}
    for slug, over in (("sand-rectangle-inclined", {}), ("cohesive-frictional", {"phi": 25.0, "c": 10e3, "B": 2.5, "L": 2.5, "q": 20e3, "V": 1.2e6, "H": 0.1e6}), ("dense-square-vertical", {"phi": 35.0, "B": 2.0, "L": 2.0, "H": 0.0}), ("base-inclined", {"phi": 32.0, "B": 1.5, "L": 6.0, "alpha": 5.0, "H": 0.08e6, "V": 0.6e6})):
        cases.append(case("en1997", f"D.4.bearing-resistance-{slug}", "en1997.D.4.bearing-resistance", "Annex D.4 drained bearing resistance: N factors, shape s, base inclination b, load inclination i", base | over, out, [ref(GH), ref(NU, True), ref(SY, True)], annex="en"))
    for slug, over in (("rectangle-footing", {"B": 4.0, "L": 6.0, "q": 150e3, "E_m": 20e6, "z_max": 12.0}), ("square-footing", {"B": 3.0, "L": 3.0, "q": 200e3, "E_m": 40e6, "z_max": 9.0})):
        cases.append(case("en1997", f"F.stress-strain-settlement-{slug}", "en1997.F.stress-strain-settlement", "Annex F.1 stress-strain method: s = integral sigma_z / E_m dz, Boussinesq stresses of a uniform rectangle below the centre", over, {"s": ("m", NUM), "sigma_z_mid": ("Pa", NUM)}, [ref(GH), ref(NU)], annex="en"))
    for slug, over in (("clay-layer", {"H0": 4.0, "e0": 0.9, "sigma_v0": 80e3, "d_sigma": 60e3, "C_c": 0.3}), ("soft-layer", {"H0": 2.5, "e0": 1.1, "sigma_v0": 50e3, "d_sigma": 120e3, "C_c": 0.45})):
        cases.append(case("en1997", f"F.oedometer-settlement-{slug}", "en1997.F.oedometer-settlement", "Annex F.2 oedometer method for normally consolidated clay: s = H_0 C_c / (1 + e_0) log10((sigma'_v0 + d sigma) / sigma'_v0)", over, {"s": ("m", EQ), "delta_e": ("1", EQ)}, [ref(GH), ref(NU)], annex="en"))
    wall = {"gamma": 19e3, "phi": 32.0, "delta_a": 0.0, "delta_p": 0.0, "q": 10e3}
    wout = {"K_ah": ("1", NUM), "K_ph": ("1", NUM), "t": ("m", NUM)}
    for slug, over in (("cantilever-smooth", {"kind": "cantilever", "h": 3.0}), ("cantilever-friction", {"kind": "cantilever", "h": 3.0, "delta_a": 18.0, "delta_p": 21.0, "q": 0.0}), ("free-earth-prop", {"kind": "propped", "h": 5.0, "prop_depth": 0.5, "phi": 30.0, "delta_a": 20.0, "q": 0.0})):
        cases.append(case("en1997", f"9.embedded-wall-{slug}", "en1997.9.embedded-wall", "9.7 and Annex C: horizontal Coulomb earth pressures, moment equilibrium for the required embedment t (unfactored, cantilever about the toe, free earth support about the prop)", wall | over, wout, [ref(GH), ref(NU), ref(SY, True)], annex="en"))
    cases.append(case("en1997", "7.pile-group-3x4", "en1997.7.pile-group", "7.6.2.1 group efficiency by the Converse-Labarre formula", {"m": 3, "n": 4, "d": 0.6, "s": 1.8, "R_c": 1.2e6}, {"E_g": ("1", EQ), "R_group": ("N", EQ)}, [ref(NU, True), ref(SY, True)], annex="en"))
    cases.append(case("en1997", "7.pile-group-2x2-wide", "en1997.7.pile-group", "7.6.2.1 group efficiency by the Converse-Labarre formula", {"m": 2, "n": 2, "d": 0.4, "s": 2.4, "R_c": 0.9e6}, {"E_g": ("1", EQ), "R_group": ("N", EQ)}, [ref(NU, True), ref(SY, True)], annex="en"))
    for n in (3, 5):
        cases.append(case("en1997", f"7.pile-resistance-{n}-profiles", "en1997.7.pile-resistance", "7.6.2.2 Eq. 7.8 and Table A.10 correlation factors xi_1, xi_2 for the characteristic resistance from n profiles", {"D": 0.6, "q_s": [30e3, 80e3, 120e3], "h": [3.0, 4.0, 5.0], "q_b": 3e6, "n_profiles": n, "R_min_ratio": 0.9}, {"R_c": ("N", EQ), "xi_1": ("1", EXACT), "xi_2": ("1", EXACT), "R_ck": ("N", EQ)}, [ref(NU, True)], annex="en", table="A.10"))
    cases.append(case("en1997", "8.anchor-resistance-fixed-length", "en1997.8.anchor-resistance", "8.5.2 and 8.6 fixed-length pull-out R_a = pi d l_f tau_f, horizontal component for the wall", {"d": 0.15, "l_f": 6.0, "tau_f": 200e3, "theta": 15.0, "a_d": 500e3}, {"R_a": ("N", EQ), "R_a_h": ("N", EQ), "utilisation": ("1", EQ)}, [ref(NU, True)], annex="en"))
    cases.append(case("en1997", "8.anchor-elongation-free-length", "en1997.8.anchor-elongation", "8.5.3 free-length elastic elongation of a ground anchor under the design force", {"E": 195e9, "A_t": 0.001, "l_free": 12.0, "a_d": 500e3}, {"delta": ("m", NUM), "N": ("N", NUM)}, [ref(PN), ref(AN), ref(NU, True)], annex="en"))
    write("en1997", "third-party-v2-extended", cases)


def en1998():
    cases = []
    for slug, over in (("class-c", {"h": [5.0, 10.0, 20.0], "v": [150.0, 250.0, 400.0]}), ("class-b", {"h": [4.0, 26.0, 10.0], "v": [200.0, 500.0, 900.0]}), ("class-d-soft", {"h": [12.0, 30.0], "v": [90.0, 160.0]}), ("class-a", {"h": [2.0, 40.0], "v": [600.0, 1200.0]})):
        cases.append(case("en1998", f"3.1.vs30-{slug}", "en1998.3.1.vs30", "3.1.2 Eq. 3.1 v_s,30 = 30 / sum(h_i / v_i) over the top 30 m and Table 3.1 ground class", over, {"vs30": ("m/s", NUM), "class_index": ("1", EXACT)}, [ref(NU, True), ref(SY, True)], annex="en", table="3.1"))
    for slug, over in (("roof", {"Z": 12.0}), ("mid-height-resonance", {"Z": 6.0, "T_a": 0.45}), ("base", {"Z": 0.0})):
        cases.append(case("en1998", f"4.3.5.2.nonstructural-{slug}", "en1998.4.3.5.2.nonstructural", "4.3.5.2 Eq. 4.24 and 4.25 seismic force of a non-structural element", {"a_g": 1.5, "S": 1.25, "H": 12.0, "T_a": 0.15, "T_1": 0.45, "W_a": 1000.0, "gamma_a": 1.5, "q_a": 2.0} | over, {"S_a": ("1", EQ), "F_a": ("N", EQ)}, [ref(NU, True), ref(SY, True)], annex="en"))
    write("en1998", "third-party-v2-extended", cases)


def en1999():
    cases = []
    sec = {"h": 0.2, "b": 0.1, "tf": 0.01, "tw": 0.006, "E": 70e9, "G": 27e9, "f_o": 240e6, "gamma_M1": 1.1}
    out_n = {"A": ("m^2", SPREAD), "Iz": ("m^4", SPREAD), "N_cr": ("N", SPREAD), "lambda": ("1", SPREAD), "chi": ("1", SPREAD), "N_b_Rd": ("N", SPREAD)}
    for slug, over in (("class-a-L2", {"L": 2.0, "alpha": 0.2, "lambda_0": 0.1}), ("class-a-L4", {"L": 4.0, "alpha": 0.2, "lambda_0": 0.1}), ("class-b-L4", {"L": 4.0, "alpha": 0.32, "lambda_0": 0.0}), ("stocky-plateau", {"L": 0.8, "alpha": 0.2, "lambda_0": 0.1})):
        cases.append(case("en1999", f"6.3.1.flexural-buckling-{slug}", "en1999.6.3.1.flexural-buckling", "6.3.1.2 Eq. 6.49 to 6.51 flexural buckling of an extruded I section, buckling curve parameters of Table 6.6", sec | over, out_n, [ref(SP), ref(SY, True), ref(NU, True)], annex="en", table="6.6"))
    out_l = {"Iz": ("m^4", SPREAD), "It": ("m^4", SPREAD), "Iw": ("m^6", SPREAD), "M_cr": ("N*m", SPREAD), "lambda_LT": ("1", SPREAD), "chi_LT": ("1", SPREAD), "M_b_Rd": ("N*m", SPREAD)}
    for slug, over in (("class-2-L2", {"L": 2.0}), ("class-2-L4", {"L": 4.0}), ("class-3-L6", {"L": 6.0, "alpha_LT": 0.2, "lambda_0": 0.4})):
        cases.append(case("en1999", f"6.3.2.ltb-{slug}", "en1999.6.3.2.ltb", "6.3.2.2 Eq. 6.56 to 6.58 lateral-torsional buckling with M_cr of a simply supported beam under uniform moment (fork supports)", sec | {"alpha_LT": 0.1, "lambda_0": 0.6, "k_s": 0.0} | over, out_l, [ref(SP), ref(SY, True), ref(NU, True)], annex="en", table="6.7"))
    for slug, over in (("equal-plates", {"F": 50e3, "a": 0.02, "b": 0.02, "c": 0.002}), ("thick-inner", {"F": 80e3, "a": 0.015, "b": 0.04, "c": 0.003})):
        cases.append(case("en1999", f"8.5.pin-bending-moment-{slug}", "en1999.8.5.pin-bending-moment", "8.5.10 pin in double shear: design bending moment M_Ed = F (b + 4 c + 2 a) / 8 with uniform bearing pressures (EN 1993-1-8 Table 3.10 note)", over, {"M_Ed": ("N*m", NUM)}, [ref(AN), ref(PN), ref(NU, True)], annex="en"))
    write("en1999", "third-party-v2-extended", cases)


if __name__ == "__main__":
    for fn in (en1990, en1991, en1992, en1993, en1994, en1996, en1997, en1998, en1999):
        fn()
