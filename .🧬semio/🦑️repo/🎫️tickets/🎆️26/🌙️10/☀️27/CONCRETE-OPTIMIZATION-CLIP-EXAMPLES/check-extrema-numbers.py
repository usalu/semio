"""🧮 Compare the extrema narration with the model: bearings, orderings and physical claims."""

import ast
import importlib
import re
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
sys.stdout.reconfigure(encoding="utf-8")
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
tree = ast.parse((CLIP / "📉️extrema/🐍️.py").read_text(encoding="utf-8"))
scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat1_LocalGlobalExtrema")
rows = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and node.targets[0].id == "NARRATION")
study = model.orientation_study()
fixed = study["design"]
at = lambda theta: model.evaluate({**fixed, "orientation": float(theta)})
failures = []


def expect(condition, message):
    if not condition:
        failures.append(message)
    print("[DEBUG]", "ok  " if condition else "FAIL", message)


expected = {
    "formulate_2": {90, 180, 270, 360},
    "domain": {study["reference"]},
    "sweep_5": {study["range"][1]},
    "local": {study["start"]},
    "local_2": {study["local_minima"][1]},
    "global": {study["global_minimum"]},
    "argument": {study["global_minimum"]},
    "negate_2": {study["global_minimum"]},
    "compare_2": {study["start"]},
    "feasible": {study["feasible_minimum"]},
    "scope_2": set(study["constraint"]) | {study["feasible_minimum"]},
    "constraint": {90},
    "domain_2": set(study["range"]),
    "sweep_3": {study["local_maxima"][0]},
}
for key, en, de in rows:
    found = {language: {int(n) for n in re.findall(r"(\d+)\s?(?:°|degrees|Grad)", text)} for language, text in (("en", en), ("de", de))}
    want = expected.get(key, set())
    expect(found["en"] == want and found["de"] == want, f"{key}: bearings en {sorted(found['en'])} de {sorted(found['de'])} == model {sorted(want)}")
    digits = {language: re.findall(r"\d+(?:[.,]\d+)?", re.sub(r"(\d+)\s?(?:°|degrees|Grad)", "", text)) for language, text in (("en", en), ("de", de))}
    expect(not any(digits.values()), f"{key}: no other numbers in the clause {digits}")

south, north, east, west = at(180), at(360), at(90), at(270)
expect(study["global_minimum"] == 180 and study["local_minima"] == [180, 360], "minima at 180 and 360")
expect(north["electricity"] > south["electricity"], "north worse than south")
expect(study["constraint"] == [270, 450] and study["feasible_minimum"] == 360, "constraint 270-450 selects 360")
expect(study["start"] == 315 and study["descent"][-1]["theta"] == 360, "descent from 315 ends at 360")
expect(at(180)["heating"] < at(90)["heating"], "sweep: heating falls from east to south")
expect(south["cooling"] < west["cooling"], "sweep_2/3: cooling peaks in the west, modest in the south")
expect(west["cooling"] >= .95 * max(at(t)["cooling"] for t in range(90, 451)), "sweep_3: cooling is at its peak in the west")
expect(north["cooling"] < south["cooling"] and north["heating"] > south["heating"], "sweep_4: north has least cooling and more heating")
expect(abs(at(450)["electricity"] - east["electricity"]) < 1e-6, "sweep_5: 450 equals 90")
expect(west["electricity"] >= .98 * max(e for _, e in study["samples"]), "west is at the highest electricity")
peak = max(study["samples"], key=lambda row: row[1])
expect(peak[0] in study["local_maxima"] and study["local_maxima"][0] == 282, f"sweep_3: total peaks at {peak[0]} deg = local maximum {study['local_maxima'][0]}, not at 270")
for angle in (355, 365):
    near = at(angle)
    expect(near["cooling"] - north["cooling"] > north["heating"] - near["heating"] > 0, f"local_2: at {angle} cooling rises ({near['cooling'] - north['cooling']:.2f}) faster than heating falls ({north['heating'] - near['heating']:.2f})")
expect(north["lighting"] > south["lighting"] and north["heating"] > south["heating"], "sweep_4: heating and lighting are higher at north than at south")
expect(north["heating"] - south["heating"] > south["cooling"] - north["cooling"] > 0, "global_2: winter sun saves more heating (north to south) than the summer sun adds cooling")
best_saving = max(study["savings"], key=lambda row: row[1])
expect(best_saving[0] == study["global_minimum"], "negate_2: highest saving at the minimum")
expect(study["tariff"] > 0 and study["reference_electricity"] == east["electricity"], "reference is the east façade")
sys.exit(1 if failures else 0)
