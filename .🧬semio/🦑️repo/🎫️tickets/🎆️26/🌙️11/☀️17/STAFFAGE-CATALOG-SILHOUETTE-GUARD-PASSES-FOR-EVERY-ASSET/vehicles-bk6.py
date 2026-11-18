import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
for sid, ids in (("saddle", "=saddle"), ("rack", "=rack"), ("chaincase", "=chaincase"), ("tail-lamp", "=tail-lamp")):
    d.to_poly(200, "front", sid, ids, "0.006")
d.commit()
