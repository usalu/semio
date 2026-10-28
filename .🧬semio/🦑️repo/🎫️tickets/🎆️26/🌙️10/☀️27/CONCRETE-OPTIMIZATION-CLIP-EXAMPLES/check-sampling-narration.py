"""📏 Check the sampling narration against the model: clause lengths, step keys, structural numbers and qualitative claims."""
import ast
import importlib
import math
import re
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
source = (ROOT / "🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🎲️sampling/🐍️.py").read_text(encoding="utf-8")
tree = ast.parse(source)
scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat2_GridRandomSampling")
narration = next(ast.literal_eval(node.value) for node in scene.body if isinstance(node, ast.Assign) and node.targets[0].id == "NARRATION")
problems = []


def check(condition, message):
    if not condition:
        problems.append(message)


keys = [row[0] for row in narration]
check(len(set(keys)) == len(keys), "duplicate keys")
used = set(re.findall(r'self\.step\("([a-z_0-9]+)"', source))
check(used == set(keys), f"step keys differ from narration: {sorted(used ^ set(keys))}")
seconds = 0.0
for key, en, de in narration:
    check(len(en) <= 95 and len(de) <= 95, f"{key} longer than 95 characters")
    check(en != de, f"{key} identical in both languages")
    seconds += max(len(de.split()) / 1.6, len(de.split()) / 2.2, 1.5)
print(f"[DEBUG] {len(keys)} clauses, about {seconds:.0f} s of caption reading time")

levels, strata = model.levels(), model.lhs_strata()
samples = {kind: model.samples(kind) for kind in ("grid", "random", "lhs")}
surface = model.surface(41)
check(all(len(rows) == 9 for rows in samples.values()), "nine samples per set")
check(len(levels["wwr"]) == 3 and len(levels["overhang"]) == 3 and len(levels["orientation"]) == 4 and len(levels["insulation"]) == 3, "levels 3, 3, 4, 3")
check(math.prod(len(levels[key]) for key in ("wwr", "overhang", "orientation", "insulation")) == 108 and 108 // 9 == 12, "108 runs, twelve times nine")
check(strata["count"] == 9 and strata["count"] ** 2 == 81, "nine strata, 81 cells")
indices = [tuple(item["strata_index"]) for item in samples["lhs"]]
check(sorted(i for i, _ in indices) == list(range(9)) and sorted(j for _, j in indices) == list(range(9)), "one LHS sample per stratum on both axes")
check(len(set(indices)) == 9 and len(set(indices)) < 81, "LHS does not fill every 2D cell")
optimum = surface["minimum"]
design = optimum["design"]
check(0.1 < design["wwr"] < 0.8 and 0 < design["overhang"] < 1.2, "optimum lies inside the range (reveal_2)")
for kind, rows in samples.items():
    best = min(item["electricity"] for item in rows)
    check(best > optimum["electricity"], f"best of {kind} must stay above the optimum (compare_2)")
check(not any(item["wwr"] >= 0.55 and item["overhang"] >= 0.8 for item in samples["random"]), "random draws leave large windows with deep overhangs empty (gaps)")
box = model.sampling_diagnostics("random")["largest_empty_box"]
check(box["overhang"][1] >= 1.19, "the empty random box reaches the deepest overhang (gaps)")
gaps = {key: (en, de) for key, en, de in narration}["gaps"]
check([round(box["wwr"][0] * 100), round(box["wwr"][1] * 100)] == [35, 70] and [round(box["overhang"][0], 2), round(box["overhang"][1], 2)] == [0.15, 1.2], "gaps caption numbers equal the drawn empty box")
check(all(token in gaps[0] for token in ("35", "70", "0.15", "1.20")) and all(token in gaps[1] for token in ("35", "70", "0,15", "1,20")), "gaps caption names the box edges in both languages")
check(not any(box["wwr"][0] < item["wwr"] < box["wwr"][1] and box["overhang"][0] < item["overhang"] < box["overhang"][1] for item in samples["random"]), "no random draw inside the empty box (gaps)")
best_random = min(samples["random"], key=lambda item: item["electricity"])
pair_ids = model.sampling_diagnostics("random")["closest_pair"][:2]
check(samples["random"].index(best_random) in pair_ids, "the best random draw belongs to the closest pair (gaps_3)")
check(min(samples["grid"], key=lambda item: item["electricity"])["electricity"] < max(samples["grid"], key=lambda item: item["electricity"])["electricity"], "grid worst differs from best (grid_4)")
pair = model.sampling_diagnostics("random")["closest_pair"]
check(pair[2] < 0.1, "random cluster is a close pair (gaps_2)")
check(min(item["electricity"] for item in samples["lhs"]) < min(item["electricity"] for item in samples["random"]), "LHS best below random best (plan)")
print("[DEBUG] best per set:", {kind: round(min(item["electricity"] for item in rows), 1) for kind, rows in samples.items()}, "optimum", round(optimum["electricity"], 1))
print("PROBLEMS" if problems else "OK", *problems, sep="\n")
sys.exit(1 if problems else 0)
