import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
d.to_poly(100, "plan", "hood", "hood-side-l,hood-side-r,hood-top,hood-front-l,hood-front-r,louvre-vanes", "0.004")
s = d.solid(100, "hood-top")
print(d.obj(s)["sections"][0])
s = d.solid(100, "radiator-shell"); print(d.obj(s))
d.commit()
