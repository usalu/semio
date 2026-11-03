"""Generates the EN 1990 road-bridge showcase `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, its check-outcome fixtures `🚦️check-outcome/road-bridge-showcase-{pass,fail}` and the DSL asset `🖼️assets/🏢️road-bridge-showcase` (python3 -I en1990.py [--write|--check]).
A two-span continuous road bridge: permanent actions per span (influence of the adjacent span from the two-span elastic line), LM1 groups gr1a, gr1b and gr2 as trafficGroups rows, wind and thermal, bridge deflection and vibration rows, a fatigue spectrum, test series, a FORM analysis and a seismic action. Every psi, gamma and K_FI comes from the registry through the annex; the numbers below are documented inputs, not normative values."""
import copy, json, pathlib, re, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, coerce, finish, load

ART = artifact("⚖️en1990")
SNAPSHOT = "🧬️schema/📸️snapshot/🔣️.json"
SPAN = 30.0
ADJACENT = -1.0 / 3.0


def member(mid, en, de):
    return {
        "id": mid, "labelEn": en, "labelDe": de, "rdStr": 1_400_000.0, "rdGeo": 1_400_000.0, "rdEquStab": 1_000_000.0, "rdEquDestab": 800_000.0, "rdFat": 500_000.0,
        "span": SPAN, "precamber": 0.0, "deflectionW1": 0.016, "deflectionW2": 0.008, "deflectionW3": 0.024, "deflectionLimitRatio": 500.0, "deflectionLimitRatioW3": 300.0,
        "slsLimitChar": 1_400_000.0, "slsLimitFreq": 1_400_000.0, "slsLimitQp": 1_400_000.0, "vibrationFrequency": 4.2, "vibrationFrequencyMin": 3.0,
        "frameHeight": 0.0, "frameDrift": 0.0, "storeyHeight": 0.0, "storeyDrift": 0.0,
        "materialStrengthK": 355_000_000.0, "materialConversionFactor": 1.0, "materialPartialFactor": 1.0, "materialDesignStrength": 300_000_000.0,
        "nominalDimension": 0.0, "dimensionTolerance": 0.0, "designDimension": 0.0, "memberAlphaCr": 0.0, "secondOrderAnalysis": 0.0,
    }


def sls(sid, mid):
    return {
        "id": sid, "memberId": mid, "deckAcceleration": 0.0, "deckAccelerationLimit": 0.0, "deckTwist": 0.0, "deckTwistLimit": 0.0,
        "bridgeDeflection": 0.03, "bridgeDeflectionLimit": SPAN / 800.0, "trackGauge": 0.0, "lineSpeed": 0.0, "ballastDepth": 0.0, "deckMass": 0.0,
        "naturalFrequency": 4.2, "endRotation": 0.0, "basicWindSpeed": 0.0, "dampingRatio": 0.0,
    }


