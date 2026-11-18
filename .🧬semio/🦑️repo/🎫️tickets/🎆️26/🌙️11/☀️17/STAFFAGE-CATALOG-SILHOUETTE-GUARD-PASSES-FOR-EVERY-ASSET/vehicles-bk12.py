import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
sc = 50
STEER = (0.299, 0.932, -70)
spec = [("seat-tube", "=seat-tube", None), ("down-tube", "=main-tube", None), ("head-tube", "=head-tube,=head-cup-top,=head-cup-bottom", None), ("chainstay", "=chainstay-l,=chainstay-r", None), ("seatstay", "=seatstay-l,=seatstay-r", None), ("rack", "=rack-deck,=rack-rim", None), ("saddle", "=saddle,=seat-lug,=seat-post", None), ("chaincase", "=chaincase,=chaincase-rim", None), ("tail-lamp", "=tail-lamp", None), ("fender-r", "=fender-r", None),
        ("fork", "=fork-l,=fork-r,=fork-crown", STEER), ("stem-bar", "=stem-neck,=stem-clamp,=handlebar-l,=handlebar-r", STEER), ("fender-f", "=fender-f", STEER)]
for sid, ids, xf in spec:
    d.to_poly(sc, "elevation", sid, ids, "0.004", xf=xf)
d.commit()
