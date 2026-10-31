import sys, json, math, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
from qc import segd
G = f"{T}/🗑️generated/CVF-V"
OCC = ("body", "detail", "texture")
def _dseg(p, a, b):
    dx, dy = b[0]-a[0], b[1]-a[1]; l2 = dx*dx+dy*dy; t = 0 if l2 == 0 else max(0, min(1, ((p[0]-a[0])*dx+(p[1]-a[1])*dy)/l2))
    return math.hypot(p[0]-a[0]-t*dx, p[1]-a[1]-t*dy)
def inside(pt, rings):
    c = False
    for ring in rings:
        n = len(ring); j = n - 1
        for i in range(n):
            a, b = ring[i], ring[j]
            if _dseg(pt, a, b) <= 1e-4: return False
            if (a[1] > pt[1]) != (b[1] > pt[1]) and pt[0] < a[0] + (pt[1]-a[1])/(b[1]-a[1])*(b[0]-a[0]): c = not c
            j = i
    return c
def inter_t(p, q, r, s):
    d1 = (q[0]-p[0], q[1]-p[1]); d2 = (s[0]-r[0], s[1]-r[1]); den = d1[0]*d2[1]-d1[1]*d2[0]
    if abs(den) < 1e-14: return None
    t = ((r[0]-p[0])*d2[1]-(r[1]-p[1])*d2[0])/den; u = ((r[0]-p[0])*d1[1]-(r[1]-p[1])*d1[0])/den
    return t if 0 < t < 1 and 0 <= u <= 1 else None
def prep(a, k, pose_bones=None):
    R = Rig(a); rep = a["representations"][k]; m = 1000/float(k); out = []
    for idx, s in enumerate(rep["geometry"]["elevation"]):
        pen = s.get("penMm") or rep["pens"].get(s["role"])
        if pen is None or s["role"] == "ground": continue
        pts = flatten(shape_world_d(R, s), 5)
        # drop consecutive duplicates
        P = []
        for x, y in pts:
            q = (x*m, -y*m)
            if not P or math.hypot(q[0]-P[-1][0], q[1]-P[-1][1]) > 0.01: P.append(q)
        closed = s["kind"] != "path" or s["d"].rstrip().endswith("Z")
        xs = [p[0] for p in P]; ys = [p[1] for p in P]
        out.append(dict(s=s, P=P, closed=closed, pen=pen, idx=idx, box=(min(xs), max(xs), min(ys), max(ys)), occ=closed and s["filled"] and s["role"] in OCC))
    out.sort(key=lambda p: (p["s"]["z"], p["idx"]))
    for o, p in enumerate(out): p["order"] = o
    return R, out
def joined(a, b, parents):
    sa, sb = a["s"], b["s"]
    return sa.get("region") is not None and sa.get("region") == sb.get("region") and (sa["bone"] == sb["bone"] or parents.get(sa["bone"]) == sb["bone"] or parents.get(sb["bone"]) == sa["bone"])
def runs(p, items, parents):
    hid = [q for q in items if q["occ"] and q is not p and (q["order"] > p["order"] or joined(p, q, parents)) and not (q["box"][0] > p["box"][1] or q["box"][1] < p["box"][0] or q["box"][2] > p["box"][3] or q["box"][3] < p["box"][2])]
    P = p["P"]; n = len(P); edges = [(P[i], P[(i+1) % n]) for i in range(n if p["closed"] else n-1)]
    res = []; cur = []
    for a, b in edges:
        ts = [0.0, 1.0]
        for q in hid:
            Q = q["P"]; m = len(Q)
            if max(a[0], b[0]) < q["box"][0] or min(a[0], b[0]) > q["box"][1] or max(a[1], b[1]) < q["box"][2] or min(a[1], b[1]) > q["box"][3]: continue
            for i in range(m):
                t = inter_t(a, b, Q[i], Q[(i+1) % m])
                if t is not None: ts.append(t)
        ts = sorted(set(ts))
        for t0, t1 in zip(ts, ts[1:]):
            if t1 - t0 < 1e-9: continue
            mid = (a[0] + (b[0]-a[0])*(t0+t1)/2, a[1] + (b[1]-a[1])*(t0+t1)/2)
            vis = not any(inside(mid, [q["P"]]) for q in hid if q["box"][0] <= mid[0] <= q["box"][1] and q["box"][2] <= mid[1] <= q["box"][3])
            p0 = (a[0] + (b[0]-a[0])*t0, a[1] + (b[1]-a[1])*t0); p1 = (a[0] + (b[0]-a[0])*t1, a[1] + (b[1]-a[1])*t1)
            if vis:
                if not cur: cur = [p0]
                cur.append(p1)
            elif cur: res.append(cur); cur = []
    if cur: res.append(cur)
    return res
