"""🧬 Hand-built EN 1998 check-outcome documents (pass and fail) from one committed mutation snapshot. Run: python -I doc-oracles-v2-en1998-fixtures.py"""
import copy
import json
import pathlib

ROOT = next(p for p in pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*en1998"))
SUBSET = ROOT / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures"
base = json.load(open(SUBSET / "🧬️mutations/↪️change-tower-m-rd-nm/✅apply/🔣️.before.json", encoding="utf8"))


def rich():
    doc = copy.deepcopy(base)
    doc["site"]["importanceClass"] = "III"
    first = doc["buildings"][0]
    second = copy.deepcopy(first)
    second["id"], second["name"] = "bldg-annex", "Annex"
    for system in second["systems"]:
        system["baseShearResistanceN"] = 900000.0
    first.update({"t1Method": "rayleigh", "jointGapM": 0.30, "setbackRatio": 0.1, "planEnvelopeGapRatio": 0.02, "nonstructuralWeightN": 20000.0, "nonstructuralHeightM": 14.0, "nonstructuralPeriodS": 0.3, "nonstructuralQ": 2.0, "nonstructuralResistanceN": 30000.0, "beamMomentSumNm": 500000.0, "columnMomentSumNm": 900000.0, "overturningBaseXM": 16.0, "overturningBaseYM": 24.0, "tieDownMomentNm": 2000000.0})
    for storey in first["storeys"]:
        storey["shearResistanceN"] = 1000000.0
        storey["centreOfMassXM"] += 1.0
    doc["buildings"] = [first, second]
    return doc


def failing():
    doc = rich()
    first = doc["buildings"][0]
    first["systems"][0]["baseShearResistanceN"] = 100000.0
    first["jointGapM"] = 0.05
    first["beamMomentSumNm"] = 900000.0
    first["columnMomentSumNm"] = 800000.0
    first["nonstructuralResistanceN"] = 10000.0
    first["tieDownMomentNm"] = 0.0
    first["overturningBaseYM"] = 1.0
    first["storeys"][2]["stiffnessX"] = 40000000.0
    first["storeys"][1]["driftXM"] = 0.04
    first["storeys"][1]["shearResistanceN"] = 150000.0
    first["storeys"][3]["centreOfStiffnessXM"] = 20.0
    first["storeys"][3]["centreOfMassYM"] = 14.0
    first["setbackRatio"] = 0.3
    first["members"][0]["rho"] = 0.06
    first["members"].append({"id": "steel-c2", "material": "steel", "role": "column", "detailingCompatibleWithQ": True, "minDimensionM": 0.3, "rho": 0.0, "rhoPrime": 0.0, "omegaWd": 0.0, "steelSectionClass": 4, "fastenerDiameterMm": 0.0, "timberThicknessMm": 0.0, "wallThicknessM": 0.0, "wallEffectiveHeightM": 0.0})
    return doc


for name, doc in (("seismic-rich-pass", rich()), ("seismic-rich-fail", failing())):
    target = SUBSET / "🚦️check-outcome" / name
    target.mkdir(parents=True, exist_ok=True)
    (target / "🔣️.json").write_text(json.dumps(doc, indent=1, ensure_ascii=False) + "\n", encoding="utf8")
    print(target)
