import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT=sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else None
if ROOT is None:
    p=HERE
    while p!=os.path.dirname(p):
        if os.path.isfile(os.path.join(p,"bunfig.toml")) and os.path.isfile(os.path.join(p,"bun.lock")):ROOT=p;break
        p=os.path.dirname(p)
PR="🧰️framework/🔨️modules/🏃️process/"
P=PR+"🔒️leases/"
O=P+"🧭️order/"
A=P+"🚪️admission/"
def rd(rel):return open(os.path.join(ROOT,rel),encoding="utf8").read()
def once(t,o,n):
    assert t.count(o)==1,"anchor: "+o[:90]
    return t.replace(o,n)
out={}
# 1-2: explicit option + pure sweep decision
l=rd(P+"🟦️.ts")
if "claimSweepDueV1" not in l:
    l=once(l,"  readonly maximumDatabaseBytes?: number;\n};\n/** 🗃️ Admits the actual database","  readonly maximumDatabaseBytes?: number;\n  /** ⏳️ How long a request that must not wait for its resource (timeout 0) still waits for the directory admission gate, which is held only for microseconds. Only callers that wait for the gate as part of the queue protocol pass it; a real try-lock passes nothing and stays a single attempt. */\n  readonly admissionPatienceMs?: number;\n};\n/** 🗃️ Admits the actual database")
    l=once(l,"patience=!cleanup&&options.timeoutMs===0?leaseOrderPolicyV1().gate.admissionPatienceMilliseconds??0:0;","patience=!cleanup&&options.timeoutMs===0?options.admissionPatienceMs??0:0;")
    l=once(l,'  const catalog=await queueClaimCatalog(pendingOptions(),"shared");let claim:ResourceLease;','  const patience=leaseOrderPolicyV1().gate.admissionPatienceMilliseconds;\n  const catalog=await queueClaimCatalog({...pendingOptions(),admissionPatienceMs:patience},"shared");let claim:ResourceLease;')
    l=once(l,"claim=await acquireResourceLease({...pendingOptions(),...resourceQueueClaimV1(","claim=await acquireResourceLease({...pendingOptions(),admissionPatienceMs:patience,...resourceQueueClaimV1(")
    l=once(l,"const lease = await acquireResourceLeaseBody({ ...options, timeoutMs:","const lease = await acquireResourceLeaseBody({ ...options, admissionPatienceMs: leaseOrderPolicyV1().gate.admissionPatienceMilliseconds, timeoutMs:")
    l=once(l,"  if(options.control&&claimSweepDue(resolve(options.directory)))await retireResourceQueueClaimsV1","  if(options.control&&claimSweepDueV1(lastClaimSweep,resolve(options.directory),Date.now(),leaseOrderPolicyV1().gate.claimSweepIntervalMilliseconds,options.timeoutMs))await retireResourceQueueClaimsV1")
    a=l.index("/** 🧹️ Whether this process may start another retirement sweep");b=l.index("/** 🎟️ Whether an exclusive request is queued")
    l=l[:a]+'''/** 🧹️ Whether this process may start another retirement sweep of settled queue claims. The sweep takes the directory admission gate exclusively for a moment, so it never runs for a caller that must not wait (timeout 0) and runs at most once per `gate.claimSweepIntervalMilliseconds` per directory; a skipped caller does not move the clock. */
export function claimSweepDueV1(sweeps:Map<string,number>,directory:string,nowMilliseconds:number,intervalMilliseconds:number,timeoutMs:number|undefined):boolean{
  if(timeoutMs===0)return false;
  const last=sweeps.get(directory);
  if(intervalMilliseconds>0&&last!==undefined&&nowMilliseconds-last<intervalMilliseconds)return false;
  sweeps.set(directory,nowMilliseconds);return true;
}
'''+l[b:]
    out[P+"🟦️.ts"]=l
# 3: required policy fields
s=json.loads(rd(O+"🧬️schema/🔣️.json"));g=s["definitions"]["Policy"]["properties"]["gate"]
if "admissionPatienceMilliseconds" not in g["required"]:
    g["required"]+=["admissionPatienceMilliseconds","claimSweepIntervalMilliseconds"]
    out[O+"🧬️schema/🔣️.json"]=json.dumps(s,ensure_ascii=False,indent=2)+"\n"
