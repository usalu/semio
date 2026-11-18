import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
for sc, hw in ((200, 0.04), (100, 0.03), (50, 0.03)):
    s = d.shape(sc, "elevation", "tyre-f")
    d.add_key_after(s, "d", "solid", {"kind": "extruded", "sweep": "w", "z0": -hw, "z1": hw})
s = d.shape(500, "elevation", "fork-line")
d.add_key_after(s, "d", "solid", {"kind": "extruded", "sweep": "w", "z0": -0.098, "z1": 0.098})
d.commit()
