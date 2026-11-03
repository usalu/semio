"""🔮️ Independent oracle of the EN 1990 scene: the γ·ψ factors of the generated combinations from the printed tables, the glyph magnitudes, the normal tail Φ(−β) (scipy when installed, erfc otherwise),
the FORM design point on the reliability circle, the Annex D Table D1 factors, the ψ chart, the S–N slopes and the member geometry. Reads {"scene", "diagrams", "document", "columns", "glyphs"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import importlib.util
import json
import math
import sys
from pathlib import Path

try:
    from scipy.stats import norm as scipy_norm
except ImportError:
    scipy_norm = None

_FACTORS_PATH = next(parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py" for parent in Path(__file__).resolve().parents if (parent / "⚖️compliance" / "📜️annex-and-tables" / "⚖️action-factors" / "🐍️.py").is_file())
_SPEC = importlib.util.spec_from_file_location("action_factors", _FACTORS_PATH)
FACTORS = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(FACTORS)
PSI = {name: tuple(FACTORS.ndp(f"ndp.en1990.psi.{name}.{i}", "En") for i in range(3)) for name in ("office", "wind", "snow", "snow_high", "residential", "storage")}
KN_NODES = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 20.0, 30.0]
KN_KNOWN = [2.31, 2.01, 1.89, 1.83, 1.80, 1.77, 1.74, 1.72, 1.68, 1.67]
KN_UNKNOWN = {3.0: 3.37, 4.0: 2.63, 5.0: 2.33, 6.0: 2.18, 8.0: 2.00, 10.0: 1.92, 20.0: 1.76, 30.0: 1.73}
BETA_50 = {"rc1": 3.3, "rc2": 3.8, "rc3": 4.3}
GAMMA = {fam: (FACTORS.ndp(f"ndp.en1990.gG_sup.{fam}.A1", "En"), FACTORS.ndp(f"ndp.en1990.gG_inf.{fam}.A1", "En"), FACTORS.ndp(f"ndp.en1990.gQ.{fam}.A1", "En")) for fam in ("str", "equ")}


def phi(x):
    return scipy_norm.cdf(x) if scipy_norm else 0.5 * math.erfc(-x / math.sqrt(2.0))


def psi_of(action, altitude):
    category = action["category"]
    if category == "snow" and altitude > 1000.0:
        category = "snow_high"
    return PSI.get(category)


def expected_factor(family, equation, lead, action, k_fi, altitude):
    cls = action["class"]
    g_sup, g_inf, g_q = GAMMA["equ" if family == "equ" else "str"]
    if cls == "Permanent":
        if family in ("str", "equ"):
            return g_inf if action["favourable"] else k_fi * g_sup
        return 1.0
    if cls == "Variable":
        psi = psi_of(action, altitude)
        if psi is None:
            return None
        psi0, psi1, psi2 = psi
        is_lead = action["id"] == lead
        if family in ("str", "equ"):
            return k_fi * g_q * (1.0 if is_lead else psi0)
        if family == "sls-char":
            return 1.0 if is_lead else psi0
        if family == "sls-freq":
            return psi1 if is_lead else psi2
        if family == "sls-qp":
            return psi2
    return None


def main():
    data = json.load(sys.stdin)
    failures = []
    count = [0]

    def check(ok, message):
        count[0] += 1
        if not ok:
            failures.append(message)

    document = data["document"]
    altitude = 1200.0
    actions = {a["id"]: a for a in document["actions"]}
    spans = {m["id"]: m["span"] for m in document["members"]}

    for entity in data["scene"]["entities"]:
        geometry = entity["geometry"]
        if geometry["kind"] == "member" and ":" not in entity["entity"]["id"]:
            axis = geometry["data"]["axis"]["data"]["points"]
            length = math.dist([axis[0][k] for k in "xyz"], [axis[1][k] for k in "xyz"])
            check(abs(length - spans[entity["entity"]["id"]]) < 1e-9, f"{entity['entity']['id']}: axis length {length}")
            bounds = data["scene"]["bounds"]
            for point in axis:
                check(all(bounds["min"][k] - 1e-9 <= point[k] <= bounds["max"][k] + 1e-9 for k in "xyz"), f"{entity['entity']['id']}: axis outside bounds")

    for column in data["columns"]:
        for action_id, factor in column["factors"].items():
            expected = expected_factor(column["family"], column["equation"], column["leading"], actions[action_id], document["kFi"], altitude)
            if expected is not None and column["family"] in ("str", "equ", "sls-char", "sls-freq", "sls-qp") and column["equation"] in ("6.10", "6.14b", "6.15b", "6.16b"):
                check(abs(expected - factor) < 1e-9, f"{column['id']}: factor of {action_id} {factor} vs {expected}")

    for glyph in data["glyphs"]:
        column = next(c for c in data["columns"] if c["id"] == glyph["column"])
        factor = column["factors"].get(glyph["action"])
        value = actions[glyph["action"]]["value"]
        expected = abs(factor * value) if factor else abs(value)
        check(abs(glyph["magnitude"] - expected) < 1e-6, f"glyph {glyph['action']}: {glyph['magnitude']} vs {expected}")

    for diagram in data["diagrams"]:
        series = {s["data"]["id"]: s["data"] for s in diagram["series"]}
        rows = {len(c["values"]) for c in diagram["tableTwin"]["columns"]}
        check(len(rows) == 1, f"{diagram['id']}: table twin columns differ in length")
        kind = diagram["id"].split(":")[0]
        if kind == "reliability":
            for q in series["pf"]["points"]:
                check(abs(q["y"] - phi(-q["x"])) <= 1e-9 * phi(-q["x"]) + 1e-18, f"reliability: P_f at beta {q['x']}")
            marker = next(m for m in diagram["markers"] if m["id"] == "beta")
            check(abs(marker["at"]["x"] - min(document["beta"], 7.0)) < 1e-9, "reliability: marker beta")
            check(abs(marker["at"]["y"] - phi(-marker["at"]["x"])) <= 1e-9 * marker["at"]["y"], "reliability: marker on the curve")
            if abs(document["referencePeriodYears"] - 50.0) < 1e-9:
                for name, target in BETA_50.items():
                    check(abs(series["target-" + name]["value"] - target) < 1e-9, f"reliability: {name} target")
        if kind == "form":
            beta = max(document["beta"], 0.5)
            for q in series["circle"]["points"]:
                check(abs(math.hypot(q["x"], q["y"]) - beta) < 1e-9, "form: circle radius")
            star = next(m for m in diagram["markers"] if m["id"] == "design-point")["at"]
            norm = math.hypot(0.7, 0.8)
            check(abs(star["x"] - beta * 0.7 / norm) < 1e-9 and abs(star["y"] + beta * 0.8 / norm) < 1e-9, f"form: design point {star}")
            line = series["limit-state"]["points"]
            n = (star["x"] / beta, star["y"] / beta)
            for q in line:
                check(abs(q["x"] * n[0] + q["y"] * n[1] - beta) < 1e-9, "form: limit state is tangent to the circle")
        if kind == "annex-d":
            ns = diagram["tableTwin"]["columns"][0]["values"]
            known = diagram["tableTwin"]["columns"][1]["values"]
            unknown = diagram["tableTwin"]["columns"][2]["values"]
            for n_value, k in zip(KN_NODES, KN_KNOWN):
                if n_value in ns:
                    check(abs(known[ns.index(n_value)] - k) < 1e-9, f"annex-d: known at n={n_value}")
            for n_value, k in KN_UNKNOWN.items():
                if n_value in ns:
                    check(abs(unknown[ns.index(n_value)] - k) < 1e-9, f"annex-d: unknown at n={n_value}")
            check(all(a >= b for a, b in zip(known, known[1:])), "annex-d: k_n decreases with n")
        if kind == "psi":
            index, p0, p1, p2 = [c["values"] for c in diagram["tableTwin"]["columns"]]
            check(all(a >= b >= c >= 0.0 for a, b, c in zip(p0, p1, p2)), "psi: psi0 >= psi1 >= psi2")
            known_rows = set(PSI.values())
            check(all((a, b, c) in known_rows for a, b, c in zip(p0, p1, p2)), "psi: rows are Table A1.1 rows")
        if kind == "fatigue":
            wohler = series["sn"]["points"]
            delta_d = (2.0 / 5.0) ** (1.0 / 3.0)
            for a, b in zip(wohler, wohler[1:]):
                slope = math.log10(b["y"] / a["y"]) / math.log10(b["x"] / a["x"])
                upper = a["y"] >= delta_d - 1e-9
                if upper and b["y"] >= delta_d - 1e-9:
                    check(abs(slope + 1.0 / 3.0) < 1e-6, f"{diagram['id']}: slope {slope} above the knee")
                elif a["y"] < delta_d and b["y"] < delta_d:
                    check(abs(slope + 1.0 / 5.0) < 1e-6, f"{diagram['id']}: slope {slope} below the knee")
        if kind == "matrix":
            columns = data["columns"]
            for position, column in enumerate(columns):
                for table in diagram["tableTwin"]["columns"][1:]:
                    header = table["header"]["en"]
                    action_id = header.replace("γ·ψ of ", "")
                    check(abs(table["values"][position] - column["factors"].get(action_id, 0.0)) < 1e-12, f"matrix: {action_id} in {column['id']}")

    print(json.dumps({"ok": not failures, "checks": count[0], "failures": failures}, ensure_ascii=False))
    sys.exit(0 if not failures else 1)


main()
