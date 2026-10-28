import importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
for w in (0.2, 0.3, 0.4, 0.5, 0.6, 0.8):
    print(w, [tuple(round(m.evaluate({"wwr": w, "overhang": d})[k]) for k in ("heating","cooling","lighting","electricity")) for d in (0, .3, .6, .9, 1.2)])
