"""🔍️ Compares the Rust formulas (transcribed line by line, NOT executed: no build slot) with the blessed third-party library values of every case file; prints a markdown table of every quantity beyond its profile.

Usage: /home/deck/semio/.venv/bin/python -I oracle-third-party-diff.py > ../🗑️generated/oracles/diff.md
"""

import json
import math
import re
import sys
from pathlib import Path

NORM = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
ART = NORM / "🗿️artifacts"
LEAF = "🏅️standards/🔖️1/🪆️subsets/✳️any"
sys.path.insert(0, str(NORM / "🔮️oracles/🏃️harness"))
TOL = {"closed-form-v1": 1e-9, "eurocode-quantities-v1": 1e-4, "numerical-v1": 5e-3, "method-spread-v1": 3e-2, "table-exact-v1": 0.0}
ROWS = []


def cases(art, name):
    folder = next(ART.glob(f"*{art}"))
    return {c["id"]: c for c in json.loads((folder / LEAF / "🧪️tests/🧫️fixtures/🔮️oracle-cases" / f"{name}.json").read_text(encoding="utf-8"))["cases"]}


def compare(area, case, ours, rust_ref, tol=None):
    """⚖️ Record every quantity of `case` that `ours` (dict) computes."""
    for name, want in case["expected"].items():
        if name not in ours:
            continue
        lib, mine = want["value"], ours[name]
        scale = max(abs(lib), abs(mine), 1e-300)
        rel = abs(lib - mine) / scale
        limit = tol if tol is not None else TOL[want["profile"]]
        ROWS.append((area, case["id"], name, lib, mine, rel, limit, "ok" if rel <= limit or abs(lib - mine) <= 1e-9 else "DISAGREE", rust_ref))


R = math.radians
EN1992 = "🏛️en1992/" + LEAF + "/🧬️schema/💡️inferences/⚖️checks"


def conc(fck):
    """📈️ `🧱️materials-and-time/🦀️.rs:23` conc(): Table 3.1 row for a standard grade, formulas otherwise (MPa)."""
    T = {12: (1.6, 1.1, 2.0, 27.0), 16: (1.9, 1.3, 2.5, 29.0), 20: (2.2, 1.5, 2.9, 30.0), 25: (2.6, 1.8, 3.3, 31.0), 30: (2.9, 2.0, 3.8, 33.0), 35: (3.2, 2.2, 4.2, 34.0), 40: (3.5, 2.5, 4.6, 35.0), 45: (3.8, 2.7, 4.9, 36.0), 50: (4.1, 2.9, 5.3, 37.0), 55: (4.2, 3.0, 5.5, 38.0), 60: (4.4, 3.1, 5.7, 39.0), 70: (4.6, 3.2, 6.0, 41.0), 80: (4.8, 3.4, 6.3, 42.0), 90: (5.0, 3.5, 6.6, 44.0)}
    fcm = fck + 8.0
    tab = T.get(int(fck))
    fctm = 0.30 * fck ** (2 / 3) if fck <= 50 else 2.12 * math.log(1 + fcm / 10)
    hi = fck > 50
    q = max((90 - fck) / 100, 0) ** 4
    return {"fcm": fcm, "fctm": tab[0] if tab else fctm, "fctk_5": tab[1] if tab else 0.7 * fctm, "fctk_95": tab[2] if tab else 1.3 * fctm, "Ecm": tab[3] * 1000 if tab else 22000 * (fcm / 10) ** 0.3, "eps_c1": min(0.7 * fcm**0.31, 2.8), "eps_c2": 2.0 + 0.085 * (fck - 50) ** 0.53 if hi else 2.0, "eps_cu2": 2.6 + 35 * q if hi else 3.5, "n": 1.4 + 23.4 * q if hi else 2.0}


