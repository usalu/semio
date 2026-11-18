import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "street-lamp")
d.set(d.shape(50, "elevation", "head-driver"), "d", "M 0.03 0.055 C 0.04 0.075 0.11 0.078 0.13 0.07")
for i, y in enumerate((0.076, 0.076, 0.076, 0.076, 0.074, 0.072)):
    n = d.shape(50, "elevation", "head-fin-%d" % i); pts = d.obj(n)["points"]; pts[1][1] = y
    d.set(n, "points", pts)
d.commit()