def build():
    permanents = [("G-deck-s1", "gSup", 100_000.0), ("G-deck-s2", "gSup", 100_000.0), ("G-surf-s1", "gInf", 30_000.0), ("G-surf-s2", "gInf", 30_000.0)]
    variables = [
        ("Q-ts", "road_traffic", 300_000.0, "gr1a"), ("Q-udl", "road_udl", 90_000.0, "gr1a"), ("Q-ped", "road_pedestrian", 25_000.0, "gr1a"),
        ("Q-sa", "road_single_axle", 400_000.0, "gr1b"), ("Q-brake", "road_horizontal", 120_000.0, "gr2"),
        ("Q-wind", "wind_bridge", 50_000.0, ""), ("Q-temp", "temperature_bridge", 40_000.0, ""),
    ]
    spans = ("span-1", "span-2")
    effects = []
    for m, own in zip(spans, ("s1", "s2")):
        other = "s2" if own == "s1" else "s1"
        effects += [{"memberId": m, "actionId": f"G-deck-{own}", "influence": 1.0}, {"memberId": m, "actionId": f"G-deck-{other}", "influence": ADJACENT},
                    {"memberId": m, "actionId": f"G-surf-{own}", "influence": 1.0}, {"memberId": m, "actionId": f"G-surf-{other}", "influence": ADJACENT}]
        effects += [{"memberId": m, "actionId": q[0], "influence": 1.0} for q in variables]
        effects += [{"memberId": m, "actionId": "A-impact", "influence": 1.0}, {"memberId": m, "actionId": "E-1", "influence": 1.0}]
    demo = load(ART / "🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json")
    return {
        "annex": "De", "projectId": "road-bridge-showcase", "structureKind": "roadBridge", "altitudeM": 200.0, "consequenceClass": 2, "reliabilityClass": "RC2",
        "designWorkingLifeCategory": 5, "designWorkingLifeYears": 100.0, "referencePeriodYears": 100.0, "supervisionLevel": "dsl3", "inspectionLevel": "il3", "betaComputed": 4.7,
        "permanents": [{"id": i, "kind": k, "gk": g} for i, k, g in permanents],
        "variables": [{"id": i, "category": c, "qk": q, "group": g} for i, c, q, g in variables],
        "accidentals": [{"id": "A-impact", "ad": 50_000.0}],
        "seismics": [{"id": "E-1", "aEk": 60_000.0, "importanceClass": "III"}],
        "members": [member("span-1", "Span 1 of the continuous deck", "Feld 1 des Durchlauftraegers"), member("span-2", "Span 2 of the continuous deck", "Feld 2 des Durchlauftraegers")],
        "bridgeSls": [sls("sls-s1", "span-1"), sls("sls-s2", "span-2")],
        "effects": effects,
        "fatigueSpectra": [
            {"id": "FS-1", "memberId": "span-1", "detailCategory": 71_000_000.0, "stressRange": 60_000_000.0, "cycles": 200_000.0},
            {"id": "FS-2", "memberId": "span-1", "detailCategory": 71_000_000.0, "stressRange": 30_000_000.0, "cycles": 1_000_000.0},
            {"id": "FS-3", "memberId": "span-2", "detailCategory": 71_000_000.0, "stressRange": 50_000_000.0, "cycles": 300_000.0},
            {"id": "FS-4", "memberId": "span-2", "detailCategory": 71_000_000.0, "stressRange": 25_000_000.0, "cycles": 1_500_000.0},
        ],
        "reliabilityAnalyses": [{"id": "rel-1", "resistanceDistribution": "normal", "resistanceMean": 330_000.0, "resistanceSd": 30_000.0, "actionDistribution": "normal", "actionMean": 150_000.0, "actionSd": 30_000.0}],
        "testSeries": [
            {"id": "tes-1", "quantity": "strength", "distribution": "normal", "sampleSize": 10, "mean": 40.0, "stdDev": 4.0, "variationKnown": 0.0, "conversionFactor": 0.9, "required": 25.0},
            {"id": "tes-2", "quantity": "bearing-stiffness", "distribution": "lognormal", "sampleSize": 6, "mean": 12.0, "stdDev": 1.5, "variationKnown": 0.0, "conversionFactor": 0.9, "required": 7.0},
        ],
        "placements": [{"id": "abutment-A", "x": 0.0, "y": 0.0, "z": 0.0, "yaw": 0.0}, {"id": "pier-P", "x": SPAN, "y": 0.0, "z": 0.0, "yaw": 0.0}, {"id": "abutment-B", "x": 2 * SPAN, "y": 0.0, "z": 0.0, "yaw": 0.0}],
        "cases": [],
        "factorOverrides": [],
        "trafficGroups": [
            {"id": "TG-ts", "group": "GR1A", "variableId": "Q-ts"}, {"id": "TG-udl", "group": "GR1A", "variableId": "Q-udl"}, {"id": "TG-ped", "group": "GR1A", "variableId": "Q-ped"},
            {"id": "TG-sa", "group": "GR1B", "variableId": "Q-sa"}, {"id": "TG-brake", "group": "GR2", "variableId": "Q-brake"},
        ],
    }


