"""Check every number and claim of the algorithms narration against the model; run: python check-algorithms-narration.py"""
import ast
import importlib
import re
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIP = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
tree = ast.parse((CLIP / "🧬️algorithms/🐍️.py").read_text(encoding="utf-8"))
rows = next(ast.literal_eval(node.value) for cls in tree.body if isinstance(cls, ast.ClassDef) for node in cls.body
            if isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in node.targets))

surface, gradient, pattern = model.surface(41), model.gradient_path(), model.pattern_path()
genetic, convergence = model.genetic(), model.convergence()
minimum, series = surface["minimum"], {k: [v for _, v in s] for k, s in convergence["series"].items()}
optimum = minimum["design"]
generations = genetic["generations"]
first = generations[0]
lookup = {i["id"]: i for i in first["population"]}
later = {i["id"]: i for i in generations[1]["population"]}


def within(name, fraction=0.01):
    return series[name][-1] <= (1 + fraction) * minimum["electricity"]


def reach(name, fraction=0.01):
    return next((i + 1 for i, v in enumerate(series[name]) if v <= (1 + fraction) * minimum["electricity"]), None)


def featured():
    for child in first["children"]:
        parents = [lookup[i] for i in child["from"]]
        cross = {**parents[0]["design"], **{k: lookup[child["genes"][k]]["design"][k] for k in ("wwr", "overhang")}}
        ce = model.evaluate(cross)["electricity"]
        if ce < min(p["electricity"] for p in parents) and child["electricity"] > ce:
            return child, ce
    return None, None


child, crossed = featured()
spread = max(s[-1] for s in series.values()) - min(s[-1] for s in series.values())
facts = {
    "runs_map": convergence["runs_map"] == 1681,
    "budget": convergence["budget"] == 50,
    "start": (gradient[0]["design"]["wwr"], gradient[0]["design"]["overhang"]) == (.75, .1),
    "gradient 5 runs per step": all(i["runs"] == 5 for i in gradient),
    "gradient 50 runs": gradient[-1]["evaluations"] == 50,
    "pattern 49 runs": pattern[-1]["evaluations"] == 49,
    "genetic 50 runs": generations[-1]["evaluations"] == 50,
    "population 8": all(len(g["population"]) == 8 for g in generations),
    "six children": all(len(g["children"]) == 6 for g in generations[:-1]),
    "two elites": all(len(g["elites"]) == 2 for g in generations[:-1]),
    "seven generations": len(generations) - 1 == 7,
    "slope wwr > 0 and overhang < 0": gradient[0]["slope"]["wwr"] > 0 > gradient[0]["slope"]["overhang"],
    "optimum has less window and more overhang than the start": optimum["wwr"] < .75 and optimum["overhang"] > .1,
    "first step crosses the valley floor in WWR": (gradient[0]["design"]["wwr"] - optimum["wwr"]) * (gradient[1]["design"]["wwr"] - optimum["wwr"]) < 0,
    "gradient within 1 %": within("gradient"),
    "pattern within 1 %": within("pattern"),
    "genetic within 1 %": within("genetic"),
    "pattern search needs exactly 16 runs to get within 1 %": reach("pattern") == 16,
    "the other two need more than twice as many": all(reach(k) and reach(k) > 2 * reach("pattern") for k in ("gradient", "genetic")),
    "pattern search arrives first": all(reach("pattern") < reach(k) for k in ("gradient", "genetic")),
    "gradient descent raises the electricity at some iterate": any(b["electricity"] > a["electricity"] for a, b in zip(gradient, gradient[1:])),
    "genetic best improves in steps (at least 3 plateaus)": sum(a > b + .05 for a, b in zip(series["genetic"], series["genetic"][1:])) >= 3,
    "all three end within 3.3 kWh/a of the map minimum": abs(round(max(s[-1] for s in series.values()), 1) - round(minimum["electricity"], 1) - 3.3) < .05,
    "a mutation improves a child": any(any(c["mutation"].values()) and c["electricity"] < model.evaluate({**lookup[c["from"][0]]["design"], **{k: lookup[c["genes"][k]]["design"][k] for k in ("wwr", "overhang")}})["electricity"] for c in first["children"]),
    "a mutation worsens a child in generation 1": any(any(c["mutation"].values()) and c["electricity"] > model.evaluate({**later[c["from"][0]]["design"], **{k: later[c["genes"][k]]["design"][k] for k in ("wwr", "overhang")}})["electricity"] for c in generations[1]["children"]),
    "a mid-ranked card (rank 4 to 7 of 8) wins a duel": any(d[0] != d[1] and 4 <= sorted(i["electricity"] for i in first["population"]).index(lookup[d[2]]["electricity"]) + 1 <= 7 for d in first["tournaments"]),
    "the tolerance rule stops the genetic run after 15 simulations, more than 10 % above the map minimum": next(n for n in range(6, 51) if series["genetic"][n - 6] - series["genetic"][n - 1] < 1.0) == 15 and series["genetic"][14] > 1.1 * minimum["electricity"],
    "the tolerance rule fires first for all three methods": all(next(n for n in range(6, 51) if series[k][n - 6] - series[k][n - 1] < 1.0) < next((n for n in range(11, 51) if series[k][n - 11] - series[k][n - 1] <= 0), 99) for k in series),
}
numbers = {"1,681": "runs_map", "50": "budget", "49": "pattern 49 runs", "75 %": "start", "0.1 m": "start", "8 ": "population 8", "6 new": "six children", "7 gen": "seven generations"}
for key, good in facts.items():
    print(("PASS" if good else "FAIL"), key)
print("series ends", {k: round(v[-1], 2) for k, v in series.items()}, "minimum", round(minimum["electricity"], 2), "reach", {k: reach(k) for k in series})
problems = 0
for key, en, de in rows:
    for text in (en, de):
        expanded = re.sub(r"\d", "0", text)
        if len(text) > 100:
            print("LONG", key, len(text))
            problems += 1
    for number in re.findall(r"\d[\d,.]*", en):
        if number.strip(",.") not in {"1,681", "50", "49", "75", "0.1", "8", "6", "7", "1", "2", "5", "4", "16", "61", "0.91", "496", "0,91", "66", "0.99", "0,99", "494.8", "494,8", "3.3", "3,3", "15"}:
            print("UNKNOWN NUMBER", key, number)
            problems += 1
    if re.sub(r"\d+([.,]\d+)?", "#", en).count("#") != re.sub(r"\d+([.,]\d+)?", "#", de).count("#"):
        print("NUMBER COUNT DIFFERS", key)
        problems += 1
print(len(rows), "clauses;", problems, "textual problems;", sum(not good for good in facts.values()), "failed facts")
pool = [(row["electricity"], row["design"]) for row in gradient] + [(p["electricity"], p["design"]) for row in gradient for p in row["probes"]]
pool += [(row["electricity"], row["design"]) for row in pattern] + [(p["electricity"], p["design"]) for row in pattern for p in row["probes"]]
pool += [(item["electricity"], item["design"]) for g in generations for item in g["population"]] + [(c["electricity"], c["design"]) for g in generations for c in g["children"]]
best = min(pool, key=lambda item: item[0])
print("map minimum:", round(optimum["wwr"] * 100), round(optimum["overhang"], 2), round(minimum["electricity"], 1), "solver best:", round(best[1]["wwr"] * 100), round(best[1]["overhang"], 2), round(best[0], 1), "(narration close: 66 %, 0.99 m, 494.8 kWh/a)")
