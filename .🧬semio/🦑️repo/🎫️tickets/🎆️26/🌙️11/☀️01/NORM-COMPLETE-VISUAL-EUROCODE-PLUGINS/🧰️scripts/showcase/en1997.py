"""Generates the EN 1997 showcase `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, its check-outcome fixtures `🚦️check-outcome/ground-showcase-{pass,fail}` and the DSL asset `🖼️assets/🏘️ground-showcase` (python3 -I en1997.py [--write|--check]).
Three layers (sand, clay with CPT N_kt, gravel) under a footing, pile, wall, slope, uplift/heave, anchor and embedded-wall design; investigations (borehole, CPT, SPT with readings), characteristic bases (derived and declared), retaining-wall water (layer waters with a well) and a heave case. Rows start from the committed `geotechnical-rich-pass` document; the 71 catalogue cases come from the demo document. Magnitudes are documented inputs, never normative values."""
import copy, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load
from dslfmt import emit

ART = artifact("🌍️en1997")
FX = ART / "🧪️tests/🧫️fixtures"
SCHEMA = ART / "🧬️schema/🔣️.json"
UNITS = {k: "m" for k in "depthTop depthBottom width length embedment settlementLimit diameter height baseWidth stemThickness reinforcementSpacing reinforcementWidth circleCentreX circleCentreZ circleRadius headDepth freeLength fixedLength fixedDiameter minFreeLength retainedHeight propDepth thickness spacing drainageHeight piezometricDepth headDifference flowLength wellRadius wellWaterDepth influenceRadius settlingThickness dropHeight setPerBlow elasticCompression groundLevel depth positionX positionY groundwaterLevel investigationDepth".split()}
UNITS |= {k: "Pa" for k in "cohesionEffective cohesionUndrained oedometricModulus cptQc unitShaftResistance unitBaseResistance elasticModulus surcharge bondStress shaftCohesionSum baseCohesion preloadStress stressIncrement porePressure totalStress normalStress shearStress undrainedStrength".split()}
UNITS |= {k: "N" for k in "verticalPermanent verticalVariable horizontalPermanent horizontalVariable compressionPermanent compressionVariable tensionPermanent tensionVariable lateralHeadLoad dynamicResistanceMean dynamicResistanceMin hammerWeight designForce tendonYieldForce capVertical singleResistance permanentStabilizing permanentDestabilizing variableDestabilizing shaftResistance baseResistance pullOutResistance".split()}
UNITS |= {k: "J" for k in "momentPermanent momentVariable capMomentX capMomentY".split()}


def readings(prefix, profile):
    return [{"id": f"{prefix}-{i + 1}", "depth": depth, "measurement": value} for i, (depth, value) in enumerate(profile)]


