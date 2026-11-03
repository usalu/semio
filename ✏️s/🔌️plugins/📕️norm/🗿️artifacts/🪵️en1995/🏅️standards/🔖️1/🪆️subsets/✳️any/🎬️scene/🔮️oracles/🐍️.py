"""🔮️ Independent oracle of the EN 1995 scene: section and hull areas (shapely when installed, shoelace otherwise), the EN 1995-1-1 6.3.2 k_c and 6.3.3 k_crit curves, the Johansen failure modes with equal embedment strength (beta = 1 closed forms),
the effective charring depth of EN 1995-1-2 4.2.2 and scene bounds. Reads a list of {"scene", "diagrams", "members", "connections"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import json
import math
import sys

try:
    from shapely.geometry import Polygon
except ImportError:
    Polygon = None


def kc(lam, beta):
    if lam <= 0.3:
        return 1.0
    k = 0.5 * (1.0 + beta * (lam - 0.3) + lam * lam)
    return 1.0 / (k + math.sqrt(max(k * k - lam * lam, 0.0)))


def kcrit(lam):
    if lam <= 0.75:
        return 1.0
    return 1.56 - 0.75 * lam if lam <= 1.4 else 1.0 / (lam * lam)


def shoelace(points):
    n = len(points)
    return abs(sum(points[i][0] * points[(i + 1) % n][1] - points[(i + 1) % n][0] * points[i][1] for i in range(n))) / 2.0


def ring(r):
    return [(p["x"], p["y"]) for p in r["points"]]


def hull_area(polygon):
    outer = ring(polygon["outer"])
    holes = [ring(h) for h in polygon["holes"]]
    if Polygon:
        return Polygon(outer, holes).area
    return shoelace(outer) - sum(shoelace(h) for h in holes)


def single_shear(fh, my, d, t1, t2):
    r = t2 / t1
    return {
        "a": fh * t1 * d,
        "b": fh * t2 * d,
        "c": 0.5 * fh * t1 * d * (math.sqrt(1.0 + 2.0 * (1.0 + r + r * r) + r * r) - (1.0 + r)),
        "d": 1.05 * fh * t1 * d / 3.0 * (math.sqrt(4.0 + 12.0 * my / (fh * d * t1 * t1)) - 1.0),
        "e": 1.05 * fh * t2 * d / 3.0 * (math.sqrt(4.0 + 12.0 * my / (fh * d * t2 * t2)) - 1.0),
        "f": 1.15 * math.sqrt(2.0 * my * fh * d),
    }


def double_shear(fh, my, d, t1, t2):
    return {
        "g": fh * t1 * d,
        "h": 0.5 * fh * t2 * d,
        "j": 1.05 * fh * t1 * d / 3.0 * (math.sqrt(4.0 + 12.0 * my / (fh * d * t1 * t1)) - 1.0),
        "k": 1.15 * math.sqrt(2.0 * my * fh * d),
    }


def data(node):
    return node["data"] if isinstance(node, dict) and "data" in node else node


def main():
    documents = json.load(sys.stdin)
    failures = []
    checks = [0]

    def check(ok, message):
        checks[0] += 1
        if not ok:
            failures.append(message)

    for document in documents:
        scene = document["scene"]
        members = {m["id"]: m for m in document["members"]}
        connections = {c["id"]: c for c in document["connections"]}
        bounds = scene["bounds"]
        entities = {e["entity"]["id"]: e for e in scene["entities"]}
        for entity in scene["entities"]:
            geometry = entity["geometry"]
            if geometry["kind"] != "member":
                continue
            member = data(geometry)
            for point in data(member["axis"])["points"]:
                check(all(bounds["min"][k] - 1e-9 <= point[k] <= bounds["max"][k] + 1e-9 for k in "xyz"), f"{entity['entity']['id']}: axis outside bounds")
        for mid, m in members.items():
            fire = m["fire"]
            section = data(data(entities[mid]["geometry"])["section"])
            if fire is None:
                if "parts" in section:
                    total = sum(
                        math.prod(p["value"] for p in data(part["shape"])["params"] if p["name"] in ("h", "b"))
                        for part in section["parts"]
                    )
                    check(abs(total - m["b"] * m["h"]) < 1e-9, f"{mid}: lamellae area {total} vs {m['b'] * m['h']}")
                continue
            check(fire["dEf"] > fire["dChar"] > 0.0, f"{mid}: effective depth must exceed char depth")
            check(abs(fire["bFi"] - max(m["b"] - 2.0 * fire["dEf"], 0.0)) < 1e-12 and abs(fire["hFi"] - max(m["h"] - 2.0 * fire["dEf"], 0.0)) < 1e-12, f"{mid}: residual section")
            char = hull_area(data(data(entities[f"{mid}:char"]["geometry"])["section"])["polygon"])
            zsl = hull_area(data(data(entities[f"{mid}:zsl"]["geometry"])["section"])["polygon"])
            c = fire["dChar"]
            check(abs(char - (m["b"] * m["h"] - (m["b"] - 2 * c) * (m["h"] - 2 * c))) < 1e-9, f"{mid}: char hull area")
            check(abs(zsl - ((m["b"] - 2 * c) * (m["h"] - 2 * c) - fire["bFi"] * fire["hFi"])) < 1e-9, f"{mid}: zero-strength hull area")

        for cid, c in connections.items():
            fastener = data(entities[f"{cid}:fasteners"]["geometry"])
            pattern = data(fastener["pattern"])
            check(pattern["nx"] == c["rows"] and pattern["ny"] == c["perRow"], f"{cid}: grid counts")
            check(abs(pattern["sy"] - c["a1"]) < 1e-12 and abs(pattern["sx"] - c["a2"]) < 1e-12, f"{cid}: grid spacings")

        for diagram in document["diagrams"]:
            kind, _, owner = diagram["id"].partition(":")
            series = {s["data"]["id"]: s["data"] for s in diagram["series"]}
            rows = {len(col["values"]) for col in diagram["tableTwin"]["columns"]}
            check(len(rows) == 1, f"{diagram['id']}: table twin columns differ in length")
            if kind == "kc":
                m = members[owner]
                for sid, beta in (("kc-solid", 0.2), ("kc-glued", 0.1)):
                    for q in series[sid]["points"]:
                        check(abs(q["y"] - kc(q["x"], beta)) < 1e-9, f"{diagram['id']}: {sid} at {q['x']}")
                own = 0.2 if m["betaC"] > 0.15 else 0.1
                for marker in diagram["markers"]:
                    rel = m["relY"] if marker["id"] == "marker-y" else m["relZ"]
                    check(abs(marker["at"]["y"] - kc(rel, own)) < 1e-9 and abs(marker["at"]["x"] - min(rel, 3.0)) < 1e-9, f"{diagram['id']}: {marker['id']}")
            if kind == "kcrit":
                for q in series["kcrit"]["points"]:
                    check(abs(q["y"] - kcrit(q["x"])) < 1e-9, f"{diagram['id']}: kcrit at {q['x']}")
            if kind == "johansen":
                c = connections[owner]
                heights = [series[f"mode-{m['mode']}"]["ring"][2]["y"] for m in c["modes"]]
                check(all(abs(h * 1e3 - m["rk"]) < 1e-6 * max(1.0, m["rk"]) for h, m in zip(heights, c["modes"])), f"{diagram['id']}: bar heights")
                lowest = min(range(len(c["modes"])), key=lambda i: c["modes"][i]["rk"])
                check(abs(diagram["markers"][0]["at"]["x"] - (lowest + 1)) < 1e-12, f"{diagram['id']}: governing mode")
                if not c["steel"] and c["kind"] != "screw":
                    formula = (double_shear if c["planes"] >= 2 else single_shear)(c["fh"], c["my"], c["d"], c["t1"], c["t2"])
                    for mode in c["modes"]:
                        check(abs(mode["rk"] - formula[mode["mode"]]) <= 1e-6 * max(1.0, formula[mode["mode"]]), f"{diagram['id']}: mode {mode['mode']} {mode['rk']} vs {formula[mode['mode']]}")
            if kind == "fire":
                m = members[owner]
                char = [q["y"] for q in series["d-char"]["points"]]
                eff = series["d-ef"]["points"]
                for q_char, q_eff in zip(series["d-char"]["points"], eff):
                    minutes = q_char["x"]
                    check(abs(q_eff["y"] - (q_char["y"] + min(minutes / 20.0, 1.0) * 7.0)) < 1e-9, f"{diagram['id']}: d_ef at {minutes} min")
                check(all(b >= a for a, b in zip(char, char[1:])), f"{diagram['id']}: char depth must not decrease")

    print(json.dumps({"ok": not failures, "checks": checks[0], "failures": failures}, ensure_ascii=False))
    sys.exit(0 if not failures else 1)


main()
