"""Dense, kinematically exact excavator clip keys: every key carries joint angles and the cylinder values solved by exc.solve (pins meet at every key)."""
import json, math, re, sys
from exc import *
SRC = "src/excavator.json"
a = json.load(open(SRC))
bucket = [s for s in a["geometry"]["elevation"] if s["bone"] == "bucket" and s["kind"] == "path" and s["id"] in ("bucket-body", "bucket-teeth")]
def pts(d):
    nums = [float(x) for x in re.findall(r"-?\d*\.?\d+", d)]
    return list(zip(nums[0::2], nums[1::2]))
BP = [p for s in bucket for p in pts(s["d"])]
def miny(db, ds, dk):
    ab, asx, ak, P1, P2, tip = fk(db, ds, dk)
    return min(P2[1] + rot(ak, x, y)[1] for x, y in BP)
def hermite(ks, t, period=None):
    n = len(ks)
    def at(i):
        if period: return ks[i % n]
        return ks[min(max(i, 0), n - 1)]
    for i in range(n if period else n - 1):
        t0 = ks[i][0]; t1 = ks[(i + 1) % n][0] + (period if period and i + 1 >= n else 0)
        if t0 <= t <= t1 or (period and i == n - 1):
            tt = (t - t0) / (t1 - t0)
            def tangent(j, other_dt):
                pj, pn, pp = at(j), at(j + 1), at(j - 1)
                tn = pn[0] + (period if period and j + 1 >= n else 0); tp = pp[0] - (period if period and j - 1 < 0 else 0)
                return [(pn[c] - pp[c]) / (tn - tp) for c in (1, 2, 3)]
            m0 = tangent(i, 0); m1 = tangent(i + 1, 0)
            h = t1 - t0; res = []
            for c in range(3):
                p0 = ks[i][c + 1]; p1 = ks[(i + 1) % n][c + 1]
                h00 = 2 * tt ** 3 - 3 * tt ** 2 + 1; h10 = tt ** 3 - 2 * tt ** 2 + tt; h01 = -2 * tt ** 3 + 3 * tt ** 2; h11 = tt ** 3 - tt ** 2
                res.append(h00 * p0 + h10 * h * m0[c] + h01 * p1 + h11 * h * m1[c])
            return res
    return list(ks[-1][1:])
def lift(db, ds, dk):
    while miny(db, ds, dk) < 0.004 and db < 45: db += 0.25
    return db, ds, dk
def bones_for(db, ds, dk):
    r = solve(db, ds, dk)
    return {"boom": {"angleDeg": round(db, 3)}, "stick": {"angleDeg": round(ds, 3)}, "bucket": {"angleDeg": round(dk, 3)},
            "boom-cyl": {"angleDeg": round(r["barrel"][0] - 30, 3)}, "boom-rod": {"dx": round(r["d"][0] - 2.1, 4)},
            "stick-cyl": {"angleDeg": round(r["barrel"][1] + 33, 3)}, "stick-rod": {"dx": round(r["d"][1] - 2.85, 4)},
            "bucket-cyl": {"angleDeg": round(r["barrel"][2] + 4, 3)}, "bucket-rod": {"dx": round(r["d"][2] - 1.25, 4)}}
dig = [(0, -18, -8, 45), (1.5, -24, -26, 55), (3.5, -16, -42, -40), (5, 5, -32, -45), (6.5, 12, -12, -45), (7.5, 12, -12, 45)]
dig = [(t, *lift(db, ds, dk)) for t, db, ds, dk in dig]
dump = [(0, -24, -38, 60), (2, 12, -12, -45), (3.2, 12, -12, 45), (6.5, -25, -50, -85), (9, -25, -50, -85)]
worst = 0
def mk(knots, times, period, pose_end=None):
    out = []
    for t in times:
        db, ds, dk = hermite(knots, t, period)
        db, ds, dk = lift(db, ds, dk)
        out.append({"t": round(t, 3), "pose": "digging", "ease": "smooth", "bones": bones_for(db, ds, dk)})
    return out
clips = {c["id"]: c for c in a["clips"]}
c = clips["dig-cycle"]; c["keys"] = mk(dig, [0.5 * i for i in range(18)], 9)
d = clips["dump-and-stow"]
d["keys"] = mk(dump, [0.4 * i for i in range(0, 17)], None) + [d["keys"][-2], d["keys"][-1]]
d["keys"] = [k for k in d["keys"] if k["t"] < 6.5 or k["pose"] == "stowed"]
d["keys"] = sorted({k["t"]: k for k in d["keys"]}.values(), key=lambda k: k["t"])
for k in d["keys"]:
    if k["pose"] == "stowed": k.pop("bones", None)
json.dump(a, open(SRC, "w"), ensure_ascii=False, indent=2)
print(len(c["keys"]), len(d["keys"]), [k["t"] for k in d["keys"]])
