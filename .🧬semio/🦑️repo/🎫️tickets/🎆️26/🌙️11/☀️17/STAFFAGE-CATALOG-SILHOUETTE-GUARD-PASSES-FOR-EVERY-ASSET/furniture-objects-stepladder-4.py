import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "stepladder")
H = l.hulls("stepladder", 500, "")
def poly(o, pts):
    for k in ("x0", "y0", "x1", "y1", "r0", "r1", "x", "y", "w", "h", "r", "penMm", "cx", "cy", "rx", "ry"): o.pop(k, None)
    o.update({"kind": "poly", "points": pts, "closed": True, "filled": True}); return o
for sid in ("front-rail-l", "front-rail-r", "rear-rail-l", "rear-rail-r"):
    for view in ("front", "plan"):
        n = d.shape(500, view, sid); d.replace_obj(n, poly(d.obj(n), H[(sid, view)]))
for i in range(4):
    n = d.shape(500, "front", "tread-%d" % i); d.replace_obj(n, poly(d.obj(n), H[("tread-%d" % i, "front")]))
n = d.shape(500, "front", "platform"); d.replace_obj(n, poly(d.obj(n), H[("platform", "front")]))
for side in ("l", "r"):
    n = d.shape(500, "front", "foot-front-" + side); o = poly(d.obj(n), H[("foot-front-" + side, "front")]); d.replace_obj(n, o)
    c = dict(o); c["id"] = "foot-rear-" + side; c["points"] = H[("foot-rear-" + side, "front")]; d.insert_after(n, c)
tp = []
for i in range(4): tp += H[("tread-%d" % i, "plan")]
n = d.shape(500, "plan", "treads"); d.replace_obj(n, poly(d.obj(n), l.hull_of(tp)))
n = d.shape(500, "plan", "platform"); d.replace_obj(n, poly(d.obj(n), H[("platform", "plan")]))
d.commit()
