import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "cafe-chair")
for i, sg in ((0, 1), (1, -1)):
    n = d.shape(50, "plan", "stretcher-tube-%d" % i)
    d.set(n, "y0", sg * 0.2157); d.set(n, "y1", sg * 0.2254)
d.commit()
