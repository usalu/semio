"""🔮️ Independent oracle of the EN 1991 scene: closed-form peak velocity pressure, c_pe(A), snow shape coefficients, fire curves, Janssen silo pressure (ODE integration with scipy when installed, Runge-Kutta otherwise),
the LM1 influence line, and the areas of the wind pressure zones (vector areas).
Reads {"scene", "diagrams", "building", "wind", "silo", "bridgeSpan"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}."""

import json
import math
import sys

try:
    from scipy.integrate import solve_ivp
except ImportError:
    solve_ivp = None

Z0 = {0: 0.003, 1: 0.01, 2: 0.05, 3: 0.3, 4: 1.0}
ZMIN = {0: 1.0, 1: 1.0, 2: 2.0, 3: 5.0, 4: 10.0}


def qp(z, vb, rho, terrain, c_o):
    z0 = Z0[terrain]
    zc = max(z, ZMIN[terrain])
    kr = 0.19 * (z0 / 0.05) ** 0.07
    cr = kr * math.log(zc / z0)
    iv = 1.0 / (c_o * math.log(zc / z0))
    return (1.0 + 7.0 * iv) * 0.5 * rho * (cr * c_o * vb) ** 2


def mu1(alpha):
    if alpha <= 30.0:
        return 0.8
    return 0.8 * (60.0 - alpha) / 30.0 if alpha < 60.0 else 0.0


def standard(t):
    return 20.0 + 345.0 * math.log10(8.0 * t + 1.0)


def external(t):
    return 20.0 + 660.0 * (1.0 - 0.687 * math.exp(-0.32 * t) - 0.313 * math.exp(-3.8 * t))


def hydrocarbon(t):
    return 20.0 + 1080.0 * (1.0 - 0.325 * math.exp(-0.167 * t) - 0.675 * math.exp(-2.5 * t))


def janssen(depth, gamma, a, mu, k):
    if solve_ivp is not None:
        solution = solve_ivp(lambda z, p: [gamma * k - mu * k / a * p[0]], (0.0, max(depth, 1e-9)), [0.0], rtol=1e-10, atol=1e-8)
        return float(solution.y[0][-1]) if depth > 0 else 0.0
    steps = 2000
    h = depth / steps
    p = 0.0
    f = lambda q: gamma * k - mu * k / a * q
    for _ in range(steps):
        k1 = f(p)
        k2 = f(p + h / 2 * k1)
        k3 = f(p + h / 2 * k2)
        k4 = f(p + h * k3)
        p += h / 6 * (k1 + 2 * k2 + 2 * k3 + k4)
    return p


def walk(node):
    yield node
    if isinstance(node, dict):
        for value in node.values():
            yield from walk(value)
    elif isinstance(node, list):
        for value in node:
            yield from walk(value)


def vector_area(points):
    ox, oy, oz = points[0]["x"], points[0]["y"], points[0]["z"]
    sx = sy = sz = 0.0
    for a, b in zip(points[1:], points[2:]):
        u = (a["x"] - ox, a["y"] - oy, a["z"] - oz)
        v = (b["x"] - ox, b["y"] - oy, b["z"] - oz)
        sx += u[1] * v[2] - u[2] * v[1]
        sy += u[2] * v[0] - u[0] * v[2]
        sz += u[0] * v[1] - u[1] * v[0]
    return 0.5 * math.sqrt(sx * sx + sy * sy + sz * sz)


def zone_areas(scene):
    areas = {}
    for entity in scene["entities"]:
        letter = entity["entity"]["id"].removeprefix("wind-zone-")
        if not entity["entity"]["id"].startswith("wind-zone-") or len(letter) != 1:
            continue
        for node in walk(entity["geometry"]):
            if isinstance(node, dict) and isinstance(node.get("zones"), list):
                areas[letter] = sum(vector_area(z["polygon"]["outer"]["points"]) for z in node["zones"])
    return areas


