"""Generates the EN 1996 showcase `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, its check-outcome fixtures `🚦️check-outcome/masonry-showcase-{pass,fail}` and the DSL asset `🖼️assets/🏘️masonry-showcase` (python3 -I en1996.py [--write|--check]).
Five walls from committed fixtures and documented derivations: a load-bearing wall with concentrated loads and a seismic load case, a slender braced wall, two shear walls with opening, lintel, shear bar, chase and movement joint, and a confined wall with piers, ties, prestress, confined element and flange. Magnitudes are documented inputs, never normative values."""
import copy, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load
from dslfmt import emit

ART = artifact("🪨️en1996")
FX = ART / "🧪️tests/🧫️fixtures"
SCHEMA = ART / "🧬️schema/🔣️.json"


class SuffixUnits:
    def get(self, key, default=None):
        for suffix, unit in (("M2", "m2"), ("Pa", "Pa"), ("M", "m")):
            if key.endswith(suffix):
                return unit
        return "N" if key.endswith("N") and key not in ("diaphragmStiffnessNPerM",) else default


def fixture(name):
    return load(FX / "🚦️check-outcome" / name / "🔣️.json")


def build():
    seismic, detail, demo = fixture("masonry-seismic-pass"), fixture("detail-rich-pass"), load(FX / "📇️catalogue/📚️demo-document/🔣️.json")
    bearing = seismic["walls"][0]
    slender = clone(bearing, id="wall-slender-ok", labelEn="Slender braced wall", labelDe="Schlanke ausgesteifte Wand", thicknessM=0.3, heightM=3.0, lengthM=4.0, supportSides=2, loadCases=copy.deepcopy(bearing["loadCases"][:2]))
    wall_a, wall_b = copy.deepcopy(detail["walls"])
    confined = clone(wall_b, id="wall-confined", labelEn="Confined shear wall with piers and prestress", labelDe="Eingefasste Schubwand mit Pfeilern und Vorspannung", bracingSystem="Confined", soilHeightM=1.0, planXM=0.0, planYM=12.0, planAngleDeg=0.0, inPlaneShearN=20000.0, inPlaneMomentNm=30000.0, diaphragmStiffnessNPerM=5000000000.0,
                     piers=[{"id": "pier-1", "positionM": 0.0, "widthM": 0.9}, {"id": "pier-2", "positionM": 2.7, "widthM": 0.9}],
                     ties=[{"id": "tie-1", "kind": "WallTie", "positionM": 1.0, "levelM": 1.0, "capacityN": 5000.0}, {"id": "tie-2", "kind": "RingTie", "positionM": 2.0, "levelM": 2.75, "capacityN": 40000.0}],
                     prestress=[{"id": "ps-1", "forceN": 60000.0, "areaM2": 0.0004, "fPkPa": 1770000000.0, "positionM": 2.0, "eccentricityM": 0.05, "lossRatio": 0.15}],
                     confined=[{"id": "conf-1", "positionM": 0.5, "widthM": 0.25, "depthM": 0.3, "asM2": 0.0002}],
                     flanges=[{"id": "fl-1", "lengthM": 0.9, "thicknessM": 0.365, "positionM": 4.0}])
    walls = [bearing, slender, wall_a, wall_b, confined]
    for plan_y, wall in zip((0.0, 3.0, 6.0, 9.0), walls):
        wall.update(planXM=0.0, planYM=plan_y)
    return {
        "annex": "De", "reliabilityClass": "RC2", "masonryClass": "Class1", "designSituation": "Persistent", "storeys": 2,
        "walls": walls, "placements": [{"id": f"place-{w['id']}", "x": 8.0 * i, "y": 0.0, "z": 0.0, "yaw": 0.0} for i, w in enumerate(walls)], "cases": copy.deepcopy(demo["cases"]),
        "chases": copy.deepcopy(detail["chases"]), "movementJoints": copy.deepcopy(detail["movementJoints"]),
    }


def failing(doc):
    doc = copy.deepcopy(doc)
    rows = {w["id"]: w for w in doc["walls"]}
    rows["wall-slender-ok"].update(thicknessM=0.175, heightM=3.2, lengthM=3.0)
    rows["wall-concentrated"]["loadCases"][1]["concentrated"][0]["forceN"] = 400000.0
    rows["wall-concentrated"]["loadCases"][2]["concentrated"][0]["forceN"] = 300000.0
    rows["wall-b"].update(thicknessM=0.115, heightM=6.0)
    doc["chases"][0]["depthM"] = 0.1
    return doc


def write_text(rel, text):
    path = ART / rel
    changed = not path.exists() or path.read_text(encoding="utf-8") != text
    if "--check" not in sys.argv and changed:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    print(("would change " if "--check" in sys.argv and changed else "wrote " if changed else "unchanged ") + rel)
    return changed and "--check" in sys.argv


def dsl(doc):
    template = next(ART.glob("🖼️assets/🧱️loadbearing-wall/*/🗣️.dsl.semio")).read_text(encoding="utf-8")
    return emit(doc, load(SCHEMA), template, "semio norm.en1996.dsl v1", units=SuffixUnits(), trailing=("reliabilityClass",))


def snapshot_validate(doc):
    import jsonschema
    errors = [e.message for e in jsonschema.Draft202012Validator(load(SCHEMA)).iter_errors(doc)]
    if errors:
        print("\n".join(errors[:40]))
        sys.exit(1)


if __name__ == "__main__":
    good = build()
    snapshot_validate(good)
    snapshot_validate(failing(good))
    drift = write_text("🖼️assets/🏘️masonry-showcase/🏘️masonry-showcase/🗣️.dsl.semio", dsl(good))
    for rel, doc in (("🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/masonry-showcase-pass/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/masonry-showcase-fail/🔣️.json", failing(good))):
        try:
            finish(doc, ART, rel)
        except SystemExit as stop:
            drift = drift or bool(stop.code)
    sys.exit(1 if drift and "--check" in sys.argv else 0)
