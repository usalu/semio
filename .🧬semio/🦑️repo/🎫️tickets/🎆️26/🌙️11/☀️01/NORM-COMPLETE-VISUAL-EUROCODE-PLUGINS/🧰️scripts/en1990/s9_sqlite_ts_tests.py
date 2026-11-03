from spec import *
import json
SQ = SUB + "/🚪️io/🪶️sqlite/📸️snapshot"
p = SQ + "/🧪️tests/🟦️.ts"; t = open(p, encoding="utf8").read()
if "fatigueSpectra" in t: raise SystemExit("done")
def rep(old, new, count=1):
    global t
    assert t.count(old) == count, (old[:70], t.count(old)); t = t.replace(old, new)
rep("effects:f.effects.map(e=>({...e,influence:binary64(e.influence)}))};", "effects:f.effects.map(e=>({...e,influence:binary64(e.influence)})),fatigueSpectra:f.fatigueSpectra.map(x=>({...x,detailCategory:binary64(x.detailCategory),stressRange:binary64(x.stressRange),cycles:binary64(x.cycles)})),reliabilityAnalyses:f.reliabilityAnalyses.map(x=>({...x,resistanceMean:binary64(x.resistanceMean),resistanceSd:binary64(x.resistanceSd),actionMean:binary64(x.actionMean),actionSd:binary64(x.actionSd)})),testSeries:f.testSeries.map(x=>({...x,mean:binary64(x.mean),stdDev:binary64(x.stdDev),variationKnown:binary64(x.variationKnown),conversionFactor:binary64(x.conversionFactor),required:binary64(x.required)}))};")
rep(" for(const e of s.effects)e.influence=value;\n", " for(const e of s.effects)e.influence=value;\n for(const x of s.fatigueSpectra){x.detailCategory=value;x.stressRange=value;x.cycles=value}\n for(const x of s.reliabilityAnalyses){x.resistanceMean=value;x.resistanceSd=value;x.actionMean=value;x.actionSd=value}\n for(const x of s.testSeries){x.mean=value;x.stdDev=value;x.variationKnown=value;x.conversionFactor=value;x.required=value}\n")
rep("bridgeSls:[],effects:[]};", "bridgeSls:[],effects:[],fatigueSpectra:[],reliabilityAnalyses:[],testSeries:[]};")
rep("source.bridgeSls=[];source.effects=[];}", "source.bridgeSls=[];source.effects=[];source.fatigueSpectra=[];source.reliabilityAnalyses=[];source.testSeries=[];}")
c = json.load(open(SQ + "/🧪️tests/🧫️fixtures/🎛️semantic.json", encoding="utf8"))
import re
old = re.search(r'expect\(normSemanticContract\["schemaBytes"\]\)\.toEqual\(5052\);expect\(normSemanticContract\["tableWidths"\]\)\.toEqual\(\{[^}]*\}\);expect\(normSemanticContract\["cases"\]\)\.toEqual\(\[[^\]]*\]\);', t).group(0)
t = t.replace(old, f'expect(normSemanticContract["schemaBytes"]).toEqual({c["schemaBytes"]});expect(normSemanticContract["tableWidths"]).toEqual({json.dumps(c["tableWidths"], separators=(",", ":"))});expect(normSemanticContract["cases"]).toEqual({json.dumps(c["cases"], separators=(",", ":"))});')
rep("maxTables:9,maxColumns:57", "maxTables:12,maxColumns:57")
rep("{...limits,maxTables:8}", "{...limits,maxTables:11}")
open(p, "w", encoding="utf8").write(t)
print("ok")
