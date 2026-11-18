import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
sc = int(sys.argv[1])
tol = str({100: 0.006, 200: 0.012}[sc])
d = m.V("🚜️vehicles/vintage-car-1929")
def has(view, sid):
    try: d.shape(sc, view, sid); return True
    except KeyError: return False
for v, sid, ids in (("front", "fender-l", "fender-front-l,fender-rear-l"), ("front", "fender-r", "fender-front-r,fender-rear-r"), ("plan", "fender-fl", "fender-front-l"), ("plan", "fender-fr", "fender-front-r"), ("plan", "fender-rl", "fender-rear-l"), ("plan", "fender-rr", "fender-rear-r")):
    if has(v, sid): d.to_poly(sc, v, sid, ids, tol)
d.commit()
