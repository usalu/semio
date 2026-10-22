import sys,os,json
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
T=L+"/🏗️build/🧠️memory/🧪️tests/"
fx=T+"🔣️.json"
d=json.load(open(fx,encoding="utf8"))
names={g["name"] for g in d["governor"]}
if "baseline-is-not-charged" not in names:
    d["governor"].append({"name":"baseline-is-not-charged","workers":4,"budgetMegabytes":300,"baseMegabytes":4000,"baselineMegabytes":4000,"garbageShare":0.0,"expectParallel":4,"expectWaits":False,"tasks":[{"memoryMegabytes":20,"durationMilliseconds":200} for _ in range(8)]})
    d["governor"].append({"name":"growth-over-baseline-is-charged","workers":4,"budgetMegabytes":100,"baseMegabytes":4000,"baselineMegabytes":4000,"garbageShare":0.0,"expectWaits":True,"tasks":[{"memoryMegabytes":60,"durationMilliseconds":200} for _ in range(6)]})
    open(fx,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=1)+"\n")
    print("fixture written")
sc=T+"🧬️schema/🔣️.json"
s=json.load(open(sc,encoding="utf8"))
props=s["properties"]["governor"]["items"]["properties"]
if "baselineMegabytes" not in props:
    new={}
    for k,v in props.items():
        new[k]=v
        if k=="baseMegabytes": new["baselineMegabytes"]={"type":"integer","minimum":0}
    s["properties"]["governor"]["items"]["properties"]=new
    raw=open(sc,encoding="utf8").read()
    ind=1 if json.dumps(json.loads(raw),ensure_ascii=False,indent=1)+"\n"==raw else 2
    open(sc,"w",encoding="utf8").write(json.dumps(s,ensure_ascii=False,indent=ind)+"\n")
    print("schema written",ind)
apply(T+"🐍️oracle.py",[
 ('    budget_bytes = scenario["budgetMegabytes"] * MEGABYTE\n','    budget_bytes = scenario["budgetMegabytes"] * MEGABYTE\n    baseline_bytes = scenario.get("baselineMegabytes", 0) * MEGABYTE\n'),
 ('        if row["runningBefore"] > 0 and row["usedBytes"] > budget_bytes:','        if row["runningBefore"] > 0 and row["usedBytes"] - baseline_bytes > budget_bytes:'),
])
apply(T+"🟦️.ts",[
 ('const governor=memory.createMemoryGovernorV1({budgetBytes:scenario.budgetMegabytes*MEGABYTE,workers:scenario.workers,sample,','const governor=memory.createMemoryGovernorV1({budgetBytes:scenario.budgetMegabytes*MEGABYTE,baselineBytes:((scenario as {baselineMegabytes?:number}).baselineMegabytes??0)*MEGABYTE,workers:scenario.workers,sample,'),
 ('''test("the governor refuses a missing worker count or budget and rejects every waiter on abort",async()=>{''','''test("the default sample reads constant-time counters, never a heap walk, and the baseline is what survives one collection",()=>{
 const ports=memory.createMemoryPortsV1(),retained:object[]=[];
 for(let index=0;index<400000;index++)retained.push({index,text:"x".repeat(12)+index,list:[index,index+1,{inner:index}]});
 const started=performance.now();let last=0;for(let round=0;round<200;round++)last=ports.sample();
 const perCall=(performance.now()-started)/200,counters=process.memoryUsage();
 expect(perCall).toBeLessThan(5);
 expect(Math.abs(last-(counters.heapUsed+counters.external))/last).toBeLessThan(0.25);
 expect(retained.length).toBe(400000);
 const order:string[]=[];
 expect(memory.measureMemoryBaselineV1({collect:()=>{order.push("collect");},sample:()=>{order.push("sample");return 7;}})).toBe(7);
 expect(order).toEqual(["collect","sample"]);
});

test("the governor refuses a missing worker count or budget and rejects every waiter on abort",async()=>{
 expect(()=>memory.createMemoryGovernorV1({budgetBytes:MEGABYTE,workers:1,baselineBytes:-1})).toThrow();'''),
])
