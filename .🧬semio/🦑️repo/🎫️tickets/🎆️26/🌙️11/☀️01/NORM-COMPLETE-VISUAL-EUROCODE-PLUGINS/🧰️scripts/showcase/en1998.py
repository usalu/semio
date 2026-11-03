"""Generates the EN 1998 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (python3 -I en1998.py).

The compliant multipart subject of the model (RC office, bridge, silo, tank, foundation, retaining wall, tower, assessment) extended by the opt-in rows that switch the remaining check families on: a second building (joint gap, steel members, given period), a masonry house (simple-masonry rule, wall slenderness), a timber hall (ductile connections), a building without modelled structure (period method refusal, system placeholder), the nonstructural and capacity-design data of the office, a soil profile with liquefaction data, a diaphragm, three synthetic accelerograms, bridge piers with bearings and a placement, a tie beam between two footings, a seismic slope, wall water, three assessments (KL1 to KL3) and a building irregular in elevation. The fixtures `🚦️check-outcome/seismic-showcase-{pass,fail}` are this document and a failing variant. Zone-0 exemption (`.zone0`) needs a_gR = 0 and so excludes every other building check: it is the one fragment a single document cannot reach."""
import copy, math, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load

ART = artifact("🫨️en1998")
BASE = load(ART / "🧪️tests/🧫️fixtures/🧬️mutations/✏️change-silo-height-m/✅apply/🔣️.before.json")
doc = copy.deepcopy(BASE)
for silo in doc["silos"]:
    silo.pop("shellStrengthMpa", None)
    silo.pop("shellThicknessM", None)
doc["annex"] = "De"
doc["site"].update(deGroundCombo="C-T", enGroundType="c", enSpectrumType="type1", importanceClass="ii", vS30MS=285.0)

office = doc["buildings"][0]
office.update(jointGapM=0.15, nonstructuralWeightN=30000.0, nonstructuralHeightM=14.0, nonstructuralPeriodS=0.2, nonstructuralQ=2.0, nonstructuralResistanceN=25000.0,
              columnMomentSumNm=1.8e6, beamMomentSumNm=1.2e6, tieDownMomentNm=2.0e6)


def scaled(storeys, factor, prefix=None):
    out = []
    for i, s in enumerate(storeys):
        s = copy.deepcopy(s)
        s["permanentGkN"] *= factor
        for v in s["variables"]:
            v["qkN"] *= factor
        if prefix:
            s["id"] = f"{prefix}{i + 1}"
            for v in s["variables"]:
                v["id"] = f"{s['id']}-q"
        out.append(s)
    return out


def building(bid, name, width, length, storeys, systems, members, **leaves):
    row = clone(office, id=bid, name=name, planWidthM=width, planLengthM=length, storeys=storeys, systems=systems, members=members, jointGapM=0.0, nonstructuralWeightN=0.0, nonstructuralHeightM=0.0, nonstructuralPeriodS=0.0, nonstructuralResistanceN=0.0,
                columnMomentSumNm=0.0, beamMomentSumNm=0.0, overturningBaseXM=0.0, overturningBaseYM=0.0, tieDownMomentNm=0.0)
    row.update(leaves)
    return row


def systems(material, system_type, ductility, q0, ratio, resistance):
    return [{"id": f"sys-{d}", "direction": d, "systemType": system_type, "material": material, "ductilityClass": ductility, "q0": q0, "alphaUOverAlpha1": ratio, "kW": 1.0, "baseShearResistanceN": resistance} for d in ("x", "y")]


def member(mid, material, role, **leaves):
    row = clone(office["members"][0], id=mid, material=material, role=role, minDimensionM=0.0, rho=0.0, rhoPrime=0.0, omegaWd=0.0, detailingCompatibleWithQ=True)
    row.update(leaves)
    return row


steel = building("bldg-steel-hall", "Steel braced hall", 30.0, 20.0, scaled(office["storeys"][:2], 0.6, "h"), systems("steel", "frame", "dcm", 3.5, 1.0, 1.0e6),
                 [member("col-st1", "steel", "column", steelSectionClass=1), member("beam-st1", "steel", "beam", steelSectionClass=1)], t1Method="given", t1GivenS=0.5, ct=0.085)
masonry = building("bldg-masonry-house", "Confined masonry house", 12.0, 10.0, scaled(office["storeys"][:3], 0.08, "m"), systems("masonry", "masonry", "dcl", 1.5, 1.0, 0.6e6),
                   [member("wall-m1", "masonry", "wall", minDimensionM=0.3, wallThicknessM=0.30, wallEffectiveHeightM=2.8)], t1Method="ct", ct=0.05, claimsSimpleMasonry=True, masonryWallAreaRatio=0.10, driftLimitClass="brittle")
