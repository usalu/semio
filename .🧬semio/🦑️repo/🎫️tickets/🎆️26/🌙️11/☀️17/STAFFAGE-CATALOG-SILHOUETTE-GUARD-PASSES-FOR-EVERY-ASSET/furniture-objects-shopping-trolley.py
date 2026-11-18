import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "shopping-trolley")
sh = lambda s: d.shape(100, "front", s)
d.set(sh("basket"), "points", [[-0.2895, 0.88], [0.2895, 0.88], [0.2045, 0.38], [-0.2045, 0.38]])
d.set(sh("wheel-front-l"), "x", 0.2); d.set(sh("wheel-front-r"), "x", -0.24)
for s, x in (("hub-front-l", 0.198), ("hub-front-r", -0.242)):
    d.set(sh(s), "x", x); d.set(sh(s), "w", 0.044); d.set(sh(s), "y", 0.0405); d.set(sh(s), "h", 0.044)
for s in ("chassis-line", "rear-fork", "front-fork"): d.set(sh(s), "penMm", 0.15)
d.commit()
