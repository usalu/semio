import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
for sc, tol in ((50, "0.004"), (100, "0.008")):
    d.to_poly(sc, "front", "roof-cap", "roof-panel,body-upper", tol)
d.commit()