timber = building("bldg-timber-hall", "Timber frame hall", 20.0, 12.0, scaled(office["storeys"][:1], 0.1, "t"), systems("timber", "frame", "dcm", 2.5, 1.0, 0.3e6),
                  [member("beam-t1", "timber", "beam", fastenerDiameterMm=12.0, timberThicknessMm=90.0)], t1Method="rayleigh", ct=0.05)
for s in timber["storeys"]:
    s.update(stiffnessX=2.0e7, stiffnessY=2.0e7)
pending_storeys = scaled(office["storeys"][:1], 0.2, "p")
for s in pending_storeys:
    s.update(stiffnessX=0.0, stiffnessY=0.0, driftXM=0.0, driftYM=0.0, shearResistanceN=0.0)
pending = building("bldg-pending", "Annex building (structure not yet modelled)", 10.0, 8.0, pending_storeys, [], [], t1Method="rayleigh", ct=0.05)
setback_storeys = scaled(office["storeys"][:4], 0.5, "r")
for i, s in enumerate(setback_storeys):
    shrink = 1.0 if i < 2 else 0.7
    s.update(permanentGkN=s["permanentGkN"] * shrink, stiffnessX=s["stiffnessX"] * 0.5 * shrink, stiffnessY=s["stiffnessY"] * 0.5 * shrink, centreOfMassXM=9.0, centreOfMassYM=7.0, centreOfStiffnessXM=9.0, centreOfStiffnessYM=7.0, shearResistanceN=300000.0)
    for v in s["variables"]:
        v["qkN"] *= shrink
setback_members = [member("col-r1", "rc", "column", minDimensionM=0.35, rho=0.012, omegaWd=0.12), member("beam-r1", "rc", "beam", minDimensionM=0.3, rho=0.01, rhoPrime=0.005, omegaWd=0.08)]
setback = building("bldg-setback", "Setback RC block (irregular in elevation)", 18.0, 14.0, setback_storeys, systems("rc", "frame", "dcm", 3.0, 1.2, 0.9e6), setback_members, t1Method="ct", ct=0.075, elevationRegular=False, setbackRatio=0.15,
                   storeyStiffnessBasis="declared", storeyStiffnessRatio=0.5)
doc["buildings"] += [steel, masonry, timber, pending, setback]

doc["assessments"].append({**doc["assessments"][0], "id": "as-2", "knowledgeLevel": "kl3", "limitState": "nc", "chordRotationDemandRad": 0.02, "chordRotationYieldRad": 0.005, "chordRotationUltimateRad": 0.035, "sectionDepthM": 0.5, "sectionWidthM": 0.3, "effectiveDepthM": 0.46,
                          "shearSpanM": 1.75, "axialForceN": 300000.0, "tensionSteelAreaM2": 0.0012, "compressionSteelAreaM2": 0.0008, "barDiameterMm": 20.0, "steelYieldMpa": 500.0, "concreteStrengthMpa": 30.0, "stirrupLegAreaM2": 1.0e-4, "stirrupSpacingM": 0.1,
                          "stirrupYieldMpa": 500.0, "coreWidthM": 0.22, "coreDepthM": 0.42, "restrainedBarCount": 6.0, "restrainedBarSpacingM": 0.15, "shearForceDemandN": 1.0e5})
doc["assessments"].append({**doc["assessments"][1], "id": "as-3", "supportedBuildingId": "bldg-setback", "knowledgeLevel": "kl1", "limitState": "dl", "chordRotationDemandRad": 0.004, "shearForceDemandN": 8.0e4})
doc["tanks"][0].update(shellStrengthMpa=235.0, shellThicknessM=0.012)
doc["towers"][0].update(eiNm2=5.7e10, baseRotationalStiffnessNm=3.0e11)

foundation = doc["foundations"][0]
foundation.update(liqN160=12.0, liqFinesPercent=10.0, liqDepthM=6.0, liqWaterDepthM=2.0, liqUnitWeightNM3=18000.0, liqMagnitude=6.5, liqLayerThicknessM=3.0, footingWidthM=14.0, bearingAxialN=20.0e6, bearingShearN=0.8e6, bearingMomentNm=6.0e6, bearingNMaxN=40.0e6,
                  soilFrictionAngleDeg=32.0, pileDiameterM=0.6, pileLengthM=18.0, pileModulusPa=30.0e9, pileSubgradeModulusNM3=20.0e6, pileHeadShearN=150000.0, pileHeadFixed=1.0, pileInterfaceDepthM=8.0, pileUpperVsMS=150.0, pileLowerVsMS=300.0, pileMomentRdNm=400000.0)
