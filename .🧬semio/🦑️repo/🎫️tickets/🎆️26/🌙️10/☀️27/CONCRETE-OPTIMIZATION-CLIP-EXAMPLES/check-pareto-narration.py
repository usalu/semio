"""🔎 Check every number and claim in the Pareto narration against the model.

Usage: python -I check-pareto-narration.py   (exit code 1 when a claim does not hold)
"""
import ast
import importlib
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
tree = ast.parse((CLIP / "🏆️pareto/🐍️.py").read_text(encoding="utf-8"))
rows = next(ast.literal_eval(node.value) for cls in tree.body if isinstance(cls, ast.ClassDef) for node in cls.body
            if isinstance(node, ast.Assign) and getattr(node.targets[0], "id", "") == "NARRATION")
text = {key: (en, de) for key, en, de in rows}
data = model.pareto()
by_id = {d["id"]: d for d in data["designs"]}
front, steps = data["front"], data["steps"]
failures = []


def check(label, condition, detail=""):
    print(("ok   " if condition else "FAIL ") + label + ("" if condition else "  " + detail))
    if not condition:
        failures.append(label)


def number(value, digits, german):
    shown = f"{round(value, digits):,.{digits}f}"
    return shown.replace(",", "\0").replace(".", ",").replace("\0", ".") if german else shown


def contains(key, value, digits=0, tail=""):
    return all((number(value, digits, german) + tail) in text[key][index] for index, german in enumerate((False, True)))


designs = data["designs"]
levels = {name: sorted({d["design"][name] for d in designs}) for name in ("wwr", "insulation", "overhang")}
check("grid: every combination of window size, insulation and overhang, no counts in the clause",
      len(designs) == len(levels["wwr"]) * len(levels["insulation"]) * len(levels["overhang"]) and not any(c.isdigit() for c in text["grid"][0] + text["grid"][1]))
print(f"     grid on screen: {len(designs)} designs = {len(levels['wwr'])} WWR x {len(levels['insulation'])} t x {len(levels['overhang'])} d")
pair = data["dominated_example"]
winner, loser = by_id[pair["winner"]], by_id[pair["loser"]]
check("dominated: loser has more glass and more insulation", loser["design"]["wwr"] > winner["design"]["wwr"] and loser["design"]["insulation"] > winner["design"]["insulation"])
check("dominated_2: winner cheaper and less electricity", winner["cost"] < loser["cost"] and winner["electricity"] < loser["electricity"])
cheap = min(designs, key=lambda d: (d["cost"], d["electricity"]))
frugal = min(designs, key=lambda d: (d["electricity"], d["cost"]))
check("extremes: no design is both cheapest and most frugal", cheap["id"] != frugal["id"] and cheap["electricity"] > frugal["electricity"] and cheap["cost"] < frugal["cost"])
order = sorted(designs, key=lambda d: (d["cost"], d["electricity"]))
kept, best = [], float("inf")
for item in order:
    if item["electricity"] < best:
        kept.append(item["id"])
        best = item["electricity"]
check("front: sweep keeps exactly the model front", kept == front, f"{kept} vs {front}")
first = steps[0]
check("walk_2: first step cost and saving", contains("walk_2", first["cost"], 0, " CHF") and contains("walk_2", first["saved"], 0, " kWh/a"), f"{first['cost']:.1f} {first['saved']:.1f}")
first_to, first_from = by_id[first["to"]]["design"], by_id[first["from"]]["design"]
check("walk_2: the first step adds exactly the named overhang and nothing else", contains("walk_2", first_to["overhang"], 1, " m") and first_from["overhang"] == 0
      and first_to["wwr"] == first_from["wwr"] and first_to["insulation"] == first_from["insulation"])
check("walk_3: first step price per kWh/a", contains("walk_3", first["ratio"], 1, " CHF"), f"{first['ratio']:.2f}")
check("payback: tariff and first payback years", contains("payback", data["tariff"], 2) and contains("payback", first["payback"], 0, " "), f"{data['tariff']} {first['payback']:.1f}")
check("payback_2: service life", contains("payback_2", data["service_life"], 0, " "), str(data["service_life"]))
check("payback: first step repays within the service life", first["payback"] <= data["service_life"])
ratios = [step["ratio"] for step in steps]
check("knee: every later step saves less per franc (price rises along the front)", all(a < b for a, b in zip(ratios, ratios[1:])), str([round(r, 1) for r in ratios]))
over = [step["payback"] > data["service_life"] for step in steps]
check("knee: payback crosses the lifetime once and stays above", over == sorted(over), str([round(s["payback"], 1) for s in steps]))
check("knee_2: knee is an interior front member", front[0] != data["knee"]["tariff"] != front[-1], data["knee"]["tariff"])
check("knee_3: the knee moves with the tariff", data["knee"]["tariff"] != data["knee"]["tariff_hi"], str(data["knee"]))
refined = data["refinement"]
check("approximate: the unevaluated design dominates a front member", bool(set(refined["dominates"]) & set(front)), str(refined["dominates"]))
check("approximate: the refinement has a deeper overhang than any simulated design and dominates its member",
      refined["design"]["overhang"] > max(d["design"]["overhang"] for d in designs) and refined["cost"] < by_id[refined["member"]]["cost"]
      and refined["electricity"] < by_id[refined["member"]]["electricity"], str(refined))
knee_design, tariff = by_id[data["knee"]["tariff"]]["design"], data["tariff"]
check("decision_2: the named knee design and tariff",
      contains("decision_2", knee_design["wwr"] * 100, 0, " %") and contains("decision_2", knee_design["overhang"], 1, " m") and contains("decision_2", knee_design["insulation"] * 100, 0, " cm")
      and contains("decision_2", tariff, 2, " CHF/kWh"), str(knee_design))
check("front_2: 'simulated designs' named", "simulated designs" in text["front_2"][0] and "simulierten Entwürfe" in text["front_2"][1])
third = data["third_example"]
low, dominator = by_id[third["dominated_2d"]], by_id[third["dominator"]]
check("third_2: dominated in 2D by a cheaper, frugal but dimmer design", dominator["cost"] < low["cost"] and dominator["electricity"] < low["electricity"] and dominator["daylight"] < low["daylight"])
check("third_3: nondominated with daylight, front grows", third["dominated_2d"] in data["front3"] and third["dominated_2d"] not in front and len(data["front3"]) > len(front))
picks = [model.weighted_pick(k / 100) for k in range(101)]
indexes = [front.index(i) for i in picks]
check("weights_2: pick slides monotonically along the front", indexes == sorted(indexes) and len(set(indexes)) >= 3, str(sorted(set(indexes))))
print(f"{len(failures)} failure(s)")
sys.exit(1 if failures else 0)
