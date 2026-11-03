"""🚦️ Independent EN 1993 document oracle: a committed snapshot (JSON on stdin or a path) to the utilisation of member resistance, flexural and lateral-torsional buckling, deflection and joint resistance per member and joint (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: the deflection formula against PyNite). Written from EN 1990 6.10, EN 1993-1-1 5.5, 6.2, 6.3, 7.2, EN 1993-1-5 4.4 and EN 1993-1-8 3.4, 3.6, 3.9, 4.5.3.3 with the clause tables (Tables 5.2, 6.1, 6.2, 6.4, 3.4, ENV 1993-1-1 Annex F C1), not from the Rust. Class 4 sections use the effective section of EN 1993-1-5 4.4 (flange outstand k = 0.43, web internal element with ψ = -1 from the gross section; the second iteration on the effective stresses is not taken), and the member interaction 6.3.3 follows Tables B.1 to B.3 (method 2). A family id (`en1993.6.3.2.mb`) reports the worst row over every ultimate combination, as the shared step layer reads it. The member N is positive in compression; SI throughout."""

import importlib.util
import json
import math
import pathlib
import sys

def _sibling(folder):
    """🔌 The oracle module of a sibling folder, loaded by path (the isolated interpreter has no script directory on its path)."""
    path = pathlib.Path(__file__).resolve().parent / folder / "🐍️.py"
    spec = importlib.util.spec_from_file_location("en1993_oracle_" + str(abs(hash(folder))), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


FAMILIES = _sibling("🧭️families")
SECOND_WAVE = _sibling("🧭️families-w5")
PARAMS = {"en": dict(gm0=1.0, gm1=1.0, gm2=1.25, gm3=1.25), "de": dict(gm0=1.0, gm1=1.1, gm2=1.25, gm3=1.25)}
PSI0 = {"office": 0.7, "residential": 0.7, "congregation": 0.7, "shopping": 0.7, "storage": 1.0, "snow": 0.5, "snow_high": 0.7, "wind": 0.6, "temperature": 0.6}
C1_TABLE = [(-1.0, 2.752), (-0.75, 2.927), (-0.5, 2.704), (-0.25, 2.281), (0.0, 1.879), (0.25, 1.563), (0.5, 1.323), (0.75, 1.141), (1.0, 1.0)]
BOLT_AREA_MM2 = {12: 84.3, 14: 115.0, 16: 157.0, 18: 192.0, 20: 245.0, 22: 303.0, 24: 353.0, 27: 459.0, 30: 561.0, 33: 694.0, 36: 817.0}
BOLT_FUB_MPA = {"4.6": 400.0, "4.8": 400.0, "5.6": 500.0, "5.8": 500.0, "6.8": 600.0, "8.8": 800.0, "10.9": 1000.0}
K_FI = {"RC1": 0.9, "RC2": 1.0, "RC3": 1.1}
ALPHA = {"a0": 0.13, "a": 0.21, "b": 0.34, "c": 0.49, "d": 0.76}


def combos(doc, member_id, lead_scalar=None, forces=None):
    """⚖️ EN 1990 6.10a/b rows (ultimate) and the characteristic row of the member actions or of scalar forces keyed by load case."""
    g, variables = None, []
    cases = {c["id"]: c for c in doc["loadCases"]}
    rows = forces if forces is not None else [(cases[r["loadCaseId"]], r["action"]) for r in doc["memberActions"] if r["memberId"] == member_id and r["loadCaseId"] in cases]
    zero = {k: 0.0 for k in ("n", "vy", "vz", "my", "mz", "t")}
    g = dict(zero)
    for case, action in rows:
        if case["kind"].lower() in ("permanent", "prestress"):
            g = {k: g[k] + action[k] for k in g}
        else:
            variables.append((case, action))
    k_fi = K_FI[doc["reliabilityClass"]]
    uls = []
    if not variables:
        if sum(abs(g[k]) for k in ("n", "my", "vz")) > 0:
            uls.append({k: k_fi * 1.35 * v for k, v in g.items()})
        return uls, dict(g)
    for i, (lead, a) in enumerate(variables):
        others = {k: sum(k_fi * 1.5 * PSI0.get(c["category"], 0.7) * x[k] for j, (c, x) in enumerate(variables) if j != i) for k in g}
        for factor in (1.35, 0.85 * 1.35):
            uls.append({k: k_fi * (factor * g[k] + 1.5 * a[k]) + others[k] for k in g})
    lead, a = variables[0]
    sls = {k: g[k] + a[k] + sum(PSI0.get(c["category"], 0.7) * x[k] for c, x in variables[1:]) for k in g}
    return uls, sls


def epsilon(fy):
    return math.sqrt(235.0 / (fy / 1e6))


def section_class(s, fy):
    """🏷️ Table 5.2 class by section kind: rolled I flange outstand in compression and web in compression and bending (ψ = -1); circular hollow section d/t limits 50, 70, 90 ε² (sheet 3); angle h/t ≤ 15 ε and (b+h)/2t ≤ 11.5 ε is class 3, otherwise 4 (sheet 3, compression rule also used for bending)."""
    eps = epsilon(fy)
    if s["kind"] == "Chs":
        return next((i + 1 for i, lim in enumerate((50, 70, 90)) if s["h"] / max(s["tw"], s["tf"]) <= lim * eps * eps), 4)
    if s["kind"] == "Angle":
        leg, short, t = max(s["h"], s["b"]), min(s["h"], s["b"]), max(s["tw"], s["tf"])
        return 3 if leg / t <= 15 * eps and (leg + short) / (2 * t) <= 11.5 * eps else 4
    c_f = (s["b"] - s["tw"] - 2 * s["r"]) / 2 / s["tf"]
    c_w = (s["h"] - 2 * s["tf"] - 2 * s["r"]) / s["tw"]
    cls = lambda ratio, limits: next((i + 1 for i, lim in enumerate(limits) if ratio <= lim * eps), 4)
    return max(cls(c_f, (9, 10, 14)), cls(c_w, (33, 38, 42)), cls(c_w, (72, 83, 124)))


def rho_outstand(lam):
    return 1.0 if lam <= 0.748 else min((lam - 0.188) / lam ** 2, 1.0)


def rho_internal(lam):
    return 1.0 if lam <= 0.673 else min((lam - 0.22) / lam ** 2, 1.0)


def effective_area(s, fy):
    """📐 EN 1993-1-5 4.4 effective area in uniform compression: I section flanges and web, angle leg outstands (k = 0.43), circular hollow section by the EN 1993-1-6 meridional shell reduction χ_x (Q = 25)."""
    eps = epsilon(fy)
    t = max(s["tw"], s["tf"])
    if s["kind"] == "Angle":
        leg, short = max(s["h"], s["b"]), min(s["h"], s["b"])
        rho = lambda w: rho_outstand((w - t) / t / (28.4 * eps * math.sqrt(0.43)))
        return s["area"] - (leg - t) * t * (1 - rho(leg)) - (short - t) * t * (1 - rho(short))
    if s["kind"] == "Chs":
        r = (s["h"] - t) / 2
        lam = math.sqrt(fy / (0.605 * 210e9 * t / r))
        alpha = 0.62 / (1 + 1.91 * (math.sqrt(r / t) / 25.0) ** 1.44) ** 0.6
        lp = math.sqrt(alpha / 0.4)
        chi_x = 1.0 if lam <= 0.2 else 1 - 0.6 * (lam - 0.2) / (lp - 0.2) if lam < lp else alpha / lam ** 2
        return s["area"] * chi_x
    c_f = max((s["b"] - s["tw"] - 2 * s["r"]) / 2, 1e-6)
    c_w = max(s["h"] - 2 * s["tf"] - 2 * s["r"], 1e-6)
    rf = rho_outstand(c_f / s["tf"] / (28.4 * eps * math.sqrt(0.43)))
    rw = rho_internal(c_w / s["tw"] / (28.4 * eps * math.sqrt(4.0)))
    return s["area"] - 4 * c_f * s["tf"] * (1 - rf) - c_w * s["tw"] * (1 - rw)


def route_open(s):
    """🏭 A hollow section without a chosen fabrication route has no Table 6.2 curve: its buckling rows are not evaluated."""
    return s["kind"] in ("Hss", "Chs") and s["fabrication"] == "Unset"


def buckling_curve(s, y_axis, fy):
    """📉 Table 6.2 for rolled and welded I sections and hollow sections; EN 1993-1-1 Table 6.2 sheet 1."""
    h, b, tf, high = s["h"] * 1e3, s["b"] * 1e3, s["tf"] * 1e3, fy >= 460e6
    kind = s["kind"]
    if kind in ("Hss", "Chs"):
        return ("a0" if high else "a") if s["fabrication"] == "HotFinished" else "c"
    if kind == "Angle":
        return "b"
    if kind == "WeldedI":
        return ("b" if y_axis else "c") if tf <= 40 else ("c" if y_axis else "d")
    if h / b > 1.2:
        if tf <= 40:
            return "a0" if high else ("a" if y_axis else "b")
        return "a" if high else ("b" if y_axis else "c")
    if tf <= 100:
        return "a" if high else ("b" if y_axis else "c")
    return "c" if high else "d"


def chi(lam, curve):
    """📉 χ of Eq. 6.49."""
    if lam <= 0.2:
        return 1.0
    phi = 0.5 * (1 + ALPHA[curve] * (lam - 0.2) + lam * lam)
    return min(1.0 / (phi + math.sqrt(max(phi * phi - lam * lam, 0.0))), 1.0)


def c1_of(psi):
    p = min(max(psi, -1.0), 1.0)
    for (x0, y0), (x1, y1) in zip(C1_TABLE[:-1], C1_TABLE[1:]):
        if x0 <= p <= x1:
            return y0 + (p - x0) / (x1 - x0) * (y1 - y0)
    return 1.0


def critical_moment(s, m, length, member):
    """🔄 M_cr of a doubly symmetric section by the closed form with the C1, C2 of the moment diagram and the load height; angles have no such closed form and are not assessed for lateral-torsional buckling (infinite)."""
    if s["kind"] == "Angle":
        return math.inf
    diagram = member["momentDiagram"].lower()
    c1, c2 = (1.0, 0.0) if diagram == "uniform" else (1.127, 0.454) if diagram == "parabolic" else (c1_of(member["endMomentRatioPsi"]), 0.0)
    zg = {"topflange": s["h"] / 2, "bottomflange": -s["h"] / 2}.get(member["loadApplication"].lower(), 0.0)
    e, g = m["eModulus"], m["gModulus"]
    a = math.pi ** 2 * e * s["iz"] / length ** 2
    return c1 * a * (math.sqrt(s["iw"] / s["iz"] + length ** 2 * g * s["it"] / (math.pi ** 2 * e * s["iz"]) + (c2 * zg) ** 2) - c2 * zg)


def effective_bending_modulus(s, fy):
    """📐 W_eff,y of a doubly symmetric I in bending about y (EN 1993-1-5 4.4, Table 4.1 and 4.2): compression flange outstand with k = 0.43, web as internal element with ψ = -1 (k = 23.9) so b_eff = ρ h_w / 2 split 0.4/0.6, welded corners (r = 0); the stress ratio is taken from the gross section."""
    eps = epsilon(fy)
    h, b, tw, tf = s["h"], s["b"], s["tw"], s["tf"]
    hw = h - 2 * tf
    c = (b - tw) / 2
    rho_f = rho_outstand(c / tf / (28.4 * eps * math.sqrt(0.43)))
    rho_w = rho_internal_psi(hw / tw / (28.4 * eps * math.sqrt(23.9)), -1.0)
    half = hw / 2
    b_eff = rho_w * half
    z_flange = h / 2 - tf / 2
    parts = [
        ((tw + 2 * rho_f * c) * tf, z_flange, (tw + 2 * rho_f * c) * tf ** 3 / 12),
        (b * tf, -z_flange, b * tf ** 3 / 12),
        (0.4 * b_eff * tw, half - 0.4 * b_eff / 2, tw * (0.4 * b_eff) ** 3 / 12),
        (0.6 * b_eff * tw, 0.6 * b_eff / 2, tw * (0.6 * b_eff) ** 3 / 12),
        (half * tw, -half / 2, tw * half ** 3 / 12),
    ]
    area = sum(a for a, _, _ in parts)
    e_n = sum(a * z for a, z, _ in parts) / area
    inertia = sum(i + a * (z - e_n) ** 2 for a, z, i in parts)
    return inertia / max(h / 2 - e_n, h / 2 + e_n)


def rho_internal_psi(lam, psi):
    return 1.0 if lam <= 0.5 + math.sqrt(0.085 - 0.055 * psi) else min((lam - 0.055 * (3 + psi)) / lam ** 2, 1.0)


def cm_factor(member):
    """🔗 C_m of Table B.3 for the declared moment diagram: constant, uniform load without end moments (α_h = 0) or the straight line between the end moments."""
    diagram = member["momentDiagram"].lower()
    return 1.0 if diagram == "uniform" else 0.95 if diagram == "parabolic" else max(0.6 + 0.4 * member["endMomentRatioPsi"], 0.4)


def interaction_annex_b(n, my, mz, n_rk, my_rk, mz_rk, chi_y, chi_z, chi_lt, lam_y, lam_z, cmy, cmz, cm_lt, cls, torsion, gm1, d_my=0.0, d_mz=0.0):
    """🔗 EN 1993-1-1 Annex B, Tables B.1 and B.2, equations 6.61 and 6.62: the two utilisations (I sections, not hollow)."""
    ny, nz = n / (chi_y * n_rk / gm1), n / (chi_z * n_rk / gm1)
    low = cls <= 2
    kyy = min(cmy * (1 + (lam_y - 0.2) * ny), cmy * (1 + 0.8 * ny)) if low else min(cmy * (1 + 0.6 * lam_y * ny), cmy * (1 + 0.6 * ny))
    kzz = min(cmz * (1 + (2 * lam_z - 0.6) * nz), cmz * (1 + 1.4 * nz)) if low else min(cmz * (1 + 0.6 * lam_z * nz), cmz * (1 + 0.6 * nz))
    kyz = 0.6 * kzz if low else kzz
    if not torsion:
        kzy = 0.6 * kyy if low else 0.8 * kyy
    else:
        d = max(cm_lt - 0.25, 1e-6)
        if low:
            kzy = max(1 - 0.1 * lam_z / d * nz, 1 - 0.1 / d * nz) if lam_z >= 0.4 else min(0.6 + lam_z, 1 - 0.1 * lam_z / d * nz)
        else:
            kzy = max(1 - 0.05 * lam_z / d * nz, 1 - 0.05 / d * nz)
    m_y, m_z = (my + d_my) / (chi_lt * my_rk / gm1), (mz + d_mz) / (mz_rk / gm1)
    return max(ny + kyy * m_y + kyz * m_z, nz + kzy * m_y + kzz * m_z)


def mn_eta(n, my, mz, n_rd, m_y_rd, m_z_rd, area, b, tf, cls, kind="RolledI"):
    """⚖ EN 1993-1-1 6.2.9.1 (class 1, 2: Eq. 6.38 to 6.41 with α = 2, β = max(5n, 1); circular hollow section M_N = M_pl (1 - n^1.7) with α = β = 2) and 6.2.9.2 (class 3, 4 and angles: linear sum of the stress utilisations)."""
    if cls >= 3 or kind == "Angle":
        return abs(n) / n_rd + abs(my) / m_y_rd + abs(mz) / max(m_z_rd, 1e-9)
    if kind == "Chs":
        nn = abs(n) / n_rd
        if nn >= 1.0:
            return nn + abs(my) / m_y_rd + abs(mz) / max(m_z_rd, 1e-9)
        return (abs(my) / (m_y_rd * (1 - nn ** 1.7))) ** 2 + (abs(mz) / (m_z_rd * (1 - nn ** 1.7))) ** 2
    nn = abs(n) / n_rd
    a = min((area - 2 * b * tf) / area, 0.5)
    m_ny = min(m_y_rd * (1 - nn) / (1 - 0.5 * a), m_y_rd)
    m_nz = m_z_rd if nn <= a else m_z_rd * (1 - ((nn - a) / (1 - a)) ** 2)
    return (abs(my) / m_ny) ** 2 + (abs(mz) / m_nz) ** max(5 * nn, 1.0)


def member_checks(doc, member, p):
    """🏗️ The ultimate and serviceability utilisations of one member."""
    section = next((s for s in doc["sections"] if s["id"] == member["sectionId"]), None)
    material = next((m for m in doc["materials"] if m["id"] == member["materialId"]), None)
    if section is None or material is None or route_open(section):
        return []
    fy, fu, e = material["fy"], material["fu"], material["eModulus"]
    kind = member["memberType"].lower().replace("_", "").replace("-", "")
    cls = section_class(section, fy) if section["kind"] != "Hss" else 0
    if cls == 0:
        return []
    a_res = effective_area(section, fy) if cls == 4 else section["area"]
    uls, sls = combos(doc, member["id"])
    rows = {}
    note = lambda key, value: rows.__setitem__(key, max(rows.get(key, 0.0), value))
    iy, iz = math.sqrt(section["iy"] / section["area"]), math.sqrt(section["iz"] / section["area"])
    lam1 = math.pi * math.sqrt(e / fy)
    eff = math.sqrt(a_res / section["area"])
    lam_ax = {ax: member[f"bucklingLength{ax.upper()}"] / i / lam1 * eff for ax, i in (("y", iy), ("z", iz))}
    chis = [chi(lam_ax[ax], buckling_curve(section, ax == "y", fy)) for ax in ("y", "z")]
    n_b_rd = min(chis) * a_res * fy / p["gm1"]
    w_y = section["wPlY"] if cls <= 2 else section["wElY"] if cls == 3 else min(effective_bending_modulus(section, fy), section["wElY"]) if section["kind"] in ("RolledI", "WeldedI") else section["wElY"] * min(a_res / section["area"], 1.0)
    w_z = section["wPlZ"] if cls <= 2 else section["wElZ"] if cls == 3 else section["wElZ"] * min(a_res / section["area"], 1.0)
    length_ltb = max(member["ltbLength"], member["ltbRestraintSpacing"])
    m_cr = critical_moment(section, material, length_ltb, member)
    lam_lt = 10.0 if m_cr <= 0 else math.sqrt(w_y * fy / m_cr)
    curve_lt = ("c" if section["h"] / max(section["b"], 1e-9) <= 2 else "d") if "weld" in section["kind"].lower() else ("a" if section["h"] / max(section["b"], 1e-9) <= 2 else "b")
    m_b_rd = chi(lam_lt, curve_lt) * w_y * fy / p["gm1"]
    v_z_rd, v_y_rd = (a * fy / math.sqrt(3) / p["gm0"] for a in (section["shearAreaZ"], section["shearAreaY"]))
    mid = member["id"]
    for a in uls:
        if a["n"] >= 0 and kind != "tie":
            note("en1993.6.2.4.n", a["n"] / (a_res * fy / p["gm0"]))
        elif a["n"] < 0:
            note("en1993.6.2.3.nt", -a["n"] / min(section["area"] * fy / p["gm0"], 0.9 * section["areaNet"] * fu / p["gm2"]))
        m_y_rd = w_y * fy / p["gm0"]
        if cls <= 2 and v_z_rd > 0:
            v = abs(a["vz"])
            rho = 0.0 if v <= 0.5 * v_z_rd else (2 * v / v_z_rd - 1) ** 2
            aw = (section["h"] - 2 * section["tf"]) * section["tw"]
            m_y_rd = min((section["wPlY"] - rho * aw * aw / (4 * section["tw"])) * fy / p["gm0"], m_y_rd) if section["kind"] in ("RolledI", "WeldedI") else m_y_rd * (1 - rho)
        if kind not in ("brace", "tie"):
            note("en1993.6.2.5.my", abs(a["my"]) / m_y_rd)
        note("en1993.6.2.9.mn", mn_eta(a["n"], a["my"], a["mz"], a_res * fy / p["gm0"], m_y_rd, w_z * fy / p["gm0"], section["area"], section["b"], section["tf"], cls, section["kind"]))
        for axis, ved, vrd in (("z", abs(a["vz"]), v_z_rd), ("y", abs(a["vy"]), v_y_rd)):
            if ved > 0 or axis == "z":
                note(f"en1993.6.2.6.v{axis}", ved / vrd)
        if a["n"] > 0 and kind not in ("beam", "tie"):
            note("en1993.6.3.1.nb", a["n"] / n_b_rd)
        if abs(a["my"]) > 0 and kind not in ("column", "brace", "tie") and section["kind"] != "Angle":
            note("en1993.6.3.2.mb", abs(a["my"]) / m_b_rd)
        if kind in ("beamcolumn", "column", "") and (a["n"] > 0 or abs(a["my"]) > 0 or abs(a["mz"]) > 0):
            torsion = kind not in ("column", "brace", "tie")
            cm = cm_factor(member)
            note("en1993.6.3.3.intb", interaction_annex_b(max(a["n"], 0.0), abs(a["my"]), abs(a["mz"]), a_res * fy, w_y * fy, w_z * fy, chis[0], chis[1], chi(lam_lt, curve_lt) if torsion else 1.0, lam_ax["y"], lam_ax["z"], cm, cm, cm, cls, torsion, p["gm1"]))
        for key, value in FAMILIES.torsion_notes(member, section, material, a, p):
            note(key, value)
    if sls is not None and member["length"] > 0:
        delta = 5 * abs(sls["my"]) * member["length"] ** 2 / (48 * e * section["iy"])
        rows["en1993.7.sls"] = delta / (member["length"] / max(member["deflectionLimitRatio"], 1.0))
    return [(f"{key}", value) for key, value in rows.items()]


def joint_checks(doc, joint, p):
    """🔩 Bolted shear, bearing, slip and welded simplified resistance of one joint under its ultimate scalar forces."""
    cases = {c["id"]: c for c in doc["loadCases"]}
    scalar = lambda key: [(cases[a["loadCaseId"]], {"n": a[key], "vy": 0.0, "vz": 0.0, "my": 0.0, "mz": 0.0, "t": 0.0}) for a in joint["actions"] if a["loadCaseId"] in cases]
    value = lambda key: max((r["n"] for r in combos(doc, None, forces=scalar(key))[0]), key=abs, default=0.0)
    shear, tension = value("shear"), value("tension")
    jid, out = joint["id"], []
    if joint["kind"] == "Bolted":
        n = joint["boltRows"] * joint["boltsPerRow"]
        d = joint["boltDiameter"]
        a_s = BOLT_AREA_MM2.get(round(d * 1e3), math.pi / 4 * (d * 1e3 - 0.9382 * d * 1e3 * 0.15) ** 2) * 1e-6
        f_ub = BOLT_FUB_MPA.get(joint["boltClass"], 800.0) * 1e6
        d_mm = d * 1e3
        d0 = (d_mm + 1 if d_mm <= 14 else d_mm + 2 if d_mm <= 24 else d_mm + 3) / 1e3
        alpha_v = 0.6 if joint["boltClass"] in ("4.6", "5.6", "8.8") else 0.5
        p1 = joint["pitch"]
        p2 = joint["gauge"] if joint["gauge"] > 0 else joint["pitch"]
        f_v = n * max(joint["shearPlanes"], 1) * alpha_v * a_s * f_ub / p["gm2"]
        alpha_b = min(joint["endDistance"] / (3 * d0), p1 / (3 * d0) - 0.25, f_ub / joint["plateFu"], 1.0)
        k1 = min(2.8 * joint["edgeDistance"] / d0 - 1.7, 1.4 * p2 / d0 - 1.7, 2.5)
        f_b = n * max(k1, 0.0) * max(alpha_b, 0.0) * joint["plateFu"] * d * joint["plateThickness"] / p["gm2"]
        out += [(f"en1993.1-8.3.6.shear.{jid}", shear / f_v), (f"en1993.1-8.3.6.bearing.{jid}", shear / f_b)]
        if tension > 0:
            f_t = n * 0.9 * a_s * f_ub / p["gm2"]
            out.append((f"en1993.1-8.3.6.tension.{jid}", tension / f_t))
            if shear > 0:
                out.append((f"en1993.1-8.3.6.interaction.{jid}", max(shear / f_v + tension / (1.4 * f_t), tension / f_t)))
        if joint["category"].upper() in ("B", "C"):
            f_pc = joint["preloadForce"] if joint["preloadForce"] > 0 else 0.7 * f_ub * a_s
            f_s = max(joint["slipFactorKs"], 0.1) * max(joint["frictionSurfaces"], 1) * max(joint["frictionMu"], 0.1) * f_pc * max(n, 1) / p["gm3"]
            out.append((f"en1993.1-8.3.9.slip.{jid}", shear / f_s))
    elif joint["kind"] == "Welded":
        beta = 0.8 if joint["weldGrade"].startswith("S235") else 0.85 if joint["weldGrade"].startswith("S275") else 0.9 if joint["weldGrade"].startswith("S355") else 1.0
        out.append((f"en1993.1-8.4.5.simplified.{jid}", math.hypot(abs(joint["weldForceLongitudinal"]) if joint["weldForceLongitudinal"] != 0 else shear, joint["weldForceTransverse"]) / (joint["weldThroat"] * joint["weldLength"] * joint["weldFu"] / (beta * math.sqrt(3) * p["gm2"]))))
    return out


GAMMA_C, GAMMA_INST, CONE_K1 = 1.5, 1.0, 8.9
BOLT_FYB_MPA = {"4.6": 240.0, "4.8": 320.0, "5.6": 300.0, "5.8": 400.0, "6.8": 480.0, "8.8": 640.0, "10.9": 900.0}


def stress_area_mm2(d_mm):
    """🔩 Table of tensile stress areas A_s (mm2) of EN ISO 898-1; other diameters from the thread core estimate."""
    table = {12: 84.3, 14: 115.0, 16: 157.0, 18: 192.0, 20: 245.0, 22: 303.0, 24: 353.0, 27: 459.0, 30: 561.0, 33: 694.0, 36: 817.0}
    return table.get(math.floor(d_mm + 0.5), math.pi / 4 * (d_mm - 0.9382 * d_mm * 0.15) ** 2)


def bolt_group_peak(points, fx, fy, mz):
    """🔩 Elastic bolt group: the vector sum of the direct share and the torsion share of the most loaded bolt (EN 1993-1-8 3.12 simplified)."""
    n = len(points)
    polar = sum(x * x + y * y for x, y in points)
    k = mz / polar if polar > 0 else 0.0
    return max(math.hypot(fx / n - k * y, fy / n + k * x) for x, y in points)


def plate_rows(doc, p, out):
    """🧩 Rows `en1993.1-8.3.6.plate`, `3.10.plateblock`, `1-1.6.2.3.platenet` and `1-8.4.5.plateweld` per fin, web and flange cover plate: elastic bolt group, block tearing, cover plate tension and the directional fillet weld."""
    joints = {j["id"]: j for j in doc["joints"]}
    gm0, gm2 = p["gm0"], p["gm2"]
    for pl in doc.get("jointPlates", []):
        keys = ("plateThickness", "plateFu", "plateFy", "boltDiameter", "holeDiameter", "endDistance", "edgeDistance")
        if not (all(pl[k] > 0 for k in keys) and pl["boltRows"] > 0 and pl["boltsPerRow"] > 0) or pl["jointId"] not in joints:
            continue
        j = joints[pl["jointId"]]
        v = max([abs(a["shear"]) for a in j["actions"]] + [0.0])
        n_ed = max([a["tension"] for a in j["actions"]] + [0.0])
        if pl["role"] != "flangeCover":
            fx, fy, mz = n_ed, v, v * pl["eccentricity"] * 1e3
        else:
            member = next((m for m in doc["members"] if m["id"] == j["memberId"]), None)
            section = next((s for s in doc["sections"] if member and s["id"] == member["sectionId"]), None)
            lever = section["h"] - section["tf"] if section else 0.0
            if not (section and lever > 0 and section["area"] > 0):
                continue
            fx, fy, mz = 0.0, abs(abs(j["momentDemand"]) / lever + n_ed * section["b"] * section["tf"] / section["area"]), 0.0
        if fx == 0.0 and fy == 0.0:
            continue
        d, d0, e1, e2 = pl["boltDiameter"] * 1e3, pl["holeDiameter"] * 1e3, pl["endDistance"] * 1e3, pl["edgeDistance"] * 1e3
        fu, fy_mpa = pl["plateFu"] / 1e6, pl["plateFy"] / 1e6
        fub = {"4.6": 400.0, "4.8": 400.0, "5.6": 500.0, "5.8": 500.0, "6.8": 600.0, "8.8": 800.0, "10.9": 1000.0}.get(pl["boltClass"], 800.0)
        p1, p2 = pl["pitch"] * 1e3, pl["gauge"] * 1e3
        n1, n2 = pl["boltRows"], pl["boltsPerRow"]
        points = [((c - (n2 - 1) / 2) * p2, (r - (n1 - 1) / 2) * p1) for r in range(n1) for c in range(n2)]
        f_ed = bolt_group_peak(points, fx, fy, mz)
        alpha_end = min(e1 / (3 * d0), fub / fu, 1.0)
        alpha_inner = min(p1 / (3 * d0) - 0.25, fub / fu, 1.0)
        alpha = min(alpha_end, alpha_inner) if n1 > 1 else alpha_end
        k1 = min(2.8 * e2 / d0 - 1.7, 1.4 * p2 / d0 - 1.7, 2.5) if n2 > 1 else min(2.8 * e2 / d0 - 1.7, 2.5)
        alpha_v = 0.6 if pl["boltClass"] in ("4.6", "5.6", "8.8") else 0.5
        planes = max(pl["shearPlanes"], 1)
        t_eff = pl["plateThickness"] * 1e3 * max(pl["plateCount"], 1)
        fv_rd = planes * alpha_v * fub * stress_area_mm2(d) / gm2
        fb_rd = k1 * alpha * fu * d * t_eff / gm2
        out.append((f"en1993.1-8.3.6.plate.{pl['id']}", f_ed / min(fv_rd, fb_rd), True))
        rows_v = max(e1 + (n1 - 1) * p1 - (n1 - 0.5) * d0, 0.0)
        if n2 <= 1:
            a_nt, a_nv = max((e2 - 0.5 * d0) * t_eff, 0.0), rows_v * t_eff
        else:
            a_nt, a_nv = max((n2 - 1) * (p2 - d0) * t_eff, 0.0), 2.0 * rows_v * t_eff
        block = (1.0 if pl["eccentricity"] <= 0 else 0.5) * fu * a_nt / gm2 + fy_mpa * a_nv / (math.sqrt(3) * gm0)
        out.append((f"en1993.1-8.3.10.plateblock.{pl['id']}", math.hypot(fx, fy) / block, True))
        if pl["role"] == "flangeCover" and pl["plateWidth"] > 0:
            gross = pl["plateWidth"] * 1e3 * t_eff
            net = (pl["plateWidth"] * 1e3 - n2 * d0) * t_eff
            out.append((f"en1993.1-1.6.2.3.platenet.{pl['id']}", fy / min(gross * fy_mpa / gm0, 0.9 * net * fu / gm2), True))
        if pl["role"] == "finPlate" and pl["weldThroat"] > 0 and pl["weldFu"] > 0 and pl["weldBetaW"] > 0 and pl["plateHeight"] > 0:
            a, length = pl["weldThroat"] * 1e3, pl["plateHeight"] * 1e3
            sigma = mz / (a * length * length / 3.0) / math.sqrt(2)
            tau = fy / (2.0 * a * length)
            weld_fu = pl["weldFu"] / 1e6
            equivalent = math.sqrt(sigma ** 2 + 3.0 * (sigma ** 2 + tau ** 2))
            out.append((f"en1993.1-8.4.5.plateweld.{pl['id']}", max(equivalent / (weld_fu / (pl["weldBetaW"] * gm2)), abs(sigma) / (0.9 * weld_fu / gm2)), True))


def base_forces(j):
    """🧱 Anchor tension T and compression C (N) from the base plate statics: T = (M + N z_r) / (z_l + z_r), C = T - N."""
    n, m, zl, zr = j["baseNormalForce"], abs(j["baseMoment"]), j["baseLeverLeft"], j["baseLeverRight"]
    if zl + zr <= 0:
        return max(n, 0.0), max(-n, 0.0)
    t = (m + n * zr) / (zl + zr)
    return (0.0, max(-n, 0.0)) if t <= 0 else (t, max(t - n, 0.0))


def flange_compression(j):
    """🧱 Compression (N) of the left and the right flange T-stub: moment equilibrium about the opposite flange line when the whole section is in compression, else C on the compression side."""
    n, m, zl, zr = j["baseNormalForce"], abs(j["baseMoment"]), j["baseLeverLeft"], j["baseLeverRight"]
    if zl + zr <= 0:
        return 0.0, max(-n, 0.0)
    if (m + n * zr) / (zl + zr) > 0:
        return 0.0, base_forces(j)[1]
    return max((-n * zr - m) / (zl + zr), 0.0), max((m - n * zl) / (zl + zr), 0.0)


def flange_width(doc, j):
    """🧱 Flange width b_fc (mm) of the section of the member of the joint, None while missing."""
    member = next((m for m in doc["members"] if m["id"] == j["memberId"]), None)
    section = next((s for s in doc["sections"] if member and s["id"] == member["sectionId"]), None)
    return section["b"] * 1e3 if section and section["b"] > 0 else None


def required_c(tfc, bfc, bp, area):
    """🧱 Smallest c with (t_fc + 2c) min(b_fc + 2c, B_p) >= area by bisection."""
    if tfc * min(bfc, bp) >= area:
        return 0.0
    lo, hi = 0.0, 1e5
    for _ in range(200):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if (tfc + 2.0 * mid) * min(bfc + 2.0 * mid, bp) < area else (lo, mid)
    return hi


def bearing(b, j, bfc, gm0):
    """🧱 Iterated c-strip of the base plate on grout and concrete: f_jd = beta_j F_Rdu / (b_eff l_eff) with F_Rdu = min(A_c0 f_cd sqrt(A_c1/A_c0), 3 f_cd A_c0), beta_j = 2/3, c = t_p sqrt(f_y / (3 f_jd gamma_M0))."""
    lp, bp, tp, fyp, tfc = j["basePlateLength"] * 1e3, j["plateWidth"] * 1e3, j["plateThickness"] * 1e3, j["plateFy"] / 1e6, j["columnFlangeThickness"] * 1e3
    a, w, h = b["blockLength"] * 1e3, b["blockWidth"] * 1e3, b["blockDepth"] * 1e3
    fcd = b["concreteFck"] / 1e6 / GAMMA_C
    c, f_jd, b1, l1 = tp, 0.0, lp, bp
    for _ in range(8):
        b1, l1 = min(tfc + 2.0 * c, lp), min(bfc + 2.0 * c, bp)
        b2, d2 = min(a, b1 + h, 3.0 * b1), min(w, l1 + h, 3.0 * l1)
        a_c0, a_c1 = b1 * l1, b2 * d2
        f_jd = (2.0 / 3.0) * min(a_c0 * fcd * math.sqrt(a_c1 / a_c0), 3.0 * fcd * a_c0) / (b1 * l1)
        c = tp * math.sqrt(fyp / (3.0 * f_jd * gm0))
    return f_jd, b1, f_jd * b1 * l1, c, (lp, bp, tp, fyp, tfc)


def foundation_rows(doc, p, out):
    """🧱 Rows `grout`, `bearing`, `anchort`, `anchorv`, `cbend`, `tbend`, `mjrd` and `cone` per foundation block (EN 1993-1-8 6.2.2, 6.2.5, 6.2.6.11, 6.2.8.3; EN 1992-4 7.2.1.4)."""
    joints = {j["id"]: j for j in doc["joints"]}
    gm0, gm2 = p["gm0"], p["gm2"]
    for b in doc.get("foundationBlocks", []):
        j = joints.get(b["jointId"])
        bfc = flange_width(doc, j) if j else None
        if j is None or bfc is None or not (b["concreteFck"] > 0 and b["groutFck"] > 0 and j["plateWidth"] > 0 and j["basePlateLength"] > 0 and j["plateThickness"] > 0 and j["plateFy"] > 0 and b["blockLength"] > 0 and b["blockWidth"] > 0):
            continue
        limit = 0.2 * min(j["basePlateLength"], j["plateWidth"])
        u_grout = max(b["groutThickness"] / limit, 0.2 * b["concreteFck"] / b["groutFck"])
        out.append((f"en1993.1-8.6.2.grout.{b['id']}", u_grout, False))
        t = base_forces(j)[0]
        c_ed = max(flange_compression(j))
        f_jd, b_eff, fc_rd, c_final, (lp, bp, tp, fyp, tfc) = bearing(b, j, bfc, gm0)
        grout_ok = u_grout <= 1.0
        if grout_ok and c_ed > 0:
            out.append((f"en1993.1-8.6.2.bearing.{b['id']}", c_ed / fc_rd, True))
        fub = {"4.6": 400.0, "4.8": 400.0, "5.6": 500.0, "5.8": 500.0, "6.8": 600.0, "8.8": 800.0, "10.9": 1000.0}.get(b["anchorClass"], 800.0)
        a_s = b["anchorStressArea"] * 1e6
        count = b["anchorCount"]
        ft_one = 0.9 * fub * a_s / gm2
        if t > 0 and count > 0 and a_s > 0:
            out.append((f"en1993.1-8.6.2.anchort.{b['id']}", t / count / ft_one, True))
        v = max([abs(a["shear"]) for a in j["actions"]] + [0.0])
        if v > 0 and count > 0 and a_s > 0:
            alpha = 0.44 - 0.0003 * BOLT_FYB_MPA.get(b["anchorClass"], 640.0)
            out.append((f"en1993.1-8.6.2.anchorv.{b['id']}", v / (count * alpha * fub * a_s / gm2), True))
        if grout_ok and c_ed > 0:
            c_req = required_c(tfc, bfc, bp, c_ed / f_jd)
            b_req = tfc + 2.0 * c_req
            t_req = tp * c_req / c_final
            ed, rd = (t_req, tp) if b_req <= lp else (b_req, lp)
            out.append((f"en1993.1-8.6.2.cbend.{b['id']}", ed / rd, True))
        m, e, lb = j["boltToWeb"] * 1e3, j["edgeDistance"] * 1e3, j["anchorLength"] * 1e3
        stub = None
        if min(m, e, tp, a_s, lb) > 0 and count > 0 and j["plateFy"] > 0:
            l_cp, l_nc = 2.0 * math.pi * m, 4.0 * m + 1.25 * e
            l1, l2 = min(l_cp, l_nc), l_nc
            sum_ft = count * ft_one
            prying_arm = min(e, 1.25 * m)
            mp1, mp2 = 0.25 * l1 * tp ** 2 * fyp / gm0, 0.25 * l2 * tp ** 2 * fyp / gm0
            prying = lb <= 8.8 * m ** 3 * a_s * count / (l1 * tp ** 3)
            ft1, ft2 = (4.0 * mp1 / m, (2.0 * mp2 + prying_arm * sum_ft) / (m + prying_arm)) if prying else (2.0 * mp1 / m, math.inf)
            stub = (min(ft1, ft2, sum_ft), sum_ft)
        if stub and t > 0:
            out.append((f"en1993.1-8.6.2.tbend.{b['id']}", t / stub[0], True))
        if grout_ok and stub and j["baseMoment"] != 0:
            ft = min(stub[0], stub[1])
            n_ed, zl, zr = j["baseNormalForce"], j["baseLeverLeft"] * 1e3, j["baseLeverRight"] * 1e3
            lo, hi = max(-fc_rd, n_ed - ft), min(ft, n_ed + fc_rd)
            m_rd = (hi * (zl + zr) - n_ed * zr) / 1e3 if lo <= hi else 0.0
            out.append((f"en1993.1-8.6.2.mjrd.{b['id']}", abs(j["baseMoment"]) / m_rd if m_rd > 0 else 1e6, True))
        if t > 0 and count > 0 and b["anchorEmbedment"] > 0:
            hef, c1 = b["anchorEmbedment"] * 1e3, b["anchorEdgeDistance"] * 1e3
            ccr = 1.5 * hef
            n0 = CONE_K1 * math.sqrt(b["concreteFck"] / 1e6) * hef ** 1.5
            ratio = (min(c1, ccr) + ccr) / (2.0 * ccr) if c1 > 0 else 1.0
            psi_s = min(0.7 + 0.3 * min(c1, ccr) / ccr, 1.0) if c1 > 0 else 1.0
            n_rk_c = n0 * ratio * psi_s * min(0.5 + hef / 200.0, 1.0)
            out.append((f"en1993.1-8.6.2.cone.{b['id']}", t / count / (n_rk_c / (GAMMA_C * GAMMA_INST)), True))


TEMPS_21 = [10, 0, -10, -20, -30, -40, -50]
LEVELS_21 = [(0.25, "r25"), (0.5, "r50"), (0.75, "r75")]


def table_21():
    import pathlib
    root = next(q for q in pathlib.Path(__file__).resolve().parents if q.name == "📕️norm")
    entries = json.loads((root / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").read_text(encoding="utf-8"))["entries"]
    prefix = "ndp.en1993.1-10.t21."
    return {e["id"][len(prefix):]: e["recommended"] for e in entries if e["id"].startswith(prefix)}


def column_21(cells, pair, key, t_ed):
    """🌡️ One stress column of Table 2.1 at T_Ed by two-point interpolation of the bracketing reference temperatures; None while any cell of the column is not transcribed."""
    values = [cells.get(f"{pair}.{key}.t{t}") for t in TEMPS_21]
    if any(v is None for v in values):
        return None
    if t_ed >= TEMPS_21[0]:
        return values[0]
    for (t0, v0), (t1, v1) in zip(zip(TEMPS_21, values), zip(TEMPS_21[1:], values[1:])):
        if t1 <= t_ed <= t0:
            return v0 + (v1 - v0) * (t0 - t_ed) / (t0 - t1)
    return values[-1]


def limit_21(cells, pair, ratio, t_ed):
    """🌡️ t_max for the stress level: interpolation between the adjacent stress columns when both are transcribed, otherwise the nearest transcribed column at or above the ratio; None outside the table."""
    if ratio > 0.75 + 1e-9 or t_ed < -50.0:
        return None
    cols = [column_21(cells, pair, key, t_ed) for _, key in LEVELS_21]
    at_or_above = [i for i, (level, _) in enumerate(LEVELS_21) if level >= ratio - 1e-9]
    first = at_or_above[0]
    usable = [i for i in at_or_above if cols[i] is not None]
    if not usable:
        return None
    if usable[0] != first or abs(LEVELS_21[first][0] - ratio) < 1e-9 or first == 0 or cols[first - 1] is None:
        return cols[usable[0]]
    lo, hi = LEVELS_21[first - 1][0], LEVELS_21[first][0]
    return cols[first - 1] + (cols[first] - cols[first - 1]) * (ratio - lo) / (hi - lo)


def toughness_rows(doc):
    """🌡️ EN 1993-1-10 2.1 per member: t / t_max at T_Ed = T_md + the shifts (cold forming dT = -3 * 100 t / (2 r)) from the registry table, by independent lookup; members whose column or stress level is not covered carry no row."""
    cells = table_21()
    sections = {x["id"]: x for x in doc["sections"]}
    materials = {x["id"]: x for x in doc["materials"]}
    out = []
    for m in doc["members"]:
        sec, mat = sections.get(m["sectionId"]), materials.get(m["materialId"])
        if not sec or not mat:
            continue
        t = max(sec["tf"], sec["tw"]) * 1e3
        cold = -3.0 * 100.0 * t / (2.0 * m["coldFormingRadius"] * 1e3) if m["coldFormingRadius"] > 0 else 0.0
        t_ed = m["designTemperature"] + m["radiationShift"] + m["stressShift"] + m["marginShift"] + m["strainRateShift"] + cold
        pair = f"{mat['grade'].lower()}-{mat['subgrade'].lower()}"
        if not mat["subgrade"] or not any(k.startswith(pair + ".") for k in cells):
            continue
        t_max = limit_21(cells, pair, m["toughnessStressRatio"], t_ed)
        if t_max is not None:
            out.append((f"en1993.1-10.t.{m['id']}", t / t_max, True))
    return out


def tower_rows(doc, p):
    """🗼 EN 1993-3-1 tower leg rows: flexural buckling N_Ed/N_b,Rd (y-y; both principal axes for angles) with the wind amplification max(c_f c_d, 1), and the EN 1993-1-1 6.2.9 interaction at the permanent-load design point (N from the tower rows, M_y, M_z from the member rows of the same member, only when they carry bending)."""
    rows = []
    cases = {c["id"]: c for c in doc["loadCases"]}
    zero = {k: 0.0 for k in ("n", "vy", "vz", "my", "mz", "t")}
    for tower in doc.get("towerLegs", []):
        member = next((m for m in doc["members"] if m["id"] == tower["memberId"]), None)
        section = next((s for s in doc["sections"] if member and s["id"] == member["sectionId"]), None)
        material = next((m for m in doc["materials"] if member and m["id"] == member["materialId"]), None)
        if section is None or material is None or route_open(section):
            continue
        fy, e = material["fy"], material["eModulus"]
        forces = [(cases[a["loadCaseId"]], dict(zero, n=a["force"])) for a in tower["actions"] if a["loadCaseId"] in cases]
        uls, _ = combos(doc, None, forces=forces)
        n_ed = max((abs(r["n"]) for r in uls), default=0.0)
        lam1 = math.pi * math.sqrt(e / fy)
        axes = (("y", member["bucklingLengthY"], section["iy"]), ("z", member["bucklingLengthZ"], section["iz"])) if section["kind"] == "Angle" else (("y", member["bucklingLengthY"], section["iy"]),)
        chi_min = min(chi(length / math.sqrt(i_moment / section["area"]) / lam1, buckling_curve(section, axis == "y", fy)) for axis, length, i_moment in axes)
        amp = max(tower["forceCoefficient"] * tower["dynamicFactor"], 1.0)
        n_b = chi_min * section["area"] * fy / p["gm1"] / amp
        rows.append((f"en1993.3-1.tower.{tower['id']}", n_ed / n_b, False))
        member_uls, _ = combos(doc, member["id"])
        if not member_uls:
            continue
        gov = max(member_uls, key=lambda a: sum(abs(a[k]) / max(max(abs(b[k]) for b in member_uls), 1e-300) for k in ("n", "my", "mz", "vy", "vz")))
        my, mz = abs(gov["my"]), abs(gov["mz"])
        if my <= 0 and mz <= 0:
            continue
        cls = section_class(section, fy)
        a_res = effective_area(section, fy) if cls == 4 else section["area"]
        w_y = section["wPlY"] if cls <= 2 else section["wElY"] if cls == 3 else section["wElY"] * min(a_res / section["area"], 1.0)
        w_z = section["wPlZ"] if cls <= 2 else section["wElZ"] if cls == 3 else section["wElZ"] * min(a_res / section["area"], 1.0)
        eta = mn_eta(n_ed, my, mz, a_res * fy / p["gm0"], w_y * fy / p["gm0"], w_z * fy / p["gm0"], section["area"], section["b"], section["tf"], cls, section["kind"])
        rows.append((f"en1993.3-1.tower.nm.{tower['id']}", eta, False))
    return rows


def not_applicable_rows(doc):
    """🚫 Rows without a resistance: a hollow member or tower leg whose fabrication route is open (no Table 6.2 curve) and the lateral-torsional buckling of an angle beam (EN 1993-1-1 6.3.2 and Table 6.4 cover I and H sections only)."""
    cases = {c["id"]: c for c in doc["loadCases"]}
    rows = []
    for member in doc["members"]:
        section = next((s for s in doc["sections"] if s["id"] == member["sectionId"]), None)
        if section is None:
            continue
        if route_open(section):
            rows.append(f"en1993.6.3.1.route.{member['id']}")
            rows += [f"en1993.3-1.tower.{t['id']}" for t in doc["towerLegs"] if t["memberId"] == member["id"]]
        elif section["kind"] == "Angle" and member["memberType"].lower() not in ("column", "brace", "tie"):
            actions = [r for r in doc["memberActions"] if r["memberId"] == member["id"] and r["loadCaseId"] in cases]
            lead = [r["loadCaseId"] for r in actions if cases[r["loadCaseId"]]["kind"].lower() not in ("permanent", "prestress")]
            ids = [f"uls-6.10{x}-{c}" for c in lead for x in "ab"] if lead else ["uls-g"]
            rows += [f"en1993.6.3.2.mb.{i}" for i in ids if any(abs(r["action"]["my"]) > 0 for r in actions)]
    return [{"id": i, "utilization": 0.0, "status": "notApplicable"} for i in rows]


def evaluate(doc):
    """🚦️ The check rows of every member and joint of a snapshot."""
    p = PARAMS[doc["annex"].lower()]
    rows = [r for m in doc["members"] for r in member_checks(doc, m, p)]
    rows += [r for j in doc["joints"] for r in joint_checks(doc, j, p)]
    checks = [{"id": i, "utilization": u} for i, u in rows]
    plates = []
    plate_rows(doc, p, plates)
    foundation_rows(doc, p, plates)
    plates += toughness_rows(doc)
    plates += tower_rows(doc, p)
    status = lambda u, cites: "fail" if u > 1.0 + 1e-9 else "unverifiedBasis" if cites else "pass"
    return {"checks": checks + [{"id": i, "utilization": u, "status": status(u, cites)} for i, u, cites in plates] + not_applicable_rows(doc) + FAMILIES.evaluate(doc, combos, p, section_class) + SECOND_WAVE.evaluate(doc, sys.modules[__name__], FAMILIES, p)}


def self_check_effective_section():
    """🧪 W_eff,y of the slender welded I of the class 4 documents against the second moment of the same effective plate strips integrated by `sectionproperties` (shapely union of the strips)."""
    from shapely.geometry import box
    from sectionproperties.pre.geometry import Geometry
    from sectionproperties.analysis import Section

    section = dict(h=0.60, b=0.20, tw=0.006, tf=0.008)
    fy = 355e6
    eps = epsilon(fy)
    h, b, tw, tf = section["h"], section["b"], section["tw"], section["tf"]
    hw, c = h - 2 * tf, (b - tw) / 2
    rho_f = rho_outstand(c / tf / (28.4 * eps * math.sqrt(0.43)))
    rho_w = rho_internal_psi(hw / tw / (28.4 * eps * math.sqrt(23.9)), -1.0)
    b_eff = rho_w * hw / 2
    x0, x1 = b / 2 - tw / 2, b / 2 + tw / 2
    strips = [
        box(x0 - rho_f * c, h - tf, x1 + rho_f * c, h),
        box(0, 0, b, tf),
        box(x0, h - tf - 0.4 * b_eff, x1, h - tf),
        box(x0, h / 2, x1, h / 2 + 0.6 * b_eff),
        box(x0, tf, x1, h / 2),
    ]
    geometry = Geometry(strips[0].union(strips[1]).union(strips[2]).union(strips[3]).union(strips[4]))
    geometry.create_mesh(mesh_sizes=[1e-6])
    analysis = Section(geometry)
    analysis.calculate_geometric_properties()
    ixx = analysis.get_ic()[0]
    ytop = h - analysis.get_c()[1]
    theirs = ixx / max(ytop, analysis.get_c()[1])
    ours = effective_bending_modulus(dict(h=h, b=b, tw=tw, tf=tf), fy)
    assert abs(ours - theirs) <= 1e-3 * ours, (ours, theirs)
    return {"wEff": ours, "sectionproperties": theirs}


def self_check_annex_b():
    """🧪 Worked example E06 of the target specification (HEB 240 S355, L = 4 m pinned, N = 800 kN, M_y = 100 kNm at one end): k_yy = 0.6446, k_zy = 0.9163 and the EN utilisations 0.4138 and 0.5857 of equations 6.61 and 6.62."""
    n_rk, my_rk = 3763.0e3, 373.8e3
    chi_y, chi_z, chi_lt = 0.8807, 0.6243, 1.0
    value = interaction_annex_b(800e3, 100e3, 0.0, n_rk, my_rk, 1e12, chi_y, chi_z, chi_lt, 0.5079, 0.8605, 0.6, 0.6, 0.6, 1, True, 1.0)
    ny, nz = 800e3 / (chi_y * n_rk), 800e3 / (chi_z * n_rk)
    kyy = 0.6 * (1 + (0.5079 - 0.2) * ny)
    kzy = 1 - 0.1 * 0.8605 / 0.35 * nz
    assert abs(kyy - 0.6446) < 1e-3 and abs(kzy - 0.9163) < 1e-3, (kyy, kzy)
    u61, u62 = ny + kyy * 100e3 / my_rk, nz + kzy * 100e3 / my_rk
    assert abs(u61 - 0.4138) < 1e-3 and abs(u62 - 0.5857) < 1e-3 and abs(value - max(u61, u62)) < 1e-9, (u61, u62, value)
    return {"u61": u61, "u62": u62}


def self_check_fixture_sections():
    """🧪 Area and second moments of every rolled I section of the committed oracle documents against `sectionproperties` (`i_section` with the root radius); the code shear areas of 6.2.6(3) are an approximation of the elastic shear area and are not compared."""
    import pathlib
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import i_section

    seen = {}
    documents = pathlib.Path(__file__).resolve().parents[2] / "🧪️tests" / "🧫️fixtures" / "📄️oracle-documents"
    for path in sorted(documents.glob("*/🔣️.json")):
        for s in json.loads(path.read_text(encoding="utf-8"))["sections"]:
            if s["kind"] == "RolledI" and s["id"] not in seen:
                geometry = i_section(d=s["h"] * 1e3, b=s["b"] * 1e3, t_f=s["tf"] * 1e3, t_w=s["tw"] * 1e3, r=s["r"] * 1e3, n_r=16)
                geometry.create_mesh(mesh_sizes=[20.0])
                section = Section(geometry)
                section.calculate_geometric_properties()
                ixx, iyy, _ = section.get_ic()
                theirs = {"area": section.get_area() / 1e6, "iy": ixx / 1e12, "iz": iyy / 1e12}
                for key, value in theirs.items():
                    assert abs(s[key] - value) <= 5e-3 * value, (s["id"], key, s[key], value)
                seen[s["id"]] = theirs
            if s["kind"] in ("Angle", "Chs") and f"{path.parent.name}/{s['id']}" not in seen:
                from sectionproperties.pre.library import angle_section, circular_hollow_section
                wall = max(s["tw"], s["tf"]) * 1e3
                geometry = angle_section(d=max(s["h"], s["b"]) * 1e3, b=min(s["h"], s["b"]) * 1e3, t=wall, r_r=0, r_t=0, n_r=1) if s["kind"] == "Angle" else circular_hollow_section(d=s["h"] * 1e3, t=wall, n=128)
                geometry.create_mesh(mesh_sizes=[wall / 3])
                section = Section(geometry)
                section.calculate_geometric_properties()
                if s["kind"] == "Angle":
                    i11, i22 = section.get_ip()
                    theirs = {"area": section.get_area() / 1e6, "iy": i11 / 1e12, "iz": i22 / 1e12}
                else:
                    ixx, iyy, _ = section.get_ic()
                    theirs = {"area": section.get_area() / 1e6, "iy": ixx / 1e12, "iz": iyy / 1e12}
                for key, value in theirs.items():
                    assert abs(s[key] - value) <= 5e-3 * value, (s["id"], key, s[key], value)
                seen[f"{path.parent.name}/{s['id']}"] = theirs
    return seen


def self_check():
    """🧪 5 M L²/(48 E I) of the deflection against a PyNite simply supported beam under the uniform load q = 8 M / L²."""
    from Pynite import FEModel3D

    length, e, i_y, m_ed = 6.0, 210e9, 1.0e-4, 80e3
    model = FEModel3D()
    model.add_material("S", e, 81e9, 0.3, 7850)
    for name, x in (("A", 0.0), ("B", length)):
        model.add_node(name, x, 0, 0)
    model.add_section("Sec", 0.01, i_y, i_y, 1e-6)
    model.add_member("M", "A", "B", "S", "Sec")
    model.def_support("A", True, True, True, True, False, False)
    model.def_support("B", False, True, True, False, False, False)
    model.add_member_dist_load("M", "Fy", -8 * m_ed / length ** 2, -8 * m_ed / length ** 2)
    model.analyze()
    theirs = abs(model.members["M"].deflection("dy", 0.5 * length))
    ours = 5 * m_ed * length ** 2 / (48 * e * i_y)
    assert abs(ours - theirs) <= 5e-3 * ours, (ours, theirs)
    annex_b = self_check_annex_b()
    print(json.dumps({"deflection": ours, "pyNite": theirs, "annexB": annex_b, "effectiveSection": self_check_effective_section(), "fixtureSections": self_check_fixture_sections(), "families": FAMILIES.self_check(), "secondWave": SECOND_WAVE.self_check()}))


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        self_check()
    else:
        paths = [a for a in sys.argv[1:] if not a.startswith("--")]
        print(json.dumps(evaluate(json.load(open(paths[0], encoding="utf8") if paths else sys.stdin))))
