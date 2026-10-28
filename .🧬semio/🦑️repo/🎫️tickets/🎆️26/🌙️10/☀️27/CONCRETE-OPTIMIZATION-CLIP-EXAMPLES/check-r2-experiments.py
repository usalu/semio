"""🧮 Recompute every number of the experiments clip from the teaching model and print them for comparison with the rendered frames."""

import importlib.util
import math
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding="utf-8")
sys.dont_write_bytecode = True

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
MODEL_FILE = CLIP / "🧮️model" / "🐍️.py"
STUDY_FILE = CLIP / "🧮️model" / "🔣️.json"
SPEC = importlib.util.spec_from_file_location("experiments_model_check", MODEL_FILE)
MODEL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODEL)


def de(value, digits=0):
    """🔢 German number format with thousands dots and decimal comma."""
    text = f"{round(value, digits):,.{digits}f}".replace("-", "−")
    return text.replace(",", "\0").replace(".", ",").replace("\0", ".")


def en(value, digits=0):
    """🔢 English number format with thousands commas and decimal point."""
    return f"{round(value, digits):,.{digits}f}".replace("-", "−")


def show(label, value, digits=0):
    """📋 Print one value in both languages with its exact model value."""
    print(f"{label:<44} en {en(value, digits):>12}   de {de(value, digits):>12}   raw {value!r}")


STUDY = MODEL.load_study()
PARAMETERS = STUDY["parameters"]
FACTORIAL = MODEL.factorial()
FRACTION = MODEL.fractional()
RUNS = FACTORIAL["runs"]
FACADE_WIDTH = PARAMETERS["room"]["width"]["value"]
STEP = 0.3
EER = PARAMETERS["systems"]["eer"]["value"]
COOP = PARAMETERS["systems"]["cop"]["value"]
TARIFF = PARAMETERS["economics"]["tariff"]["value"]

print("== Factorial runs (r1 30%/0 m, r2 30%/1 m, r3 70%/0 m, r4 70%/1 m) ==")
for index, run in enumerate(RUNS, 1):
    print(f"r{index} coded {run['coded']} wwr {run['design']['wwr']} overhang {run['design']['overhang']}")
    show(f"  cooling kWh/a r{index}", run["cooling"], 3)
    show(f"  solar_summer r{index}", run["solar_summer"], 3)
    show(f"  shading_summer r{index}", run["shading_summer"], 4)
    show(f"  window_area r{index}", run["window_area"], 3)
    show(f"  sun profile (deg) r{index}", MODEL.sun_state(180, "summer")["profile"], 2)

print()
print("== Effects, coefficients ==")
for key in ("a", "b", "ab"):
    show(f"effect {key} (kWh/a)", FACTORIAL["effects"][key], 3)
for index, value in enumerate(FACTORIAL["coefficients"]):
    show(f"coefficient {index}", value, 3)

print()
print("== Means used in the effect strips (one decimal, as on screen) ==")
runs = [run["response"] for run in RUNS]
a_high, a_low = (runs[2] + runs[3]) / 2, (runs[0] + runs[1]) / 2
b_high, b_low = (runs[1] + runs[3]) / 2, (runs[0] + runs[2]) / 2
show("mean at 70 % (A high)", a_high, 3)
show("mean at 30 % (A low)", a_low, 3)
show("mean with 1.0 m (B high)", b_high, 3)
show("mean without overhang (B low)", b_low, 3)
show("A effect (exact)", a_high - a_low, 3)
show("B effect (exact)", b_high - b_low, 3)
print("A effect from rounded means:", round(round(a_high, 1) - round(a_low, 1), 1))
print("B effect from rounded means:", round(round(b_high, 1) - round(b_low, 1), 1))

print()
print("== Savings and interaction ==")
small_saving = runs[0] - runs[1]
large_saving = runs[2] - runs[3]
show("saving at 30 % (cooling, kWh/a)", small_saving, 3)
show("saving at 70 % (cooling, kWh/a)", large_saving, 3)
show("ratio saving70 / saving30", large_saving / small_saving, 3)
show("ratio WWR 70 / 30 (window area)", RUNS[2]["window_area"] / RUNS[0]["window_area"], 3)
show("interaction AB = half difference", (small_saving - large_saving) / 2, 3)
show("difference of savings (kWh/a)", large_saving - small_saving, 3)
print("AB from rounded savings:", round((round(small_saving, 1) - round(large_saving, 1)) / 2, 1))

