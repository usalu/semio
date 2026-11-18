import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
d.to_world_poly(200, "plan", "sweep-l", "=handlebar-l", "0.006")
d.to_world_poly(200, "plan", "sweep-r", "=handlebar-r", "0.006")
d.commit()
