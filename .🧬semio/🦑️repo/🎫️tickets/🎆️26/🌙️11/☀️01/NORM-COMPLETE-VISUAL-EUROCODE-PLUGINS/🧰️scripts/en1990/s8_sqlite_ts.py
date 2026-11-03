import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
from spec import *
import json
SQ = SUB + "/🚪️io/🪶️sqlite/📸️snapshot"
p = SQ + "/🟦️.ts"; t = open(p, encoding="utf8").read()
if "FATIGUE_FLOATS" in t: raise SystemExit("done")
def rep(old, new):
    global t
    assert t.count(old) == 1, (old[:70], t.count(old)); t = t.replace(old, new)
rep("En1990MemberEffect as MemberEffect,", "En1990MemberEffect as MemberEffect,En1990FatigueSpectrum as FatigueSpectrum,En1990ReliabilityAnalysis as ReliabilityAnalysis,En1990TestSeries as TestSeries,")
rep('export type En1990SqliteMemberEffect=Omit<MemberEffect,"influence">&{influence:Binary64};', 'export type En1990SqliteMemberEffect=Omit<MemberEffect,"influence">&{influence:Binary64};\nexport type En1990SqliteFatigueSpectrum=Omit<FatigueSpectrum,"detailCategory"|"stressRange"|"cycles">&{detailCategory:Binary64;stressRange:Binary64;cycles:Binary64};\nexport type En1990SqliteReliabilityAnalysis=Omit<ReliabilityAnalysis,"resistanceMean"|"resistanceSd"|"actionMean"|"actionSd">&{resistanceMean:Binary64;resistanceSd:Binary64;actionMean:Binary64;actionSd:Binary64};\nexport type En1990SqliteTestSeries=Omit<TestSeries,"mean"|"stdDev"|"variationKnown"|"conversionFactor"|"required">&{mean:Binary64;stdDev:Binary64;variationKnown:Binary64;conversionFactor:Binary64;required:Binary64};')
rep('|"bridgeSls"|"effects">&{', '|"bridgeSls"|"effects"|"fatigueSpectra"|"reliabilityAnalyses"|"testSeries">&{')
rep("effects:En1990SqliteMemberEffect[]};", "effects:En1990SqliteMemberEffect[];fatigueSpectra:En1990SqliteFatigueSpectrum[];reliabilityAnalyses:En1990SqliteReliabilityAnalysis[];testSeries:En1990SqliteTestSeries[]};")
rep("const EFFECT_FLOATS:readonly Ieee754Column[]=[{index:7,width:64}];", "const EFFECT_FLOATS:readonly Ieee754Column[]=[{index:7,width:64}];\nconst FATIGUE_FLOATS:readonly Ieee754Column[]=[{index:6,width:64},{index:7,width:64},{index:8,width:64}];\nconst RELIABILITY_FLOATS:readonly Ieee754Column[]=[{index:5,width:64},{index:6,width:64},{index:8,width:64},{index:9,width:64}];\nconst TEST_FLOATS:readonly Ieee754Column[]=[{index:7,width:64},{index:8,width:64},{index:9,width:64},{index:10,width:64},{index:11,width:64}];")
rep("[action.category,action.qk],VARIABLE_FLOATS", "[action.category,action.qk,action.group],VARIABLE_FLOATS")
rep(" return out.finish();\n}", """ for(let position=0;position<snapshot.fatigueSpectra.length;position++){const spectrum=snapshot.fatigueSpectra[position]!;await write(out,"en1990_fatigue_spectrum",BigInt(position+1),[1n,BigInt(position),spectrum.id,spectrum.memberId,resolved(members,spectrum.memberId),spectrum.detailCategory,spectrum.stressRange,spectrum.cycles],FATIGUE_FLOATS,options)}
 for(let position=0;position<snapshot.reliabilityAnalyses.length;position++){const analysis=snapshot.reliabilityAnalyses[position]!;await write(out,"en1990_reliability_analysis",BigInt(position+1),[1n,BigInt(position),analysis.id,analysis.resistanceDistribution,analysis.resistanceMean,analysis.resistanceSd,analysis.actionDistribution,analysis.actionMean,analysis.actionSd],RELIABILITY_FLOATS,options)}
 for(let position=0;position<snapshot.testSeries.length;position++){const series=snapshot.testSeries[position]!;if(!Number.isInteger(series.sampleSize)||series.sampleSize<0||series.sampleSize>4294967295)throw Error("EN1990 sample size exceeds u32");await write(out,"en1990_test_series",BigInt(position+1),[1n,BigInt(position),series.id,series.quantity,series.distribution,BigInt(series.sampleSize),series.mean,series.stdDev,series.variationKnown,series.conversionFactor,series.required],TEST_FLOATS,options)}
 return out.finish();
}""")
rep("entities(tables[3]!,3,VARIABLE_FLOATS,options)", "entities(tables[3]!,4,VARIABLE_FLOATS,options)")
rep("const effectRows=await entities(tables[8]!,8,EFFECT_FLOATS,options);", "const effectRows=await entities(tables[8]!,8,EFFECT_FLOATS,options);const fatigueRows=await entities(tables[9]!,9,FATIGUE_FLOATS,options);const reliabilityRows=await entities(tables[10]!,10,RELIABILITY_FLOATS,options);const testRows=await entities(tables[11]!,12,TEST_FLOATS,options);")
rep("qk:readBinary64(row,2,VARIABLE_FLOATS)})}", "qk:readBinary64(row,2,VARIABLE_FLOATS),group:artifactSqliteText(row,3)})}")
rep(" const annex=artifactSqliteText(document,1);", """ const fatigueSpectra:En1990SqliteFatigueSpectrum[]=[];for(const row of await ordered([...fatigueRows.values()],options)){await checkpoint(options,fatigueSpectra.length,0);validateReference(row,5,artifactSqliteText(row,4),memberReferences);fatigueSpectra.push({id:artifactSqliteText(row,3),memberId:artifactSqliteText(row,4),detailCategory:readBinary64(row,6,FATIGUE_FLOATS),stressRange:readBinary64(row,7,FATIGUE_FLOATS),cycles:readBinary64(row,8,FATIGUE_FLOATS)})}
 const reliabilityAnalyses:En1990SqliteReliabilityAnalysis[]=[];for(const row of await ordered([...reliabilityRows.values()],options)){await checkpoint(options,reliabilityAnalyses.length,0);reliabilityAnalyses.push({id:artifactSqliteText(row,3),resistanceDistribution:artifactSqliteText(row,4),resistanceMean:readBinary64(row,5,RELIABILITY_FLOATS),resistanceSd:readBinary64(row,6,RELIABILITY_FLOATS),actionDistribution:artifactSqliteText(row,7),actionMean:readBinary64(row,8,RELIABILITY_FLOATS),actionSd:readBinary64(row,9,RELIABILITY_FLOATS)})}
 const testSeries:En1990SqliteTestSeries[]=[];for(const row of await ordered([...testRows.values()],options)){await checkpoint(options,testSeries.length,0);const sampleSize=artifactSqliteInteger(row,6);if(sampleSize<0n||sampleSize>4294967295n)throw Error("EN1990 sample size exceeds u32");testSeries.push({id:artifactSqliteText(row,3),quantity:artifactSqliteText(row,4),distribution:artifactSqliteText(row,5),sampleSize:Number(sampleSize),mean:readBinary64(row,7,TEST_FLOATS),stdDev:readBinary64(row,8,TEST_FLOATS),variationKnown:readBinary64(row,9,TEST_FLOATS),conversionFactor:readBinary64(row,10,TEST_FLOATS),required:readBinary64(row,11,TEST_FLOATS)})}
 const annex=artifactSqliteText(document,1);""")
