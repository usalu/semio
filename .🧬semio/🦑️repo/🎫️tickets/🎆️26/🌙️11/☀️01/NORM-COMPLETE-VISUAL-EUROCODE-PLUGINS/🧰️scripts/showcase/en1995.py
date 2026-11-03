"""Generates the EN 1995 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`: the committed design-cases document extended with one row of every typed entity (taper, notch and hole, fastener pattern, CLT panel, floor, wall and roof diaphragm, truss, bracing, stress-laminated deck) taken from the committed mutation fixtures, so every diagram and entity of the artifact has a populated row."""
import copy, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load

ART = artifact("🪵️en1995")
FIXTURES = ART / "🧪️tests/🧫️fixtures"
doc = copy.deepcopy(load(FIXTURES / "🚦️check-outcome/design-cases-pass/🔣️.json"))
COLLECTIONS = ("tapers", "cutouts", "fasteners", "panels", "floors", "diaphragms", "trusses", "bracings", "decks")


def first_row(collection, row_id):
    for path in sorted(FIXTURES.glob("🧬️mutations/*/✅apply/🔣️.before.json")):
        rows = load(path).get(collection, [])
        hit = next((r for r in rows if r["id"] == row_id), None)
        if hit:
            return hit
    sys.exit(f"no committed fixture row {collection}[{row_id}]")


SINGULAR = {"tapers": "taper", "cutouts": "cutout", "fasteners": "fastener", "panels": "panel", "floors": "floor", "diaphragms": "diaphragm", "trusses": "truss", "bracings": "bracing", "decks": "deck"}
for collection in COLLECTIONS:
    doc[collection] = [first_row(collection, f"{SINGULAR[collection]}-1")]
doc["cutouts"].append(clone(doc["cutouts"][0], id="cutout-2", kind="hole"))
doc["tapers"].append(clone(doc["tapers"][0], id="taper-2", kind="single-tapered", taperAngleRad=0.08, apexDepthM=0.0, tensionSide="compression"))
doc["diaphragms"].append(clone(doc["diaphragms"][0], id="diaphragm-2", kind="roof"))
doc["placements"] = [{"id": "beam-B1", "x": 0.0, "y": 0.0, "z": 0.0, "yaw": 0.0}]
finish(doc, ART, "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json")