print()
print("== Blocked sun chain at 30 % and 70 % (summer solar, kWh/a) ==")
blocked_small = RUNS[0]["solar_summer"] - RUNS[1]["solar_summer"]
blocked_large = RUNS[2]["solar_summer"] - RUNS[3]["solar_summer"]
show("blocked sun 30 %", blocked_small, 3)
show("blocked sun 70 %", blocked_large, 3)
show("blocked 70 / EER (should equal saving 70)", blocked_large / EER, 3)
show("blocked 30 / EER (should equal saving 30)", blocked_small / EER, 3)
show("EER", EER, 1)

print()
print("== Ray legend (scene RAY_KWH) ==")
ray_kwh = (RUNS[2]["solar_summer"] - RUNS[3]["solar_summer"]) / (RUNS[3]["shading_summer"] * RUNS[3]["window_area"] / FACADE_WIDTH) * STEP
show("RAY_KWH", ray_kwh, 1)
show("window height 70 % (m)", RUNS[3]["window_area"] / FACADE_WIDTH, 2)
show("glass height per ray (m)", STEP, 1)

print()
print("== Model centre run and curvature ==")
CENTRE = FACTORIAL["centre"]
show("centre predicted (bilinear)", CENTRE["predicted"], 3)
show("centre actual (model run)", CENTRE["actual"], 3)
show("curvature (%)", CENTRE["curvature"] * 100, 2)
show("centre actual - predicted", CENTRE["actual"] - CENTRE["predicted"], 3)
show("centre share of saving at 0.5 m", FACTORIAL["centre_savings"]["share"] * 100, 2)

print()
print("== Fraction (C = A·B) ==")
show("alias contrast A·B + C", FRACTION["alias_contrast"], 3)
show("true AB", FRACTION["true"]["ab"], 3)
show("true C", FRACTION["true"]["c"], 3)
show("error from confounding", abs(FRACTION["alias_contrast"] - FRACTION["true"]["ab"]), 3)
for index, run in enumerate(FRACTION["runs"], 1):
    show(f"fraction run {index} cooling", run["response"], 3)

print()
print("== Sun, room and climate facts on screen ==")
SUN = MODEL.sun_state(180, "summer")
show("summer noon profile angle (deg)", SUN["profile"], 2)
show("summer noon altitude (deg)", SUN["altitude"], 2)
show("room width (m)", PARAMETERS["room"]["width"]["value"], 0)
show("room depth (m)", PARAMETERS["room"]["depth"]["value"], 0)
show("room height (m)", PARAMETERS["room"]["height"]["value"], 0)

print()
print("== Curvature: true curve at 50 % WWR against the straight model line ==")
BASE_DESIGN = dict(CENTRE["design"])
for depth in (0.0, 0.5, 1.0):
    found = MODEL.evaluate({**BASE_DESIGN, "overhang": depth})
    show(f"true cooling at 50 % and {depth} m", found["cooling"], 3)
show("straight line end at 0 m (bilinear)", FACTORIAL["coefficients"][0] - FACTORIAL["coefficients"][2], 3)
show("straight line end at 1 m (bilinear)", FACTORIAL["coefficients"][0] + FACTORIAL["coefficients"][2], 3)
show("straight line value at 0.5 m", FACTORIAL["coefficients"][0], 3)
print("centre design", CENTRE["design"])

print()
print("== Ray drawing check at 70 % (scene draws rays every 0.3 m, blocked = round(n * shading)) ==")
SILL_70 = 2.7 - min(RUNS[3]["window_area"] / FACADE_WIDTH, 2.7 - 0.05)
HEIGHTS_70 = []
index = 0
while 2.7 - 0.3 * (index + 0.5) > SILL_70 + 0.05:
    HEIGHTS_70.append(2.7 - 0.3 * (index + 0.5))
    index += 1
BLOCKED_70 = min(len(HEIGHTS_70), round(len(HEIGHTS_70) * RUNS[3]["shading_summer"]))
show("rays drawn at 70 % (count)", len(HEIGHTS_70), 0)
show("rays drawn as blocked at 70 % (count)", BLOCKED_70, 0)
show("RAY_KWH if blocked/7 rays", 837.0162596751233 / len(HEIGHTS_70), 1)
show("total solar per drawn ray at 70 % (1428/7)", RUNS[2]["solar_summer"] / len(HEIGHTS_70), 1)
show("direct blocked per drawn ray at 70 % (837/7)", (RUNS[2]["solar_summer"] - RUNS[3]["solar_summer"]) / len(HEIGHTS_70), 1)
SILL_30 = 2.7 - min(RUNS[1]["window_area"] / FACADE_WIDTH, 2.7 - 0.05)
HEIGHTS_30 = []
index = 0
while 2.7 - 0.3 * (index + 0.5) > SILL_30 + 0.05:
    HEIGHTS_30.append(2.7 - 0.3 * (index + 0.5))
    index += 1
