"""🧫 Adds the clay case with two CPT readings (n < 3, cautious minimum) to the ground-derivation fixture; expected values are blessed afterwards by the oracle (`--bless`)."""
import copy
import json
import pathlib
import sys

FIXTURE = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🎯️ground-derivation/🔣️.json")
data = json.loads(FIXTURE.read_text(encoding="utf-8"))
if any(c["id"] == "clay-two-cpt-readings" for c in data["cases"]):
    sys.exit("case already present")
base = data["cases"][0]["document"]
clay = copy.deepcopy(next(l for l in base["layers"] if l["id"] == "clay-1"))
investigation = {"id": "cpt-B", "positionX": 0.0, "positionY": 0.0, "groundLevel": 0.0, "depth": 6.0, "kind": "cpt", "readings": [
    {"id": "cpt-B-r1", "depth": 1.0, "measurement": 618000.0},
    {"id": "cpt-B-r2", "depth": 2.0, "measurement": 666000.0}]}
document = {"annex": base["annex"], "groundwaterLevel": base["groundwaterLevel"], "layers": [clay], "investigations": [investigation],
    "characteristicBases": [{"id": "b1", "layerId": "clay-1", "parameter": "undrainedStrength", "basis": "derived"}]}
data["cases"].append({"id": "clay-two-cpt-readings", "description": "c_u of a clay from 2 CPT readings (n < 3): the lowest derived value, cautious", "document": document, "expected": {}})
FIXTURE.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("added clay-two-cpt-readings")
