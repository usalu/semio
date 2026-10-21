#!/usr/bin/env python3
"""Writes src/fixture.json: language-agnostic cases (inputs) and their expected outputs, computed by the independent Python oracle; the TypeScript implementation is held to the same answers."""
import copy,json,os,random,subprocess,sys
here=os.path.dirname(os.path.abspath(__file__))
SCHEMA=os.path.join(here,"src","schema.json");ORACLE=os.path.join(here,"src","oracle.py")
TABLE=json.load(open(os.path.join(here,"src","slo.json"),encoding="utf8"))
def ask(op,cases):
    run=subprocess.run(["python3","-I",ORACLE],input=json.dumps({"op":op,"schemaPath":SCHEMA,"cases":cases}),capture_output=True,text=True)
    if run.returncode: sys.exit(run.stderr)
    return json.loads(run.stdout)
T=lambda en,de:{"en":en,"de":de}
def slo(id,probe="p",metric="wallMs",statistic="p50",direction="lower",goal=300,gate=1000,tools=("nx",),enforcement="advisory",rel=0.15,absolute=25,source="provisional"):
    return {"id":id,"title":T(id,id),"probe":probe,"metric":metric,"statistic":statistic,"direction":direction,"goal":goal,"gate":gate,"tools":list(tools),"enforcement":enforcement,"tolerance":{"relative":rel,"absolute":absolute},"source":source}
def sample(i,value,metric="wallMs",ok=True,load=0.2,kind=None,exit_=0):
    return {"index":i,"ok":ok,"exit":exit_ if ok else (1 if exit_==0 else exit_),"values":{} if not ok or value is None else {metric:value},"startKind":kind,"loadPerCore":load}
def ser(probe="p",tool="nx",lane="oneshot",status="measured",reason=None,samples=()):
    return {"probe":probe,"tool":tool,"lane":lane,"status":status,"reason":reason,"samples":list(samples)}
def built(cases): return ask("series",cases)
percentiles=[]
rnd=random.Random(7)
def pc(name,values): percentiles.append({"name":name,"values":values})
pc("single",[5.0]);pc("two",[1.0,3.0]);pc("three-unsorted",[9.0,1.0,5.0]);pc("twenty",[float(i) for i in range(1,21)]);pc("twenty-one",[float(i) for i in range(1,22)])
pc("hundred",[float(i) for i in range(1,101)]);pc("hundred-one",[float(i)*1.5 for i in range(1,102)]);pc("duplicates",[3.0]*7+[9.0]*3);pc("fractions",[0.1,0.2,0.30000000000000004,0.4,0.5,0.6,0.7]);pc("big",[1e9+i for i in range(10)])
pc("reverse",[float(i) for i in range(30,0,-1)])
for n in (2,4,5,7,10,19,20,33,60,99,100,150):
    pc(f"random-{n}",[round(rnd.uniform(5,900),3) for _ in range(n)])
loads=[{"name":f"load-{i}","perCore":v} for i,v in enumerate([None,0.0,0.49,0.5,1.0,1.49,1.5,3.2])]
series_in=[]
series_in.append({"name":"mixed-with-failure","probe":"p","tool":"nx","lane":"oneshot","status":"measured","reason":None,"samples":[sample(0,120.0,kind="marker"),sample(1,None,ok=False),sample(2,130.5,kind="first-output",load=1.7),sample(3,125.0,kind="exit",load=None)]})
series_in.append({"name":"no-loads","probe":"p","tool":"nx","lane":None,"status":"measured","reason":None,"samples":[sample(0,10.0,load=None),sample(1,11.0,load=None)]})
series_in.append({"name":"two-metrics","probe":"e","tool":"bazel","lane":"resident","status":"measured","reason":None,"samples":[{"index":0,"ok":True,"exit":0,"values":{"startMs":50.0,"greenMs":900.0},"startKind":"marker","loadPerCore":0.3},{"index":1,"ok":True,"exit":0,"values":{"startMs":70.0,"greenMs":950.0},"startKind":"marker","loadPerCore":0.4},{"index":2,"ok":False,"exit":1,"values":{"startMs":60.0},"startKind":"marker","loadPerCore":0.4}]})
series_in.append({"name":"unavailable","probe":"p","tool":"nx","lane":"resident","status":"unavailable","reason":"service-unavailable","samples":[]})
series_in.append({"name":"empty-measured","probe":"p","tool":"nx","lane":None,"status":"measured","reason":None,"samples":[]})
series_out=built(series_in)
def sr(**kw): return built([kw])[0]
def rep_series(probe="p",tool="nx",values=(100.0,),load=0.2,metric="wallMs",**kw):
    return sr(probe=probe,tool=tool,lane=kw.get("lane","oneshot"),status="measured",reason=None,samples=[sample(i,v,metric=metric,load=load) for i,v in enumerate(values)])
