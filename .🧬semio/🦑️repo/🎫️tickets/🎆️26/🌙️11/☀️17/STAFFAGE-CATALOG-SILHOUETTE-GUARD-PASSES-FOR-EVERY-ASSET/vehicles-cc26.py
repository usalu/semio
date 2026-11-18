import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
s = d.shape(100, "elevation", "headlamp")
o = d.obj(s)
d.replace_shape(s, {"kind": "ellipse", "cx": 1.74, "cy": 0.725, "rx": 0.1, "ry": 0.05, **{k: v for k, v in o.items() if k not in ("kind", "d")}})
d.commit()