def main():
    data = json.load(sys.stdin)
    failures = []
    counter = [0]

    def check(ok, message):
        counter[0] += 1
        if not ok:
            failures.append(message)

    def close(a, b, tolerance=1e-6):
        return abs(a - b) <= tolerance * max(1.0, abs(b))

    building, wind, silo = data["building"], data["wind"], data["silo"]
    d, b, h = building["depth"], building["width"], building["height"]
    e = min(b, 2.0 * h)
    areas = zone_areas(data["scene"])
    wall = {"A": 0.0, "B": 0.0, "C": 0.0}
    wa, wb = (d, 0.0) if e >= 5.0 * d else ((e / 5.0, d - e / 5.0) if e >= d else (e / 5.0, 4.0 * e / 5.0))
    widths = {"A": wa, "B": wb, "C": max(d - e, 0.0) if e < d else 0.0}
    for letter, width in widths.items():
        check(close(areas.get(letter, 0.0), 2.0 * width * h), f"wall zone {letter}: {areas.get(letter, 0.0)} vs {2.0 * width * h}")
    check(close(areas["D"], b * h) and close(areas["E"], b * h), "windward and leeward wall areas")
    roof_letters = [k for k in areas if k in "FGHIJ"]
    if building["roofType"] == "flat":
        f, g, hh = e / 4.0 * e / 10.0, (b - e / 2.0) * e / 10.0, (e / 2.0 - e / 10.0) * b
        expected = {"F": 2.0 * f, "G": g, "H": hh, "I": d * b - 2.0 * f - g - hh}
        for letter, value in expected.items():
            check(close(areas.get(letter, 0.0), value), f"flat roof zone {letter}: {areas.get(letter, 0.0)} vs {value}")
    else:
        slope = b * d / math.cos(math.radians(building["pitchDeg"]))
        check(close(sum(areas[k] for k in roof_letters), slope), f"pitched roof zones cover the slope area {slope}")

    for diagram in data["diagrams"]:
        rows = {len(c["values"]) for c in diagram["tableTwin"]["columns"]}
        check(len(rows) == 1, f"{diagram['id']}: table twin columns differ in length")
        series = {s["data"]["id"]: s["data"] for s in diagram["series"]}
        kind = diagram["id"]
        if kind == "qp-profile":
            for q in series["qp"]["points"]:
                check(close(q["y"], qp(q["x"], wind["vb"], wind["rho"], wind["terrain"], wind["orography"]) / 1000.0, 1e-9), f"qp({q['x']})")
        if kind == "cpe-area":
            for sid, s in series.items():
                if not sid.startswith("cpe-"):
                    continue
                c1, c10 = s["points"][0]["y"], s["points"][-1]["y"]
                for q in s["points"]:
                    expected = c1 if q["x"] <= 1.0 else (c10 if q["x"] >= 10.0 else c1 - (c1 - c10) * math.log10(q["x"]))
                    check(close(q["y"], expected, 1e-9), f"{sid}: c_pe({q['x']})")
        if kind == "snow-mu":
            for q in series["mu1"]["points"]:
                check(close(q["y"], mu1(q["x"]), 1e-9), f"mu1({q['x']})")
            for q in series["mu2"]["points"]:
                check(close(q["y"], min(0.8 + 0.8 * q["x"] / 30.0, 1.6), 1e-9), f"mu2({q['x']})")
        if kind == "snow-mu3":
            for q in series["mu3"]["points"]:
                check(close(q["y"], min(0.2 + 10.0 * q["x"], 2.0), 1e-9), f"mu3({q['x']})")
        if kind == "fire-curves":
            for sid, f in (("fire-standard", standard), ("fire-external", external), ("fire-hydrocarbon", hydrocarbon)):
                for q in series[sid]["points"]:
                    check(close(q["y"], f(q["x"]), 1e-9), f"{sid}({q['x']})")
        if kind == "bridge-influence":
            span = data["bridgeSpan"]
            for q in series["influence-moment"]["points"]:
                check(close(q["y"], min(q["x"], span - q["x"]) / 2.0, 1e-9), f"influence({q['x']})")
        if kind == "silo-pressure":
            for q in series["p-h"]["points"][::8]:
                reference = janssen(q["x"], silo["gamma"], silo["a"], silo["mu"], silo["k"]) / 1000.0
                check(close(q["y"], reference, 1e-5), f"Janssen p_h({q['x']}): {q['y']} vs {reference}")
            top = series["p-h"]["points"][-1]
            closed = silo["gamma"] * silo["a"] / silo["mu"] * (1.0 - math.exp(-top["x"] * silo["mu"] * silo["k"] / silo["a"])) / 1000.0
            check(close(top["y"], closed, 1e-9), "Janssen closed form")

    print(json.dumps({"ok": not failures, "checks": counter[0], "failures": failures}, ensure_ascii=False))
    sys.exit(0 if not failures else 1)


main()
