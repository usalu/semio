import importlib.util, os, json, sys
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "market-parasol")
for sc in (50, 100):
    prof = d.obj(d.solid(sc, "canopy"))["profile"]
    up = []
    for a, r in prof:
        if r == 0 and a == 0 and up: break
        up.append((a, r))
        if a == -0.32611 and abs(r - 1.19366) < 1e-6: break
    rim = up[-1][1]
    right = [[round(r, 5), round(2.35 + a, 5)] for a, r in up]
    pts = [[-x, y] for x, y in right] + [[x, y] for x, y in reversed(right)]
    pts = [list(p) for p in pts]
    n = d.shape(sc, "front", "canopy"); o = d.obj(n)
    for k in ("d", "solid"): o.pop(k, None)
    o.update({"kind": "poly", "points": [[x, y] for x, y in [(-right[-1][0], right[-1][1])] + [(-x, y) for x, y in reversed(right[:-1])] + [(x, y) for x, y in right[1:]]], "closed": True})
    d.replace_obj(n, o)
    for it in d.at("asset", "representations", str(sc), "geometry", "front")["items"]:
        sid = json.loads(d.t[l.sp.child(it, "id")["s"]:l.sp.child(it, "id")["e"]])
        if sid in ("rib-front", "rib-oblique-b", "rib-oblique-d", "rib-front-tip", "rib-oblique-b-tip", "rib-oblique-d-tip"): d.set(it, "role", "texture")
d.commit()
