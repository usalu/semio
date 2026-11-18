import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
drop = ("kind", "points", "closed")
for view in ("front", "plan"):
    s = d.shape(100, view, "handlebar")
    o = d.obj(s)
    d.replace_shape(s, {"kind": "ellipse", "cx": -0.09, "cy": 0.0, "rx": 0.022, "ry": 0.022, **{k: v for k, v in o.items() if k not in drop}})
s = d.shape(100, "plan", "muffler")
d.set(s, "x0", -0.84); d.set(s, "x1", -0.55); d.set(s, "r0", 0.05); d.set(s, "r1", 0.05)
ref = d.obj(d.shape(50, "plan", "engine-cover"))
extra = {k: ref[k] for k in ("z", "tone", "region", "material") if k in ref}
b = d.shape(100, "plan", "body")
d.insert_shape_after(b, {"kind": "ellipse", "cx": -0.43, "cy": -0.185, "rx": 0.11, "ry": 0.03, "id": "engine-cover", "bone": "swing", "role": "body", "filled": True, **extra})
b = d.shape(100, "front", "body")
d.insert_shape_after(b, {"kind": "ellipse", "cx": -0.185, "cy": 0.52, "rx": 0.03, "ry": 0.09, "id": "engine-cover", "bone": "frame", "role": "body", "filled": True, **extra})
d.commit()
