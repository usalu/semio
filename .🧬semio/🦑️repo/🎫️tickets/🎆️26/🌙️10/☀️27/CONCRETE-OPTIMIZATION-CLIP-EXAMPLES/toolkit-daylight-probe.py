import importlib, sys
from pathlib import Path
T = Path(__file__).resolve().parent
ROOT = next(p for p in T.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
rows = []
for o in range(90, 451, 30):
    for w in range(2, 17):
        for d in range(6):
            r = m.evaluate({"orientation": o, "wwr": w / 20, "overhang": d / 5 * 1.2})
            rows.append((r["autonomy"], r["daylight"], o, w / 20, d / 5 * 1.2))
print("[DEBUG] autonomy", min(rows)[:2], max(rows)[:2])
print("[DEBUG] DF", min(r[1] for r in rows), max(r[1] for r in rows))
for w in (.3, .45, .8):
    for d in (0, 1.2):
        r = m.evaluate({"wwr": w, "overhang": d})
        print("[DEBUG]", w, d, round(r["autonomy"], 3), round(r["daylight"], 2), round(r["lighting"]))
s = m.surface(41)
ev = [m.evaluate({"wwr": a, "overhang": b})["autonomy"] for a in s["wwr"] for b in s["overhang"]]
print("[DEBUG] south surface autonomy", min(ev), max(ev), s["wwr"][0], s["wwr"][-1], s["overhang"][-1])
p = m.pareto()
pv = [m.evaluate(d["design"])["autonomy"] for d in p["designs"]]
print("[DEBUG] pareto", min(pv), max(pv), len(pv))