judge_cases=[]
def jc(name,s,series): judge_cases.append({"name":name,"slo":s,"tool":"nx","series":series})
jc("pass-goal",slo("a"),rep_series(values=(90.0,100.0,110.0,95.0,105.0)))
jc("warn-between",slo("a"),rep_series(values=(500.0,510.0,520.0)))
jc("fail-quiet",slo("a"),rep_series(values=(1500.0,1600.0,1700.0)))
jc("fail-saturated-is-unproven",slo("a"),rep_series(values=(1500.0,1600.0,1700.0),load=2.0))
jc("pass-saturated-stays-pass",slo("a"),rep_series(values=(100.0,100.0,100.0),load=3.0))
jc("warn-saturated-inflated",slo("a"),rep_series(values=(500.0,500.0,500.0),load=3.0))
jc("fail-unknown-load",slo("a"),rep_series(values=(1500.0,1600.0,1700.0),load=None))
jc("too-few-p50",slo("a"),rep_series(values=(100.0,110.0)))
jc("p95-19-samples",slo("a",statistic="p95"),rep_series(values=tuple(100.0+i for i in range(19))))
jc("p95-20-samples",slo("a",statistic="p95"),rep_series(values=tuple(100.0+i for i in range(20))))
jc("p99-99-samples",slo("a",statistic="p99"),rep_series(values=tuple(100.0+i for i in range(99))))
jc("p99-100-samples",slo("a",statistic="p99"),rep_series(values=tuple(100.0+i for i in range(100))))
jc("max-one-sample",slo("a",statistic="max"),rep_series(values=(250.0,)))
jc("higher-pass",slo("h",metric="hitRatio",statistic="min",direction="higher",goal=1,gate=1,enforcement="blocking"),rep_series(metric="hitRatio",values=(1.0,1.0,1.0)))
jc("higher-fail",slo("h",metric="hitRatio",statistic="min",direction="higher",goal=1,gate=1,enforcement="blocking"),rep_series(metric="hitRatio",values=(1.0,0.9,1.0)))
jc("higher-warn",slo("h",metric="hitRatio",statistic="min",direction="higher",goal=1,gate=0.95),rep_series(metric="hitRatio",values=(1.0,0.97,1.0)))
jc("recorded",slo("r",goal=None,gate=None),rep_series(values=(5000.0,5200.0,5100.0),load=3.0))
jc("gate-only-pass",slo("g",goal=None,gate=400),rep_series(values=(350.0,360.0,370.0)))
jc("gate-only-fail",slo("g",goal=None,gate=400),rep_series(values=(450.0,460.0,470.0)))
jc("no-series",slo("a"),None)
jc("unavailable-service",slo("a"),series_out[3])
jc("all-failed",slo("a"),sr(probe="p",tool="nx",lane="oneshot",status="measured",reason=None,samples=[sample(0,None,ok=False),sample(1,None,ok=False)]))
jc("empty-measured",slo("a"),series_out[4])
jc("metric-absent",slo("a",metric="startMs"),rep_series(values=(1.0,2.0,3.0)))
jc("bytes-pass",slo("m",metric="peakRssBytes",statistic="max",goal=536870912,gate=1073741824),rep_series(metric="peakRssBytes",values=(300000000.0,310000000.0,305000000.0),load=3.0))
jc("blocking-fail",slo("b",enforcement="blocking",goal=300,gate=300,statistic="p95"),rep_series(values=tuple(400.0+i for i in range(30))))
judge_out=ask("judge",judge_cases)
def tally(rows):
    c=lambda s:sum(1 for r in rows if r["status"]==s)
    return {"pass":c("pass"),"warn":c("warn"),"fail":c("fail"),"unproven":c("unproven"),"recorded":c("recorded"),"blockingFailed":sum(1 for r in rows if r["status"]=="fail" and r["enforcement"]=="blocking")}
