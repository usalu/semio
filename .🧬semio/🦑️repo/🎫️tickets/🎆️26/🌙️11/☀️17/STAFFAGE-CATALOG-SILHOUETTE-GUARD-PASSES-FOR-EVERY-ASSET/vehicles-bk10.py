import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
d.rename_to_world_poly(100, "plan", "crossbar", "crossbar", "=handlebar-l,=handlebar-r", "0.004")
d.commit()
