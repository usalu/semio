import json,sys
src,out=sys.argv[1],sys.argv[2]
tonight=json.load(open(src))["episodes"]
def comp(jobs):return [{"id":j["id"],"class":j["class"],"arrival":j["arrival"],"service":j["service"],"lane":j["lane"][:64],"observedWait":j["observedWait"]} for j in jobs]
sims=[{"id":e["id"],"source":"queue notices of the shard-7 graph builder lease, 10-25 19:00 to 10-26 07:30 (arrival = first notice, service = difference of consecutive acquisitions)","resource":e["resource"],"spanSeconds":e["spanSeconds"],"jobs":comp(e["jobs"])} for e in tonight[:3]]
dev=[{"id":f"dev{i}","class":"interactive","arrival":3600*(i+1),"service":300} for i in range(6)]
sims.append({"id":"tonight-0-with-dev","source":"tonight-0 plus six interactive rebuilds of 300 s, one per hour","resource":tonight[0]["resource"],"spanSeconds":tonight[0]["spanSeconds"],"jobs":comp(tonight[0]["jobs"])+dev})
sims.append({"id":"background-under-interactive-stream","source":"a background request followed by an interactive request every 10 min, 40 min each","resource":"synthetic","jobs":[{"id":"holder","class":"agent","arrival":0,"service":3000},{"id":"background","class":"background","arrival":60,"service":600}]+[{"id":f"ui{i:02d}","class":"interactive","arrival":120+600*i,"service":2400} for i in range(40)]})
sims.append({"id":"equal-class-fifo","source":"same class keeps arrival order","resource":"synthetic","jobs":[{"id":f"a{i}","class":"agent","arrival":i*7,"service":100} for i in range(12)]})
W=1700000000000
im=lambda load,comps:{"digest":"0123456789abcdef","loadedAtMs":load,"components":comps}
standing=lambda progress,load=W,comps=None,**x:{"version":1,"priority":"agent","implementation":im(load,comps or {"graph.generation":"aaaaaaaaaaaa"}),"progressAtMs":progress,**x}
G="repository-graph-facts:/w:shard:7"
states=[
 {"id":"legacy","nowMs":W+1000000,"holdSinceMs":W,"resource":G,"expected":{"state":"legacy"}},
 {"id":"progressing","nowMs":W+1000000,"holdSinceMs":W,"resource":G,"standing":standing(W+999000),"expected":{"state":"progressing"}},
 {"id":"silent","nowMs":W+1000000,"holdSinceMs":W,"resource":G,"standing":standing(W+880000),"expected":{"state":"silent"}},
 {"id":"stuck-graph","nowMs":W+3000000,"holdSinceMs":W,"resource":G,"standing":standing(W+1700000),"expected":{"state":"stuck"}},
 {"id":"silent-not-stuck-other-resource","nowMs":W+3000000,"holdSinceMs":W,"resource":"native-fixed-image:/w/.cache/b/g","standing":standing(W+1700000),"expected":{"state":"silent"}},
 {"id":"stuck-other-after-one-hour","nowMs":W+5000000,"holdSinceMs":W,"resource":"native-fixed-image:/w/.cache/b/g","standing":standing(W+1000000),"expected":{"state":"stuck"}},
 {"id":"hold-newer-than-progress","nowMs":W+3000000,"holdSinceMs":W+2990000,"resource":G,"standing":standing(W+10000),"expected":{"state":"progressing"}},
 {"id":"stale-newer-waiter","nowMs":W+2000000,"holdSinceMs":W,"resource":G,"standing":standing(W+1999000),"waiter":im(W+500000,{"graph.generation":"bbbbbbbbbbbb"}),"bound":["graph.generation"],"expected":{"state":"stale","staleIds":["graph.generation"]}},
 {"id":"not-stale-older-waiter","nowMs":W+2000000,"holdSinceMs":W,"resource":G,"standing":standing(W+1999000,W+600000),"waiter":im(W+500000,{"graph.generation":"bbbbbbbbbbbb"}),"bound":["graph.generation"],"expected":{"state":"progressing"}},
 {"id":"not-stale-unbound-component","nowMs":W+2000000,"holdSinceMs":W,"resource":G,"standing":standing(W+1999000),"waiter":im(W+500000,{"graph.generation":"bbbbbbbbbbbb"}),"bound":[],"expected":{"state":"progressing"}},
 {"id":"not-stale-same-value","nowMs":W+2000000,"holdSinceMs":W,"resource":G,"standing":standing(W+1999000),"waiter":im(W+500000,{"graph.generation":"aaaaaaaaaaaa"}),"bound":["graph.generation"],"expected":{"state":"progressing"}},
 {"id":"stale-wins-over-stuck","nowMs":W+9000000,"holdSinceMs":W,"resource":G,"standing":standing(W+10000),"waiter":im(W+8000000,{"graph.generation":"bbbbbbbbbbbb","graph.record":"cccccccccccc"}),"bound":["graph.generation","graph.record"],"expected":{"state":"stale","staleIds":["graph.generation"]}},
]
eta=[
 {"id":"linear-from-counter","elapsedMs":900000,"phaseElapsedMs":600000,"done":100,"total":400,"historyMs":[],"expected":{"seconds":1800,"source":"progress","overdue":False}},
 {"id":"history-median-odd","elapsedMs":600000,"phaseElapsedMs":0,"historyMs":[3600000,4200000,3000000],"expected":{"seconds":3000,"source":"history","overdue":False}},
 {"id":"history-median-even","elapsedMs":1000000,"phaseElapsedMs":0,"historyMs":[1000000,3000000],"expected":{"seconds":1000,"source":"history","overdue":False}},
 {"id":"past-median-not-overdue-yet","elapsedMs":2200000,"phaseElapsedMs":0,"historyMs":[1000000,3000000],"expected":{"seconds":0,"source":"history","overdue":True}},
 {"id":"overdue","elapsedMs":9000000,"phaseElapsedMs":0,"historyMs":[1000000,3000000],"expected":{"seconds":0,"source":"history","overdue":True}},
 {"id":"too-few-samples","elapsedMs":1000,"phaseElapsedMs":0,"historyMs":[1000000],"expected":None},
 {"id":"counter-below-minimum-fraction-uses-history","elapsedMs":1000000,"phaseElapsedMs":50000,"done":1,"total":1000,"historyMs":[2000000,2000000],"expected":{"seconds":1000,"source":"history","overdue":False}},
 {"id":"counter-complete","elapsedMs":1000000,"phaseElapsedMs":50000,"done":10,"total":10,"historyMs":[],"expected":{"seconds":0,"source":"progress","overdue":False}},
 {"id":"no-data","elapsedMs":1000,"phaseElapsedMs":0,"historyMs":[],"expected":None},
]
prio=[
 {"id":"explicit-background","environment":{"SEMIO_LEASE_PRIORITY":"background"},"agentSession":True,"interactiveTerminal":False,"expected":"background"},
 {"id":"explicit-interactive-beats-agent-session","environment":{"SEMIO_LEASE_PRIORITY":"interactive"},"agentSession":True,"interactiveTerminal":False,"expected":"interactive"},
 {"id":"terminal-person","environment":{},"agentSession":False,"interactiveTerminal":True,"expected":"interactive"},
 {"id":"terminal-inside-agent-session","environment":{},"agentSession":True,"interactiveTerminal":True,"expected":"agent"},
 {"id":"headless","environment":{},"agentSession":False,"interactiveTerminal":False,"expected":"agent"},
 {"id":"unknown-value-ignored","environment":{"SEMIO_LEASE_PRIORITY":"urgent"},"agentSession":False,"interactiveTerminal":False,"expected":"agent"},
]
progress=[
 {"id":"graph-phase","resource":"build:graph-phase:inference:120:566","expected":{"phase":"inference","done":120,"total":566}},
 {"id":"graph-phase-with-detail","resource":"build:graph-phase:assembly:3:9:x,y","expected":{"phase":"assembly","done":3,"total":9}},
 {"id":"trailing-counter","resource":"dependencies:store-ownership:state:1234","expected":{"phase":"dependencies:store-ownership:state","done":1234}},
 {"id":"queue-notice-ignored","resource":"lease-queue:{\"version\":1}","expected":None},
 {"id":"sweep-ignored","resource":"lease-sweep:claims","expected":None},
 {"id":"plain-name","resource":"cargo:registry","expected":{"phase":"cargo:registry"}},
 {"id":"counter-without-digits","resource":"phase:x","expected":{"phase":"phase:x"}},
]
W2=W
req=lambda reason="stale",at=W2+100000,comps=None:{"version":1,"holderPid":4242,"holderStartedAtMs":W2,"requesterPid":9999,"requesterStartedAtMs":W2+500000,"resource":G,"reason":reason,"atMs":at,"requesterComponents":comps or {"graph.generation":"bbbbbbbbbbbb"}}
yc=lambda id,request,expected,**x:{"id":id,"request":request,"pid":4242,"startedAtMs":W2,"nowMs":W2+200000,"holdsExclusive":True,"holdSinceMs":W2,"standing":standing(W2+190000),"bound":["graph.generation"],"expected":expected,**x}
yields=[
 yc("stale-accepted",req(),{"accept":True,"ids":["graph.generation"]}),
 yc("wrong-pid",{**req(),"holderPid":1},{"accept":False,"refusal":"target"}),
 yc("wrong-start-time",{**req(),"holderStartedAtMs":W2+1},{"accept":False,"refusal":"target"}),
 yc("expired",req(at=W2-700000),{"accept":False,"refusal":"expired"}),
 yc("not-held-exclusively",req(),{"accept":False,"refusal":"not-held"},holdsExclusive=False),
 yc("same-component-value",req(comps={"graph.generation":"aaaaaaaaaaaa"}),{"accept":False,"refusal":"not-stale"}),
 yc("component-not-bound",req(),{"accept":False,"refusal":"not-stale"},bound=[]),
 yc("requester-older-than-holder",{**req(),"requesterStartedAtMs":W2-1},{"accept":False,"refusal":"not-stale"}),
 yc("stuck-accepted",req("stuck",at=W2+2900000),{"accept":True},nowMs=W2+3000000,standing=standing(W2+10000)),
 yc("stuck-claim-but-holder-progressing",req("stuck"),{"accept":False,"refusal":"not-stuck"}),
]
def tc(id,expected,**x):
    base={"mode":"terminate","state":"stuck","nowMs":W2+9000000,"requestAtMs":W2+8000000,"progressAtMs":W2+1000,"registeredStartedAtMs":W2,"osStartedAtMs":W2+500,"ticketArrivalMs":W2+100,"signalable":True,"self":False}
    base.update(x);base={k:v for k,v in base.items() if v is not None}
    return {"id":id,"input":base,"expected":expected}
