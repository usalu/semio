"""🕸️ Writes the repair ops of `used-climbing-net-pyramid` (ops/used-climbing-net-pyramid.json) from the document of its generator before this repair: every rope is drawn to the round ends of its body.

A rope is a slender capsule of 16 mm between two nodes and was drawn as a bar between the nodes, without the 8 mm caps of the body. The guard widens a slender solid to the drawn strip it follows when its centre line, which runs to the tips of the caps, lies within half the width of that strip on average: the centre line of a rope ended 7.5 mm past the drawn bar at both ends, just past the limit for most ropes, so they were compared at the guard's own estimate of their width (14.3 to 16.8 mm) and at 1:500 fell between the cells where the drawn ropes did not. The bars now run 8 mm past the nodes at both ends like the body, in the three views of every scale."""
import math, os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, write

SLUG, CAP = "used-climbing-net-pyramid", 0.008
a = asset(SLUG)
ops = []
r6 = lambda v: round(v, 6)
for scale in ("50", "100", "200", "500"):
    for view, shapes in a["representations"][scale]["geometry"].items():
        for s in shapes:
            if s.get("kind") != "member" or s["section"] != {"kind": "circle", "d": 2 * CAP}:
                continue
            p, q = s["from"], s["to"]
            d = [q[k] - p[k] for k in "xyz"]
            n = math.sqrt(sum(c * c for c in d))
            u = dict(zip("xyz", (c / n for c in d)))
            ops.append({"op": "shape-set", "scale": scale, "view": view, "id": s["id"], "set": {"from": {**p, **{k: r6(p[k] - CAP * u[k]) for k in "xyz"}}, "to": {**q, **{k: r6(q[k] + CAP * u[k]) for k in "xyz"}}}})
write(SLUG, ops)
print(f"{SLUG}: {len(ops)} ops")
