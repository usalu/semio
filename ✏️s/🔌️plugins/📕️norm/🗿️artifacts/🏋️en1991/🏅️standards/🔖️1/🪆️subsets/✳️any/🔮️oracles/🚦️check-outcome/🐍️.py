"""🚦️ Independent EN 1991 document oracle: a committed snapshot (JSON on stdin or a path) to the utilisation (required over assumed) of every check family of the spec-trace `check` rows (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: third-party agreement). It wraps the required values of `../⚖️compliance/🐍️.py` (imposed, self-weight, snow, wind faces; scipy.stats, scipy.integrate) and adds floor force, concentrated load Q_k (Table 6.2 / NA Table 6.1DE), partition allowance (6.3.1.2(8)), the nominal fire gas temperature (ISO 834, external and hydrocarbon curves), the net heat flux (Eq. 3.2 with scipy.constants sigma), the design fire load q_f,d (Annex E), thermal ΔT (EN 1991-1-5 6.1.3 to 6.1.5), construction load (1-6 Table 4.1) and vehicle impact F = v sqrt(k m) (1-7 Annex C). Written from the clauses, not from the Rust. SI throughout."""

import importlib.util
import json
import math
import pathlib
import sys

import numpy as np

HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("en1991_compliance", HERE.parent / "⚖️compliance" / "🐍️.py")
comp = importlib.util.module_from_spec(spec)
spec.loader.exec_module(comp)
spec_h = importlib.util.spec_from_file_location("en1991_hazard_families", HERE.parent / "🌪️hazard-families" / "🐍️.py")
families = importlib.util.module_from_spec(spec_h)
spec_h.loader.exec_module(families)

SIGMA = 5.670374419e-8
CONCENTRATED_DE = {"A1": 1.0, "A2": 1.0, "A3": 1.0, "B1": 1.0, "B2": 1.5, "B3": 4.0, "C1": 4.0, "C2": 4.0, "C3": 4.0, "C4": 4.0, "C5": 4.0, "C6": 10.0, "D1": 4.0, "D2": 4.0, "D3": 4.0, "E1": 7.0, "T1": 2.0, "T2": 3.0, "Z": 2.0, "F": 20.0, "G": 90.0, "H": 1.0}
CONCENTRATED_EN = {"A": 2.0, "B": 4.5, "C1": 4.0, "C2": 4.0, "C3": 4.0, "C4": 4.0, "C5": 4.5, "D1": 4.0, "D2": 7.0, "E1": 7.0, "F": 20.0, "G": 90.0, "H": 1.0}
PARTITION = {"de": 0.8, "en": 0.5}
CONSTRUCTION = {"storage": 0.2, "scaffolding": 1.0, "formwork": 1.0, "personnel": 1.0}
BRIDGE_SHIFT = {1: (-3.0, 16.0), 2: (4.0, 4.0), 3: (8.0, 2.0)}
DELTA_Q1 = [(25.0, 1.10), (250.0, 1.50), (2500.0, 1.90), (5000.0, 2.00), (10000.0, 2.13)]
OMEGA_N, OMEGA_M = 0.35, 0.75
VEHICLE_STIFFNESS = 300e3
COMBUSTION = 0.8
VB0_DE = {1: 22.5, 2: 25.0, 3: 27.5, 4: 30.0}
CSCD_SIMPLE_HEIGHT = 15.0
OPENING_TOLERANCE = 0.005
T_LIM_H = 20.0 / 60.0
FRACTILE_MJ = {"dwelling": 948.0, "residential": 948.0, "office": 511.0, "hospital": 280.0, "school": 347.0, "hotel": 377.0, "library": 1824.0, "shopping": 730.0, "mall": 730.0, "theatre": 365.0, "transport": 122.0}


def characteristic_fire_load(doc):
    """🧮️ q_f,k of exactly the chosen basis: the Table E.4 80 % fractile of the occupancy row, or the user-specified value."""
    if doc["fireLoadBasis"] == "occupancyTable":
        return FRACTILE_MJ[doc["fireOccupancy"].strip().lower()] * 1e6
    assert doc["fireLoadBasis"] == "userSpecified" and doc["fireLoadDensityQf"] > 0
    return doc["fireLoadDensityQf"]


def fire_curve_c(curve, minutes):
    if curve == "standard":
        return 20.0 + 345.0 * math.log10(8.0 * minutes + 1.0)
    if curve == "external":
        return 20.0 + 660.0 * (1.0 - 0.687 * math.exp(-0.32 * minutes) - 0.313 * math.exp(-3.8 * minutes))
    if curve == "hydrocarbon":
        return 20.0 + 1080.0 * (1.0 - 0.325 * math.exp(-0.167 * minutes) - 0.675 * math.exp(-2.5 * minutes))
    return None


def h_net(theta_g, alpha_c):
    theta_m = 293.15
    return alpha_c * (theta_g - theta_m) + 1.0 * 0.8 * 1.0 * SIGMA * (theta_g ** 4 - theta_m ** 4)


def delta_q1(area):
    xs = [math.log(a) for a, _ in DELTA_Q1]
    return float(np.interp(math.log(max(area, 1.0)), xs, [d for _, d in DELTA_Q1]))


def heating_c(t_star):
    return 20.0 + 1325.0 * (1.0 - 0.324 * math.exp(-0.2 * t_star) - 0.204 * math.exp(-1.7 * t_star) - 0.472 * math.exp(-19.0 * t_star))


def compartment(af, height):
    """🚪 Square compartment with 20 % wall openings (the production model): A_t, A_v, h_eq and O."""
    af, height = max(af, 1.0), max(height, 0.5)
    wall = 4.0 * math.sqrt(af) * height
    at, av = 2.0 * af + wall, 0.2 * wall
    return at, av, height, av * math.sqrt(height) / at


