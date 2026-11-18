import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
s = d.shape(50, "plan", "body-shell")
half = [[-1.947, 0.55], [-1.945, 0.61], [-1.9, 0.72], [-1.8, 0.74], [-1.5, 0.76], [-0.8, 0.76], [0.8, 0.76], [1.3, 0.75], [1.56, 0.741], [1.62, 0.714], [1.7, 0.698], [1.75, 0.69], [1.85, 0.694], [1.9, 0.669], [1.92, 0.653], [1.935, 0.55], [1.944, 0.42], [1.951, 0.39]]
pts = [[-1.947, 0]] + half + [[1.951, 0]] + [[x, -y] for x, y in reversed(half)] + [[-1.947, 0]]
d.drawn_array(s, "points", pts)
d.commit()
