"""🌍 Independent EN 1998 families beyond the building base shear, written from the clause text and the textbook closed forms (never from the Rust): ground type by Table 3.1 (EN 1998-1 3.1.2), importance and regularity claims (4.2.3, 4.2.5), the retrofit assessment capacity with confidence factor and limit-state return period (EN 1998-3 2.1, 3.3), the bridge period ratio and capacity-design pier shear (EN 1998-2), tank base shear with the Housner masses and the sloshing wave (EN 1998-4 Annex A), the silo shell and anchorage, the foundation stiffness ratio (EN 1998-5) and the tower overturning moment on the clamped-free beam eigenvalue found with scipy (EN 1998-6). Rows carry `status` (`fail` above 1, `unverifiedBasis` when the verdict cites a registry row; no EN 1998 row is verified, `info` for informative values, `notApplicable`)."""

import math

import numpy as np
from scipy import optimize

G = 9.81
TR = {"nc": 2475.0, "sd": 475.0, "dl": 225.0}
CF = {"kl1": 1.35, "kl2": 1.20, "kl3": 1.00}
GAMMA_I_BRIDGE = {1: 0.8, 2: 1.0, 3: 1.3}
GAMMA_0 = 1.35
SLOSH_C = 0.84
AVG_RATIO = 0.9
ETA_CONVECTIVE = math.sqrt(10.0 / (5.0 + 0.5))
GROUND_VS = (800.0, 360.0, 180.0)
GROUND_SPT = (50.0, 15.0)
GROUND_CU = (250.0, 70.0)
Q_TOWER = {False: 3.5, True: 2.0}


def row(out, key, util, status, value=None):
    out.append({"id": key, "utilization": util, "status": status, **({"value": value} if value is not None else {})})


def ratio(demand, capacity):
    if capacity < 2.2e-16:
        return 0.0 if demand < 2.2e-16 else 1.0e6
    return min(demand / capacity, 1.0e6)


def verdict(util, cites=False):
    """🚦 `fail` above 1 (within 1e-9), `unverifiedBasis` for a passing verdict that cites a registry row, `pass` otherwise."""
    return "fail" if util > 1.0 + 1e-9 else "unverifiedBasis" if cites else "pass"


def vs30_of(layers):
    """🌍 Time-averaged v_s over the top 30 m, the last layer continuing below a shorter profile (EN 1998-1 Eq. 3.1)."""
    if not layers or any(l["thicknessM"] <= 0 or l["shearWaveVelocityMS"] <= 0 for l in layers):
        return 0.0
    depth, travel = 0.0, 0.0
    for l in layers:
        h = min(l["thicknessM"], 30.0 - depth)
        if h <= 0:
            break
        travel += h / l["shearWaveVelocityMS"]
        depth += h
    if depth < 30.0:
        travel += (30.0 - depth) / layers[-1]["shearWaveVelocityMS"]
    return 30.0 / travel


def ground_type(site, vs30):
    """🪨 Table 3.1: S2 liquefiable, S1 soft clay (>= 10 m, PI > 40), E alluvium 5 to 20 m of class C or D over v_s > 800 m/s, else A to D by v_s,30, N_SPT or c_u (a boundary value takes the lower class)."""
    if site["liquefiableLayer"]:
        return "S2"
    if site["softClayThicknessM"] >= 10.0 and site["softClayPi"] > 40.0:
        return "S1"
    alluvium = "C" if 180.0 < site["alluviumVSMS"] <= 360.0 else "D" if 0.0 < site["alluviumVSMS"] <= 180.0 else None
    if 5.0 <= site["alluviumThicknessM"] <= 20.0 and alluvium and site["belowAlluviumVSMS"] > 800.0:
        return "E"
    pick = lambda v, hi, lo: "B" if v > hi else "C" if v > lo else "D"
    if vs30 > 0:
        return "A" if vs30 > GROUND_VS[0] else pick(vs30, GROUND_VS[1], GROUND_VS[2])
    if site["nSpt"] > 0:
        return pick(site["nSpt"], *GROUND_SPT)
    if site["cUKpa"] > 0:
        return pick(site["cUKpa"], *GROUND_CU)
    return None