def parametric_gas_c(t_h, af, at, av, heq, b, qfd_mj):
    """🔥 EN 1991-1-2 Annex A: theta_g(t) and theta_max (degC) with the heating curve (A.1), t_max (A.9), Gamma_lim (A.10) and the three cooling branches (A.11)."""
    o = av * math.sqrt(heq) / at
    gamma = lambda oo: ((oo / b) / (0.04 / 1160.0)) ** 2
    q_td = qfd_mj * af / at
    t_vent = 0.2e-3 * q_td / o
    fuel = t_vent < T_LIM_H
    g_heat, t_max = gamma(o), t_vent
    if fuel:
        g_heat = gamma(0.1e-3 * q_td / T_LIM_H)
        if o > 0.04 and q_td < 75.0 and b < 1160.0:
            g_heat *= 1.0 + ((o - 0.04) / 0.04) * ((q_td - 75.0) / 75.0) * ((1160.0 - b) / 1160.0)
        t_max = T_LIM_H
    theta_max = heating_c(t_max * g_heat)
    if t_h <= t_max:
        return heating_c(t_h * g_heat), theta_max
    g_vent = gamma(o)
    t_star_max = t_vent * g_vent
    x = T_LIM_H * g_vent / t_star_max if fuel else 1.0
    slope = 625.0 if t_star_max <= 0.5 else 250.0 * (3.0 - t_star_max) if t_star_max < 2.0 else 250.0
    return max(theta_max - slope * (t_h * g_vent - x * t_star_max), 20.0), theta_max


def vb_of(doc):
    return VB0_DE[int(doc.get("windZone", 2))] if str(doc.get("annex", "de")).lower() == "de" else doc["enVb"]


def covered(required, assumed):
    if required <= 0:
        return 0.0
    return required / max(assumed, 1e-9) if assumed > 0 else None


def evaluate(doc, status=None, values=None):
    """📋️ Every check id this oracle reproduces with its utilisation; `status` collects the non-numeric statuses and `values` the informative quantities of the row collections."""
    rows = {}
    status = {} if status is None else status
    values = {} if values is None else values
    for row in comp.snapshot_checks(doc):
        if row["assumed"] > 0:
            rows[row["id"]] = row["required"] / row["assumed"]
    de = str(doc.get("annex", "de")).lower() == "de"
    for floor in doc.get("floors", []):
        key = f"en1991.1-1.imposed.{floor['id']}"
        if key in rows:
            rows[f"en1991.1-1.floor-force.{floor['id']}"] = rows[key]
        table = CONCENTRATED_DE if de else CONCENTRATED_EN
        if floor["category"] in table and floor["assumedQkConcentrated"] > 0:
            rows[f"en1991.1-1.concentrated.{floor['id']}"] = 1000.0 * table[floor["category"]] / floor["assumedQkConcentrated"]
        if floor["assumedPartitions"] > 0:
            rows[f"en1991.1-1.partitions.{floor['id']}"] = 1000.0 * PARTITION["de" if de else "en"] / floor["assumedPartitions"]
    mode = doc.get("fireMode", "none")
    if mode == "none":
        rows["en1991.1-2.fire.none"], status["en1991.1-2.fire.none"] = 0.0, "notApplicable"
    if mode == "nominal":
        theta = fire_curve_c(doc["fireCurve"], doc["fireDuration"] / 60.0)
        if theta is not None and doc["assumedGasTemperature"] > 0:
            theta_k = theta + 273.15
            rows["en1991.1-2.gas-temperature"] = theta_k / doc["assumedGasTemperature"]
            alpha_c = 50.0 if doc["fireCurve"] == "hydrocarbon" else 25.0
            if doc["assumedHNet"] > 0:
                rows["en1991.1-2.h-net"] = h_net(theta_k, alpha_c) / doc["assumedHNet"]
        qfk = characteristic_fire_load(doc)
        qfd = qfk * COMBUSTION * delta_q1(doc["fireCompartmentArea"])
        if doc["assumedQfD"] > 0:
            rows["en1991.1-2.fire-load"] = qfd / doc["assumedQfD"]
    if mode == "parametric":
        qfk = characteristic_fire_load(doc)
        qfd = qfk * COMBUSTION * delta_q1(doc["fireCompartmentArea"])
        at, av, heq, o_geom = compartment(doc["fireCompartmentArea"], doc["fireCompartmentHeight"])
        o_in = doc["fireOpeningFactor"]
        av_eff = o_in * at / math.sqrt(heq) if o_in > 1e-9 else av
        theta, theta_max = parametric_gas_c(doc["fireDuration"] / 3600.0, max(doc["fireCompartmentArea"], 1.0), at, av_eff, heq, doc["fireThermalInertia"], qfd / 1e6)
        required_k = max(theta, theta_max) + 273.15
        rows["en1991.1-2.opening-factor"] = abs(o_in - o_geom) / OPENING_TOLERANCE
        o_eff = o_in if o_in > 1e-9 else av / at * math.sqrt(heq)
        q_td = qfd / 1e6 * max(doc["fireCompartmentArea"], 1.0) / at
        rows["en1991.1-2.validity"] = max(q_td / 1000.0, 50.0 / q_td, o_eff / 0.20, 0.02 / o_eff)
        status["en1991.1-2.validity"] = "warning" if rows["en1991.1-2.validity"] > 1.0 else "pass"
        if doc["assumedGasTemperature"] > 0:
            rows["en1991.1-2.gas-temperature"] = required_k / doc["assumedGasTemperature"]
        if doc["assumedHNet"] > 0:
            rows["en1991.1-2.h-net"] = h_net(required_k, 35.0) / doc["assumedHNet"]
        if doc["assumedQfD"] > 0:
            rows["en1991.1-2.fire-load"] = qfd / doc["assumedQfD"]
    for element in doc.get("dynamicElements", []):
        h = element["height"]
        n1 = element["firstFrequency"] if element["firstFrequency"] > 0 else 46.0 / h
        required = 1.0 if h < CSCD_SIMPLE_HEIGHT else comp.structural_factor(h, element["width"], doc["terrainCategory"], vb_of(doc), element["logDecrement"], n1)["cscd"]
        if element["assumedCscd"] > 0:
            rows[f"en1991.1-4.dyn-cscd.{element['id']}"] = required / element["assumedCscd"]
    if doc.get("thermalElementType") in ("building", "bridge") and doc.get("assumedDeltaT", 0) > 0:
        t_max, t_min, t0 = doc["tMax"], doc["tMin"], doc["t0"]
        if doc["thermalElementType"] == "bridge":
            if doc["thermalBridgeType"] in BRIDGE_SHIFT:
                dn, dx = BRIDGE_SHIFT[doc["thermalBridgeType"]]
                n = max(t0 - (t_min + dn), t_max + dx - t0)
            else:
                n = None
        else:
            n = max(t0 - t_min, t_max - t0)
        if n is not None:
            m = abs(doc["deltaTM"])
            rows["en1991.1-5.thermal"] = max(n + OMEGA_M * m, OMEGA_N * n + m) / doc["assumedDeltaT"]
    if doc.get("constructionActivity") in CONSTRUCTION and doc.get("assumedConstructionQk", 0) > 0:
        rows["en1991.1-6.construction"] = 1000.0 * CONSTRUCTION[doc["constructionActivity"]] / doc["assumedConstructionQk"]
    for case in doc.get("accidentalCases", []):
        for i, impact in enumerate(case.get("impact", [])):
            if impact["assumedForce"] > 0:
                rows[f"en1991.1-7.impact.{case['id']}.{i}"] = impact["vehicleSpeed"] * math.sqrt(VEHICLE_STIFFNESS * impact["vehicleMass"]) / impact["assumedForce"]
    hazard_rows(doc, rows)
    collection_rows(doc, rows, status, values)
    wave3_rows(doc, rows, status, values)
    rail_silo_rows(doc, rows, status, values)
    families.hazards(doc, rows, status, values, comp)
    return rows

