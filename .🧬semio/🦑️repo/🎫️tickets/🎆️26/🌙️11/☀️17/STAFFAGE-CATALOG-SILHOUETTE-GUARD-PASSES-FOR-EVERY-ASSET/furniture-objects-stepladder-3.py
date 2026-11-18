import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "stepladder")
for sid, x, w, y in (("spreader-front-l", -0.265, 0.207, 0.168), ("spreader-front-r", -0.265, 0.207, -0.192), ("spreader-rear-l", -0.123, 0.207, 0.138), ("spreader-rear-r", -0.123, 0.207, -0.162)):
    n = d.shape(50, "plan", sid); o = d.obj(n)
    for k in ("x0", "y0", "x1", "y1", "r0", "r1"): o.pop(k, None)
    o.update({"kind": "rect", "x": x, "y": y, "w": w, "h": 0.024})
    d.replace_obj(n, o)
d.commit()
