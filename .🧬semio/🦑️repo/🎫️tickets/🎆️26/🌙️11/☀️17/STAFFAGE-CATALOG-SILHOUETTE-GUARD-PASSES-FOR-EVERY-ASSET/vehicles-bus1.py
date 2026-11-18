import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
IDS = "*,!tyre,!rim,!hub,!stud,!drum,!wheel,!mirror"
tol = {500: "0.03", 200: "0.012", 100: "0.008", 50: "0.005"}
for sc in [int(a) for a in sys.argv[1:]]:
    for v in ("elevation", "front", "plan"):
        d.to_poly(sc, v, "body-shell", IDS, tol[sc])
d.commit()
