"""🧮 Round 2 check of the experiments clip: narration keys and bilingual numbers, on-screen strings recomputed from the model, and printed arithmetic evaluated literally."""

import ast
import importlib.util
import re
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
sys.dont_write_bytecode = True

TICKET = Path(__file__).resolve().parent
CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
SCENE = CLIP / "🧪️experiments" / "🐍️.py"
FRAMES = TICKET / "🗑️generated" / "frames" / "Beat3_FactorialExperiments"
SPEC = importlib.util.spec_from_file_location("experiments_model_verify", CLIP / "🧮️model" / "🐍️.py")
MODEL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODEL)
RESULTS = []


def verdict(ok, where, detail):
    """✅ Record and print one check result."""
    RESULTS.append(ok)
    print(f"{'OK' if ok else 'MISMATCH':<9} {where:<52} {detail}")


def close(a, b, tol=0.051):
    """📏 Whether two rounded printed values agree within one printed unit of the last digit."""
    return abs(a - b) <= tol


tree = ast.parse(SCENE.read_text(encoding="utf-8"))
narration = None
for node in ast.walk(tree):
    if isinstance(node, ast.Assign) and any(getattr(t, "id", "") == "NARRATION" for t in node.targets):
        narration = ast.literal_eval(node.value)
keys = [row[0] for row in narration]
print(f"narration rows {len(narration)}, unique keys {len(set(keys))}")
verdict(len(narration) == len(set(keys)) and all(row[1] and row[2] for row in narration), "narration: unique keys, en and de non-empty", f"{len(narration)} rows")
for key in keys:
    for lang in ("en", "de"):
        frame = FRAMES / f"Beat3_FactorialExperiments-{lang}-{key}.png"
        if not frame.exists():
            verdict(False, f"frame exists {lang}-{key}", "missing")
verdict(all((FRAMES / f"Beat3_FactorialExperiments-{lang}-{key}.png").exists() for key in keys for lang in ("en", "de")), "frames exist for every key in en and de", f"{len(keys)} keys x 2")
for key, en, de in narration:
    en_numbers = sorted(re.findall(r"\d+(?:[.,]\d+)?", en))
    de_numbers = sorted(re.findall(r"\d+(?:[.,]\d+)?", de))
    same = [n.replace(",", ".") for n in en_numbers] == [n.replace(",", ".") for n in de_numbers]
    decimal_point_in_de = re.findall(r"\d\.\d", de)
    if not same or decimal_point_in_de:
        verdict(False, f"narration numbers en vs de: {key}", f"en {en_numbers} de {de_numbers} decimal points in de {decimal_point_in_de}")
verdict(True, "narration numbers match in en and de (per row)", "checked all rows, decimal comma in de")

