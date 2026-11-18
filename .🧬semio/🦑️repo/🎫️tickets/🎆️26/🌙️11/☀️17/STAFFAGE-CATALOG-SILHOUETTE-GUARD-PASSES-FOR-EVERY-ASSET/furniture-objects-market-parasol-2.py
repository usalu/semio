import importlib.util, os, json, sys
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "market-parasol")
for view in ("elevation", "front"):
    src = d.obj(d.shape(50, view, "mast")); src.pop("penMm", None)
    n = d.shape(100, view, "mast"); o = d.obj(n)
    for k in ("kind", "points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "filled"): o.pop(k, None)
    o.update({k: v for k, v in src.items() if k in ("kind", "points", "closed", "x0", "y0", "x1", "y1", "r0", "r1", "filled")})
    d.replace_obj(n, o)
    b = d.shape(100, view, "base-block"); p = d.obj(b); p.update({"id": "base-plate", "x": -0.14, "y": 0.2, "w": 0.28, "h": 0.015, "r": 0.002}); p.pop("penMm", None)
    d.insert_after(b, p)
d.commit()
