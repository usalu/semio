import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
for sid, z in (("mirror-l", 0.79), ("mirror-r", -0.79)):
    s = d.solid(50, sid)
    d.set(s, "radii", [0.07, 0.045, 0.05])
    d.set(d.field(s, "transform"), "translation", [0.86, 1.07, z])
for sid, z in (("mirror-l", 0.79), ("mirror-r", -0.79)):
    s = d.shape(50, "plan", sid)
    o = d.obj(s)
    d.replace_shape(s, {"kind": "ellipse", "cx": 0.86, "cy": z, "rx": 0.07, "ry": 0.05, **{k: v for k, v in o.items() if k not in ("kind", "x", "y", "w", "h", "r")}})
s = d.shape(50, "front", "mirror-l"); d.set(s, "x", 0.74); d.set(s, "w", 0.1)
s = d.shape(50, "front", "mirror-r"); d.set(s, "w", 0.1)
d.commit()
