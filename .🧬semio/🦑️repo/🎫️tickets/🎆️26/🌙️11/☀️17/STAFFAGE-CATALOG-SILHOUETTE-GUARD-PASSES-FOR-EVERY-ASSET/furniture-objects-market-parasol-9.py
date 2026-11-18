import importlib.util, os, json, sys
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "market-parasol")
ux, uy, vx, vy = 0.9646, -0.2635, 0.2635, 0.9646
import math
th = math.atan2(0.326, 1.19366)
ux, uy, vx, vy = math.cos(math.atan2(-0.2635, 0.9646)), math.sin(math.atan2(-0.2635, 0.9646)), -math.sin(math.atan2(-0.2635, 0.9646)), math.cos(math.atan2(-0.2635, 0.9646))
for sc in (50, 100):
    prof = d.obj(d.solid(sc, "canopy"))["profile"]
    up = []
    for a, r in prof:
        up.append((a, r))
        if a == -0.32611 and abs(r - 1.19366) < 1e-6: break
    pts = [[round(r * ux + a * uy, 5), round(r * vx + a * vy, 5)] for a, r in up]
    n = d.shape(sc, "front", "canopy"); o = d.obj(n)
    o.update({"bone": "rib-right", "kind": "poly", "points": pts, "closed": True, "solid": {"kind": "revolved", "axisBone": "mast", "axis": "u", "axisX": 0, "mirror": True}})
    d.replace_obj(n, o)
d.commit()
