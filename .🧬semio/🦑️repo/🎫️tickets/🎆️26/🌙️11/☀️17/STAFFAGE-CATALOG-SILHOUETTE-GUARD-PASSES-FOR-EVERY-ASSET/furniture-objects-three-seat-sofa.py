import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "three-seat-sofa")
for sc in (100, 50):
    n = d.shape(sc, "plan", "arm-r"); o = d.obj(d.shape(sc, "elevation", "skirt"))
    o.update({"y": -0.43, "h": 0.86}); d.insert_after(n, o)
    for k in (1, 2, 3):
        b = d.shape(sc, "elevation", "back-%d" % k)
        w = d.val("asset", "representations", str(sc), "geometry", "elevation", [i for i, it in enumerate(d.at("asset", "representations", str(sc), "geometry", "elevation")["items"]) if l.sp.json.loads(d.t[l.sp.child(it, "id")["s"]:l.sp.child(it, "id")["e"]]) == "back-%d" % k][0], "d")
        W, H = w.split(" L ")[3].split()[0], None
        d.set(b, "d", "M 0 0 L 0 0.65 L %s 0.65 L %s 0 Z" % (W, W))
        p = d.shape(sc, "plan", "back-%d" % k); d.set(p, "r", 0)
    for side in ("l", "r"):
        e = d.shape(sc, "elevation", "arm-" + side)
        d.set(e, "d", "M 0.09 0 Q 0 0 0 0.09 L 0 0.44 Q 0 0.52 0.08 0.52 L 0.12 0.52 Q 0.2 0.52 0.2 0.44 L 0.2 0.09 Q 0.2 0 0.11 0 Z")
f = d.shape(100, "front", "arm-l")
d.set(f, "d", "M -0.36 0.1 Q -0.45 0.1 -0.45 0.19 L -0.45 0.5 Q -0.45 0.62 -0.33 0.62 L 0.33 0.62 Q 0.45 0.62 0.45 0.5 L 0.45 0.19 Q 0.45 0.1 0.36 0.1 Z")
f = d.shape(50, "front", "arm-l")
d.set(f, "d", "M -0.36 0.1 Q -0.45 0.1 -0.45 0.19 L -0.45 0.5 Q -0.45 0.62 -0.33 0.62 L 0.33 0.62 Q 0.45 0.62 0.45 0.5 L 0.45 0.19 Q 0.45 0.1 0.36 0.1 Z")
d.commit()
