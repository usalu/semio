"""Count overlapping thumbnail pairs on the cost-electricity plot for a few ranges and tile sizes."""
import importlib, sys, math
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
d = m.pareto()["designs"]
costs = [x["cost"] for x in d]; els = [x["electricity"] for x in d]
x0, x1 = math.floor((min(costs)-100)/250)*250, math.ceil((max(costs)+100)/250)*250
y0, y1 = math.floor((min(els)-50)/50)*50, math.ceil((max(els)+50)/50)*50
print(x0, x1, y0, y1, min(els), max(els))
for width, height in ((5.1, 4.1), (5.1, 4.3)):
    for w, h in ((.34, .28), (.29, .24)):
        pts = [((c-x0)/(x1-x0)*width, (e-y0)/(y1-y0)*height) for c, e in zip(costs, els)]
        n = sum(1 for i in range(len(pts)) for j in range(i) if abs(pts[i][0]-pts[j][0]) < w and abs(pts[i][1]-pts[j][1]) < h)
        print(width, height, w, h, "overlapping pairs", n)