def site_rows(doc, out):
    site = doc["site"]
    de = doc.get("annex", "de").lower() == "de"
    by_foundation = {}
    for l in doc.get("soilLayers", []):
        by_foundation.setdefault(l["foundationId"], []).append(l)
    profiles = [vs30_of(by_foundation.get(f["id"], [])) for f in doc.get("foundations", [])]
    positive = [v for v in profiles if v > 0]
    vs30 = min(positive) if positive else site["vS30MS"]
    letter = ground_type(site, vs30)
    if letter is not None:
        stated = site["deGroundCombo"].strip().upper()[0] if de else site["enGroundType"].strip().upper()[0]
        if letter in "ABCDE":
            row(out, "en1998.site.groundClass", 0.0 if letter == stated else 1.0, "unverifiedBasis" if letter == stated else "fail")
        else:
            row(out, "en1998.site.groundClass", 0.0, "unverifiedBasis")
    if de and site["aGr"] > 0 and abs(site["aGr"] - site["sApR"] / 2.5) > 1e-9:
        row(out, "en1998.site.aGr.derived", 0.0, "warning")
    if not de:
        expected = {"A": "A-R", "B": "B-R", "C": "C-R", "D": "C-S", "E": "C-S"}.get(site["enGroundType"].strip().upper()[0], "B-R")
        ok = site["deGroundCombo"].strip().upper() == expected
        row(out, "en1998.site.deNaConsistencyUnderEn", 0.0 if ok else 1.0, "pass" if ok else "fail")


def building_rows(doc, main, out):
    for b in doc.get("buildings", []):
        bid = b["id"]
        gamma = main.GAMMA_I[doc["site"]["importanceClass"].strip().upper()]
        row(out, f"en1998.1.{bid}.importance", 0.0, "info", gamma)
        row(out, f"en1998.1.{bid}.regularity.claim", 0.0 if b["planRegular"] and b["elevationRegular"] else 1.0, "pass" if b["planRegular"] and b["elevationRegular"] else "fail")
        lx, ly = b["planWidthM"], b["planLengthM"]
        slender = max(lx, ly) / min(lx, ly) / 4.0 if lx > 0 and ly > 0 else 0.0
        recess = max(b.get("planEnvelopeGapRatio", 0.0), 0.0) / 0.05 if lx > 0 and ly > 0 else 0.0
        setback = max(b.get("setbackRatio", 0.0), 0.0) / 0.20
        plan_ok, elevation_ok = slender <= 1.0 and recess <= 1.0, setback <= 1.0
        consistent = (not b["planRegular"] or plan_ok) and (not b["elevationRegular"] or elevation_ok)
        row(out, f"en1998.1.{bid}.regularity.derived", 0.0 if consistent else 1.0, "pass" if consistent else "fail")


def assessment_rows(doc, main, a_gr, s, tb, tc, td, gamma_i, out):
    buildings = doc.get("buildings", [])
    de = doc.get("annex", "de").lower() == "de"
    for a in doc.get("assessments", []):
        known = any(b["id"] == a["supportedBuildingId"] for b in buildings)
        row(out, f"en1998.3.{a['id']}.supportedBuilding", 0.0 if known else 1.0, "pass" if known else "fail")
        row(out, f"en1998.3.{a['id']}.limitState", 0.0, "pass")
        limit_state, kl = a["limitState"].lower(), a["knowledgeLevel"].lower()
        factor = (475.0 if de else TR[limit_state]) / 475.0
        factor = factor ** (1.0 / 3.0)
        cf = 1.0 if limit_state == "dl" else CF[kl]
        r_d = a["rKN"] / (cf * a["gammaEl"]) if a["gammaEl"] > 0 else 0.0
        target = a["supportedBuildingId"] or (buildings[0]["id"] if buildings else "")
        b = next((x for x in buildings if x["id"] == target), None)
        if b is not None and b["systems"]:
            storeys, n = b["storeys"], len(b["storeys"])
            masses = [main.weight(st, i + 1 == n) / G for i, st in enumerate(storeys)]
            t1, _ = main.fundamental_period(b, masses, sum(st["heightM"] for st in storeys))
            e_d = main.design_base_shear(b, b["systems"][0], masses, t1, gamma_i * a_gr * factor, s, tb, tc, td)[2]
            util = ratio(e_d, max(r_d, 1e-9))
            row(out, f"en1998.3.{a['id']}.capacity", util, verdict(util), e_d)
        method = a.get("analysisMethod", "derived").lower()
        pushover = b is not None and b["storeys"] and all(st["shearResistanceN"] > 0 for st in b["storeys"])
        used = "ns" if (method == "derived" and pushover) or method == "pushover" else "nlth" if method == "nlth" else "lf" if method in ("derived", "lf") else "mrs"
        ok = kl != "kl1" or used in ("lf", "mrs")
        row(out, f"en1998.3.{a['id']}.analysisMethod", 0.0, "pass" if ok else "warning")


