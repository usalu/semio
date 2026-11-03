"""🧱 Completes the EN 1998 check-outcome fixtures with the wall water and foundation sliding fields and builds the walls pass/fail documents. Run: python -I fixtures.py"""
import copy, json, pathlib

ROOT = next(pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1998")) / "🏅️standards/🔖️1/🪆️subsets/✳️any"
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
defs = json.load(open(ROOT / "🧬️schema/🔣️.json", encoding="utf8"))["$defs"]
ENTITIES = {"retainingWalls": "En1998RetainingWall", "foundations": "En1998Foundation"}


def complete(doc):
    for key, name in ENTITIES.items():
        order = list(defs[name]["properties"])
        doc[key] = [{k: row.get(k, 0.0) for k in order} for row in doc[key]]
    return doc


def save(path, doc):
    path.parent.mkdir(exist_ok=True)
    open(path, "w", encoding="utf8").write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")


for p in sorted(FIX.glob("*/🔣️.json")):
    doc = json.load(open(p, encoding="utf8"))
    save(p, complete(doc))

base = json.load(open(FIX / "seismic-tie-slope-pass/🔣️.json", encoding="utf8"))


def walls(fail):
    doc = copy.deepcopy(base)
    wall = doc["retainingWalls"][0]
    wall.update({"heightM": 4.0, "phiDeg": 32.0, "soilGamma": 18000.0, "r": 1.5, "waterHeightM": 2.0, "saturatedGamma": 20000.0, "permeabilityMS": 0.001, "hRdNPerM": 30000.0 if fail else 400000.0})
    dry = copy.deepcopy(wall)
    dry.update({"id": "rw-2", "waterHeightM": 0.0, "saturatedGamma": 0.0, "permeabilityMS": 0.0, "hRdNPerM": 40000.0 if fail else 250000.0})
    low = copy.deepcopy(wall)
    low.update({"id": "rw-3", "permeabilityMS": 0.0, "hRdNPerM": 15000.0 if fail else 400000.0})
    doc["retainingWalls"] = [wall, dry, low]
    for f, passive, angle in zip(doc["foundations"], (300000.0, 0.0), (1.0, 0.5)):
        f.update({"baseFrictionAngleDeg": angle if fail else 30.0, "basePorePressurePa": 0.0, "passiveResistanceN": 0.0 if fail else passive})
    doc["tieBeams"], doc["slopes"] = [], []
    return doc


save(FIX / "seismic-walls-pass/🔣️.json", complete(walls(False)))
save(FIX / "seismic-walls-fail/🔣️.json", complete(walls(True)))
