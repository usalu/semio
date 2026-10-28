import importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
for w in (.2, .3, .38, .5, .6, .8):
    print(w, [round(m.evaluate({"wwr": w, "overhang": d})["electricity"]) for d in (0, .05, .1, .15, .2, .3, .4, .6, .8, 1.0, 1.2)], [round(m.evaluate({"wwr": w, "overhang": d})["shading_summer"], 2) for d in (0, .05, .1, .15, .2, .3)])
print([ (s["hour"], round(s["altitude"],1), round(s["across"],1), round(s["profile"],1), round(s["weight"],2)) for s in m.sun_samples(180, "summer")])
