"""Probe model numbers for the sampling round 2 scene."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")


def show(title, design):
    r = design if "electricity" in design else model.evaluate(design)
    print(f"[DEBUG] {title}: wwr {design['wwr']:.3f} d {design['overhang']:.3f} elec {r['electricity']:.2f} heat {r['heating']:.2f} cool {r['cooling']:.2f} light {r['lighting']:.2f} "
          f"hd {r['heating_demand']:.1f} cd {r['cooling_demand']:.1f} aut {r['autonomy']:.4f} df {r['daylight']:.2f} shs {r['shading_summer']:.3f} shw {r['shading_winter']:.3f} "
          f"win {r['window_area']:.2f} cost {r['cost']:.0f}")


show("baseline", model.load_study()["baseline"])
for kind in ("grid", "random", "lhs"):
    items = model.samples(kind)
    best = min(items, key=lambda s: s["electricity"])
    worst = max(items, key=lambda s: s["electricity"])
    show(kind + " best", best)
    show(kind + " worst", worst)
show("optimum", model.surface(41)["minimum"]["design"])
d = model.sampling_diagnostics("random")
print("[DEBUG]", d)
d = model.sampling_diagnostics("lhs")
print("[DEBUG]", d)
for i, s in enumerate(model.samples("random")):
    print("[DEBUG] random", i, round(s["wwr"], 3), round(s["overhang"], 3), round(s["electricity"], 1), round(s["autonomy"], 3), round(s["shading_summer"], 2))
