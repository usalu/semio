import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/motor-scooter")
s = d.shape(100, "elevation", "body")
pts = d.obj(s)["points"]
pts[7] = [0.562, 0.505]
d.drawn_array(s, "points", pts)
d.commit()
