"""Probe how the best bearing moves with the window size and the overhang (scene agent, round 2)."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
study = model.orientation_study()
design = study["design"]
for wwr in (.1, .2, .3, .45, .6, .7, .8):
    table = {t: model.evaluate({**design, "wwr": wwr, "orientation": float(t)})["electricity"] for t in range(90, 451)}
    minima = [t for t in range(91, 450) if table[t] < table[t - 1] and table[t] <= table[t + 1]]
    feasible = min(range(270, 451), key=table.get)
    print("[DEBUG] wwr", wwr, "global", min(table, key=table.get), round(min(table.values())), "minima", [(t, round(table[t])) for t in minima], "feasible", feasible, round(table[feasible]), "E360", round(table[360]), "E180", round(table[180]))
for overhang in (0, .6, 1.2):
    table = {t: model.evaluate({**design, "overhang": overhang, "orientation": float(t)})["electricity"] for t in range(90, 451)}
    minima = [t for t in range(91, 450) if table[t] < table[t - 1] and table[t] <= table[t + 1]]
    print("[DEBUG] overhang", overhang, "minima", [(t, round(table[t])) for t in minima], "feasible", min(range(270, 451), key=table.get))
r = model.evaluate({**design, "orientation": 180.0})
print("[DEBUG] 180", {k: round(v, 3) for k, v in r.items()})
print("[DEBUG] compass", [(t, round(model.evaluate({**design, "orientation": float(t)})["electricity"], 1)) for t in (90, 135, 180, 225, 270, 315, 360, 405, 450)])
print("[DEBUG] cum cooling", [(t, round(model.evaluate({**design, "orientation": float(t)})["cooling"]), round(model.evaluate({**design, "orientation": float(t)})["heating"])) for t in range(90, 451, 15)])
