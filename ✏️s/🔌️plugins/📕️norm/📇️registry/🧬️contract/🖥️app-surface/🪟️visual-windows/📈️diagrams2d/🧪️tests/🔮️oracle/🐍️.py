import json
import sys

import numpy as np
from shapely.geometry import LineString, Point, Polygon


def distance(shape, p):
    pt = Point(p)
    kind = shape["kind"]
    if kind == "path":
        return LineString(shape["points"]).distance(pt)
    if kind == "area":
        poly = Polygon(shape["points"])
        return 0.0 if poly.contains(pt) else poly.exterior.distance(pt)
    if kind == "point":
        return max(Point(shape["at"]).distance(pt) - shape["radius"], 0.0)
    lo, hi = shape["min"], shape["max"]
    dx = max(lo[0] - p[0], p[0] - hi[0], 0.0)
    dy = max(lo[1] - p[1], p[1] - hi[1], 0.0)
    return float(np.hypot(dx, dy))


def main():
    c = json.load(sys.stdin)
    out = {
        "distances": [distance(s, p) for s, p in c["queries"]],
        "log": [float(v) for v in np.logspace(c["log"][0], c["log"][1], c["log"][1] - c["log"][0] + 1)],
        "linear": [float(v) for v in np.arange(c["linear"][0], c["linear"][1] + c["linear"][2] / 2, c["linear"][2])],
    }
    json.dump(out, sys.stdout)


main()
