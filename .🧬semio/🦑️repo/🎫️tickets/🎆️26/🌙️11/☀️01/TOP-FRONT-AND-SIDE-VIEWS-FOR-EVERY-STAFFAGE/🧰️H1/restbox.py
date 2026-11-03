import sys
from lib import Ctx
aid, sc = sys.argv[1], sys.argv[2]
c = Ctx(aid)
for s in c.reps[sc]["geometry"]["elevation"]:
    x0, x1, y0, y1 = c.restbox(s)
    print(f"{s['id']:<22} {s['bone']:<12} {s['role']:<6} z{s['z']:<6} x[{x0:.3f},{x1:.3f}] y[{y0:.3f},{y1:.3f}] {s.get('region') or s.get('regions') or ''}")
