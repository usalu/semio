import sys, json, math, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
G = f"{T}/🗑️generated/CVF-V"
# quick near-miss finder on the rest pose: usage qc.py id scale [thr=0.15]   (no hidden-line removal => conservative; counts only pairs that never touch)
def segd(p, q, r, s):
    d1 = (q[0]-p[0], q[1]-p[1]); d2 = (s[0]-r[0], s[1]-r[1]); den = d1[0]*d2[1]-d1[1]*d2[0]
    if abs(den) > 1e-14:
        t = ((r[0]-p[0])*d2[1]-(r[1]-p[1])*d2[0])/den; u = ((r[0]-p[0])*d1[1]-(r[1]-p[1])*d1[0])/den
        if 0 <= t <= 1 and 0 <= u <= 1: return 0.0, ((p[0]+t*d1[0]), (p[1]+t*d1[1]))
    best = (1e18, p)
    for x, a, b in ((p, r, s), (q, r, s), (r, p, q), (s, p, q)):
        dx, dy = b[0]-a[0], b[1]-a[1]; l2 = dx*dx+dy*dy; t = 0 if l2 == 0 else max(0, min(1, ((x[0]-a[0])*dx+(x[1]-a[1])*dy)/l2))
        c = (a[0]+t*dx, a[1]+t*dy); d = math.hypot(x[0]-c[0], x[1]-c[1])
        if d < best[0]: best = (d, x)
    return best
def load(id, k):
    a = json.load(open(f"{G}/work/{id}.json"))["asset"]; R = Rig(a); rep = a["representations"][k]; pens = rep["pens"]; out = []
    for s in rep["geometry"]["elevation"]:
        pen = s.get("penMm") or pens.get(s["role"])
        if pen is None or s["role"] == "ground": continue
        pts = flatten(shape_world_d(R, s), 6); closed = s["kind"] != "path" or s["d"].rstrip().endswith("Z") or s["filled"]
        out.append((s["id"], s, pts, closed, pen))
    return a, out
def run(id, k, thr=0.15):
    a, sh = load(id, k); den = float(k); m = 1000/den; res = []
    boxes = []
    for sid, s, pts, closed, pen in sh:
        P = [(x*m, -y*m) for x, y in pts]
        xs = [p[0] for p in P]; ys = [p[1] for p in P]; boxes.append((sid, s, P, closed, pen, min(xs), max(xs), min(ys), max(ys)))
    for i in range(len(boxes)):
        for j in range(i+1, len(boxes)):
            A, B = boxes[i], boxes[j]
            reach = thr + (A[4]+B[4])/2 + 0.01
            if A[5] > B[6]+reach or B[5] > A[6]+reach or A[7] > B[8]+reach or B[7] > A[8]+reach: continue
            if A[1].get("region") and A[1].get("region") == B[1].get("region") and A[1]["bone"] != B[1]["bone"]: pass
            best = 1e18; at = None
            ea = list(zip(A[2], A[2][1:] + ([A[2][0]] if A[3] else []))); eb = list(zip(B[2], B[2][1:] + ([B[2][0]] if B[3] else [])))
            for p, q in ea:
                for r, s2 in eb:
                    if max(p[0], q[0]) < min(r[0], s2[0]) - reach or min(p[0], q[0]) > max(r[0], s2[0]) + reach or max(p[1], q[1]) < min(r[1], s2[1]) - reach or min(p[1], q[1]) > max(r[1], s2[1]) + reach: continue
                    d, c = segd(p, q, r, s2)
                    if d < best: best = d; at = c
                    if best <= 1e-4: break
                if best <= 1e-4: break
            if best <= 1e-4: continue
            ink = best - (A[4]+B[4])/2
            if 0 < ink < thr: res.append((ink, A[0], B[0], at))
    res.sort()
    return res
if __name__ == "__main__":
    id, k = sys.argv[1], sys.argv[2]; thr = float(sys.argv[3]) if len(sys.argv) > 3 else 0.15
    r = run(id, k, thr)
    print(id, k, len(r), "near-miss pairs")
    for ink, a, b, at in r[:30]: print(f"  {a} + {b}  ink {ink:.3f}  @({at[0]:.2f},{at[1]:.2f})mm")
