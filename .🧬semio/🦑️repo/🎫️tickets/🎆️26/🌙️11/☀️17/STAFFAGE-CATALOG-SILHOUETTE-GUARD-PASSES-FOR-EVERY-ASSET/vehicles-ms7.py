import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
for sid, a, b in (("grip-l", 0.29, 0.339), ("grip-r", -0.29, -0.339)):
    s = d.shape(50, "plan", sid)
    o = d.obj(s)
    d.replace_shape(s, {"kind": "capsule", "x0": 0.3937, "y0": a, "x1": 0.3937, "y1": b, "r0": 0.031, "r1": 0.031, **{k: v for k, v in o.items() if k not in ("kind", "cx", "cy", "rx", "ry", "solid", "bone")}, "bone": "frame"})
d.commit()