m=rd(O+"🟦️.ts")
if "admissionPatienceMilliseconds?:number" in m:
    m=once(m,"admissionPatienceMilliseconds?:number;claimSweepIntervalMilliseconds?:number","admissionPatienceMilliseconds:number;claimSweepIntervalMilliseconds:number")
    out[O+"🟦️.ts"]=m
fx=json.loads(rd(O+"🧪️tests/🧫️fixtures/🔣️.json"))
if "admissionPatienceMilliseconds" not in fx["policy"]["gate"]:
    fx["policy"]["gate"].update({"admissionPatienceMilliseconds":2000,"claimSweepIntervalMilliseconds":0})
    out[O+"🧪️tests/🧫️fixtures/🔣️.json"]=json.dumps(fx,ensure_ascii=False,indent=1)+"\n"
# processes test
pt=rd(O+"🧪️tests/📥️processes/🟦️.ts")
if "claimSweepIntervalMilliseconds:0}})" not in pt.split("probe=async")[1][:300]:
    pt=once(pt,"scenario({gate:{releaseWhileWaiting,pollInitialMilliseconds:20,pollMaximumMilliseconds:40}})","scenario({gate:{releaseWhileWaiting,pollInitialMilliseconds:20,pollMaximumMilliseconds:40,admissionPatienceMilliseconds:0,claimSweepIntervalMilliseconds:0}})")
if 'test("a request that must not wait for its resource still waits' in pt:
    a=pt.index('test("a request that must not wait for its resource still waits');b=pt.index('test("a request that must not wait still fails promptly')
    pt=pt[:a]+'''test("a queued request that must not wait for its resource still waits for a busy admission gate; a raw try-lock stays a single attempt; patience 0 is the negative control",async()=>{
 for(const patient of [true,false]){
  const {directory,env}=scenario({gate:{releaseWhileWaiting:true,pollInitialMilliseconds:5,pollMaximumMilliseconds:40,admissionPatienceMilliseconds:patient?3000:0,claimSweepIntervalMilliseconds:0}});configure(env);
  (await acquireResourceLease({directory,resource:"free:warm",mode:"shared",signal:signal()})).release();
  for(const queued of [false,true]){
   const gate=await holdGate(directory,800),release=setTimeout(()=>gate.release(),800),started=Date.now();
   const options={directory,resource:"free:try",mode:"exclusive" as const,signal:signal(),timeoutMs:0};
   const outcome=await (queued?acquireQueuedResourceLease({...options,owner:"try"+counter}):acquireResourceLease(options)).then(lease=>{lease.release();return "granted";},error=>String(error));
   clearTimeout(release);try{gate.release();}catch{}
   if(patient&&queued){expect(outcome).toBe("granted");expect(Date.now()-started).toBeGreaterThanOrEqual(600);}
   else{expect(outcome).toContain("timed out");expect(Date.now()-started).toBeLessThan(700);}
  }
 }
});

'''+pt[b:]
    out[O+"🧪️tests/📥️processes/🟦️.ts"]=pt
