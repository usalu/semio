import sys, math; sys.path.insert(0, ".")
from silhouette_edit import *
from silhouette_rig import *
p = find("bamboo-clump")
raw, doc, idx = load(p)
a = doc["asset"]
fr = rest_frames(a["rig"]["bones"])
def hull(pts):
    pts = sorted(set(pts))
    def cross(o, a, b): return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    lo = []
    for q in pts:
        while len(lo) >= 2 and cross(lo[-2], lo[-1], q) <= 0: lo.pop()
        lo.append(q)
    up = []
    for q in reversed(pts):
        while len(up) >= 2 and cross(up[-2], up[-1], q) <= 0: up.pop()
        up.append(q)
    return lo[:-1] + up[:-1]
new = {}
for s in a["volume"]["solids"]:
    if s["kind"] != "ellipsoid" or not s["id"].startswith("mass-"): continue
    q = s["transform"]["rotation"]; t = s["transform"]["translation"]
    th = 2 * math.atan2(q[2], q[3])
    ra = s["radii"]
    pts = []
    for i in range(0, 25):
        for j in range(0, 72):
            ph = math.pi * i / 24; la = 2 * math.pi * j / 72
            lp = (ra[0] * math.sin(ph) * math.cos(la), ra[1] * math.sin(ph) * math.sin(la), ra[2] * math.cos(ph))
            b = (lp[0] * math.cos(th) - lp[1] * math.sin(th) + t[0], lp[0] * math.sin(th) + lp[1] * math.cos(th) + t[1], lp[2] + t[2])
            w = to_world(fr[s["bone"]], b)
            pts.append((round(w[2], 4), round(w[1], 4)))
    h = hull(pts)
    step = max(1, len(h) // 40)
    new[s["id"]] = [[x, y] for x, y in h[::step]]
ops = []
for sc in sys.argv[1:] or ["50"]:
    for k, sh in enumerate(a["representations"][sc]["geometry"]["front"]):
        if sh["id"] in new:
            n = dict(sh); n["points"] = new[sh["id"]]
            ops.append(("replace", ("asset", "representations", sc, "geometry", "front", k), n))
print(len(ops), apply(p, ops, expect=raw))
