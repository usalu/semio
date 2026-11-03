"""H1: the cyclist 1:200 side-view crank lines were drawn with the body pen 0.18 mm, below the 0.2 mm feature limit; give the two crank strokes their own 0.2 mm pen."""
from lib import Ctx

c = Ctx("cyclist")
for s in c.reps["200"]["geometry"]["elevation"]:
    if s["id"] in ("crank-r", "crank-l"):
        s["penMm"] = 0.2
c.save()
print("cyclist 1:200 crank pens set")