terminate=[
 tc("stuck-after-grace-and-identity",{"terminate":True}),
 tc("report-mode-never",{"terminate":False},mode="report"),
 tc("request-mode-never",{"terminate":False},mode="request"),
 tc("progressing-never",{"terminate":False},state="progressing"),
 tc("stale-is-not-stuck",{"terminate":False},state="stale"),
 tc("inside-grace",{"terminate":False,"refusal":"grace"},requestAtMs=W2+8990000),
 tc("no-request-yet",{"terminate":False,"refusal":"grace"},requestAtMs=None),
 tc("progressed-since-request",{"terminate":False,"refusal":"progressed"},progressAtMs=W2+8500000),
 tc("start-time-unknown",{"terminate":False,"refusal":"unknown"},osStartedAtMs=None),
 tc("pid-reused-identity-differs",{"terminate":False,"refusal":"identity"},osStartedAtMs=W2+60000),
 tc("another-user",{"terminate":False,"refusal":"owner"},signalable=False),
 tc("never-itself",{"terminate":False},self=True),
 tc("legacy-too-young",{"terminate":False},state="legacy",nowMs=W2+3600000,requestAtMs=None,registeredStartedAtMs=None),
 tc("legacy-old-and-older-than-ticket",{"terminate":True},state="legacy",nowMs=W2+6000000,requestAtMs=None,registeredStartedAtMs=None,osStartedAtMs=W2),
 tc("legacy-process-started-after-ticket",{"terminate":False,"refusal":"identity"},state="legacy",nowMs=W2+6000000,requestAtMs=None,registeredStartedAtMs=None,osStartedAtMs=W2+900000),
]
json.dump({"version":1,"yieldCases":yields,"terminateCases":terminate,"progressCases":progress,"priorityCases":prio,"simulations":sims,"stateCases":states,"etaCases":eta},open(out,"w"),indent=1,ensure_ascii=False)
open(out,"a").write("\n")
print(len(sims),"simulations",sum(len(s["jobs"]) for s in sims),"jobs")
