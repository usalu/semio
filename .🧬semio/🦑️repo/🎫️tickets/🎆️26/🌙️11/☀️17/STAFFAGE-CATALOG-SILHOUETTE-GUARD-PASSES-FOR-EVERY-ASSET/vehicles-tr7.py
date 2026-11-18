import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
tol = {500: "0.02", 200: "0.008", 100: "0.004", 50: "0.003"}
for sc in [int(a) for a in sys.argv[1:]]:
    for sid, ids, xf in (("hvac-box", "=hvac-box", (0, 0.3, 180)), ("resistor-box", "=resistor-box", None), ("pan-base", "=pan-base", None)):
        try: d.shape(sc, "elevation", sid)
        except KeyError: continue
        d.to_poly(sc, "elevation", sid, ids, tol[sc], xf=xf)
d.commit()
