import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
s = d.shape(50, "front", "silencer")
o = d.obj(s)
new = {"kind": "ellipse", "cx": -0.35, "cy": 0.13, "rx": 0.0998, "ry": 0.075, **{k: v for k, v in o.items() if k not in ("kind", "x", "y", "w", "h", "r")}}
d.replace_shape(s, new)
tail = {"kind": "capsule", "x0": -0.3, "y0": 0.15, "x1": -0.3, "y1": 0.25, "r0": 0.03, "r1": 0.025, "id": "tail-pipe", "bone": "chassis", "role": "detail", "z": 0.8, "filled": True, "tone": 0.4, "region": "exhaust-system", "material": "exhaust-steel"}
d.insert_shape_after(s, tail)
d.commit()
