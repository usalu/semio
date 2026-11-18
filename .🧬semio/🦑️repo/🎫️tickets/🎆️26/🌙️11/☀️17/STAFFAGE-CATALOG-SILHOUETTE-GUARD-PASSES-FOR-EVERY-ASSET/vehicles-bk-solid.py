import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
for sc, sid, hw in ((50, "tyre-f", 0.02), (50, "rim-f", 0.0125), (100, "wheel-f", 0.02), (200, "wheel-f", 0.03)):
    s = d.shape(sc, "elevation", sid)
    d.add_key_after(s, "d", "solid", {"kind": "extruded", "sweep": "w", "z0": -hw, "z1": hw})
d.commit()
