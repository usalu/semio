"""W3-THIRDPARTY: generates third-party-w3.json for EN 1992 and EN 1997 (blessed by the first reference through the oracle harness) and prints a mirror comparison of the pure-formula checks with our Rust formulas.

Run: /home/deck/semio/.venv/bin/python -I w3-thirdparty-gen.py [--write]
"""

import importlib.util
import json
import pathlib
import sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
HARNESS = ROOT / "🔮️oracles/🏃️harness/🐍️.py"
SPEC = importlib.util.spec_from_file_location("h", HARNESS)
H = importlib.util.module_from_spec(SPEC)
sys.modules["h"] = H
SPEC.loader.exec_module(H)

SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"
OUT = {"en1992": ROOT / "🗿️artifacts/🏛️en1992" / SUB / "third-party-w3.json", "en1997": ROOT / "🗿️artifacts/🌍️en1997" / SUB / "third-party-w3.json"}
ECQ, NUM, SPREAD = "eurocode-quantities-v1", "numerical-v1", "method-spread-v1"
SC = {"oracle": "norm-eurocode-structuralcodes", "adapter": "structuralcodes", "kind": "third-party-library"}
CP = {"oracle": "norm-eurocode-concreteproperties", "adapter": "concreteproperties", "kind": "third-party-library"}
GH = {"oracle": "norm-eurocode-groundhog", "adapter": "groundhog", "kind": "third-party-library"}
NP = {"oracle": "norm-eurocode-scipy-numerics", "adapter": "numerical", "kind": "independent-formulation"}
SY = {"oracle": "norm-eurocode-sympy", "adapter": "sympy", "kind": "independent-formulation"}
EC2 = "EN 1992-1-1"
EC7 = "EN 1997-1"
EC72 = "EN 1997-2"


def quantity(unit, profile=ECQ, tolerance=None):
    return {"unit": unit, "profile": profile} | ({"tolerance": tolerance} if tolerance else {})


def case(cid, check, standard, clause, inputs, shape, refs, annex=None, table=None):
    citation = {"standard": standard, "edition": "2004", "clause": clause} | ({"annex": annex} if annex else {}) | ({"table": table} if table else {})
    return {"id": cid, "check": check, "citation": citation, "inputs": inputs, "expected": {k: dict(v, value=0.0) for k, v in shape.items()}, "references": refs}


LAWS = {"fcm": quantity("Pa"), "fctm": quantity("Pa", ECQ, 0.02), "fctk_5": quantity("Pa", ECQ, 0.04), "fctk_95": quantity("Pa", ECQ, 0.03), "Ecm": quantity("Pa", ECQ, 0.02), "fcd": quantity("Pa"), "eps_c1": quantity("1"), "eps_cu1": quantity("1", ECQ, 0.003), "eps_c2": quantity("1"), "eps_cu2": quantity("1"), "n": quantity("1")}
CREEP = {"h_0": quantity("m", NUM)} | {k: quantity("1") for k in ("phi_RH", "phi_0", "phi", "eps_cd_0", "eps_cd", "eps_ca", "eps_cs")}
MPA = 1.0e6


def high_strength(fck):
    """Table 3.1 parabola-rectangle parameters above C50/60 for the numerical reference (eps_c2, eps_cu2, n from the Table 3.1 formulas)."""
    q = ((90.0 - fck) / 100.0) ** 4
    return {"eps_c2": (2.0 + 0.085 * (fck - 50.0) ** 0.53) * 1e-3, "eps_cu2": (2.6 + 35.0 * q) * 1e-3, "n": 1.4 + 23.4 * q} if fck > 50 else {}


