import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
drop = ("kind", "x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "points", "closed", "d")
def has(sc, v, sid):
    try: d.shape(sc, v, sid); return True
    except KeyError: return False
for sc in (50, 100):
    if has(sc, "elevation", "mirror"):
        node = d.shape(sc, "elevation", "mirror")
        d.replace_shape(node, {"kind": "rect", "x": 11.79, "y": 2.115, "w": 0.12, "h": 0.37, "r": 0.02, **{k: v for k, v in d.obj(node).items() if k not in drop}})
    for sid, x in (("mirror-l", 1.2), ("mirror-r", -1.275)):
        s = d.shape(sc, "front", sid); d.set(s, "y", 2.115); d.set(s, "h", 0.37); d.set(s, "r", 0.02)
    for sid, y in (("mirror-l", 1.2), ("mirror-r", -1.275)):
        s = d.shape(sc, "plan", sid); d.set(s, "x", 11.79); d.set(s, "w", 0.12); d.set(s, "r", 0.02)
d.commit()
