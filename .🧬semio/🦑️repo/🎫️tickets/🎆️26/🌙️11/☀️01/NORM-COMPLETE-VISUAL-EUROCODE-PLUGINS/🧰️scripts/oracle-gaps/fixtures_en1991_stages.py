"""🧫 Writes the EN 1991 check-outcome fixture `stages-en`: the showcase under the EN annex with one execution stage per Table 3.1 duration class (EN 1991-1-6), W3-CORR-1991-1998."""
import json, pathlib
base = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
doc = json.load(open(base / "🎪️showcase/🔣️.json", encoding="utf8"))
doc["annex"] = "En"
doc["constructionStages"] = [
    {"id": "brief", "activity": "personnel", "durationDays": 2.0, "returnYears": 4.0, "assumedQk": 1500.0},
    {"id": "short", "activity": "personnel", "durationDays": 30.0, "returnYears": 2.0, "assumedQk": 1500.0},
    {"id": "mid", "activity": "personnel", "durationDays": 200.0, "returnYears": 5.0, "assumedQk": 1500.0},
    {"id": "long", "activity": "personnel", "durationDays": 400.0, "returnYears": 100.0, "assumedQk": 1500.0},
]
out = base / "🚦️check-outcome" / "stages-en"
out.mkdir(exist_ok=True)
with open(out / "🔣️.json", "w", encoding="utf8") as handle:
    json.dump(doc, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
