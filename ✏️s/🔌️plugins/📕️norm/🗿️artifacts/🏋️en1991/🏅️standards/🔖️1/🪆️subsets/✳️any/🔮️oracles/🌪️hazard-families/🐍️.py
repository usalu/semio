"""🌪️ Independent EN 1991 hazard families, written from the clause text (never from the Rust): bridge traffic LM1 to LM4 with braking and centrifugal force (EN 1991-2 4.2 to 4.5), crane wheel and drive loads (EN 1991-3 2.5, 2.7), silo Janssen pressures solved as an ODE with scipy (EN 1991-4 5.2, 5.3), bridge wind (EN 1991-1-4 8.3), vortex shedding (Annex E), impact (EN 1991-1-7 Annex C), internal gas explosion (Annex D), bridge section temperature (EN 1991-1-5 6.1) and roof snow drift (EN 1991-1-3 5.3.6, 6.2). Every row carries the status the report assigns: `fail` above a utilisation of 1, `warning` above 0.95, `unverifiedBasis` when the verdict rests on a registry row (no EN 1991 row is verified), `notApplicable` for an unclaimed topic. SI throughout; `--self-check` pins the closed forms against brute force and scipy."""

import math
import sys

import numpy as np
from scipy import integrate, optimize

CAP, TOL, NEAR = 1.0e6, 1.0e-9, 0.95
LM1_Q = {1: 300e3, 2: 200e3, 3: 100e3}
LM1_UDL = {1: 9000.0, 2: 2500.0}
AXLE_SPACING = 1.2
BRAKING = (0.6, 0.10, 180e3, 900e3)
CENTRIFUGAL = (200.0, 1500.0, 0.2, 40.0)
CRANE_WHEEL = {"HC1": 50e3, "HC2": 100e3, "HC3": 160e3, "HC4": 250e3}
CRANE_PHI2 = {"HC1": (1.05, 0.17), "HC2": (1.10, 0.34), "HC3": (1.15, 0.51), "HC4": (1.20, 0.68)}
PHI5, MU_DRIVE = 1.5, 0.2
VORTEX_ONSET, NU_AIR, VEHICLE_K = 1.25, 15.0e-6, 300e3
G = 9.81
SV_SPACING, SV_WIDTH = 1.5, 3.0
BRIDGE_CFX0 = (2.4, 1.3, 0.5, 4.0)
BRIDGE_CFZ, BRIDGE_LONG = 0.9, 0.25
PARAPET_OPEN, TRAFFIC_ADD = 0.3, {0: 0.0, 1: 2.0, 2: 4.0}
SHIFT = {1: (-3.0, 16.0), 2: (4.0, 4.0), 3: (8.0, 2.0)}
DTM = {1: (18.0, 13.0), 2: (15.0, 18.0), 3: (10.0, 5.0), 4: (15.0, 8.0), 5: (15.0, 8.0)}
KSUR = {0: (0.7, 0.9), 1: (1.6, 1.1), 2: (1.0, 1.0), 3: (0.7, 1.2), 4: (0.7, 1.2)}
OMEGA_N, OMEGA_M = 0.35, 0.75


def ratio(demand, capacity):
    if not (math.isfinite(demand) and math.isfinite(capacity)):
        return CAP
    if capacity < 2.2e-16:
        return 0.0 if demand < 2.2e-16 else CAP
    return min(demand / capacity, CAP)


class Emit:
    """🧾 Collects rows in the order the report builds them; `pending` models the thread-local registry-read log that the next citing check drains."""

    def __init__(self, rows, status, values):
        self.rows, self.status, self.values, self.pending = rows, status, values, False

    def put(self, key, util, state, value=None):
        self.rows[key], self.status[key] = util, state
        if value is not None:
            self.values[key] = value

    def covers(self, key, required, assumed, cited=False, drain=False, value=None):
        effective = min(required, assumed) if required <= assumed + 1e-9 else required
        util = ratio(effective, max(assumed, abs(effective) * 1e-6, 5e-324))
        state = "fail" if util > 1.0 + TOL else "warning" if util > NEAR else "pass"
        if state != "fail" and (cited or (drain and self.pending)):
            state = "unverifiedBasis"
        if drain or cited:
            self.pending = False
        self.put(key, util, state, required if value is None else value)

    def limit(self, key, computed, limit, state_when_ok="pass", cited=False, drain=False, value=None, warn=False, forced=None):
        util = ratio(computed, limit)
        state = "fail" if util > 1.0 + TOL else "warning" if warn and util > NEAR else state_when_ok
        if forced:
            state = forced
        if state in ("pass", "warning") and (cited or (drain and self.pending)):
            state = "unverifiedBasis"
        if drain or cited:
            self.pending = False
        self.put(key, util, state, computed if value is None else value)

    def info(self, key, value, reads=False):
        self.pending = self.pending or reads
        self.put(key, 0.0, "unverifiedBasis" if self.pending else "pass", value)
        self.pending = False

    def na(self, key):
        self.put(key, 0.0, "notApplicable")

    def invalid(self, collection, row_id, leaf):
        self.put(f"en1991.input.{collection}.{row_id}.{leaf}", 1.0, "fail")


