import sys,os,json
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
M=L+"/🏗️build/🧠️memory/"
apply(M+"🟨️.mjs",[
 ('import v8 from "node:v8";\n','import v8 from "node:v8";\n') if False else ('import schema from','import{totalmem}from"node:os";\nimport schema from'),
 ('''/** 📏️ The managed memory budget of one graph construction in bytes: the explicit environment value, else a share of the host capacity grant the process inherited, else the declared default; never below the declared minimum and never above the declared default. */
export function graphMemoryBudgetBytesV1(environment,policy=P){''','''/** 🖥️ The memory of the host in megabytes as this process may use it (the container limit when one applies), 0 when unknown. */
export function hostMemoryMegabytesV1(){
 const total=totalmem(),limit=typeof process.constrainedMemory==="function"?process.constrainedMemory():0;
 const bytes=Number.isFinite(limit)&&limit>0&&limit<total?limit:total;
 return Number.isFinite(bytes)&&bytes>0?Math.floor(bytes/MEGABYTE):0;
}

/** 📏️ The managed memory budget of one graph construction in bytes: the explicit environment value, else derived from the machine: the larger of the declared share of the host memory and the declared share of the host capacity grant the process inherited (fixed plus per job megabytes), within the declared minimum and maximum. Nothing is a constant of one machine. */
export function graphMemoryBudgetBytesV1(environment,policy=P,hostMegabytes=hostMemoryMegabytesV1()){'''),
 (''' let granted;try{granted=JSON.parse(environment[policy.planEnvironment]??"null")?.memoryMegabytes;}catch{}
 const share=typeof granted==="number"&&Number.isFinite(granted)&&granted>0?Math.floor(granted*policy.grantShare):policy.defaultBudgetMegabytes;
 return Math.max(policy.minimumBudgetMegabytes,Math.min(policy.defaultBudgetMegabytes,share))*MEGABYTE;''',''' let plan;try{plan=JSON.parse(environment[policy.planEnvironment]??"null");}catch{}
 const part=value=>typeof value==="number"&&Number.isFinite(value)&&value>0?value:0,granted=part(plan?.memoryMegabytes)+part(plan?.fixedMegabytes);
 const derived=Math.max(Math.floor(part(hostMegabytes)*policy.hostShare),Math.floor(granted*policy.grantShare));
 return Math.max(policy.minimumBudgetMegabytes,Math.min(policy.maximumBudgetMegabytes,derived))*MEGABYTE;'''),
])
apply(M+"🟨️.d.mts",[
 ('export function graphMemoryBudgetBytesV1(environment:Readonly<Record<string,string|undefined>>,policy?:GraphMemoryPolicyV1):number;','export function hostMemoryMegabytesV1():number;\nexport function graphMemoryBudgetBytesV1(environment:Readonly<Record<string,string|undefined>>,policy?:GraphMemoryPolicyV1,hostMegabytes?:number):number;'),
 ('readonly defaultBudgetMegabytes:number;readonly minimumBudgetMegabytes:number;readonly grantShare:number;','readonly minimumBudgetMegabytes:number;readonly maximumBudgetMegabytes:number;readonly hostShare:number;readonly grantShare:number;'),
])
def rewrite_policy(p):
    raw=open(p,encoding="utf8").read(); d=json.loads(raw)
    return raw,d
# schema Policy
sp=M+"🧬️schema/🔣️.json"
raw,d=rewrite_policy(sp)
pol=d["definitions"]["Policy"]["const"]
if "defaultBudgetMegabytes" in pol:
    new={}
    for k,v in pol.items():
        if k=="defaultBudgetMegabytes": continue
        new[k]=v
        if k=="minimumBudgetMegabytes": new["maximumBudgetMegabytes"]=16384; new["hostShare"]=0.25
    d["definitions"]["Policy"]["const"]=new
    for cand in (1,2):
        if json.dumps(json.loads(raw),ensure_ascii=False,indent=cand)+"\n"==raw: ind=cand;break
    else: ind=None
    assert ind is not None, "schema format"
    open(sp,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=ind)+"\n"); print("schema policy written")
    # if Policy has a schema of its own listing properties
    print([k for k in d["definitions"] if k!="Policy"])
