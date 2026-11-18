import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
s = d.shape(50, "front", "hatch-panel")
d.drawn_array(s, "points", [[0, -0.05], [0.8, -0.05], [0.8, 0.03], [0.74, 0.095], [0, 0.08]])
d.commit()