def failing(doc):
    doc = copy.deepcopy(doc)
    doc["projectId"] = "road-bridge-showcase-failing"
    next(t for t in doc["trafficGroups"] if t["id"] == "TG-sa")["group"] = "GR1A"
    for m in doc["members"]:
        m["rdStr"], m["rdGeo"], m["rdFat"], m["deflectionW1"] = 600_000.0, 600_000.0, 100_000.0, 0.04
    doc["bridgeSls"][0]["bridgeDeflection"] = 0.06
    doc["fatigueSpectra"][0]["cycles"] = 6_000_000.0
    doc["testSeries"][0]["required"] = 40.0
    doc["betaComputed"] = 3.0
    return doc


def kebab(key):
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "-", key).lower()


def cell(value):
    if isinstance(value, float):
        return str(int(value)) if value == int(value) else repr(value)
    if isinstance(value, int):
        return str(value)
    text = value
    return text if re.fullmatch(r"[A-Za-z0-9_.\-]+", text) else '"' + text.replace('"', '\\"') + '"'


def dsl(doc):
    template = next(ART.glob("🖼️assets/🏢️road-bridge-compliant/*/🗣️.dsl.semio")).read_text(encoding="utf-8").split("\n")
    heads = {ln.split(" ", 1)[0]: ln for ln in template if re.match(r"^[a-z\-]+ \[.*\] \{$", ln)}
    scalars = [k for k, v in doc.items() if not isinstance(v, list)]
    def scalar(k):
        v = doc[k]
        return f"{kebab(k)}={cell(kebab(v) if k in ('structureKind', 'reliabilityClass', 'annex') else v.lower() if k in ('supervisionLevel', 'inspectionLevel') else v)}"
    out = ["semio norm.en1990.dsl v1", " ".join(scalar(k) for k in scalars)]
    for k, rows in ((k, v) for k, v in doc.items() if isinstance(v, list)):
        name = kebab(k)
        out.append(heads[name])
        for r in rows:
            vals = [v.lower() if f in ("importanceClass", "group", "kind") and isinstance(v, str) else v for f, v in r.items()]
            out.append("  " + " ".join(cell(v) for v in vals))
        out.append("}")
    return "\n".join(out) + "\n"


def write_text(rel, text):
    path = ART / rel
    changed = not path.exists() or path.read_text(encoding="utf-8") != text
    if "--check" in sys.argv:
        print(("would change " if changed else "unchanged ") + rel)
        return changed
    if changed:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    print(("wrote " if changed else "unchanged ") + rel)
    return False


def snapshot_validate(doc):
    import jsonschema
    schema = load(ART / SNAPSHOT)
    errors = [e.message for e in jsonschema.Draft7Validator(schema).iter_errors(coerce(doc, schema, schema))]
    if errors:
        print("\n".join(errors[:40]))
        sys.exit(1)


if __name__ == "__main__":
    good = build()
    snapshot_validate(good)
    snapshot_validate(failing(good))
    drift = write_text("🖼️assets/🏢️road-bridge-showcase/🏢️road-bridge-showcase/🗣️.dsl.semio", dsl(good))
    for rel, doc in (("🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/road-bridge-showcase-pass/🔣️.json", good), ("🧪️tests/🧫️fixtures/🚦️check-outcome/road-bridge-showcase-fail/🔣️.json", failing(good))):
        try:
            finish(doc, ART, rel, expect_empty=("cases", "factorOverrides"), schema_rel=SNAPSHOT)
        except SystemExit as stop:
            drift = drift or bool(stop.code)
    sys.exit(1 if drift and "--check" in sys.argv else 0)
