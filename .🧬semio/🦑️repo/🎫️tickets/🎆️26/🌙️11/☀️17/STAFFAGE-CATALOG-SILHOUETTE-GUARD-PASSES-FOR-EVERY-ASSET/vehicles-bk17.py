import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
sc = 50
for sid, ids in (("stem-low", "=stem-neck"), ("fork-crown", "=fork-crown"), ("head-lamp", "=head-lamp")):
    d.to_world_poly(sc, "front", sid, ids, "0.003")
for sid, ids in (("stem-head", "=stem-neck,=stem-clamp"), ("fork-crown", "=fork-crown"), ("head-lamp", "=head-lamp")):
    d.to_world_poly(sc, "plan", sid, ids, "0.003")
d.commit()