FOOTBRIDGE_Q = (2000.0, 120000.0, 30.0, 2500.0, 5000.0)
FOOTBRIDGE_H = (0.10, 10000.0)
FOOTBRIDGE_WALK = (1.7, 2.1)
FOOTBRIDGE_COMFORT_V = 5.0
RAIL_PHI = {"careful": (1.44, 0.82, 1.67), "standard": (2.16, 0.73, 2.0)}
BETA_3 = {"grab": 0.5, "magnet": 1.0}
TEST_LOAD = {"static": 1.1, "dynamic": 1.25}
K_MEAN = 1.1


def footbridge_q(length):
    """🚶 EN 1991-2 5.3.2.1: q_fk = 2.0 + 120 / (L + 30) kN/m2 limited to 2.5 .. 5.0 kN/m2 (Pa here)."""
    base, slope, offset, low, high = FOOTBRIDGE_Q
    return float(np.clip(base + slope / (length + offset), low, high))


def rail_phi(maintenance, length):
    """🚆 EN 1991-2 6.4.5.2: Phi_2 = 1.44 / (sqrt(L) - 0.2) + 0.82 in 1.00 .. 1.67, Phi_3 = 2.16 / (sqrt(L) - 0.2) + 0.73 in 1.00 .. 2.00."""
    if maintenance not in RAIL_PHI:
        return None
    a, b, top = RAIL_PHI[maintenance]
    return float(np.clip(a / max(math.sqrt(length) - 0.2, 0.01) + b, 1.0, top))


def collection_rows(doc, rows, status, values):
    """🧾 The row collections (craneHoists, footbridges, railDynamics, loadDispersals, bulkSolids, rotatingMachines) the way the report emits them: informative values read from the registry rows are `unverifiedBasis` at utilisation 0, a registry read is drained by the next informative row (so a second row of the same computation passes), an inadmissible input is a `fail` at utilisation 1 and ends its row."""
    pending = []

    def info(key, value, reads=()):
        pending.extend(reads)
        rows[key] = 0.0
        status[key] = "unverifiedBasis" if pending else "pass"
        values[key] = value
        pending.clear()

    def invalid(collection, row_id, leaf):
        rows[f"en1991.input.{collection}.{row_id}.{leaf}"] = 1.0
        status[f"en1991.input.{collection}.{row_id}.{leaf}"] = "fail"

    def positive(x):
        return math.isfinite(x) and x > 0

    def not_negative(x):
        return math.isfinite(x) and x >= 0

    for r in doc.get("craneHoists", []):
        beta = BETA_3.get(r["releaseDevice"])
        if beta is not None:
            pending.append("beta-3")
        else:
            invalid("craneHoists", r["id"], "releaseDevice")
        if not (math.isfinite(r["releaseFraction"]) and 0.0 <= r["releaseFraction"] <= 1.0):
            invalid("craneHoists", r["id"], "releaseFraction")
        elif beta is not None:
            info(f"en1991.3.hoist-release.{r['id']}", 1.0 - r["releaseFraction"] * (1.0 + beta))
        if not positive(r["ratedLoad"]):
            invalid("craneHoists", r["id"], "ratedLoad")
            continue
        pending.extend(["test-static", "test-dynamic"])
        for name, factor in TEST_LOAD.items():
            info(f"en1991.3.hoist-test-{name}.{r['id']}", factor * r["ratedLoad"])
    for r in doc.get("footbridges", []):
        bad = next((leaf for leaf in ("loadedLength", "deckWidth") if not positive(r[leaf])), None)
        if bad:
            invalid("footbridges", r["id"], bad)
            continue
        length, width = r["loadedLength"], r["deckWidth"]
        q = footbridge_q(length)
        pending.append("registry")
        info(f"en1991.2.footbridge-load.{r['id']}", q)
        info(f"en1991.2.footbridge-horizontal.{r['id']}", max(FOOTBRIDGE_H[0] * q * length * width, FOOTBRIDGE_H[1]))
        if positive(r["naturalFrequency"]):
            low, high = FOOTBRIDGE_WALK
            info(f"en1991.2.footbridge-walking.{r['id']}", r["naturalFrequency"], ("walk",))
            required = r["naturalFrequency"] < FOOTBRIDGE_COMFORT_V
            values[f"en1991.2.footbridge-walking.{r['id']}.comfort-required"] = required
            values[f"en1991.2.footbridge-walking.{r['id']}.resonant"] = required and low <= r["naturalFrequency"] <= high
    for r in doc.get("railDynamics", []):
        valid = positive(r["determinantLength"])
        if not valid:
            invalid("railDynamics", r["id"], "determinantLength")
        phi = rail_phi(r["maintenance"], r["determinantLength"] if valid else 1.0)
        if phi is None:
            invalid("railDynamics", r["id"], "maintenance")
        elif valid:
            info(f"en1991.2.rail-dynamic.{r['id']}", phi)
    for r in doc.get("loadDispersals", []):
        fields = (("wheelLength", positive(r["wheelLength"])), ("wheelWidth", positive(r["wheelWidth"])), ("pavementThickness", not_negative(r["pavementThickness"])), ("slabThickness", not_negative(r["slabThickness"])))
        bad = next((leaf for leaf, ok in fields if not ok), None)
        if bad:
            invalid("loadDispersals", r["id"], bad)
            continue
        spread = 2.0 * (r["pavementThickness"] + 0.5 * r["slabThickness"])
        info(f"en1991.2.dispersal-footprint.{r['id']}", (r["wheelLength"] + spread) * (r["wheelWidth"] + spread), ("ratio",))
    for r in doc.get("bulkSolids", []):
        phi = r["internalFriction"]
        if not (math.isfinite(phi) and 0.0 < phi < math.pi / 2):
            invalid("bulkSolids", r["id"], "internalFriction")
            continue
        info(f"en1991.4.bulk-k-mean.{r['id']}", K_MEAN * (1.0 - math.sin(phi)), ("k-mean",))
    for r in doc.get("rotatingMachines", []):
        bad = next((leaf for leaf, ok in (("mass", positive(r["mass"])), ("eccentricity", not_negative(r["eccentricity"])), ("frequency", positive(r["frequency"]))) if not ok), None)
        if bad:
            invalid("rotatingMachines", r["id"], bad)
            continue
        info(f"en1991.3.machine-force.{r['id']}", r["mass"] * r["eccentricity"] * (2.0 * math.pi * r["frequency"]) ** 2)


