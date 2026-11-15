"""Usage: python3 author-25-simplify.py file.json tol — Douglas-Peucker simplification of the closed polygons `pts` of a shape json."""
import json, math, sys
d = json.load(open(sys.argv[1])); eps = float(sys.argv[2])
def rdp(p, a, b, out):
    (x0, y0), (x1, y1) = p[a], p[b]; l = math.hypot(x1 - x0, y1 - y0) or 1e-9
    best, bi = 0, -1
    for i in range(a + 1, b):
        dd = abs((x1 - x0) * (y0 - p[i][1]) - (x0 - p[i][0]) * (y1 - y0)) / l
        if dd > best: best, bi = dd, i
    if best > eps: rdp(p, a, bi, out); out.append(bi); rdp(p, bi, b, out)
for s in d:
    p = s["pts"] + [s["pts"][0]]; h = len(p) // 2; keep = {0, h, len(p) - 1}; o = []
    rdp(p, 0, h, o); rdp(p, h, len(p) - 1, o); keep |= set(o)
    s["pts"] = [p[i] for i in sorted(keep)][:-1]
json.dump(d, open(sys.argv[1], "w"))
print([(s["id"], len(s["pts"])) for s in d])
