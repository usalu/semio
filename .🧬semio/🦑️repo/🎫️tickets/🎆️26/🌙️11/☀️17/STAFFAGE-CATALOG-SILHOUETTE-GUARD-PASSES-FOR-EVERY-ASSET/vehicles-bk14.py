import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
sc = 50
for v in ("front", "plan"):
    d.to_world_poly(sc, v, "fender-f", "=fender-f", "0.004")
d.to_world_poly(sc, "plan", "chainring", "=chainring", "0.004")
d.to_world_poly(sc, "front", "chainring", "=chainring", "0.004")
d.commit()
