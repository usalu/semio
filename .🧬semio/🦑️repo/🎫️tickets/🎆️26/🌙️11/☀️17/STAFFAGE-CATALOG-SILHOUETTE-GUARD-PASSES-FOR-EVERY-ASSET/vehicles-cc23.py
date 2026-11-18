import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
drop = ("kind", "x", "y", "w", "h", "r")
for sid, z in (("headlamp-l", 0.6), ("headlamp-r", -0.6)):
    s = d.shape(50, "plan", sid)
    d.replace_shape(s, {"kind": "ellipse", "cx": 1.82, "cy": z, "rx": 0.12, "ry": 0.1, **{k: v for k, v in d.obj(s).items() if k not in drop}})
    s = d.shape(50, "front", sid)
    d.replace_shape(s, {"kind": "ellipse", "cx": z, "cy": 0.715, "rx": 0.1, "ry": 0.07, **{k: v for k, v in d.obj(s).items() if k not in drop}})
d.commit()
