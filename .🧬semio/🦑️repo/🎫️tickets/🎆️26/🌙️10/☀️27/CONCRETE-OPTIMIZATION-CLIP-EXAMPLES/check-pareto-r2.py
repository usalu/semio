"""✅ Round 2 contract of the Pareto clip: every narrated claim and every derived on-screen number against the model."""

import ast
import importlib.util
from pathlib import Path

CLIP = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
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
    print(("ok   " if condition else "MISS ") + label)
    if not condition:
        FAILED.append(label)


def clause(key, *needles):
    en, de = NARRATION[key][1:]
    check(f"{key} mentions {needles}", all(needle in en and needle.replace(".", ",") in de for needle in needles))


first, second = STEPS[0], STEPS[1]
check("step 1 changes only the overhang 0 -> 0.6 m", BY[first["from"]]["design"]["overhang"] == 0 and BY[first["to"]]["design"]["overhang"] == .6
      and all(BY[first["from"]]["design"][key] == BY[first["to"]]["design"][key] for key in ("wwr", "insulation")))
clause("walk_2", "0.6\u00a0m", "192\u00a0CHF", "65\u00a0kWh/a")
check("walk_2 numbers", round(first["cost"]) == 192 and round(first["saved"]) == 65)
clause("walk_3", "3.0\u00a0CHF")
check("walk_3 ratio 3.0", round(first["ratio"], 1) == 3.0)
clause("payback", "0.25\u00a0CHF", "12 ")
check("payback 11.8 ~ 12", round(first["payback"]) == 12 and TARIFF == .25)
clause("payback_2", "30 ")
check("lifetime 30", LIFE == 30)
clause("knee_2", "34 ")
check("next step after the knee pays back in 34 a > 30 a", round(second["payback"]) == 34 and second["payback"] > LIFE and DATA["knee"]["tariff"] == first["to"])
clause("knee_3", "0.40\u00a0CHF", "21 ")
check("at 0.40 that step repays in 21 a <= 30 a and the knee moves one on", round(second["payback_hi"]) == 21 and DATA["knee"]["tariff_hi"] == second["to"])
ratios = [step["ratio"] for step in STEPS]
check("knee: price per saved kWh/a strictly rises", all(a < b for a, b in zip(ratios, ratios[1:])))
check("knee: payback exceeds the lifetime from step 2 on", [step["payback"] > LIFE for step in STEPS] == [False] + [True] * 6)
check("grid: 6 x 3 x 2 levels", [len({item["design"][key] for item in DATA["designs"]}) for key in ("wwr", "insulation", "overhang")] == [6, 3, 2])
check("rule_2: dominated pair is strictly better in both", all(BY[DATA["dominated_example"]["winner"]][key] < BY[DATA["dominated_example"]["loser"]][key] for key in ("cost", "electricity")))
member, refined = BY[DATA["refinement"]["member"]], DATA["refinement"]
designs = [member["design"]]
for key in ("overhang", "wwr", "insulation"):
    designs.append({**designs[-1], key: refined["design"][key]})
found = [MODEL.evaluate(design) for design in designs]
deltas = [(found[k + 1]["cost"] - found[k]["cost"], found[k + 1]["electricity"] - found[k]["electricity"]) for k in range(3)]
check("approximate: overhang deeper, window bigger, insulation thinner", designs[1]["overhang"] > designs[0]["overhang"] and designs[2]["wwr"] > designs[1]["wwr"] and designs[3]["insulation"] < designs[2]["insulation"])
check("approximate: each stage cost sign + + -", deltas[0][0] > 0 and deltas[1][0] > 0 and deltas[2][0] < 0)
check("approximate: net dominates the member", found[-1]["cost"] < member["cost"] and found[-1]["electricity"] < member["electricity"])
check("approximate: net = refinement", abs(found[-1]["cost"] - refined["cost"]) < 1e-6)
check("approximate: stage deltas add up to the net at 0.1 resolution", abs(sum(round(d[0], 1) for d in deltas) - round(found[-1]["cost"] - member["cost"], 1)) < .11)
tiles = [item["electricity"] for item in DATA["designs"]]
lights = [item["daylight"] for item in DATA["designs"]]
check("ramp ends are the tile range 520 / 776 kWh/a", (round(min(tiles)), round(max(tiles))) == (520, 776))
check("daylight ramp ends are the tile range 1.8 / 6.4 %", (round(min(lights), 1), round(max(lights), 1)) == (1.8, 6.4))
front = [BY[i] for i in FRONT]
span_c = max(d["cost"] for d in front) - min(d["cost"] for d in front)
span_e = max(d["electricity"] for d in front) - min(d["electricity"] for d in front)
for weight in (.05, .25, .5, .75, .98):
    worth = weight * span_c / ((1 - weight) * span_e)
    pick = FRONT[max([0] + [k + 1 for k, step in enumerate(STEPS) if step["ratio"] <= worth])]
    check(f"weights {weight}: pick = last front step with price <= worth ({worth:.1f} CHF per kWh/a)", pick == MODEL.weighted_pick(weight))
for identifier in ("w30-t12-d0", "w80-t12-d0", "w70-t30-d0", "w50-t30-d60"):
    parts = [BY[identifier][key] for key in ("lighting", "cooling", "heating")]
    scale = 10
    floors = [int(v * scale) for v in parts]
    check(f"bar {identifier}: parts add to the 1 decimal total", abs(sum(parts) - BY[identifier]["electricity"]) < 1e-9)
tc, tn = BY[FRONT[2]], BY[FRONT[1]]
check("decision_2 knee: 30 % window, 0.6 m, 12 cm", BY[DATA["knee"]["tariff"]]["design"]["wwr"] == .3 and BY[DATA["knee"]["tariff"]]["design"]["overhang"] == .6 and BY[DATA["knee"]["tariff"]]["design"]["insulation"] == .12)
print("\n%d failed" % len(FAILED), FAILED)
