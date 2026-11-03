"""🔮️ Independent oracle of the EN 1993 scene: closed-form section areas, the EN 1993-1-1 6.3.1.2 reduction factor, the EN 1993-1-9 Wöhler line, the M-N polygon (shapely when installed, shoelace otherwise)
and scene bounds. Reads {"scene", "diagrams", "sections"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import json
import math
import sys

try:
    from shapely.geometry import Point, Polygon
except ImportError:
    Polygon = None

ALPHA = {"a₀": 0.13, "a": 0.21, "b": 0.34, "c": 0.49, "d": 0.76}


def chi(lam, alpha):
    if lam <= 0.2:
        return 1.0
    phi = 0.5 * (1.0 + alpha * (lam - 0.2) + lam * lam)
    return min(1.0, 1.0 / (phi + math.sqrt(max(phi * phi - lam * lam, 0.0))))


def i_area(h, b, tw, tf):
    return 2.0 * b * tf + (h - 2.0 * tf) * tw


def i_polygon(h, b, tw, tf):
    w, d = b / 2.0, h / 2.0
    return [(-w, -d), (w, -d), (w, -d + tf), (tw / 2, -d + tf), (tw / 2, d - tf), (w, d - tf), (w, d), (-w, d), (-w, d - tf), (-tw / 2, d - tf), (-tw / 2, -d + tf), (-w, -d + tf)]


def shoelace(points):
    n = len(points)
    return abs(sum(points[i][0] * points[(i + 1) % n][1] - points[(i + 1) % n][0] * points[i][1] for i in range(n))) / 2.0


def area(points):
    return Polygon(points).area if Polygon else shoelace(points)


def params(section_data):
    return {p["name"]: p["value"] for p in section_data["params"]}


def main():
    data = json.load(sys.stdin)
    failures = []
    checks = [0]

    def check(ok, message):
        checks[0] += 1
        if not ok:
            failures.append(message)

    sections = {s["id"]: s for s in data["sections"]}
    for entity in data["scene"]["entities"]:
        geometry = entity["geometry"]
        if geometry["kind"] != "member" or geometry["data"]["section"]["kind"] != "parametric":
            continue
        shape = geometry["data"]["section"]["data"]
        if shape["profile"] == "iShape":
            p = params(shape)
            closed = i_area(p["h"], p["b"], p["tw"], p["tf"])
            polygon = area(i_polygon(p["h"], p["b"], p["tw"], p["tf"]))
            check(abs(closed - polygon) < 1e-12, f"{entity['entity']['id']}: closed form {closed} vs polygon {polygon}")
            for section in sections.values():
                if abs(section["h"] - p["h"]) < 1e-12 and abs(section["b"] - p["b"]) < 1e-12:
                    check(abs(closed - section["area"]) / section["area"] < 0.05, f"{entity['entity']['id']}: outline {closed} vs catalogue {section['area']}")
        axis = geometry["data"]["axis"]["data"]["points"]
        bounds = data["scene"]["bounds"]
        for point in axis:
            check(all(bounds["min"][k] - 1e-9 <= point[k] <= bounds["max"][k] + 1e-9 for k in "xyz"), f"{entity['entity']['id']}: axis outside bounds")

    for diagram in data["diagrams"]:
        kind = diagram["id"].split(":")[0]
        series = [s["data"] for s in diagram["series"]]
        rows = {len(c["values"]) for c in diagram["tableTwin"]["columns"]}
        check(len(rows) == 1, f"{diagram['id']}: table twin columns differ in length")
        if kind == "buckling":
            curves = {}
            for s in series:
                name = s["id"].split("-")[-1]
                if name in ALPHA:
                    curves[name] = s["points"]
                    for q in s["points"]:
                        check(abs(q["y"] - chi(q["x"], ALPHA[name])) < 1e-9, f"{diagram['id']}: {name} at {q['x']}")
                if s["id"] == "euler":
                    check(all(abs(q["y"] * q["x"] * q["x"] - 1.0) < 1e-9 for q in s["points"]), f"{diagram['id']}: Euler hyperbola")
            for marker in diagram["markers"]:
                x, y = marker["at"]["x"], marker["at"]["y"]
                check(any(abs(chi(x, a) - y) < 1e-6 for a in ALPHA.values()), f"{diagram['id']}: marker {marker['id']} not on a buckling curve")
        if kind == "sn":
            wohler = next(s for s in series if s["id"] == "wohler")["points"]
            for a, b in zip(wohler, wohler[1:]):
                slope = math.log10(b["y"] / a["y"]) / math.log10(b["x"] / a["x"])
                expected = -1.0 / 3.0 if b["x"] <= 5.0e6 * (1 + 1e-9) else (-1.0 / 5.0 if b["x"] <= 1.0e8 * (1 + 1e-9) else 0.0)
                check(abs(slope - expected) < 1e-9, f"{diagram['id']}: slope {slope} vs {expected} near N={a['x']:.3g}")
            reference = next(q for q in wohler if abs(q["x"] - 2.0e6) < 1.0)
            category = reference["y"]
            check(abs(category - round(category)) < 1e-6, f"{diagram['id']}: Delta sigma_C at 2e6 cycles is {category}")
            limits = {s["id"]: s["value"] for s in series if "value" in s}
            check(abs(limits["delta-d"] - category * (2.0 / 5.0) ** (1.0 / 3.0)) < 1e-9, f"{diagram['id']}: fatigue limit")
        if kind == "mn":
            ring = [(q["x"], q["y"]) for q in next(s for s in series if "ring" in s)["ring"]]
            check(len(ring) >= 3, f"{diagram['id']}: ring too small")
            check(abs(area(ring) - shoelace(ring)) < 1e-6 * max(1.0, shoelace(ring)), f"{diagram['id']}: polygon area")
            for marker in diagram["markers"]:
                point = (marker["at"]["x"], marker["at"]["y"])
                inside = Polygon(ring).buffer(1e-9).contains(Point(point)) if Polygon else True
                check(marker.get("utilisation") is None or marker["utilisation"] > 1.0 or inside, f"{diagram['id']}: compliant demand outside the polygon")

    print(json.dumps({"ok": not failures, "checks": checks[0], "failures": failures}, ensure_ascii=False))
    sys.exit(0 if not failures else 1)


main()
