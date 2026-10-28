import importlib.util, itertools
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
v = m._frozen()[1]
def run(kw):
    for (sec, k), val in kw.items(): v[sec][k] = val
    m._evaluate.cache_clear(); 
    best = min(((m._evaluate(180.0, w/100, d/10, .16)["electricity"], w, d) for w in range(10, 81, 1) for d in range(0, 13)))
    corner = m._evaluate(180.0, .1, 0.0, .16)["electricity"], m._evaluate(180.0, .8, 1.2, .16)["electricity"], m._evaluate(180.0, .8, 0, .16)["electricity"], m._evaluate(180.0, .1, 1.2, .16)["electricity"]
    return best, [round(c) for c in corner]
for s, w, l in itertools.product((.4, .5, .6), (.3, .4, .5), (.5, .7)):
    print(s, w, l, run({("shading","summer_height_ratio"): s, ("shading","winter_height_ratio"): w, ("daylight","sky_angle_loss"): l}))
