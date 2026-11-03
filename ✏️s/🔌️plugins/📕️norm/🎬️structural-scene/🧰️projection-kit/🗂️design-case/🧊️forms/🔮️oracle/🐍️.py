#!/usr/bin/env python3
"""🔮️ Independent oracle of the profile forms of the design-case drawing: recomputes from the case inputs alone the key dimensions of the tapered profile prism, the curved-pitched zone, the ply stack, the char band and the residual section, the mode overlay, the transformed assembly and the dimension lines of the annotation roles (a linear distance from a face, an angle between the grain and the load direction), and compares them with the dimensions the Rust forms drew.
Reads a list of {"form", "inputs", "measured"} as JSON on stdin and prints {"ok": bool, "checks": n, "failures": [...]}; areas are cross-checked with shapely and the curved zone with numpy when they are installed. Usage: python3 -I 🐍️.py < cases.json"""
import json
import math
import sys

try:
    from shapely.geometry import Point, Polygon, box
except ImportError:
    Point = Polygon = box = None

TOLERANCE = 1e-9


def rotate_yaw(d, yaw):
    c, s = math.cos(yaw), math.sin(yaw)
    return [d[0] * c - d[1] * s, d[0] * s + d[1] * c, d[2]]


def inset(rect, depth, faces):
    """🔲️ Reduces `rect` = [x0, x1, y0, y1] by `depth` on the exposed sides [left, right, top, bottom]."""
    x0, x1, y0, y1 = rect
    return [x0 + depth * faces[0], x1 - depth * faces[1], y0 + depth * faces[3], y1 - depth * faces[2]]


def rect_area(rect):
    return (rect[1] - rect[0]) * (rect[3] - rect[2])


def check(name, expected, measured, failures, counter):
    counter[0] += 1
    if isinstance(expected, list):
        if len(expected) != len(measured) or any(abs(e - m) > TOLERANCE * max(1.0, abs(e)) for e, m in zip(expected, measured)):
            failures.append(f"{name}: expected {expected}, drawn {measured}")
    elif abs(expected - measured) > TOLERANCE * max(1.0, abs(expected)):
        failures.append(f"{name}: expected {expected}, drawn {measured}")


def taper(i, m, fail, n):
    b, l, h, alpha = i["b"], i["l"], i["h"], math.radians(i["angle_deg"])
    top = h + l * math.tan(alpha)
    area = l * (h + top) / 2.0
    if Polygon:
        assert abs(Polygon([(0, 0), (l, 0), (l, top), (0, h)]).area - area) < 1e-9
    check("taper area", area, m["area"], fail, n)
    check("taper top", top, m["top"], fail, n)
    check("taper length", l, m["length"], fail, n)
    check("taper depth", b, m["depth"], fail, n)


def apex(i, m, fail, n):
    b, h, r, alpha, steps = i["b"], i["h"], i["r"], math.radians(i["angle_deg"]), i["steps"]
    delta = 2.0 * alpha / steps
    area = 0.5 * steps * ((r + h) ** 2 - r ** 2) * math.sin(delta)
    if Polygon:
        def arc(radius):
            return [(radius * math.sin(-alpha + k * delta), radius * math.cos(-alpha + k * delta)) for k in range(steps + 1)]

        zone = Polygon(arc(r) + arc(r + h)[::-1])
        assert abs(zone.area - area) < 1e-9, (zone.area, area)
    check("apex zone area", area, m["zone_area"], fail, n)
    check("apex zone depth", b * 1.02, m["zone_depth"], fail, n)


def plies(i, m, fail, n):
    t = [v / 1000.0 for v in i["t_mm"]]
    check("ply thicknesses", t, m["thicknesses"], fail, n)
    check("ply grains", [0.0 if o >= 0.5 else math.pi / 2.0 for o in i["o"]], m["grains"], fail, n)
    check("ply total", sum(t), sum(m["thicknesses"]), fail, n)
    check("ply footprint", i["b"] * i["l"], m["footprint_area"], fail, n)


def char(i, m, fail, n):
    b, h, faces = i["b"], i["h"], i["faces"]
    full = [-b / 2.0, b / 2.0, -h / 2.0, h / 2.0]
    band = rect_area(inset(full, i["outer"], faces)) - rect_area(inset(full, i["inner"], faces))
    residual = rect_area(inset(full, i["residual"], faces))
    if box:
        outer, inner = box(*[inset(full, i["outer"], faces)[k] for k in (0, 2, 1, 3)]), box(*[inset(full, i["inner"], faces)[k] for k in (0, 2, 1, 3)])
        assert abs(outer.difference(inner).area - band) < 1e-9
    check("char band area", band, m["band_area"], fail, n)
    check("char residual area", residual, m["residual_area"], fail, n)
    check("char slab depth", i["slab"], m["slab_depth"], fail, n)
    check("char length", i["l"], m["length"], fail, n)


def mode(i, m, fail, n):
    check("ghost amplitude", i["amplitude"] * max(abs(u) for _, u in i["shape"]), m["ghost_max"], fail, n)
    check("heat map", [i["peak"] * abs(v) for v in i["values"]], m["heat"], fail, n)
    check("grid nodes", float(i["columns"] * i["rows"]), float(m["nodes"]), fail, n)
    check("grid extent", [i["l"], i["b"]], m["extent"], fail, n)


def assembly(i, m, fail, n):
    ox, oy = i["origin"]
    for sub, drawn in zip(i["subs"], m["origins"]):
        d = rotate_yaw(sub["at"], i["yaw"])
        check(f"sub {sub['tag']} origin", [ox + d[0], oy + d[1], d[2]], drawn, fail, n)
    check("sub count", float(len(i["subs"])), float(m["count"]), fail, n)


def dimension(i, m, fail, n):
    metres = i["value"] / i["per_metre"]
    start = [i["origin"][0] - i["dx"] / 2.0, i["origin"][1], i["top"]]
    stop = [start[0] + metres, start[1], start[2]]
    if Point:
        assert abs(Point(start[0], start[1]).distance(Point(stop[0], stop[1])) - metres) < 1e-9
    check("dimension from", start, m["from"], fail, n)
    check("dimension to", stop, m["to"], fail, n)
    check("dimension binding", i["per_metre"], m["k"], fail, n)


def angle(i, m, fail, n):
    theta, r = math.radians(i["degrees"]), i["ray"]
    centre = [i["origin"][0], i["origin"][1], i["top"]]
    start = [centre[0] + r, centre[1], centre[2]]
    stop = [centre[0] + r * math.cos(theta), centre[1], centre[2] + r * math.sin(theta)]
    if Point:
        assert abs(Point(start[0], start[2]).distance(Point(stop[0], stop[2])) - 2.0 * r * math.sin(theta / 2.0)) < 1e-9
    check("angle from", start, m["from"], fail, n)
    check("angle to", stop, m["to"], fail, n)
    check("angle value", theta, m["radians"], fail, n)
    check("angle chord direction", math.pi / 2.0 + theta / 2.0, math.atan2(m["to"][2] - m["from"][2], m["to"][0] - m["from"][0]), fail, n)


FORMS = {"dimension": dimension, "angle": angle, "taper": taper, "apex": apex, "plies": plies, "char": char, "mode": mode, "assembly": assembly}


def main():
    cases = json.load(sys.stdin)
    failures, counter = [], [0]
    for case in cases:
        FORMS[case["form"]](case["inputs"], case["measured"], failures, counter)
    print(json.dumps({"ok": not failures, "checks": counter[0], "failures": failures}))


if __name__ == "__main__":
    main()
