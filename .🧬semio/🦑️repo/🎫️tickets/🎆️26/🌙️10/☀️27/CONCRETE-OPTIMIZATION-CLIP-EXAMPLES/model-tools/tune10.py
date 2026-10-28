import importlib.util, itertools, sys
from pathlib import Path
spec = importlib.util.spec_from_file_location("m", Path("🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🐍️.py"))
m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
st = m._frozen()[0]["algorithms"]
opt = m.surface()["minimum"]["design"]
print(opt)
for start in ({"wwr": .75, "overhang": .1}, {"wwr": .75, "overhang": .0}, {"wwr": .2, "overhang": .2}, {"wwr": .3, "overhang": .1}):
    st["start"] = start
    for lr in (0.0003, 0.0004, 0.0005, 0.0006, 0.0008, 0.001):
        st["gradient"]["learning_rate"] = lr
        path = m._gradient_path.__wrapped__()
        print(start, lr, [(round(x["design"]["wwr"],2), round(x["design"]["overhang"],2), round(x["electricity"])) for x in path])