# new corpus, schema, test
corpus={"version":1,
 "sweep":[
  {"id":"first-call-sweeps-then-skips-inside-the-interval","intervalMilliseconds":1000,"calls":[{"atMs":0,"timeoutMs":None,"expect":True},{"atMs":999,"timeoutMs":None,"expect":False},{"atMs":1000,"timeoutMs":None,"expect":True}]},
  {"id":"a-caller-that-must-not-wait-never-sweeps-and-never-moves-the-clock","intervalMilliseconds":1000,"calls":[{"atMs":0,"timeoutMs":0,"expect":False},{"atMs":1,"timeoutMs":None,"expect":True},{"atMs":2,"timeoutMs":0,"expect":False},{"atMs":500,"timeoutMs":None,"expect":False},{"atMs":1001,"timeoutMs":None,"expect":True}]},
  {"id":"interval-zero-sweeps-every-time-a-caller-may-wait","intervalMilliseconds":0,"calls":[{"atMs":0,"timeoutMs":None,"expect":True},{"atMs":0,"timeoutMs":None,"expect":True},{"atMs":0,"timeoutMs":0,"expect":False}]},
  {"id":"a-finite-timeout-may-sweep","intervalMilliseconds":1000,"calls":[{"atMs":0,"timeoutMs":5000,"expect":True},{"atMs":10,"timeoutMs":5000,"expect":False}]}],
 "gate":[
  {"id":"raw-try-lock-is-not-stalled-by-a-busy-gate","api":"raw","patienceMilliseconds":3000,"gateHeldMilliseconds":1200,"expect":{"outcome":"timed-out","minElapsedMilliseconds":0,"maxElapsedMilliseconds":900}},
  {"id":"raw-try-lock-with-patience-zero-is-the-same","api":"raw","patienceMilliseconds":0,"gateHeldMilliseconds":1200,"expect":{"outcome":"timed-out","minElapsedMilliseconds":0,"maxElapsedMilliseconds":900}},
  {"id":"queued-try-lock-waits-for-the-gate-within-the-patience","api":"queued","patienceMilliseconds":3000,"gateHeldMilliseconds":700,"expect":{"outcome":"granted","minElapsedMilliseconds":600,"maxElapsedMilliseconds":2500}},
  {"id":"queued-try-lock-admission-wait-is-bounded-at-the-patience","api":"queued","patienceMilliseconds":600,"gateHeldMilliseconds":3500,"expect":{"outcome":"timed-out","minElapsedMilliseconds":500,"maxElapsedMilliseconds":2500}},
  {"id":"queued-try-lock-with-patience-zero-is-a-single-attempt","api":"queued","patienceMilliseconds":0,"gateHeldMilliseconds":1200,"expect":{"outcome":"timed-out","minElapsedMilliseconds":0,"maxElapsedMilliseconds":900}}]}
sch={"$schema":"http://json-schema.org/draft-07/schema#","$id":"https://semio.tech/framework/process/lease-admission/v1","title":"Directory admission corpus: sweep cadence and bounded admission wait","definitions":{"Corpus":{"type":"object","additionalProperties":False,"required":["version","sweep","gate"],"properties":{"version":{"const":1},
 "sweep":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","intervalMilliseconds","calls"],"properties":{"id":{"type":"string","minLength":1},"intervalMilliseconds":{"type":"integer","minimum":0},"calls":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["atMs","timeoutMs","expect"],"properties":{"atMs":{"type":"integer","minimum":0},"timeoutMs":{"type":["integer","null"],"minimum":0},"expect":{"type":"boolean"}}}}}}},
 "gate":{"type":"array","minItems":1,"items":{"type":"object","additionalProperties":False,"required":["id","api","patienceMilliseconds","gateHeldMilliseconds","expect"],"properties":{"id":{"type":"string","minLength":1},"api":{"enum":["raw","queued"]},"patienceMilliseconds":{"type":"integer","minimum":0},"gateHeldMilliseconds":{"type":"integer","minimum":1},"expect":{"type":"object","additionalProperties":False,"required":["outcome","minElapsedMilliseconds","maxElapsedMilliseconds"],"properties":{"outcome":{"enum":["granted","timed-out"]},"minElapsedMilliseconds":{"type":"integer","minimum":0},"maxElapsedMilliseconds":{"type":"integer","minimum":1}}}}}}}}}}
