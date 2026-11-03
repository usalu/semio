"""Generates the EN 1991 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`: one bridge-kind document whose rows populate every one of the 32 collections, including the hazard collections (helicopter, ties, construction loads, falling load, execution imperfection, derailment) and the v12 collections (footbridge, rail dynamics, load dispersal, bulk solid, rotating machine) (python3 -I en1991.py [--check])."""
import copy, glob, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, finish, load

ART = artifact("🏋️en1991")
BASE = load(sorted(glob.glob(str(ART / "🧪️tests/🧫️fixtures/🧬️mutations/*/✅apply/🔣️.after.json")))[0])
doc = copy.deepcopy(BASE)
doc.update(
    structureKind="bridge",
    fireMode="nominal",
    fireDuration=3600.0,
    placements=[{"id": "frame", "x": 0.0, "y": 0.0, "z": 0.0, "yaw": 0.0}],
    accidentalCases=[
        {"id": "impact-1", "impact": [{"vehicleMass": 15000.0, "vehicleSpeed": 13.9, "assumedForce": 500000.0}], "explosion": []},
        {"id": "explosion-1", "impact": [], "explosion": [{"explosionMass": 25.0, "standoff": 5.0, "assumedPressure": 50000.0}]},
    ],
    bridgeLanes=[{"id": "lane-1", "width": 3.0, "offset": -1.5, "lm1Lane": 1}, {"id": "lane-2", "width": 3.0, "offset": 1.5, "lm1Lane": 2}],
    constructionStages=[
        {"id": "short", "activity": "personnel", "durationDays": 30.0, "returnYears": 2.0, "assumedQk": 1500.0},
        {"id": "long", "activity": "personnel", "durationDays": 400.0, "returnYears": 5.0, "assumedQk": 1500.0},
    ],
    roofSurfaces=[{"id": "roof-west", "face": "west", "shape": "duopitch", "pitchDeg": 15.0, "extent": 10.0}],
    specialVehicles=[{"id": "sv-600", "designation": "SV600", "x": 10.0, "y": 0.0}],
    windBridges=[{"id": "wb-1", "deckWidth": 12.0, "deckDepth": 2.0, "parapetKind": 1, "parapetHeight": 1.1, "trafficKind": 0, "heightAboveGround": 20.0, "span": 40.0, "assumedForce": 100000.0}],
    dynamicElements=[{"id": "tower", "height": 80.0, "width": 4.0, "firstFrequency": 0.5, "logDecrement": 0.05, "massPerLength": 500.0, "strouhal": 0.18, "assumedCscd": 1.0}],
    impactActions=[
        {"id": "car", "kind": 0, "mass": 1500.0, "speed": 15.0, "stiffness": 0.0, "height": 1.0, "x": 0.0, "y": 0.0, "assumedForce": 600000.0},
        {"id": "ship", "kind": 1, "mass": 1.0e6, "speed": 3.0, "stiffness": 1.0e6, "height": 2.0, "x": 0.0, "y": 5.0, "assumedForce": 5000000.0},
    ],
    internalExplosions=[{"id": "room", "volume": 100.0, "ventArea": 2.0, "ventPressure": 5000.0, "assumedPressure": 20000.0}],
    thermalSections=[{"id": "deck", "deckType": 1, "surfacing": 2, "length": 60.0, "depth": 1.5, "alphaMicro": 12.0, "assumedRange": 40.0}],
    cases=[{"id": "case-1", "labelEn": "Persistent situation", "labelDe": "Ständige Situation", "entryId": "", "numbers": [], "texts": []}],
    helicopterLandings=[{"id": "heli-roof", "mass": 3000.0, "x": 5.0, "y": 5.0, "height": 20.0, "assumedForce": 200000.0}],
    tieRuns=[
        {"id": "tie-peripheral", "kind": 0, "spacing": 5.0, "span": 6.0, "deadLoad": 5000.0, "imposedLoad": 3000.0, "psi": 0.3, "x": 0.0, "y": 0.0, "providedForce": 80000.0},
        {"id": "tie-internal", "kind": 1, "spacing": 5.0, "span": 6.0, "deadLoad": 5000.0, "imposedLoad": 3000.0, "psi": 0.3, "x": 0.0, "y": 3.0, "providedForce": 80000.0},
    ],
    constructionLoads=[
        {"id": "q-ca", "stage": "short", "kind": 0, "magnitude": 1000.0},
        {"id": "q-cb", "stage": "short", "kind": 1, "magnitude": 200.0},
        {"id": "q-cc", "stage": "long", "kind": 0, "magnitude": 1000.0},
    ],
    fallingLoads=[{"id": "drop", "mass": 100.0, "dropHeight": 2.0, "stoppingDistance": 0.1, "x": 2.0, "y": 2.0, "assumedForce": 25000.0}],
    executionImperfections=[{"id": "imperfection-1", "height": 4.0, "tolerance": 0.02, "verticalLoad": 1.0e6, "x": 1.0, "y": 1.0, "assumedForce": 6000.0}],
    derailmentCases=[{"id": "derail-1", "distance": 3.0, "designForceX": 1000000.0, "designForceY": 300000.0, "x": 0.0, "y": -6.0, "assumedForce": 1200000.0}],
    craneHoists=[{"id": "hoist-1", "ratedLoad": 100000.0, "releaseFraction": 0.5, "releaseDevice": "grab"}],
    footbridges=[{"id": "fb-1", "loadedLength": 40.0, "deckWidth": 3.0, "naturalFrequency": 1.9}],
    railDynamics=[{"id": "rd-1", "determinantLength": 12.0, "maintenance": "careful"}],
    loadDispersals=[{"id": "ld-1", "wheelLength": 0.4, "wheelWidth": 0.4, "pavementThickness": 0.1, "slabThickness": 0.25}],
    bulkSolids=[{"id": "bs-1", "internalFriction": 0.5236}],
    rotatingMachines=[{"id": "rm-1", "mass": 500.0, "eccentricity": 0.002, "frequency": 25.0}],
    fatigueModels=[{"id": "flm-1", "model": "flm1", "lane": 1, "assumedAxleLoad": 210000.0, "assumedUdl": 2700.0}, {"id": "flm-3", "model": "flm3", "lane": 1, "assumedAxleLoad": 120000.0, "assumedUdl": 0.0}],
    supportCollisions=[{"id": "col-pier", "location": "deck", "assumedForceX": 1000000.0, "assumedForceY": 500000.0, "height": 1.25, "positionX": 0.3 * BASE["bridgeSpan"]}, {"id": "col-soffit", "location": "soffit", "assumedForceX": 500000.0, "assumedForceY": 0.0, "height": 1.25, "positionX": 0.6 * BASE["bridgeSpan"]}],
    localFires=[{"id": "fire-1", "fireDiameter": 2.0, "heatReleaseRate": 5.0e6, "ceilingHeight": 3.0, "horizontalDistance": 1.0, "assumedFlux": 60000.0}],
    snowGuards=[{"id": "guard-1", "roofId": "roof-main", "slopeLength": 6.0, "assumedForce": 8000.0}],
)
doc["roofs"].append({"id": "roof-continuous", "roofType": "duopitch", "pitchDeg": 10.0, "cE": 1.0, "cT": 1.0, "hasParapet": True, "parapetHeight": 0.6, "driftObstructionHeight": 0.0, "multiSpan": True, "assumedSk": 200.0})
finish(doc, ART, "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json")
