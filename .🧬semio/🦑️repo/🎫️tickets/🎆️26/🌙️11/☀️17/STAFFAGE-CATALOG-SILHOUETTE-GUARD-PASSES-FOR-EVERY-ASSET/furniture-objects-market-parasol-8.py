import importlib.util, os, json, sys
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "market-parasol")
d.set(d.at("asset", "dimensions"), "widthM", 2.59); d.set(d.at("asset", "dimensions"), "depthM", 2.59)
d.commit()
