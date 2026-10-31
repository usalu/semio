"""Gap repair loop: probe -> bridge (merge) near-miss outlines -> probe ... Each repair moves the lower-z shape's outline near the contact point
onto/under its neighbour (an overlap of OVER mm) so the pair becomes a join instead of a gap (merging parts closer than the minimum gap)."""
import sys, json, math, collections
from io_ import *
from job import submit
import geom
OVER = 0.07
def repair_once(name, scales, limit_per_scale=400, overrides=None):
    sfile = f"{T}/🗑️generated/CVF-H2/struct/{name}.json"
    submit("probe", name, json=sfile, scales=scales, limit=3)
    viol = json.load(open(sfile))
    doc = load(name); a = doc["asset"]
    shapes = {}
    for sc in scales:
        shapes[sc] = {s["id"]: s for s in a["representations"][str(sc)]["geometry"]["elevation"]}
    stats = collections.Counter(); done = set()
    gaps = sorted([v for v in viol if v["type"] == "gap"], key=lambda v: v["gapMm"])
    for v in gaps:
        sc = v["scale"]; key = (sc, v["a"], v["b"])
        if stats[sc] >= limit_per_scale: continue
        sa, sb = shapes[sc][v["a"]], shapes[sc][v["b"]]
        mover, other = (sa, sb) if v["zA"] <= v["zB"] else (sb, sa)
        if overrides and (v["a"], v["b"]) in overrides: mover = shapes[sc][overrides[(v["a"], v["b"])]]; other = sb if mover is sa else sa
        # prefer moving open strokes and small shapes
        if mover is sa:
            p0, p1 = v["localA"], v["localAtoB_A"]
        else:
            p0, p1 = v["localB"], v["localBtoA_B"]
        dx, dy = p1["x"] - p0["x"], p1["y"] - p0["y"]; L = math.hypot(dx, dy)
        if L < 1e-9: continue
        k = sc / 1000.0
        # want centre distance -> 0 (+ overshoot): move by centreMm (+OVER)
        amount = (v["centreMm"] + OVER) * k
        rho = max(0.25 * k, amount * 1.0)
        geom.nudge(mover, (p0["x"], p0["y"]), (dx / L * amount, dy / L * amount), rho)
        stats[sc] += 1
    save(name, doc)
    counts = collections.Counter((v["scale"], v["type"]) for v in viol)
    return counts, stats
if __name__ == "__main__":
    name = sys.argv[1]; scales = [int(x) for x in sys.argv[2].split(",")]; rounds = int(sys.argv[3])
    for i in range(rounds):
        counts, stats = repair_once(name, scales)
        print(i, dict(counts), dict(stats), flush=True)
        if not any(t == "gap" for (_, t) in counts): break
