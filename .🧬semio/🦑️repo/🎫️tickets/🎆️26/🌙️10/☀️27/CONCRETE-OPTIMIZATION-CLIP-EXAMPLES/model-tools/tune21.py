import importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
for d in (0, .6, 1.0, 1.2):
    print("d", d)
    for w in (.2, .3, .4, .5, .6, .7, .8):
        e = m.evaluate({"wwr": w, "overhang": d}); print("  ", w, [round(e[k]) for k in ("heating", "cooling", "lighting", "electricity")], round(e["shading_summer"], 2), round(e["shading_winter"], 2))