def build():
    base = load(FX / "🚦️check-outcome/geotechnical-rich-pass/🔣️.json")
    demo = load(FX / "📇️catalogue/📚️demo-document/🔣️.json")
    sand, gravel = base["layers"]
    sand = clone(sand, depthBottom=5.0)
    clay = {"id": "layer-clay", "soilType": "clay", "depthTop": 5.0, "depthBottom": 8.0, "gamma": 18500.0, "gammaPrime": 8500.0, "phiPrimeDeg": 24.0, "cohesionEffective": 5000.0, "cohesionUndrained": 60000.0, "oedometricModulus": 8000000.0, "poissonRatio": 0.35, "cptQc": 1200000.0, "sptN": 8.0, "cptNkt": 15.0}
    pile = base["piles"][0]
    driven = clone(pile, id="pile-P2", pileType="driven", diameter=0.4, length=12.0, alphaS=0.6, unitShaftResistance=60000.0, unitBaseResistance=5000000.0, compressionPermanent=350000.0, compressionVariable=80000.0, tensionPermanent=50000.0, tensionVariable=30000.0,
                   bendingStiffnessEi=100000000.0, horizontalSubgradeModulus=5000000.0, lateralHeadLoad=15000.0, elasticModulus=30000000000.0, settlingThickness=3.0, negativeFrictionBeta=0.25, dynamicTestCount=5.0, dynamicResistanceMean=1400000.0, dynamicResistanceMin=1250000.0,
                   hammerWeight=25000.0, dropHeight=1.0, setPerBlow=0.004, elasticCompression=0.01, hammerEfficiency=0.8, concreteClass="c3037", testProfiles=[{"id": "test-1", "shaftResistance": 1100000.0, "baseResistance": 600000.0}, {"id": "test-2", "shaftResistance": 1050000.0, "baseResistance": 620000.0}])
    wall = base["retainingWalls"][0]
    reinforced = clone(wall, id="wall-W2", height=5.0, embedment=1.0, baseWidth=3.5, earthPressureMode="increasedActive", wallMovement="propped", ocr=1.5, seismicKh=0.1, seismicKv=0.05, reinforcementSpacing=0.6, reinforcementWidth=0.1, reinforcementFriction=0.5, verticalPermanent=420000.0, horizontalPermanent=50000.0)
    slope = base["slopes"][0]
    circle = clone(slope, id="slope-S2", angleDeg=26.0, height=8.0, length=16.0, governingLayerId="layer-clay", circleCentreX=4.0, circleCentreZ=12.0, circleRadius=11.0, width=1.0)
    heave = {"id": "upl-U2", "permanentStabilizing": 700000.0, "permanentDestabilizing": 300000.0, "variableDestabilizing": 30000.0, "porePressure": 30000.0, "totalStress": 120000.0}
    cantilever = clone(base["embeddedWalls"][0], id="ewall-E2", kind="cantilever", retainedHeight=3.0, embedment=5.5, propDepth=0.0, thickness=0.5, undrainedStrength=60000.0)
    group2 = clone(base["pileGroups"][0], id="group-G2", pileId="pile-P2", rows=1, columns=3, spacing=1.2, capVertical=1500000.0, capMomentX=80000.0, capMomentY=0.0, singleResistance=1100000.0, shaftCohesionSum=300000.0, baseCohesion=90000.0)
    oedometer = {"id": "lab-T2", "layerId": "layer-clay", "kind": "oedometer", "voidRatio": 0.9, "preloadStress": 80000.0, "drainageHeight": 0.01, "consolidationTime": 180.0, "compressionIndex": 0.25, "swellingIndex": 0.05, "stressIncrement": 100000.0, "secondaryCompressionIndex": 0.01, "designLife": 50.0, "points": []}
    triaxial = {"id": "lab-T3", "layerId": "layer-gravel", "kind": "triaxial", "voidRatio": 0.5, "preloadStress": 150000.0, "drainageHeight": 0.01, "consolidationTime": 90.0, "compressionIndex": 0.0, "swellingIndex": 0.0, "stressIncrement": 0.0, "secondaryCompressionIndex": 0.0, "designLife": 0.0,
                "points": [{"id": "pt-1", "normalStress": 100000.0, "shearStress": 70000.0}, {"id": "pt-2", "normalStress": 200000.0, "shearStress": 140000.0}, {"id": "pt-3", "normalStress": 300000.0, "shearStress": 210000.0}]}
    waters = [base["layerWaters"][0], {"id": "water-clay", "layerId": "layer-clay", "piezometricDepth": 4.0, "permeability": 0.000000001, "headDifference": 0.8, "flowLength": 3.0, "wellRadius": 0.0, "wellWaterDepth": 0.0, "influenceRadius": 0.0, "wellCapacity": 0.0},
              {"id": "water-sand-well", "layerId": "layer-sand", "piezometricDepth": 4.0, "permeability": 0.0001, "headDifference": 1.0, "flowLength": 6.0, "wellRadius": 0.15, "wellWaterDepth": 3.0, "influenceRadius": 60.0, "wellCapacity": 0.01}]
    cpt = [(0.5, 7500000.0), (1.5, 8000000.0), (2.5, 8500000.0), (3.5, 8200000.0), (4.5, 8800000.0), (5.5, 1100000.0), (6.5, 1200000.0), (7.5, 1300000.0), (9.0, 11500000.0), (11.0, 12500000.0)]
    spt = [(1.5, 16.0), (3.0, 18.0), (4.5, 19.0), (6.0, 8.0), (9.0, 27.0), (12.0, 29.0)]
    investigations = [
        {"id": "cpt-1", "positionX": 0.0, "positionY": 0.0, "groundLevel": 0.0, "depth": 12.0, "kind": "cpt", "readings": readings("c1", cpt)},
        {"id": "cpt-2", "positionX": 12.0, "positionY": 0.0, "groundLevel": 0.0, "depth": 12.0, "kind": "cpt", "readings": readings("c2", [(d, v * 1.05) for d, v in cpt])},
        {"id": "spt-1", "positionX": 6.0, "positionY": 8.0, "groundLevel": 0.0, "depth": 12.0, "kind": "spt", "readings": readings("s1", spt)},
        {"id": "bh-1", "positionX": 6.0, "positionY": -8.0, "groundLevel": 0.0, "depth": 20.0, "kind": "borehole", "readings": []},
    ]
    bases = [{"id": "cb-sand-phi", "layerId": "layer-sand", "parameter": "frictionAngle", "basis": "derived"}, {"id": "cb-clay-cu", "layerId": "layer-clay", "parameter": "undrainedStrength", "basis": "derived"}, {"id": "cb-gravel-phi", "layerId": "layer-gravel", "parameter": "frictionAngle", "basis": "declared"}]
    return {
        **{k: base[k] for k in ("structureId", "geotechnicalCategory", "designSituation", "designApproach", "annex", "groundwaterLevel", "investigationDepth")},
        "structureId": "geo-showcase", "layers": [sand, clay, gravel], "footings": copy.deepcopy(base["footings"]), "piles": [pile, driven], "retainingWalls": [wall, reinforced], "slopes": [slope, circle], "upliftCases": [base["upliftCases"][0], heave],
        "anchors": copy.deepcopy(base["anchors"]), "embeddedWalls": [base["embeddedWalls"][0], cantilever], "pileGroups": [base["pileGroups"][0], group2], "labTests": [base["labTests"][0], oedometer, triaxial], "layerWaters": waters,
        "placements": [{"id": f"place-{i}", "x": x, "y": 0.0, "z": 0.0, "yaw": 0.0} for i, x in enumerate((0.0, 12.0, 24.0))], "cases": copy.deepcopy(demo["cases"]), "investigations": investigations, "characteristicBases": bases,
    }


