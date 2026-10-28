"""🔎 Check that every claim and number in the experiments narration matches the model."""
import ast
import importlib
import re
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
tree = ast.parse((CLIP / "🧪️experiments/🐍️.py").read_text(encoding="utf-8"))
scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat3_FactorialExperiments")
narration = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in node.targets))
factorial, fractional = model.factorial(), model.fractional()
runs = factorial["runs"]
r1, r2, r3, r4 = (run["response"] for run in runs)
failures = []


def check(condition, message):
    print(("ok   " if condition else "FAIL ") + message)
    if not condition:
        failures.append(message)


levels = {factor["id"]: factor["levels"] for factor in factorial["factors"]}
centre = factorial["centre"]
answer_numbers = {str(round(r3 - r4)), str(round(r1 - r2))}
allowed = {"30", "70", "50", "0", "0.5", "0,5", "1.0", "1,0", "1", "2", "4", "8", "100"} | answer_numbers
allowed_numbers = {str(int(round(levels["wwr"][0] * 100))), str(int(round(levels["wwr"][1] * 100))), str(int(round(centre["design"]["wwr"] * 100)))}
check(allowed_numbers == {"30", "70", "50"}, f"window levels {sorted(allowed_numbers)} and centre 50 % match the narration")
check(levels["overhang"] == [0, 1] and centre["design"]["overhang"] == .5, "overhang levels 0 and 1 m and centre 0.5 m match the narration")
check([run["coded"] for run in runs] == [[-1, -1], [-1, 1], [1, -1], [1, 1]], "run order (A-,B-),(A-,B+),(A+,B-),(A+,B+)")
saving_small, saving_large = r1 - r2, r3 - r4
check(saving_small > 0 and saving_large > saving_small, f"saving_30 < saving_70: {saving_small:.1f} < {saving_large:.1f} kWh/a (little sun to block, saves more)")
check(runs[0]["solar_summer"] < runs[2]["solar_summer"], "far more sun arrives at 70 % than at 30 % without overhang")
check(factorial["effects"]["ab"] < 0 and abs(factorial["effects"]["ab"]) > .2 * abs(factorial["effects"]["b"]), "non-parallel lines: interaction effect is clear")
check(abs(factorial["effects"]["ab"] - .5 * ((r4 - r3) - (r2 - r1))) < 1e-9, "interaction effect is half the difference of the two savings")
check(abs(factorial["effects"]["a"] - ((r3 + r4) / 2 - (r1 + r2) / 2)) < 1e-9, "main effect A is the mean at 70 minus the mean at 30 percent")
check(abs(factorial["effects"]["b"] - ((r2 + r4) / 2 - (r1 + r3) / 2)) < 1e-9, "main effect B is the mean with overhang minus the mean without")
check(abs(factorial["coefficients"][0] - sum((r1, r2, r3, r4)) / 4) < 1e-9, "setting A = B = 0 predicts the mean of the four runs")
check(abs(centre["predicted"] - factorial["coefficients"][0]) < 1e-9 and centre["actual"] < centre["predicted"], f"the model predicts a higher cooling demand than the room needs: {centre['predicted']:.1f} > {centre['actual']:.1f}")
check(abs(centre["curvature"]) >= .08, f"curvature {centre['curvature'] * 100:.1f} % is at least 8 %")
check(factorial["centre_savings"]["share"] > .5, f"the first half metre delivers most of the saving: {factorial['centre_savings']['share'] * 100:.0f} %")
check(len(fractional["runs"]) == 4 and all(run["coded"][2] == run["coded"][0] * run["coded"][1] for run in fractional["runs"]), "fraction: four runs with C = AB")
check(abs(fractional["alias_contrast"] - fractional["true"]["sum"]) < 1e-6, "fraction: the one contrast measures A·B + C together")
check(len(fractional["full"]) == 8, "full plan needs eight runs")
q_small = (runs[0]["solar_summer"] - runs[1]["solar_summer"]) / (runs[1]["shading_summer"] * runs[0]["window_area"] / 8)
q_large = (runs[2]["solar_summer"] - runs[3]["solar_summer"]) / (runs[3]["shading_summer"] * runs[2]["window_area"] / 8)
check(abs(q_small - q_large) / q_large < .01, f"one ray = one fixed amount of sun: {q_small * .3:.1f} kWh/a (30 %) = {q_large * .3:.1f} kWh/a (70 %) per 0.3 m of glass height")
check(all(abs((run["solar_summer"] - other["solar_summer"]) / (run["cooling_demand"] - other["cooling_demand"]) - 1) < 2e-3 and round((run["solar_summer"] - other["solar_summer"]) / 3) == round(run["response"] - other["response"]) for run, other in ((runs[0], runs[1]), (runs[2], runs[3]))), "blocked sun ÷ EER = cooling electricity saved (to the shown kWh/a)")
check(all(abs(run["cooling_demand"] / run["response"] - 3) < 1e-9 for run in runs), "cooling electricity = cooling demand ÷ EER 3")
check(round(runs[0]["window_area"] / 8 / .3) == 3 and round(runs[2]["window_area"] / 8 / .3) == 7, "30 % window catches 3 rays, 70 % window 7 rays")
text = " ".join(row[1] for row in narration) + " " + " ".join(row[2] for row in narration)
digits = set(re.findall(r"\d+(?:[.,]\d+)?", text))
check(digits <= allowed, f"narration digits {sorted(digits)} are design inputs only")
check(len(narration) == 35 and len({row[0] for row in narration}) == 35, "35 distinct clauses")
closing = next(row for row in narration if row[0] == "answer")
check(all(number in closing[1] and number in closing[2] for number in answer_numbers) and answer_numbers == {"279", "122"}, f"closing answer names the model savings {sorted(answer_numbers)} kWh/a at 70 % and 30 %")
check(all(len(row[1]) <= 100 and len(row[2]) <= 105 for row in narration), "clauses stay short: " + ", ".join(f"{row[0]}={max(len(row[1]), len(row[2]))}" for row in narration if max(len(row[1]), len(row[2])) > 95))
print("[DEBUG] failures:", len(failures))
sys.exit(1 if failures else 0)