study = MODEL.load_study()["parameters"]
EER = study["systems"]["eer"]["value"]
RUNS = MODEL.factorial()["runs"]
FACTORIAL = MODEL.factorial()
FRACTION = MODEL.fractional()
shown = [round(run["response"], 1) for run in RUNS]
ray = RUNS[2]["solar_summer"] / round(RUNS[2]["window_area"] / study["room"]["width"]["value"] / .3)
direct = ray * study["climate"]["direct_share_summer"]["value"]
verdict(round(direct) == 122 and round(ray) - round(direct) == 82, "legend: 1 ray = 204 = 122 direct + 82 diffuse", f"ray {ray:.1f} direct {direct:.1f}")
stopped70 = RUNS[2]["solar_summer"] - RUNS[3]["solar_summer"]
stopped30 = RUNS[0]["solar_summer"] - RUNS[1]["solar_summer"]
verdict(round(7 * direct * RUNS[3]["shading_summer"]) == round(stopped70), "saving_70 chain: 7 x 122.4 x 0.977 = 837", f"printed 7*122.4*0.977 = {7 * 122.4 * 0.977:.2f}, model {stopped70:.2f}")
verdict(round(3 * direct * RUNS[1]["shading_summer"]) == round(stopped30), "saving_30 chain: 3 x 122.4 x 1.000 = 367", f"printed {3 * 122.4 * 1.0:.2f}, model {stopped30:.2f}")
load70 = (round(RUNS[2]["cooling_demand"], 1), round(RUNS[3]["cooling_demand"], 1))
load30 = (round(RUNS[0]["cooling_demand"], 1), round(RUNS[1]["cooling_demand"], 1))
verdict(close(load70[0], 1700.0) and close(load70[1], 863.3) and close(load70[0] - load70[1], 836.7), "saving_70 cooling load 1,700.0 -> 863.3 (-836.7)", f"model {load70}")
verdict(close(load30[0], 938.4) and close(load30[1], 571.5) and close(load30[0] - load30[1], 366.9), "saving_30 cooling load 938.4 -> 571.5 (-366.9)", f"model {load30}")
verdict(close((load70[0] - load70[1]) / EER, 278.9, .06) and close(shown[2] - shown[3], 278.9), "70 % chain: 836.7 / 3.0 = 278.9 = 566.7 - 287.8", f"{(load70[0] - load70[1]) / EER:.2f}")
verdict(close((load30[0] - load30[1]) / EER, 122.3, .06) and close(shown[0] - shown[1], 122.3), "30 % chain: 366.9 / 3.0 = 122.3 = 312.8 - 190.5", f"{(load30[0] - load30[1]) / EER:.2f}")
verdict(round(stopped70 / stopped30, 1) == 2.3 and round(7 / 3, 1) == 2.3 and round((shown[2] - shown[3]) / (shown[0] - shown[1]), 1) == 2.3, "ratio chain 7/3, 837/367, 278.9/122.3 all 2.3", f"{stopped70 / stopped30:.3f} {(shown[2] - shown[3]) / (shown[0] - shown[1]):.3f}")
mean70, mean30 = (shown[2] + shown[3]) / 2, (shown[0] + shown[1]) / 2
mean1m, mean0m = (shown[1] + shown[3]) / 2, (shown[0] + shown[2]) / 2
verdict(close(mean70 - mean30, 175.6, .06) and abs(mean70 - mean30 - 175.6) < .06, "main A: (566.7+287.8)/2 - (312.8+190.5)/2 = 175.6", f"{mean70:.2f} - {mean30:.2f} = {mean70 - mean30:.2f}")
verdict(abs((mean1m - mean0m) - (-200.6)) < .06, "main B: (190.5+287.8)/2 - (312.8+566.7)/2 = -200.6", f"{mean1m:.2f} - {mean0m:.2f} = {mean1m - mean0m:.2f}")
effect_70, effect_30 = shown[3] - shown[2], shown[1] - shown[0]
diff_70_30 = -278.9 - (-122.3)
verdict(abs(diff_70_30 - (-156.6)) < .06 and abs(0.5 * diff_70_30 - (-78.3)) < .06, "difference (three rows): -278.9 - (-122.3) = -156.6 and 1/2 x (-156.6) = -78.3", f"{diff_70_30:.2f} and {0.5 * diff_70_30:.2f}")
verdict(abs(0.5 * (effect_70 - effect_30) - (-78.3)) < .06, "difference_2: 1/2 x (-278.9 - (-122.3)) = -78.3", f"{0.5 * (effect_70 - effect_30):.2f}")
interaction_gap = (shown[2] - shown[0] + shown[1]) - shown[3]
verdict(close(interaction_gap, 156.6, .06), "interaction bracket: 444.4 - 287.8 = 156.6", f"{interaction_gap:.2f}")
coef = FACTORIAL["coefficients"]
verdict(round(coef[0]) == 339 and round(coef[1]) == 88 and round(coef[2]) == -100 and round(coef[3]) == -39 and round(sum(coef)) == 288, "model: 339 + 88 - 100 - 39 = 288", f"{coef}")
verdict(round(FACTORIAL["effects"]["ab"] / 2, 1) == -39.2 and round(FACTORIAL["effects"]["a"] / 2, 1) == 87.8, "model_3: effect / 2 = coefficient (87.8, -100.3, -39.2)", f"{FACTORIAL['effects']['ab'] / 2:.3f}")
centre = FACTORIAL["centre"]
verdict(close(centre["predicted"], 339.4) and close(centre["actual"], 298.3) and close(centre["actual"] - centre["predicted"], -41.1, .06), "confirm: 339.4 predicted, 298.3 simulated, gap -41.1", f"{centre['predicted']:.2f} {centre['actual']:.2f}")
verdict(close(centre["curvature"] * 100, -12.1, .06), "confirm_2: -12.1 %", f"{centre['curvature'] * 100:.2f}")
load_centre = MODEL.evaluate({**centre["design"]})["cooling_demand"]
verdict(close(load_centre / EER, 298.3, .06) and close(load_centre, 894.9, .06), "confirm chain: 894.9 / 3.0 = 298.3", f"{load_centre:.2f} / {EER} = {load_centre / EER:.2f}")
curve = {x: MODEL.evaluate({**centre["design"], "overhang": x})["cooling"] for x in (0.0, 0.5, 1.0)}
half, full = curve[0.0] - curve[0.5], curve[0.0] - curve[1.0]
verdict(abs(half - 141.4) < .06 and abs(full - 203.9) < .06 and round(100 * half / full) == 69, "confirm_3: 141.4 of 203.9 kWh/a = 69 %", f"{half:.2f} / {full:.2f} = {100 * half / full:.2f} %")
contrast, truth = FRACTION["alias_contrast"], FRACTION["true"]
verdict(close(contrast, -72.6, .06) and close(truth["ab"], -78.3, .06) and close(truth["c"], 5.7, .06) and close(abs(contrast - truth["ab"]), 5.7, .06), "purpose_4: contrast -72.6, A.B -78.3, C +5.7, error 5.7", f"{contrast:.2f} {truth['ab']:.2f} {truth['c']:.2f}")
fraction = [round(run["response"], 1) for run in FRACTION["runs"]]
verdict(fraction == [316.5, 186.3, 564.8, 289.4], "purpose_2 fraction runs 316.5, 186.3, 564.8, 289.4", f"{fraction}")
thin = MODEL.evaluate({**centre["design"], "wwr": .7, "overhang": 1.0, "insulation": .1})
thick = MODEL.evaluate({**centre["design"], "wwr": .7, "overhang": 1.0, "insulation": .3})
verdict(round(thin["u_wall"], 2) == .29 and round(thick["u_wall"], 2) == .11, "purpose_4 reason: U-wall 0.29 -> 0.11", f"{thin['u_wall']:.4f} {thick['u_wall']:.4f}")
verdict(close(thin["cooling"], 285.9) and close(thick["cooling"], 289.4), "purpose_4 reason: cooling 285.9 -> 289.4 at 70 %, 1.0 m", f"{thin['cooling']:.2f} {thick['cooling']:.2f}")
verdict(close(shown[3], 287.8) and close(shown[1], 190.5) and close(shown[2] - shown[3], 278.9) and close(shown[0] - shown[1], 122.3), "answer: 287.8 / 190.5 kWh/a and savings 278.9 / 122.3", f"{shown}")
verdict(close(MODEL.evaluate({**centre["design"], "overhang": 0.5})["cooling"], 298.3, .06) and round(MODEL.evaluate({**centre["design"], "overhang": 1.0})["cooling"], 1) == 235.8, "curve values 298.3 at 0.5 m, 235.8 at 1.0 m", f"{MODEL.evaluate({**centre['design'], 'overhang': 1.0})['cooling']:.2f}")
verdict(close(sum(shown) / 4, 339.4, .06), "model_2: mean of the four simulations = 339.4", f"{sum(shown) / 4:.3f}")
print(f"\nmismatching checks: {RESULTS.count(False)} of {len(RESULTS)}")
