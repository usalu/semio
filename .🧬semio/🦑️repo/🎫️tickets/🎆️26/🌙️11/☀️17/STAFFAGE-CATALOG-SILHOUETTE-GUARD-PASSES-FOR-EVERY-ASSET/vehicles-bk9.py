import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
d.rename_to_world_poly(100, "plan", "chainstays", "chainstay-l", "=chainstay-l", "0.004")
d.insert_world_poly_after(100, "plan", "chainstays", "chainstay-r", "=chainstay-r", "0.004")
d.rename_to_world_poly(100, "plan", "spine", "seat-tube", "=seat-tube", "0.004")
d.commit()
