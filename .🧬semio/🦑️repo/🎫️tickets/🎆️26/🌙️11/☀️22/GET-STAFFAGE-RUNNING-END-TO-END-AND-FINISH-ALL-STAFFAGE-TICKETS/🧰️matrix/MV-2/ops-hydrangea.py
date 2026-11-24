"""💠 Writes `ops/hydrangea.json`: at 1:500 the root collar is a frustum that widens upwards (radius 0.17 m on the ground, 0.22 m at 0.22 m; body solid `root-collar` and both elevations), but its plan was the circle of the ground radius, 50 mm short of the elevations on every side. The plan circle takes the radius the collar has seen from above, 0.22 m. Run: python3 ops-hydrangea.py"""
import json, math, os

HERE = os.path.dirname(os.path.abspath(__file__))
R, N = 0.22, 24
points = [[round(R * math.cos(2 * math.pi * i / N), 4), round(R * math.sin(2 * math.pi * i / N), 4)] for i in range(N)]
ops = [{"op": "set", "scale": "500", "view": "plan", "id": "root-collar", "bone": "root-plate", "set": {"points": points}}]
out = os.path.join(HERE, "ops", "hydrangea.json")
open(out, "w").write("[\n" + ",\n".join("  " + json.dumps(op) for op in ops) + "\n]\n")
print(out, len(ops), "ops")
