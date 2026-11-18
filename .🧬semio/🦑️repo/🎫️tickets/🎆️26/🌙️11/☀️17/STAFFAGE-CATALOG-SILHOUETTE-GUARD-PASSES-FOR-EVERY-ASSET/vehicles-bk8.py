import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
sc = 100
spec = [("chainstays", "=chainstay-l,=chainstay-r"), ("spine", "=seat-tube,=main-tube"), ("saddle", "=saddle"), ("rack", "=rack"), ("chaincase", "=chaincase"), ("tail-lamp", "=tail-lamp"), ("fender-r", "=fender-r"), ("fender-f", "=fender-f")]
for sid, ids in spec:
    d.to_world_poly(sc, "plan", sid, ids, "0.005")
d.commit()
