import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "folding-deck-chair")
for sc in (50, 100):
    for sid, vid in (("canvas-a", "canvas-back"), ("canvas-b", "canvas-seat")):
        prof = d.obj(d.solid(sc, vid))["profile"]
        n = d.shape(sc, "elevation", sid)
        o = d.obj(n)
        o["points"] = [[round(x, 5), round(y, 5)] for x, y in prof]; o["closed"] = True; o["filled"] = True; o.pop("penMm", None)
        d.replace_obj(n, o)
d.commit()
