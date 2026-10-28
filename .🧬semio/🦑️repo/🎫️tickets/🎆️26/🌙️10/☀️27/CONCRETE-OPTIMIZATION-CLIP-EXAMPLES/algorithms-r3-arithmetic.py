"""🔎 Every substituted ledger row of Beat6_EvolutionarySearch must close from the numbers printed in it, for every design the clip puts into the ledger; run with PYTHONUTF8=1."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
room = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🎨️visuals.🏠️room.🐍️")
scene = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧬️algorithms.🐍️")

cop, eer = room.systems()["cop"], room.systems()["eer"]
hours, kilowatt = room.constant("lighting", "hours"), 0.432
half = scene._half
designs = [item["design"] for item in model.gradient_path()] + [item["design"] for item in model.pattern_path()]
designs += [item["design"] for generation in model.genetic()["generations"] for item in generation["population"]]
designs += [child["design"] for generation in model.genetic()["generations"] for child in generation["children"]] + [model.surface(41)["minimum"]["design"]]
designs += [probe["design"] for item in model.gradient_path() for probe in item["probes"]] + [probe["design"] for item in model.pattern_path() for probe in item["probes"]]
problems = 0
for design in designs:
    printed, found = scene._figures(design), room.result(design)
    rows = (("loss - gains = heat", half(printed["loss"] - printed["gains"]), printed["heat_demand"]),
            ("heat / COP", half(printed["heat_demand"] / cop), printed["heat"]), ("cold / EER", half(printed["cold_demand"] / eer), printed["cold"]),
            ("kW * h * share", half(kilowatt * hours * printed["share"] / 100), printed["lamps"]), ("sum of parts", half(printed["heat"] + printed["cold"] + printed["lamps"]), printed["total"]))
    for name, derived, shown in rows:
        if abs(derived - shown) > 1e-9:
            problems += 1
            print(f"PROBLEM {name} wwr {design['wwr']:.3f} d {design['overhang']:.3f}: from the printed numbers {derived}, printed {shown}")
    for name, shown, exact, tolerance in (("heat", printed["heat"], found["heating"], .06), ("cold", printed["cold"], found["cooling"], .06), ("lamps", printed["lamps"], found["lighting"], .06),
                                          ("total", printed["total"], found["electricity"], .16)):
        if abs(shown - exact) > tolerance:
            problems += 1
            print(f"PROBLEM {name} printed {shown} against the model {exact:.3f}")
print(len(designs), "designs;", problems, "problems")
