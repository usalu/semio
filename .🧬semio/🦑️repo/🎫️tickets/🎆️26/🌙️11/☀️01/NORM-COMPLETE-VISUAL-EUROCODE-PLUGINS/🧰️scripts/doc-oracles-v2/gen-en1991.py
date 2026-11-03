"""Generates the EN 1991 check-outcome fixtures: every assumed value of a committed snapshot is scaled so that the oracle's required/assumed hits a target utilisation: python -I gen-en1991.py"""
import copy, importlib.util, json, pathlib

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any")
BASE = ROOT / "🧪️tests/🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/✅apply/🔣️.after.json"
OUT = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
spec = importlib.util.spec_from_file_location("o", ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
base = json.load(open(BASE, encoding="utf8"))
base.update(fireMode="nominal", fireCurve="standard", fireDuration=1800.0, fireOccupancy="office", windZone=2, terrainCategory=3)
base["roofs"].append({"id": "roof-flat", "roofType": "flat", "pitchDeg": 0.0, "cE": 1.0, "cT": 1.0, "hasParapet": False, "parapetHeight": 0.0, "driftObstructionHeight": 0.0, "multiSpan": False, "assumedSk": 200.0})
base["floors"].append({"id": "archive-l2", "category": "E1", "area": 120.0, "assumedQk": 3000.0, "assumedQkConcentrated": 5000.0, "assumedPartitions": 200.0})
base["selfWeightElements"].append({"id": "screed", "material": "cement_screed", "thickness": 0.05, "assumedGk": 1000.0})


def setter(doc, key, value):
    parts = key.split(".")
    ident = parts[-1]
    if key.startswith("en1991.1-7.impact."):
        case = parts[-2]
        for c in doc["accidentalCases"]:
            if c["id"] == case:
                c["impact"][int(ident)]["assumedForce"] = value
        return
    table = {"imposed": ("floors", "assumedQk"), "floor-force": ("floors", "assumedQk"), "concentrated": ("floors", "assumedQkConcentrated"), "partitions": ("floors", "assumedPartitions"),
             "self-weight": ("selfWeightElements", "assumedGk"), "snow": ("roofs", "assumedSk"), "wind": ("windFaces", "assumedWp")}
    family = parts[-2]
    if family in table:
        for row in doc[table[family][0]]:
            if row["id"] == ident:
                row[table[family][1]] = value
        return
    scalars = {"en1991.1-2.gas-temperature": "assumedGasTemperature", "en1991.1-2.h-net": "assumedHNet", "en1991.1-2.fire-load": "assumedQfD", "en1991.1-5.thermal": "assumedDeltaT", "en1991.1-6.construction": "assumedConstructionQk"}
    doc[scalars[key]] = value


def current(doc, key):
    parts = key.split(".")
    if key.startswith("en1991.1-7.impact."):
        return next(c for c in doc["accidentalCases"] if c["id"] == parts[-2])["impact"][int(parts[-1])]["assumedForce"]
    table = {"imposed": ("floors", "assumedQk"), "floor-force": ("floors", "assumedQk"), "concentrated": ("floors", "assumedQkConcentrated"), "partitions": ("floors", "assumedPartitions"),
             "self-weight": ("selfWeightElements", "assumedGk"), "snow": ("roofs", "assumedSk"), "wind": ("windFaces", "assumedWp")}
    if parts[-2] in table:
        return next(r for r in doc[table[parts[-2]][0]] if r["id"] == parts[-1])[table[parts[-2]][1]]
    return doc[{"en1991.1-2.gas-temperature": "assumedGasTemperature", "en1991.1-2.h-net": "assumedHNet", "en1991.1-2.fire-load": "assumedQfD", "en1991.1-5.thermal": "assumedDeltaT", "en1991.1-6.construction": "assumedConstructionQk"}[key]]


def scale(doc, targets):
    rows = oracle.evaluate(doc)
    for index, (key, u) in enumerate(sorted(rows.items())):
        if u > 0 and ".floor-force." not in key:
            setter(doc, key, current(doc, key) * u / targets[index % len(targets)])
    return doc


for name, targets in (("actions-all-pass", (0.55, 0.7, 0.8, 0.9, 0.6)), ("actions-all-fail", (1.3, 1.6, 2.0, 1.15, 1.5))):
    doc = scale(copy.deepcopy(base), targets)
    (OUT / name).mkdir(parents=True, exist_ok=True)
    json.dump(doc, open(OUT / name / "🔣️.json", "w", encoding="utf8"), ensure_ascii=False, indent=2)
