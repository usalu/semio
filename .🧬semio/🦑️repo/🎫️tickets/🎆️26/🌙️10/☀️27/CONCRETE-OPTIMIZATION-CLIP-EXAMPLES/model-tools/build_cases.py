import importlib.util
import json
from pathlib import Path

clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
spec = importlib.util.spec_from_file_location("model", clip / "🧮️model/🐍️.py")
model = importlib.util.module_from_spec(spec)
spec.loader.exec_module(model)
cases_path = clip / "🧪️cases/🔣️.json"
old = json.loads(cases_path.read_text(encoding="utf-8"))
keep = {key: old[key] for key in ("version", "languages", "audiences", "chapters")}


def r(value, digits=6):
    return round(value, digits)


def rows(items, keys):
    return [{key: r(item[key]) for key in keys} for item in items]


base = model.evaluate()
orientation = model.orientation_study()
at = lambda angle: model.evaluate({**orientation["design"], "orientation": angle})["electricity"]
surface = model.surface()
grid, random, lhs = (model.samples(kind) for kind in ("grid", "random", "lhs"))
factorial, fractional = model.factorial(), model.fractional()
study = model.load_study()
weights = study["weights"]
variants = model.variants(extra=True)
pareto = model.pareto()
gradient, pattern, genetic, convergence = model.gradient_path(), model.pattern_path(), model.genetic(), model.convergence()
diagnostics = model.sampling_diagnostics()

