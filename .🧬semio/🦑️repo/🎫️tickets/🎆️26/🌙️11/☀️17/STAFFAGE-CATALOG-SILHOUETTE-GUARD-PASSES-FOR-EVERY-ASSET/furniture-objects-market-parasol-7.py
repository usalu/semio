import importlib.util, os, json, sys
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "market-parasol")
for sc in (50, 100):
    d.set(d.shape(sc, "front", "stretcher-right"), "x1", 0)
    l.delete_items(d, d.at("asset", "representations", str(sc), "geometry", "front"), lambda s: s == "stretcher-left")
d.commit()