FLM1_FACTORS = (0.7, 0.3)
LM1_AXLE_N = {1: 300e3, 2: 200e3, 3: 100e3}
LM1_UDL_PA = {1: 9000.0, 2: 2500.0, 3: 2500.0}
FLM3_AXLE_N = 120e3
FLM3_SPACINGS_M = (1.2, 6.0, 1.2)
COLLISION_N = {"deck": (1000e3, 500e3), "soffit": (500e3, None)}
HASEMI_PLATEAU, HASEMI_INTERCEPT, HASEMI_SLOPE, HASEMI_TAIL, HASEMI_EXPONENT = 100e3, 136.3e3, 121e3, 15e3, -3.7


def hasemi_flux(diameter, power, ceiling, distance):
    """🔥 EN 1991-1-2 Annex C: Hasemi heat flux under a ceiling the flame reaches, from the virtual source z' and the horizontal flame length L_h."""
    q_h = power / (1.11e6 * ceiling ** 2.5)
    q_d = power / (1.11e6 * diameter ** 2.5)
    l_h = ceiling * (2.9 * q_h ** 0.33 - 1.0)
    z_prime = 2.4 * diameter * (q_d ** 0.4 - q_d ** (2.0 / 3.0)) if q_d < 1.0 else 2.4 * diameter * (1.0 - q_d ** 0.4)
    y = (distance + ceiling + z_prime) / (l_h + ceiling + z_prime)
    return HASEMI_PLATEAU if y <= 0.30 else HASEMI_INTERCEPT - HASEMI_SLOPE * y if y < 1.0 else HASEMI_TAIL * y ** HASEMI_EXPONENT


def wave3_rows(doc, rows, status, values):
    """🧷 Fatigue models, support collisions, localised fires and snow guards the way the report emits them: a covering check cites the unverified registry values (status `unverifiedBasis` when it covers, `fail` above a utilisation of 1), the snow guard, the flame length and the plume temperature read no registry value (`pass`)."""
    bridge = doc.get("structureKind") == "bridge"

    def covers(key, required, assumed, cited=True):
        rows[key] = required / assumed
        values[key] = required
        status[key] = "fail" if rows[key] > 1.0 + 1e-9 else "unverifiedBasis" if cited else "pass"

    def invalid(collection, row_id, leaf):
        rows[f"en1991.input.{collection}.{row_id}.{leaf}"] = 1.0
        status[f"en1991.input.{collection}.{row_id}.{leaf}"] = "fail"

    for r in doc.get("fatigueModels", []):
        if not bridge:
            continue
        if r["lane"] not in LM1_AXLE_N:
            invalid("fatigueModels", r["id"], "lane")
            continue
        if r["model"] == "flm1":
            covers(f"en1991.2.fatigue-axle.{r['id']}", FLM1_FACTORS[0] * LM1_AXLE_N[r["lane"]], r["assumedAxleLoad"])
            covers(f"en1991.2.fatigue-udl.{r['id']}", FLM1_FACTORS[1] * LM1_UDL_PA[r["lane"]], r["assumedUdl"])
        elif r["model"] == "flm3":
            covers(f"en1991.2.fatigue-axle.{r['id']}", FLM3_AXLE_N, r["assumedAxleLoad"])
            key = f"en1991.2.fatigue-vehicle.{r['id']}"
            rows[key], status[key], values[key] = 0.0, "unverifiedBasis", sum(FLM3_SPACINGS_M)
        else:
            invalid("fatigueModels", r["id"], "model")
    for r in doc.get("supportCollisions", []):
        if not bridge:
            continue
        if not r["height"] > 0:
            invalid("supportCollisions", r["id"], "height")
        elif not 0.0 <= r["positionX"] <= doc["bridgeSpan"]:
            invalid("supportCollisions", r["id"], "positionX")
        elif r["location"] not in COLLISION_N:
            invalid("supportCollisions", r["id"], "location")
        else:
            force_x, force_y = COLLISION_N[r["location"]]
            covers(f"en1991.2.collision-x.{r['id']}", force_x, r["assumedForceX"])
            if force_y is not None:
                covers(f"en1991.2.collision-y.{r['id']}", force_y, r["assumedForceY"])
            low, high = 0.5, 1.5
            key = f"en1991.2.collision-height.{r['id']}"
            rows[key], status[key], values[key] = 0.0, "unverifiedBasis", r["height"]
            values[key + ".inside"] = low <= r["height"] <= high
    for r in doc.get("localFires", []):
        if not (r["fireDiameter"] > 0 and r["heatReleaseRate"] > 0):
            invalid("localFires", r["id"], "fireDiameter" if not r["fireDiameter"] > 0 else "heatReleaseRate")
            continue
        diameter, power, ceiling = r["fireDiameter"], r["heatReleaseRate"], r["ceilingHeight"]
        flame = -1.02 * diameter + 0.0148 * power ** 0.4
        origin = -1.02 * diameter + 0.00524 * power ** 0.4
        key = f"en1991.1-2.local-flame.{r['id']}"
        rows[key], status[key], values[key] = 0.0, "pass", max(flame, 0.0)
        if ceiling <= 0:
            continue
        if flame >= ceiling:
            covers(f"en1991.1-2.local-flux.{r['id']}", hasemi_flux(diameter, power, ceiling, r["horizontalDistance"]), r["assumedFlux"])
        elif ceiling > origin:
            key = f"en1991.1-2.local-plume.{r['id']}"
            theta = min(20.0 + 0.25 * (0.8 * power) ** (2.0 / 3.0) * (ceiling - origin) ** (-5.0 / 3.0), 900.0)
            rows[key], status[key], values[key] = 0.0, "pass", theta + 273.15
    sk = {row["id"]: row for row in comp.snapshot_checks(doc)}
    for r in doc.get("snowGuards", []):
        roof = next((x for x in doc.get("roofs", []) if x["id"] == r["roofId"]), None)
        if roof is None:
            invalid("snowGuards", r["id"], "roofId")
        elif r["slopeLength"] > 0 and f"en1991.1-3.snow.{roof['id']}" in sk:
            snow = sk[f"en1991.1-3.snow.{roof['id']}"]["required"]
            covers(f"en1991.1-3.snow-guard.{r['id']}", snow * r["slopeLength"] * math.sin(math.radians(roof["pitchDeg"])), r["assumedForce"], cited=False)


