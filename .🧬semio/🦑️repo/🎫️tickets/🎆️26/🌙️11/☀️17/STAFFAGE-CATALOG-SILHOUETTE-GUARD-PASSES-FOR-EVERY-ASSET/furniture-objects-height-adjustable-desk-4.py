import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "height-adjustable-desk")
n = d.solid(50, "top-rail-mount-4"); o = d.obj(n)
for i, x in ((1, 0.002), (2, 1.098)):
    c = dict(o); c.update({"id": "top-rail-cap-%d" % i, "role": "body", "size": [0.004, 0.05, 0.05], "transform": {"translation": [x, -0.05, 0.275]}})
    d.insert_after(n, c)
d.commit()