def en1992():
    cases = []
    for k, fck in enumerate((12, 16, 20, 25, 35, 40, 45, 50, 55, 70, 80)):
        cases.append(case(f"en1992.3.1.laws-c{fck}", "en1992.3.1.concrete-laws", EC2, "3.1.3 Table 3.1; 3.1.7; 3.15; fctm, fctk_5, fctk_95 and Ecm: the standard prints the rounded Table 3.1 row, the library evaluates the formulas (tolerance 2 percent, Ecm 1 percent)", {"fck": fck * MPA, "alpha_cc": 0.85 if k % 2 == 0 else 1.0, "gamma_c": 1.5}, LAWS, [SC], table="3.1"))
    for fck, a, g in ((25, 1.0, 1.5), (50, 0.85, 1.5), (90, 1.0, 1.5), (30, 1.0, 1.2)):
        cases.append(case(f"en1992.3.15.fcd-c{fck}-a{int(a * 100)}-g{int(g * 100)}", "en1992.3.15.fcd", EC2, "3.1.6 Eq. 3.15", {"f_ck": fck * MPA, "alpha_cc": a, "gamma_c": g}, {"f_cd": quantity("Pa")}, [SC]))
    vrdc = (("vmin-governs", 30, 0.45, 0.3, 0.0003, 0.0), ("high-strength-prestress", 50, 0.6, 0.4, 0.0024, 2.0), ("sigma-cp-capped", 25, 0.5, 0.3, 0.0016, 10.0), ("axial-tension", 30, 0.45, 0.3, 0.0012, -1.0), ("large-depth", 35, 0.9, 0.5, 0.0045, 0.0), ("k-capped-thin", 30, 0.1, 1.0, 0.0008, 0.0))
    for name, fck, d, bw, asl, scp in vrdc:
        for tag, crd, k1 in (("en", 0.12, 0.15), ("de", 0.1, 0.12)):
            if tag == "de" and d > 0.6:
                continue
            cases.append(case(f"en1992.6.2.vrdc-{name}-{tag}-constants", "en1992.6.2.shear-vrdc", EC2, "6.2.2(1) Eq. 6.2a, 6.2b, 6.3N", {"f_ck": fck * MPA, "d": d, "b_w": bw, "A_sl": asl, "sigma_cp": scp * MPA, "alpha_cc": 1.0 if tag == "en" else 0.85, "gamma_c": 1.5, "C_Rdc": crd, "k1": k1}, {"V_Rd_c": quantity("N")}, [SC], annex=tag))
    for fck, cot, asw, s in ((20, 1.0, 0.000226, 0.15), (35, 1.75, 0.000157, 0.2), (50, 1.5, 0.000226, 0.15), (50, 2.0, 0.000113, 0.2), (70, 2.0, 0.000226, 0.2), (90, 2.5, 0.000157, 0.15), (35, 2.5, 0.000157, 0.2), (35, 1.0, 0.000157, 0.2)):
        cases.append(case(f"en1992.6.2.links-c{fck}-cot{cot}", "en1992.6.2.shear-links", EC2, "6.2.3(2) Eq. 6.6N, 6.7N cot theta in [1, 2.5], Eq. 6.8, 6.9; 6.2.2 Eq. 6.5", {"f_ck": fck * MPA, "b": 0.3, "z": 0.405, "d": 0.45, "Ac": 0.15, "cot_theta": cot, "Asw": asw, "s": s, "fyk": 500.0 * MPA, "gamma_s": 1.15, "alpha_cc": 1.0, "gamma_c": 1.5}, {"V_Rd_max": quantity("N"), "V_Rd_s": quantity("N"), "V_Ed_max_unreinforced": quantity("N")}, [SC], annex="en"))
    for name, fck, b, h, d, asl, a in (("c20", 20, 0.25, 0.5, 0.45, 0.0009, 0.85), ("c40", 40, 0.3, 0.6, 0.55, 0.002, 1.0), ("c50-heavy", 50, 0.3, 0.55, 0.5, 0.003, 0.85), ("c70", 70, 0.3, 0.6, 0.55, 0.0025, 0.85), ("c90", 90, 0.35, 0.6, 0.55, 0.003, 1.0)):
        cases.append(case(f"en1992.6.1.flexure-{name}", "en1992.6.1.flexure-rect", EC2, "6.1; 3.1.7(1) Figure 3.3", {"fck": fck * MPA, "fyk": 500.0 * MPA, "b": b, "h": h, "d": d, "As": asl, "gamma_c": 1.5, "gamma_s": 1.15, "alpha_cc": a} | high_strength(fck), {"M_Rd": quantity("N*m", NUM)}, [SC, CP, NP]))
    for name, fck, rh, ac, u, cem, t0, t, ts in (("c35-rh80", 35, 80.0, 0.3, 2.0, 1, 7.0, 36500.0, 7.0), ("c20-rh40-slow", 20, 40.0, 0.1, 1.2, 0, 14.0, 18250.0, 3.0), ("c60-rh60", 60, 60.0, 0.2, 1.6, 1, 28.0, 18250.0, 3.0), ("c45-rh90-rapid", 45, 90.0, 0.25, 1.5, 2, 3.0, 10000.0, 7.0), ("c30-rh50-rapid-late", 30, 50.0, 0.15, 1.6, 2, 90.0, 10000.0, 3.0), ("c80-rh70", 80, 70.0, 0.3, 1.8, 2, 28.0, 18250.0, 3.0), ("c30-rh60-slow-short-term", 30, 60.0, 0.15, 1.6, 0, 28.0, 60.0, 3.0)):
        cases.append(case(f"en1992.3.1.creep-shrinkage-{name}", "en1992.3.1.creep-shrinkage", EC2, "3.1.4 and Annex B Eq. B.1-B.13; 3.1.4(6) Eq. 3.8-3.12", {"fck": fck * MPA, "RH": rh, "Ac": ac, "u": u, "cement_class": cem, "t0": t0, "t": t, "t_s": ts}, CREEP, [SC], table="B"))
    return cases