rep("members,bridgeSls,effects};\n await artifactSqliteCheckpoint(options,\"reconstructSnapshot\",1,1)", "members,bridgeSls,effects,fatigueSpectra,reliabilityAnalyses,testSeries};\n await artifactSqliteCheckpoint(options,\"reconstructSnapshot\",1,1)")
open(p, "w", encoding="utf8").write(t)

# semantic contract + tests
ext = json.load(open(SQ + "/_extent.json"))
p = SQ + "/🧪️tests/🧫️fixtures/🎛️semantic.json"; d = json.load(open(p, encoding="utf8"))
d["schemaBytes"] = ext["schemaBytes"]; d["tableWidths"] = dict(sorted(ext["tableWidths"].items()))
open(p, "w", encoding="utf8").write(json.dumps(d, indent=2, ensure_ascii=False) + "\n")
import os; os.remove(SQ + "/_extent.json")
p = SQ + "/🧪️tests/🦀️.rs"; t = open(p, encoding="utf8").read()
rep("max_tables:9,max_columns:57", "max_tables:12,max_columns:57")
rep("max_tables:8,..limits", "max_tables:11,..limits")
rep("source.bridge_sls.clear();source.effects.clear();}", "source.bridge_sls.clear();source.effects.clear();source.fatigue_spectra.clear();source.reliability_analyses.clear();source.test_series.clear();}")
open(p, "w", encoding="utf8").write(t)
print("ts ok", ext["schemaBytes"])
