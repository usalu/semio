import sys, os, math
sys.path.insert(0, os.path.dirname(__file__))
from lib import load
from rest import fk

doc = load(sys.argv[1])
rest = fk(doc)


def wb(s):
    b = s["bone"]
    (ox, oy), a, _ = rest[b]
    c, sn = math.cos(math.radians(a)), math.sin(math.radians(a))
    pts = []
    k = s["kind"]
    if k == "rect":
        pts = [(s["x"], s["y"]), (s["x"] + s["w"], s["y"]), (s["x"] + s["w"], s["y"] + s["h"]), (s["x"], s["y"] + s["h"])]
    elif k == "poly":
        pts = [tuple(p) for p in s["points"]]
    elif k == "ellipse":
        pts = [(s["cx"] - s["rx"], s["cy"] - s["ry"]), (s["cx"] + s["rx"], s["cy"] + s["ry"])]
    elif k == "capsule":
        pts = [(s["x0"] - s["r0"], s["y0"] - s["r0"]), (s["x1"] + s["r1"], s["y1"] + s["r1"])]
    elif k == "path":
        import re
        nums = [float(v) for v in re.findall(r"-?\d+\.?\d*", s["d"])]
        pts = list(zip(nums[0::2], nums[1::2]))
    w = [(ox + c * x - sn * y, oy + sn * x + c * y) for x, y in pts]
    xs, ys = [p[0] for p in w], [p[1] for p in w]
    return f"x[{min(xs):.2f},{max(xs):.2f}] y[{min(ys):.2f},{max(ys):.2f}]"


for sc, r in doc["asset"]["representations"].items():
    if r["status"] != "authored":
        continue
    print("== 1:" + sc)
    for s in r["geometry"]["elevation"]:
        if sys.argv[2:] and s["id"].split("-")[0] in ("bolt", "lugs"):
            continue
        print(f"  {s['id']:18s} {s['bone']:12s} {s['kind'][:4]} z{s['z']} t{s['tone']} {s.get('region') or s.get('regions')} {s.get('material','')} {wb(s)}")
