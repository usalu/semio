"""Dump the pareto grid of the model for scene design."""
import importlib, sys
from pathlib import Path
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
m = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
p = m.pareto()
f3 = set(p["front3"]); f = set(p["front"])
for d in sorted(p["designs"], key=lambda d: d["cost"]):
    print(f'{d["id"]:12s} cost={d["cost"]:8.1f} el={d["electricity"]:7.1f} h={d["heating"]:6.1f} c={d["cooling"]:6.1f} l={d["lighting"]:6.1f} dl={d["daylight"]:.2f} {"F" if d["id"] in f else " "}{"3" if d["id"] in f3 else " "}')
for k in ("dominated_example","step","knee","refinement","third_example","picks","tariff","tariff_hi","service_life"):
    print(k, p[k])
for s in p["steps"]: print(s)
print({k: v for k, v in p["designs"][0]["design"].items()})
