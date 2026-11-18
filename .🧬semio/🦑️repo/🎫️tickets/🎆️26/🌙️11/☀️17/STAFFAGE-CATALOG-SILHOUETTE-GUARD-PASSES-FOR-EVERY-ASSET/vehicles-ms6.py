import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
for sid, a, b in (("grip-l", 0.29, 0.339), ("grip-r", -0.29, -0.339)):
    s = d.shape(50, "front", sid)
    o = d.obj(s)
    d.replace_shape(s, {"kind": "capsule", "x0": a, "y0": 1.066, "x1": b, "y1": 1.066, "r0": 0.031, "r1": 0.031, **{k: v for k, v in o.items() if k not in ("kind", "cx", "cy", "rx", "ry", "solid", "bone")}, "bone": "frame"})
e = d.shape(50, "front", "engine-cover")
o = d.obj(e); pl = d.obj(d.shape(50, "plan", "engine-cover"))
d.replace_shape(e, {**o, **{k: pl[k] for k in ("z", "tone", "region", "material") if k in pl}})
d.commit()
