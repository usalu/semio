import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
web = d.obj(d.solid(200, "chainstay-web"))
d.insert_solid_after(50, "chainstay-r", web)
d.commit()