# fixture
fx=M+"🧪️tests/🔣️.json"
f=json.load(open(fx,encoding="utf8"))
f["policy"]=d["definitions"]["Policy"]["const"]
def row(name,env,host,expect): return {"name":name,"environment":env,"hostMegabytes":host,"expect":expect}
MB=1048576
plan=lambda **k: json.dumps(k)
f["budgets"]=[
 row("host-share-of-a-large-host",{},32768,{"bytes":8192*MB}),
 row("host-share-of-a-small-host",{},8192,{"bytes":2048*MB}),
 row("grant-share-wins-on-a-small-host",{"SEMIO_HOST_CAPACITY_PLAN":plan(jobs=1,fixedMegabytes=5120,memoryMegabytes=0)},8192,{"bytes":2560*MB}),
 row("grant-adds-fixed-and-per-job-megabytes",{"SEMIO_HOST_CAPACITY_PLAN":plan(jobs=2,fixedMegabytes=3072,memoryMegabytes=3072)},8192,{"bytes":3072*MB}),
 row("host-share-beats-the-small-show-grant",{"SEMIO_HOST_CAPACITY_PLAN":plan(jobs=1,fixedMegabytes=5120,memoryMegabytes=0)},32768,{"bytes":8192*MB}),
 row("never-below-the-minimum",{},2048,{"bytes":1024*MB}),
 row("never-above-the-maximum",{},262144,{"bytes":16384*MB}),
 row("unknown-host-without-grant-is-the-minimum",{},0,{"bytes":1024*MB}),
 row("grant-not-json-uses-the-host",{"SEMIO_HOST_CAPACITY_PLAN":"{"},16384,{"bytes":4096*MB}),
 row("grant-with-negative-and-text-megabytes-is-ignored",{"SEMIO_HOST_CAPACITY_PLAN":plan(fixedMegabytes=-5,memoryMegabytes="lots")},16384,{"bytes":4096*MB}),
 row("explicit-megabytes",{"SEMIO_GRAPH_MEMORY_MB":"2048"},32768,{"bytes":2048*MB}),
 row("explicit-minimum",{"SEMIO_GRAPH_MEMORY_MB":"1024"},32768,{"bytes":1024*MB}),
 row("explicit-below-minimum-refused",{"SEMIO_GRAPH_MEMORY_MB":"512"},32768,{"error":True}),
 row("explicit-not-a-number-refused",{"SEMIO_GRAPH_MEMORY_MB":"plenty"},32768,{"error":True}),
 row("explicit-empty-refused",{"SEMIO_GRAPH_MEMORY_MB":""},32768,{"error":True}),
 row("explicit-beats-the-grant-and-the-host",{"SEMIO_GRAPH_MEMORY_MB":"4096","SEMIO_HOST_CAPACITY_PLAN":plan(memoryMegabytes=2048)},32768,{"bytes":4096*MB}),
]
open(fx,"w",encoding="utf8").write(json.dumps(f,ensure_ascii=False,indent=1)+"\n")
ts=M+"🧪️tests/🧬️schema/🔣️.json"
t=json.load(open(ts,encoding="utf8"))
it=t["properties"]["budgets"]["items"]
if "hostMegabytes" not in it["properties"]:
    it["required"]=["name","environment","hostMegabytes","expect"]
    it["properties"]={"name":it["properties"]["name"],"environment":it["properties"]["environment"],"hostMegabytes":{"type":"integer","minimum":0},"expect":it["properties"]["expect"]}
    raw=open(ts,encoding="utf8").read()
    ind=1 if json.dumps(json.loads(raw),ensure_ascii=False,indent=1)+"\n"==raw else 2
    open(ts,"w",encoding="utf8").write(json.dumps(t,ensure_ascii=False,indent=ind)+"\n")
apply(M+"🧪️tests/🐍️oracle.py",[
 ('def budget(policy, environment):','def budget(policy, environment, host):'),
 ('''    granted = None
    try:
        plan = json.loads(environment.get(policy["planEnvironment"], "null"))
        granted = plan.get("memoryMegabytes") if isinstance(plan, dict) else None
    except ValueError:
        granted = None
    if isinstance(granted, (int, float)) and not isinstance(granted, bool) and math.isfinite(granted) and granted > 0:
        share = math.floor(granted * policy["grantShare"])
    else:
        share = policy["defaultBudgetMegabytes"]
    return {"bytes": max(policy["minimumBudgetMegabytes"], min(policy["defaultBudgetMegabytes"], share)) * MEGABYTE}''','''    try:
        plan = json.loads(environment.get(policy["planEnvironment"], "null"))
    except ValueError:
        plan = None

    def part(value):
        return value if isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value) and value > 0 else 0
    granted = part(plan.get("memoryMegabytes")) + part(plan.get("fixedMegabytes")) if isinstance(plan, dict) else 0
    derived = max(math.floor(part(host) * policy["hostShare"]), math.floor(granted * policy["grantShare"]))
    return {"bytes": max(policy["minimumBudgetMegabytes"], min(policy["maximumBudgetMegabytes"], derived)) * MEGABYTE}'''),
 ('budget(policy, row["environment"]) for row in fixture["budgets"]','budget(policy, row["environment"], row["hostMegabytes"]) for row in fixture["budgets"]'),
])
apply(M+"🧪️tests/🟦️.ts",[
 ('const run=()=>memory.graphMemoryBudgetBytesV1(row.environment);','const run=()=>memory.graphMemoryBudgetBytesV1(row.environment,memory.GRAPH_MEMORY_POLICY_V1,row.hostMegabytes);'),
])