DERAIL_I_N_M = 45000.0
DERAIL_II_FACTOR = 1.4
DERAIL_LENGTH_M = 20.0
DERAIL_OFFSET_LIMIT_M = 1.5 * 1.435
LM71_AXLE = 250000.0
LM71_UDL = 80000.0
RAIL_GROUPS = ["gr11", "gr12", "gr13", "gr14", "gr15", "gr16", "gr17", "gr21", "gr22", "gr23", "gr24", "gr26", "gr27", "gr31"]
TRACTION = (33000.0, 1000000.0)
BRAKING = (20000.0, 6000000.0)
NOSING = 100000.0
SERVICE_AXLE = 40000.0
DUST_VENT = (3.264e-5, -0.569, 0.27, -0.5, 0.753, 0.1)
DUST_DOMAIN = (("kst", "kst", 10.0, 300.0), ("pMax", "pMax", 500000.0, 1000000.0), ("volume", "volume", 0.1, 10000.0), ("staticOpeningPressure", "pStat", 10000.0, 100000.0), ("designPressure", "pRed", 10000.0, 200000.0))


def dust_vent_area(kst, p_max, volume, p_stat, p_red):
    """💥 Bartknecht vent area A = [c1 p_max K_St p_red^e1 + c2 (p_stat - 0.1) p_red^e2] V^e3 in m2 with the pressures in bar."""
    c1, e1, c2, e2, e3, offset = DUST_VENT
    pm, ps, pr = p_max / 1e5, p_stat / 1e5, p_red / 1e5
    return (c1 * pm * kst * pr ** e1 + c2 * (ps - offset) * pr ** e2) * volume ** e3


def rail_silo_rows(doc, rows, status, values):
    """🚂 Rail derailments (EN 1991-2 6.7.1), rail load groups (Table 6.11), footbridge service vehicles (5.6.3) and dust explosions (vent area after EN 14491) the way the report emits them: `unverifiedBasis` when a registry value is cited, `pass` when none is, `fail` above a utilisation of 1."""
    bridge = doc.get("structureKind") == "bridge"
    alpha = doc.get("railAlpha", 1.0)

    def covers(key, required, assumed, cited=True):
        rows[key] = required / assumed
        values[key] = required
        status[key] = "fail" if rows[key] > 1.0 + 1e-9 else "unverifiedBasis" if cited else "pass"

    def invalid(collection, row_id, leaf):
        rows[f"en1991.input.{collection}.{row_id}.{leaf}"] = 1.0
        status[f"en1991.input.{collection}.{row_id}.{leaf}"] = "fail"

    def positive(x):
        return math.isfinite(x) and x > 0

    for r in doc.get("dustExplosions", []):
        if not positive(r["ventArea"]):
            invalid("dustExplosions", r["id"], ["ventArea"][0])
            continue
        given = {"kst": r["kst"], "pMax": r["pMax"], "volume": r["volume"], "staticOpeningPressure": r["staticOpeningPressure"], "designPressure": r["designPressure"]}
        outside = next((leaf for leaf, _, low, high in DUST_DOMAIN if not (math.isfinite(given[leaf]) and low <= given[leaf] <= high)), None)
        if outside is None and given["designPressure"] < given["staticOpeningPressure"]:
            outside = "designPressure"
        if outside is not None:
            invalid("dustExplosions", r["id"], outside)
            continue
        covers(f"en1991.4.dust-vent-area.{r['id']}", dust_vent_area(r["kst"], r["pMax"], r["volume"], r["staticOpeningPressure"], r["designPressure"]), r["ventArea"])
    for r in doc.get("railLoadGroups", []):
        if not bridge:
            continue
        if r["loadGroup"] not in RAIL_GROUPS:
            invalid("railLoadGroups", r["id"], "loadGroup")
            continue
        bad = next((leaf for leaf, x in (("factorVertical", r["factorVertical"]), ("factorLongitudinal", r["factorLongitudinal"]), ("factorLateral", r["factorLateral"])) if not (math.isfinite(x) and 0.0 <= x <= 1.0)), None)
        if bad is not None:
            invalid("railLoadGroups", r["id"], bad)
            continue
        covers(f"en1991.2.rail-group-axle.{r['id']}", r["factorVertical"] * alpha * LM71_AXLE, r["assumedAxleLoad"], cited=False)
        covers(f"en1991.2.rail-group-udl.{r['id']}", r["factorVertical"] * alpha * LM71_UDL, r["assumedUdl"], cited=False)
        span = doc.get("bridgeSpan", 0.0)
        if positive(span):
            longitudinal = max(min(TRACTION[0] * span, TRACTION[1]), min(BRAKING[0] * span, BRAKING[1]))
            covers(f"en1991.2.rail-group-longitudinal.{r['id']}", r["factorLongitudinal"] * longitudinal, r["assumedLongitudinal"])
        covers(f"en1991.2.rail-group-lateral.{r['id']}", r["factorLateral"] * alpha * NOSING, r["assumedLateral"])
    for r in doc.get("railDerailments", []):
        if not bridge:
            continue
        line = DERAIL_I_N_M if r["situation"] == "i" else DERAIL_II_FACTOR * alpha * LM71_UDL if r["situation"] == "ii" else None
        if line is None:
            invalid("railDerailments", r["id"], "situation")
        elif not positive(r["assumedLineLoad"]):
            invalid("railDerailments", r["id"], "assumedLineLoad")
        elif not positive(r["loadedLength"]):
            invalid("railDerailments", r["id"], "loadedLength")
        elif not (math.isfinite(r["offset"]) and r["offset"] >= 0.0):
            invalid("railDerailments", r["id"], "offset")
        else:
            covers(f"en1991.2.derail-line-load.{r['id']}", line, r["assumedLineLoad"])
            covers(f"en1991.2.derail-length.{r['id']}", DERAIL_LENGTH_M, r["loadedLength"])
            covers(f"en1991.2.derail-offset.{r['id']}", r["offset"], DERAIL_OFFSET_LIMIT_M)
    for r in doc.get("footbridges", []):
        if not (positive(r["loadedLength"]) and positive(r["deckWidth"])):
            continue
        axle = r.get("assumedServiceAxle", 0.0)
        if not (math.isfinite(axle) and axle >= 0.0):
            invalid("footbridges", r["id"], "assumedServiceAxle")
        elif axle > 0.0:
            covers(f"en1991.2.footbridge-service.{r['id']}", SERVICE_AXLE, axle)


