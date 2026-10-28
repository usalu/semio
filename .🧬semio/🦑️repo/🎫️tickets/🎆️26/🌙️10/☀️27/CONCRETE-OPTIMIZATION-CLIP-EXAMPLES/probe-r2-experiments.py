"""Probe model numbers for the experiments clip round 2."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
F = model.factorial()
R = F["runs"]
study = model.load_study()["parameters"]
print("eer", study["systems"]["eer"]["value"], "direct", study["climate"]["direct_share_summer"]["value"], "internal", study["climate"]["internal_gains_summer"]["value"], "g", study["envelope"].get("g_value"))
for r in R:
    print(r["id"], r["coded"], "wwr", r["design"]["wwr"], "d", r["design"]["overhang"], "win", round(r["window_area"], 2), "solar", round(r["solar_summer"], 1), "shade", round(r["shading_summer"], 4),
          "cd", round(r["cooling_demand"], 1), "cool", round(r["response"], 2), "day", round(r["daylight"], 2), "aut", round(r["autonomy"], 3), "light", round(r["lighting"], 1), "elec", round(r["electricity"], 1))
for lo, hi in ((0, 1), (2, 3)):
    a, b = R[lo], R[hi]
    blocked = a["solar_summer"] - b["solar_summer"]
    dd = a["cooling_demand"] - b["cooling_demand"]
    print("pair", lo, hi, "blocked", round(blocked, 2), "share", round(blocked / a["solar_summer"], 4), "d_demand", round(dd, 2), "/EER", round(dd / 3, 3), "saving", round(a["response"] - b["response"], 3),
          "dd/blocked", round(dd / blocked, 4))
print("effects", F["effects"], "coef", F["coefficients"])
print("centre", F["centre"]["predicted"], F["centre"]["actual"], F["centre"]["curvature"], F["centre_savings"])
for d in (0, .25, .5, .75, 1.0):
    x = model.evaluate({**F["centre"]["design"], "overhang": d})
    print("centre curve", d, round(x["cooling"], 2), "shade", round(x["shading_summer"], 3), "solar", round(x["solar_summer"], 1))
for w in (.3, .5, .7):
    for d in (0, .5, 1.0):
        x = model.evaluate({**F["centre"]["design"], "wwr": w, "overhang": d})
        print("wwr", w, "d", d, round(x["cooling"], 1), "solar", round(x["solar_summer"], 1), "shade", round(x["shading_summer"], 3))
print("fraction", [(r["coded"], round(r["response"], 2), r["design"]["wwr"], r["design"]["overhang"], r["design"]["insulation"]) for r in model.fractional()["runs"]])
for t in (.10, .16, .30):
    x = model.evaluate({**F["centre"]["design"], "wwr": .7, "overhang": 1.0, "insulation": t})
    print("ins", t, round(x["cooling"], 2), round(x["cooling_demand"], 1), "u", round(x["u_wall"], 3))
