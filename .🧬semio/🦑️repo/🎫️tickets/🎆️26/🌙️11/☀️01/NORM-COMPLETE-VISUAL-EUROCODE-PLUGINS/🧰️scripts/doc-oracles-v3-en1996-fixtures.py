"""🧬 Hand-built EN 1996 detailing check-outcome documents (pass and fail): fire thickness, chases, movement joints, openings with lintels, reinforced masonry, bar layout and the floor diaphragm of two bracing walls. Run: python -I doc-oracles-v3-en1996-fixtures.py"""
import copy
import json
import pathlib

ROOT = next(pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1996"))
FIXTURES = ROOT / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome"
base = json.load(open(FIXTURES / "masonry-rich-pass/🔣️.json", encoding="utf8"))


def walls():
    a = copy.deepcopy(base["walls"][0])
    a.update({"id": "wall-a", "labelEn": "Reinforced wall with window", "labelDe": "Bewehrte Wand mit Fenster", "reinforced": True, "asVerticalM2": 1.0e-3, "asHorizontalM2": 4.0e-4, "fYdPa": 435e6, "bracingSystem": "Reinforced", "inPlaneShearN": 0.0, "diaphragmStiffnessNPerM": 1.0e12, "wallType": "Shear"})
    a["openings"] = [{"id": "win-1", "widthM": 1.2, "heightM": 1.4, "sillHeightM": 0.9, "positionM": 0.8}]
    a["lintels"] = [{"id": "lintel-1", "positionM": 0.6, "spanM": 1.4, "depthM": 0.25, "bearingLengthM": 0.25}]
    a["bars"] = [{"id": "bar-shear", "kind": "Shear", "diameterM": 0.008, "lengthM": 0.5, "spacingM": 0.4}]
    b = copy.deepcopy(a)
    b.update({"id": "wall-b", "labelEn": "Second bracing wall", "labelDe": "Zweite Aussteifungswand", "lengthM": 4.0, "planYM": 6.0, "openings": [], "lintels": [], "bars": [], "inPlaneShearN": 0.0})
    return a, b


def passing():
    doc = copy.deepcopy(base)
    doc["walls"] = list(walls())
    doc["chases"] = [{"id": "chase-1", "wallId": "wall-a", "kind": "Vertical", "xM": 3.0, "yM": 0.4, "widthM": 0.05, "lengthM": 1.2, "depthM": 0.02}]
    doc["movementJoints"] = [{"id": "joint-1", "wallId": "wall-a", "positionM": 2.5}]
    return doc


def failing():
    doc = passing()
    a, b = doc["walls"]
    a["openings"][0]["sillHeightM"] = 1.9
    a["lintels"][0].update({"positionM": 1.0, "spanM": 0.9})
    a["bars"][0]["spacingM"] = 0.004
    a.update({"asVerticalM2": 1.0e-4, "asHorizontalM2": 0.0, "diaphragmStiffnessNPerM": 1.0e5})
    b.update({"lengthM": 30.0, "thicknessM": 0.175, "unitWidthM": 0.175, "fireReiMin": 180, "diaphragmStiffnessNPerM": 1.0e5})
    doc["chases"][0]["depthM"] = 0.06
    doc["movementJoints"] = [{"id": "joint-1", "wallId": "wall-b", "positionM": 8.0}, {"id": "joint-2", "wallId": "wall-b", "positionM": 26.0}]
    return doc


for name, doc in (("detail-rich-pass", passing()), ("detail-rich-fail", failing())):
    target = FIXTURES / name
    target.mkdir(parents=True, exist_ok=True)
    (target / "🔣️.json").write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n", encoding="utf8")
