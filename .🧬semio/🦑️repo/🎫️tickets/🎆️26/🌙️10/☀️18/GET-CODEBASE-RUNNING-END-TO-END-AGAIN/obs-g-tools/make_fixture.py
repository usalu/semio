#!/usr/bin/env python3
"""[obs-g] writes the golden corpus of the fleet status module: hand-built scenarios whose key numbers are asserted by hand in the test, plus seeded cases, each with the report the independent oracle computes. usage: make_fixture.py <fleet-status dir>"""
import importlib.util,json,sys,os
D=sys.argv[1].rstrip("/")
spec=importlib.util.spec_from_file_location("oracle",D+"/🧪️tests/🐍️.py");oracle=importlib.util.module_from_spec(spec);spec.loader.exec_module(oracle)
AT=1_793_000_000_000
def src(failed=()): return [{"id":i,"ok":i not in failed,"milliseconds":10,**({"error":"boom"} if i in failed else {})} for i in ["leases","capacity","bazel","storage","sealed","generation"]]
def facts(**kw):
    base={"version":1,"atMs":AT,"workspace":"/w","sources":src()}
    base.update(kw);return base
HOST={"platform":"darwin","cores":10,"totalMegabytes":16384,"availableMegabytes":6000,"pressure":"normal","swapUsedMegabytes":0,"load":2.5}
def holder(pid,resource,mode="exclusive",held=100,**extra):
    h={"pid":pid,"resource":resource,"mode":mode,"class":"graph","heldSeconds":held};h.update(extra);return h
def waiter(pid,resource,position,waited=60,blockers=("x",),**extra):
    w={"pid":pid,"resource":resource,"mode":"exclusive","class":"graph","waitedSeconds":waited,"blockers":list(blockers),"position":position};w.update(extra);return w
cases=[]
def add(id_,language,f): cases.append({"id":id_,"language":language,"facts":f,"report":oracle.summarize(f,language)})
add("empty","en",facts(sources=src(failed=["leases","capacity","bazel","storage","sealed","generation"])))
st={"state":"progressing","priority":"agent","implementationDigest":"d1"*8,"implementationAgeSeconds":30}
add("join-grant-and-children","en",facts(host=HOST,leases={"holders":[
  holder(101,"build-graph-state:/w:selected:7",held=500,command="bun repo run x",agent="a1",standing=dict(st,phase="inference",done=5,total=10,etaMinutes=3,parentPid=100)),
  holder(102,"graph-claim:/w:t",mode="shared",held=100,standing=dict(st,parentPid=101,implementationDigest="e2"*8,implementationAgeSeconds=900)),
  holder(103,"nx:daemon-slot:1",held=50,standing=dict(st,chain=[7,102]))],"waiters":[],"cycles":[]},
 capacity={"floorMegabytes":2048,"headroomMegabytes":3000,"capacityMegabytes":14336,"grants":[{"id":"g1","operation":"run","pid":100,"jobs":2,"reservedMegabytes":3000,"observedMegabytes":2000,"heldSeconds":600,"agent":"a1"}],"waiters":[]}))
add("lease-queue-estimate","de",facts(host=HOST,leases={"holders":[holder(10,"R",held=900,standing=dict(st,phase="x",etaMinutes=10))],"waiters":[
  waiter(11,"R",1,blockers=["10/1"],historyKey="k1"),waiter(12,"R",2,blockers=["10/1","11/q"],historyKey="k2"),waiter(13,"R",3,blockers=["10/1"],historyKey="k3"),waiter(14,"S",1,blockers=[])],"cycles":[]},
 history={"k1":[100000,140000],"k2":[60000],"k3":[1000,3000]}))
add("capacity-estimate","en",facts(host=HOST,capacity={"floorMegabytes":2048,"headroomMegabytes":1000,"capacityMegabytes":14336,"grants":[
  {"id":"G1","operation":"run","pid":1,"jobs":1,"reservedMegabytes":4000,"observedMegabytes":3000,"heldSeconds":100},
  {"id":"G2","operation":"cargo","pid":2,"jobs":1,"reservedMegabytes":2000,"observedMegabytes":None,"heldSeconds":50}],"waiters":[
  {"id":"A","operation":"run","pid":3,"needMegabytes":3000,"waitedSeconds":30,"reason":"memory","reasonText":"not enough free memory","position":1},
  {"id":"B","operation":"show","pid":4,"needMegabytes":3500,"waitedSeconds":20,"reason":"queue-order","reasonText":"an earlier request is still waiting","position":2},
  {"id":"C","operation":"run","pid":5,"needMegabytes":20000,"waitedSeconds":10,"reason":"queue-order","reasonText":"an earlier request is still waiting","position":3}]},
 activeMilliseconds={"run":[600000,600000],"cargo":[100000,100000]}))
add("graph-custody","en",facts(host=HOST,leases={"holders":[
  holder(21,"build-graph-state:/w:selected:7",held=700,standing=dict(st,phase="inference",done=55,total=117,etaMinutes=12)),
  holder(22,"repository-graph-facts:/w:shard:7",mode="shared",held=650),holder(23,"graph-claim:/w:a",mode="shared",held=40),holder(24,"graph-claim:/w:b",mode="shared",held=30),holder(23,"graph-claim-slot:/w:0",mode="shared",held=40)],
  "waiters":[waiter(31,"build-graph-state:/w:selected:7",1,waited=420,blockers=["21/1"]),waiter(32,"build-graph-state:/w:selected:7",2,waited=300,blockers=["21/1"])],"cycles":[]}))
