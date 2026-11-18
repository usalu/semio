import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
s = d.shape(100, "front", "body-shell")
half = [[0.72, 0.2], [0.76, 0.35], [0.76, 0.9], [0.758, 0.97], [0.745, 0.99], [0.74, 1.04], [0.74, 1.12], [0.718, 1.2], [0.69, 1.3], [0.665, 1.4], [0.65, 1.44], [0.623, 1.46], [0.609, 1.48], [0.52, 1.495]]
pts = [[0, 0.2]] + half + [[0, 1.5]] + [[-x, y] for x, y in reversed(half)] + [[0, 0.2]]
d.drawn_array(s, "points", pts)
d.commit()
