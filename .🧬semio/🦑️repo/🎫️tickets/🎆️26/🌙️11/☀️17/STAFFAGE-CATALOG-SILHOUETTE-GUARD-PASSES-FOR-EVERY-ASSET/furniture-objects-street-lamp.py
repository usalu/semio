import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "street-lamp")
def prof_d(p):
    L = [(r, y) for y, r in p]
    half = [(r, y) for r, y in L if True]
    pts = [(-r, y) for r, y in half] + [(r, y) for r, y in reversed(half)]
    return "M " + " L ".join("%g %g" % q for q in pts) + " Z"
for sc in (100, 50):
    p = [q for q in d.obj(d.solid(sc, "base-cover"))["profile"]]
    half = []
    for i in range(len(p)):
        if i and p[i][0] == p[i - 1][0] and p[i][0] == 0.3: break
        half.append(p[i])
    half = [(r, y) for y, r in half]
    pts = [(-r, y) for r, y in half] + [(r, y) for r, y in reversed(half)]
    dd = "M " + " L ".join("%g %g" % q for q in pts) + " Z"
    for v in ("elevation", "front"): d.set(d.shape(sc, v, "base-cover"), "d", dd)
d.commit()