show("rays drawn at 30 % (count)", len(HEIGHTS_30), 0)
show("blocked at 30 % shading (count)", min(len(HEIGHTS_30), round(len(HEIGHTS_30) * RUNS[1]["shading_summer"])), 0)
show("shading_summer 30 % with overhang", RUNS[1]["shading_summer"], 4)

print()
print("== Costs on the scene ==")
for run in RUNS:
    show(f"cost r ({run['design']['wwr']}, {run['design']['overhang']})", run["cost"], 1)
print(f"tariff {TARIFF} CHF/kWh, COP {COOP}")

print()
print("== On-screen comparison (text read from the frames, en and de) ==")
CHECKS = []


def check(where, shown, computed):
    """✅ Compare one on-screen string with the recomputed string and record the verdict."""
    verdict = "OK" if shown == computed else "MISMATCH"
    CHECKS.append((verdict, where, shown, computed))
    print(f"{verdict:<9} {where:<40} screen {shown!r:<22} model {computed!r}")


def cooling(wwr, overhang):
    """❄️ Cooling electricity of one design with the study baseline for everything else."""
    return MODEL.evaluate({**BASE_DESIGN, "wwr": wwr, "overhang": overhang})["cooling"]


check("plot/card run 1 (30 %, 0 m)", "313", en(RUNS[0]["response"]))
check("plot/card run 2 (30 %, 1 m)", "190", en(RUNS[1]["response"]))
check("plot/card run 3 (70 %, 0 m)", "567", en(RUNS[2]["response"]))
check("plot/card run 4 (70 %, 1 m)", "288", en(RUNS[3]["response"]))
check("factors step (30 %, 0.5 m)", "194", en(cooling(0.3, 0.5)))
check("factors_2 (30 %, 0 m)", "313", en(cooling(0.3, 0.0)))
check("question centre room bar (50 %, 0.5 m)", "298", en(cooling(0.5, 0.5)))
check("legend ray kWh/a", "122", en(ray_kwh))
check("chain blocked sun 30 %", "367", en(blocked_small))
check("chain blocked sun 70 %", "837", en(blocked_large))
check("chain 30 % / EER", "−122", en(-blocked_small / EER))
check("chain 70 % / EER", "−279", en(-blocked_large / EER))
check("saving 30 % (plot arrow)", "−122", en(-small_saving))
check("saving 70 % (plot arrow)", "−279", en(-large_saving))
check("ratio window area", "2.3", en(RUNS[2]["window_area"] / RUNS[0]["window_area"], 1))
check("ratio saving 70/30", "2.3", en(large_saving / small_saving, 1))
check("main A mean 70 %", "427.2", en(a_high, 1))
check("main A mean 30 %", "251.6", en(a_low, 1))
check("main A effect (rounded means)", "+175.6", "+" + en(round(a_high, 1) - round(a_low, 1), 1))
check("main B mean with 1 m", "239.1", en(b_high, 1))
check("main B mean without", "439.7", en(b_low, 1))
check("main B effect (rounded means)", "−200.6", en(round(b_high, 1) - round(b_low, 1), 1))
check("difference bracket (savings difference)", "157", en(large_saving - small_saving))
check("interaction strip value", "−78.3", en((round(small_saving, 1) - round(large_saving, 1)) / 2, 1))
check("interaction vs narration 'half the difference of the two savings'", "−78.3", en((round(large_saving, 1) - round(small_saving, 1)) / 2, 1))
check("model constant", "339", en(FACTORIAL["coefficients"][0]))
check("model slope A (88)", "88", en(FACTORIAL["coefficients"][1]))
check("model slope B (100)", "−100", en(FACTORIAL["coefficients"][2]))
check("model AB (39)", "−39", en(FACTORIAL["coefficients"][3]))
check("model_3 A effect ÷ 2", "+87.8", "+" + en(FACTORIAL["effects"]["a"] / 2, 1))
check("model_3 B effect ÷ 2", "−100.3", en(FACTORIAL["effects"]["b"] / 2, 1))
check("model_3 AB effect ÷ 2", "−39.2", en(FACTORIAL["effects"]["ab"] / 2, 1))
check("waterfall result (sum of 4 runs)", "288", en(FACTORIAL["coefficients"][0] + FACTORIAL["coefficients"][1] + FACTORIAL["coefficients"][2] + FACTORIAL["coefficients"][3]))
check("curvature model at 0 m", "439.7", en(FACTORIAL["coefficients"][0] - FACTORIAL["coefficients"][2], 1))
check("curvature model at 1 m", "239.1", en(FACTORIAL["coefficients"][0] + FACTORIAL["coefficients"][2], 1))
check("confirm model 50 % 0.5 m", "339", en(CENTRE["predicted"]))
check("confirm room 50 % 0.5 m", "298", en(CENTRE["actual"]))
check("confirm_2 curvature", "−12.1 %", en(CENTRE["curvature"] * 100, 1) + " %")
check("confirm_2 gap", "−41", en(CENTRE["actual"] - CENTRE["predicted"]))
check("confirm_3 share of 1.0 m saving", "69 %", en(FACTORIAL["centre_savings"]["share"] * 100) + " %")
check("true cooling 50 % 1.0 m (curve end)", "236", en(cooling(0.5, 1.0)))
check("fraction run 1 (30 %, 0 m, 30 cm)", "316.5", en(FRACTION["runs"][0]["response"], 1))
check("fraction run 2 (30 %, 1 m, 10 cm)", "186.3", en(FRACTION["runs"][1]["response"], 1))
check("fraction run 3 (70 %, 0 m, 10 cm)", "564.8", en(FRACTION["runs"][2]["response"], 1))
check("fraction run 4 (70 %, 1 m, 30 cm)", "289.4", en(FRACTION["runs"][3]["response"], 1))
check("purpose thumbnail 5 (centre)", "298", en(CENTRE["actual"]))
check("purpose effect A (rounded)", "+176", "+" + en(FACTORIAL["effects"]["a"]))
check("purpose effect B (rounded)", "−201", en(FACTORIAL["effects"]["b"]))
check("purpose effect AB (rounded)", "−78", en(FACTORIAL["effects"]["ab"]))
check("contrast A·B + C", "−72.6", en(FRACTION["alias_contrast"], 1))
check("true A·B", "−78.3", en(FRACTION["true"]["ab"], 1))
check("true C", "+5.7", "+" + en(FRACTION["true"]["c"], 1))
check("error from confounding", "5.7", en(abs(FRACTION["alias_contrast"] - FRACTION["true"]["ab"]), 1))
check("answer saving 70 %", "279", en(large_saving))
check("answer saving 30 %", "122", en(small_saving))
check("answer room 70 % with 1 m", "288", en(RUNS[3]["response"]))
check("answer room 30 % with 1 m", "190", en(RUNS[1]["response"]))
check("sun profile on screen", "66°", en(MODEL.sun_state(180, "summer")["profile"]) + "°")
YMAX = 200 * math.ceil(max(run["response"] for run in RUNS) / 200)
check("plot y-axis top", "600", en(YMAX))
check("repeat_2 bar 1 (298 × (1 − 13 %), unlabelled)", "260", en(CENTRE["actual"] * (1 - .13)))
check("repeat_2 bar 2 (298 × (1 + 7 %))", "319", en(CENTRE["actual"] * (1 + .07)))
check("repeat_2 bar 3 (298 × (1 + 15 %))", "343", en(CENTRE["actual"] * (1 + .15)))
print(f"mismatching string checks: {sum(1 for item in CHECKS if item[0] == 'MISMATCH')} of {len(CHECKS)}")

