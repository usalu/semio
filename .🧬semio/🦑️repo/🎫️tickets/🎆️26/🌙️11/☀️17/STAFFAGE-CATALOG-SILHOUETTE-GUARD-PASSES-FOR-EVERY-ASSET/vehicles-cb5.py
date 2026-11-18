import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
sc = int(sys.argv[1]); view = sys.argv[2]; tol = {50: "0.004", 100: "0.006", 200: "0.005", 500: "0.012"}[sc]
for sid in sys.argv[3].split(","):
    print(view, sid, d.retrace_each(sc, view, sid, [sid], tol))
d.commit()
