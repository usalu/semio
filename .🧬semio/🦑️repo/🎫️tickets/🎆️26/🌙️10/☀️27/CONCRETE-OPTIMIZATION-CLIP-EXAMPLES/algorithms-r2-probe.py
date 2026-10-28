"""🔎 Probe for the round 2 algorithms scene: which first-generation children get worse by their mutation, and the stop-rule results."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
ga = model.genetic()
first = ga["generations"][0]
lookup = {item["id"]: item for item in first["population"]}
for child in first["children"]:
    cross = {**lookup[child["from"][0]]["design"], **{key: lookup[child["genes"][key]]["design"][key] for key in ("wwr", "overhang")}}
    crossed = model.evaluate(cross)["electricity"]
    print("[DEBUG]", child["id"], {k: round(v, 3) for k, v in child["mutation"].items()}, "cross", round(crossed, 1), "child", round(child["electricity"], 1), "delta", round(child["electricity"] - crossed, 1))
c = model.convergence()
for name, series in c["series"].items():
    values = [v for _, v in series]
    tol = next((n for n in range(6, 51) if values[n - 6] - values[n - 1] < 1.0), None)
    stag = next((n for n in range(11, 51) if values[n - 11] - values[n - 1] <= 0), None)
    print("[DEBUG]", name, "tolerance", tol, values[tol - 1] if tol else None, "stagnation", stag, values[stag - 1] if stag else None, "final", values[-1])
