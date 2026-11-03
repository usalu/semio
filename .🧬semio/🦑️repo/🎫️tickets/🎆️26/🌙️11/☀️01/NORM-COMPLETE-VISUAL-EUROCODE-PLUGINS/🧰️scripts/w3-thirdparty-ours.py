"""W3-THIRDPARTY: our-side mirror of the Rust formulas (transcribed from the Rust sources named per function) against the third-party blessed expectations of the oracle case files; prints every quantity whose relative difference exceeds REPORT and the maximum per check.

Run: /home/deck/semio/.venv/bin/python -I w3-thirdparty-ours.py
"""

import json
import math
import pathlib

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases"
NDP = {e["id"]: e for e in json.loads((ROOT / "⚖️compliance/📜️annex-and-tables/🔖️ndp/🔣️.json").read_text(encoding="utf8"))["entries"]}
REPORT = 1.0e-4
CUBE = {12: 15, 16: 20, 20: 25, 25: 30, 30: 37, 35: 45, 40: 50, 45: 55, 50: 60, 55: 67, 60: 75, 70: 85, 80: 95, 90: 105}
TABLE = {12: (1.6, 1.1, 2.0), 16: (1.9, 1.3, 2.5), 20: (2.2, 1.5, 2.9), 25: (2.6, 1.8, 3.3), 30: (2.9, 2.0, 3.8), 35: (3.2, 2.2, 4.2), 40: (3.5, 2.5, 4.6), 45: (3.8, 2.7, 4.9), 50: (4.1, 2.9, 5.3), 55: (4.2, 3.0, 5.5), 60: (4.4, 3.1, 5.7), 70: (4.6, 3.2, 6.0), 80: (4.8, 3.4, 6.3), 90: (5.0, 3.5, 6.6)}


def ndp(i, annex):
    e = NDP[i]
    return e["naDe"] if annex == "de" else e["recommended"]


def conc(fck):
    """materials-and-time::conc"""
    fcm, hi = fck + 8.0, fck > 50.0
    fctm = 0.3 * fck ** (2 / 3) if fck <= 50 else 2.12 * math.log(1 + fcm / 10)
    q = max((90.0 - fck) / 100.0, 0.0) ** 4
    row = TABLE.get(int(fck)) if abs(fck - int(fck)) < 1e-9 else None
    ecm = NDP[f"ndp.en1992.t3-1.ecm.c{int(fck)}-{CUBE[int(fck)]}"]["recommended"] * 1000.0 if row and f"ndp.en1992.t3-1.ecm.c{int(fck)}-{CUBE[int(fck)]}" in NDP else 22000.0 * (fcm / 10.0) ** 0.3
    return {"fcm": fcm, "fctm": row[0] if row else fctm, "fctk_5": row[1] if row else 0.7 * fctm, "fctk_95": row[2] if row else 1.3 * fctm, "Ecm": ecm, "eps_c1": min(0.7 * fcm**0.31, 2.8) / 1e3, "eps_cu1": (2.8 + 27.0 * ((98.0 - fcm) / 100.0) ** 4 if hi else 3.5) / 1e3, "eps_c2": (2.0 + 0.085 * (fck - 50.0) ** 0.53 if hi else 2.0) / 1e3, "eps_cu2": (2.6 + 35.0 * q if hi else 3.5) / 1e3, "n": 1.4 + 23.4 * q if hi else 2.0}


def laws(i):
    c = conc(i["fck"] / 1e6)
    out = {k: v * 1e6 for k, v in c.items() if k in ("fcm", "fctm", "fctk_5", "fctk_95", "Ecm")} | {k: c[k] for k in ("eps_c1", "eps_cu1", "eps_c2", "eps_cu2", "n")}
    return out | {"fcd": i["alpha_cc"] * i["fck"] / i["gamma_c"]}


def fcd(i):
    return {"f_cd": i["alpha_cc"] * i["f_ck"] / i["gamma_c"]}


def annex_of(i, key, value):
    return next(a for a in ("en", "de") if abs(ndp(key, a) - value) < 1e-9)


