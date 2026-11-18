import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "three-seat-sofa")
for sc in (100, 50):
    for side in ("l", "r"): d.set(d.shape(sc, "plan", "arm-" + side), "r", 0.09)
d.commit()
