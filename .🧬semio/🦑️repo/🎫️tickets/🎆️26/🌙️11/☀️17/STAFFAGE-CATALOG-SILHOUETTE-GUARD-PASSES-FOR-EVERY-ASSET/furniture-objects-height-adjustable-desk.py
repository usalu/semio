import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "height-adjustable-desk")
n = d.shape(100, "elevation", "top-rail"); o = d.obj(n)
o.update({"id": "top-rail-mount", "role": "detail", "x": 0.05, "y": -0.025, "w": 0.1, "h": 0.025}); o.pop("penMm", None); o.pop("r", None)
d.insert_after(n, o)
n = d.shape(50, "elevation", "top-rail"); o = d.obj(n)
for i, x in enumerate((0.07, 0.37, 0.67, 0.97), 1):
    m = dict(o); m.update({"id": "top-rail-mount-%d" % i, "role": "detail", "x": x, "y": -0.03, "w": 0.06, "h": 0.03}); m.pop("penMm", None); m.pop("r", None)
    d.insert_after(n, m)
for sc in (100, 50):
    n = d.shape(sc, "elevation", "top-cable-tray")
    d.set(n, "filled", True); d.set(n, "closed", True)
for side, x in (("l", -0.05), ("r", 0.032)):
    n = d.shape(50, "front", side + "-drive"); o = d.obj(n)
    o.update({"id": side + "-drive-mount", "role": "detail", "x": x, "y": 0.03, "w": 0.018, "h": 0.11}); o.pop("penMm", None); o.pop("r", None)
    d.insert_after(n, o)
d.commit()
