import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "playground-swing")
for side, xt, xc in (("left", 0.224, 0.205), ("right", -0.236, -0.255)):
    n = d.shape(50, "front", "beam-hanger-" + side); o = d.obj(n)
    for k in ("d",): o.pop(k, None)
    c = dict(o); c.update({"id": "hanger-collar-" + side, "kind": "rect", "filled": True, "x": xc, "y": 2.2425, "w": 0.05, "h": 0.115})
    o.update({"kind": "rect", "filled": True, "x": xt, "y": 2.205, "w": 0.012, "h": 0.04})
    d.replace_obj(n, o); d.insert_after(n, c)
d.commit()
