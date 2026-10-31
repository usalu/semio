"""CVF-VMC scratch: nearest visible outline points of two shapes. usage: pair.py asset.json den pose idA idB  (prints paper mm and model metres)"""
import json, math, sys
import prefilter as P
f, den, pid, ia, ib = sys.argv[1], int(sys.argv[2]), sys.argv[3], sys.argv[4], sys.argv[5]
a = json.load(open(f))["asset"]
rep = a["representations"][str(den)]
pose = [p for p in a["poses"] if p["id"] == pid][0]
items = P.prepare(a, rep, pose, den)
for it in items:
    it["box"] = P.box(it["rings"])
by = {i["id"]: i for i in items}
def runs(it):
    hid = [q for q in items if q["occ"] and q is not it and (q["order"] > it["order"] or P.joined(it, q))]
    return P.sample_runs(it, hid)
A, B = runs(by[ia]), runs(by[ib])
best = min(((math.hypot(p[0] - q[0], p[1] - q[1]), p, q) for p in A for q in B), key=lambda t: t[0])
k = 1000.0 / den
print("d=%.3f mm ink=%.3f" % (best[0], best[0] - (by[ia]["pen"] + by[ib]["pen"]) / 2))
for lab, p in (("A", best[1]), ("B", best[2])):
    print(lab, "paper %.2f,%.2f  model %.3f,%.3f" % (p[0], p[1], p[0] / k, -p[1] / k))
