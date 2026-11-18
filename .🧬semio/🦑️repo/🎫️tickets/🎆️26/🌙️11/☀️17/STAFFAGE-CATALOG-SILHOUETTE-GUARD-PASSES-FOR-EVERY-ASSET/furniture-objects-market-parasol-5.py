import importlib.util, os, json, sys
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "market-parasol")
for sc, r in ((50, 0.006), (100, 0.0075)):
    n = d.shape(sc, "front", "stretcher"); o = d.obj(n)
    for k in ("points", "closed", "solid", "x", "y", "w", "h"): o.pop(k, None)
    a = dict(o); a.update({"id": "stretcher-right", "kind": "capsule", "filled": True, "x0": 0, "y0": 1.9574, "x1": 0.4808, "y1": 2.2245, "r0": r, "r1": r, "role": "body"})
    b = dict(a); b.update({"id": "stretcher-left", "x1": -0.4808})
    d.replace_obj(n, a); d.insert_after(n, b)
d.commit()
