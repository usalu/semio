"""Generates the EN 1992 showcase `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, its check-outcome fixtures `🚦️check-outcome/rc-building-showcase-{pass,fail}` and the DSL asset `🖼️assets/🏢️rc-building-showcase` (python3 -I en1992.py [--write|--check]).
Members come from committed fixtures (beam, prestressed tank wall with section/bar/tendon points, flat slabs with stud rows, unbalanced-moment beta method and openings), derived members (slab, column, wall, ribbed slab, tension member, bridge deck) use documented inputs; grades, anchors (fastening kernel) and the 115 catalogue cases come from the demo document."""
import copy, json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load
from dslfmt import emit

ART = artifact("🏛️en1992")
FX = ART / "🧪️tests/🧫️fixtures"
SCHEMA = ART / "🧬️schema/🔣️.json"


def fixture(name):
    return load(FX / "🚦️check-outcome" / name / "🔣️.json")


def pick(doc, mid):
    return copy.deepcopy(next(m for m in doc["members"] if m["id"] == mid))


def labels(m, en, de):
    return clone(m, labelEn=en, labelDe=de)


def build():
    beam_doc, wall_doc, moments, openings, demo = fixture("beam-sls-pass"), fixture("prestressed-wall-pass"), fixture("flat-slab-punching-moments"), fixture("flat-slab-punching-openings"), load(FX / "📇️catalogue/📚️demo-document/🔣️.json")
    beam = labels(pick(beam_doc, "beam-B1"), "Floor beam B1", "Deckenbalken B1")
    wall = labels(pick(wall_doc, "wall-W1"), "Prestressed tank wall W1", "Vorgespannte Behälterwand W1")
    wall["sectionPoints"] = [{"id": f"sp-{i + 1}", "y": y, "z": z} for i, (y, z) in enumerate(((-0.175, -0.25), (0.175, -0.25), (0.175, 0.25), (-0.175, 0.25)))]
    wall["bars"] = [{"id": f"bar-{i + 1}", "layerId": "vert", "y": -0.125 + 0.05 * i, "z": -0.2, "diameter": 0.02, "length": 0.0, "bend": "Auto"} for i in range(6)]
    interior = labels(pick(moments, "slab-FS-interior"), "Flat slab interior column, unbalanced moments", "Flachdecke Innenstütze, Momentenanteil")
    edge = labels(pick(moments, "slab-FS-edge"), "Flat slab edge column, unbalanced moments", "Flachdecke Randstütze, Momentenanteil")
    corner = labels(pick(moments, "slab-FS-corner"), "Flat slab corner column, unbalanced moments", "Flachdecke Eckstütze, Momentenanteil")
    for row in corner["studRows"]:
        row["spacingRadial"] = 0.17
    near = labels(pick(openings, "slab-FS-interior-opening-near"), "Flat slab, opening next to the column", "Flachdecke, Öffnung neben der Stütze")
    slab = clone(beam, id="slab-S1", labelEn="One-way floor slab S1", labelDe="Einachsig gespannte Deckenplatte S1", kind="Slab", width=1.0, height=0.25, effectiveDepth=0.21, span=4.5, stirrups=None, tightness=None, hdOverH=0.0, liquidSigmaS=0.0, liquidRhoPEff=0.0, liquidFCtEff=0.0, liquidSRMax=0.0,
                 longitudinal=[{**beam["longitudinal"][0], "id": "bot", "diameter": 0.012, "count": 9, "anchorageLength": 0.45, "lapLength": 0.55}], fire={"rating": "R60", "axisDistance": 0.03, "slabSystem": "OneWay"}, deflectionSensitive=True,
                 actions=[{**beam["actions"][0], "gKLine": 6000.0}, {**beam["actions"][1], "qKLine": 3000.0}], useFem=False)
    ribbed = clone(slab, id="slab-R1", labelEn="Ribbed slab R1", labelDe="Rippendecke R1", kind="RibbedSlab", width=0.6, height=0.35, effectiveDepth=0.3, span=7.5, stirrups=beam["stirrups"],
                   longitudinal=[{**slab["longitudinal"][0], "diameter": 0.02, "count": 2}], actions=[{**beam["actions"][0], "gKLine": 3500.0}, {**beam["actions"][1], "qKLine": 2400.0}])
    column = clone(beam, id="column-C1", labelEn="Braced column C1", labelDe="Ausgesteifte Stütze C1", kind="Column", width=0.4, height=0.4, effectiveDepth=0.35, span=3.2, support="Fixed", bucklingLength=2.4, tightness=None, hdOverH=0.0, liquidSigmaS=0.0, liquidRhoPEff=0.0, liquidFCtEff=0.0, liquidSRMax=0.0,
                   longitudinal=[{**beam["longitudinal"][0], "id": "tens", "diameter": 0.02, "count": 4, "position": "Bottom"}, {**beam["longitudinal"][0], "id": "comp", "diameter": 0.02, "count": 4, "position": "Top"}],
                   actions=[{**beam["actions"][0], "source": "External", "gKLine": 0.0, "nK": -600000.0, "mK": 25000.0, "vK": 15000.0}, {**beam["actions"][1], "source": "External", "qKLine": 0.0, "nK": -300000.0, "mK": 15000.0, "vK": 10000.0}])
    shear_wall = clone(column, id="wall-SW1", labelEn="Shear wall SW1", labelDe="Schubwand SW1", kind="Wall", width=0.25, height=2.5, effectiveDepth=2.3, span=3.0, bucklingLength=2.1,
                       longitudinal=[{**column["longitudinal"][0], "diameter": 0.012, "count": 12}, {**column["longitudinal"][1], "diameter": 0.012, "count": 12}])
    tie = clone(beam, id="tie-T1", labelEn="Tension member T1", labelDe="Zugglied T1", kind="TensionMember", width=0.3, height=0.3, effectiveDepth=0.26, span=5.0, support="Fixed", tightness=None, hdOverH=0.0, liquidSigmaS=0.0, liquidRhoPEff=0.0, liquidFCtEff=0.0, liquidSRMax=0.0,
                longitudinal=[{**beam["longitudinal"][0], "diameter": 0.025, "count": 6}], actions=[{**beam["actions"][0], "source": "External", "gKLine": 0.0, "nK": 400000.0}, {**beam["actions"][1], "source": "External", "qKLine": 0.0, "nK": 200000.0}])
    bridge = clone(beam, id="deck-D1", labelEn="Bridge deck girder D1", labelDe="Brückenüberbau D1", kind="Bridge", exposure="Xd1", width=0.5, height=1.2, effectiveDepth=1.12, span=14.0, tightness=None, hdOverH=0.0, liquidSigmaS=0.0, liquidRhoPEff=0.0, liquidFCtEff=0.0, liquidSRMax=0.0,
                   bridgeSigmaC=10000000.0, bridgeDeltaSigmaS=60000000.0, longitudinal=[{**beam["longitudinal"][0], "diameter": 0.032, "count": 8}],
                   actions=[{**beam["actions"][0], "gKLine": 45000.0}, {**beam["actions"][1], "qKLine": 22000.0}])
    members = [beam, slab, ribbed, column, shear_wall, tie, bridge, wall, interior, edge, corner, near]
    anchor = copy.deepcopy(demo["anchors"][0])
    for action in anchor["actions"]:
        action["nK"], action["vK"] = 0.7 * action["nK"], 0.7 * action["vK"]
    second = clone(anchor, id="anc-2", hEf=0.15, cracked=False, fUk=800000000.0, fYk=640000000.0, aS=0.000157, d=0.016, c1=0.2, fCk=30000000.0, headDiameter=0.04,
                   actions=[{**anchor["actions"][0], "nK": 12000.0, "vK": 3000.0}, {**anchor["actions"][1], "nK": 6000.0, "vK": 2000.0}])
    return {
        "annex": "De", "reliabilityClass": "RC2", "title": "Reinforced concrete office building with tank, bridge and fastenings", "designWorkingLifeYears": 50.0, "deltaCDev": 0.01, "cementType": "N",
        "concreteGrades": [{"id": "c30", "name": "C30/37", "fCk": 30000000.0}, {"id": "c35", "name": "C35/45", "fCk": 35000000.0}],
        "reinforcementGrades": copy.deepcopy(beam_doc["reinforcementGrades"]), "prestressSteels": copy.deepcopy(wall_doc["prestressSteels"]),
        "members": members, "anchors": [anchor, second],
        "placements": [{"id": f"place-{m['id']}", "x": 8.0 * i, "y": 0.0, "z": 0.0, "yaw": 0.0} for i, m in enumerate(members)],
        "cases": copy.deepcopy(demo["cases"]),
    }


