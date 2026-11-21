"""🛴 Round 5 handcraft of humans/child-on-kick-scooter, every row stated by hand on top of the round 4 source: the knee rebuilt as a real joint (the thigh end tapers inside the shank's condyles past the joint line, the shank's top tapers inside the thigh above it, both 42 mm in radius at the joint, an 8-year-old's knee breadth of about 75 mm plus the leggings) at 1:50 and 1:100, so no ball stands proud of the leg in any pose. Reads `🗑️generated/r5-child-on-kick-scooter/volumes.before.json` (the round 4 source), writes `volumes/humans/child-on-kick-scooter.json`."""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("g11", os.path.join(T, "r4-g11-kit.py"))
G = importlib.util.module_from_spec(spec)
spec.loader.exec_module(G)

SRC = os.path.join(T, "🗑️generated", "r5-child-on-kick-scooter", "volumes.before.json")
OUT = os.path.join(T, "volumes", "humans", "child-on-kick-scooter.json")
V = json.load(open(SRC, encoding="utf-8"))["volumes"]

G.edit(V, "50", "thigh-g", 0.27, 1.0, [(0.280, 0.005, 0.0, 0.045, 0.045), (0.302, 0.004, 0.0, 0.042, 0.042), (0.316, 0.002, 0.0, 0.035, 0.035), (0.328, 0.000, 0.0, 0.018, 0.018)])
G.edit(V, "50", "shank-g", -1.0, 0.031, [(-0.032, 0.004, 0.0, 0.013, 0.013), (-0.020, 0.004, 0.0, 0.028, 0.029), (-0.006, 0.003, 0.0, 0.038, 0.039), (0.008, 0.001, 0.0, 0.042, 0.043), (0.030, 0.000, 0.0, 0.041, 0.042)])
G.edit(V, "100", "thigh-g", 0.27, 1.0, [(0.280, 0.005, 0.0, 0.045, 0.045), (0.302, 0.004, 0.0, 0.042, 0.042), (0.322, 0.001, 0.0, 0.030, 0.030), (0.332, 0.000, 0.0, 0.016, 0.016)])
G.edit(V, "100", "shank-g", -1.0, 0.031, [(-0.030, 0.004, 0.0, 0.016, 0.016), (-0.012, 0.003, 0.0, 0.035, 0.036), (0.008, 0.001, 0.0, 0.042, 0.043), (0.030, 0.000, 0.0, 0.041, 0.042)])

json.dump({"volumes": V}, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, separators=(",", ":"))
print(OUT)
