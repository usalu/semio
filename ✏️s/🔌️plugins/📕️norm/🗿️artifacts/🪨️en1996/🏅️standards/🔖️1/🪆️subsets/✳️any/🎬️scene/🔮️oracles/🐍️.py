"""🔮️ Independent oracle of the EN 1996 scene: mesh volumes of the wall zones against the closed-form wall volume with openings, the Annex G reduction factor Φ_m, the power law of f_k and
its caps, the 60° load spreading, the isotropic yield-line load 24 m/l² and the tabulated fire thickness. Reads
{"scene", "diagrams", "walls", "kE", "yieldSquare"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import json
import math
import sys

FIRE_CLASSES = [30, 60, 90, 120, 180, 240]
FIRE_CLAY_GROUP_1_MM = [100, 100, 150, 175, 240, 240]


def mesh_volume(entity):
    shape = entity["geometry"]["data"]["shape"]
    data = shape["data"]
    v = data["vertices"]
    total = 0.0
    for t in data["triangles"]:
        a, b, c = v[t["a"]], v[t["b"]], v[t["c"]]
        cross = (b["y"] * c["z"] - b["z"] * c["y"], b["z"] * c["x"] - b["x"] * c["z"], b["x"] * c["y"] - b["y"] * c["x"])
        total += (a["x"] * cross[0] + a["y"] * cross[1] + a["z"] * cross[2]) / 6.0
    return total


def opening_area(wall):
    total = sum(o["width"] for o in wall["openings"])
    gap = max((wall["length"] - total) / (len(wall["openings"]) + 1), 0.0)
    cursor, area = gap, 0.0
    for o in wall["openings"]:
        x0, x1 = min(cursor, wall["length"]), min(cursor + o["width"], wall["length"])
        z0 = min(max(o["sill"], 0.0), wall["height"])
        z1 = min(max(o["sill"] + o["height"], 0.0), wall["height"])
        area += max(x1 - x0, 0.0) * max(z1 - z0, 0.0)
        cursor += o["width"] + gap
    return area


def phi_m(slenderness, e_mk, t, k_e):
    e = max(abs(e_mk), 0.05 * t)
    a1 = 1.0 - 2.0 * e / t
    if a1 <= 0.0:
        return 0.0
    u = (slenderness / math.sqrt(k_e) - 0.063) / (0.73 - 1.17 * e / t)
    return a1 * math.exp(-0.5 * u * u)


def beta(a1_over_h, ab_over_aef):
    if ab_over_aef > 0.45:
        return 1.0
    return max(1.0, min((1.0 + 0.3 * a1_over_h) * (1.5 - 1.1 * ab_over_aef), min(1.25 + a1_over_h / 2.0, 1.5)))


def main():
    data = json.load(sys.stdin)
    failures = []
    checks = [0]

    def check(ok, message):
        checks[0] += 1
        if not ok:
            failures.append(message)

    walls = {w["id"]: w for w in data["walls"]}
    entities = {e["entity"]["id"]: e for e in data["scene"]["entities"]}
    bounds = data["scene"]["bounds"]

    for wall in walls.values():
        zones = [entities[f"{wall['id']}:{z}"] for z in ("zone-bottom", "zone-mid", "zone-top") if f"{wall['id']}:{z}" in entities]
        expected = (wall["length"] * wall["height"] - opening_area(wall)) * wall["thickness"]
        total = sum(mesh_volume(z) for z in zones)
        check(abs(total - expected) < 1e-9, f"{wall['id']}: zone volume {total} vs {expected}")
        for z in zones:
            for p in z["geometry"]["data"]["shape"]["data"]["vertices"]:
                check(all(bounds["min"][k] - 1e-9 <= p[k] <= bounds["max"][k] + 1e-9 for k in "xyz"), f"{z['entity']['id']}: vertex outside bounds")
        units = entities.get(f"{wall['id']}:units")
        if units is not None:
            check(0.0 < mesh_volume(units) < wall["length"] * wall["height"] * wall["thickness"], f"{wall['id']}: unit volume")
        for load in wall["concentrated"]:
            zone = entities[f"{wall['id']}:spread-{load['id']}"]
            ring = zone["geometry"]["data"]["shape"]["data"]["polygon"]["outer"]["points"]
            top, mid = (ring[1]["x"] - ring[0]["x"]), (ring[2]["x"] - ring[3]["x"])
            check(abs(top - load["bearingLength"]) < 1e-9, f"{zone['entity']['id']}: top width {top}")
            check(abs(mid - (load["bearingLength"] + wall["height"] / math.sqrt(3.0))) < 1e-9, f"{zone['entity']['id']}: mid width {mid}")

    for diagram in data["diagrams"]:
        kind, _, wall_id = diagram["id"].partition(":")
        if kind == "beta":
            wall_id, lc_id, c_id = wall_id.split(":")
        wall = walls.get(wall_id)
        series = {s["data"]["id"]: s["data"] for s in diagram["series"]}
        rows = {len(c["values"]) for c in diagram["tableTwin"]["columns"]}
        check(len(rows) == 1, f"{diagram['id']}: table twin columns differ in length")
        if kind == "slenderness":
            for sid, s in series.items():
                if not sid.startswith("phi-m-") or sid == "phi-m-wall":
                    continue
                ratio = int(sid.rsplit("-", 1)[1]) / 100.0
                for q in s["points"]:
                    check(abs(q["y"] - phi_m(q["x"], ratio * wall["thickness"], wall["thickness"], data["kE"])) < 1e-9, f"{diagram['id']}: {sid} at {q['x']}")
        if kind == "fk":
            curves = {sid: s["points"] for sid, s in series.items() if sid.startswith("fk-m") and sid != "fk-wall"}
            for sid, pts in curves.items():
                slopes = [math.log(b["y"] / a["y"]) / math.log(b["x"] / a["x"]) for a, b in zip(pts, pts[1:]) if b["y"] < 0.79 * b["x"] and a["y"] < 0.79 * a["x"]]
                check(all(abs(s - slopes[0]) < 1e-6 for s in slopes), f"{diagram['id']}: {sid} is not a power law")
                check(all(q["y"] <= 0.8 * q["x"] + 1e-9 for q in pts), f"{diagram['id']}: {sid} exceeds 0.8 f_b")
                check(all(b["y"] >= a["y"] - 1e-12 for a, b in zip(pts, pts[1:])), f"{diagram['id']}: {sid} not monotone")
            if "fk-m50" in curves and "fk-m100" in curves:
                ratios = [b["y"] / a["y"] for a, b in zip(curves["fk-m50"], curves["fk-m100"]) if b["y"] < 0.79 * b["x"]]
                check(all(abs(r - ratios[0]) < 1e-6 for r in ratios), f"{diagram['id']}: f_m exponent not constant")
        if kind == "beta":
            load = next(c for c in wall["concentrated"] if c["id"] == f"{lc_id}-{c_id}")
            ratio = load["ab"] / load["aEf"]
            for q in series["beta"]["points"]:
                check(abs(q["y"] - beta(q["x"], ratio)) < 1e-9, f"{diagram['id']}: beta at a1/H {q['x']}")
            for q in series["beta-envelope"]["points"]:
                check(abs(q["y"] - beta(q["x"], 0.0)) < 1e-9, f"{diagram['id']}: envelope at a1/H {q['x']}")
            at = load["a1"] / wall["height"]
            marker = next(m for m in diagram["markers"] if m["id"] == "load")
            check(abs(marker["at"]["x"] - min(at, series["beta"]["points"][-1]["x"])) < 1e-9 and abs(marker["at"]["y"] - beta(at, ratio)) < 1e-9, f"{diagram['id']}: design point")
            check(all(1.0 - 1e-12 <= q["y"] <= 1.5 + 1e-12 for q in series["beta"]["points"]), f"{diagram['id']}: beta outside 1..1.5")
        if kind == "fire" and wall is not None and wall["clayGroup1"]:
            for sid, factor in (("t-min", 1.0),):
                for q, row in zip(series[sid]["points"], FIRE_CLAY_GROUP_1_MM):
                    check(abs(q["y"] - row * factor) < 1e-9, f"{diagram['id']}: {sid} at R{q['x']}")
            check([q["x"] for q in series["t-min"]["points"]] == FIRE_CLASSES, f"{diagram['id']}: fire classes")

    square = data["yieldSquare"]
    check(abs(square["pressure"] - 24.0 * square["m"] / square["l"] ** 2) < 1e-4 * square["pressure"], f"isotropic square yield load {square['pressure']} vs {24.0 * square['m'] / square['l'] ** 2}")

    print(json.dumps({"ok": not failures, "checks": checks[0], "failures": failures}, ensure_ascii=False))
    sys.exit(0 if not failures else 1)


main()
