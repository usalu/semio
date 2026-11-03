"""🧭️ Independent EN 1993 document families beyond the members and joints: plated panels (EN 1993-1-5 3.3, 4.5, 5, 6, 7, 8, 9, 10; EN 1993-1-7 6.2), Palmgren-Miner fatigue (EN 1993-1-9 7, 8), fire (EN 1993-1-2 3.4, 4.2.3.2, 4.2.4, 4.2.5, ISO 834 with `scipy.integrate.solve_ivp`), cold-formed effective area (EN 1993-1-3 / 1-5 4.4) and storeys (EN 1993-1-1 5.2, 5.3.2, 5.3.3 with a planar frame eigenvalue by `scipy.linalg.eigh`). Written from the clause formulas, not from the Rust; every function returns rows `{id, utilization[, status]}`. SI throughout, N and Pa."""

import functools
import json
import math
import pathlib

import numpy as np
from scipy.integrate import solve_ivp
from scipy.linalg import eigh
from scipy.optimize import brentq

E_STEEL, NU = 210e9, 0.3
RHO_A = 7850.0


@functools.lru_cache(maxsize=1)
def registry():
    root = next(q for q in pathlib.Path(__file__).resolve().parents if q.name == "📕️norm")
    entries = json.loads((root / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").read_text(encoding="utf-8"))["entries"]
    return {e["id"]: e for e in entries}


def eps_of(fy_pa):
    return math.sqrt(235.0 / (fy_pa / 1e6))


def rho_internal(lam, psi=1.0):
    """📐 EN 1993-1-5 Eq. 4.2 (internal compression elements)."""
    return 1.0 if lam <= 0.5 + math.sqrt(0.085 - 0.055 * psi) else min((lam - 0.055 * (3 + psi)) / lam ** 2, 1.0)


def sigma_e(t, b):
    return math.pi ** 2 * E_STEEL * t ** 2 / (12 * (1 - NU ** 2) * b ** 2)


def lambda_p(b, t, fy, k_sigma):
    return (b / t) / (28.4 * eps_of(fy) * math.sqrt(k_sigma))


def governing_force(doc, combos, actions, key="force"):
    """⚖️ The largest |value| over the EN 1990 6.10a/b ultimate rows of the scalar actions of one panel (None without actions)."""
    cases = {c["id"]: c for c in doc["loadCases"]}
    rows = [(cases[a["loadCaseId"]], {"n": a[key], "vy": 0.0, "vz": 0.0, "my": 0.0, "mz": 0.0, "t": 0.0}) for a in actions if a["loadCaseId"] in cases]
    uls, _ = combos(doc, None, forces=rows)
    return max((abs(r["n"]) for r in uls), default=None)


def column_chi(lam, alpha_e):
    phi = 0.5 * (1 + alpha_e * (lam - 0.2) + lam ** 2)
    return 1.0 if lam <= 0.2 else min(1.0 / (phi + math.sqrt(max(phi * phi - lam * lam, 0.0))), 1.0)


def stiffened_plate(p, gm1):
    """🧱 EN 1993-1-5 4.5.4 plate-like and column-like buckling interpolated, resistance rho_c f_y / gamma_M1 (gross stiffener, beta_A,c = 1)."""
    a, b, t, fy = p["a"], p["b"], p["thickness"], p["fy"]
    sigma_cr_p = p["kSigma"] * sigma_e(t, b)
    sigma_cr_c = math.pi ** 2 * E_STEEL * p["stiffenerInertia"] / (p["stiffenerArea"] * a * a)
    i = math.sqrt(p["stiffenerInertia"] / p["stiffenerArea"])
    alpha_e = p["stiffenerAlpha"] + 0.09 / (i / p["stiffenerEccentricity"])
    chi_c = column_chi(math.sqrt(fy / sigma_cr_c), alpha_e)
    rho = rho_internal(lambda_p(b, t, fy, p["kSigma"]))
    xi = min(max(sigma_cr_p / sigma_cr_c - 1.0, 0.0), 1.0)
    rho_c = (rho - chi_c) * xi * (2 - xi) + chi_c
    return rho_c * fy / gm1


def shear_lag(p, hogging):
    """↔ EN 1993-1-5 3.3 / Table 3.1: kappa = alpha_0 b_0 / L_e, beta (sagging or hogging), beta_ult = beta^kappa."""
    b0, le, asl, t = p["halfWidth"], p["equivalentSpan"], p["stiffenerArea"], p["thickness"]
    kappa = math.sqrt(1 + asl / (b0 * t)) * b0 / le
    if kappa <= 0.02:
        beta = 1.0
    elif hogging:
        beta = 1 / (1 + 6.0 * (kappa - 1 / (2500 * kappa)) + 1.6 * kappa ** 2) if kappa <= 0.70 else 1 / (8.6 * kappa)
    else:
        beta = 1 / (1 + 6.4 * kappa ** 2) if kappa <= 0.70 else 1 / (5.9 * kappa)
    return kappa, beta, beta ** kappa


def k_tau(a, hw, t, i_sl):
    """📐 EN 1993-1-5 Annex A.3: k_tau of a web with up to two longitudinal stiffeners (one stiffener inertia lumped)."""
    ratio = a / hw
    base = 5.34 + 4.0 / ratio ** 2 if ratio >= 1 else 4.0 + 5.34 / ratio ** 2
    if i_sl <= 0:
        return base
    return base + max(9.0 / ratio ** 2 * (i_sl / (t ** 3 * hw)) ** 0.75, 2.1 / t * (i_sl / hw) ** (1 / 3))


def shear_resistance(p, gm1):
    """✂ EN 1993-1-5 5.2 with a non-rigid end post: V_bw,Rd + V_bf,Rd, capped by eta f_yw h_w t / (sqrt(3) gamma_M1), eta = 1.2 for S <= 460."""
    a, hw, t, fy = p["a"], p["b"], p["thickness"], p["fy"]
    eps = eps_of(fy)
    eta = 1.2
    kt = k_tau(a, hw, t, p["stiffenerInertia"])
    lam = hw / (37.4 * t * eps * math.sqrt(kt))
    chi = eta if lam < 0.83 / eta else 0.83 / lam
    v_bw = chi * fy * hw * t / (math.sqrt(3) * gm1)
    v_bf = 0.0
    if p["flangeWidth"] > 0 and p["flangeThickness"] > 0 and p["flangeFy"] > 0 and p["flangeMoment"] > 0:
        bf, tf, fyf = p["flangeWidth"], p["flangeThickness"], p["flangeFy"]
        c = a * (0.25 + 1.6 * bf * tf ** 2 * fyf / (t * hw ** 2 * fy))
        v_bf = bf * tf ** 2 * fyf / (c * gm1) * max(1 - (p["bendingMoment"] / p["flangeMoment"]) ** 2, 0.0)
    cap = eta * fy * hw * t / (math.sqrt(3) * gm1)
    return min(v_bw + v_bf, cap), v_bw


def patch(p, kind, gm1):
    """📍 EN 1993-1-5 6.2 to 6.5: F_Rd = f_yw L_eff t_w / gamma_M1 with l_y of types a, b (6.8) and c (6.10), chi_F = 0.5 / lambda_F <= 1."""
    a, hw, tw, fyw = p["a"], p["b"], p["thickness"], p["fy"]
    tf, bf, fyf, ss, c = p["flangeThickness"], p["flangeWidth"], p["flangeFy"], p["patchLength"], p["patchOffset"]
    m1 = fyf * bf / (fyw * tw)
    k_f = {"Through": 6 + 2 * (hw / a) ** 2, "OneFlangeNearEnd": 3.5 + 2 * (hw / a) ** 2, "EndFlange": min(2 + 6 * (ss + c) / hw, 6.0)}[kind]
    f_cr = 0.9 * k_f * E_STEEL * tw ** 3 / hw

    def l_y(m2):
        if kind == "Through":
            return min(ss + 2 * tf * (1 + math.sqrt(m1 + m2)), a)
        le = min(k_f * E_STEEL * tw ** 2 / (2 * fyw * hw), ss + c)
        terms = [le + tf * math.sqrt(m1 / 2 + (le / tf) ** 2 + m2), le + tf * math.sqrt(m1 + m2)]
        return min(terms + ([ss + c] if kind == "EndFlange" else []))

    ly = l_y(0.0)
    lam_f = math.sqrt(ly * tw * fyw / f_cr)
    if lam_f > 0.5:
        m2 = 0.02 * (hw / tf) ** 2
        ly = l_y(m2)
        lam_f = math.sqrt(ly * tw * fyw / f_cr)
    chi_f = min(0.5 / lam_f, 1.0)
    return fyw * chi_f * ly * tw / gm1


def mv_interaction(p, gm1):
    """⚖ EN 1993-1-5 7.1: eta_1 + (1 - M_f,Rd / M_pl,Rd)(2 eta_3 - 1)^2 <= 1 for eta_3 > 0.5."""
    _, v_bw = shear_resistance({**p, "flangeWidth": 0.0}, gm1)
    eta1, eta3 = p["bendingMoment"] / p["plasticMoment"], p["shearForce"] / v_bw
    if eta3 <= 0.5 or p["bendingMoment"] < p["flangeMoment"]:
        return max(eta1, eta3)
    return eta1 + (1 - p["flangeMoment"] / p["plasticMoment"]) * (2 * eta3 - 1) ** 2


def tee_constants(p):
    """🔩 St Venant constant and polar moment about the attachment point of a flat or tee stiffener outline by `sectionproperties` (finite elements on the solid outline)."""
    from sectionproperties.analysis import Section
    from sectionproperties.pre.library import rectangular_section
    from sectionproperties.pre import Geometry

    h, t = p["stiffenerDepth"] * 1e3, p["stiffenerThickness"] * 1e3
    web = rectangular_section(d=h, b=t).shift_section(-t / 2, 0.0)
    shape = web
    if p["stiffenerShape"] == "Tee":
        bf, tf = p["stiffenerFlangeWidth"] * 1e3, p["stiffenerFlangeThickness"] * 1e3
        shape = web + rectangular_section(d=tf, b=bf).shift_section(-bf / 2, h)
    shape.create_mesh(mesh_sizes=[min(t, 10.0) / 2])
    section = Section(shape)
    section.calculate_geometric_properties()
    section.calculate_warping_properties()
    ixx, iyy, _ = section.get_ic()
    area, (cx, cy) = section.get_area(), section.get_c()
    polar = ixx + iyy + area * (cx ** 2 + cy ** 2)
    return section.get_j() * 1e-12, polar * 1e-12


def plated_rows(doc, p, params):
    gm0, gm1 = params["gm0"], params["gm1"]
    pid, fy = p["id"], p["fy"]
    out = []
    sigma = governing_force(doc, plated_rows.combos, p["actions"])
    if sigma is not None:
        rd = fy * rho_internal(lambda_p(p["b"], p["thickness"], fy, p["kSigma"])) / gm0
        out.append((f"en1993.1-5.plate.{pid}", sigma / rd))
    if p["stiffenerInertia"] > 0 and p["stiffenerArea"] > 0 and p["compressionStress"] > 0:
        out.append((f"en1993.1-5.4.5.{pid}", p["compressionStress"] / stiffened_plate(p, gm1), ["ndp.en1993.v-362.1"]))
    if p["halfWidth"] > 0 and p["equivalentSpan"] > 0 and p["compressionStress"] > 0:
        _, _, beta_ult = shear_lag(p, p["hogging"] == 1)
        out.append((f"en1993.1-5.3.sl.{pid}", p["compressionStress"] / beta_ult / (fy / gm1), ["ndp.en1993.v-360.1"]))
    if p["patchType"] in ("Through", "OneFlangeNearEnd", "EndFlange") and p["patchForce"] > 0 and p["a"] > 0 and p["flangeThickness"] > 0 and p["longitudinalInertia"] <= 0:
        tag = {"Through": "a", "OneFlangeNearEnd": "b", "EndFlange": "c"}[p["patchType"]]
        f_rd = patch(p, p["patchType"], gm1)
        out.append((f"en1993.1-5.6.{tag}.{pid}", p["patchForce"] / f_rd, [] if tag == "a" else ["ndp.en1993.v-371.1"]))
        if p["patchMomentResistance"] > 0:
            out.append((f"en1993.1-5.7.2.{pid}", (p["patchForce"] / f_rd + 0.8 * p["patchMoment"] / p["patchMomentResistance"]) / 1.4))
    if p["shearForce"] != 0 and p["a"] > 0 and p["b"] > 0 and p["thickness"] > 0 and fy > 0:
        out.append((f"en1993.1-5.5.panel.{pid}", abs(p["shearForce"]) / shear_resistance(p, gm1)[0]))
    if p["plasticMoment"] > 0 and p["shearForce"] > 0 and p["bendingMoment"] > 0:
        out.append((f"en1993.1-5.7.1.{pid}", mv_interaction(p, gm1), ["ndp.en1993.v-367.1"]))
    if p["transverseStiffenerInertia"] > 0 and p["a"] > 0:
        a, hw, t = p["a"], p["b"], p["thickness"]
        i_min = 1.5 * hw ** 3 * t ** 3 / a ** 2 if a / hw < math.sqrt(2) else 0.75 * hw * t ** 3
        out.append((f"en1993.1-5.9.3.{pid}", i_min / p["transverseStiffenerInertia"], ["ndp.en1993.v-369.1"]))
        if p["stiffenerShape"] in ("Flat", "Tee") and p["stiffenerDepth"] > 0:
            it, ip = tee_constants(p)
            out.append((f"en1993.1-5.9.2.{pid}", 5.3 * fy / E_STEEL / (it / ip)))
    if p["lateralPressure"] > 0 and p["lateralPressureCoefficient"] > 0 and p["thickness"] > 0:
        sig = p["lateralPressureCoefficient"] * p["lateralPressure"] * p["b"] ** 2 / p["thickness"] ** 2
        out.append((f"en1993.1-7.lateral.{pid}", sig / (fy / gm0)))
    if p["flangeInducedK"] > 0 and p["flangeArea"] > 0 and p["flangeFy"] > 0:
        hw, t = p["b"], p["thickness"]
        out.append((f"en1993.1-5.8.{pid}", (hw / t) / (p["flangeInducedK"] * E_STEEL / p["flangeFy"] * math.sqrt(hw * t / p["flangeArea"]))))
    if p["alphaCr"] > 0 and (p["stressX"] > 0 or p["stressZ"] > 0 or p["shearStress"] > 0):
        sx, sz, tau = p["stressX"] / fy, p["stressZ"] / fy, p["shearStress"] / fy
        ult = 1 / math.sqrt(sx * sx + sz * sz - sx * sz + 3 * tau * tau)
        lam = math.sqrt(ult / p["alphaCr"])
        rho = rho_internal(lam)
        out.append((f"en1993.1-5.10.{pid}", gm1 / (rho * ult), ["ndp.en1993.v-372.1"]))
    return out


K_Y = [(20, 1.0), (100, 1.0), (200, 1.0), (300, 1.0), (400, 1.0), (500, 0.78), (600, 0.47), (700, 0.23), (800, 0.11), (900, 0.06), (1000, 0.04), (1100, 0.02), (1200, 0.0)]
K_E = [(20, 1.0), (100, 1.0), (200, 0.90), (300, 0.80), (400, 0.70), (500, 0.60), (600, 0.31), (700, 0.13), (800, 0.09), (900, 0.0675), (1000, 0.045), (1100, 0.0225), (1200, 0.0)]


def interp(table, theta):
    xs, ys = zip(*table)
    return float(np.interp(theta, xs, ys))


def c_steel(theta):
    """🔥 EN 1993-1-2 3.4.1.2 specific heat of carbon steel [J/(kg K)]."""
    if theta < 600:
        return 425 + 0.773 * theta - 1.69e-3 * theta ** 2 + 2.22e-6 * theta ** 3
    if theta < 735:
        return 666 + 13002 / (738 - theta)
    if theta < 900:
        return 545 + 17820 / (theta - 731)
    return 650.0


def iso834(t_s):
    return 20 + 345 * math.log10(8 * t_s / 60 + 1)


def steel_temperature(minutes, am_v, d_p, lam_p, rho_p, c_p):
    """🔥 EN 1993-1-2 4.2.5.2 Eq. 4.27 integrated to `minutes` of ISO 834 by `solve_ivp` (the clause increment limited to non-negative while the gas heats)."""
    phi = c_p * rho_p / (c_steel(20.0) * RHO_A) * d_p * am_v if d_p > 1e-6 else 0.0

    def unprotected(t, y):
        net = 25.0 * (iso834(t) - y[0]) + 0.7 * 5.67e-8 * ((iso834(t) + 273.0) ** 4 - (y[0] + 273.0) ** 4)
        return [max(am_v / (c_steel(y[0]) * RHO_A) * net, 0.0)]

    def rhs(t, y):
        g_rate = 345 / math.log(10) * 8 / 60 / (8 * t / 60 + 1)
        base = lam_p / d_p * am_v / (c_steel(y[0]) * RHO_A) / (1 + phi / 3) * (iso834(t) - y[0])
        return [max(base - (math.exp(phi / 10) - 1) * g_rate, 0.0)]

    solution = solve_ivp(unprotected if d_p <= 1e-6 else rhs, (0.0, minutes * 60.0), [20.0], max_step=5.0, rtol=1e-8, atol=1e-8)
    return float(solution.y[0][-1])


def critical_temperature(mu0):
    return 39.19 * math.log(1 / (0.9674 * mu0 ** 3.833) - 1) + 482


def fire_rows(doc, fire, combos):
    fid = fire["id"]
    minutes = float(fire["rating"][1:])
    member = next((m for m in doc["members"] if m["id"] == fire["memberId"]), None)
    section = next((s for s in doc["sections"] if member and s["id"] == member["sectionId"]), None)
    material = next((m for m in doc["materials"] if member and m["id"] == member["materialId"]), None)
    heating = steel_temperature(minutes, fire["sectionFactor"], fire["protectionThickness"], fire["protectionConductivity"], fire["protectionDensity"], fire["protectionSpecificHeat"])
    theta_a = max(fire["designTemperature"], heating)
    class4 = False
    out = []
    if section and material:
        from_parent = fire_rows.section_class(section, material["fy"])
        class4 = from_parent == 4
    theta_cr = 350.0 if class4 else critical_temperature(fire["mu0"])

    def at(d_p):
        return steel_temperature(minutes, fire["sectionFactor"], d_p, fire["protectionConductivity"], fire["protectionDensity"], fire["protectionSpecificHeat"]) - theta_cr

    t_req = 0.0 if at(1e-6) <= 0 else brentq(at, 1e-6, 0.5, xtol=1e-7)
    out.append((f"en1993.1-2.prot.{fid}", t_req / fire["protectionThickness"] if fire["protectionThickness"] > 0 else math.inf))
    out.append((f"en1993.1-2.theta.{fid}", theta_a / theta_cr))
    if member and section and material:
        ky, ke = interp(K_Y, theta_a), interp(K_E, theta_a)
        fy = material["fy"]
        lam = member["bucklingLengthZ"] / math.sqrt(section["iz"] / section["area"]) / (math.pi * math.sqrt(E_STEEL / fy))
        lam_t = lam * math.sqrt(ky / ke) if ke > 0 else math.inf
        alpha = 0.65 * math.sqrt(235e6 / fy)
        phi = 0.5 * (1 + alpha * lam_t + lam_t ** 2)
        chi = 1 / (phi + math.sqrt(phi * phi - lam_t * lam_t))
        ns = [r["n"] for r in combos(doc, member["id"])[0]]
        n_ed = max([0.0] + ns)
        if n_ed > 0:
            out.append((f"en1993.1-2.nfi.{fid}", 0.65 * n_ed / (chi * section["area"] * ky * fy / 1.0)))
    return out


def fatigue_rows(fat):
    """🔄 EN 1993-1-9 7 and 8: Miner sum with N_R from the S-N curve (m = 3 to 5e6, m = 5 to the cut-off at 1e8) at Delta sigma_C / gamma_Mf; gamma_Mf = 1.0 (damage tolerant) or 1.15 (safe life) for low consequence."""
    gmf = {"DamageTolerant": 1.0, "SafeLife": 1.15}[fat["method"]]
    dc = fat["category"] * 1e6 / gmf
    dd = (2 / 5) ** (1 / 3) * dc
    dl = (5 / 100) ** (1 / 5) * dd
    if not fat["spectrum"]:
        return []
    damage = 0.0
    for band in fat["spectrum"]:
        ds, n = band["deltaSigma"], band["cycles"]
        if ds < dl:
            continue
        nr = 2e6 * (dc / ds) ** 3 if ds >= dd else 5e6 * (dd / ds) ** 5
        damage += n / nr
    return [(f"en1993.1-9.fat.{fat['id']}", damage)]


def cold_formed_row(doc, cf):
    sigma = governing_force(doc, cold_formed_row.combos, cf["actions"])
    if sigma is None:
        return []
    lam = lambda_p(cf["bBar"], cf["thickness"], cf["fy"], cf["kSigma"])
    return [(f"en1993.1-3.cf.{cf['id']}", sigma / (rho_internal(lam, cf["psi"]) * cf["grossResistance"]))]


EDGE = {"Fixed": (True, True, True), "Pinned": (True, True, False), "Free": (False, False, False), "Roller": (False, True, False)}


def frame_modes(doc):
    """🕸 (alpha_cr, sway) of the first eight elastic buckling modes of the planar frame, lowest first; the load factors are those of the planar frame of `frameNodes` and `frameBars` under the node loads: Euler-Bernoulli bars (4 elements each, cubic Hermite stiffness and consistent geometric stiffness of the first-order axial force), pinned bar ends keep their own rotation."""
    nodes = {n["id"]: n for n in doc["frameNodes"]}
    index, pts = {}, []
    for n in doc["frameNodes"]:
        index[n["id"]] = len(pts)
        pts.append((n["x"], n["z"]))
    rot = {}
    elements = []
    segments = 4
    dof_count = 0
    dofs_of = {}
    for node_id, i in index.items():
        dofs_of[node_id] = [3 * i, 3 * i + 1, 3 * i + 2]
    dof_count = 3 * len(pts)
    for bar in doc["frameBars"]:
        member = next(m for m in doc["members"] if m["id"] == bar["memberId"])
        section = next(s for s in doc["sections"] if s["id"] == member["sectionId"])
        material = next(m for m in doc["materials"] if m["id"] == member["materialId"])
        a_id, b_id = bar["startNodeId"], bar["endNodeId"]
        (xa, za), (xb, zb) = (nodes[a_id]["x"], nodes[a_id]["z"]), (nodes[b_id]["x"], nodes[b_id]["z"])
        chain = [list(dofs_of[a_id])]
        for k in range(1, segments):
            chain.append([dof_count, dof_count + 1, dof_count + 2])
            pts.append((xa + (xb - xa) * k / segments, za + (zb - za) * k / segments))
            dof_count += 3
        chain.append(list(dofs_of[b_id]))
        for end, slot in ((bar["releaseStart"], 0), (bar["releaseEnd"], -1)):
            if end == "Pinned":
                chain[slot] = [chain[slot][0], chain[slot][1], dof_count]
                dof_count += 1
        length = math.hypot(xb - xa, zb - za) / segments
        c, s = (xb - xa) / math.hypot(xb - xa, zb - za), (zb - za) / math.hypot(xb - xa, zb - za)
        for d0, d1 in zip(chain[:-1], chain[1:]):
            elements.append((d0 + d1, length, c, s, material["eModulus"], section["area"], section["iy"]))
    k_mat = np.zeros((dof_count, dof_count))

    def local(length, ea, ei):
        k = np.zeros((6, 6))
        k[np.ix_([0, 3], [0, 3])] = ea / length * np.array([[1, -1], [-1, 1]])
        l = length
        k[np.ix_([1, 2, 4, 5], [1, 2, 4, 5])] = ei / l ** 3 * np.array([[12, 6 * l, -12, 6 * l], [6 * l, 4 * l * l, -6 * l, 2 * l * l], [-12, -6 * l, 12, -6 * l], [6 * l, 2 * l * l, -6 * l, 4 * l * l]])
        return k

    def geometric(length, n):
        l = length
        g = np.zeros((6, 6))
        g[np.ix_([1, 2, 4, 5], [1, 2, 4, 5])] = n / (30 * l) * np.array([[36, 3 * l, -36, 3 * l], [3 * l, 4 * l * l, -3 * l, -l * l], [-36, -3 * l, 36, -3 * l], [3 * l, -l * l, -3 * l, 4 * l * l]])
        return g

    def rotation(c, s):
        r = np.zeros((6, 6))
        for o in (0, 3):
            r[o:o + 3, o:o + 3] = [[c, s, 0], [-s, c, 0], [0, 0, 1]]
        return r

    for dofs, length, c, s, e, a, i in elements:
        r = rotation(c, s)
        k_mat[np.ix_(dofs, dofs)] += r.T @ local(length, e * a, e * i) @ r
    force = np.zeros(dof_count)
    held = set()
    for n in doc["frameNodes"]:
        d = dofs_of[n["id"]]
        force[d[0]] += n["loadX"]
        force[d[1]] += n["loadZ"]
        for flag, dof in zip(EDGE[n["support"]], d):
            if flag:
                held.add(dof)
    free = [d for d in range(dof_count) if d not in held]
    u = np.zeros(dof_count)
    u[free] = np.linalg.solve(k_mat[np.ix_(free, free)], force[free])
    g_mat = np.zeros((dof_count, dof_count))
    for dofs, length, c, s, e, a, i in elements:
        r = rotation(c, s)
        loc = r @ u[dofs]
        n_axial = e * a / length * (loc[3] - loc[0])
        g_mat[np.ix_(dofs, dofs)] += r.T @ geometric(length, n_axial) @ r
    mu, vectors = eigh(-g_mat[np.ix_(free, free)], k_mat[np.ix_(free, free)])
    modes = sorted(((1 / m, vectors[:, k]) for k, m in enumerate(mu) if m > 1e-12), key=lambda mode: mode[0])
    corners = [dofs_of[n["id"]] for n in doc["frameNodes"]]
    out = []
    for alpha, vector in modes[:8]:
        shape = np.zeros(dof_count)
        shape[free] = vector
        out.append((alpha, max(abs(shape[c[0]]) for c in corners) >= max(abs(shape[c[1]]) for c in corners)))
    return out


def frame_alpha(doc):
    """🕸 The elastic critical load factors of `frame_modes`, lowest first."""
    return [alpha for alpha, _ in frame_modes(doc)]


def frame_governing(doc):
    """🎯 EN 1993-1-1 5.2.1: the lowest sway mode (largest corner displacement horizontal) of the first eight modes, else the lowest mode."""
    modes = frame_modes(doc)
    return next((alpha for alpha, sway in modes if sway), modes[0][0])


def storey_rows(doc, storey, frame_value):
    sid = storey["id"]
    horne = None
    if storey["verticalLoad"] > 0 and storey["drift"] > 0:
        horne = storey["horizontalLoad"] / storey["verticalLoad"] * storey["height"] / storey["drift"]
    out = []
    limit = registry()["ndp.en1993.1-1.alpha-cr-min"]["recommended"]
    if horne is not None:
        eigen = frame_value
        alpha = eigen if eigen is not None else horne
        if eigen is not None:
            tol = registry()["ndp.en1993.v-026.2"]["recommended"]
            out.append((f"en1993.1-1.5.2.horne.{sid}", max(horne / eigen - 1, 0.0) / tol, ["ndp.en1993.v-026.2"]))
        out.append((f"en1993.1-1.5.2.alpha.{sid}", limit / alpha))
    if storey["verticalLoad"] > 0 and storey["buildingHeight"] > 0:
        m = max(storey["columns"], 1)
        phi = 1 / 200 * min(max(2 / math.sqrt(storey["buildingHeight"]), 2 / 3), 1.0) * math.sqrt(0.5 * (1 + 1 / m))
        out.append((f"en1993.1-1.5.3.2.sway.{sid}", 0.0, "info"))
    if storey["bracingLoad"] > 0 and storey["bracingLength"] > 0:
        div = registry()["ndp.en1993.bracing.deflection-limit-divisor"]["recommended"]
        out.append((f"en1993.1-1.5.3.3.bracing.{sid}", storey["bracingDeflection"] / (storey["bracingLength"] / div), ["ndp.en1993.bracing.deflection-limit-divisor", "ndp.en1993.bracing.imperfection-divisor"]))
    return out


def hole_clearance(d_mm):
    """⚙ EN 1090-2 nominal clearance: 1 mm up to M14, 2 mm up to M24, 3 mm above."""
    return d_mm + (1 if d_mm <= 14 else 2 if d_mm <= 24 else 3)


def joint_ultimate(doc, j, combos, key):
    """⚖ The ultimate scalar joint force of largest magnitude (EN 1990 6.10a/b rows of the joint actions), signed."""
    cases = {c["id"]: c for c in doc["loadCases"]}
    rows = [(cases[a["loadCaseId"]], {"n": a[key], "vy": 0.0, "vz": 0.0, "my": 0.0, "mz": 0.0, "t": 0.0}) for a in j["actions"] if a["loadCaseId"] in cases]
    return max((r["n"] for r in combos(doc, None, forces=rows)[0]), key=abs, default=0.0)


def throat_stresses(f_axis, f_leg, area):
    """🧵 EN 1993-1-8 Figure 4.5 by rotation: the weld force (axis x, leg direction y of an equal-leg fillet whose legs run along y and z) acts on the throat plane of normal (y + z) / sqrt 2; the traction F / (a l) is rotated onto the throat frame (axis, normal, in-plane transverse) and returns (sigma_perp, tau_perp, tau_par)."""
    traction = np.array([f_axis, f_leg, 0.0]) / area
    frame = np.array([[1.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, 1.0, -1.0]])
    frame[1:] /= np.sqrt(2.0)
    tau_par, sigma, tau_perp = frame @ traction
    return float(sigma), float(tau_perp), float(tau_par)


def joint_rows(doc, j, params, combos):
    """🔩 EN 1993-1-8 3.10.2 (block tearing, concentric loading: V_eff,1,Rd = f_u A_nt / gamma_M2 + f_y A_nv / (sqrt(3) gamma_M0) from the net areas of the tension face and the shear faces of the bolt pattern), Table 3.4 (punching B_p,Rd = 0.6 pi d_m t_p f_u / gamma_M2, d_m the mean of the across-flats and across-points width of the nut, rows from the registry, never wider than the regular hexagon of the same across flats), 4.10 (welded plate on an unstiffened flange: b_eff = t_w + 2 s + 7 k t_f <= b_p against f_y,p b_p / f_u,p) and 4.5.3.2 (directional method of a fillet weld: the longitudinal and transverse weld forces rotated onto the throat plane, both criteria sqrt(sigma_perp^2 + 3 (tau_perp^2 + tau_par^2)) <= f_u / (beta_w gamma_M2) and sigma_perp <= 0.9 f_u / gamma_M2)."""
    out, jid = [], j["id"]
    member = next(m for m in doc["members"] if m["id"] == j["memberId"])
    fy_member = next((m["fy"] for m in doc["materials"] if m["id"] == member["materialId"]), j["plateFu"] / 1.44)
    gm0, gm2 = params["gm0"], params["gm2"]
    shear, tension = joint_ultimate(doc, j, combos, "shear"), joint_ultimate(doc, j, combos, "tension")
    if j["kind"] == "Bolted" and j["boltRows"] > 0 and j["boltsPerRow"] > 0:
        if shear > 0:
            d0 = hole_clearance(j["boltDiameter"] * 1e3) / 1e3
            n1, n2, t = j["boltRows"], j["boltsPerRow"], j["plateThickness"]
            p2 = j["gauge"] if j["gauge"] > 0 else j["pitch"]
            l_v = max(j["endDistance"] + (n1 - 1) * j["pitch"] - (n1 - 0.5) * d0, 0.0)
            if n2 <= 1:
                a_nt, a_nv = max((j["edgeDistance"] - 0.5 * d0) * t, 0.0), l_v * t
            else:
                a_nt, a_nv = max((n2 - 1) * (p2 - d0) * t, 0.0), 2 * l_v * t
            v_rd = j["plateFu"] * a_nt / gm2 + fy_member * a_nv / (math.sqrt(3) * gm0)
            out.append((f"en1993.1-8.3.10.block.{jid}", shear / v_rd, ["ndp.en1993.v-405.1", "ndp.en1993.gamma-m0", "ndp.en1993.gamma-m2"]))
        if tension > 0:
            size = f"m{round(j['boltDiameter'] * 1e3)}"
            flats, points = (registry()[f"ndp.en1993.1-8.3.6.head-nut.{size}.across-{k}"]["recommended"] * 1e-3 for k in ("flats", "points"))
            assert points <= flats / math.cos(math.radians(30)), (flats, points)
            d_m = 0.5 * (flats + points)
            b_p = 0.6 * math.pi * d_m * j["plateThickness"] * j["plateFu"] / gm2
            out.append((f"en1993.1-8.3.6.punching.{jid}", tension / (j["boltRows"] * j["boltsPerRow"]) / b_p, ["ndp.en1993.v-402.1", "ndp.en1993.gamma-m2"]))
    if j["kind"] == "Welded" and min(j["columnFlangeThickness"], j["plateThickness"], j["plateWidth"], j["weldThroat"], j["plateFu"]) > 0:
        s_leg = math.sqrt(2) * j["weldThroat"]
        fy_f = j["columnFy"] if j["columnFy"] > 0 else fy_member
        fy_p = j["plateFy"] if j["plateFy"] > 0 else fy_member
        k = min(j["columnFlangeThickness"] / j["plateThickness"] * fy_f / fy_p, 1.0)
        b_eff = min(j["columnWebThickness"] + 2 * s_leg + 7 * k * j["columnFlangeThickness"], j["plateWidth"])
        out.append((f"en1993.1-8.4.11.beff.{jid}", fy_p / j["plateFu"] * j["plateWidth"] / b_eff, ["ndp.en1993.v-413.1"]))
    weld_par, weld_perp = (abs(j["weldForceLongitudinal"]) if j["weldForceLongitudinal"] != 0 else shear), j["weldForceTransverse"]
    if j["kind"] == "Welded" and (weld_par != 0 or weld_perp != 0) and j["weldThroat"] > 0 and j["weldLength"] > 0:
        beta = 0.8 if j["weldGrade"].startswith("S235") else 0.85 if j["weldGrade"].startswith("S275") else 0.9 if j["weldGrade"].startswith("S355") else 1.0
        sigma, tau_perp, tau_par = throat_stresses(weld_par, weld_perp, j["weldThroat"] * j["weldLength"])
        equiv = math.sqrt(sigma**2 + 3 * (tau_perp**2 + tau_par**2))
        out.append((f"en1993.1-8.4.5.directional.{jid}", max(equiv * beta * gm2 / j["weldFu"], abs(sigma) * gm2 / (0.9 * j["weldFu"])), ["ndp.en1993.gamma-m2"]))
    return out


def weld_group_row(g, params):
    """🧵 EN 1993-1-8 4.5.3.3 elastic vector method: weld lines integrated numerically (midpoint rule, 4000 cells per line), the force per length at the weld end points F/L_w in the load direction plus the torsion T r / I_p, against f_u / (sqrt(3) beta_w gamma_M2) a."""
    if min(g["throat"], g["depth"]) <= 0 or g["shearForce"] == 0 or g["steelFu"] <= 0 or g["betaW"] <= 0:
        return []
    b, d = g["width"] * 1e3, g["depth"] * 1e3
    lines = [((0.0, -d / 2), (0.0, d / 2))] if g["pattern"] == "Line" else [((-b / 2, -d / 2), (-b / 2, d / 2)), ((b / 2, -d / 2), (b / 2, d / 2))]
    if g["pattern"] == "Box":
        lines += [((-b / 2, -d / 2), (b / 2, -d / 2)), ((-b / 2, d / 2), (b / 2, d / 2))]
    cells = 4000
    total = polar = 0.0
    for (x0, y0), (x1, y1) in lines:
        ds = math.hypot(x1 - x0, y1 - y0) / cells
        for i in range(cells):
            t = (i + 0.5) / cells
            polar += ((x0 + t * (x1 - x0)) ** 2 + (y0 + t * (y1 - y0)) ** 2) * ds
        total += ds * cells
    force = abs(g["shearForce"])
    torque = force * g["eccentricity"] * 1e3
    peak = max(math.hypot(force / total + torque * x / polar, torque * y / polar) for line in lines for x, y in line)
    return [(f"en1993.1-8.4.5.group.{g['id']}", peak / (g["steelFu"] / 1e6 / math.sqrt(3) / (g["betaW"] * params["gm2"]) * g["throat"] * 1e3), ["ndp.en1993.gamma-m2"])]


def ndp_value(name):
    return registry()["ndp.en1993." + name]["recommended"]


def stress_range_rows(doc, row):
    """⏳ EN 1993-1-9 5.1 to 5.4 and 7.1: fibre stresses of the two load cases from the member actions, the range with the compressive part reduced by the registry factor for non-welded and stress-relieved details, gamma_Ff k_f, the size effect (25/t)^0.2 and the damage n / N_R on the S-N curve of the category."""
    detail = next((d for d in doc["fatigueDetails"] if d["id"] == row["detailId"]), None)
    member = next((m for m in doc["members"] if detail and m["id"] == detail["memberId"]), None)
    section = next((x for x in doc["sections"] if member and x["id"] == member["sectionId"]), None)
    if not (detail and member and section):
        return []

    def stress(case):
        acts = [a["action"] for a in doc["memberActions"] if a["memberId"] == member["id"] and a["loadCaseId"] == case]
        if not acts:
            return None
        n, my, mz = (sum(a[k] for a in acts) for k in ("n", "my", "mz"))
        return n / section["area"] + my * row["fibreZ"] / section["iy"] - mz * row["fibreY"] / section["iz"]

    hi, lo = stress(row["maxCaseId"]), stress(row["minCaseId"])
    if hi is None or lo is None:
        return []
    hi, lo = max(hi, lo), min(hi, lo)
    factor = ndp_value("1-9.stress-range.compression-factor")
    cut = (lambda v: factor * v if v < 0 else v) if row["relief"] != "AsWelded" else (lambda v: v)
    design = ndp_value("1-9.gamma-ff") * (row["concentration"] or 1.0) * (cut(hi) - cut(lo))
    gmf = {"DamageTolerant": 1.0, "SafeLife": 1.15}[detail["method"]]
    dc = detail["category"] * 1e6 * ((25 / (row["thickness"] * 1e3)) ** 0.2 if row["thickness"] * 1e3 > 25 else 1.0) / gmf
    dd = (2 / 5) ** (1 / 3) * dc
    dl = (5 / 100) ** (1 / 5) * dd
    if design < dl:
        damage = 0.0
    else:
        damage = row["cycles"] / (2e6 * (dc / design) ** 3 if design >= dd else 5e6 * (dd / design) ** 5)
    return [(f"en1993.1-9.5.range.{row['id']}", damage, ["ndp.en1993.1-9.gamma-ff"])]


def effective_length_rows(doc, row):
    """🪜 EN 1993-1-1 6.3.1.3: K from the textbook nomogram equations (tangent form) by brentq on a pole-free bracket, L_cr = K L over the buckling length entered at the member."""
    member = next((m for m in doc["members"] if m["id"] == row["memberId"]), None)
    if not member:
        return []
    big = 1e9
    g = lambda side: 0.0 if row["restraint" + side] == "Fixed" else big if row["restraint" + side] == "Pinned" else (min(row["columnStiffness" + side] / row["beamStiffness" + side], big) if row["beamStiffness" + side] > 0 else big)
    ga, gb = g("A"), g("B")
    if row["sway"] == "NonSway":
        f = lambda x: ga * gb * x * x * math.sin(x) / 4 + (ga + gb) * (math.sin(x) - x * math.cos(x)) / 2 + 2 * (1 - math.cos(x)) / x - math.sin(x)
        k = math.pi / brentq(f, math.pi + 1e-12, 2 * math.pi - 1e-12) if f(2 * math.pi - 1e-12) < 0 else 0.5
    else:
        f = lambda x: (ga * gb * x * x - 36) * math.sin(x) - 6 * (ga + gb) * x * math.cos(x)
        k = math.pi / brentq(f, 1e-9, math.pi - 1e-12) if f(math.pi - 1e-12) > 0 else 1.0
    entered = member["bucklingLengthY"] if row["plane"] == "MajorAxis" else member["bucklingLengthZ"]
    return [(f"en1993.1-1.6.3.1.3.lcr.{row['id']}", k * member["length"] / entered, [])]


def fastener_rows(f, params):
    """🪛 EN 1993-1-3 8.3: bearing on the thinner sheet with interpolation over t2/t1, pull-over and pull-out of screws, net section, interaction and spacing, every coefficient read from the registry rows."""
    kind = "screw" if f["kind"] == "SelfTappingScrew" else "rivet"
    c = lambda name: ndp_value(f"1-3.fastener.{kind}.{name}")
    gm2, d, n = params["gm2"], f["diameter"], f["count"]
    if f["headThickness"] <= f["baseThickness"]:
        t1, fu1, t2 = f["headThickness"], f["headStrength"], f["baseThickness"]
    else:
        t1, fu1, t2 = f["baseThickness"], f["baseStrength"], f["headThickness"]
    equal, thick = c("bearing-equal") * math.sqrt(t1**3 * d) * fu1 / gm2, c("bearing-thick") * t1 * d * fu1 / gm2
    bearing = float(np.interp(t2 / t1, [1.0, ndp_value("1-3.fastener.thickness-ratio")], [equal, thick]))
    shear = min(bearing, f["shearResistance"]) if f["shearResistance"] > 0 else bearing
    ids = [e for e in registry() if e.startswith("ndp.en1993.1-3.fastener.")]
    out = []
    u_v = u_t = None
    if f["shearForce"]:
        u_v = abs(f["shearForce"]) / n / shear
        out.append((f"en1993.1-3.8.3.shear.{f['id']}", u_v, ids))
        if f["sheetWidth"] > 0:
            net = (f["sheetWidth"] - f["perRow"] * d) * t1 * fu1 / gm2
            out.append((f"en1993.1-3.8.3.net.{f['id']}", abs(f["shearForce"]) / net, ids))
    if f["tensionForce"] and kind == "screw" and f["washerDiameter"] > 0:
        over = ndp_value("1-3.fastener.screw.pull-over") * f["washerDiameter"] * f["headThickness"] * f["headStrength"] / gm2
        pull = ndp_value("1-3.fastener.screw.pull-out") * d * f["baseThickness"] * f["baseStrength"] / gm2
        u_t = abs(f["tensionForce"]) / n / min(over, pull)
        out.append((f"en1993.1-3.8.3.tension.{f['id']}", u_t, ids))
        if u_v is not None:
            out.append((f"en1993.1-3.8.3.interaction.{f['id']}", u_v + u_t, ids))
    return out


def forming_rows(doc, g):
    """🥫 EN 1993-1-3 3.2.2: f_ya = f_yb + (f_u - f_yb) k n t^2 / A_g limited to (f_u + f_yb) / 2; the yield strength used over f_ya."""
    cf = next((m for m in doc["coldFormedMembers"] if m["id"] == g["memberId"]), None)
    if not cf or cf["fy"] <= 0 or cf["thickness"] <= 0 or g["grossArea"] <= 0 or g["ultimateStrength"] <= cf["fy"]:
        return []
    k = ndp_value("1-3.fya.k-roll" if g["forming"] == "RollForming" else "1-3.fya.k-other")
    radius_ok = not (cf["cornerRadius"] > 0 and cf["cornerRadius"] > ndp_value("1-3.fya.max-radius-ratio") * cf["thickness"])
    n = g["bends"] if radius_ok else 0
    fya = min(cf["fy"] + (g["ultimateStrength"] - cf["fy"]) * k * n * cf["thickness"] ** 2 / g["grossArea"], (g["ultimateStrength"] + cf["fy"]) / 2)
    used = g["usedYield"] or cf["fy"]
    return [(f"en1993.1-3.3.2.2.fya.{g['id']}", used / max(fya, cf["fy"]), ["ndp.en1993.1-3.fya.k-roll"])]


def ductility_rows(doc, row):
    """🧪 EN 1993-1-1 3.2.2: elongation at failure over 15 % and 15 eps_y over eps_u of the certificate values."""
    mat = next((m for m in doc["materials"] if m["id"] == row["materialId"]), None)
    if not mat:
        return []
    out = []
    if row["elongation"] > 0:
        out.append((f"en1993.3.2.2.elongation.{row['id']}", ndp_value("1-1.ductility.elongation") / row["elongation"], ["ndp.en1993.v-011.1"]))
    if row["ultimateStrain"] > 0 and mat["eModulus"] > 0:
        out.append((f"en1993.3.2.2.strain.{row['id']}", ndp_value("1-1.ductility.eps-u-factor") * mat["fy"] / mat["eModulus"] / row["ultimateStrain"], ["ndp.en1993.v-011.1"]))
    return out


def frame_row(doc):
    """🕸 EN 1993-1-1 5.2.1: alpha_cr of the governing mode against the lower bound `alpha-cr-min`, anchored on the first frame bar like the check."""
    limit = registry()["ndp.en1993.1-1.alpha-cr-min"]["recommended"]
    return [(f"en1993.1-1.5.2.frame.{doc['frameBars'][0]['id']}", limit / frame_governing(doc), ["ndp.en1993.1-1.alpha-cr-min"])]


def evaluate(doc, combos, params, section_class):
    plated_rows.combos = combos
    cold_formed_row.combos = combos
    fire_rows.section_class = section_class
    rows = []
    for p in doc.get("platedPanels", []):
        rows += plated_rows(doc, p, params)
    for fat in doc.get("fatigueDetails", []):
        rows += fatigue_rows(fat)
    for fire in doc.get("fireExposures", []):
        rows += fire_rows(doc, fire, combos)
    for cf in doc.get("coldFormedMembers", []):
        rows += cold_formed_row(doc, cf)
    tension_rows.combos = combos
    rows += restraint_rows(doc, section_class, params)
    for j in doc.get("hollowJoints", []):
        rows += hollow_rows(j)
    for c in doc.get("tensionComponents", []):
        rows += tension_rows(doc, c, params)
    for j in doc["joints"]:
        rows += joint_rows(doc, j, params, combos)
    for g in doc.get("weldGroups", []):
        rows += weld_group_row(g, params)
    for row in doc.get("fatigueStresses", []):
        rows += stress_range_rows(doc, row)
    for row in doc.get("bucklingLengths", []):
        rows += effective_length_rows(doc, row)
    for row in doc.get("fastenedJoints", []):
        rows += fastener_rows(row, params)
    for row in doc.get("formingGains", []):
        rows += forming_rows(doc, row)
    for row in doc.get("materialDuctility", []):
        rows += ductility_rows(doc, row)
    if doc.get("frameBars") and not doc.get("frameActions"):
        rows += frame_row(doc)
    if doc.get("storeys"):
        eigen = frame_alpha(doc)[0] if doc.get("frameBars") else None
        for storey in doc["storeys"]:
            rows += storey_rows(doc, storey, eigen)
    out = []
    entries = registry()
    for row in rows:
        if len(row) == 3 and isinstance(row[2], list):
            unverified = any(entries[n]["status"] != "verified" for n in row[2])
            out.append({"id": row[0], "utilization": row[1], "status": "fail" if row[1] > 1.0 + 1e-9 else "unverifiedBasis" if unverified else "pass"})
        elif len(row) == 3:
            out.append({"id": row[0], "utilization": row[1], "status": row[2]})
        else:
            out.append({"id": row[0], "utilization": row[1]})
    return out


def bimoment_max(length, ei_w, g_it, torque, x_a, udl, support):
    """🌀 Largest |B| = |E I_w theta''| of the warping torsion E I_w theta'''' - G I_t theta'' = m_t for a torque T at x_a (or T/L per length) with fork or fixed ends; the closed-form solution theta = A + B x + C cosh(kx) + D sinh(kx) (+ particular) fitted per segment by `numpy.linalg.solve` (None for other supports)."""
    if support not in ("Fork", "Fixed"):
        return None
    k = math.sqrt(g_it / ei_w)
    pieces = [(0.0, length)] if udl else [(0.0, x_a), (x_a, length)]
    pieces = [(a, b) for a, b in pieces if b - a > 1e-12]
    m = torque / length if udl else 0.0

    def basis(x):
        c, s = math.cosh(k * x), math.sinh(k * x)
        return np.array([[1, x, c, s], [0, 1, k * s, k * c], [0, 0, k * k * c, k * k * s], [0, 0, k ** 3 * s, k ** 3 * c]])

    def particular(x):
        return np.array([-m * x * x / (2 * g_it), -m * x / g_it, -m / g_it, 0.0])

    n = 4 * len(pieces)
    mat, rhs = np.zeros((n, n)), np.zeros(n)
    row = 0

    def end_rows(piece, x, order):
        nonlocal row
        b = basis(x)
        for d in order:
            mat[row, 4 * piece:4 * piece + 4] = b[d]
            rhs[row] = -particular(x)[d]
            row += 1

    order = (0, 2) if support == "Fork" else (0, 1)
    end_rows(0, pieces[0][0], order)
    end_rows(len(pieces) - 1, pieces[-1][1], order)
    if len(pieces) == 2:
        b = basis(pieces[0][1])
        for d in (0, 1, 2):
            mat[row, 0:4], mat[row, 4:8] = b[d], -b[d]
            row += 1
        mat[row, 0:4] = -ei_w * b[3] + g_it * b[1]
        mat[row, 4:8] = ei_w * b[3] - g_it * b[1]
        rhs[row] = -torque
        row += 1
    coefficients = np.linalg.solve(mat, rhs)
    peak = 0.0
    for i, (a, b) in enumerate(pieces):
        for x in np.linspace(a, b, 801):
            theta2 = basis(x)[2] @ coefficients[4 * i:4 * i + 4] + particular(x)[2]
            peak = max(peak, abs(ei_w * theta2))
    return peak


def torsion_notes(member, section, material, a, params):
    """🌀 EN 1993-1-1 6.2.7 rows: the St Venant shear tau = T t_max / I_t against f_y / sqrt(3) / gamma_M0 and the normal stress of axial force, bending and the warping bimoment against f_y / gamma_M0."""
    rows = []
    if abs(a["t"]) <= 0 or section["it"] <= 0:
        return rows
    t_max = max(section["tf"], section["tw"])
    rows.append(("en1993.6.2.7.t", abs(a["t"]) * t_max / section["it"] / (material["fy"] / math.sqrt(3) / params["gm0"])))
    if section["iw"] > 0 and member["length"] > 0 and section["kind"] in ("RolledI", "WeldedI"):
        udl = member["torsionLoad"] == "Udl"
        peak = bimoment_max(member["length"], material["eModulus"] * section["iw"], material["gModulus"] * section["it"], abs(a["t"]), member["torquePosition"] * member["length"], udl, member["warpingSupport"])
        if peak is not None:
            w_w = section["iw"] / (section["b"] * (section["h"] - section["tf"]) / 4)
            sigma = abs(a["n"]) / section["area"] + abs(a["my"]) / section["wElY"] + abs(a["mz"]) / section["wElZ"] + peak / w_w
            rows.append(("en1993.6.2.7.w", sigma / (material["fy"] / params["gm0"])))
    return rows


def c_factors(distribution, psi):
    if distribution == "Udl":
        return 1.127, 0.454
    if distribution == "Linear":
        table = [(-1.0, 2.752), (-0.75, 2.927), (-0.5, 2.704), (-0.25, 2.281), (0.0, 1.879), (0.25, 1.563), (0.5, 1.323), (0.75, 1.141), (1.0, 1.0)]
        return float(np.interp(min(max(psi, -1.0), 1.0), *zip(*table))), 0.0
    return 1.0, 0.0


def restraint_rows(doc, section_class, params):
    """🧷 EN 1993-1-1 6.3.2.2 (M_cr of a fork-fork beam by the closed form with C1, C2 and the load height), 6.3.4 (general method) and 6.3.5.3 (stable length L_m, Eq. 6.67) per restraint row without springs or intermediate supports."""
    rows = []
    for r in doc.get("restraints", []):
        member = next((m for m in doc["members"] if m["id"] == r["memberId"]), None)
        section = next((s for s in doc["sections"] if member and s["id"] == member["sectionId"]), None)
        material = next((m for m in doc["materials"] if member and m["id"] == member["materialId"]), None)
        if not (member and section and material) or section["kind"] != "RolledI":
            continue
        fy, gm1 = material["fy"], params["gm1"]
        cls = section_class(section, fy)
        if cls > 3:
            continue
        w = section["wPlY"] if cls <= 2 else section["wElY"]
        alpha_lt = 0.21 if section["h"] / section["b"] <= 2 else 0.34
        c1, c2 = c_factors(r["loadDistribution"], member["endMomentRatioPsi"])
        simple = r["startSupport"] == r["endSupport"] == "Fork" and r["intermediatePosition"] <= 0 and r["torsionalSpring"] <= 0 and r["lateralSpringStiffness"] <= 0
        mcr = None
        if simple and member["length"] > 0:
            l, e, g = member["length"], material["eModulus"], material["gModulus"]
            a = math.pi ** 2 * e * section["iz"] / l ** 2
            mcr = c1 * a * (math.sqrt(section["iw"] / section["iz"] + l * l * g * section["it"] / (math.pi ** 2 * e * section["iz"]) + (c2 * r["loadHeight"]) ** 2) - c2 * r["loadHeight"])
        if mcr and r["designMoment"] > 0:
            lam = math.sqrt(w * fy / mcr)
            chi_lt = 1.0 if lam <= 0.2 or r["designMoment"] / mcr <= 0.04 else column_chi(lam, alpha_lt)
            rows.append((f"en1993.1-1.6.3.2.mcrfe.{r['id']}", r["designMoment"] / (chi_lt * w * fy / gm1), ["ndp.en1993.v-113b.1"]))
            if r["generalMethod"] == 1:
                alpha_ult = 1 / (abs(r["designAxial"]) / (section["area"] * fy) + r["designMoment"] / (w * fy))
                n_cr_z = math.pi ** 2 * material["eModulus"] * section["iz"] / member["bucklingLengthZ"] ** 2
                alpha_cr = 1 / (abs(r["designAxial"]) / n_cr_z + r["designMoment"] / mcr)
                lam_op = math.sqrt(alpha_ult / alpha_cr)
                alpha = max(alpha_lt, (0.49 if fy < 460e6 else 0.21) if abs(r["designAxial"]) > 0 else 0.0)
                chi_op = 1.0 if lam_op <= 0.2 else column_chi(lam_op, alpha)
                rows.append((f"en1993.1-1.6.3.4.general.{r['id']}", gm1 / (chi_op * alpha_ult), ["ndp.en1993.v-123.1"]))
        if r["hingeToRestraint"] > 0:
            i_z = math.sqrt(section["iz"] / section["area"]) * 1e3
            c1_m = c_factors("Linear", member["endMomentRatioPsi"])[0]
            n_term = abs(r["designAxial"]) / (section["area"] * 1e6) / 57.4
            w_term = (section["wPlY"] * 1e9) ** 2 / (756 * c1_m ** 2 * section["area"] * 1e6 * section["it"] * 1e12) * (fy / 1e6 / 235) ** 2
            rows.append((f"en1993.1-1.6.3.5.lm.{r['id']}", r["hingeToRestraint"] / (38 * i_z / math.sqrt(n_term + w_term) / 1e3)))
    return rows



def tension_rows(doc, c, params):
    """🪢 EN 1993-1-11 6.2 (cable F_Ed / (F_uk / gamma_R)), EN 1993-1-8 3.6.1 (threaded rod k_t A_s f_u / gamma_M2) and Table 3.10 (pin: shear 0.6 A f_up, moment 1.5 W_el f_yp, bearing 1.5 t d f_y; M_Ed = F (b + 4c + 2a) / 8 with the middle plate b, the gap c and the outer plates a)."""
    n_ed = governing_force(doc, tension_rows.combos, c["actions"])
    if n_ed is None:
        return []
    cid, out = c["id"], []
    if c["partialFactor"] > 0:
        out.append((f"en1993.1-11.6.2.cable.{cid}", n_ed / (c["fUk"] / c["partialFactor"]), ["ndp.en1993.v-860.1"]))
    if c["threadFactor"] > 0 and c["rodStressArea"] > 0:
        out.append((f"en1993.1-8.3.6.rod.{cid}", n_ed / (c["threadFactor"] * c["rodStressArea"] * c["rodFu"] / params["gm2"])))
    if c["pinDiameter"] > 0:
        d, t = c["pinDiameter"], c["pinPlateThickness"]
        a_pin = math.pi * d * d / 4
        v_rd = 0.6 * a_pin * c["pinFup"] / params["gm2"]
        m_ed = n_ed * (c["pinInnerThickness"] + 4 * c["pinGap"] + 2 * c["pinOuterThickness"]) / 8
        m_rd = 1.5 * (math.pi * d ** 3 / 32) * c["pinFyp"] / params["gm0"]
        out.append((f"en1993.1-8.3.13.pin.{cid}", (m_ed / m_rd) ** 2 + (n_ed / 2 / v_rd) ** 2))
        out.append((f"en1993.1-8.3.13.pinb.{cid}", n_ed / (1.5 * t * d * c["pinPlateFy"] / params["gm0"])))
    return out



def hollow_rows(j):
    """🕳 EN 1993-1-8 7.4 and Tables 7.8, 7.11 (welded RHS K and N gap joints): chord face failure N_1,Rd = 8.9 k_n f_y0 t_0^2 sqrt(gamma) (b_1 + b_2 + h_1 + h_2) / (4 b_0 sin theta_1) / gamma_M5, N_2,Rd by the sine ratio, and the brace effective width N_i,Rd = f_yi t_i (2 h_i - 4 t_i + b_i + b_eff) / gamma_M5 (mm, N, MPa inside)."""
    if min(j["chordWidth"], j["chordThickness"], j["brace1Thickness"], j["brace2Thickness"], j["brace1Angle"], j["brace2Angle"], j["chordFy"]) <= 0:
        return []
    gm5 = registry()["ndp.en1993.1-8.gamma-m5"]["recommended"]
    b0, t0, fy0, fyi = j["chordWidth"] * 1e3, j["chordThickness"] * 1e3, j["chordFy"] / 1e6, j["braceFy"] / 1e6
    braces = [(j[f"brace{i}Width"] * 1e3, j[f"brace{i}Height"] * 1e3, j[f"brace{i}Thickness"] * 1e3, math.radians(j[f"brace{i}Angle"]), abs(j[f"brace{i}Force"])) for i in (1, 2)]
    beta = sum(b + h for b, h, _, _, _ in braces) / (4 * b0)
    n = j["chordStress"] / 1e6 / fy0
    k_n = 1.0 if n >= 0 else min(1.3 - 0.4 * n / beta, 1.0)
    n1_rd = 8.9 * k_n * fy0 * t0 ** 2 * math.sqrt(b0 / (2 * t0)) * beta / math.sin(braces[0][3]) / gm5
    face = max(braces[0][4] / n1_rd, braces[1][4] / (n1_rd * math.sin(braces[0][3]) / math.sin(braces[1][3])))
    ratios = []
    for b, h, t, _, force in braces:
        b_eff = min(10 / (b0 / t0) * fy0 * t0 / (fyi * t) * b, b)
        ratios.append(force / (fyi * t * (2 * h - 4 * t + b + b_eff) / gm5))
    return [(f"en1993.1-8.7.4.gap.{j['id']}", max(face, *ratios), ["ndp.en1993.1-8.gamma-m5", "ndp.en1993.v-440.1", "ndp.en1993.v-440.2", "ndp.en1993.v-440.3"])]



def self_check():
    """🧪 Fire: the clause increment scheme (explicit, 5 s) against `solve_ivp`; frame: the portal against the stability-function closed form (brentq); torsion constant of a flat bar against the thin-rectangle series."""
    ivp = steel_temperature(60, 150.0, 0.02, 0.2, 800.0, 1700.0)
    phi = 1700.0 * 800.0 / (c_steel(20.0) * RHO_A) * 0.02 * 150.0
    theta, g_prev = 20.0, 20.0
    for k in range(720):
        t = 5.0 * (k + 1)
        g = iso834(t)
        d = 0.2 / 0.02 * 150.0 / (c_steel(theta) * RHO_A) / (1 + phi / 3) * (iso834(t - 2.5) - theta) * 5.0 - (math.exp(phi / 10) - 1) * (g - g_prev)
        theta, g_prev = theta + max(d, 0.0), g
    assert abs(ivp - theta) <= 0.01 * theta, (ivp, theta)
    h, ic, ib, span, load = 4.0, 1.126e-4, 1.126e-4, 6.0, 200000.0
    kappa = 6 * ib * h / (ic * span)

    def stability(k):
        s = (k * math.sin(k) - k * k * math.cos(k)) / (2 - 2 * math.cos(k) - k * math.sin(k))
        c = (k * k - k * math.sin(k)) / (k * math.sin(k) - k * k * math.cos(k))
        return s, c

    def residual(k):
        s, c = stability(k)
        return s * (1 + c) * (s * (1 + c) / (s + kappa) - 2) + k * k

    grid = np.linspace(1e-3, math.pi - 1e-3, 4000)
    root = next(brentq(residual, a, b) for a, b in zip(grid[:-1], grid[1:]) if residual(a) < 0 <= residual(b))
    closed = root * root * E_STEEL * ic / (h * h * load)
    portal = {"frameNodes": [{"id": "a", "x": 0.0, "z": 0.0, "support": "Fixed", "loadX": 0.0, "loadZ": 0.0}, {"id": "b", "x": 0.0, "z": h, "support": "Free", "loadX": 0.0, "loadZ": -load}, {"id": "c", "x": span, "z": h, "support": "Free", "loadX": 0.0, "loadZ": -load}, {"id": "d", "x": span, "z": 0.0, "support": "Fixed", "loadX": 0.0, "loadZ": 0.0}],
              "frameBars": [{"id": f"bar-{i}", "memberId": "m", "startNodeId": s, "endNodeId": e, "releaseStart": "Rigid", "releaseEnd": "Rigid"} for i, (s, e) in enumerate((("a", "b"), ("b", "c"), ("d", "c")))],
              "members": [{"id": "m", "sectionId": "s", "materialId": "e"}], "sections": [{"id": "s", "area": 0.0106, "iy": ic}], "materials": [{"id": "e", "eModulus": E_STEEL}]}
    numeric = frame_alpha(portal)[0]
    assert abs(numeric - closed) <= 0.01 * closed, (numeric, closed)
    k = math.sqrt(81e9 * 1e-6 / (210e9 * 5e-7))
    peak = bimoment_max(6.0, 210e9 * 5e-7, 81e9 * 1e-6, 5000.0, 3.0, False, "Fork")
    exact = 5000.0 / (2 * k) * math.tanh(k * 3.0)
    assert abs(peak - exact) <= 1e-3 * exact, (peak, exact)
    j, _ = tee_constants({"stiffenerShape": "Flat", "stiffenerDepth": 0.12, "stiffenerThickness": 0.01})
    series = 0.12 * 0.01 ** 3 / 3 * (1 - 0.63 * 0.01 / 0.12)
    assert abs(j - series) <= 0.02 * series, (j, series)
    group = {"id": "g", "pattern": "TwoLines", "throat": 0.005, "width": 0.1, "depth": 0.2, "shearForce": 100000.0, "eccentricity": 0.15, "steelFu": 510e6, "betaW": 0.9}
    ip_closed = 2 * (200.0 ** 3 / 12 + 200.0 * 50.0 ** 2)
    q_closed = math.hypot(1e5 / 400.0 + 1e5 * 150.0 * 50.0 / ip_closed, 1e5 * 150.0 * 100.0 / ip_closed)
    resistance = 510 / math.sqrt(3) / (0.9 * 1.25) * 5.0
    assert abs(weld_group_row(group, {"gm2": 1.25})[0][1] - q_closed / resistance) <= 1e-6 * q_closed / resistance
    block = 510e3 * 380e-6 / 1.25 + 355e3 * 1340e-6 / math.sqrt(3)
    assert abs(block - 429.69) <= 0.01, block
    sigma, tau_perp, tau_par = throat_stresses(0.0, 250e3, 1000.0)
    assert abs(sigma - 250.0 / math.sqrt(2)) < 1e-9 and abs(tau_perp - sigma) < 1e-9 and tau_par == 0.0
    assert abs(math.sqrt(sigma**2 + 3 * tau_perp**2) / (490.0 / (0.9 * 1.25)) - 0.8117297233008836) < 1e-12
    assert abs(250.0 / (490.0 / (math.sqrt(3) * 0.9 * 1.25)) - 0.994161815568871) < 1e-12
    return {"fireIvp": ivp, "fireClauseScheme": theta, "portalNumeric": numeric, "portalClosedForm": closed, "flatBarJ": j, "flatBarSeries": series, "bimoment": peak, "bimomentClosedForm": exact}
