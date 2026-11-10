import json,sys,importlib.util,os
S="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗓️scheduler/🧪️tests"
spec=importlib.util.spec_from_file_location("oracle",S+"/🐍️.py");o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
def j(t,agent,**k):
    d=dict(ticket=t,at=0,agent=agent,durationMs=1000,priority="agent",depth=0,key=t,peakMb=512);d.update(k);return d
base={"heartbeatTimeoutMs":10**9}
sc=[
 dict(id="shared-crate-first",policy={**base,"maxConcurrent":1},budgetMb=8192,jobs=[j("leaf","a"),j("mid","a",depth=2),j("core","a",depth=5)]),
 dict(id="two-agents-round-robin",policy={**base,"maxConcurrent":1,"maxPerAgent":1},budgetMb=8192,jobs=[j("a%d"%i,"a") for i in range(5)]+[j("b%d"%i,"b") for i in range(2)]),
 dict(id="dedupe-same-target",policy={**base,"maxConcurrent":4},budgetMb=8192,jobs=[j("x1","a",key="core:build"),j("x2","b",key="core:build",at=10),j("x3","c",key="core:build",at=20),j("y1","c")]),
 dict(id="aging-lifts-background",policy={**base,"maxConcurrent":1,"agingMs":1000},budgetMb=8192,jobs=[j("old","z",priority="background")]+[j("i%d"%n,"w%d"%n,at=n*200,priority="interactive") for n in range(10)]),
 dict(id="backfill-and-drain",policy={**base,"agingMs":100},budgetMb=4000,jobs=[j("run","a",peakMb=3000,durationMs=5000),j("big","b",peakMb=3000,at=100),j("small","c",peakMb=500,at=200)]),
 dict(id="oversized-runs-alone",policy=base,budgetMb=4096,jobs=[j("huge","a",peakMb=20000),j("next","b",at=10)]),
 dict(id="compiler-lanes",policy={**base,"sharedCompilerLanes":{"rustc":1,"tsc":2}},budgetMb=99999,jobs=[j("r%d"%i,"a%d"%i,lane="rustc") for i in range(3)]+[j("t%d"%i,"a%d"%i,lane="tsc") for i in range(3)]),
 dict(id="cancel-queued-and-running",policy={**base,"maxConcurrent":1},budgetMb=8192,jobs=[j("c1","a",cancelAt=500),j("c2","b",cancelAt=200),j("c3","c")]),
 dict(id="mixed-agents",policy={**base,"maxConcurrent":3,"maxPerAgent":2},budgetMb=6000,jobs=[j("m%d"%i,"abcd"[i%4],at=(i//4)*300,depth=(i*5)%7,priority=["agent","interactive","background"][i%3],peakMb=[256,1024,2048][i%3],key="m%d"%(i%9)) for i in range(16)]),
 dict(id="idle-agent-pruned-then-returns",policy={**base,"maxConcurrent":1,"agingMs":100},budgetMb=8192,jobs=[j("p1","a"),j("p2","b",at=100),j("p3","a",at=5000),j("p4","b",at=5000)]),
]
for s in sc: s["starts"]=o.simulate(s)
json.dump({"version":1,"scenarios":sc},open(S+"/🧫️fixtures/🔣️.json","w",encoding="utf8"),ensure_ascii=False,indent=1);open(S+"/🧫️fixtures/🔣️.json","a").write("\n")
for s in sc: print(s["id"],s["starts"][:8])