def v_rd_c(annex, fck, bw, d, asl, scp, crd_num=None, k1=None):
    """✂️ `✂️shear-torsion/🦀️.rs:5` v_rd_c(): Eq. 6.2a/b with the NDP rows of 📜️ndp-tables (EN 0.18/0.035/0.15, DE 0.15/0.0525/0.12)."""
    gc, fcd = 1.5, (0.85 if annex == "de" else 1.0) * fck / 1.5
    k = min(1 + math.sqrt(200 / d), 2.0)
    rho = min(asl / (bw * d), 0.02)
    scp = min(scp, 0.2 * fcd)
    crd = (0.15 if annex == "de" else 0.18) / gc
    vmin_c = 0.0525 / gc if annex == "de" else 0.035
    vmin = vmin_c * k**1.5 * math.sqrt(fck)
    k1 = 0.12 if annex == "de" else 0.15
    return max(crd * k * (100 * rho * fck) ** (1 / 3) + k1 * scp, vmin + k1 * scp) * bw * d


def en1992():
    c = cases("en1992", "third-party")
    for fck in (30, 60, 90):
        lib = c[f"en1992.3.1.laws-c{fck}"]
        o = conc(float(fck))
        ours = {"fcm": o["fcm"] * 1e6, "fctm": o["fctm"] * 1e6, "fctk_5": o["fctk_5"] * 1e6, "fctk_95": o["fctk_95"] * 1e6, "Ecm": o["Ecm"] * 1e6, "eps_c1": o["eps_c1"] / 1e3, "eps_c2": o["eps_c2"] / 1e3, "eps_cu2": o["eps_cu2"] / 1e3, "n": o["n"], "fcd": 0.85 * fck * 1e6 / 1.5}
        compare("EN 1992 Table 3.1", lib, ours, "🧱️materials-and-time/🦀️.rs:23-50")
    for cid, cs in c.items():
        if cid.startswith("en1992.6.2.vrdc-"):
            i, annex = cs["inputs"], "de" if cid.endswith("de-constants") else "en"
            ours = {"V_Rd_c": v_rd_c(annex, i["f_ck"] / 1e6, i["b_w"] * 1e3, i["d"] * 1e3, i["A_sl"] * 1e6, i["sigma_cp"] / 1e6) * 1.0}
            compare("EN 1992 6.2.2", cs, ours, "✂️shear-torsion/🦀️.rs:5-19")
    for cid, cs in c.items():
        if cid.startswith("en1992.6.2.links-"):
            i = cs["inputs"]
            fck, bw, z, cot = i["f_ck"] / 1e6, i["b"] * 1e3, i["z"] * 1e3, i["cot_theta"]
            nu, fcd = 0.6 * (1 - fck / 250), fck / 1.5
            ours = {"V_Rd_max": 1.0 * bw * z * nu * fcd * cot / (1 + cot * cot), "V_Rd_s": i["Asw"] * 1e6 / (i["s"] * 1e3) * z * (i["fyk"] / 1e6 / 1.15) * cot, "V_Ed_max_unreinforced": 0.5 * bw * i["d"] * 1e3 * nu * fcd}
            compare("EN 1992 6.2.3", cs, ours, "✂️shear-torsion/🦀️.rs:58-66")
    for cid in ("en1992.7.16.span-depth", "en1992.7.16.span-depth-compression"):
        i = c[cid]["inputs"]
        fck, rho, rp, k = i["f_ck"] / 1e6, i["rho"], i["rho_prime"], i["capital_k"]
        s = math.sqrt(fck)
        rho0 = s * 1e-3
        basic = 11 + 1.5 * s * rho0 / rho + 3.2 * s * (rho0 / rho - 1) ** 1.5 if rho <= rho0 else 11 + 1.5 * s * rho0 / (rho - rp) + s / 12 * math.sqrt(rp / rho0)
        compare("EN 1992 7.4.2", c[cid], {"l_over_d": k * basic}, "🪟️serviceability/🦀️.rs:215-221")
    i = c["en1992.7.2.cracked-c30"]["inputs"]
    ecm = 22000 * ((i["fck"] / 1e6 + 8) / 10) ** 0.3
    ae, b, d, a, m = i.get("Es", 200e9) / 1e6 / ecm, i["b"] * 1e3, i["d"] * 1e3, i["As"] * 1e6, i["M_Ed"] * 1e3
    x = d * (math.sqrt((ae * a / (b * d)) ** 2 + 2 * ae * a / (b * d)) - ae * a / (b * d))
    i2 = b * x**3 / 3 + ae * a * (d - x) ** 2
    compare("EN 1992 7.2", c["en1992.7.2.cracked-c30"], {"x_cr": x / 1e3, "I_cr": i2 / 1e12, "sigma_c": m * x / i2 * 1e6, "sigma_s": ae * m * (d - x) / i2 * 1e6}, "🪟️serviceability/🦀️.rs:6-22 cracked_state (n = 0)")
    t = c["en1992.6.30.t-rd-max"]["inputs"]
    compare("EN 1992 6.3.2", c["en1992.6.30.t-rd-max"], {"T_Rd_max": 2 * t["nu"] * t["alpha_cw"] * t["f_cd"] * t["a_k"] * t["t_ef_i"] * 0.5}, "🧰️helpers-and-annex-parts/🦀️.rs:191-195 (cot theta = 1)")