def housner(h_over_r):
    """🫙 Housner impulsive and convective mass ratios with D = 2 R: tanh(0.866 D/H)/(0.866 D/H) and 0.230 (D/H) tanh(3.68 H/D)."""
    d_over_h = 2.0 / h_over_r
    x = 0.866 * d_over_h
    return math.tanh(x) / x, 0.230 * d_over_h * math.tanh(3.68 / d_over_h)


def tank_period(height, radius):
    return 2.0 * math.pi / math.sqrt(3.68 * G / (2.0 * radius) * math.tanh(3.68 * height / (2.0 * radius)))


def structure_rows(doc, main, a_gr, s, tb, tc, td, gamma_i, out):
    plateau = lambda a, eta=1.0, t=None: a * s * 2.5 * eta
    for br in doc.get("bridges", []):
        row(out, f"en1998.2.{br['id']}.periodRatio", ratio(1.0, br["periodRatio"]), "pass" if br["periodRatio"] >= 1.0 else "fail")
    for p in doc.get("piers", []):
        if p["heightM"] > 0:
            util = ratio(GAMMA_0 * p["momentRdNm"] / p["heightM"], max(p["shearRdN"], 1e-9))
            bridge = p["bridgeId"]
            row(out, f"en1998.2.{bridge}.pier.{p['id']}.shear", util, verdict(util, True))
    for si in doc.get("silos", []):
        q = min(max(si["qNominal"], 1.0), 1.5)
        psi = main.PSI2.get(si["contentCategory"].lower(), 0.3)
        mass = (si["permanentGkN"] + min(max(si["fillingRatio"], 0.0), 1.0) * psi * si["contentQkN"]) / G
        n_ed = mass * (G + AVG_RATIO * a_gr)
        row(out, f"en1998.4.{si['id']}.wall", ratio(n_ed, max(si["nRdN"], 1e-9)), verdict(ratio(n_ed, max(si["nRdN"], 1e-9))))
        s_d = max(gamma_i * a_gr * s * 2.5 / q, 0.2 * gamma_i * a_gr)
        util = ratio(s_d * mass, max(si["vRdN"], 1e-9))
        row(out, f"en1998.4.{si['id']}.anchor", util, verdict(util))
    for tk in doc.get("tanks", []):
        fluid_h = tk["heightM"] * min(max(tk["fillingRatio"], 0.0), 1.0)
        h_over_r = fluid_h / max(tk["radiusM"], 1e-9)
        t_c = tank_period(fluid_h, tk["radiusM"])
        s_i = gamma_i * a_gr * s * 2.5
        eta_c = ETA_CONVECTIVE
        s_c = gamma_i * (a_gr * s * (1.0 + t_c / tb * (2.5 * eta_c - 1.0)) if t_c <= tb else a_gr * s * 2.5 * eta_c if t_c <= tc else a_gr * s * 2.5 * eta_c * tc / t_c if t_c <= td else a_gr * s * 2.5 * eta_c * tc * td / t_c ** 2)
        m_fluid = min(max(tk["fillingRatio"], 0.0), 1.0) * tk["contentQkN"] / G
        m_wall = tk["permanentGkN"] / G
        mi, mc = housner(h_over_r)
        v = math.hypot((m_fluid * mi + m_wall) * s_i, m_fluid * mc * s_c)
        util = ratio(v, max(tk["vRdN"], 1e-9))
        row(out, f"en1998.4.{tk['id']}.shear", util, verdict(util))
        slosh, freeboard = SLOSH_C * tk["radiusM"] * s_c / G, max(tk["heightM"] - fluid_h, 0.0)
        util = ratio(slosh, max(freeboard, 1e-9))
        row(out, f"en1998.4.{tk['id']}.slosh", util, verdict(util, True), slosh)
    buildings = {b["id"] for b in doc.get("buildings", [])}
    for f in doc.get("foundations", []):
        known = f["supportedBuildingId"] in buildings
        row(out, f"en1998.5.{f['id']}.supportedBuilding", 0.0 if known else 1.0, "pass" if known else "fail")
        r = f["kFoundation"] / f["kSoil"] if f["kSoil"] > 0 else 0.0
        util = ratio(1.5, r)
        row(out, f"en1998.5.{f['id']}.radiationDamping", max(util, 1.0) if r < 1.5 else util, "fail" if r < 1.5 - 1e-12 else "pass")
    for t in doc.get("towers", []):
        if t["eiNm2"] <= 0 or t["heightM"] <= 0 or t["baseRotationalStiffnessNm"] > 0:
            continue
        beta = optimize.brentq(lambda x: 1.0 + math.cos(x) * math.cosh(x), 1.0, 3.0)
        mass = (t["permanentGkN"] + sum(main.phi(v["category"], False, t["correlatedOccupancy"]) * main.PSI2.get(v["category"].lower(), 0.3) * v["qkN"] for v in t["variables"])) / G
        period = 2.0 * math.pi / beta ** 2 * math.sqrt(mass * t["heightM"] ** 3 / t["eiNm2"])
        row(out, f"en1998.6.{t['id']}.period", 0.0, "info", period)
        q = min(max(t["qNominal"], 1.0), Q_TOWER[bool(t["isChimney"])])
        s_d = main.design_spectrum(gamma_i * a_gr, s, tb, tc, td, q, period)
        moment = s_d * mass * 2.0 / 3.0 * t["heightM"]
        util = ratio(moment, max(t["mRdNm"], 1e-9))
        row(out, f"en1998.6.{t['id']}.overturning", util, verdict(util))