foundation.update(baseFrictionAngleDeg=25.0, basePorePressurePa=10000.0)
doc["foundations"].append(clone(foundation, id="fd-2", supportedBuildingId="bldg-steel-hall", areaM2=60.0, pRdPa=300000.0, passiveResistanceN=150000.0, kFoundation=300000.0, kSoil=100000.0, footingWidthM=8.0, bearingAxialN=3.0e6, bearingShearN=0.15e6,
                          bearingMomentNm=0.8e6, bearingNMaxN=8.0e6, pileDiameterM=0.0, pileLengthM=0.0, pileModulusPa=0.0, pileSubgradeModulusNM3=0.0, pileHeadShearN=0.0, pileHeadFixed=0.0, pileInterfaceDepthM=0.0, pileUpperVsMS=0.0, pileLowerVsMS=0.0, pileMomentRdNm=0.0))
doc["soilLayers"] = [{"id": lid, "foundationId": foundation["id"], "thicknessM": t, "unitWeightNM3": g, "shearWaveVelocityMS": vs, "n160": n, "finesPercent": fines} for lid, t, g, vs, n, fines in
                     [("l1", 3.0, 18000.0, 180.0, 18.0, 10.0), ("l2", 4.0, 17500.0, 150.0, 8.0, 20.0), ("l3", 5.0, 19000.0, 260.0, 32.0, 8.0), ("l4", 18.0, 20000.0, 420.0, 50.0, 5.0)]]
doc["soilLayers"] += [{"id": lid, "foundationId": "fd-2", "thicknessM": th, "unitWeightNM3": g, "shearWaveVelocityMS": vs, "n160": n, "finesPercent": fines} for lid, th, g, vs, n, fines in [("l5", 4.0, 18000.0, 170.0, 16.0, 12.0), ("l6", 20.0, 20000.0, 400.0, 45.0, 5.0)]]
doc["tieBeams"] = [{"id": "tb-1", "foundationAId": "fd-1", "foundationBId": "fd-2", "span": 12.0, "width": 0.4, "depth": 0.6, "steelArea": 0.0012, "steelDesignStrength": 434.8e6, "relativeDisplacement": 0.01, "displacementLimit": 0.03}]
doc["slopes"] = [{"id": "sl-1", "governingLayerId": "l1", "height": 8.0, "angle": 30.0, "frictionAngle": 32.0, "cohesion": 5000.0, "topographicAmplification": 1.2, "circleCentreX": 4.0, "circleCentreZ": 12.0, "circleRadius": 13.0}]
doc["foundations"][0].update(pileAxialLoadN=0.0)
doc["foundations"][1].update(pileAxialLoadN=0.0)
doc["foundations"].append(clone(doc["foundations"][1], id="fd-3", pileDiameterM=0.8, pileLengthM=16.0, pileModulusPa=30.0e9, pileSubgradeModulusNM3=25.0e6, pileHeadShearN=100000.0, pileHeadFixed=1.0, pileInterfaceDepthM=0.0, pileUpperVsMS=0.0, pileLowerVsMS=0.0, pileMomentRdNm=900000.0, pileAxialLoadN=1.0e6))
doc["soilLayers"] += [{"id": lid, "foundationId": "fd-3", "thicknessM": th, "unitWeightNM3": g, "shearWaveVelocityMS": vs, "n160": n, "finesPercent": fines} for lid, th, g, vs, n, fines in [("l7", 4.0, 17500.0, 140.0, 2.0, 15.0), ("l8", 20.0, 20000.0, 380.0, 45.0, 5.0)]]
doc["slopes"].append({**doc["slopes"][0], "id": "sl-2", "governingLayerId": "l7", "height": 1.0, "angle": 2.0, "circleCentreX": 0.0, "circleCentreZ": 0.0, "circleRadius": 0.0})
doc["bridges"][0]["fundamentalPeriodS"] = 0.25
doc["isolationSystems"] = [{"id": "iso-1", "bridgeId": "br-1", "buildingId": "", "strengthN": 3.0e5, "stiffnessNPerM": 1.5e7, "yieldDisplacementM": 0.01, "capacityM": 0.5}]
doc["pierBases"] = [{"id": "pb-1", "bridgeId": "br-1", "pierId": "pier-1", "radiusM": 2.5, "soilVsMS": 250.0, "soilUnitWeightNM3": 19000.0, "poissonRatio": 0.35}]
doc["retainingWalls"][0].update(waterHeightM=1.5, saturatedGamma=20000.0, permeabilityMS=1.0e-6)
doc["diaphragms"] = [{"id": "dia-1", "buildingId": "bldg-office", "storeyId": "s2", "material": "concrete", "thicknessM": 0.2, "spanM": 24.0, "depthM": 16.0, "openingWidthM": 2.4, "shearStrengthMpa": 1.0, "chordAreaM2": 0.01, "chordStrengthMpa": 435.0, "gammaD": 1.3}]