def en1993():
    c = cases("en1993", "third-party")
    text = (ART / next(ART.glob("*en1993")).name / LEAF / "🧬️schema/📸️snapshot/🦀️.rs").read_text(encoding="utf-8")
    names = {"area": "A", "iy": "Iy", "iz": "Iz", "it": "It", "iw": "Iw", "w_el_y": "Wel_y", "w_el_z": "Wel_z", "w_pl_y": "Wpl_y", "w_pl_z": "Wpl_z"}
    for m in re.finditer(r"pub fn catalogue_(heb\d+)\(\) -> SteelSection \{\s*SteelSection \{(.*?)\n    \}\n\}", text, re.S):
        values = {k: float(v) for k, v in re.findall(r"(\w+): (-?[0-9.e-]+),", m.group(2))}
        compare("EN 1993 catalogue", c[f"en1993.section.{m.group(1)}"], {names[k]: values[k] for k in names}, "🧬️schema/📸️snapshot/🦀️.rs:787-890 catalogue_" + m.group(1), tol=2e-2)
    mcr = c["en1993.6.3.2.mcr-ipe300-l6-c1-1.0"]["expected"]
    i = c["en1993.6.3.2.mcr-ipe300-l6-c1-1.0"]["inputs"]
    pi2 = math.pi**2
    iz, it, iw = mcr["Iz"]["value"], mcr["It"]["value"], mcr["Iw"]["value"]
    value = 1.0 * pi2 * i["E"] * iz / i["L"] ** 2 * math.sqrt(iw / iz + i["L"] ** 2 * i["G"] * it / (pi2 * i["E"] * iz))
    compare("EN 1993 6.3.2", c["en1993.6.3.2.mcr-ipe300-l6-c1-1.0"], {"M_cr": value}, "🧮️clauses/🧱️stability/🦀️.rs:216 mcr_closed_form (c2 = 0)")
    h = c["en1993.6.2.av-rolled-i-heb240"]["inputs"]
    compare("EN 1993 6.2.6", c["en1993.6.2.av-rolled-i-heb240"], {"A_v": max(h["a"] - 2 * h["b1"] * h["tf1"] + (h["tw"] + 2 * h["r1"]) * h["tf1"], h["eta"] * h["hw"] * h["tw"])}, "🧮️clauses/🧮️resistance/🦀️.rs:27 av_rolled_i")
    r = c["en1993.6.2.av-rhs-200x100x8"]["inputs"]
    compare("EN 1993 6.2.6", c["en1993.6.2.av-rhs-200x100x8"], {"A_v": r["a"] * r["h"] / (r["b"] + r["h"])}, "🧮️clauses/🧮️resistance/🦀️.rs:47 av_rhs")
    compare("EN 1993 6.2.6", c["en1993.6.2.av-chs-139.7x5"], {"A_v": 2 * c["en1993.6.2.av-chs-139.7x5"]["inputs"]["a"] / math.pi}, "🧮️clauses/🧮️resistance/🦀️.rs:51 av_chs")
    u = c["en1993.6.2.av-channel-pfc200"]["inputs"]
    compare("EN 1993 6.2.6", c["en1993.6.2.av-channel-pfc200"], {"A_v": u["a"] - 2 * u["b"] * u["tf"] + (u["tw"] + u["r"]) * u["tf"]}, "🧮️clauses/🧮️resistance/🦀️.rs:31 av_rolled_u")
    v = c["en1993.6.2.my-v-rd-heb240"]["inputs"]
    aw = v["h_w"] * v["t_w"]
    compare("EN 1993 6.2.8", c["en1993.6.2.my-v-rd-heb240"], {"M_y_V_Rd": min((v["w_pl_y"] - v["rho"] * aw * aw / (4 * v["t_w"])) * v["f_y"], v["w_pl_y"] * v["f_y"])}, "🧮️clauses/🧮️resistance/🦀️.rs:65 m_y_v_rd_i")
    n = c["en1993.6.2.my-n-rd-heb240"]["inputs"]
    compare("EN 1993 6.2.9", c["en1993.6.2.my-n-rd-heb240"], {"M_N_y_Rd": min(n["mpl_y_rd"] * (1 - n["n"]) / (1 - 0.5 * n["a"]), n["mpl_y_rd"])}, "🧮️clauses/🧮️resistance/🦀️.rs:84 m_n_y_rd_i")