def gaps(a, k, thr=0.15, dbg=False):
    R, items = prep(a, k); parents = {b["id"]: b["parent"] for b in a["rig"]["bones"]}
    vis = []
    for p in items:
        for r in runs(p, items, parents): vis.append((p["s"]["id"], r, p["pen"]))
    out = {}
    B = []
    for sid, r, pen in vis:
        xs = [q[0] for q in r]; ys = [q[1] for q in r]; B.append((sid, r, pen, min(xs), max(xs), min(ys), max(ys)))
    for i in range(len(B)):
        for j in range(i+1, len(B)):
            A, C = B[i], B[j]
            if A[0] == C[0]: continue
            reach = thr + (A[2]+C[2])/2 + 0.01
            if A[3] > C[4]+reach or C[3] > A[4]+reach or A[5] > C[6]+reach or C[5] > A[6]+reach: continue
            best = 1e18; at = None
            for p, q in zip(A[1], A[1][1:]):
                for r2, s in zip(C[1], C[1][1:]):
                    if max(p[0], q[0]) < min(r2[0], s[0]) - reach or min(p[0], q[0]) > max(r2[0], s[0]) + reach or max(p[1], q[1]) < min(r2[1], s[1]) - reach or min(p[1], q[1]) > max(r2[1], s[1]) + reach: continue
                    d, c = segd(p, q, r2, s)
                    if d < best: best = d; at = c
                    if best <= 1e-4: break
                if best <= 1e-4: break
            if best <= 1e-4: continue
            ink = best - (A[2]+C[2])/2
            if 0 < ink < thr:
                key = tuple(sorted((A[0], C[0])))
                if key not in out or ink < out[key][0]: out[key] = (ink, at)
    return sorted((v[0], k_[0], k_[1], v[1]) for k_, v in out.items())
import copy
def posed(a, pose):
    b = copy.deepcopy(a)
    for bone in b["rig"]["bones"]:
        o = pose.get("bones", {}).get(bone["id"])
        if o:
            off = o.get("angleDeg", 0); j = bone.get("joint") or {}
            if "minDeg" in j: off = max(j["minDeg"], min(j["maxDeg"], off))
            bone["angleDeg"] = bone["angleDeg"] + off
            bone["x"] = bone["x"] + o.get("dx", 0); bone["y"] = bone["y"] + o.get("dy", 0)
    return b
def allposes(a, k, thr):
    out = {}
    for p in a["poses"]:
        if p.get("key"): continue
        for ink, x, y, at in gaps(posed(a, p), k, thr):
            key = (x, y)
            if key not in out or ink < out[key][0]: out[key] = (ink, p["id"], at)
    return sorted((v[0], k_[0], k_[1], v[1], v[2]) for k_, v in out.items())
if __name__ == "__main__":
    args = sys.argv[1:]; thr = 0.15
    if "-t" in args: i = args.index("-t"); thr = float(args[i+1]); del args[i:i+2]
    id = args[0]; ks = args[1:] or ["500", "200", "100", "50"]
    a = json.load(open(f"{G}/work/{id}.json"))["asset"]
    for k in ks:
        r = allposes(a, k, thr)
        print(id, k, len(r), f"gap pairs <{thr}")
        for ink, x, y, pose, at in r[:25]: print(f"  {x} + {y}  ink {ink:.3f} [{pose}] @({at[0]:.2f},{at[1]:.2f})mm")
