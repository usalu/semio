"""🚦️ Independent EN 1990 document oracle: a committed snapshot (JSON on stdin or a path) to the utilisation of every check family of the spec-trace `check` rows (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: closed forms and third-party libraries). It wraps the brute-force combination, FORM and Table D1 oracle `../⚖️compliance/🐍️.py` (scipy) for effects and adds the divisions by the member resistance or criterion that the report defines: STR/GEO 6.10, 6.10a/b, reversal `.min`, set C, EQU (destabilising effect over R_d, stabilising demand over R_d + E_stb), 6.11b/6.12b, SLS 6.14b to 6.16b, transient A2, A1.4 deflection and vibration, fatigue range and Palmgren-Miner damage, Annex B (beta, DSL, IL, working life), Annex C FORM, Annex D design value, E90.10 to E90.12. Utilisation of a minimum check is required over provided (EN 1990 6.4.1 style). Written from EN 1990 and the clause tables, not from the Rust. SI throughout."""

import importlib.util
import json
import math
import pathlib
import sys

import numpy as np
from scipy.stats import norm

HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("en1990_compliance", HERE.parent / "⚖️compliance" / "🐍️.py")
comp = importlib.util.module_from_spec(spec)
spec.loader.exec_module(comp)

STATUS, VALUES = {}, {}
KD_NODES = (1, 2, 3, 4, 5, 6, 8, 10, 20, 30)
KD_D2 = (4.36, 3.77, 3.56, 3.44, 3.37, 3.33, 3.27, 3.23, 3.16, 3.13)
KD_INFINITY = 3.04


def KD_KNOWN_D2(n):
    """📈 k_d,n of Table D2 (V_X known): the tabulated nodes, linear in 1/n between them (n = infinity: 3.04)."""
    nodes = [1.0 / x for x in KD_NODES] + [0.0]
    values = list(KD_D2) + [KD_INFINITY]
    return float(np.interp(1.0 / n, nodes[::-1], values[::-1]))


def xd_ok(series):
    """✅ The design value X_d of the series exists: a positive sample, mean and spread (the Annex D.7.2 evaluation is determinate)."""
    return series["sampleSize"] >= 1 and series["mean"] > 0 and series["stdDev"] >= 0 and series["conversionFactor"] > 0


DWL = {1: (10, 10), 2: (10, 30), 3: (15, 30), 4: (50, 50), 5: (100, 100)}
ALPHA_CR_FIRST_ORDER = 10.0
GROUP_COEFFICIENT = {
    ("gr1a", "road_traffic"): 1.0, ("gr1a", "road_udl"): 1.0, ("gr1a", "road_pedestrian"): 1.0, ("gr1b", "road_single_axle"): 1.0,
    ("gr2", "road_horizontal"): 1.0, ("gr3", "road_gr3_pedestrian"): 1.0, ("gr4", "road_gr4_crowd"): 1.0, ("gr5", "road_special"): 1.0,
}


def effective_group(doc, variable):
    """🚦️ The exclusive group of a variable: its trafficGroups row (lower-cased key) wins over the free group text."""
    row = next((r for r in doc.get("trafficGroups", []) if r["variableId"] == variable["id"]), None)
    return row["group"].lower() if row else variable.get("group", "")
TRAFFIC_GROUP = {
    "road_traffic": "gr1a", "road_udl": "gr1a", "road_pedestrian": "gr1a", "road_single_axle": "gr1b", "road_horizontal": "gr2",
    "road_gr3_pedestrian": "gr3", "road_gr4_crowd": "gr4", "road_special": "gr5", "footbridge_crowd": "fb-crowd", "footbridge_service": "fb-service",
    "rail_traffic": "lm71", "rail_sw2": "sw2", "rail_unloaded": "unloaded", "rail_hslm": "hslm",
}


def divide(demand, capacity):
    return demand / capacity if capacity > 0 else None


def actions_of(doc, member):
    eff = {(x["memberId"], x["actionId"]): x["influence"] for x in doc["effects"]}
    acts = []
    for p in doc["permanents"]:
        acts.append(dict(id=p["id"], cls="P" if p["kind"] == "prestress" else "G", value=eff.get((member["id"], p["id"]), 0.0) * p["gk"], cat="", group=""))
    for q in doc["variables"]:
        acts.append(dict(id=q["id"], cls="Q", value=eff.get((member["id"], q["id"]), 0.0) * q["qk"], cat=comp.category(q["category"], doc["altitudeM"]), group=q.get("group", "")))
    for a in doc["accidentals"]:
        acts.append(dict(id=a["id"], cls="A", value=eff.get((member["id"], a["id"]), 0.0) * a["ad"], cat="", group=""))
    gi = {"I": 0.8, "II": 1.0, "III": 1.2, "IV": 1.4}
    for s in doc["seismics"]:
        acts.append(dict(id=s["id"], cls="E", value=eff.get((member["id"], s["id"]), 0.0) * gi[s["importanceClass"]] * s["aEk"], cat="", group=""))
    return acts