def en1997():
    c = cases("en1997", "third-party")
    nq = lambda p: math.exp(math.pi * math.tan(R(p))) * math.tan(math.pi / 4 + R(p) / 2) ** 2
    for p in (20, 30, 35):
        q = nq(float(p))
        compare("EN 1997 Annex D", c[f"en1997.D.4.bearing-factors-phi{p}"], {"Nq": q, "Nc": (q - 1) / math.tan(R(p)), "Ng_ec7": 2 * (q - 1) * math.tan(R(p))}, "📇️catalogue/🏛️foundations/🦀️.rs:46-60")
    for cid, cs in c.items():
        if cid.startswith("en1997.9.earth-pressure"):
            p, d = cs["inputs"]["phi"], cs["inputs"]["delta"]
            s = math.sin(R(p))
            out = {"Ka_rankine": (1 - s) / (1 + s), "Kp_rankine": (1 + s) / (1 - s)}
            if d:
                pr, dr = R(p), R(d)
                out["Ka_coulomb"] = math.cos(pr) ** 2 / (math.cos(dr) * (1 + math.sqrt(math.sin(pr + dr) * math.sin(pr) / math.cos(dr))) ** 2)
                out["Kp_coulomb"] = math.cos(pr) ** 2 / (math.cos(dr) * (1 - math.sqrt(math.sin(pr + dr) * math.sin(pr) / math.cos(dr))) ** 2)
            compare("EN 1997 earth pressure", cs, out, "📇️catalogue/🧱️earth-pressure/🦀️.rs:8-26")
    for cid, cs in c.items():
        if cid.startswith("en1997.2.cpt-phi"):
            i = cs["inputs"]
            compare("EN 1997 CPT", cs, {"phi_deg": 17.6 + 11 * math.log10((i["qc"] / 1e5) / math.sqrt(i["sigma_v0_eff"] / 1e5))}, "📐️geotechnical-parts/🦀️.rs:438")


def main():
    en1992()
    en1993()
    en1997()
    print("| area | case | quantity | library | ours | rel | limit | verdict | Rust source |\n|---|---|---|---|---|---|---|---|---|")
    for area, cid, name, lib, mine, rel, limit, verdict, ref in ROWS:
        if verdict != "ok" or "--all" in sys.argv:
            print(f"| {area} | `{cid}` | {name} | {lib:.6g} | {mine:.6g} | {rel:.2e} | {limit:.0e} | {verdict} | `{ref}` |")
    print(f"\ncompared {len(ROWS)} quantities, {sum(r[7] != 'ok' for r in ROWS)} beyond tolerance", file=sys.stderr)


main()
