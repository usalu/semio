import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
drop = ("kind", "x", "y", "w", "h", "r")
s = d.shape(100, "front", "silencer")
o = d.obj(s)
d.replace_shape(s, {"kind": "ellipse", "cx": -0.35, "cy": 0.14, "rx": 0.1, "ry": 0.07, **{k: v for k, v in o.items() if k not in drop}})
d.insert_shape_after(s, {"kind": "capsule", "x0": -0.3, "y0": 0.16, "x1": -0.3, "y1": 0.25, "r0": 0.03, "r1": 0.025, "id": "tail-pipe", "bone": "chassis", "role": "detail", "filled": True})
s = d.shape(100, "front", "hatch-panel")
d.drawn_array(s, "points", [[0, -0.05], [0.8, -0.05], [0.8, 0.03], [0.74, 0.095], [0, 0.08]])
for sid in ("tyre-fl", "tyre-fr", "tyre-rl", "tyre-rr"):
    d.set(d.shape(100, "front", sid), "r", 0.03)
d.commit()
