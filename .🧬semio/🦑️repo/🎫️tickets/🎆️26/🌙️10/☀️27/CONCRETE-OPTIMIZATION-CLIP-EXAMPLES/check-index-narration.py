"""✅ Check that the index clip's narration claims hold for the current model (run with the teaching Python).

Usage: python check-index-narration.py
Exit code 1 and one line per failed claim when the model drifted away from the narration.
"""
import ast
import importlib
import re
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
SCENE = ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/⚖️index/🐍️.py"
tree = ast.parse(SCENE.read_text(encoding="utf-8"))
cls = next(n for n in tree.body if isinstance(n, ast.ClassDef) and n.name == "Beat4_NormalizedIndex")
rows = next(ast.literal_eval(n.value) for n in cls.body if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "NARRATION" for t in n.targets))
clause = {key: (en.replace(" ", " "), de.replace(" ", " ")) for key, en, de in rows}
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
scene = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.⚖️index.🐍️")
failures = []


def check(ok, claim):
    if not ok:
        failures.append(claim)


check(len(clause) == len(rows), "narration keys are unique")
check(all(len(en) <= 95 and len(de) <= 95 for en, de in clause.values()), "every clause has at most 95 characters")
study = model.load_study()["weights"]
balanced = study["balanced"]
order = [c["id"] for c in model.criteria()]
for index, text in enumerate(clause["weights_2"]):
    shares = [int(v) for v in re.findall(r"(\d+) %", text)]
    check(shares == [round(balanced[c] * 100) for c in order], f"weights_2 clause {index} names the balanced weights {shares}")
check(all("100 %" in text for text in clause["weights"]), "weights mentions 100 %")
five, six = [v["id"] for v in model.variants()], [v["id"] for v in model.variants(extra=True)]
check(len(five) == 5 and len(six) == 6, "five variants plus one extra")
raw = lambda i, c: next(v for v in model.variants(True) if v["id"] == i)[next(x["key"] for x in model.criteria() if x["id"] == c)]
direction = {c["id"]: c["direction"] for c in model.criteria()}
best = {c: min(five, key=lambda i: raw(i, c) * (1 if direction[c] == "min" else -1)) for c in order}
check(len(set(best.values())) > 1, "no variant is best in all four criteria (best)")
weights = lambda w: dict(zip(order, w))
heavy = model.reweigh(balanced, "cost", .6)
win = lambda w, ids=None: min(model.index(w, ids), key=lambda r: r["score"])["id"]
check(win(balanced) != win(heavy), "the cost-driven winner differs from the balanced winner (sensitivity_3)")
check([round(100 * heavy[c]) for c in order] == [23, 60, 11, 6], "cost 60 % reweighs to 23 / 60 / 11 / 6")
cost_series = [win(model.reweigh(balanced, "cost", k / 40)) for k in range(41)]
check(len(set(cost_series)) > 1, "the winner switches somewhere between cost weight 0 and 100 % (sweep)")
gaps = []
for k in range(41):
    scores = sorted(r["score"] for r in model.index(model.reweigh(balanced, "cost", k / 40)))
    gaps.append(scores[1] - scores[0])
check(any(g < .03 for g in gaps), "some cost weight is a close call below 0.03 (sweep_2)")
order5 = [r["id"] for r in sorted(model.index(balanced), key=lambda r: r["rank"])]
rows6 = sorted(model.index(balanced, six), key=lambda r: r["rank"])
order6 = [r["id"] for r in rows6 if r["id"] in five]
check(order5 != order6, "the sixth variant changes the order of the original five (range_3)")
check(rows6[0]["id"] != six[-1], "the sixth variant is not the winner")
check(scene.best_id("cost", five) == scene.best_id("cost", six), "adding the sixth variant keeps the best cost (the ruler only stretches)")
check(raw(six[-1], "cost") > max(raw(i, "cost") for i in five), "the sixth variant is dearer than the five (range_2 stretches the cost ruler)")
sums = {s: sorted(five, key=scene.Beat4_NormalizedIndex.sums(None, s).get) for s in (1, 1000)}
check(sums[1] != sums[1000], "naive sums rank differently in CHF/kg and kCHF/t (units_3)")
for criterion in order:
    rows_ = {r["id"]: r["normalized"][criterion] for r in model.index(balanced)}
    values = {i: raw(i, criterion) for i in five}
    low, high = min(values.values()), max(values.values())
    for i in five:
        n = (values[i] - low) / (high - low)
        n = 1 - n if direction[criterion] == "max" else n
        check(abs(n - rows_[i]) < 1e-9, f"ruler geometry equals model normalization for {criterion}/{i}")
winner = win(balanced, five)
cap = scene.BUDGET_CHF
over = [i for i in five if raw(i, "cost") > cap]
check(len(over) >= 1 and winner not in over, "the authored client budget removes a variant but keeps the balanced winner (limits)")
check(any(raw(i, "cost") > cap for i in five if i != winner) and raw(winner, "cost") <= cap, "the winner is inside the cap")
switches = scene.Beat4_NormalizedIndex.switch_shares(None, balanced)
check(len(switches) >= 2 and all(0 < s < 100 for s in switches), f"the sweep marks at least two switch shares {switches}")
check(scene.FLOOR_AREA == 48 and abs(model.evaluate(model.variants()[0]["design"])["intensity"] * 48 - model.evaluate(model.variants()[0]["design"])["electricity"]) < 1e-6, "electricity ÷ 48 m² equals the intensity shown in the conversion line")
for line in failures:
    print("FAIL", line)
print("narration claims:", "ok" if not failures else f"{len(failures)} failed", "| winners:", win(balanced), win(heavy), "| naive CHF:", sums[1][0], "kCHF:", sums[1000][0])
sys.exit(1 if failures else 0)