def lane_layout(carriageway, widths):
    if widths:
        total = sum(widths)
        return len(widths), total / len(widths), max(carriageway - total, 0.0)
    w = max(carriageway, 0.0)
    if w < 5.4:
        return 1, 3.0, max(w - 3.0, 0.0)
    if w < 6.0:
        return 2, w / 2.0, 0.0
    n = int(w // 3.0)
    return n, 3.0, w - 3.0 * n


def tandem_moment(axle, span, spacing=AXLE_SPACING):
    """🏗️ Largest mid-span-governing moment of two equal axles on a simple span, found by bounded minimisation of -M(x) (scipy), the closed form P (L - s/2)^2 / (2 L) being the check."""
    if span <= spacing:
        return axle * span / 4.0
    def under_front(x):
        reaction = axle * (2 * span - 2 * x - spacing) / span
        return reaction * x
    best = optimize.minimize_scalar(lambda x: -under_front(x), bounds=(0.0, span - spacing), method="bounded", options={"xatol": 1e-10})
    return max(-best.fun, axle * span / 4.0)


def traffic(doc, out):
    bridge = doc.get("structureKind") == "bridge"
    for lane in doc.get("bridgeLanes", []):
        if not bridge:
            out.na(f"en1991.2.lane.{lane['id']}.none")
    for vehicle in doc.get("specialVehicles", []):
        if not bridge:
            out.na(f"en1991.2.lm3-vehicle.{vehicle['id']}.none")
    if not bridge:
        out.na("en1991.2.bridge.none")
        return
    widths = [lane["width"] for lane in doc.get("bridgeLanes", [])]
    carriageway, span = doc["bridgeLaneWidth"], doc["bridgeSpan"]
    n, w_lane, w_rem = lane_layout(carriageway, widths)
    selected = doc["bridgeLane"]
    ok = 1 <= selected <= n
    lane = max(1, min(selected, n))
    out.limit("en1991.2.notional-lanes", float(selected) if ok else n + 1.0, float(max(n, 1)))
    q_tandem, q_udl = LM1_Q.get(lane, 0.0), LM1_UDL.get(lane, 2500.0)
    out.pending = True
    assumed_t, assumed_u = doc["assumedBridgeTandem"], doc["assumedBridgeUdl"]
    out.covers("en1991.2.lm1-tandem", tandem_moment(q_tandem, span), tandem_moment(assumed_t, span) if assumed_t > 0 else 0.0, value=tandem_moment(q_tandem, span))
    half = AXLE_SPACING / 2.0
    x = doc["bridgeTandemX"]
    eta = lambda s: max(min(s, span - s), 0.0) / 2.0
    on_deck = x - half >= 0 and x + half <= span
    out.limit("en1991.2.lm1-tandem-position", (eta(x - half) + eta(x + half)) if on_deck else span, max(span / 2.0, 1e-6))
    first = next((r["width"] for r in doc.get("bridgeLanes", []) if r["lm1Lane"] == 1), lane_layout(carriageway, [])[1])
    for lane_row in doc.get("bridgeLanes", []):
        lid, number = lane_row["id"], lane_row["lm1Lane"]
        if number == 0:
            out.invalid("bridgeLanes", lid, "lm1Lane")
            continue
        out.covers(f"en1991.2.lane-width.{lid}", lane_row["width"], 3.0)
        reach = abs(lane_row["offset"]) + lane_row["width"] / 2.0
        out.covers(f"en1991.2.lane-inside.{lid}", reach, max(carriageway, 0.0) / 2.0)
        out.pending = True
        out.covers(f"en1991.2.lm1-lane.{lid}", LM1_Q.get(number, 0.0), assumed_t)
        out.pending = True
        out.covers(f"en1991.2.lm1-lane-udl.{lid}", LM1_UDL.get(number, 2500.0), assumed_u)
    special_vehicles(doc, out, span, carriageway)
    low, high = BRAKING[2], BRAKING[3]
    q_lk = min(max(BRAKING[0] * 2.0 * LM1_Q[1] + BRAKING[1] * LM1_UDL[1] * first * span, low), high)
    out.pending = True
    out.limit("en1991.2.braking", q_lk, high, cited=True, value=q_lk)
    radius = doc["bridgeCurveRadius"]
    if radius < 0 or not math.isfinite(radius):
        out.put("en1991.input.bridgeCurveRadius", 1.0, "fail")
    elif radius == 0:
        out.na("en1991.2.centrifugal")
    else:
        q_v = sum(2.0 * LM1_Q[i] for i in range(1, min(n, 3) + 1))
        q_tk = 0.0 if radius > CENTRIFUGAL[1] else CENTRIFUGAL[2] * q_v if radius < CENTRIFUGAL[0] else CENTRIFUGAL[3] * q_v / radius
        out.info("en1991.2.centrifugal", q_tk, reads=True)
    out.covers("en1991.2.lm1-udl", q_udl, assumed_u)
    if w_rem > 0.05:
        out.covers("en1991.2.lm1-remaining", 2500.0, assumed_u)
    out.pending = True
    out.covers("en1991.2.lm2", 400e3, doc["assumedBridgeLm2"])
    out.covers("en1991.2.footway", 5000.0, doc["assumedBridgeFootway"])
    out.covers("en1991.2.lm3", 200e3, doc["assumedBridgeLm3"])
    out.covers("en1991.2.lm4", 5000.0, doc["assumedBridgeLm4"])
    members = {"gr1a": (1, 0, 0, 0, 1), "gr1b": (0, 1, 0, 0, 0), "gr2": (1, 0, 0, 0, 0), "gr3": (0, 0, 0, 0, 1), "gr4": (0, 0, 0, 1, 0), "gr5": (1, 0, 1, 0, 0)}
    selected_group = doc["bridgeLoadGroup"].strip()
    if selected_group not in members:
        out.put("en1991.input.bridgeLoadGroup", 1.0, "fail")
        return
    need = {"t": LM1_Q[lane], "u": q_udl, "lm2": 400e3, "lm3": 200e3, "lm4": 5000.0, "fw": 5000.0}
    have = {"t": assumed_t, "u": assumed_u, "lm2": doc["assumedBridgeLm2"], "lm3": doc["assumedBridgeLm3"], "lm4": doc["assumedBridgeLm4"], "fw": doc["assumedBridgeFootway"]}
    rt = lambda key, scale=1.0: (0.0 if need[key] * scale <= 0 else need[key] * scale / max(have[key], 1e-9))
    demand = {}
    for group, (lm1, lm2, lm3, lm4, fw) in members.items():
        parts = ([rt("t"), rt("u")] if lm1 else []) + ([rt("lm2")] if lm2 else []) + ([rt("lm3")] if lm3 else []) + ([rt("lm4")] if lm4 else []) + ([rt("fw", 3000.0 / 5000.0 if group == "gr1a" else 1.0)] if fw else [])
        demand[group] = max(parts + [0.0])
    top = max(demand.values())
    governors = [g for g, d in demand.items() if abs(d - top) < 1e-6]
    chosen = demand[selected_group]
    covered = chosen <= 1.0 + TOL
    state = "fail" if not covered else "warning" if selected_group not in governors else "pass"
    out.put("en1991.2.group.selected", ratio(chosen, 1.0), state, chosen)


def special_vehicles(doc, out, span, carriageway):
    for v in doc.get("specialVehicles", []):
        try:
            total, axle = (float(x) for x in v["designation"].strip().split("/"))
        except ValueError:
            out.put(f"en1991.input.specialVehicles.{v['id']}.designation", 1.0, "fail")
            continue
        lines = total / axle
        if not (total > 0 and axle in (150.0, 200.0) and 2.0 <= lines <= 30.0 and abs(lines - round(lines)) < 1e-9):
            out.put(f"en1991.input.specialVehicles.{v['id']}.designation", 1.0, "fail")
            continue
        out.pending = True
        out.covers(f"en1991.2.lm3-vehicle.{v['id']}", axle * 1000.0, doc["assumedBridgeLm3"], cited=True, drain=True)
        length = (round(lines) - 1) * SV_SPACING
        out.pending = True
        out.covers(f"en1991.2.lm3-vehicle-length.{v['id']}", length, span, drain=True)
        half = length / 2.0
        target = min(max(v["x"], min(half, span / 2.0)), max(span - half, span / 2.0))
        out.limit(f"en1991.2.lm3-vehicle-position.{v['id']}", abs(v["x"] - target), 0.05, warn=True)
        half_w, road = SV_WIDTH / 2.0, max(carriageway, 0.0)
        y_target = min(max(v["y"], min(half_w, road / 2.0)), max(road - half_w, road / 2.0))
        out.limit(f"en1991.2.lm3-vehicle-lateral.{v['id']}", abs(v["y"] - y_target), 0.05, warn=True)


def crane(doc, out):
    if not doc.get("craneClaimed"):
        out.na("en1991.3.crane.none")
        return
    runway, base, trolley = doc["craneRunwayLength"], doc["craneWheelBase"], doc["craneX"]
    out.limit("en1991.3.crane-position", abs(trolley - runway / 2.0) + base / 2.0, max(runway / 2.0, 1e-6))
    cls, hoist = doc["craneClass"], doc["hoistClass"]
    if cls not in CRANE_WHEEL or hoist not in CRANE_PHI2:
        for key, value in (("craneClass", cls), ("hoistClass", hoist)):
            if (key == "craneClass" and cls not in CRANE_WHEEL) or (key == "hoistClass" and hoist not in CRANE_PHI2):
                out.put(f"en1991.input.{key}", 1.0, "fail")
        return
    phi_min, beta = CRANE_PHI2[hoist]
    wheel = CRANE_WHEEL[cls] * (phi_min + beta * max(doc["hoistingSpeed"], 0.0))
    horizontal = PHI5 * MU_DRIVE * CRANE_WHEEL[cls]
    if runway > 0 and base > 0:
        sections = np.linspace(0.0, runway, 41)
        best = 0.0
        for c in sections:
            for front in (0.0, c, runway, c + base, c - base, base, runway - base, runway + base):
                load = [front, front - base]
                m = sum(wheel * (x * (runway - c) / runway if x <= c else c * (runway - x) / runway) for x in load if 0 <= x <= runway)
                best = max(best, m)
        reaction = max(wheel * ((runway - x) / runway if 0 <= x <= runway else 0.0) + wheel * ((runway - (x - base)) / runway if 0 <= x - base <= runway else 0.0) for x in (0.0, base))
        out.info("en1991.3.crane-runway", reaction)
    out.covers("en1991.3.crane-vertical", wheel, doc["assumedCraneWheel"], cited=True)
    out.covers("en1991.3.crane-horizontal", horizontal, doc["assumedCraneHorizontal"], cited=True)


def janssen_hf(gamma, mu, k, a, z):
    sol = integrate.solve_ivp(lambda s, y: [gamma - mu * k * y[0] / a], [0.0, z], [0.0], rtol=1e-11, atol=1e-9)
    return k * sol.y[0][-1]


def silo(doc, out):
    if not doc.get("siloClaimed"):
        out.na("en1991.4.silo.none")
        return
    gamma, h, a, mu, k = doc["siloBulkDensity"], doc["siloHeight"], doc["siloHydraulicRadius"], doc["siloMu"], doc["siloK"]
    if doc["siloKind"] == "tank":
        out.covers("en1991.4.tank", gamma * h, doc["assumedSiloPressure"])
        return
    if doc["siloKind"] != "silo":
        out.put("en1991.input.siloKind", 1.0, "fail")
        return
    d = doc["siloDiameter"]
    out.limit("en1991.4.silo-hydraulic-radius", a, max(d / 4.0, 1e-6), forced="warning" if abs(a - d / 4.0) > 1e-6 else "pass")
    out.limit("en1991.4.silo-eccentricity", doc["siloEccentricity"], max(d / 2.0, 1e-6), cited=True)
    angle = doc["siloHopperAngleDeg"]
    out.limit("en1991.4.silo-hopper", abs(angle), 90.0, cited=True)
    p_h = janssen_hf(gamma, mu / 1.15, 1.2 * k, a, h)
    out.covers("en1991.4.silo-pressure", p_h, doc["assumedSiloPressure"])
    out.covers("en1991.4.silo-patch", 0.2 * p_h, doc["assumedSiloPatch"])
    p_w = 1.15 * mu * janssen_hf(gamma, 1.15 * mu, k / 1.2, a, h)
    out.covers("en1991.4.silo-wall-friction", p_w, doc["assumedSiloWallFriction"])
    mass_t = math.pi / 4.0 * d * d * h * gamma / G / 1000.0
    out.info("en1991.4.silo-class", h / d, reads=True)
    out.covers("en1991.4.silo-discharge", 1.15 * p_h, doc["assumedSiloPressure"], cited=True)
    out.covers("en1991.4.silo-discharge-friction", 1.1 * p_w, doc["assumedSiloWallFriction"], cited=True)


def qp_at(doc, z, comp):
    zone, terrain = int(doc.get("windZone", 2)), int(doc.get("terrainCategory", 2))
    de = str(doc.get("annex", "de")).lower() == "de"
    vb0 = {1: 22.5, 2: 25.0, 3: 27.5, 4: 30.0}.get(zone) if de else doc.get("enVb", 25.0)
    if vb0 is None or terrain not in range(5) or doc.get("mixedTerrainDistance", 0.0) > 0.0:
        return None, vb0
    co, rho = max(doc.get("orographyFactor", 1.0), 1.0), doc.get("airDensity", 1.25)
    if de and terrain >= 2:
        return comp.na_profile(0.5 * 1.25 * vb0 ** 2, z, bool(doc.get("coastOrIsland"))) * co * co * rho / 1.25, vb0
    return comp.en_profile(z, terrain, vb0, rho, co)[4], vb0


def wind(doc, out, comp):
    for r in doc.get("windBridges", []):
        pk, tk = r["parapetKind"], r["trafficKind"]
        if pk not in (0, 1, 2):
            out.put(f"en1991.input.windBridges.{r['id']}.parapetKind", 1.0, "fail")
            continue
        if tk not in TRAFFIC_ADD:
            out.put(f"en1991.input.windBridges.{r['id']}.trafficKind", 1.0, "fail")
            continue
        bad = next((leaf for leaf in ("deckWidth", "deckDepth", "span", "heightAboveGround") if not r[leaf] > 0), None)
        if bad:
            out.put(f"en1991.input.windBridges.{r['id']}.{bad}", 1.0, "fail")
            continue
        qp, _ = qp_at(doc, r["heightAboveGround"], comp)
        if qp is None:
            out.na(f"en1991.1-4.bridge-wind.{r['id']}")
            continue
        parapet = {0: 0.0, 1: 2.0 * PARAPET_OPEN, 2: 2.0 * max(r["parapetHeight"], 0.0)}[pk]
        d_tot = r["deckDepth"] + max(parapet, TRAFFIC_ADD[tk])
        ratio_bd = r["deckWidth"] / d_tot
        narrow, wide, lo, hi = BRIDGE_CFX0
        cfx = narrow if ratio_bd <= lo else wide if ratio_bd >= hi else float(np.interp(ratio_bd, [lo, hi], [narrow, wide]))
        fx = qp * cfx * d_tot
        out.pending = True
        out.covers(f"en1991.1-4.bridge-wind.{r['id']}", fx, r["assumedForce"], cited=True, drain=True)
        out.pending = True
        out.info(f"en1991.1-4.bridge-wind-vertical.{r['id']}", qp * BRIDGE_CFZ * r["deckWidth"])
        out.pending = True
        out.info(f"en1991.1-4.bridge-wind-longitudinal.{r['id']}", BRIDGE_LONG * fx)
        out.info(f"en1991.1-4.bridge-wind-reaction.{r['id']}", fx * r["span"] / 2.0)
    for r in doc.get("dynamicElements", []):
        if not all(r[leaf] > 0 for leaf in ("height", "width", "logDecrement", "strouhal", "massPerLength")):
            continue
        n1 = r["firstFrequency"] if r["firstFrequency"] > 0 else 46.0 / r["height"]
        terrain = int(doc.get("terrainCategory", 2))
        _, vb = qp_at(doc, 10.0, comp)
        if vb is None or terrain not in range(5):
            continue
        out.info(f"en1991.1-4.dyn-frequency.{r['id']}", n1)
        vm = comp.en_profile(r["height"], terrain, vb, doc.get("airDensity", 1.25), max(doc.get("orographyFactor", 1.0), 1.0))[2]
        vcrit = r["width"] * n1 / r["strouhal"]
        out.limit(f"en1991.1-4.vortex-shedding.{r['id']}", VORTEX_ONSET * vm, vcrit, state_when_ok="pass", cited=True, warn=False, value=vcrit)
        if out.status[f"en1991.1-4.vortex-shedding.{r['id']}"] == "fail":
            out.status[f"en1991.1-4.vortex-shedding.{r['id']}"] = "unverifiedBasis"
        out.info(f"en1991.1-4.vortex-scruton.{r['id']}", 2.0 * r["logDecrement"] * r["massPerLength"] / (doc.get("airDensity", 1.25) * r["width"] ** 2))
        out.info(f"en1991.1-4.vortex-reynolds.{r['id']}", r["width"] * vcrit / NU_AIR, reads=True)


def accidental(doc, out):
    for r in doc.get("impactActions", []):
        if r["kind"] > 2:
            out.put(f"en1991.input.impactActions.{r['id']}.kind", 1.0, "fail")
            continue
        if not r["mass"] > 0:
            out.put(f"en1991.input.impactActions.{r['id']}.mass", 1.0, "fail")
            continue
        if r["kind"] != 2 and not r["speed"] > 0:
            out.put(f"en1991.input.impactActions.{r['id']}.speed", 1.0, "fail")
            continue
        if r["kind"] == 1 and r["stiffness"] <= 0:
            out.na(f"en1991.1-7.impact-force.{r['id']}")
            continue
        k = r["stiffness"] if r["stiffness"] > 0 else VEHICLE_K
        fx = 5.0 * r["mass"] * G if r["kind"] == 2 else r["speed"] * math.sqrt(k * r["mass"])
        out.pending = True
        out.covers(f"en1991.1-7.impact-force.{r['id']}", fx, r["assumedForce"], cited=True, drain=True)
        if r["kind"] != 1:
            out.pending = True
            out.info(f"en1991.1-7.impact-lateral.{r['id']}", 0.5 * fx)
        if r["kind"] != 2:
            out.info(f"en1991.1-7.impact-energy.{r['id']}", 0.5 * r["mass"] * r["speed"] ** 2)
    for r in doc.get("internalExplosions", []):
        if not (r["volume"] > 0 and r["ventArea"] > 0):
            out.put(f"en1991.input.internalExplosions.{r['id']}.{'volume' if not r['volume'] > 0 else 'ventArea'}", 1.0, "fail")
            continue
        av_v, pv = r["ventArea"] / r["volume"], r["ventPressure"] / 1000.0
        pd = max(3.0 + pv, 3.0 + pv / 2.0 + 0.04 / av_v ** 2) * 1000.0
        out.covers(f"en1991.1-7.gas-explosion.{r['id']}", pd, r["assumedPressure"], cited=True, drain=True)
        out.limit(f"en1991.1-7.gas-explosion-domain.{r['id']}", max(5.0 / r["volume"], r["volume"] / 1000.0), 1.0, cited=True, value=r["volume"])
        if out.status[f"en1991.1-7.gas-explosion-domain.{r['id']}"] == "fail":
            out.status[f"en1991.1-7.gas-explosion-domain.{r['id']}"] = "unverifiedBasis"
    if not doc.get("accidentalCases"):
        out.na("en1991.1-7.accidental.none")
    for case in doc.get("accidentalCases", []):
        for i, ex in enumerate(case.get("explosion", [])):
            z = ex["standoff"] / ex["explosionMass"] ** (1.0 / 3.0) if ex["standoff"] > 1e-16 and ex["explosionMass"] > 0 else None
            required = 0.0 if z is None else max(1772.0 / z ** 3 - 114.0 / z ** 2 + 108.0 / z, 0.0) * 1000.0
            out.covers(f"en1991.1-7.explosion.{case['id']}.{i}", required, ex["assumedPressure"])


def thermal(doc, out):
    for r in doc.get("thermalSections", []):
        deck, surf = r["deckType"], r["surfacing"]
        cls = {1: 1, 2: 2, 3: 3, 4: 3, 5: 3}.get(deck)
        if cls is None:
            out.put(f"en1991.input.thermalSections.{r['id']}.deckType", 1.0, "fail")
            continue
        if surf not in KSUR:
            out.put(f"en1991.input.thermalSections.{r['id']}.surfacing", 1.0, "fail")
            continue
        bad = next((leaf for leaf in ("length", "depth", "alphaMicro") if not r[leaf] > 0), None)
        if bad:
            out.put(f"en1991.input.thermalSections.{r['id']}.{bad}", 1.0, "fail")
            continue
        shift_min, shift_max = SHIFT[cls]
        con, exp = doc["t0"] - (doc["tMin"] + shift_min), doc["tMax"] + shift_max - doc["t0"]
        span_k = con + exp
        heat0, cool0 = DTM[deck]
        kh, kc = KSUR[surf] if deck == 1 else (1.0, 1.0)
        heat, cool = kh * heat0, kc * cool0
        out.pending = True
        out.covers(f"en1991.1-5.section-range.{r['id']}", span_k, r["assumedRange"], cited=True, drain=True)
        out.info(f"en1991.1-5.section-gradient.{r['id']}", heat)
        alpha, m = r["alphaMicro"] * 1e-6, max(heat, cool)
        out.info(f"en1991.1-5.section-curvature.{r['id']}", alpha * m / r["depth"])
        out.info(f"en1991.1-5.section-rotation.{r['id']}", alpha * m * r["length"] / (2.0 * r["depth"]))
        out.info(f"en1991.1-5.section-expansion.{r['id']}", alpha * span_k * r["length"])
        n = max(con, exp)
        out.info(f"en1991.1-5.section-combination.{r['id']}", max(n + OMEGA_M * m, OMEGA_N * n + m), reads=True)


def snow(doc, out, comp):
    de = str(doc.get("annex", "de")).lower() == "de"
    if de and doc.get("northGermanLowlandSnow"):
        sk = comp.snow_ground(str(doc.get("snowZone", "2")), doc.get("altitude", 150.0))
        for roof in doc.get("roofs", []):
            alpha = roof.get("pitchDeg", 0.0)
            mu = 0.8 if alpha <= 30 else 0.8 * (60.0 - alpha) / 30.0 if alpha < 60 else 0.0
            out.pending = True
            out.info(f"en1991.1-3.snow-exceptional.{roof['id']}", mu * roof.get("cE", 1.0) * roof.get("cT", 1.0) * 2.3 * sk)
    surfaces = doc.get("roofSurfaces", [])
    sk = comp.snow_ground(str(doc.get("snowZone", "2")), doc.get("altitude", 150.0)) if de else doc.get("enSk", 850.0)
    faces = {f["id"]: f for f in doc.get("windFaces", [])}
    for i, s in enumerate(surfaces):
        face = faces.get(s["face"])
        if face is None or face["zone"].strip().upper() not in "FGHIJKLMN":
            continue
        shape = s["shape"].strip().lower()
        if shape not in ("flat", "monopitch", "duopitch", "hipped"):
            continue
        out.covers(f"en1991.1-4.roof-surface-extent.{s['id']}", s["extent"], doc["depth"])
        alpha = s["pitchDeg"]
        mu1 = lambda a: 0.8 if a <= 30 else 0.8 * (60.0 - a) / 30.0 if a < 60 else 0.0
        if shape in ("flat", "monopitch", "duopitch"):
            out.pending = True
            out.info(f"en1991.1-3.roof-surface-mu.{s['id']}", mu1(alpha), reads=True)
        height = 0.0 if shape == "flat" else max(s["extent"], 0.0) * math.tan(math.radians(min(max(alpha, 0.0), 60.0)))
        later = [o for o in surfaces[i + 1:] if o["face"] == s["face"]]
        cap = 2.0 if de else 4.0
        if later and height > 1e-6:
            nxt = later[0]
            mu_s = 0.5 * mu1(alpha) if alpha > 15.0 else 0.0
            mu_w = min(max(min((s["extent"] + nxt["extent"]) / (2.0 * height), 2000.0 * height / sk), 0.8), cap)
            out.info(f"en1991.1-3.roof-step-drift.{nxt['id']}", max(mu_s + mu_w, mu1(nxt["pitchDeg"])), reads=True)
        roof = next((r for r in doc.get("roofs", []) if s["face"].startswith(r["id"])), (doc.get("roofs") or [None])[0])
        if roof and not later and roof.get("hasParapet") and roof.get("parapetHeight", 0.0) > 0:
            out.info(f"en1991.1-3.roof-parapet-drift.{s['id']}", min(max(2000.0 * roof["parapetHeight"] / sk, 0.8), cap), reads=True)


CONSTRUCTION_QK = {"storage": 200.0, "scaffolding": 1000.0, "formwork": 1000.0, "personnel": 1000.0}


def stage_return_years(days):
    """🗓️ EN 1991-1-6 Table 3.1 (spec E16.01, EN annex only; the DE NA table is modified and not keyed): up to 3 days 2 years, up to 3 months 5, up to 1 year 10, longer 50."""
    return 2.0 if days <= 3.0 else 5.0 if days <= 90.0 else 10.0 if days <= 365.0 else 50.0


def c_prob(p):
    """📆 EN 1991-1-4 Eq. (4.2) with K = 0.2 and n = 0.5."""
    return ((1.0 - 0.2 * math.log(-math.log(1.0 - p))) / (1.0 - 0.2 * math.log(-math.log(0.98)))) ** 0.5


def stages(doc, out):
    de = str(doc.get("annex", "de")).lower() == "de"
    for r in doc.get("constructionStages", []):
        if r["activity"] not in CONSTRUCTION_QK and r["activity"] not in ("machinery", "concreting"):
            out.put(f"en1991.input.constructionStages.{r['id']}.activity", 1.0, "fail")
            continue
        if r["durationDays"] <= 0:
            out.put(f"en1991.input.constructionStages.{r['id']}.durationDays", 1.0, "fail")
            continue
        if r["activity"] in CONSTRUCTION_QK:
            out.covers(f"en1991.1-6.stage-load.{r['id']}", CONSTRUCTION_QK[r["activity"]], r["assumedQk"])
        if de:
            out.put(f"en1991.1-6.stage-return-period.{r['id']}", 0.0, "unverifiedBasis")
        else:
            out.covers(f"en1991.1-6.stage-return-period.{r['id']}", stage_return_years(r["durationDays"]), r["returnYears"], cited=True)
        out.pending = True
        out.info(f"en1991.1-6.stage-wind.{r['id']}", c_prob(1.0 / max(r["returnYears"], 1.01)) ** 2)


def walls(doc, out, comp):
    h_over_d = doc.get("height", 0.0) / max(doc.get("depth", 0.1), 0.1)
    for face in doc.get("windFaces", []):
        zone = face["zone"].strip().upper()
        if zone not in ("A", "B", "C", "D", "E"):
            continue
        c10, c1 = comp.wall_cpe(zone, h_over_d)
        for suffix, user, table in (("cpe10", face["cPe10"], c10), ("cpe1", face["cPe1"], c1)):
            out.pending = True
            out.limit(f"en1991.1-4.{suffix}.{face['id']}", abs(user - table), 0.05, warn=True, drain=True)


def isotherm(doc, out):
    lapse = 0.5 * max(doc.get("altitude", 0.0), 0.0) / 100.0
    margin = min(doc["tMax"] - (37.0 - lapse), (-24.0 - lapse) - doc["tMin"])
    out.pending = True
    out.info("en1991.1-5.isotherm", abs(margin))


def hazards(doc, rows, status, values, comp):
    out = Emit(rows, status, values)
    traffic(doc, out)
    crane(doc, out)
    silo(doc, out)
    wind(doc, out, comp)
    accidental(doc, out)
    thermal(doc, out)
    snow(doc, out, comp)
    stages(doc, out)
    walls(doc, out, comp)
    isotherm(doc, out)


def self_check():
    from math import isclose
    for span in (2.0, 5.0, 12.0, 30.0, 60.0):
        closed = max((span - AXLE_SPACING / 2.0) ** 2 / (2.0 * span), span / 4.0) if span > AXLE_SPACING else span / 4.0
        brute = max(max(f * (2 * span - 2 * f - AXLE_SPACING) / span for f in np.linspace(0.0, span - AXLE_SPACING, 40001)) if span > AXLE_SPACING else 0.0, span / 4.0)
        assert isclose(brute, closed, rel_tol=1e-6) or span <= AXLE_SPACING, (span, brute, closed)
        assert isclose(tandem_moment(1.0, span), closed, rel_tol=1e-6), span
    a, gamma, mu, k, z = 1.5, 8000.0, 0.4, 0.5, 12.0
    z_o = a / (k * mu)
    assert isclose(janssen_hf(gamma, mu, k, a, z), gamma * a / mu * (1 - math.exp(-z / z_o)), rel_tol=1e-8)
    assert lane_layout(7.0, [])[:2] == (2, 3.0) and lane_layout(5.7, [])[:2] == (2, 2.85) and lane_layout(5.0, []) == (1, 3.0, 2.0)
    assert isclose(15.0 * math.sqrt(300e3 * 1500.0) / 1000.0, 318.198, rel_tol=1e-4)
    print("hazard families self-check ok")


if __name__ == "__main__":
    self_check()
