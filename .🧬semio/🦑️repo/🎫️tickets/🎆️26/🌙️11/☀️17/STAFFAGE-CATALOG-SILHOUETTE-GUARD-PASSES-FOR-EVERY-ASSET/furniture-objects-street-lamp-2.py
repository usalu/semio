import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "street-lamp")
d.set(d.solid(50, "pole-cap"), "profile", [[5.86, 0.0], [5.86, 0.042], [5.8661, 0.0415], [5.8721, 0.0385], [5.877, 0.032], [5.8794, 0.02], [5.88, 0.004], [5.88, 0.0]])
d.commit()
def poly(o, pts):
    for k in ("d", "x0", "y0", "x1", "y1", "r0", "r1", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry"): o.pop(k, None)
    o.update({"kind": "poly", "points": pts, "closed": True, "filled": True}); return o
d = l.doc("🪑️objects", "street-lamp")
plan = [("head-housing", "elevation", "head"), ("lens", "elevation", "head"), ("pole-cap", "elevation", "pole")]
for sid, view, bone in plan:
    h = l.hulls("street-lamp", 50, sid, "elevation", bone)
    n = d.shape(50, view, sid); d.replace_obj(n, poly(d.obj(n), h[(sid, "elevation")]))
for sid, view in (("head-housing", "front"), ("head-housing", "plan"), ("lens", "front"), ("pole-cap", "front")):
    h = l.hulls("street-lamp", 50, sid, view)
    n = d.shape(50, view, sid); d.replace_obj(n, poly(d.obj(n), h[(sid, view)]))
d.commit()
