import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "operating-table")
parts = (("head", -1.0, 0.378), ("back", -0.618, 0.566), ("seat", -0.048, 0.246), ("leg", 0.202, 0.798))
for sc, r in ((100, 0.02), (50, 0.03)):
    for view in ("elevation", "plan"):
        n = d.shape(sc, view, "top-pad"); o = d.obj(n)
        o.pop("solid", None)
        new = []
        for nm, x, w in parts:
            p = dict(o); p.update({"id": "top-pad-" + nm, "x": x, "w": w, "r": r})
            if view == "plan": p.update({"y": -0.25, "h": 0.5})
            new.append(p)
        d.replace_obj(n, new[0])
        for p in new[1:]: d.insert_after(n, p) if False else None
        d.pend2 = getattr(d, "pend2", []) + [(n, new[1:])]
for n, rest in d.pend2:
    for p in reversed(rest): d.insert_after(n, p)
d.commit()
