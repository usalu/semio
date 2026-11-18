import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
sc = 100
STEER = (0.299, 0.932, -70)
spec = [("seat-tube", "=seat-tube", None), ("down-tube", "=main-tube", None), ("head-tube", "=head-tube", None), ("chainstay", "=chainstay-l,=chainstay-r", None), ("seatstay", "=seatstay-l,=seatstay-r", None), ("rack", "=rack", None), ("saddle", "=saddle", None), ("chaincase", "=chaincase", None), ("tail-lamp", "=tail-lamp", None), ("fender-r", "=fender-r", None),
        ("fork", "=fork-l,=fork-r,=fork-crown", STEER), ("stem", "=stem-neck", STEER), ("fender-f", "=fender-f", STEER)]
for sid, ids, xf in spec:
    d.to_poly(sc, "elevation", sid, ids, "0.005", xf=xf)
d.commit()
