"""🧬 Hand-built EN 1996 check-outcome documents (pass and fail) from one committed mutation snapshot. Run: python -I doc-oracles-v2-en1996-fixtures.py"""
import copy
import json
import pathlib

ROOT = next(pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1996"))
FIXTURES = ROOT / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures"
base = json.load(open(FIXTURES / "🧬️mutations/🏋️change-concentrated-force/✅apply/🔣️.before.json", encoding="utf8"))


def passing():
    doc = copy.deepcopy(base)
    wall = doc["walls"][0]
    wall["loadCases"][0].update({"qPWindPa": 600.0, "hKEarthN": 0.0})
    snow = copy.deepcopy(wall["loadCases"][0])
    snow.update({"id": "uls-snow", "qKSnowPa": 1000.0, "hKEarthN": 15000.0})
    snow["concentrated"][0].update({"id": "beam-C", "forceN": 60000.0, "distanceA1M": 0.5})
    wall["loadCases"].append(snow)
    return doc


def failing():
    doc = passing()
    wall = doc["walls"][0]
    wall["id"] = "wall-slender"
    wall["thicknessM"], wall["heightM"], wall["lengthM"] = 0.175, 3.2, 3.0
    wall["unitWidthM"] = 0.175
    wall["eccentricityTopM"] = 0.05
    wall["loadCases"][0]["concentrated"][0].update({"forceN": 400000.0})
    wall["loadCases"][1].update({"hKEarthN": 90000.0, "qPWindPa": 1500.0})
    return doc


for name, doc in (("masonry-rich-pass", passing()), ("masonry-rich-fail", failing())):
    target = FIXTURES / "🚦️check-outcome" / name
    target.mkdir(parents=True, exist_ok=True)
    (target / "🔣️.json").write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n", encoding="utf8")