MIN_PERIOD_RATIO, MAX_PERIOD, MAX_DAMPING = 3.0, 3.0, 0.30
RU_ALPHA, RU_EXPONENT = 0.7, 0.34
SPREAD_MIN, SPREAD_MAX = 0.1, 6.0
TABLE41_X, TABLE41_VS, TABLE41_DAMPING = (0.10, 0.20, 0.30), (0.90, 0.70, 0.60), (0.03, 0.06, 0.10)
LIQUEFIED_BELOW = 1.25


def spectrum_eta(a_g, s, tb, tc, td, t, eta):
    """📈 EN 1998-1 3.2.2.2 elastic spectrum with the damping correction."""
    if t <= tb:
        return a_g * s * (1.0 + t / tb * (2.5 * eta - 1.0))
    if t <= tc:
        return a_g * s * 2.5 * eta
    if t <= td:
        return a_g * s * 2.5 * eta * tc / t
    return a_g * s * 2.5 * eta * tc * td / t ** 2


def isolation_state(q, kd, dy, mass, spectrum):
    """🛞 Equivalent linear state of a bilinear isolation system: scipy brentq on F(d) = M S_e(T_eff(d), eta(xi(d))), written from the clause formulas."""
    k1 = kd + q / dy
    force = lambda d: k1 * d if d <= dy else q + kd * d

    def state(d):
        keff = force(d) / d
        xi = 4.0 * q * max(d - dy, 0.0) / (2.0 * math.pi * keff * d * d)
        return keff, 2.0 * math.pi * math.sqrt(mass / keff), xi, max(math.sqrt(10.0 / (5.0 + 100.0 * xi)), 0.55)

    def residual(d):
        keff, t, xi, eta = state(d)
        return force(d) - mass * spectrum(t, eta)

    if residual(1e-6) > 0 or residual(10.0) < 0:
        return None
    d = optimize.brentq(residual, 1e-6, 10.0, xtol=1e-14, rtol=1e-14)
    keff, t, xi, eta = state(d)
    return {"d": d, "keff": keff, "t": t, "xi": xi, "eta": eta, "force": force(d)}