HELICOPTER_C = 3000.0
TIE_FACTOR = {0: 0.4, 1: 0.8}
TIE_CAP = 75000.0
GRAVITY = 9.80665


def hazard_rows(doc, rows):
    """🚁 Spec rows E16.04-E16.06 and E17.03, E17.05, E17.06 from the clauses: C sqrt(m), the lesser of factor (g + psi q) s L and 75 kN, psi_0 = 1 sum per stage, energy balance of a falling mass, N Delta / h and the entered derailment force."""
    for r in doc.get("helicopterLandings", []):
        if r["mass"] > 0 and r["assumedForce"] > 0:
            rows[f"en1991.1-7.helicopter-force.{r['id']}"] = HELICOPTER_C * math.sqrt(r["mass"]) / r["assumedForce"]
    for r in doc.get("tieRuns", []):
        if r["kind"] in TIE_FACTOR and r["providedForce"] > 0 and r["spacing"] > 0 and r["span"] > 0:
            rows[f"en1991.1-7.tie-force.{r['id']}"] = min(TIE_FACTOR[r["kind"]] * (r["deadLoad"] + r["psi"] * r["imposedLoad"]) * r["spacing"] * r["span"], TIE_CAP) / r["providedForce"]
    stages = {st["id"]: st for st in doc.get("constructionStages", [])}
    for stage_id, stage in stages.items():
        loads = [l["magnitude"] for l in doc.get("constructionLoads", []) if l["stage"] == stage_id]
        if loads and stage["assumedQk"] > 0:
            rows[f"en1991.1-6.stage-combination.{stage_id}"] = sum(loads) / stage["assumedQk"]
    for r in doc.get("fallingLoads", []):
        if r["mass"] > 0 and r["stoppingDistance"] > 0 and r["assumedForce"] > 0:
            rows[f"en1991.1-6.falling-load.{r['id']}"] = r["mass"] * GRAVITY * (r["dropHeight"] + r["stoppingDistance"]) / r["stoppingDistance"] / r["assumedForce"]
    for r in doc.get("executionImperfections", []):
        if r["height"] > 0 and r["assumedForce"] > 0:
            rows[f"en1991.1-6.execution-imperfection.{r['id']}"] = r["verticalLoad"] * r["tolerance"] / r["height"] / r["assumedForce"]
    for r in doc.get("derailmentCases", []):
        if r["designForceX"] > 0 and r["assumedForce"] > 0:
            rows[f"en1991.1-7.derailment-force.{r['id']}"] = r["designForceX"] / r["assumedForce"]


def self_check_rail_silo():
    """🧪️ Third-party agreement of the rail and dust rows: the Bartknecht vent area against a sympy evaluation, the group and derailment utilisations against exact rational arithmetic."""
    import sympy as sp

    rs = {"structureKind": "bridge", "bridgeSpan": 30.0, "railAlpha": 1.21,
          "railDerailments": [{"id": "a", "situation": "i", "assumedLineLoad": 90000.0, "loadedLength": 40.0, "offset": 1.0}, {"id": "b", "situation": "ii", "assumedLineLoad": 67760.0, "loadedLength": 10.0, "offset": 4.3050}, {"id": "c", "situation": "x", "assumedLineLoad": 1.0, "loadedLength": 1.0, "offset": 0.0}],
          "railLoadGroups": [{"id": "g", "loadGroup": "gr11", "factorVertical": 0.8, "factorLongitudinal": 0.5, "factorLateral": 1.0, "assumedAxleLoad": 242000.0, "assumedUdl": 77440.0, "assumedLongitudinal": 990000.0, "assumedLateral": 60500.0}, {"id": "h", "loadGroup": "gr99", "factorVertical": 1.0, "factorLongitudinal": 1.0, "factorLateral": 1.0, "assumedAxleLoad": 1.0, "assumedUdl": 1.0, "assumedLongitudinal": 1.0, "assumedLateral": 1.0}],
          "footbridges": [{"id": "s", "loadedLength": 20.0, "deckWidth": 3.0, "assumedServiceAxle": 80000.0}, {"id": "n", "loadedLength": 20.0, "deckWidth": 3.0, "assumedServiceAxle": 0.0}],
          "dustExplosions": [{"id": "u", "kst": 150.0, "pMax": 800000.0, "volume": 100.0, "ventArea": 6.174588305006346, "staticOpeningPressure": 20000.0, "designPressure": 50000.0}, {"id": "v", "kst": 350.0, "pMax": 800000.0, "volume": 100.0, "ventArea": 1.0, "staticOpeningPressure": 20000.0, "designPressure": 50000.0}]}
    r_rows, r_status, r_values = {}, {}, {}
    rail_silo_rows(rs, r_rows, r_status, r_values)
    assert abs(r_rows["en1991.2.derail-line-load.a"] - 0.5) < 1e-12 and abs(r_rows["en1991.2.derail-length.a"] - 0.5) < 1e-12
    assert abs(r_rows["en1991.2.derail-line-load.b"] - float(sp.Rational(14, 10) * sp.Float(1.21) * 80000 / 67760)) < 1e-9 and r_status["en1991.2.derail-offset.b"] == "fail"
    assert r_status["en1991.input.railDerailments.c.situation"] == "fail" and r_status["en1991.input.railLoadGroups.h.loadGroup"] == "fail"
    assert abs(r_rows["en1991.2.rail-group-axle.g"] - 0.8 * 1.21 * 250000.0 / 242000.0) < 1e-12 and r_status["en1991.2.rail-group-axle.g"] == "pass"
    assert abs(r_rows["en1991.2.rail-group-longitudinal.g"] - 0.5 * 990000.0 / 990000.0) < 1e-12 and abs(r_rows["en1991.2.rail-group-lateral.g"] - 1.21 * 100000.0 / 60500.0) < 1e-12
    assert r_status["en1991.2.rail-group-lateral.g"] == "fail" and abs(r_rows["en1991.2.footbridge-service.s"] - 0.5) < 1e-12 and "en1991.2.footbridge-service.n" not in r_rows
    kst, p_max, volume, p_stat, p_red = sp.symbols("kst p_max volume p_stat p_red", positive=True)
    area = (sp.Float(3.264e-5) * p_max * kst * p_red ** sp.Float(-0.569) + sp.Float(0.27) * (p_stat - sp.Float(0.1)) * p_red ** sp.Rational(-1, 2)) * volume ** sp.Float(0.753)
    symbolic = float(area.subs({kst: 150, p_max: 8.0, volume: 100, p_stat: 0.2, p_red: 0.5}))
    assert abs(symbolic - dust_vent_area(150.0, 800000.0, 100.0, 20000.0, 50000.0)) < 1e-12 and abs(symbolic - 3.087294152503173) < 1e-9, symbolic
    assert abs(r_rows["en1991.4.dust-vent-area.u"] - symbolic / 6.174588305006346) < 1e-12 and "en1991.input.dustExplosions.v.kst" in r_rows


