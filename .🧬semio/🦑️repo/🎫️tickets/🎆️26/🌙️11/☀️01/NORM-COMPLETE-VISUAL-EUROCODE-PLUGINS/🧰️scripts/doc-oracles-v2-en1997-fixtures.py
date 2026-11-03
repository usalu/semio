"""🧬 Hand-built EN 1997 check-outcome documents (pass and fail) from one committed mutation snapshot. Run: python -I doc-oracles-v2-en1997-fixtures.py"""
import copy
import json
import pathlib

ROOT = next(pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1997"))
FIXTURES = ROOT / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures"
base = json.load(open(FIXTURES / "🧬️mutations/↔️change-footing-width/✅apply/🔣️.before.json", encoding="utf8"))


def passing():
    doc = copy.deepcopy(base)
    second = copy.deepcopy(doc["footings"][0])
    second["id"] = "footing-F2"
    second["width"], second["length"] = 2.0, 2.0
    doc["footings"].append(second)
    doc["placements"] = [{"id": "footing-F1", "x": 0.0, "y": 0.0, "z": 0.0, "yaw": 0.0}, {"id": "footing-F2", "x": 7.0, "y": 1.0, "z": 0.0, "yaw": 0.0}]
    doc["slopes"][0]["angleDeg"] = 20.0
    return doc


def failing():
    doc = passing()
    doc["footings"][0]["loadCases"][0].update({"verticalPermanent": 3000000.0, "horizontalPermanent": 400000.0})
    doc["footings"][1]["loadCases"][0].update({"verticalPermanent": 100000.0, "verticalVariable": 20000.0})
    doc["piles"][0].update({"compressionPermanent": 3000000.0})
    doc["retainingWalls"][0].update({"height": 7.0, "horizontalPermanent": 250000.0})
    doc["slopes"][0]["angleDeg"] = 38.0
    doc["upliftCases"][0].update({"permanentStabilizing": 400000.0})
    doc["anchors"][0].update({"designForce": 900000.0})
    return doc


for name, doc in (("geotechnical-rich-pass", passing()), ("geotechnical-rich-fail", failing())):
    target = FIXTURES / "🚦️check-outcome" / name
    target.mkdir(parents=True, exist_ok=True)
    (target / "🔣️.json").write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n", encoding="utf8")
