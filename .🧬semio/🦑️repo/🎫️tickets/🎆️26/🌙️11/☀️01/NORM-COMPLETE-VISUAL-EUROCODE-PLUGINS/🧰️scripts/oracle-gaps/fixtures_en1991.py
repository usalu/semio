"""🧫 Writes the EN 1991 check-outcome fixture `hazards-fail` (showcase with under-assumed accidental and execution forces: helicopter, ties, derailment, falling load, execution imperfection, construction stage)."""
import json, pathlib
base = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
d = json.load(open(base / "🎪️showcase/🔣️.json", encoding="utf8"))
d["helicopterLandings"][0]["assumedForce"] = 100000.0
for run in d["tieRuns"]:
    run["providedForce"] = 20000.0
d["derailmentCases"][0]["assumedForce"] = 600000.0
d["fallingLoads"][0]["assumedForce"] = 10000.0
d["executionImperfections"][0]["assumedForce"] = 3000.0
d["constructionStages"][0]["assumedQk"] = 500.0
out = base / "🚦️check-outcome/hazards-fail"
out.mkdir(exist_ok=True)
with open(out / "🔣️.json", "w", encoding="utf8") as handle:
    json.dump(d, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