def evaluate(doc):
    """📋️ Every check id this oracle reproduces with its utilisation."""
    base = comp.snapshot(doc)
    annex = "De" if doc["annex"] == "De" else "En"
    domain = 2 if doc["structureKind"] != "building" else 1
    kfi = comp.KFI[int(doc["reliabilityClass"][2:])]
    rows = {}

    def put(key, value):
        if value is not None and math.isfinite(value):
            rows[key] = value

    rail = doc["structureKind"] == "railBridge"
    for sls in doc.get("bridgeSls", []) if doc["structureKind"] in ("footbridge", "railBridge", "roadBridge") else []:
        key = f"{sls['memberId']}.{sls['id']}"
        if (doc["structureKind"] == "footbridge" or rail) and sls["deckAccelerationLimit"] > 0:
            put(f"en1990.a2.acceleration.{key}", sls["deckAcceleration"] / sls["deckAccelerationLimit"])
        if rail and sls["deckTwistLimit"] > 0:
            put(f"en1990.a2.twist.{key}", sls["deckTwist"] / sls["deckTwistLimit"])
        if sls["bridgeDeflectionLimit"] > 0:
            put(f"en1990.a2.deflection.{key}", sls["bridgeDeflection"] / sls["bridgeDeflectionLimit"])

    if doc["structureKind"] in ("roadBridge", "footbridge", "railBridge"):
        traffic = [(q, TRAFFIC_GROUP[q["category"]]) for q in doc["variables"] if q["category"] in TRAFFIC_GROUP]
        if len({g for _, g in traffic}) >= 2:
            def violates(q, g):
                return effective_group(doc, q) == "" or any(
                    o is not q and ((effective_group(doc, o) == effective_group(doc, q) and og != g) or (og == g and effective_group(doc, o) != effective_group(doc, q)))
                    for o, og in traffic
                )
            put("en1990.a2.traffic-groups", 1.0 + sum(violates(q, g) for q, g in traffic) / len(traffic))
        for row in doc.get("trafficGroups", []):
            variable = next((q for q in doc["variables"] if q["id"] == row["variableId"]), None)
            coefficient = GROUP_COEFFICIENT.get((row["group"].lower(), variable["category"])) if variable else None
            if coefficient is not None and TRAFFIC_GROUP.get(variable["category"]) == row["group"].lower():
                put(f"en1990.a2.traffic-group.{row['id']}", coefficient)

    for m in doc["members"]:
        mid = m["id"]
        for kind, field in (("str", "rdStr"), ("geo", "rdGeo")):
            for key, value in base.items():
                if key.startswith(f"en1990.{kind}.{mid}.") and key.endswith(("6.10", "6.10a", "6.10b", ".min")) and m[field] > 0:
                    put(key, value / m[field])
        put(f"en1990.geo.{mid}.set-c", divide(base[f"en1990.geo.{mid}.set-c"], m["rdGeo"]))
        for eq, field in (("6.11b", "rdStr"), ("6.12b", "rdStr")):
            if f"en1990.{eq}.{mid}" in base:
                put(f"en1990.{eq}.{mid}", divide(base[f"en1990.{eq}.{mid}"], m[field]))
        if f"en1990.a2.transient.{mid}" in base:
            put(f"en1990.a2.transient.{mid}", divide(base[f"en1990.a2.transient.{mid}"], m["rdStr"]))
        for eq, field in (("6.14b", "slsLimitChar"), ("6.15b", "slsLimitFreq"), ("6.16b", "slsLimitQp")):
            put(f"en1990.sls.{mid}.{eq}", divide(base[f"en1990.sls.{mid}.{eq}"], m[field]))
        acts = actions_of(doc, m)
        w = comp.worst(comp.combos(acts, "6.10", annex, domain, "equ", kfi), True)
        dst, stb = w[0], -w[1]
        put(f"en1990.equ.destab.{mid}", divide(max(dst - stb, 0.0), m["rdEquDestab"]))
        put(f"en1990.equ.stab.{mid}", divide(dst, m["rdEquStab"] + stb))
        if m["deflectionLimitRatio"] > 0 and m["span"] > 0:
            put(f"en1990.a14.deflection.{mid}", (m["deflectionW1"] + m["deflectionW2"] + m["deflectionW3"] - m["precamber"]) / (m["span"] / m["deflectionLimitRatio"]))
            if m["deflectionLimitRatioW3"] > 0:
                put(f"en1990.a14.deflection-w3.{mid}", m["deflectionW3"] / (m["span"] / m["deflectionLimitRatioW3"]))
        if m["vibrationFrequencyMin"] > 0 and m["vibrationFrequency"] > 0:
            put(f"en1990.a14.vibration.{mid}", m["vibrationFrequencyMin"] / m["vibrationFrequency"])
        if m["rdFat"] > 0:
            put(f"en1990.fat.{mid}", base[f"en1990.fat.{mid}"] / m["rdFat"])
        if f"en1990.fat.damage.{mid}" in base:
            put(f"en1990.fat.damage.{mid}", base[f"en1990.fat.damage.{mid}"])
        if m["materialStrengthK"] > 0 and m["materialPartialFactor"] > 0 and m["materialDesignStrength"] > 0:
            eta = m["materialConversionFactor"] if m["materialConversionFactor"] > 0 else 1.0
            put(f"en1990.e10.material.{mid}", m["materialDesignStrength"] / (eta * m["materialStrengthK"] / m["materialPartialFactor"]))
        if m["nominalDimension"] > 0 and m["dimensionTolerance"] != 0 and m["designDimension"] > 0:
            deviation = abs(m["designDimension"] - m["nominalDimension"])
            put(f"en1990.e11.geometry.{mid}", max(abs(m["dimensionTolerance"]) / deviation, 1.0) if deviation < abs(m["dimensionTolerance"]) else abs(m["dimensionTolerance"]) / deviation)
        if m["memberAlphaCr"] > 0 and m["secondOrderAnalysis"] < 0.5:
            put(f"en1990.e12.analysis.{mid}", ALPHA_CR_FIRST_ORDER / m["memberAlphaCr"])
    for series in doc.get("testSeries", []):
        xd = base.get(f"en1990.annex-d.{series['id']}")
        if xd:
            put(f"en1990.annex-d.{series['id']}", series["required"] / xd)
    for series in doc.get("testSeries", []):
        n, known = series["sampleSize"], series["variationKnown"]
        if known > 0 and n >= 1 and xd_ok(series):
            kd = KD_KNOWN_D2(n)
            put(f"en1990.annex-d.{series['id']}.direct", 0.0)
            STATUS[f"en1990.annex-d.{series['id']}.direct"] = "info"
            VALUES[f"en1990.annex-d.{series['id']}.direct"] = series["conversionFactor"] * series["mean"] * (1.0 - kd * known)
        if n < 3 and xd_ok(series):
            put(f"en1990.annex-d.{series['id']}.sample", 0.0)
            STATUS[f"en1990.annex-d.{series['id']}.sample"] = "warning"
    for analysis in doc.get("reliabilityAnalyses", []):
        if analysis["resistanceDistribution"] != "normal" or analysis["actionDistribution"] != "normal" or analysis["resistanceSd"] <= 0 or analysis["actionSd"] <= 0:
            continue
        ratio = analysis["actionSd"] / analysis["resistanceSd"]
        inside = 0.16 < ratio < 7.6
        put(f"en1990.annex-c.alpha.{analysis['id']}", 0.0)
        STATUS[f"en1990.annex-c.alpha.{analysis['id']}"] = "info" if inside else "warning"
        mr, sr, me, se = analysis["resistanceMean"], analysis["resistanceSd"], analysis["actionMean"], analysis["actionSd"]
        beta = (mr - me) / math.hypot(sr, se)
        r_star, e_star = mr - sr * sr / math.hypot(sr, se) * beta, me + se * se / math.hypot(sr, se) * beta
        r_k, e_k = norm.ppf(0.05, mr, sr), norm.ppf(0.98, me, se)
        if min(r_k, e_k, r_star, e_star) > 0:
            put(f"en1990.annex-c.partial-factors.{analysis['id']}", 0.0)
            STATUS[f"en1990.annex-c.partial-factors.{analysis['id']}"] = "info"
            VALUES[f"en1990.annex-c.partial-factors.{analysis['id']}"] = r_k / r_star * e_star / e_k
    rc = int(doc["reliabilityClass"][2:])
    for analysis in doc.get("reliabilityAnalyses", []):
        beta = base.get(f"_form.en1990.annex-c.form.{analysis['id']}")
        if beta:
            b1, b50 = comp.BETA[rc]
            put(f"en1990.annex-c.form.{analysis['id']}", comp.beta_target(b1, b50, doc["referencePeriodYears"]) / beta)
    if doc["betaComputed"] > 0:
        put("en1990.annex-c.beta", base["_limit.en1990.annex-c.beta"] / doc["betaComputed"])
    for key, field, prefix in (("dsl", "supervisionLevel", "DSL"), ("il", "inspectionLevel", "IL")):
        text = doc[field].upper()
        if text in (f"{prefix}1", f"{prefix}2", f"{prefix}3"):
            put(f"en1990.annex-b.{key}", rc / int(text[-1]))
    lo, hi = DWL.get(doc["designWorkingLifeCategory"], (None, None))
    if hi:
        put("en1990.table-2.1.design-working-life", doc["designWorkingLifeYears"] / hi)
    if doc["consequenceClass"] in (1, 2, 3):
        put("en1990.annex-b.rc-vs-cc", 0.0)
        STATUS["en1990.annex-b.rc-vs-cc"] = "pass" if doc["consequenceClass"] == int(doc["reliabilityClass"][2:]) else "warning"
    if doc["structureKind"] in ("roadBridge", "footbridge", "railBridge"):
        years = doc["designWorkingLifeYears"]
        put("en1990.a2.working-life", min(years, 100.0) / 100.0)
        STATUS["en1990.a2.working-life"] = "pass" if abs(years - 100.0) < 1e-6 else "warning"
    return rows