def table41(x, values):
    return float(np.interp(x, TABLE41_X, values))


def profile_factors(layers, water, magnitude, a_gs):
    """🌊 Governing F_L of every layer of a profile by the Annex B procedure (CSR without r_d, Rauch CRR, MSF), nine samples per layer."""
    def n_cs(n, fc):
        a, b = (0.0, 1.0) if fc <= 5 else (math.exp(1.76 - 190.0 / fc ** 2), 0.99 + fc ** 1.5 / 1000.0) if fc < 35 else (5.0, 1.2)
        return a + b * n

    def crr75(n):
        return math.inf if n >= 30 else 1.0 / (34.0 - n) + n / 135.0 + 50.0 / (10.0 * n + 45.0) ** 2 - 1.0 / 200.0

    msf = 10 ** 2.24 / magnitude ** 2.56
    bottoms = np.cumsum([l["thicknessM"] for l in layers])

    def stresses(z):
        top, total = 0.0, 0.0
        for l in layers:
            total += l["unitWeightNM3"] * max(0.0, min(l["thicknessM"], z - top))
            top += l["thicknessM"]
        total += layers[-1]["unitWeightNM3"] * max(0.0, z - top)
        return total, total - 9810.0 * max(0.0, z - water)

    def factor(z):
        i = next((k for k, b in enumerate(bottoms) if z <= b + 1e-12), len(layers) - 1)
        l = layers[i]
        resistance = math.inf if l["n160"] <= 0 else crr75(n_cs(l["n160"], l["finesPercent"])) * msf
        total, effective = stresses(z)
        return resistance / (0.65 * a_gs / G * total / effective)

    out = []
    for i, l in enumerate(layers):
        top = 0.0 if i == 0 else float(bottoms[i - 1])
        lo = min(max(top + 1e-6, 0.1), float(bottoms[i]))
        out.append(min((factor(lo + (float(bottoms[i]) - lo) * k / 8.0), lo + (float(bottoms[i]) - lo) * k / 8.0) for k in range(9)))
    return out, [float(b) for b in bottoms]


def pore_ratio(f_l):
    if f_l <= 1.0:
        return 1.0
    return 0.5 + math.asin(max(-1.0, min(1.0, 2.0 * f_l ** (-1.0 / (RU_EXPONENT * RU_ALPHA)) - 1.0))) / math.pi


