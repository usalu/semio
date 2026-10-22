#!/usr/bin/env python3
"""Writes src/schema.json: the closed contract of the performance SLO table and of `repo bench` (probes, units, SLO rows, samples, series, verdict rows, report, comparison, baseline, fixture) and the Policy const every implementation reads (statistics, minimum sample counts, percentile method, host load classes, start markers)."""
import json,os
here=os.path.dirname(os.path.abspath(__file__))
ID="^[a-z][a-z0-9]*(-[a-z0-9]+)*$"
def ref(n): return {"$ref":"#/definitions/"+n}
def obj(props,required=None,extra=False):
    return {"type":"object","additionalProperties":extra,"required":list(props) if required is None else required,"properties":props}
def arr(item,**kw): return {"type":"array","items":item,**kw}
def uniq(item,**kw): return {"type":"array","uniqueItems":True,"items":item,**kw}
def nullable(item): return {"oneOf":[item,{"type":"null"}]}
integer=lambda minimum=0:{"type":"integer","minimum":minimum}
number=lambda minimum=0:{"type":"number","minimum":minimum}
anynumber={"type":"number"}
string={"type":"string"}
nstring={"type":"string","minLength":1}
METRICS=["wallMs","startMs","greenMs","peakRssBytes","hitRatio","makespanMs","maxUnrelatedWaitMs","speedup"]
STATISTICS=["min","p50","p95","p99","max","mean"]
KINDS=["cli-help","noop","edit","graph","cache","concurrency"]
REASONS=["no-samples","too-few-samples","probe-failed","unavailable","not-cold","host-saturated","service-unavailable","unsupported","host-class","schema","start-unobserved"]
policy={
 "version":1,"id":ID,"taskId":"^\\S+:\\S+$","nonce":"^[0-9a-z]{6,32}$",
 "kinds":KINDS,"order":KINDS,
 "metrics":{
  "cli-help":{"plain":["wallMs"],"memory":[]},
  "noop":{"plain":["wallMs"],"memory":["peakRssBytes"]},
  "edit":{"plain":["startMs","greenMs"],"memory":["peakRssBytes"]},
  "graph":{"plain":["wallMs"],"memory":[]},
  "cache":{"plain":["hitRatio"],"memory":[]},
  "concurrency":{"plain":["makespanMs","maxUnrelatedWaitMs","speedup"],"memory":[]}
 },
 "units":{"wallMs":"ms","startMs":"ms","greenMs":"ms","peakRssBytes":"bytes","hitRatio":"ratio","makespanMs":"ms","maxUnrelatedWaitMs":"ms","speedup":"ratio"},
 "statistics":STATISTICS,
 "percentileMethod":"linear-inclusive",
 "minimumSamples":{"min":1,"p50":3,"p95":20,"p99":100,"max":1,"mean":3},
 "loadPerCore":{"quietBelow":0.5,"saturatedFrom":1.5},
 "maximumRuns":1000,"maximumWarmup":20,"maximumPathLength":255,"maximumLines":200000,"maximumLogBytes":268435456,
 "memoryIntervalMilliseconds":100,
 "markers":{
  "nx":["^> nx run "],
  "bazel":["^(?:INFO: )?(?:Analyzing|Analyzed)\\b","^\\[\\d+ / \\d+\\]"],
  "neutral":[]
 },
 "scriptRow":"^\\[script\\] (\\{.*\\})$",
 "defaultTolerance":{"relative":0.15,"absolute":25},
 "oneshotFlag":"--service=off","residentFlag":"--service=require",
 "serviceOperation":"service",
 "sources":["user-goal","cli-latency","graph-speed","hot-cache","conc-harness","provisional"],
 "reasons":REASONS,
 "statuses":["pass","warn","fail","unproven","recorded"],
 "compareStatuses":["improved","same","regressed","inconclusive","incomparable","new","missing"],
}
D={}
D["Policy"]={"const":policy}
D["Id"]={"type":"string","pattern":ID}
D["TaskId"]={"type":"string","pattern":"^\\S+:\\S+$"}
D["Path"]={"type":"string","minLength":1,"maxLength":255}
D["LocalizedText"]=obj({"en":nstring,"de":nstring})
D["Tool"]={"enum":["nx","bazel"]}
D["SloTool"]={"enum":["nx","bazel","neutral"]}
D["Kind"]={"enum":KINDS}
D["Lane"]={"enum":["oneshot","resident"]}
D["Metric"]={"enum":METRICS}
D["Statistic"]={"enum":STATISTICS}
D["Direction"]={"enum":["lower","higher"]}
D["Unit"]={"enum":["ms","bytes","ratio"]}
D["Enforcement"]={"enum":["advisory","blocking"]}
D["Source"]={"enum":policy["sources"]}
D["Language"]={"enum":["ts","rs","go","py"]}
D["EditOp"]={"enum":["comment","private-body","public-api"]}
D["Reason"]={"enum":REASONS}
D["Status"]={"enum":policy["statuses"]}
D["CompareStatus"]={"enum":policy["compareStatuses"]}
D["LoadClass"]={"enum":["quiet","busy","saturated","unknown"]}
D["StartKind"]={"enum":["marker","first-output","exit"]}
D["EditUnit"]=obj({"id":ref("Id"),"language":ref("Language"),"target":ref("TaskId"),"file":ref("Path"),"op":ref("EditOp"),"title":ref("LocalizedText")})
D["Probe"]=obj({"id":ref("Id"),"kind":ref("Kind"),"title":ref("LocalizedText"),"lane":ref("Lane"),"target":ref("TaskId"),"unit":ref("Id"),"workload":ref("Id"),"sessions":{"type":"integer","minimum":1,"maximum":8},"runs":{"type":"integer","minimum":1,"maximum":1000},"warmup":{"type":"integer","minimum":0,"maximum":20},"cold":{"type":"boolean"},"memory":{"type":"boolean"}},["id","kind","title","runs","warmup"])
D["Tolerance"]=obj({"relative":number(),"absolute":number()})
D["Slo"]=obj({"id":ref("Id"),"title":ref("LocalizedText"),"probe":ref("Id"),"metric":ref("Metric"),"statistic":ref("Statistic"),"direction":ref("Direction"),"goal":nullable(anynumber),"gate":nullable(anynumber),"tools":{"type":"array","minItems":1,"uniqueItems":True,"items":ref("SloTool")},"enforcement":ref("Enforcement"),"tolerance":ref("Tolerance"),"source":ref("Source")})
labels=["title","name","list","probes","slos","host","platform","cpus","memory","load","hostClass","probe","slo","tool","lane","oneshot","resident","neutral","metric","statistic","value","goal","gate","samples","status","reason","tolerance","enforcement","advisory","blocking","pass","warn","fail","unproven","recorded","improved","same","regressed","inconclusive","incomparable","new","missing","comparison","baseline","current","delta","relative","summary","series","quiet","busy","saturated","unknown","runs","warmup","kind","unit","source","progress","started","finished","startKinds","marker","first-output","exit","inflated","blockingFailed","blockingRegressed","notes","note-load","note-percentile","note-start","note-recorded","note-marks","cold","target","workload","sessions","file","language","edit","cache","graph","noop","cli-help","concurrency","nx","bazel","ms","bytes","ratio","wallMs","startMs","greenMs","peakRssBytes","hitRatio","makespanMs","maxUnrelatedWaitMs","speedup"]+REASONS
D["Labels"]=obj({k:ref("LocalizedText") for k in labels})
D["SloSet"]=obj({"version":{"const":1},"labels":ref("Labels"),"units":{"type":"array","minItems":1,"items":ref("EditUnit")},"probes":{"type":"array","minItems":1,"items":ref("Probe")},"slos":{"type":"array","minItems":1,"items":ref("Slo")}})
values={m:number() for m in METRICS}
D["Values"]=obj(values,[])
D["Sample"]=obj({"index":integer(),"ok":{"type":"boolean"},"exit":{"type":["integer","null"]},"values":ref("Values"),"startKind":nullable(ref("StartKind")),"loadPerCore":nullable(number())})
D["Stats"]=obj({"n":integer(1),"min":number(),"p50":number(),"p95":number(),"p99":number(),"max":number(),"mean":number()})
D["StatsMap"]=obj({m:ref("Stats") for m in METRICS},[])
D["StartKinds"]=obj({"marker":integer(),"first-output":integer(),"exit":integer()})
D["Series"]=obj({"probe":ref("Id"),"tool":ref("SloTool"),"lane":nullable(ref("Lane")),"status":{"enum":["measured","unavailable"]},"reason":nullable(ref("Reason")),"samples":arr(ref("Sample")),"failed":integer(),"stats":ref("StatsMap"),"loadClass":ref("LoadClass"),"startKinds":ref("StartKinds")})
D["Host"]=obj({"platform":nstring,"arch":nstring,"cpus":integer(1),"memoryBytes":integer(),"load1":nullable(number()),"loadPerCore":nullable(number()),"loadClass":ref("LoadClass"),"hostClass":nstring})
D["Row"]=obj({"slo":ref("Id"),"tool":ref("SloTool"),"probe":ref("Id"),"metric":ref("Metric"),"statistic":ref("Statistic"),"direction":ref("Direction"),"unit":ref("Unit"),"value":nullable(anynumber),"goal":nullable(anynumber),"gate":nullable(anynumber),"n":integer(),"status":ref("Status"),"reason":nullable(ref("Reason")),"inflated":{"type":"boolean"},"enforcement":ref("Enforcement"),"loadClass":ref("LoadClass")})
D["Tally"]=obj({"pass":integer(),"warn":integer(),"fail":integer(),"unproven":integer(),"recorded":integer(),"blockingFailed":integer()})
D["CompareRow"]=obj({"slo":ref("Id"),"tool":ref("SloTool"),"metric":ref("Metric"),"statistic":ref("Statistic"),"direction":ref("Direction"),"unit":ref("Unit"),"base":nullable(anynumber),"current":nullable(anynumber),"delta":nullable(anynumber),"relative":nullable(anynumber),"toleranceRelative":number(),"toleranceAbsolute":number(),"status":ref("CompareStatus"),"enforcement":ref("Enforcement"),"reason":nullable(ref("Reason"))})
D["CompareTally"]=obj({"improved":integer(),"same":integer(),"regressed":integer(),"inconclusive":integer(),"incomparable":integer(),"new":integer(),"missing":integer(),"blockingRegressed":integer()})
D["Comparison"]=obj({"version":{"const":1},"currentHostClass":nstring,"baselineHostClass":nstring,"rows":arr(ref("CompareRow")),"tally":ref("CompareTally")})
D["Report"]=obj({"version":{"const":1},"kind":{"const":"bench-report"},"startedAt":nstring,"finishedAt":nstring,"host":ref("Host"),"hostAfter":ref("Host"),"tools":{"type":"array","items":ref("Tool")},"runsOverride":nullable({"type":"integer","minimum":1,"maximum":1000}),"series":arr(ref("Series")),"rows":arr(ref("Row")),"tally":ref("Tally"),"comparison":nullable(ref("Comparison"))})
D["Fixture"]=obj({"version":{"const":1},
 "percentiles":arr(obj({"name":nstring,"values":arr(anynumber),"expected":obj({"n":integer(),"min":anynumber,"p50":anynumber,"p95":anynumber,"p99":anynumber,"max":anynumber,"mean":anynumber})})),
 "loads":arr(obj({"name":nstring,"perCore":nullable(number()),"class":ref("LoadClass")})),
 "judgements":arr(obj({"name":nstring,"slo":ref("Slo"),"series":nullable(ref("Series")),"row":ref("Row")})),
 "comparisons":arr(obj({"name":nstring,"slos":arr(ref("Slo")),"current":ref("Report"),"baseline":ref("Report"),"comparison":ref("Comparison")})),
 "tallies":arr(obj({"name":nstring,"rows":arr(ref("Row")),"tally":ref("Tally")})),
 "series":arr(obj({"name":nstring,"input":{},"series":ref("Series")})),
 "selection":arr(obj({"name":nstring,"set":{},"ids":{},"tools":arr(ref("Tool")),"pairs":arr(nstring),"error":nullable(nstring)})),
 "mutations":arr(obj({"name":nstring,"text":string,"op":ref("EditOp"),"language":ref("Language"),"nonce":nstring,"result":string})),
 "markers":arr(obj({"name":nstring,"tool":ref("SloTool"),"lines":arr(string),"first":nullable(integer())})),
 "memory":arr(obj({"name":nstring,"table":string,"pid":integer(1),"system":nstring,"rss":nullable(integer())}))})
schema={"$schema":"http://json-schema.org/draft-07/schema#","title":"Performance SLO Table and Bench","description":"The service level objectives of the build system (hot-cache latency, edit to rebuild, cache hit rate, graph, memory, concurrency), the probes that measure them on a disposable checkout copy behind the tool-neutral interface, and the report and regression comparison of `repo bench`; every build implementation is held to the same table.","type":"object","additionalProperties":False,"required":["version"],"properties":{"version":{"const":1}},"definitions":D}
json.dump(schema,open(os.path.join(here,"src","schema.json"),"w",encoding="utf8"),ensure_ascii=False,indent=2);open(os.path.join(here,"src","schema.json"),"a").write("\n")
print("schema definitions:",len(D),"labels:",len(labels))
