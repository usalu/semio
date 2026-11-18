import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "folding-deck-chair")
for sc in (200, 500):
    d.set(d.shape(sc, "elevation", "rear"), "points", [[0.0, 0.0225], [1.045, 0.0225], [1.045, -0.0225], [0.0, -0.0225]])
d.commit()
