import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
slug = sys.argv[1]
d = m.V(slug)
tol = {50: "0.003", 100: "0.005", 200: "0.01", 500: "0.03"}
for sc in [int(a) for a in sys.argv[2:]]:
    for v in ("elevation", "front", "plan"):
        print(sc, v, d.add_missing(sc, v, "frame", tol[sc], skip=tuple(i for i in ("tyre-r","tyre-f","rim-r","rim-f","hub-r","hub-f") )))
d.commit()
