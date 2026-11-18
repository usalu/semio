import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
IDS = "body-shell,body-upper,roof-cap,windscreen,hood-panel,door-window,bumper,grille,headlamp,tail-lamp,tail-pipe,slider-panel,slider-track,rear-leaf,rear-hinge,axle"
tol = {500: "0.02", 200: "0.01", 100: "0.006", 50: "0.004"}
for sc in [int(a) for a in sys.argv[1:]]:
    for v in ("elevation", "front", "plan"):
        d.to_poly(sc, v, "body-shell", IDS, tol[sc])
d.commit()
