"""Dump the genetic generations (population, tournaments, children with crossover-only value) for scene planning."""
import importlib
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")
genetic, surface = model.genetic(), model.surface(41)
print("minimum", surface["minimum"])
for index, generation in enumerate(genetic["generations"]):
    lookup = {item["id"]: item for item in generation["population"]}
    print("GEN", index, "evals", generation["evaluations"], "best", generation["best"]["id"], round(generation["best"]["electricity"], 1), "elites", generation["elites"])
    for item in generation["population"]:
        print("   pop", item["id"], round(item["design"]["wwr"] * 100, 1), round(item["design"]["overhang"], 2), round(item["electricity"], 1))
    for duel in generation["tournaments"][:12]:
        print("   duel", duel)
    for child in generation["children"]:
        cross = {**lookup[child["from"][0]]["design"], **{key: lookup[child["genes"][key]]["design"][key] for key in ("wwr", "overhang")}}
        crossed = model.evaluate(cross)["electricity"]
        print("   child", child["id"], child["from"], child["genes"], {k: round(v, 3) for k, v in child["mutation"].items()}, round(crossed, 1), "->", round(child["electricity"], 1),
              round(child["design"]["wwr"] * 100, 1), round(child["design"]["overhang"], 2))
pattern = model.pattern_path()
for index, row in enumerate(pattern):
    print("PAT", index, {k: round(v, 3) for k, v in row["step"].items()}, row["moved"], round(row["design"]["wwr"], 3), round(row["design"]["overhang"], 3), round(row["electricity"], 1), row["evaluations"],
          [(p["direction"], round(p["design"]["wwr"], 3), round(p["design"]["overhang"], 3), round(p["electricity"], 1)) for p in row["probes"]])
gradient = model.gradient_path()
for index, row in enumerate(gradient):
    print("GRAD", index, round(row["design"]["wwr"], 3), round(row["design"]["overhang"], 3), round(row["electricity"], 1), {k: round(v, 3) for k, v in row["step"].items()}, {k: round(v, 3) for k, v in row["slope"].items()})