def host(cls="darwin-arm64-10",load=0.2):
    return {"platform":cls.split("-")[0],"arch":cls.split("-")[1],"cpus":int(cls.split("-")[2]),"memoryBytes":17179869184,"load1":load*int(cls.split("-")[2]),"loadPerCore":load,"loadClass":"quiet" if load<0.5 else "saturated" if load>=1.5 else "busy","hostClass":cls}
def report(slos,series_list,cls="darwin-arm64-10",load=0.2):
    rows=[];by={ (s["probe"],s["tool"]):s for s in series_list}
    for s in slos:
        for t in s["tools"]: rows.append(ask("judge",[{"slo":s,"tool":t,"series":by.get((s["probe"],t))}])[0])
    return {"version":1,"kind":"bench-report","startedAt":"2026-10-21T00:00:00.000Z","finishedAt":"2026-10-21T00:10:00.000Z","host":host(cls,load),"hostAfter":host(cls,load),"tools":["nx"],"runsOverride":None,"series":series_list,"rows":rows,"tally":tally(rows),"comparison":None}
A=slo("a",tools=("nx",));B=slo("b",probe="q",enforcement="blocking",goal=300,gate=300,statistic="p50");H=slo("h",probe="c",metric="hitRatio",statistic="min",direction="higher",goal=1,gate=1,enforcement="blocking",rel=0,absolute=0)
def three(a,b,c,la=0.2,lb=0.2):
    return [rep_series("p","nx",a,load=la),rep_series("q","nx",b,load=lb),rep_series("c","nx",c,metric="hitRatio")]
