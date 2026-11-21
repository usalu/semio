"""🚴 Round 5 remodel of humans/cyclist, every row stated by hand: the softshell jacket no longer pinches into the lumbar hollow (a loose jacket bridges it: back 0.062 to 0.074 m behind the trunk axis between L5 and T12 instead of 0.048 to 0.068), so the hem over the seat reads as a jacket and not as a peplum, at every scale; the nested thermal shells follow (`r4-g11-kit.edit`). Reads and writes the volumes source `volumes/humans/cyclist.json` (the round 4 state is the base).

Usage: python3 r5-cyclist-remodel.py
"""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("g11", os.path.join(T, "r4-g11-kit.py"))
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)

SOURCE = os.path.join(T, "volumes", "humans", "cyclist.json")
V = json.load(open(SOURCE, encoding="utf-8"))["volumes"]

G.edit(V, "50", "jacket", -0.02, 0.16, [(-0.010, -0.022, 0.0, 0.158, 0.096, 2.3), (0.040, -0.023, 0.0, 0.140, 0.089, 2.3), (0.090, -0.025, 0.0, 0.132, 0.087, 2.3), (0.150, -0.030, 0.0, 0.140, 0.094, 2.3)], sides=None)
G.edit(V, "100", "jacket", 0.0, 0.1, [(0.020, -0.022, 0.0, 0.150, 0.093, 2.3), (0.080, -0.024, 0.0, 0.134, 0.088, 2.3)], sides=None)
G.edit(V, "200", "jacket", 0.0, 0.1, [(0.060, -0.023, 0.0, 0.136, 0.089, 2.3)], sides=None)
G.edit(V, "500", "jacket", 0.0, 0.1, [(0.080, -0.023, 0.0, 0.136, 0.089, 2.3)], sides=None)
print(G.save("cyclist", V))
