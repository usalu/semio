import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "height-adjustable-desk")
for sc in (100, 50):
    n = d.shape(sc, "front", "top-rail"); o = d.obj(n)
    o.update({"id": "top-rail-mount", "role": "detail", "y": 0.67, "h": 0.025}); o.pop("penMm", None); o.pop("r", None)
    d.insert_after(n, o)
d.commit()