def vrdc(i):
    """shear-torsion::v_rd_c; annex found from C_Rd,c x gamma_c as the wiring does."""
    annex = annex_of(i, "ndp.en1992.crd-c-shear", i["C_Rdc"] * i["gamma_c"])
    fck, bw, d, asl, scp = i["f_ck"] / 1e6, i["b_w"] * 1e3, i["d"] * 1e3, i["A_sl"] * 1e6, i["sigma_cp"] / 1e6
    gc = ndp("ndp.en1992.m-01.gamma-c-persistent", annex)
    k = min(1 + math.sqrt(200 / d), 2.0)
    rho = min(asl / (bw * d), 0.02)
    fcd_ = ndp("ndp.en1992.m-01.alpha-cc-persistent", annex) * fck / gc
    scp = min(scp, 0.2 * fcd_)
    crd = ndp("ndp.en1992.crd-c-shear", annex) / gc
    if annex == "en":
        vmc = ndp("ndp.en1992.vmin-shear", "en")
    else:
        x0, y0, x1, y1 = 600.0, ndp("ndp.en1992.vmin-shear", "de") / gc, 800.0, ndp("ndp.en1992.vmin-shear-d800", "de") / gc
        vmc = y0 if d <= x0 else y1 if d >= x1 else y0 + (d - x0) / (x1 - x0) * (y1 - y0)
    k1 = ndp("ndp.en1992.k1-shear", annex)
    v = max(crd * k * (100 * rho * fck) ** (1 / 3) + k1 * scp, vmc * k**1.5 * math.sqrt(fck) + k1 * scp)
    return {"V_Rd_c": v * bw * d}


def links(i):
    """part_1_1::shear_v_rd_max_n, shear_v_rd_s_n and shear_torsion::v_ed_max_unreinforced (EN annex)."""
    annex = annex_of(i, "ndp.en1992.m-01.alpha-cc-persistent", i["alpha_cc"])
    fck, cot = i["f_ck"] / 1e6, i["cot_theta"]
    fcd_ = ndp("ndp.en1992.m-01.alpha-cc-persistent", annex) * fck / ndp("ndp.en1992.m-01.gamma-c-persistent", annex)
    nu = 0.6 * (1 - fck / 250) if annex == "en" else None
    fywd = i["fyk"] / 1e6 / ndp("ndp.en1992.gamma-s", annex)
    return {"V_Rd_max": 1.0 * i["b"] * 1e3 * i["z"] * 1e3 * nu * fcd_ * cot / (1 + cot * cot), "V_Rd_s": i["Asw"] / i["s"] * 1e3 * i["z"] * 1e3 * fywd * cot, "V_Ed_max_unreinforced": 0.5 * i["b"] * 1e3 * i["d"] * 1e3 * nu * fcd_}


def creep(i):
    """materials-and-time::creep_terms and shrinkage (Rust adjusts t0 for both beta(t0) and beta_c)."""
    fck, rh, t0, t, ts = i["fck"] / 1e6, i["RH"], i["t0"], i["t"], i["t_s"]
    cem = "SNR"[int(i["cement_class"])]
    h0 = 2 * i["Ac"] / i["u"]
    h0mm, fcm = h0 * 1e3, fck + 8
    base = (1 - rh / 100) / (0.1 * h0mm ** (1 / 3))
    phi_rh = (1 + base * (35 / fcm) ** 0.7) * (35 / fcm) ** 0.2 if fcm > 35 else 1 + base
    a3 = (35 / fcm) ** 0.5 if fcm > 35 else 1.0
    bh = min(1.5 * (1 + (0.012 * rh) ** 18) * h0mm + 250 * a3, 1500 * a3)
    alpha = {"S": -1.0, "R": 1.0}.get(cem, 0.0)
    t0a = max(t0 * (9 / (2 + t0**1.2) + 1) ** alpha, 0.5)
    phi0 = phi_rh * 16.8 / math.sqrt(fcm) / (0.1 + t0a**0.2)
    phi = phi0 * ((t - t0a) / (bh + t - t0a)) ** 0.3
    phi_b5_only = phi0 * ((t - t0) / (bh + t - t0)) ** 0.3
    ds = {"S": (3, 0.13), "N": (4, 0.12), "R": (6, 0.11)}[cem]
    fcmb = fcm
    ecd0 = 0.85 * ((220 + 110 * ds[0]) * math.exp(-ds[1] * fcmb / 10)) * 1e-6 * (1.55 * (1 - (rh / 100) ** 3))
    kh = float(__import__("numpy").interp(h0mm, [100, 200, 300, 500], [1.0, 0.85, 0.75, 0.70]))
    ecd = (t - ts) / ((t - ts) + 0.04 * h0mm**1.5) * kh * ecd0
    eca = (1 - math.exp(-0.2 * math.sqrt(t))) * 2.5 * (fck - 10) * 1e-6
    return {"h_0": h0, "phi_RH": phi_rh, "phi_0": phi0, "phi": phi, "eps_cd_0": ecd0, "eps_cd": ecd, "eps_ca": eca, "eps_cs": ecd + eca, "_phi_b5_only": phi_b5_only}