def hazard_rows(doc, main, a_gr, s, gamma_i, out):
    """🫧 Pore pressure ratio, lateral spreading, pile in liquefied soil and degraded shear modulus of every foundation with a complete soil profile."""
    a_gs = gamma_i * a_gr * s
    for f in doc.get("foundations", []):
        layers = [l for l in doc.get("soilLayers", []) if l["foundationId"] == f["id"]]
        if not layers or f["liqMagnitude"] <= 0 or any(l["unitWeightNM3"] <= 0 or l["thicknessM"] <= 0 for l in layers):
            continue
        factors, bottoms = profile_factors(layers, f["liqWaterDepthM"], f["liqMagnitude"], a_gs)
        liquefied = []
        for i, l in enumerate(layers):
            f_l = factors[i][0]
            if l["n160"] > 0 and math.isfinite(f_l):
                row(out, f"en1998.5.{f['id']}.layer.{l['id']}.poreRatio", 0.0, "info", pore_ratio(f_l))
                if f_l < LIQUEFIED_BELOW:
                    liquefied.append((i, l, f_l))
        if liquefied:
            thickness = sum(l["thicknessM"] for _, l, _ in liquefied)
            ids = {l["id"] for _, l, _ in liquefied}
            slopes = [sl for sl in doc.get("slopes", []) if sl["governingLayerId"] in ids]
            percent = [100.0 * math.tan(math.radians(sl["angle"])) for sl in slopes]
            gentle = any(SPREAD_MIN <= p <= SPREAD_MAX for p in percent)
            steep = any(p > SPREAD_MAX for p in percent)
            row(out, f"en1998.5.{f['id']}.spreading", 0.0, "unverifiedBasis" if gentle or steep else "info", thickness)
        unsupported = min(sum(l["thicknessM"] for _, l, _ in liquefied), f["pileLengthM"])
        if unsupported > 0 and f["pileDiameterM"] > 0 and f["pileLengthM"] > 0 and f["pileModulusPa"] > 0 and f["pileSubgradeModulusNM3"] > 0:
            ei = f["pileModulusPa"] * math.pi * f["pileDiameterM"] ** 4 / 64.0
            beta = (f["pileSubgradeModulusNM3"] * f["pileDiameterM"] / (4.0 * ei)) ** 0.25
            l_eff = 1.0 * (unsupported + 1.0 / beta)
            if f.get("pileAxialLoadN", 0.0) > 0:
                n_cr = math.pi ** 2 * ei / l_eff ** 2
                util = ratio(f["pileAxialLoadN"], n_cr)
                row(out, f"en1998.5.{f['id']}.pile.liquefiedBuckling", util, verdict(util, True))
            if f["pileHeadShearN"] > 0:
                m_ed = f["pileHeadShearN"] * l_eff * (0.5 if f["pileHeadFixed"] >= 0.5 else 1.0)
                util = ratio(m_ed, max(f["pileMomentRdNm"], 1e-9))
                row(out, f"en1998.5.{f['id']}.pile.liquefiedMoment", util, verdict(util, True))
        shear = [l for l in layers if l["unitWeightNM3"] > 0 and l["shearWaveVelocityMS"] > 0]
        if shear:
            ratio_vs = table41(a_gs / G, TABLE41_VS)
            top = shear[0]
            row(out, f"en1998.5.{f['id']}.profile.shearModulus", 0.0, "info", top["unitWeightNM3"] / G * top["shearWaveVelocityMS"] ** 2 * ratio_vs ** 2)


def isolation_rows(doc, main, a_gr, s, tb, tc, td, out):
    """🛞 Isolation systems of bridges (EN 1998-2 chapter 7): displacement, period ratio, period limit, damping and isolation shear; building systems and the pier base period are not modelled here."""
    for system in doc.get("isolationSystems", []):
        bridge = next((b for b in doc.get("bridges", []) if system["bridgeId"] and not system["buildingId"] and b["id"] == system["bridgeId"]), None)
        if bridge is None or min(system["strengthN"], system["stiffnessNPerM"], system["yieldDisplacementM"]) < 0 or system["stiffnessNPerM"] <= 0 or system["yieldDisplacementM"] <= 0:
            continue
        mass = (bridge["permanentGkN"] + sum(main.phi(v["category"], False, bridge["correlatedOccupancy"]) * main.PSI2.get(v["category"].lower(), 0.3) * v["qkN"] for v in bridge["variables"])) / G
        gamma = GAMMA_I_BRIDGE[int(min(max(round(bridge["importanceClass"]), 1), 3))]
        state = isolation_state(system["strengthN"], system["stiffnessNPerM"], system["yieldDisplacementM"], mass, lambda t, eta: spectrum_eta(gamma * a_gr, s, tb, tc, td, t, eta))
        if state is None:
            continue
        key = f"en1998.isolation.{system['id']}"
        util = ratio(state["d"], max(system["capacityM"], 1e-9))
        row(out, f"{key}.displacement", util, verdict(util), state["d"])
        if bridge["fundamentalPeriodS"] > 0:
            util = ratio(MIN_PERIOD_RATIO, state["t"] / bridge["fundamentalPeriodS"])
            row(out, f"{key}.period", util, verdict(util, True), state["t"] / bridge["fundamentalPeriodS"])
        util = ratio(state["t"], MAX_PERIOD)
        row(out, f"{key}.maxPeriod", util, verdict(util, True), state["t"])
        util = ratio(state["xi"], MAX_DAMPING)
        row(out, f"{key}.damping", util, verdict(util, True), state["xi"])
        if bridge["vRdN"] > 0:
            util = ratio(state["force"], bridge["vRdN"])
            row(out, f"{key}.shear", util, verdict(util), state["force"])


