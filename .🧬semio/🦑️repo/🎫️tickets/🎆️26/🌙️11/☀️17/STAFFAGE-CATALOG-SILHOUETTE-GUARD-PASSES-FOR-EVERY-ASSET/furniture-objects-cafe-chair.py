import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "cafe-chair")
n = d.shape(100, "elevation", "seat-band"); o = d.obj(n)
d.set(n, "y", -0.028); d.set(n, "h", 0.028)
a = dict(o); a["id"] = "seat-apron"; a["x"] = 0.06; a["y"] = -0.062; a["w"] = 0.36; a["h"] = 0.034; a.pop("r", None); a["tone"] = 0.45
d.insert_after(n, a)
n = d.shape(100, "front", "seat-band"); o = d.obj(n)
d.set(n, "y", 0.432); d.set(n, "h", 0.028)
a = dict(o); a["id"] = "seat-apron"; a["x"] = -0.19; a["y"] = 0.398; a["w"] = 0.38; a["h"] = 0.034; a.pop("r", None); a["tone"] = 0.45
d.insert_after(n, a)
d.commit()
