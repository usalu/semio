"""Probe the extrema model at the story bearings (scene agent, round 2)."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
study = model.orientation_study()
design = study["design"]
for theta in (90, 180, 270, 315, 360, 450):
    r = model.evaluate({**design, "orientation": theta})
    print("[DEBUG]", theta, {k: round(r[k], 3) for k in ("heating", "cooling", "lighting", "electricity", "heating_demand", "cooling_demand", "solar_winter", "solar_summer", "sun_winter", "sun_summer", "daylight", "autonomy")})
    for season in ("winter", "summer"):
        print("[DEBUG]   ", season, model.sun_state(theta, season), [(s["hour"], round(s["bearing"]), round(s["altitude"]), round(s["weight"], 2)) for s in model.sun_samples(theta, season)])
print("[DEBUG] sun", study["sun"])
print("[DEBUG] tariff", study["tariff"], "min", study["global_minimum"], study["feasible_minimum"], study["local_minima"], study["local_maxima"])
print("[DEBUG] descent", [d["theta"] for d in study["descent"]])