def pier_base_rows(doc, a_gr, s, out):
    """🪨 Gazetas springs of a pier base with the shear modulus degraded by EN 1998-5 Table 4.1 at the design acceleration of the bridge class."""
    for base in doc.get("pierBases", []):
        bridge = next((b for b in doc.get("bridges", []) if b["id"] == base["bridgeId"]), None)
        if bridge is None or min(base["radiusM"], base["soilVsMS"], base["soilUnitWeightNM3"]) <= 0 or not 0 <= base["poissonRatio"] < 0.5:
            continue
        gamma = GAMMA_I_BRIDGE[int(min(max(round(bridge["importanceClass"]), 1), 3))]
        ratio_vs = table41(gamma * a_gr * s / G, TABLE41_VS)
        g = base["soilUnitWeightNM3"] / G * base["soilVsMS"] ** 2 * ratio_vs ** 2
        row(out, f"en1998.2.{bridge['id']}.base.{base['id']}.springs", 0.0, "info", 8.0 * g * base["radiusM"] / (2.0 - base["poissonRatio"]))


def rows(doc, main, a_gr, s, tb, tc, td, gamma_i):
    out = []
    site_rows(doc, out)
    building_rows(doc, main, out)
    assessment_rows(doc, main, a_gr, s, tb, tc, td, gamma_i, out)
    structure_rows(doc, main, a_gr, s, tb, tc, td, gamma_i, out)
    hazard_rows(doc, main, a_gr, s, gamma_i, out)
    isolation_rows(doc, main, a_gr, s, tb, tc, td, out)
    pier_base_rows(doc, a_gr, s, out)
    return out


def self_check():
    beta = optimize.brentq(lambda x: 1.0 + math.cos(x) * math.cosh(x), 1.0, 3.0)
    assert abs(beta - 1.875104068711961) < 1e-9
    assert abs(2.0 * math.pi / beta ** 2 - 1.787) < 1e-3
    mi, mc = housner(1.0)
    assert abs(mi - math.tanh(1.732) / 1.732) < 1e-3 and abs(mc - 0.46 * math.tanh(1.84)) < 1e-3
    assert abs(vs30_of([{"thicknessM": 10.0, "shearWaveVelocityMS": 200.0}, {"thicknessM": 20.0, "shearWaveVelocityMS": 400.0}]) - 30.0 / (10 / 200 + 20 / 400)) < 1e-9
    assert ground_type({"liquefiableLayer": False, "softClayThicknessM": 0, "softClayPi": 0, "alluviumThicknessM": 0, "alluviumVSMS": 0, "belowAlluviumVSMS": 0, "nSpt": 0, "cUKpa": 0}, 360.0) == "C"
    state = isolation_state(2.0e5, 6.0e6, 0.01, 3.0e5, lambda t_, eta: spectrum_eta(2.5, 1.2, 0.15, 0.5, 2.0, t_, eta))
    assert abs(state["d"] - 0.07485001860065095) < 1e-9 and abs(state["t"] - 1.1686396311770098) < 1e-9
    assert abs(pore_ratio(1.25) - 0.4304218152411323) < 1e-12 and pore_ratio(0.9) == 1.0
    assert abs(table41(0.15, TABLE41_VS) - 0.8) < 1e-12
    layer = lambda t_, n: {"thicknessM": t_, "unitWeightNM3": 18000.0, "n160": n, "finesPercent": 5.0}
    factors, bottoms = profile_factors([layer(2, 0), layer(3, 6), layer(4, 40), layer(5, 8)], 1.0, 7.0, 3.0)
    assert abs(factors[1][0] - 0.2697040547704221) < 1e-12 and bottoms == [2.0, 5.0, 9.0, 14.0]
    print("EN 1998 families self-check ok")


if __name__ == "__main__":
    self_check()
