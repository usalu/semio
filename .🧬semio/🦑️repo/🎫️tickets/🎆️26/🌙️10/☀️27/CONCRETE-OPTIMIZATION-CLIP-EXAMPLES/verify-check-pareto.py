"""✅ Verification pass of the Pareto clip: keys, model numbers as printed on the frames, printed arithmetic and German format."""

import ast
import importlib.util
import re
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
FRAMES = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/🗑️generated/frames/Beat5_ParetoFront")
SPEC = importlib.util.spec_from_file_location("optimization_model", CLIP / "🧮️model" / "🐍️.py")
MODEL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODEL)
DATA = MODEL.pareto()
BY = {item["id"]: item for item in DATA["designs"]}
FRONT, STEPS, TARIFF, HIGH, LIFE = DATA["front"], DATA["steps"], DATA["tariff"], DATA["tariff_hi"], DATA["service_life"]
TREE = ast.parse((CLIP / "🏆️pareto" / "🐍️.py").read_text(encoding="utf-8"))
SCENE = next(node for node in TREE.body if isinstance(node, ast.ClassDef))
NARRATION = {row[0]: row for row in next(ast.literal_eval(node.value) for node in SCENE.body if isinstance(node, ast.Assign) and node.targets[0].id == "NARRATION")}
FAILED = []


def check(label, condition):
    print(("ok   " if condition else "FAIL ") + label)
    if not condition:
        FAILED.append(label)


def half_up(value, digits=0):
    scale = 10 ** digits
    return int(abs(value) * scale + 0.5) / scale * (1 if value >= 0 else -1)


frame_en = sorted(path.name[len("Beat5_ParetoFront-en-"):-4] for path in FRAMES.glob("Beat5_ParetoFront-en-*.png"))
frame_de = sorted(path.name[len("Beat5_ParetoFront-de-"):-4] for path in FRAMES.glob("Beat5_ParetoFront-de-*.png"))
check(f"keys: 32 en and 32 de frames equal the NARRATION keys ({len(frame_en)} / {len(frame_de)})", frame_en == frame_de == sorted(NARRATION) and len(NARRATION) == 32)
for key, en, de in NARRATION.values():
    check(f"{key}: German has comma decimals only", not re.search(r"\d\.\d", de))
    check(f"{key}: English has point decimals only", not re.search(r"\d,\d", en))

first, second = STEPS[0], STEPS[1]
check("walk_2 numbers: step 1 costs 192 CHF and saves 65 kWh/a", half_up(first["cost"]) == 192 and half_up(first["saved"]) == 65)
check("walk_3 printed arithmetic: 192 / 65 = 2.95 -> 3.0", half_up(192 / 65, 1) == 3.0)
check("payback printed arithmetic: 65 x 0.25 ~ 16.2 and 192 / 16.2 -> 12", abs(65 * .25 - 16.2) <= .06 and half_up(192 / 16.2) == 12)
check("knee step 7 printed arithmetic: 0.25 x 0.25 = 0.0625 -> 0.063", half_up(.25 * .25, 3) == .063)
check("knee step 7 payback: 2,680 a lies inside the range that the printed 168 CHF and 0.063 CHF/a allow", 168 / .0635 <= 2680 <= 168 / .0625)
check("knee_3 printed arithmetic: 51 x 0.40 = 20.4 and 427 / 20.4 = 21", half_up(50.9 * .4, 1) == 20.4 and half_up(427 / 20.4) == 21)
check("knee_2 printed arithmetic: 427 / 12.7 = 33.6 -> 34", half_up(427 / 12.7) == 34)
check("payback table at 0.25 CHF/kWh (12, 34, 309, 335, 624, 963, 2,680)", [half_up(s["ratio"] / .25) for s in STEPS] == [12, 34, 309, 335, 624, 963, 2680])
check("payback table at 0.40 CHF/kWh (7, 21, 193, 209, 390, 602, 1,675)", [half_up(s["ratio"] / .40) for s in STEPS] == [7, 21, 193, 209, 390, 602, 1675])
check("knee claim: price per saved kWh/a strictly rises from step to step", all(a["ratio"] < b["ratio"] for a, b in zip(STEPS, STEPS[1:])))
check("knee claim: payback exceeds the 30 a lifetime from step 2 on", [s["ratio"] / TARIFF > LIFE for s in STEPS] == [False] + [True] * 6)
check("knee_3 claim: at 0.40 the step repays in 21 a <= 30 a", half_up(second["ratio"] / HIGH) == 21 and second["ratio"] / HIGH <= LIFE)

