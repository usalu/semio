import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "folding-deck-chair")
v = d.solid(50, "rear"); d.set(v, "size", [1.0215, 0.045, 0.045])
d.setv(l.sp.child(l.sp.child(v, "transform"), "translation"), [0.53425, 0.0, 0.27])
v = d.solid(50, "front"); d.set(v, "size", [0.7025, 0.045, 0.045])
d.setv(l.sp.child(l.sp.child(v, "transform"), "translation"), [0.37475, 0.0, 0.235])
d.set(d.shape(50, "elevation", "rear"), "points", [[0.0235, 0.0225], [1.044994, 0.0225], [1.045006, -0.0225], [0.0235, -0.0225]])
d.set(d.shape(50, "elevation", "front"), "points", [[0.0235, 0.0225], [0.726021, 0.022524], [0.726004, -0.022476], [0.0235, -0.0225]])
d.set(d.at("asset", "dimensions"), "widthM", 0.787)
for lang, a, b in (("en", "0.80 x 0.93 m", "0.79 x 0.93 m"), ("de", "0,80 x 0,93 m", "0,79 x 0,93 m")):
    d.sub(d.at("asset", "provenance", "notes", lang), a, b)
d.commit()
