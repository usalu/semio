import importlib.util, os
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "cafe-chair")
for i, sg in ((0, 1), (1, -1)):
    for kind, z in (("rear", 0.18), ("front", 0.19)):
        n = d.shape(50, "front", "leg-%s-tube-%d" % (kind, i))
        o = d.obj(d.shape(50, "front", "stretcher-joint-%d" % i))
        o.update({"id": "leg-%s-knuckle-%d" % (kind, i), "bone": "leg-" + kind, "cx": sg * z, "cy": 0.42, "rx": 0.0125, "ry": 0.014})
        d.insert_after(n, o)
d.commit()