winner, loser = BY[DATA["dominated_example"]["winner"]], BY[DATA["dominated_example"]["loser"]]
check("dominated pair printed rows: 19,210 - 21,696 = -2,486 CHF and 649 - 724 = -75 kWh/a", round(winner["cost"]) - round(loser["cost"]) == -2486 and round(winner["electricity"]) - round(loser["electricity"]) == -75)
check("dominated pair: winner strictly better in both (rule_2)", winner["cost"] < loser["cost"] and winner["electricity"] < loser["electricity"])

third_loser, third_winner = BY["w80-t12-d0"], BY["w30-t12-d0"]
check("daylight rows: 21,346 - 19,210 = 2,136 CHF; 776 - 649 = 127 kWh/a; 6.4 - 2.4 = 4.0 %", round(third_loser["cost"]) - round(third_winner["cost"]) == 2136 and round(third_loser["electricity"]) - round(third_winner["electricity"]) == 127 and round(third_loser["daylight"], 1) - round(third_winner["daylight"], 1) == 4.0)
check("DF printed formula 0.6 x 7.2 m2 x 75 / (180 m2 x 0.75) = 2.4 %", half_up(.6 * 7.2 * 75 / (180 * .75), 1) == 2.4)
check("DF printed formula 0.6 x 19.2 m2 x 75 / (180 m2 x 0.75) = 6.4 %", half_up(.6 * 19.2 * 75 / (180 * .75), 1) == 6.4)
check("DF formula matches the model daylight factor (2.4 and 6.4 %)", abs(BY["w30-t12-d0"]["daylight"] - 2.4) < .05 and abs(BY["w80-t12-d0"]["daylight"] - 6.4) < .05)
check("extremes printed gap: 21,552 - 19,210 = 2,342 CHF and 520 - 649 = -129 kWh/a", round(BY["w50-t30-d60"]["cost"]) - round(BY["w30-t12-d0"]["cost"]) == 2342 and round(BY["w50-t30-d60"]["electricity"]) - round(BY["w30-t12-d0"]["electricity"]) == -129)

refined = DATA["refinement"]
member = BY[refined["member"]]
designs = [member["design"]]
for key in ("overhang", "wwr", "insulation"):
    designs.append({**designs[-1], key: refined["design"][key]})
found = [MODEL.evaluate(design) for design in designs]
stage_cost = [found[k + 1]["cost"] - found[k]["cost"] for k in range(3)]
stage_power = [found[k + 1]["electricity"] - found[k]["electricity"] for k in range(3)]
check("approximate path rows CHF: +124.8, +268.8, -685.4", [round(v, 1) for v in stage_cost] == [124.8, 268.8, -685.4])
check("approximate path rows kWh/a: -5.4, -21.4, +1.8", [round(v, 1) for v in stage_power] == [-5.4, -21.4, 1.8])
check("approximate net -291.8 CHF and -25.0 kWh/a equals the sum of the printed stages", round(sum(stage_cost), 1) == -291.8 and round(sum(stage_power), 1) == -25.0)
check("approximate beats exactly the 2 front members 21,384 / 520 and 21,552 / 520", sorted(refined["dominates"]) == sorted(["w40-t30-d60", "w50-t30-d60"]) and all(BY[i]["cost"] > refined["cost"] and BY[i]["electricity"] > refined["electricity"] for i in refined["dominates"]))
check("approximate overhang 0.99 m: room chip prints 1.0 m, rows print 0.99 m (presentation mismatch)", round(refined["design"]["overhang"], 1) == 1.0 and refined["design"]["overhang"] == .99)

