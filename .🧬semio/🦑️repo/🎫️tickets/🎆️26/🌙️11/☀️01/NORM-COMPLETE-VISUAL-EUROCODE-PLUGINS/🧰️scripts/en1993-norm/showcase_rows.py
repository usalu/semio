"""Adds the wave 3 norm rows (instantaneous centre plate, stress range, buckling length, forming gain, fastened joint, material ductility) to the committed EN 1993 showcase fixture; idempotent (existing rows win), validated against the artifact schema (python3 -I showcase_rows.py)."""
import copy, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1] / "showcase"))
from common import artifact, coerce, load, validate
import json

ART = artifact("🔩️en1993")
OUT = ART / "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"
doc = load(OUT)
plate_ic = copy.deepcopy(next(r for r in doc["jointPlates"] if r["id"] == "plate-fin"))
plate_ic.update(id="plate-ic", label="Fin plate (instantaneous centre)", boltsPerRow=2, gauge=0.1, eccentricity=0.15, groupMethod="InstantaneousCentre")
rows = {
    "jointPlates": [plate_ic],
    "fatigueStresses": [{"id": "fstress-1", "detailId": "fat-1", "maxCaseId": "g-permanent", "minCaseId": "q-imposed", "fibreZ": 0.12, "fibreY": 0.0, "relief": "StressRelieved", "concentration": 1.0, "thickness": 0.017, "cycles": 1.0e7}],
    "bucklingLengths": [{"id": "buckling-1", "memberId": "member-b1", "plane": "MajorAxis", "sway": "NonSway", "restraintA": "Framed", "columnStiffnessA": 5.63e-5, "beamStiffnessA": 5.63e-5, "restraintB": "Framed", "columnStiffnessB": 5.63e-5, "beamStiffnessB": 5.63e-5}],
    "formingGains": [{"id": "gain-1", "memberId": "cf-1", "forming": "RollForming", "ultimateStrength": 420.0e6, "grossArea": 0.0006, "bends": 4, "usedYield": 370.0e6}],
    "fastenedJoints": [{"id": "fastened-1", "memberId": "cf-1", "kind": "SelfTappingScrew", "diameter": 0.0055, "washerDiameter": 0.016, "count": 4, "perRow": 2, "headThickness": 0.00075, "baseThickness": 0.0015, "headStrength": 360.0e6, "baseStrength": 360.0e6, "endDistance": 0.03, "edgeDistance": 0.015, "pitch": 0.03, "gauge": 0.03, "sheetWidth": 0.1, "shearForce": 4000.0, "tensionForce": 1200.0, "shearResistance": 0.0}],
    "materialDuctility": [{"id": "ductility-1", "materialId": "mat-s355", "elongation": 0.20, "ultimateStrain": 0.12}],
}
for name, new in rows.items():
    have = {r["id"] for r in doc.get(name, [])}
    doc[name] = doc.get(name, []) + [r for r in new if r["id"] not in have]
schema = load(ART / "🧬️schema/🔣️.json")
doc = coerce(doc, schema, schema)
errors = validate(doc, schema, schema)
if errors:
    print("\n".join(errors[:40]))
    sys.exit(1)
text = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
if OUT.read_text(encoding="utf-8") != text:
    OUT.write_text(text, encoding="utf-8")
print({k: len(v) for k, v in doc.items() if isinstance(v, list) and k in rows})
