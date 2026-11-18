import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
drop = ("kind", "points", "closed")
for view in ("front", "plan"):
    for sid, r, z0, z1 in (("grip-l", 0.031, 0.259, 0.37), ("grip-r", 0.031, -0.37, -0.259), ("handlebar", 0.0175, None, None)):
        s = d.shape(50, view, sid)
        o = d.obj(s)
        sol = dict(o["solid"])
        if z0 is not None: sol["z0"], sol["z1"] = z0, z1
        new = {"kind": "ellipse", "cx": -0.09, "cy": 0.0, "rx": r, "ry": r, **{k: v for k, v in o.items() if k not in drop and k != "solid"}, "solid": sol}
        d.replace_shape(s, new)
d.commit()
