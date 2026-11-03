"""🧫 Writes the EN 1991 check-outcome fixtures `families-pass` and `families-fail` (showcase with covered or under-assumed bridge traffic, crane, silo, bridge wind, impact, gas explosion, blast and bridge temperature, plus roof surfaces with a step and a parapet drift)."""
import copy, json, pathlib
base = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures")
show = json.load(open(base / "🎪️showcase/🔣️.json", encoding="utf8"))
show.update(bridgeLaneWidth=7.0, bridgeCurveRadius=400.0, northGermanLowlandSnow=True)
show["specialVehicles"] = [{"id": "sv-600", "designation": "600/150", "x": 10.0, "y": 3.5}]
show["windFaces"].append({"id": "roof-main-f", "zone": "F", "z": 12.0, "cPe10": -1.0, "cPe1": -1.5, "cPi": 0.2, "cS": 1.0, "cD": 1.0, "loadedArea": 10.0, "assumedWp": 5000.0})
show["roofs"][0].update(hasParapet=True, parapetHeight=1.0)
show["roofSurfaces"] = [{"id": "rs-1", "face": "roof-main-f", "shape": "duopitch", "pitchDeg": 30.0, "extent": 8.0}, {"id": "rs-2", "face": "roof-main-f", "shape": "flat", "pitchDeg": 0.0, "extent": 10.0}]
passing = copy.deepcopy(show)
passing.update(assumedBridgeTandem=400000.0, assumedBridgeUdl=12000.0, assumedBridgeLm2=600000.0, assumedBridgeFootway=8000.0, assumedBridgeLm3=400000.0, assumedBridgeLm4=8000.0)
passing.update(assumedCraneWheel=150000.0, assumedCraneHorizontal=40000.0, assumedSiloPressure=35000.0, assumedSiloPatch=8000.0, assumedSiloWallFriction=12000.0)
passing["windBridges"][0]["assumedForce"] = 5000.0
passing["internalExplosions"][0]["assumedPressure"] = 150000.0
passing["accidentalCases"][1]["explosion"][0]["assumedPressure"] = 500000.0
passing["thermalSections"][0]["assumedRange"] = 120.0
failing = copy.deepcopy(show)
failing.update(assumedBridgeTandem=200000.0, assumedBridgeUdl=6000.0, assumedBridgeLm2=300000.0, assumedBridgeFootway=3000.0, assumedBridgeLm3=100000.0, assumedBridgeLm4=3000.0)
failing.update(assumedCraneWheel=100000.0, assumedCraneHorizontal=20000.0, assumedSiloPressure=20000.0, assumedSiloPatch=3000.0, assumedSiloWallFriction=5000.0)
failing["bridgeLanes"][0]["width"] = 3.6
failing["windBridges"][0]["assumedForce"] = 2000.0
failing["impactActions"][0]["assumedForce"] = 250000.0
failing["internalExplosions"][0]["assumedPressure"] = 60000.0
failing["thermalSections"][0]["assumedRange"] = 60.0
failing["fireMode"] = "none"
failing["specialVehicles"][0]["x"] = 1.0
failing["specialVehicles"][0]["y"] = 0.5
for name, doc in (("families-pass", passing), ("families-fail", failing)):
    out = base / "🚦️check-outcome" / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(doc, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
