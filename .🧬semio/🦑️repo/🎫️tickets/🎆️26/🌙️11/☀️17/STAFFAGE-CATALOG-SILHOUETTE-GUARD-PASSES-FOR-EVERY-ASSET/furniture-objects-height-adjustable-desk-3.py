import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "height-adjustable-desk")
n = d.shape(50, "front", "top-rail-mount"); d.set(n, "y", 0.665); d.set(n, "h", 0.03)
n = d.shape(100, "front", "laptop"); o = d.obj(n)
for nm, x in (("a", -0.1425), ("b", 0.17)):
    f = dict(o); f.update({"id": "laptop-foot-" + nm, "role": "detail", "x": x, "y": 0.72, "w": 0.01506, "h": 0.0155}); f.pop("penMm", None); f.pop("r", None)
    d.insert_after(n, f)
d.commit()