comparisons=[]
def cc(name,slos,cur,base): comparisons.append({"name":name,"slos":slos,"current":cur,"baseline":base})
S=[A,B,H]
cc("same-within-tolerance",S,report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3)),report(S,three((105.0,)*3,(255.0,)*3,(1.0,)*3)))
cc("regressed-and-blocking",S,report(S,three((300.0,)*3,(400.0,)*3,(1.0,)*3)),report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3)))
cc("improved",S,report(S,three((50.0,)*3,(100.0,)*3,(1.0,)*3)),report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3)))
Z=slo("a",rel=0.25,absolute=10)
cc("boundary-exact-is-same",[Z],report([Z],[rep_series("p","nx",(135.0,)*3)]),report([Z],[rep_series("p","nx",(100.0,)*3)]))
cc("boundary-over-regressed",[Z],report([Z],[rep_series("p","nx",(135.5,)*3)]),report([Z],[rep_series("p","nx",(100.0,)*3)]))
cc("boundary-lower-exact",[Z],report([Z],[rep_series("p","nx",(65.0,)*3)]),report([Z],[rep_series("p","nx",(100.0,)*3)]))
cc("boundary-lower-over-improved",[Z],report([Z],[rep_series("p","nx",(64.5,)*3)]),report([Z],[rep_series("p","nx",(100.0,)*3)]))
cc("higher-regressed",S,report(S,three((100.0,)*3,(250.0,)*3,(0.9,)*3)),report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3)))
cc("saturated-current-inconclusive",S,report(S,three((300.0,)*3,(250.0,)*3,(1.0,)*3,la=2.0),load=2.0),report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3)))
cc("saturated-baseline-inconclusive",S,report(S,three((300.0,)*3,(250.0,)*3,(1.0,)*3)),report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3,la=2.0)))
cc("saturated-improvement-stays",S,report(S,three((50.0,)*3,(250.0,)*3,(1.0,)*3,la=2.0)),report(S,three((200.0,)*3,(250.0,)*3,(1.0,)*3)))
cc("host-class-differs",S,report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3),cls="linux-x64-8"),report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3)))
cc("unproven-sides",S,report(S,[rep_series("p","nx",(100.0,)),rep_series("q","nx",(250.0,)*3),rep_series("c","nx",(1.0,)*3,metric="hitRatio")]),report(S,three((100.0,)*3,(250.0,)*3,(1.0,)*3)))
cc("new-row",[A,B],report([A,B],[rep_series("p","nx",(100.0,)*3),rep_series("q","nx",(250.0,)*3)]),report([A],[rep_series("p","nx",(100.0,)*3)]))
cc("missing-when-probe-present",[A],report([A],[rep_series("p","nx",(100.0,)*3),rep_series("q","nx",(250.0,)*3)]),report([A,B],[rep_series("p","nx",(100.0,)*3),rep_series("q","nx",(250.0,)*3)]))
cc("not-missing-when-probe-absent",[A],report([A],[rep_series("p","nx",(100.0,)*3)]),report([A,B],[rep_series("p","nx",(100.0,)*3),rep_series("q","nx",(250.0,)*3)]))
cc("unknown-objective-uses-default-tolerance",[],report([A],[rep_series("p","nx",(140.0,)*3)]),report([A],[rep_series("p","nx",(100.0,)*3)]))
cc("zero-base",[A],report([A],[rep_series("p","nx",(30.0,)*3)]),report([A],[rep_series("p","nx",(0.0,)*3)]))
comp_out=ask("compare",[{"slos":c["slos"],"current":c["current"],"baseline":c["baseline"]} for c in comparisons])
tallies=[]
for name,rows in (("mixed",judge_out),("empty",[]),("only-pass",[r for r in judge_out if r["status"]=="pass"])): tallies.append({"name":name,"rows":rows,"tally":tally(rows)})
def mini(kind,lane=None,cold=False):
    p={"id":kind+("-cold" if cold else "")+(("-"+lane) if lane else ""),"kind":kind,"title":T("x","x"),"runs":3,"warmup":0}
    if lane: p["lane"]=lane
    if cold: p["cold"]=True
    return p
sel_set={"version":1,"labels":{},"units":[],"probes":[mini("concurrency"),mini("edit","oneshot"),mini("cli-help"),mini("graph","oneshot",True),mini("noop","oneshot"),mini("noop","resident")],"slos":[]}
for p in sel_set["probes"]:
    sel_set["slos"].append(slo(p["id"]+"-a",probe=p["id"],tools=("neutral",) if p["kind"]=="cli-help" else ("nx","bazel")))
    sel_set["slos"].append(slo(p["id"]+"-b",probe=p["id"],tools=("neutral",) if p["kind"]=="cli-help" else ("bazel",)))
selection=[]
for name,ids,tools in (("all-both",  "all",["nx","bazel"]),("all-nx","all",["nx"]),("one-id",["edit-oneshot-a"],["nx","bazel"]),("kind",["kind:noop"],["bazel"]),("probe",["probe:noop-resident"],["nx"]),("mixed-ids",["cli-help-a","kind:graph"],["nx"]),("unknown-id",["nope"],["nx"]),("unknown-kind",["kind:nope"],["nx"]),("empty-after-tool-filter",["noop-resident-a"],["bazel"]),("cold-first","all",["nx","bazel"]),("duplicates",["edit-oneshot-a","edit-oneshot-a","kind:edit"],["nx"])):
    selection.append({"name":name,"set":sel_set,"ids":ids,"tools":tools})