cases = {
    **keep,
    "baseline": {key: r(value) for key, value in base.items()},
    "ranges": {"electricity": [300, 2500], "intensity": [10, 50], "cost": [10000, 60000]},
    "orientation": {
        "local_minima": orientation["local_minima"], "local_maxima": orientation["local_maxima"],
        "global_minimum": orientation["global_minimum"], "feasible_minimum": orientation["feasible_minimum"],
        "reference": orientation["reference"], "constraint": orientation["constraint"], "samples": len(orientation["samples"]),
        "values": {"south": r(at(180)), "north": r(at(360)), "east": r(at(90)), "west": r(at(orientation["local_maxima"][0])),
                   "north_penalty": r(at(360) / at(180) - 1)},
        "minimum_penalty": 0.15, "start": orientation["start"], "descent": [item["theta"] for item in orientation["descent"]],
        "tariff": orientation["tariff"], "savings_north_chf": r(orientation["savings"][360 - 90][2]),
        "sun": {key: orientation["sun"][key] for key in ("winter_altitude", "summer_altitude", "low_summer_altitude", "winter_azimuth", "summer_azimuth")},
    },
    "surface": {"minimum": {"wwr": r(surface["minimum"]["design"]["wwr"]), "overhang": r(surface["minimum"]["design"]["overhang"]), "electricity": r(surface["minimum"]["electricity"])},
                "corners": {"low_low": r(surface["electricity"][0][0]), "high_low": r(surface["electricity"][-1][0]),
                            "low_high": r(surface["electricity"][0][-1]), "high_high": r(surface["electricity"][-1][-1])}},
    "sampling": {"counts": {"grid": len(grid), "random": len(random), "lhs": len(lhs)},
                 "electricity": {"grid": [r(item["electricity"]) for item in grid], "random": [r(item["electricity"]) for item in random], "lhs": [r(item["electricity"]) for item in lhs]},
                 "empty_corner": {"wwr_above": 0.55, "overhang_above": 0.8}, "closest": {"wwr_below": 0.05, "overhang_below": 0.1},
                 "diagnostics": {"largest_empty_box": {key: [r(value) for value in values] for key, values in diagnostics["largest_empty_box"].items()},
                                 "closest_pair": [diagnostics["closest_pair"][0], diagnostics["closest_pair"][1], r(diagnostics["closest_pair"][2])]}},
    "factorial": {"responses": [r(run["response"]) for run in factorial["runs"]], "effects": {key: r(value) for key, value in factorial["effects"].items()},
                  "coefficients": [r(value) for value in factorial["coefficients"]], "centre": {"predicted": r(factorial["centre"]["predicted"]), "actual": r(factorial["centre"]["actual"])},
                  "minimum_curvature": 0.08, "minimum_saving_ratio": 1.5, "half_share": r(factorial["centre_savings"]["share"])},
    "fractional": {"responses": [r(run["response"]) for run in fractional["runs"]], "alias_contrast": r(fractional["alias_contrast"]), "true": {key: r(value) for key, value in fractional["true"].items()}},
    "index": {"balanced": weights["balanced"], "cost_heavy": weights["cost_heavy"],
              "winner": {name: min(model.index(weights[name]), key=lambda row: row["score"])["id"] for name in weights},
              "scores_balanced": [r(row["score"]) for row in model.index(weights["balanced"])],
              "order_five": [row["id"] for row in sorted(model.index(weights["balanced"]), key=lambda row: row["rank"])],
              "order_six": [row["id"] for row in sorted(model.index(weights["balanced"], ids=[item["id"] for item in variants]), key=lambda row: row["rank"])],
              "variants": [{"id": item["id"], **{key: r(item[key]) for key in ("intensity", "cost", "daylight", "carbon")}} for item in variants]},
    "pareto": {"count": len(pareto["designs"]), "front": pareto["front"], "front3_count": len(pareto["front3"]), "dominated_example": pareto["dominated_example"], "knee": pareto["knee"],
               "steps": [{"from": step["from"], "to": step["to"], "cost": r(step["cost"]), "saved": r(step["saved"]), "payback": r(step["payback"])} for step in pareto["steps"]],
               "refinement": {"member": pareto["refinement"]["member"], "dominates": pareto["refinement"]["dominates"]}, "third_example": pareto["third_example"], "picks": pareto["picks"]},
    "algorithms": {"gradient": {"length": len(gradient), "first": r(gradient[0]["electricity"]), "last": r(gradient[-1]["electricity"]), "evaluations": gradient[-1]["evaluations"]},
                   "pattern": {"length": len(pattern), "first": r(pattern[0]["electricity"]), "last": r(pattern[-1]["electricity"]), "evaluations": pattern[-1]["evaluations"], "halvings": sum(not item["moved"] for item in pattern[1:])},
                   "genetic": {"best": [r(entry["best"]["electricity"]) for entry in genetic["generations"]], "evaluations": [entry["evaluations"] for entry in genetic["generations"]]},
                   "convergence": {key: r(rows_[-1][1]) for key, rows_ in convergence["series"].items()}, "budget": convergence["budget"], "map_minimum": r(convergence["map_minimum"])},
    "invalidModels": [
        {"id": "missing-german", "path": ["title", "de"], "remove": True},
        {"id": "invalid-language", "path": ["languages"], "value": ["en"]},
        {"id": "empty-variants", "path": ["variants"], "value": []},
        {"id": "unknown-direction", "path": ["criteria", 0, "direction"], "value": "lower"},
        {"id": "invalid-random-draw", "path": ["sampling", "random", 0], "value": [0.2]},
        {"id": "unknown-root-key", "path": ["legacy"], "value": 1},
        {"id": "missing-unit", "path": ["parameters", "climate", "heating_degree_hours", "unit"], "remove": True},
        {"id": "missing-note", "path": ["parameters", "envelope", "u_window", "note"], "remove": True},
        {"id": "text-for-number", "path": ["parameters", "room", "width", "value"], "value": "eight"},
        {"id": "duplicate-permutation", "path": ["sampling", "latin_permutation"], "value": [0, 0, 1]},
        {"id": "range-as-text", "path": ["ranges", "wwr", "min"], "value": "ten"},
        {"id": "unknown-response", "path": ["factorial", "response"], "value": "noise"},
    ],
}
cases_path.write_text(json.dumps(cases, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(len(json.dumps(cases)))
