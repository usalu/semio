import importlib.util, os, json, sys
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "market-parasol")
for sc in (50,):
    for view in ("elevation", "front"):
        for it in d.at("asset", "representations", str(sc), "geometry", view)["items"]:
            sid = json.loads(d.t[l.sp.child(it, "id")["s"]:l.sp.child(it, "id")["e"]])
            if "scallop" in sid: d.set(it, "role", "texture")
d.commit()