def failing(doc):
    doc = copy.deepcopy(doc)
    doc["footings"][0]["loadCases"][0]["verticalPermanent"] = 3000000.0
    doc["footings"][0]["loadCases"][0]["horizontalPermanent"] = 400000.0
    doc["piles"][0]["compressionPermanent"] = 3000000.0
    doc["retainingWalls"][0]["height"] = 7.0
    doc["retainingWalls"][0]["horizontalPermanent"] = 250000.0
    doc["slopes"][0]["angleDeg"] = 38.0
    doc["upliftCases"][0]["permanentStabilizing"] = 400000.0
    doc["upliftCases"][1]["porePressure"] = 110000.0
    doc["anchors"][0]["designForce"] = 900000.0
    doc["structureId"] = "geo-showcase-overloaded"
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
    template = (ART / "🖼️assets/🏗️compliant/🗣️.dsl.semio").read_text(encoding="utf-8")
    return emit(doc, load(SCHEMA), template, "semio norm.en1997.dsl v1", units=UNITS)


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
    drift = write_text("🖼️assets/🏘️ground-showcase/🏘️ground-showcase/🗣️.dsl.semio", dsl(good))
    for rel, doc in (("🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/ground-showcase-pass/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/ground-showcase-fail/🔣️.json", failing(good))):
        try:
            finish(doc, ART, rel, compact="🎪️showcase" in rel)
        except SystemExit as stop:
            drift = drift or bool(stop.code)
    sys.exit(1 if drift and "--check" in sys.argv else 0)