def en1997():
    cases = []
    for phi in (22.0, 25.0, 28.0, 32.5, 40.0, 45.0, 50.0):
        cases.append(case(f"en1997.D.4.bearing-factors-phi{phi:g}", "en1997.D.4.bearing-factors", EC7, "Annex D.4 (D.2-D.4)", {"phi": phi}, {"Nq": quantity("1"), "Nc": quantity("1"), "Ng_ec7": quantity("1")}, [GH]))
    for phi, delta in ((25.0, 0.0), (25.0, 16.7), (28.0, 18.7), (32.5, 21.7), (40.0, 26.7), (40.0, 0.0)):
        shape = {"Ka_rankine": quantity("1"), "Kp_rankine": quantity("1")} | ({"Ka_coulomb": quantity("1"), "Kp_coulomb": quantity("1", SPREAD)} if delta > 0.0 else {})
        cases.append(case(f"en1997.9.earth-pressure-phi{phi:g}-delta{delta:g}", "en1997.9.earth-pressure", EC7, "9.5 and Annex C (limit earth pressure coefficients)", {"phi": phi, "delta": delta}, shape, [GH], table="C"))
    for qc, sv in ((2.0e6, 50.0e3), (5.0e6, 100.0e3), (15.0e6, 200.0e3), (30.0e6, 300.0e3), (1.0e6, 30.0e3)):
        cases.append(case(f"en1997.2.cpt-phi-qc{qc / 1e6:g}-sv{sv / 1e3:g}", "en1997.2.cpt-friction-angle", EC7, "2.4.3 and 10.2 derived values from field tests (Kulhawy and Mayne 1990)", {"qc": qc, "sigma_v0_eff": sv}, {"phi_deg": quantity("deg")}, [GH]))
    for qnet, nk in ((0.5e6, 15.0), (2.0e6, 20.0), (0.8e6, 12.0), (3.0e6, 10.0)):
        cases.append(case(f"en1997.2.cpt-su-qnet{qnet / 1e6:g}-nk{nk:g}", "en1997.2.cpt-undrained-strength", EC7, "2.4.3 derived values from field tests (Rad and Lunne 1988)", {"q_net": qnet, "N_k": nk}, {"s_u": quantity("Pa")}, [GH]))
    for k, (qt, fs, s0, se) in enumerate(((0.5e6, 10e3, 100e3, 60e3), (2.0e6, 40e3, 200e3, 120e3), (20.0e6, 150e3, 100e3, 100e3), (0.2e6, 5e3, 50e3, 30e3))):
        cases.append(case(f"en1997.2.cpt-behaviour-index-{k + 1}", "en1997.2.cpt-behaviour-index", EC7, "2.4.3 derived values from field tests (Robertson and Wride 1998)", {"qt": qt, "fs": fs, "sigma_v0": s0, "sigma_v0_eff": se}, {"Ic": quantity("1", SPREAD)}, [GH]))
    for k, (n, er, cb, cs, cr, sv) in enumerate(((25.0, 60.0, 1.0, 1.0, 1.0, 100e3), (40.0, 72.0, 1.05, 0.9, 0.95, 50e3), (12.0, 45.0, 1.15, 1.0, 0.85, 200e3), (30.0, 78.0, 1.0, 1.0, 0.75, 150e3), (50.0, 60.0, 1.0, 0.8, 1.0, 80e3), (20.0, 60.0, 1.0, 1.0, 1.0, 25e3))):
        cases.append(case(f"en1997.2.spt-correction-{k + 1}", "en1997.2.spt-correction", EC72, "Annex F SPT: N60 = N E_r/60 C_B C_S C_R (Skempton 1986) and (N1)60 = C_N N60 with C_N = sqrt(p_a/sigma'_v0) (Liao and Whitman 1986)", {"n": n, "er": er, "cb": cb, "cs": cs, "cr": cr, "sigma_v0_eff": sv}, {"n60": quantity("1"), "n1_60": quantity("1")}, [GH]))
    for k, (sr, e, gs) in enumerate(((0.8, 0.6, 2.65), (1.0, 1.1, 2.7), (0.0, 0.5, 2.65))):
        cases.append(case(f"en1997.2.phase-relations-{k + 1}", "en1997.2.phase-relations", EC7, "2.4.3 phase relations of soil", {"S_r": sr, "e": e, "G_s": gs, "gamma_w": 9810.0}, {"gamma": quantity("N/m3"), "gamma_eff": quantity("N/m3")}, [GH]))
    base = {"c": 0.0, "q": 0.0, "alpha": 0.0}
    drained = {"N_q": "1", "N_c": "1", "N_gamma": "1", "s_q": "1", "s_gamma": "1", "s_c": "1", "i_q": "1", "i_gamma": "1", "i_c": "1", "b_q": "1", "q_ult": "Pa", "R_k": "N"}
    for name, extra in (("sand-strip-surcharge", {"phi": 35.0, "c": 0.0, "B": 2.0, "L": 20.0, "gamma": 19000.0, "q": 15000.0, "V": 800000.0, "H": 0.0, "alpha": 0.0}), ("silty-sand-eccentric-load", {"phi": 28.0, "c": 5000.0, "B": 3.0, "L": 4.5, "gamma": 18000.0, "q": 30000.0, "V": 1500000.0, "H": 150000.0, "alpha": 0.0})):
        cases.append(case(f"en1997.D.4.bearing-resistance-{name}", "en1997.D.4.bearing-resistance", EC7, "Annex D.4 drained bearing resistance: N factors, shape s, base inclination b, load inclination i", base | extra, {k: quantity(u) for k, u in drained.items()}, [GH, NP, SY], annex="en"))
    return cases


def document(artifact, cases):
    return {"$schema": "../../../../../../../../../🔮️oracles/🏃️harness/🧬️schema/🔣️.json", "schemaVersion": 1, "artifact": artifact, "annex": "de", "cases": cases}


def main(argv):
    write = "--write" in argv
    rc = 0
    for artifact, cases in (("en1992", en1992()), ("en1997", en1997())):
        doc = document(artifact, cases)
        H.validate(doc)
        results = H.run(doc, bless=True)
        for r in results:
            if r["failures"]:
                rc = 1
                print("SPREAD", r["id"], *r["failures"], sep="\n    ")
        print(artifact, len(cases), "cases", sum(1 for r in results if not r["failures"]), "agree")
        if write:
            OUT[artifact].write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