def failing(doc):
    doc = copy.deepcopy(doc)
    rows = {m["id"]: m for m in doc["members"]}
    rows["beam-B1"]["stirrups"]["spacing"] = 0.6
    rows["beam-B1"]["actions"][1]["qKLine"] = 60000.0
    rows["slab-S1"]["actions"][1]["qKLine"] = 12000.0
    rows["column-C1"]["actions"][0]["mK"] = 250000.0
    rows["slab-FS-interior-opening-near"]["actions"][0]["vKPunch"] = 700000.0
    rows["slab-FS-interior"]["actions"][0]["vKPunch"] = 700000.0
    rows["deck-D1"]["longitudinal"][0]["count"] = 3
    for action in doc["anchors"][1]["actions"]:
        action["nK"], action["vK"] = 4.0 * action["nK"], 4.0 * action["vK"]
    doc["title"] = "Reinforced concrete office building with overloaded members"
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
    template = next(ART.glob("🖼️assets/🛢️liquid-retaining-fem-anchor/*/🗣️.dsl.semio")).read_text(encoding="utf-8")
    return emit(doc, load(SCHEMA), template, "semio norm.en1992.dsl v1", trailing=("reliabilityClass",))


def snapshot_validate(doc):
    import jsonschema
    schema = load(SCHEMA)
    errors = [e.message for e in jsonschema.Draft202012Validator(schema).iter_errors(doc)]
    if errors:
        print("\n".join(errors[:40]))
        sys.exit(1)


if __name__ == "__main__":
    good = build()
    snapshot_validate(good)
    snapshot_validate(failing(good))
    drift = write_text("🖼️assets/🏢️rc-building-showcase/🏢️rc-building-showcase/🗣️.dsl.semio", dsl(good))
    for rel, doc in (("🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/rc-building-showcase-pass/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/rc-building-showcase-fail/🔣️.json", failing(good))):
        try:
            finish(doc, ART, rel, compact="🎪️showcase" in rel)
        except SystemExit as stop:
            drift = drift or bool(stop.code)
    sys.exit(1 if drift and "--check" in sys.argv else 0)
