import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
sc = 50
for v in ("front", "plan"):
    for sid, ids in (("pedal-n", "=pedal-n"), ("pedal-f", "=pedal-f"), ("arm-n", "=crank-arm-n"), ("arm-f", "=crank-arm-f"), ("stand-l", "=stand-leg-l"), ("stand-r", "=stand-leg-r")):
        d.to_world_poly(sc, v, sid, ids, "0.003")
d.commit()
