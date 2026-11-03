"""Generates the EN 1990 check-outcome fixture documents from a committed mutation snapshot: python -I gen-en1990.py"""
import copy, json, pathlib

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any")
BASE = ROOT / "🧪️tests/🧫️fixtures/🧬️mutations/💥️change-accidental-ad/✅apply/🔣️.before.json"
OUT = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
base = json.load(open(BASE, encoding="utf8"))


def common(doc):
    doc["variables"].append({"id": "Q-exec", "category": "construction_load", "qk": 15000.0, "group": ""})
    doc["effects"].append({"memberId": "beam-B1", "actionId": "Q-exec", "influence": 1.0})
    doc["effects"].append({"memberId": "beam-B1", "actionId": "G-sup", "influence": 1.0}) if False else None
    m = doc["members"][0]
    m.update(rdStr=400000.0, rdGeo=400000.0, rdEquStab=200000.0, rdEquDestab=180000.0, rdFat=60000.0, span=6.0, precamber=0.004,
             deflectionW1=0.006, deflectionW2=0.003, deflectionW3=0.009, deflectionLimitRatio=250.0, deflectionLimitRatioW3=300.0,
             slsLimitChar=250000.0, slsLimitFreq=250000.0, slsLimitQp=250000.0, vibrationFrequency=5.5, vibrationFrequencyMin=3.0,
             materialStrengthK=30e6, materialConversionFactor=0.8, materialPartialFactor=1.5, materialDesignStrength=14e6,
             nominalDimension=0.5, dimensionTolerance=0.01, designDimension=0.52, memberAlphaCr=12.0, secondOrderAnalysis=0.0)
    doc["fatigueSpectra"] = [{"id": "fat-1", "memberId": "beam-B1", "detailCategory": 71e6, "stressRange": 60e6, "cycles": 1.0e6}, {"id": "fat-2", "memberId": "beam-B1", "detailCategory": 71e6, "stressRange": 40e6, "cycles": 2.0e6}]
    doc["reliabilityAnalyses"] = [{"id": "rel-1", "resistanceDistribution": "normal", "resistanceMean": 330000.0, "resistanceSd": 30000.0, "actionDistribution": "normal", "actionMean": 150000.0, "actionSd": 30000.0}]
    doc["testSeries"] = [{"id": "tes-1", "quantity": "strength", "distribution": "normal", "sampleSize": 10, "mean": 40.0, "stdDev": 4.0, "variationKnown": 0.0, "conversionFactor": 0.9, "required": 25.0}]
    doc["betaComputed"] = 4.3
    doc["designWorkingLifeCategory"] = 4
    doc["designWorkingLifeYears"] = 50.0
    doc["supervisionLevel"] = "DSL3"
    doc["inspectionLevel"] = "IL3"
    return doc


good = common(copy.deepcopy(base))
bad = common(copy.deepcopy(base))
m = bad["members"][0]
m.update(rdStr=90000.0, rdGeo=90000.0, rdEquStab=5000.0, rdEquDestab=30000.0, slsLimitChar=30000.0, slsLimitFreq=25000.0, slsLimitQp=20000.0,
         deflectionW3=0.03, vibrationFrequency=2.0, materialDesignStrength=30e6, designDimension=0.505, memberAlphaCr=6.0, secondOrderAnalysis=0.0)
bad["fatigueSpectra"] = [{"id": "fat-1", "memberId": "beam-B1", "detailCategory": 36e6, "stressRange": 90e6, "cycles": 5.0e6}]
bad["reliabilityAnalyses"][0]["resistanceMean"] = 220000.0
bad["testSeries"][0].update(mean=30.0, stdDev=6.0, required=40.0)
bad["betaComputed"] = 2.9
good["designWorkingLifeCategory"], good["designWorkingLifeYears"] = 2, 20.0
en = copy.deepcopy(good)
en["annex"] = "En"
en["members"][0].update(rdStr=300000.0, rdGeo=300000.0)
en_bad = copy.deepcopy(en)
en_bad["members"][0].update(rdStr=100000.0, rdGeo=100000.0)
for name, doc in (("annex-all-pass", good), ("annex-all-fail", bad), ("annex-en-610ab-pass", en), ("annex-en-610ab-fail", en_bad)):
    (OUT / name).mkdir(parents=True, exist_ok=True)
    json.dump(doc, open(OUT / name / "🔣️.json", "w", encoding="utf8"), ensure_ascii=False, indent=2)
