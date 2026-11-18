import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
IDS = "*,!tyre,!rim,!hub,!stud,!drum,!-wheel,!mirror"
spec = {50: ("0.003", ("elevation", "front", "plan")), 100: ("0.008", ("elevation", "front", "plan")), 200: ("0.012", ("elevation", "front", "plan")), 500: ("0.03", ("front", "plan"))}
for sc, (tol, views) in spec.items():
    for v in views: d.to_poly(sc, v, "body-shell", IDS, tol)
d.to_poly(500, "elevation", "body-shell", IDS, "0.008")
d.commit()