def record(seed):
    freqs, n, dt = [0.8 + 0.37 * seed, 1.9 + 0.21 * seed, 3.6 + 0.43 * seed, 6.4 - 0.29 * seed], 1000, 0.02
    samples = []
    for k in range(n):
        t = k * dt
        envelope = (t / 3.0) * math.exp(1.0 - t / 3.0)
        samples.append(envelope * sum(math.sin(2 * math.pi * f * t + 1.3 * j + seed) / (1 + j) for j, f in enumerate(freqs)))
    peak = max(abs(x) for x in samples)
    return " ".join(f"{1.5 * x / peak:.4f}" for x in samples)


doc["accelerograms"] = [{"id": f"rec-{i}", "buildingId": "bldg-office", "label": f"Synthetic ground motion {i + 1}", "samples": record(i), "dtS": 0.02, "scaleFactor": 1.0} for i in range(3)]
pier = lambda i, **leaves: {"id": f"pier-{i}", "bridgeId": "br-1", "xM": 20.0 * i, "heightM": 8.0, "sectionBM": 2.0, "sectionHM": 1.5, "modulusMpa": 33000.0, "momentRdNm": 18.0e6, "shearRdN": 2.4e6, "reinforcementRatio": 0.01, "coverM": 0.08, "concreteFckMpa": 30.0, "steelFykMpa": 500.0,
                            "axialLoadN": 7.0e6, "stiffnessBasis": "declared", **leaves}
doc["piers"] = [pier(1), pier(2, reinforcementRatio=0.008, axialLoadN=8.0e6, stiffnessBasis="computed")]
bearing = lambda bid, pier, x, k: {"id": bid, "bridgeId": "br-1", "pierId": pier, "device": "elastomeric", "xM": x, "stiffnessNPerM": k, "dRdM": 0.20, "seatLengthM": 0.6, "minSupportLengthM": 0.4}
doc["bearings"] = [bearing("brg-a", "", 0.0, 20.0e6), bearing("brg-1", "pier-1", 0.0, 100.0e6), bearing("brg-2", "pier-2", 0.0, 100.0e6), bearing("brg-b", "", 60.0, 20.0e6)]
doc["joints"] = [{"id": f"joint-{i + 1}", "buildingId": "bldg-office", "storeyId": s, "columnId": "col-c1", "beamId": "beam-b1", "xM": x, "yM": 4.0, "columnMomentRdNm": mc, "beamMomentRdNm": mb} for i, (s, x, mc, mb) in enumerate([("s1", 6.0, 9.0e6, 4.0e6), ("s2", 12.0, 7.5e6, 4.0e6), ("s3", 18.0, 6.0e6, 3.5e6)])]
doc["bridges"][0].update(deckLengthM=60.0, deckEiNm2=5.0e11, importanceClass=2.0, verticalPeriodS=0.3, pierCrackedInertiaRatio=0.5)
for b in doc["buildings"]:
    for s in b["storeys"]:
        for v in s["variables"]:
            v["category"] = "b"
doc["bridges"][0]["variables"][0]["category"] = "f"
doc["silos"][0]["contentCategory"] = "e"
doc["tanks"][0]["contentCategory"] = "e"
doc["towers"][0]["variables"][0]["category"] = "e"
doc["placements"] = [{"id": "bldg-office", "x": 0.0, "y": 0.0, "z": 0.0, "yaw": 0.0}]


def failing(good):
    bad = copy.deepcopy(good)
    for s in bad["buildings"][0]["systems"]:
        s["baseShearResistanceN"] = 1.0e5
    bad["buildings"][-1]["setbackRatio"] = 0.35
    bad["tieBeams"][0].update(steelArea=0.0004, relativeDisplacement=0.04)
    bad["slopes"][0].update(frictionAngle=24.0, cohesion=0.0)
    bad["retainingWalls"][0].update(hRdNPerM=25000.0, waterHeightM=4.0)
    bad["foundations"][0].update(baseFrictionAngleDeg=5.0, basePorePressurePa=60000.0, passiveResistanceN=10000.0)
    return bad


drift = False
for rel, out in (("🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json", doc), ("🧪️tests/🧫️fixtures/🚦️check-outcome/seismic-showcase-pass/🔣️.json", doc), ("🧪️tests/🧫️fixtures/🚦️check-outcome/seismic-showcase-fail/🔣️.json", failing(doc))):
    try:
        finish(out, ART, rel, compact="🎪️showcase" in rel)
    except SystemExit as stop:
        drift = drift or bool(stop.code)
sys.exit(1 if drift else 0)