print()
print("== Semantic consistency of on-screen chains ==")
print(f"legend ray ({en(ray_kwh, 1)}) x 7 drawn blocked rays = {en(ray_kwh * 7, 1)} vs blocked sun {en(blocked_large, 1)}")
print(f"total solar per 0.3 m ray at 70 % (transmitted, g = 0.5) = {en(RUNS[2]['solar_summer'] / 7, 1)} kWh/a")
print(f"direct share of that ray = {en(RUNS[2]['solar_summer'] / 7 * PARAMETERS['climate']['direct_share_summer']['value'], 1)} kWh/a")
print(f"blocked share of transmitted solar at 70 % = {en(blocked_large / RUNS[2]['solar_summer'] * 100, 1)} %")
print(f"drawn blocked rays at 70 % = {len(HEIGHTS_70)} of {len(HEIGHTS_70)} (100 %)")
print(f"blocked share of transmitted solar at 30 % = {en(blocked_small / RUNS[0]['solar_summer'] * 100, 1)} %")
print(f"chain exact? blocked/EER {en(blocked_large / EER, 3)} vs model saving {en(large_saving, 3)}")
print(f"saving ratio {en(large_saving / small_saving, 3)} vs blocked ratio {en(blocked_large / blocked_small, 3)} vs window ratio {en(RUNS[2]['window_area'] / RUNS[0]['window_area'], 3)}")
print("interaction AB = ½ × (saving70 − saving30) =", en((large_saving - small_saving) / 2, 2), "(positive)")
print("screen shows AB = ½ × [(−278.9) − (−122.3)] = −78.3, i.e. ½ × (saving70 − saving30) with savings negated")
