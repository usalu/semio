import importlib.util
from pathlib import Path
from scipy.optimize import minimize
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
f = lambda x: m.evaluate({"orientation": 180, "insulation": .16, "wwr": x[0], "overhang": x[1]})["electricity"]
for s in ([.75,.1],[.15,1],[.3,.3],[.8,1.2],[.7,.9],[.2,.2],[.5,.5]):
    r = minimize(f, s, method="Nelder-Mead", bounds=[(.1,.8),(0,1.2)], options={"xatol":1e-6,"fatol":1e-7}); print(s, r.x, r.fun)