test='''import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {acquireQueuedResourceLease,acquireResourceLease,claimSweepDueV1,resourceLeasePathV1} from "../../🟦️.ts";
import corpus from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import authored from "../../🧭️order/🔣️policy.json";
import ticketSchema from "../../🧬️schema/📜️ticket.json";

const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"la-"));
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const admissionResource=ticketSchema.definitions.RetirementPolicy.properties.admissionResource.const;
const validate=new Ajv({strict:true,allowUnionTypes:true}).compile({definitions:schema.definitions,$ref:"#/definitions/Corpus"} as never);

test("the admission corpus satisfies its closed schema in an independent validator",()=>{
 expect(validate(corpus)).toBe(true);expect(validate({...corpus,extra:1})).toBe(false);
 expect(validate({...corpus,sweep:[{...corpus.sweep[0],calls:[{atMs:0,timeoutMs:-1,expect:true}]}]})).toBe(false);
});

for(const row of corpus.sweep)test(`claim sweep cadence: ${row.id}`,()=>{
 const sweeps=new Map<string,number>();
 for(const call of row.calls)expect(`${call.atMs}:${claimSweepDueV1(sweeps,"/lease",call.atMs,row.intervalMilliseconds,call.timeoutMs??undefined)}`).toBe(`${call.atMs}:${call.expect}`);
});

let counter=0;
for(const row of corpus.gate)test(`admission gate: ${row.id}`,async()=>{
 const directory=join(root,`d${++counter}`),file=join(root,`p${counter}.json`);mkdirSync(directory,{recursive:true});
 writeFileSync(file,JSON.stringify({...authored,gate:{...authored.gate,admissionPatienceMilliseconds:row.patienceMilliseconds,claimSweepIntervalMilliseconds:0}}));
 process.env.SEMIO_LEASE_ORDER_POLICY=file;process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=join(root,`r${counter}`);
 const signal=()=>new AbortController().signal;
 (await acquireResourceLease({directory,resource:"warm",mode:"shared",signal:signal()})).release();
 const database=new Database(resourceLeasePathV1(directory,admissionResource));database.exec("PRAGMA busy_timeout=0");database.exec("BEGIN EXCLUSIVE");
 const timer=setTimeout(()=>{try{database.exec("ROLLBACK");}catch{}},row.gateHeldMilliseconds),started=Date.now();
 const options={directory,resource:"try",mode:"exclusive" as const,signal:signal(),timeoutMs:0};
 const outcome=await (row.api==="queued"?acquireQueuedResourceLease({...options,owner:"corpus"}):acquireResourceLease(options)).then(lease=>{lease.release();return "granted";},()=>"timed-out");
 const elapsed=Date.now()-started;
 clearTimeout(timer);try{database.exec("ROLLBACK");}catch{}database.close();
 expect(outcome).toBe(row.expect.outcome);expect(elapsed).toBeGreaterThanOrEqual(row.expect.minElapsedMilliseconds);expect(elapsed).toBeLessThanOrEqual(row.expect.maxElapsedMilliseconds);
});
'''
for rel,text in ((A+"🔣️.json",json.dumps(corpus,ensure_ascii=False,indent=1)+"\n"),(A+"🧬️schema/🔣️.json",json.dumps(sch,ensure_ascii=False,indent=2)+"\n"),(A+"🧪️tests/🟦️.ts",test)):
    path=os.path.join(ROOT,rel)
    if not os.path.isfile(path):out[rel]=text
# registrations
sc=rd(PR+"📜️script.ts")
if "resource-lease-admission" not in sc:
    sc=once(sc,'      "resource-lease-order-processes": {source:"🔒️leases/🧭️order/🧪️tests/📥️processes/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n','      "resource-lease-order-processes": {source:"🔒️leases/🧭️order/🧪️tests/📥️processes/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n      "resource-lease-admission": {source:"🔒️leases/🚪️admission/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},\n')
    out[PR+"📜️script.ts"]=sc
pj=rd(PR+"📋️project.json")
if "test-resource-lease-admission" not in pj:
    a=pj.index('    "test-resource-lease-order-processes": {');b=pj.index('    "test-',a+10)
    block=pj[a:b].replace("resource-lease-order-processes","resource-lease-admission")
    pj=pj[:b]+block+pj[b:]
    json.loads(pj)
    out[PR+"📋️project.json"]=pj
for k in out:print("edit",k)
if "--write" in sys.argv:
    for k,v in out.items():
        q=os.path.join(ROOT,k);os.makedirs(os.path.dirname(q),exist_ok=True);open(q+".t","w",encoding="utf8").write(v);os.replace(q+".t",q)
    print("written")