sel_out=ask("select",selection)
for row,answer in zip(selection,sel_out): row["pairs"]=answer["pairs"];row["error"]=answer["error"]
muts=[]
for text in ("export const a=1;\n","no newline","", "line\n"):
    for op in ("comment","private-body","public-api"):
        for lang in ("ts","rs","go","py"):
            muts.append({"name":f"{lang}-{op}-{len(text)}","text":text,"op":op,"language":lang,"nonce":"abc123x9"})
muts.append({"name":"nonce-numeric","text":"a\n","op":"private-body","language":"ts","nonce":"000001ab"})
mut_out=ask("mutate",muts)
marker_cases=[
 {"name":"nx-first","tool":"nx","lines":["[script] loading","> nx run a:b","> nx run c:d"]},
 {"name":"nx-none","tool":"nx","lines":["[script] loading","done"]},
 {"name":"nx-needs-line-start","tool":"nx","lines":["x > nx run a:b"]},
 {"name":"bazel-analyzing","tool":"bazel","lines":["Starting local Bazel server","INFO: Analyzing target //a:b (1 packages loaded)","INFO: Analyzed target"]},
 {"name":"bazel-analyzed-only","tool":"bazel","lines":["INFO: Analyzed target //a:b"]},
 {"name":"bazel-progress","tool":"bazel","lines":["warming","[3 / 17] Compiling a.rs"]},
 {"name":"bazel-no-bracket","tool":"bazel","lines":["[a / b] nope"]},
 {"name":"neutral-never","tool":"neutral","lines":["> nx run a:b","INFO: Analyzing"]},
 {"name":"empty","tool":"nx","lines":[]}]
marker_out=ask("markers",marker_cases)
mem_cases=[
 {"name":"posix-tree","table":"  1  0  100\n 10  1  2048\n 11 10  4096\n 12  1  50\n 99  0  7\n","pid":10,"system":"darwin"},
 {"name":"posix-root-only","table":"10 1 300\n11 10 100\n","pid":11,"system":"linux"},
 {"name":"posix-missing","table":"10 1 300\n","pid":77,"system":"linux"},
 {"name":"posix-garbage","table":"PID PPID RSS\n10 1 300\nx y z\n11 10 100\n","pid":10,"system":"linux"},
 {"name":"windows-csv","table":"\"4\",\"0\",\"100\"\n10,4,2000000\n11,10,3000000\n12,4,500\n","pid":10,"system":"win32"},
 {"name":"deep-chain","table":"1 0 1\n2 1 1\n3 2 1\n4 3 1\n5 4 1\n","pid":2,"system":"darwin"},
 {"name":"crlf","table":"10 1 1\r\n11 10 2\r\n","pid":10,"system":"linux"}]
mem_out=ask("tree",mem_cases)
fixture={"version":1,
 "percentiles":[{**c,"expected":e} for c,e in zip(percentiles,ask("stats",percentiles))],
 "loads":[{"name":c["name"],"perCore":c["perCore"],"class":e} for c,e in zip(loads,ask("load",loads))],
 "series":[{"name":c["name"],"input":{k:c[k] for k in c if k!="name"},"series":e} for c,e in zip(series_in,series_out)],
 "judgements":[{"name":c["name"],"slo":c["slo"],"series":c["series"],"row":e} for c,e in zip(judge_cases,judge_out)],
 "comparisons":[{**c,"comparison":e} for c,e in zip(comparisons,comp_out)],
 "tallies":tallies,
 "selection":selection,
 "mutations":[{**c,"result":e} for c,e in zip(muts,mut_out)],
 "markers":[{"name":c["name"],"tool":c["tool"],"lines":c["lines"],"first":e} for c,e in zip(marker_cases,marker_out)],
 "memory":[{**c,"rss":e} for c,e in zip(mem_cases,mem_out)]}
json.dump(fixture,open(os.path.join(here,"src","fixture.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1);open(os.path.join(here,"src","fixture.json"),"a").write("\n")
print({k:len(v) if isinstance(v,list) else v for k,v in fixture.items()})