add("bazel-slots","de",facts(host=HOST,leases={"holders":[holder(40,"bazel-slot-s2",held=60)],"waiters":[],"cycles":[]},bazel={"slotPrefix":"bazel-slot-","slots":[
  {"slot":"s10","pid":None,"treeMegabytes":None},{"slot":"s2","pid":900,"treeMegabytes":420,"key":"ab"*8,"usedAt":AT-1000,"agent":"a1"},{"slot":"s1","pid":901,"treeMegabytes":200},{"slot":"d0","pid":None,"treeMegabytes":None}]}))
GI=2**30
add("storage-alerts","en",facts(host=HOST,storage={"hostFreeBytes":10*GI,"hostFreeFloorBytes":15*GI,"images":[
  {"id":"rust","kind":"partition","state":"mounted","usedBytes":46*GI,"capacityBytes":48*GI,"budgetBytes":48*GI,"source":"statfs"},{"id":"tools","kind":"partition","state":"mounted","usedBytes":3*GI,"capacityBytes":8*GI,"budgetBytes":None,"source":"statfs"},{"id":"control","kind":"control","state":"directory","usedBytes":None,"capacityBytes":None,"budgetBytes":None,"source":"unknown"}],"overAreas":["go","playwright"],
  "machine":{"totalBytes":40*GI,"effectiveTotalBytes":40*GI,"usedBytes":73*GI,"footprintBytes":97*GI,"overBytes":33*GI,"evictBytes":20*GI,"blockedBytes":0,"source":"default"}}))
add("sealed-blocked-and-drifted","de",facts(host=HOST,sealed={"mode":"sealed","entries":[
  {"entry":"🏛️sealed/🚪️entry","digest":"ab"*32,"files":1520,"bytes":18626484,"promotedAtMs":AT-7200000,"fresh":False,"changed":76,"blocked":{"atMs":AT-60000,"rows":2,"first":"a/b.ts:43:23 Unexpected ;"},"last":{"atMs":AT-7200000,"added":4,"removed":0,"changed":11}},
  {"entry":"🏛️sealed/📤️other","digest":"cd"*32,"files":10,"bytes":1000,"promotedAtMs":AT-100000,"fresh":True,"changed":0}]}))
add("stuck-and-stale-and-pressure","en",facts(host=dict(HOST,pressure="critical"),leases={"holders":[
  holder(50,"build-graph-state:/w:selected:7",held=4000,standing={"state":"stuck","priority":"agent","silentSeconds":1500,"etaOverdue":True}),
  holder(51,"nx:daemon-slot:1",held=300,standing={"state":"stale","staleIds":["format-stamps","graph"],"priority":"interactive"})],"waiters":[waiter(60,"build-graph-state:/w:selected:7",1,waited=1000,priority="interactive")],"cycles":["50/q → 60/q"]},
 capacity={"floorMegabytes":2048,"headroomMegabytes":-500,"capacityMegabytes":14336,"grants":[{"id":"g","operation":"run","pid":70,"jobs":1,"reservedMegabytes":1000,"observedMegabytes":2500,"heldSeconds":30}],"waiters":[{"id":"w","operation":"run","pid":71,"needMegabytes":3000,"waitedSeconds":900,"reason":"memory","reasonText":"x","position":1}]}))
add("generation-coverage","de",facts(host=HOST,generation={"number":566,"built":{"kind":"delta","dirty":4},"owners":4019,"implementation":"1f"*32,"publishedAtMs":AT-840000,"verified":{"atMs":AT-900000,"covered":66,"count":4019}}))
add("boundaries","en",facts(host=HOST,leases={"holders":[],"waiters":[waiter(80,"A",1,waited=900),waiter(81,"B",1,waited=899)],"cycles":[]},
 capacity={"floorMegabytes":2048,"headroomMegabytes":0,"capacityMegabytes":14336,"grants":[
  {"id":"u1","operation":"run","pid":82,"jobs":1,"reservedMegabytes":1000,"observedMegabytes":1500,"heldSeconds":1},
  {"id":"u2","operation":"run","pid":83,"jobs":1,"reservedMegabytes":1000,"observedMegabytes":1501,"heldSeconds":1}],"waiters":[
  {"id":"c1","operation":"run","pid":84,"needMegabytes":1,"waitedSeconds":600,"reason":"memory","reasonText":"x","position":1},
  {"id":"c2","operation":"run","pid":85,"needMegabytes":1,"waitedSeconds":599,"reason":"memory","reasonText":"x","position":2}]},
 storage={"hostFreeBytes":15*GI,"hostFreeFloorBytes":15*GI,"images":[
  {"id":"i1","kind":"partition","state":"mounted","usedBytes":9000,"capacityBytes":10000,"budgetBytes":None,"source":"statfs"},
  {"id":"i2","kind":"partition","state":"mounted","usedBytes":8999,"capacityBytes":10000,"budgetBytes":None,"source":"statfs"}],"overAreas":[],"machine":{"totalBytes":GI,"effectiveTotalBytes":GI,"usedBytes":1,"footprintBytes":1,"overBytes":0,"evictBytes":0,"blockedBytes":0,"source":"file"}}))
for seed in (3,4):
    for row in oracle.generate(seed,3): cases.append(row)
json.dump({"version":1,"cases":cases},open(D+"/🧪️tests/🧫️fixtures/🔣️.json","w",encoding="utf8"),ensure_ascii=False,indent=1);open(D+"/🧪️tests/🧫️fixtures/🔣️.json","a").write("\n")
print(len(cases),"cases")
