import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
FR = None
spec = {
 200: {"elevation": [("seat-tube", "=seat-tube", None), ("down-tube", "=main-tube", None), ("chainstay", "=chainstay-l,=chainstay-r,=chainstay-web", None), ("seatstay", "=seatstay-l,=seatstay-r", None), ("saddle", "=saddle", None), ("chaincase", "=chaincase", None), ("lamps", "=tail-lamp", None), ("rack", "=rack", None), ("fork", "=fork-l,=fork-r,=fork-crown", (0.299, 0.932, -70)), ("stem", "=stem-neck", (0.299, 0.932, -70))]},
}
tol = {200: "0.008"}
for sc, views in spec.items():
    for v, items in views.items():
        for sid, ids, xf in items:
            d.to_poly(sc, v, sid, ids, tol[sc], xf=xf)
d.commit()
