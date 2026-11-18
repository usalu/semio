import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "operating-table")
o = d.obj(d.solid(50, "tb-plate"))
n = d.solid(100, "tb-plate")
d.replace_obj(n, o)
for sid in ("tb-castor-a", "tb-castor-b"):
    v = d.solid(50, sid); d.setv(l.sp.child(v, "origin"), [d.obj(l.sp.child(v, "origin"))[0], 0.04, 0.17])
d.commit()
