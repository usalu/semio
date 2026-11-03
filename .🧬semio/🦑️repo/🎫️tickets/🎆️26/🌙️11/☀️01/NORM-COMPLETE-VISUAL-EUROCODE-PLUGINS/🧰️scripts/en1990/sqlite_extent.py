"""Independent model of the EN 1990 SQLite snapshot extent: rows and valueBytes of a fixture.
Mirrors `independentSqliteExtent`: integers and reals 8 bytes, NULL 0, text its UTF-8 length."""
import json, math, sys

def klass(x):
    if math.isnan(x): return "nan"
    if x == math.inf: return "positiveInfinity"
    if x == -math.inf: return "negativeInfinity"
    return "finite"

def t(s): return len(s.encode("utf-8"))
def f(x): return (0 if math.isnan(x) else 8) + 8 + t(klass(x))

def unique(names):
    seen = {}
    for n in names: seen[n] = seen.get(n, 0) + 1
    return {n for n, c in seen.items() if c == 1}

def extent(d):
    rows = 0; b = 0
    def row(v):
        nonlocal rows, b
        rows += 1; b += v
    row(8 + t(d["annex"].capitalize() if d["annex"] in ("En", "De") else d["annex"]) + t(d["projectId"]) + t(d["structureKind"]) + 8 + 8*3 + 8 + 8 + t(d["supervisionLevel"]) + t(d["inspectionLevel"]) + 8 + 8 + 0)
    b += sum(f(d[k]) - 8 for k in ("altitudeM", "designWorkingLifeYears", "referencePeriodYears", "kFiDeclared", "betaComputed")) - 0
    actions = [("permanent", a["id"]) for a in d["permanents"]] + [("variable", a["id"]) for a in d["variables"]] + [("accidental", a["id"]) for a in d["accidentals"]] + [("seismic", a["id"]) for a in d["seismics"]]
    for kind, i in actions: row(8 + 8 + t(kind) + 8 + t(i))
    for a in d["permanents"]: row(8 + t(a["kind"]) + f(a["gk"]))
    for a in d["variables"]: row(8 + t(a["category"]) + f(a["qk"]) + t(a["group"]))
    for a in d["accidentals"]: row(8 + f(a["ad"]))
    for a in d["seismics"]: row(8 + f(a["aEk"]) + t(a["importanceClass"]))
    mem_ok = unique([m["id"] for m in d["members"]])
    act_ok = unique([i for _, i in actions])
    mf = ["rdStr","rdGeo","rdEquStab","rdEquDestab","rdFat","span","precamber","deflectionW1","deflectionW2","deflectionW3","deflectionLimitRatio","deflectionLimitRatioW3","slsLimitChar","slsLimitFreq","slsLimitQp","vibrationFrequency","vibrationFrequencyMin","frameHeight","frameDrift","storeyHeight","storeyDrift","materialStrengthK","materialConversionFactor","materialPartialFactor","materialDesignStrength","nominalDimension","dimensionTolerance","designDimension","memberAlphaCr","secondOrderAnalysis"]
    for m in d["members"]: row(8*3 + t(m["id"]) + t(m["labelEn"]) + t(m["labelDe"]) + sum(f(m[k]) for k in mf))
    bf = ["deckAcceleration","deckAccelerationLimit","deckTwist","deckTwistLimit","bridgeDeflection","bridgeDeflectionLimit","trackGauge","lineSpeed","ballastDepth","deckMass","naturalFrequency","endRotation","basicWindSpeed","dampingRatio"]
    for s in d["bridgeSls"]: row(8*3 + t(s["id"]) + t(s["memberId"]) + (8 if s["memberId"] in mem_ok else 0) + sum(f(s[k]) for k in bf))
    for e in d["effects"]: row(8*3 + t(e["memberId"]) + t(e["actionId"]) + (8 if e["memberId"] in mem_ok else 0) + (8 if e["actionId"] in act_ok else 0) + f(e["influence"]))
    for s in d["fatigueSpectra"]: row(8*3 + t(s["id"]) + t(s["memberId"]) + (8 if s["memberId"] in mem_ok else 0) + sum(f(s[k]) for k in ("detailCategory","stressRange","cycles")))
    for r in d["reliabilityAnalyses"]: row(8*3 + t(r["id"]) + t(r["resistanceDistribution"]) + t(r["actionDistribution"]) + sum(f(r[k]) for k in ("resistanceMean","resistanceSd","actionMean","actionSd")))
    for s in d["testSeries"]: row(8*3 + t(s["id"]) + t(s["quantity"]) + t(s["distribution"]) + 8 + sum(f(s[k]) for k in ("mean","stdDev","variationKnown","conversionFactor","required")))
    for o in d.get("factorOverrides", []): row(8*3 + t(o["id"]) + t(o["factor"]) + t(o["target"]) + t(o["justification"]) + f(o["value"]))
    return rows, b

if __name__ == "__main__":
    d = json.load(open(sys.argv[1], encoding="utf8"))
    print("full", extent(d))
    e = dict(d)
    for k in ("permanents","variables","accidentals","seismics","members","bridgeSls","effects","fatigueSpectra","reliabilityAnalyses","testSeries","factorOverrides"): e[k] = []
    print("empty", extent(e))
