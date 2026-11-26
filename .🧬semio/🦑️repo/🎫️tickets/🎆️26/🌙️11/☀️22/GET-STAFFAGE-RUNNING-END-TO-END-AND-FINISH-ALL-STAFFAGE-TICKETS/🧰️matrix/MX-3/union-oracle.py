"""🧭️ Oracle of the polygon union: shapely `unary_union` of the bodies of every dumped loft, its area against the area of the engine's outline: union-oracle.py <dump json>"""
import json, sys
from shapely.geometry import Polygon
from shapely.ops import unary_union
bad = 0
cases = json.load(open(sys.argv[1]))
for c in cases:
    u = unary_union([Polygon(b).buffer(0) for b in c["bodies"]])
    want = u.area
    got = abs(c["engineArea"])
    if "engineBox" in c:
        b = u.bounds
        d = max(abs(b[i] - c["engineBox"][i]) for i in range(4))
        if d > 1e-6: print(f'BOX 1:{c["scale"]} {c["view"]} {c["id"]}: engine {c["engineBox"]} shapely {list(b)} ({d*1000:.0f} mm)')
    if abs(want - got) > 1e-9 + 1e-6 * want:
        bad += 1
        print(f'1:{c["scale"]} {c["view"]} {c["bone"]}/{c["id"]}: {len(c["bodies"])} bodies, engine {got:.6f} m2, shapely {want:.6f} m2')
print("bad", bad, "of", len(cases))