def nq(p):
    r = math.radians(p)
    return math.exp(math.pi * math.tan(r)) * math.tan(math.pi / 4 + r / 2) ** 2


def bearing_factors(i):
    p = i["phi"]
    return {"Nq": nq(p), "Nc": (nq(p) - 1) / math.tan(math.radians(p)), "Ng_ec7": 2 * (nq(p) - 1) * math.tan(math.radians(p))}


def earth_pressure(i):
    p, d = math.radians(i["phi"]), math.radians(i["delta"])
    s = math.sin(p)
    out = {"Ka_rankine": (1 - s) / (1 + s), "Kp_rankine": (1 + s) / (1 - s)}
    if i["delta"] > 0:
        root_a = math.sqrt(math.sin(p + d) * math.sin(p) / math.cos(d))
        root_p = math.sqrt(math.sin(p + d) * math.sin(p) / math.cos(d))
        out |= {"Ka_coulomb": math.cos(p) ** 2 / (math.cos(d) * (1 + root_a) ** 2), "Kp_coulomb": math.cos(p) ** 2 / (math.cos(d) * (1 - root_p) ** 2)}
    return out


def cpt_phi(i):
    return {"phi_deg": 17.6 + 11 * math.log10((i["qc"] / 1e5) / math.sqrt(i["sigma_v0_eff"] / 1e5))}


def cpt_su(i):
    return {"s_u": i["q_net"] / i["N_k"]}


def cpt_ic(i):
    qn, fn = (i["qt"] - i["sigma_v0"]) / i["sigma_v0_eff"], i["fs"] / (i["qt"] - i["sigma_v0"]) * 100
    return {"Ic": math.sqrt((3.47 - math.log10(qn)) ** 2 + (math.log10(fn) + 1.22) ** 2)}


def spt(i):
    n60 = i["n"] * i["er"] / 60 * i["cb"] * i["cs"] * i["cr"]
    cn = min(math.sqrt(1e5 / i["sigma_v0_eff"]), 1.7)
    return {"n60": n60, "n1_60": n60 * cn}


def phase(i):
    g = (i["G_s"] + i["S_r"] * i["e"]) * i["gamma_w"] / (1 + i["e"])
    return {"gamma": g, "gamma_eff": g - i["gamma_w"]}


MIRROR = {"en1992.3.1.concrete-laws": laws, "en1992.3.15.fcd": fcd, "en1992.6.2.shear-vrdc": vrdc, "en1992.6.2.shear-links": links, "en1992.3.1.creep-shrinkage": creep, "en1997.D.4.bearing-factors": bearing_factors, "en1997.9.earth-pressure": earth_pressure, "en1997.2.cpt-friction-angle": cpt_phi, "en1997.2.cpt-undrained-strength": cpt_su, "en1997.2.cpt-behaviour-index": cpt_ic, "en1997.2.spt-correction": spt, "en1997.2.phase-relations": phase}


def main():
    stats = {}
    for art, folder in (("en1992", "🏛️en1992"), ("en1997", "🌍️en1997")):
        for f in sorted((ROOT / "🗿️artifacts" / folder / SUB).glob("third-party*.json")):
            for c in json.loads(f.read_text(encoding="utf8"))["cases"]:
                m = MIRROR.get(c["check"])
                if not m:
                    continue
                ours = m(c["inputs"])
                for q, e in c["expected"].items():
                    if q not in ours:
                        continue
                    want, got = e["value"], ours[q]
                    rel = abs(want - got) / max(abs(want), abs(got), 1e-300)
                    tol = e.get("tolerance", {"eurocode-quantities-v1": 1e-4, "numerical-v1": 5e-3, "method-spread-v1": 3e-2}[e["profile"]])
                    s = stats.setdefault(c["check"], [0, 0.0, 0])
                    s[0] += 1
                    s[1] = max(s[1], rel)
                    s[2] += rel > tol
                    if rel > min(REPORT, tol) or rel > tol:
                        print(f"{'OVER' if rel > tol else 'near'} {f.name} {c['id']} {q}: library {want!r} ours {got!r} rel {rel:.3e} tol {tol}")
                if "_phi_b5_only" in ours:
                    print(f"info {c['id']}: phi adjusted-both {ours['phi']:.6f} vs B.5-only {ours['_phi_b5_only']:.6f} rel {abs(ours['phi'] - ours['_phi_b5_only']) / ours['phi']:.2e}")
    for check, (n, mx, over) in sorted(stats.items()):
        print(f"SUMMARY {check}: {n} quantities, max rel {mx:.3e}, over tolerance {over}")


main()