SCREEN_BARS = {"w70-t30-d0": (121.1, 568.3, 34.3, 723.7), "w80-t12-d0": (112.5, 629.4, 34.0, 775.9), "w30-t12-d0": (240.8, 310.4, 97.7, 648.9),
               "w30-t12-d60": (298.2, 188.1, 97.6, 583.9), "w40-t12-d60": (240.9, 219.1, 73.1, 533.1), "w50-t30-d60": (199.8, 274.8, 45.2, 519.8),
               "w80-t20-d0": (112.5, 630.6, 32.1, 775.2)}
for key, (light, cool, heat, total) in SCREEN_BARS.items():
    check(f"bar {key} as printed: parts add to the printed total {total}; each part within one display unit (0.1) of the model", abs(light + cool + heat - total) < .051
          and abs(total - BY[key]["electricity"]) < .051 and all(abs(a - b) <= .1 for a, b in ((light, BY[key]["lighting"]), (cool, BY[key]["cooling"]), (heat, BY[key]["heating"]))))

a, b = SCREEN_BARS[first["from"]], SCREEN_BARS[first["to"]]
changes = [round(y - x, 1) for x, y in zip(a, b)]
check(f"walk_2 printed changes {changes[:3]} add to the printed total change {changes[3]} and equal the difference of the printed values", abs(sum(changes[:3]) - changes[3]) < 1e-9 and changes == [57.4, -122.3, -0.1, -65.0])
for step_ix, step in ((0, first), (6, STEPS[-1])):
    for name in ("lighting", "cooling", "heating", "electricity"):
        change = round(round(BY[step["to"]][name], 1) - round(BY[step["from"]][name], 1), 1)
        exact = BY[step["to"]][name] - BY[step["from"]][name]
        check(f"step {step_ix + 1} {name}: printed change {change:+.1f} within 0.1 of the model {exact:+.3f}", abs(change - exact) <= .1 + 1e-9)

tiles = [item["electricity"] for item in DATA["designs"]]
lights = [item["daylight"] for item in DATA["designs"]]
check("ramp ends: 520 and 776 kWh/a, 1.8 and 6.4 % are the 36 tiles", (half_up(min(tiles)), half_up(max(tiles))) == (520, 776) and (half_up(min(lights), 1), half_up(max(lights), 1)) == (1.8, 6.4))
check("front counts: 8 in 2D and 30 in 3D", len(FRONT) == 8 and len(DATA["front3"]) == 30)
front = [BY[i] for i in FRONT]
span_c = max(d["cost"] for d in front) - min(d["cost"] for d in front)
span_e = max(d["electricity"] for d in front) - min(d["electricity"] for d in front)
worth = lambda w: w * span_c / ((1 - w) * span_e)
check(f"worth at 50/50 prints 18.2 CHF per kWh/a ({worth(.5):.2f})", half_up(worth(.5), 1) == 18.2)
check(f"worth at 98 % electricity prints 890 CHF per kWh/a ({worth(.98):.1f})", half_up(worth(.98)) == 890)
check("weights_2 step into the pick prints 670 CHF per kWh/a <= 890", half_up(STEPS[6]["ratio"]) == 670 and STEPS[6]["ratio"] <= worth(.98))
check("decision_2 knee: 30 % window, 0.6 m overhang, 12 cm insulation at 0.25 CHF/kWh", BY[DATA["knee"]["tariff"]]["design"] == {**BY[DATA["knee"]["tariff"]]["design"], "wwr": .3, "overhang": .6, "insulation": .12})
check("knee at 0.40 CHF/kWh moves to 40 % window", BY[DATA["knee"]["tariff_hi"]]["design"]["wwr"] == .4)
print("\n%d failed %s" % (len(FAILED), FAILED))
