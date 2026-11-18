import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
tol = {500: "0.03", 200: "0.012", 100: "0.008", 50: "0.005"}
for sc in [int(a) for a in sys.argv[1:]]:
    d.to_poly(sc, "front", "shell-end", "*,!wheel,!axle,!pan-,!brake,!motor,!headlamp", tol[sc])
d.commit()
