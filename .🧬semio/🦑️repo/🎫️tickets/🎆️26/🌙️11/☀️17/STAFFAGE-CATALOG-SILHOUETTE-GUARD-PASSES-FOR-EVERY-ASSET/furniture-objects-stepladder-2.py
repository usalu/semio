import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "stepladder")
for side, y in (("l", 0.134), ("r", -0.196)):
    n = d.shape(50, "plan", "spreader-front-" + side); o = d.obj(n)
    for k in ("x0", "y0", "x1", "y1", "r0", "r1"): o.pop(k, None)
    o.update({"id": "spreader-knuckle-" + side, "kind": "rect", "x": -0.106, "y": y, "w": 0.032, "h": 0.062})
    d.insert_after(n, o)
d.at("asset", "dimensions")
d.set(d.at("asset", "dimensions"), "depthM", 0.47)
d.commit()
