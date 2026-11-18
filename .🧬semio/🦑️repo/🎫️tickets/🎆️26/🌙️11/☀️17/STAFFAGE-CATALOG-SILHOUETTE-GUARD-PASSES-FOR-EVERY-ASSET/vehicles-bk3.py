import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V(sys.argv[1])
tol = {50: "0.004", 100: "0.006", 200: "0.01", 500: "0.03"}
skip = ("tyre-r", "tyre-f", "rim-r", "rim-f", "hub-r", "hub-f")
views = sys.argv[3].split(",")
for sc in [int(a) for a in sys.argv[2].split(",")]:
    for v in views:
        print(sc, v, d.add_missing(sc, v, "frame", tol[sc], skip=skip))
d.commit()
