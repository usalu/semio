import importlib
import json
import math
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m = model.load_model()
g = model.building_geometry(m)
car = model._car(m)
p = model.car_path(m, samples=1201)
poses = p["poses"]
best = []
for q in g["pilotis"]:
    d = min((model._pose_distance(pose, car, (q["x"], q["y"]), q["radius"]), i) for i, pose in enumerate(poses))
    best.append((round(d[0], 4), q["tag"], d[1] / 1200, [round(v, 3) for v in poses[d[1]]]))
best.sort()
for row in best[:8]:
    print("[DEBUG] near", row)
print("[DEBUG] gap_progress", p["gap_progress"], "crossings", json.dumps([{k: c[k] for k in c} for c in p["crossings"]])[:900])
print("[DEBUG] entry pose", poses[int(p["gap_progress"] * 1200)])
