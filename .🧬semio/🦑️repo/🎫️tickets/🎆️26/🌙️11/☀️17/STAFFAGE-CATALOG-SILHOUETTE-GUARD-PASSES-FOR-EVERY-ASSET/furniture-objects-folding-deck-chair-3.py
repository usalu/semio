import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "folding-deck-chair")
d.set(d.at("asset", "dimensions"), "widthM", 0.8)
for lang, a, b in (("en", "0.78 x 0.93 m", "0.80 x 0.93 m"), ("de", "0,78 x 0,93 m", "0,80 x 0,93 m")):
    n = d.at("asset", "provenance", "notes", lang)
    d.sub(n, a, b)
d.commit()
