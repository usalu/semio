import importlib.util, json
CLIP = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m
old = load("🗑️generated/model-r3/model-before-run.py", "old")
new = load(CLIP + "/🧮️model/🐍️.py", "new")
NEW = {"conductance", "heating_losses", "heating_gains", "heating_eta", "heating_used", "cop", "cooling_gains", "cooling_removable", "cooling_eta", "cooling_removed", "eer",
       "internal_winter", "internal_summer", "beam_winter", "beam_summer", "direct_winter", "direct_summer", "lighting_power", "lighting_hours", "lamps_on"}
def strip(x):
    if isinstance(x, dict):
        return {k: strip(v) for k, v in x.items() if k not in NEW}
    if isinstance(x, list):
        return [strip(v) for v in x]
    return x
calls = {"evaluate": lambda m: m.evaluate(), "orientation_study": lambda m: m.orientation_study(), "surface": lambda m: m.surface(41),
         "grid": lambda m: m.samples("grid"), "random": lambda m: m.samples("random"), "lhs": lambda m: m.samples("lhs"), "factorial": lambda m: m.factorial(),
         "fractional": lambda m: m.fractional(), "variants": lambda m: m.variants(True), "pareto": lambda m: m.pareto(), "gradient": lambda m: m.gradient_path(),
         "pattern": lambda m: m.pattern_path(), "genetic": lambda m: m.genetic(), "convergence": lambda m: m.convergence(),
         "index": lambda m: m.index({"electricity": .4, "cost": .3, "daylight": .2, "carbon": .1}), "diag": lambda m: m.sampling_diagnostics("random")}
for name, call in calls.items():
    a, b = json.dumps(strip(call(old)), sort_keys=True), json.dumps(strip(call(new)), sort_keys=True)
    print(name, "identical" if a == b else "DIFFERENT")
