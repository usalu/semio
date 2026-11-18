import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "operating-table")
for sc in (100, 50):
    for view in ("elevation", "plan", "front"):
        l.delete_items(d, d.at("asset", "representations", str(sc), "geometry", view), lambda s: s.startswith("pad-joint"))
d.commit()
