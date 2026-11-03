"""📌 Writes the committed check-outcome documents of the EN 1998-5 tie beams and slopes from `seismic-shear-pass`: a second foundation under the annex, a soil layer, one tie beam between the two foundations and one slope; `fail` weakens the tie steel, the displacement limit and the soil strength."""
import copy, json, pathlib

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
base = json.load(open(ROOT / "seismic-shear-pass" / "🔣️.json", encoding="utf8"))


def document(fail):
    d = copy.deepcopy(base)
    second = copy.deepcopy(d["foundations"][0])
    second["id"], second["supportedBuildingId"] = "fd-2", "bldg-annex"
    d["foundations"].append(second)
    d["soilLayers"] = [{"id": "l1", "foundationId": "fd-1", "thicknessM": 3.0, "unitWeightNM3": 19000.0, "shearWaveVelocityMS": 180.0, "n160": 18.0, "finesPercent": 10.0}]
    d["tieBeams"] = [{"id": "tb-1", "foundationAId": "fd-1", "foundationBId": "fd-2", "span": 6.0, "width": 0.4, "depth": 0.6, "steelArea": 0.0004 if fail else 0.0012, "steelDesignStrength": 434800000.0, "relativeDisplacement": 0.04 if fail else 0.01, "displacementLimit": 0.03}]
    d["slopes"] = [{"id": "sl-1", "governingLayerId": "l1", "height": 8.0, "angle": 30.0, "frictionAngle": 24.0 if fail else 32.0, "cohesion": 0.0 if fail else 5000.0, "topographicAmplification": 1.2, "circleCentreX": 4.0, "circleCentreZ": 12.0, "circleRadius": 13.0}]
    return d


for name, fail in (("seismic-tie-slope-pass", False), ("seismic-tie-slope-fail", True)):
    out = ROOT / name
    out.mkdir(exist_ok=True)
    (out / "🔣️.json").write_text(json.dumps(document(fail), indent=2, ensure_ascii=False) + "\n", encoding="utf8")