def self_check():
    """🧪️ Third-party agreement: sigma from scipy.constants, the vehicle impact force from a scipy.integrate spring-mass ODE, the ISO 834 curve against numpy, the partition and thermal sums against closed forms, Gumbel c_prob against scipy.stats."""
    from scipy import constants, integrate, stats

    assert abs(SIGMA - constants.Stefan_Boltzmann) / constants.Stefan_Boltzmann < 1e-9
    mass, speed = 15000.0, 13.9
    sol = integrate.solve_ivp(lambda t, y: [y[1], -VEHICLE_STIFFNESS / mass * y[0]], (0, 2.0), [0.0, speed], rtol=1e-10, atol=1e-12, max_step=1e-3)
    peak = VEHICLE_STIFFNESS * float(np.max(sol.y[0]))
    ours = speed * math.sqrt(VEHICLE_STIFFNESS * mass)
    assert abs(peak - ours) / ours < 1e-4, (peak, ours)
    t = np.array([15.0, 30.0, 60.0])
    assert np.allclose([fire_curve_c("standard", x) for x in t], 20.0 + 345.0 * np.log10(8.0 * t + 1.0))
    gumbel = stats.gumbel_r()
    c_prob = lambda p: math.sqrt((1 - 0.2 * math.log(-math.log(1 - p))) / (1 - 0.2 * math.log(-math.log(0.98))))
    assert abs(c_prob(0.02) - 1.0) < 1e-12 and 0.0 < float(gumbel.ppf(0.98)) < 5.0
    assert abs(max(20.0 + OMEGA_M * 8.0, OMEGA_N * 20.0 + 8.0) - 26.0) < 1e-12
    from scipy import interpolate

    log_nodes = interpolate.interp1d([math.log(a) for a, _ in DELTA_Q1], [d for _, d in DELTA_Q1])
    assert abs(delta_q1(100.0) - float(log_nodes(math.log(100.0)))) < 1e-12 and abs(delta_q1(100.0) - 1.340824) < 1e-6
    assert abs(delta_q1(100.0) - (1.10 + 0.4 * 75.0 / 225.0)) > 0.1
    table_doc = {"fireLoadBasis": "occupancyTable", "fireOccupancy": "Office", "fireLoadDensityQf": 900e6}
    user_doc = {**table_doc, "fireLoadBasis": "userSpecified"}
    assert characteristic_fire_load(table_doc) == 511e6 and characteristic_fire_load(user_doc) == 900e6
    assert abs(COMBUSTION * characteristic_fire_load(user_doc) * float(log_nodes(math.log(100.0))) / (COMBUSTION * characteristic_fire_load(table_doc) * float(log_nodes(math.log(100.0)))) - 900.0 / 511.0) < 1e-12
    mass_kg, drop, stop = 100.0, 2.0, 0.1
    arrest = integrate.solve_ivp(lambda t, y: [y[1], GRAVITY - (mass_kg * GRAVITY * (drop + stop) / stop) / mass_kg * (1.0 if y[0] >= 0 else 0.0)], (0, 0.04), [0.0, math.sqrt(2 * GRAVITY * drop)], rtol=1e-10, atol=1e-12, max_step=1e-5)
    assert abs(float(np.max(arrest.y[0])) - stop) / stop < 1e-3, float(np.max(arrest.y[0]))
    tie_doc = {"tieRuns": [{"id": "t", "kind": 1, "providedForce": 75000.0, "spacing": 5.0, "span": 6.0, "deadLoad": 5000.0, "imposedLoad": 3000.0, "psi": 0.3}]}
    tie_rows = {}
    hazard_rows(tie_doc, tie_rows)
    assert abs(tie_rows["en1991.1-7.tie-force.t"] - 1.0) < 1e-12
    for eta in (0.05, 0.5, 2.0, 8.0):
        closed = 1.0 / eta - (1.0 - math.exp(-2.0 * eta)) / (2.0 * eta * eta)
        assert abs(closed - comp.admittance(eta)) < 1e-9
    at, av, heq, o_geom = compartment(100.0, 3.0)
    theta, theta_max = parametric_gas_c(0.3, 100.0, at, av, heq, 1160.0, 548.1288)
    reference = comp.parametric(100.0, at, av, heq, 1160.0, 548.1288, T_LIM_H)
    assert abs(theta_max + 273.15 - reference["theta_max"]) < 0.5 and abs(reference["O"] - o_geom) < 1e-12
    collection_doc = {
        "craneHoists": [{"id": "h", "releaseDevice": "grab", "releaseFraction": 0.4, "ratedLoad": 100000.0}, {"id": "bad", "releaseDevice": "hook", "releaseFraction": 1.5, "ratedLoad": 0.0}],
        "footbridges": [{"id": "f", "loadedLength": 50.0, "deckWidth": 3.0, "naturalFrequency": 1.9}],
        "railDynamics": [{"id": "r", "determinantLength": 16.0, "maintenance": "careful"}],
        "loadDispersals": [{"id": "d", "wheelLength": 0.4, "wheelWidth": 0.4, "pavementThickness": 0.1, "slabThickness": 0.4}],
        "bulkSolids": [{"id": "b", "internalFriction": math.radians(30.0)}],
        "rotatingMachines": [{"id": "m", "mass": 500.0, "eccentricity": 0.002, "frequency": 25.0}],
    }
    state, numbers = {}, {}
    evaluate({**collection_doc, "annex": "de", "floors": []}, state, numbers)
    assert abs(numbers["en1991.3.hoist-release.h"] - (1.0 - 0.4 * 1.5)) < 1e-12 and state["en1991.3.hoist-release.h"] == "unverifiedBasis"
    assert state["en1991.3.hoist-test-static.h"] == "unverifiedBasis" and state["en1991.3.hoist-test-dynamic.h"] == "pass"
    assert abs(numbers["en1991.3.hoist-test-static.h"] - 110000.0) < 1e-9 and abs(numbers["en1991.3.hoist-test-dynamic.h"] - 125000.0) < 1e-9
    assert state["en1991.input.craneHoists.bad.releaseDevice"] == "fail" and state["en1991.input.craneHoists.bad.releaseFraction"] == "fail" and state["en1991.input.craneHoists.bad.ratedLoad"] == "fail"
    assert abs(numbers["en1991.2.footbridge-load.f"] - 3500.0) < 1e-9 and abs(numbers["en1991.2.footbridge-horizontal.f"] - 0.1 * 3500.0 * 50.0 * 3.0) < 1e-9
    assert numbers["en1991.2.footbridge-walking.f.resonant"] is True and numbers["en1991.2.footbridge-walking.f.comfort-required"] is True and state["en1991.2.footbridge-horizontal.f"] == "pass"
    assert abs(numbers["en1991.2.rail-dynamic.r"] - (1.44 / (4.0 - 0.2) + 0.82)) < 1e-12 and state["en1991.2.rail-dynamic.r"] == "pass"
    assert abs(numbers["en1991.2.dispersal-footprint.d"] - 1.0) < 1e-12 and abs(numbers["en1991.4.bulk-k-mean.b"] - 0.55) < 1e-12
    omega = 2.0 * np.pi * 25.0
    swing = integrate.solve_ivp(lambda t, y: [y[1], -omega ** 2 * y[0]], (0, 0.01), [0.002, 0.0], rtol=1e-12, atol=1e-14, dense_output=True)
    peak_acceleration = float(np.max(np.abs(np.gradient(swing.sol(np.linspace(0, 0.01, 4001))[1], np.linspace(0, 0.01, 4001)))))
    assert abs(500.0 * peak_acceleration - numbers["en1991.3.machine-force.m"]) / numbers["en1991.3.machine-force.m"] < 2e-3
    import sympy as sp

    D, Q, H, r = 2.0, 5.0e6, 3.0, 1.0
    q_h, q_d = sp.Float(Q) / (sp.Float(1.11e6) * sp.Float(H) ** sp.Rational(5, 2)), sp.Float(Q) / (sp.Float(1.11e6) * sp.Float(D) ** sp.Rational(5, 2))
    l_h = H * (2.9 * q_h ** sp.Float(0.33) - 1)
    z_prime = 2.4 * D * (q_d ** sp.Rational(2, 5) - q_d ** sp.Rational(2, 3))
    y = float((r + H + z_prime) / (l_h + H + z_prime))
    flux = float(np.piecewise(y, [y <= 0.3, (y > 0.3) & (y < 1.0), y >= 1.0], [100e3, lambda t: 136.3e3 - 121e3 * t, lambda t: 15e3 * t ** -3.7]))
    assert abs(hasemi_flux(D, Q, H, r) - flux) < 1e-6 and abs(flux - 50907.1348) < 1e-3, flux
    theta = float(20 + sp.Rational(1, 4) * (sp.Float(0.8e6)) ** sp.Rational(2, 3) * (6 - (-1.02 * 2 + 0.00524 * sp.Float(1e6) ** sp.Float(0.4))) ** sp.Rational(-5, 3))
    assert abs(theta - 109.9452891508) < 1e-6, theta
    wave3 = {
        "structureKind": "bridge",
        "bridgeSpan": 30.0,
        "fatigueModels": [{"id": "a", "model": "flm1", "lane": 2, "assumedAxleLoad": 200000.0, "assumedUdl": 1000.0}, {"id": "b", "model": "flm3", "lane": 1, "assumedAxleLoad": 100000.0, "assumedUdl": 0.0}],
        "supportCollisions": [{"id": "c", "location": "deck", "assumedForceX": 2.0e6, "assumedForceY": 1.0e6, "height": 1.25, "positionX": 10.0}, {"id": "d", "location": "soffit", "assumedForceX": 4.0e5, "assumedForceY": 0.0, "height": 2.0, "positionX": 40.0}],
        "localFires": [{"id": "e", "fireDiameter": 2.0, "heatReleaseRate": 5.0e6, "ceilingHeight": 3.0, "horizontalDistance": 1.0, "assumedFlux": 100000.0}],
        "snowGuards": [{"id": "g", "roofId": "missing", "slopeLength": 6.0, "assumedForce": 1000.0}],
        "roofs": [],
    }
    w_rows, w_status, w_values = {}, {}, {}
    wave3_rows(wave3, w_rows, w_status, w_values)
    assert abs(w_rows["en1991.2.fatigue-axle.a"] - 140000.0 / 200000.0) < 1e-12 and abs(w_rows["en1991.2.fatigue-udl.a"] - 750.0 / 1000.0) < 1e-12
    assert w_status["en1991.2.fatigue-axle.b"] == "fail" and abs(w_values["en1991.2.fatigue-vehicle.b"] - 8.4) < 1e-12
    assert abs(w_rows["en1991.2.collision-x.c"] - 0.5) < 1e-12 and "en1991.2.collision-y.d" not in w_rows and w_status["en1991.input.supportCollisions.d.positionX"] == "fail"
    assert abs(w_rows["en1991.1-2.local-flux.e"] - flux / 100000.0) < 1e-9 and w_status["en1991.input.snowGuards.g.roofId"] == "fail"
    self_check_rail_silo()
    print(json.dumps({"impactOde": peak, "impactClosed": ours, "sigma": constants.Stefan_Boltzmann}))


def main():
    if "--self-check-rail-silo" in sys.argv:
        return self_check_rail_silo()
    if "--self-check" in sys.argv:
        return self_check()
    paths = [a for a in sys.argv[1:] if not a.startswith("--")]
    doc = json.load(open(paths[0], encoding="utf8")) if paths else json.load(sys.stdin)
    status, values = {}, {}
    rows = evaluate(doc, status, values)
    print(json.dumps({"checks": [{"id": k, "utilization": v, **({"status": status[k]} if k in status else {}), **({"value": values[k]} if k in values else {})} for k, v in rows.items()]}))


if __name__ == "__main__":
    main()