def self_check():
    """🧪️ Third-party agreement: FORM of two normal variables against the closed form and scipy.stats, k_n against scipy.stats.t, the elastic line of a beam against anastruct and Pynite."""
    import numpy as np
    from scipy import stats

    r = comp.scipy_dist("normal", 300.0, 30.0)
    e = comp.scipy_dist("normal", 150.0, 30.0)
    beta, *_ = comp.form(r, e)
    closed = (300.0 - 150.0) / math.hypot(30.0, 30.0)
    assert abs(beta - closed) < 1e-6, (beta, closed)
    assert abs(float(stats.norm.cdf(-beta)) - float(stats.norm.cdf(-closed))) < 1e-9
    table = comp.table_d1()
    assert abs(table["unknown"][4] - float(stats.t.ppf(0.95, 4) * math.sqrt(1 + 1 / 5))) < 1e-12
    from anastruct import SystemElements

    ss = SystemElements(EA=1e9, EI=1e6)
    ss.add_element([[0, 0], [6, 0]])
    ss.add_support_hinged(1)
    ss.add_support_roll(2)
    ss.q_load(-10.0, 1)
    ss.solve()
    reaction = abs(ss.get_node_results_system(1)["Fy"])
    assert abs(reaction - 30.0) < 1e-6, reaction
    from Pynite import FEModel3D

    length, ei = 6.0, 210e9 * 1.0e-4
    model = FEModel3D()
    model.add_material("S", 210e9, 81e9, 0.3, 7850)
    for name, x in (("A", 0.0), ("B", length)):
        model.add_node(name, x, 0, 0)
    model.add_section("Sec", 0.01, 1.0e-4, 1.0e-4, 1e-6)
    model.add_member("M", "A", "B", "S", "Sec")
    model.def_support("A", True, True, True, True, False, False)
    model.def_support("B", False, True, True, False, False, False)
    model.add_member_dist_load("M", "Fy", -10.0, -10.0)
    model.analyze()
    moment = max(abs(model.members["M"].max_moment("Mz")), abs(model.members["M"].min_moment("Mz")))
    assert abs(moment - 10.0 * length ** 2 / 8) < 1e-6 * 45, moment
    mid = abs(model.members["M"].deflection("dy", 0.5 * length))
    assert abs(mid - 5 * 10.0 * length ** 4 / (384 * ei)) < 5e-3 * mid
    s = 0.25
    line = abs(model.members["M"].deflection("dy", s * length)) / mid
    assert abs(line - 3.2 * s * (1 - 2 * s * s + s ** 3)) < 5e-3, line
    print(json.dumps({"form_beta": beta, "closed": closed, "reaction": reaction, "moment": moment}))


def main():
    if "--self-check" in sys.argv:
        return self_check()
    paths = [a for a in sys.argv[1:] if not a.startswith("--")]
    doc = json.load(open(paths[0], encoding="utf8")) if paths else json.load(sys.stdin)
    rows = evaluate(doc)
    print(json.dumps({"checks": [{"id": k, "utilization": v, **({"status": STATUS[k]} if k in STATUS else {}), **({"value": VALUES[k]} if k in VALUES else {})} for k, v in rows.items()]}))


if __name__ == "__main__":
    main()
