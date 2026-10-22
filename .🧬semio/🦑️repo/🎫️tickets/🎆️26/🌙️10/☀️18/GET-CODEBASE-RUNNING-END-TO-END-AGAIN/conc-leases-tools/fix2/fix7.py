import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT=sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else None
if ROOT is None:
    p=HERE
    while p!=os.path.dirname(p):
        if os.path.isfile(os.path.join(p,"bunfig.toml")) and os.path.isfile(os.path.join(p,"bun.lock")):ROOT=p;break
        p=os.path.dirname(p)
P="🧰️framework/🔨️modules/🏃️process/🔒️leases/"
O=P+"🧭️order/"
def rd(rel):return open(os.path.join(ROOT,rel),encoding="utf8").read()
def once(t,o,n):
    assert t.count(o)==1,"anchor: "+o[:80]
    return t.replace(o,n)
out={}
s=json.loads(rd(O+"🧬️schema/🔣️.json"));g=s["definitions"]["Policy"]["properties"]["gate"]["properties"]
if "admissionPatienceMilliseconds" not in g:
    g["admissionPatienceMilliseconds"]={"type":"integer","minimum":0,"maximum":600000,"description":"How long a request that must not wait for its resource (timeout 0) still waits for the directory admission gate, which is held only for microseconds; 0 keeps the strict single attempt."}
    g["claimSweepIntervalMilliseconds"]={"type":"integer","minimum":0,"maximum":86400000,"description":"The least time between two retirement sweeps of settled queue claims that one process starts from queue acquisitions; each sweep takes the directory admission gate exclusively for a moment, so a sweep per acquisition starves every other opener."}
    out[O+"🧬️schema/🔣️.json"]=json.dumps(s,ensure_ascii=False,indent=2)+"\n"
p=json.loads(rd(O+"🔣️policy.json"))
if "admissionPatienceMilliseconds" not in p["gate"]:
    p["gate"]["admissionPatienceMilliseconds"]=5000;p["gate"]["claimSweepIntervalMilliseconds"]=30000
    out[O+"🔣️policy.json"]=json.dumps(p,ensure_ascii=False,indent=2)+"\n"
m=rd(O+"🟦️.ts")
if "admissionPatienceMilliseconds" not in m:
    m=once(m,"gate:Readonly<{releaseWhileWaiting:boolean;pollInitialMilliseconds:number;pollMaximumMilliseconds:number}>","gate:Readonly<{releaseWhileWaiting:boolean;pollInitialMilliseconds:number;pollMaximumMilliseconds:number;admissionPatienceMilliseconds?:number;claimSweepIntervalMilliseconds?:number}>")
    out[O+"🟦️.ts"]=m
l=rd(P+"🟦️.ts")
if "claimSweepDue" not in l:
    l=once(l,"const entered=Date.now();admissionFinalizer.retry();try{return await acquireUngatedResourceLease({...options,resource:admissionResource,mode,maximumDatabaseBytes:retirementSchema.properties.maximumDatabaseBytes.const},started,undefined,cleanup);}",
      "const entered=Date.now(),patience=!cleanup&&options.timeoutMs===0?leaseOrderPolicyV1().gate.admissionPatienceMilliseconds??0:0;admissionFinalizer.retry();try{return await acquireUngatedResourceLease({...options,...patience>0?{timeoutMs:patience}:{},resource:admissionResource,mode,maximumDatabaseBytes:retirementSchema.properties.maximumDatabaseBytes.const},patience>0?entered:started,undefined,cleanup);}")
    l=once(l,"  if(options.control)await retireResourceQueueClaimsV1(options.directory,options.control,0);","  if(options.control&&claimSweepDue(resolve(options.directory)))await retireResourceQueueClaimsV1(options.directory,options.control,0);")
    l=once(l,"/** 🎟️ Whether an exclusive request is queued","""const lastClaimSweep=new Map<string,number>();
/** 🧹️ Whether this process may start another retirement sweep of settled queue claims: the sweep takes the directory admission gate exclusively, so one per acquisition would make every other opener lose the gate; the interval is `gate.claimSweepIntervalMilliseconds`. */
function claimSweepDue(directory:string):boolean{
  const now=Date.now(),interval=leaseOrderPolicyV1().gate.claimSweepIntervalMilliseconds??0,last=lastClaimSweep.get(directory);
  if(interval>0&&last!==undefined&&now-last<interval)return false;
  lastClaimSweep.set(directory,now);return true;
}
/** 🎟️ Whether an exclusive request is queued""")
    out[P+"🟦️.ts"]=l
pt=rd(O+"🧪️tests/📥️processes/🟦️.ts")
if "admission gate is busy" not in pt:
    pt=once(pt,'\nconst wedge=async','''
const holdGate=async(directory:string,milliseconds:number)=>{
 const database=new Database(resourceLeasePathV1(directory,admissionResource));database.exec("PRAGMA busy_timeout=0");database.exec("BEGIN EXCLUSIVE");
 return {release:()=>{try{database.exec("ROLLBACK");}finally{database.close();}},milliseconds};
};

test("a request that must not wait for its resource still waits for a busy admission gate; the strict single attempt is the negative control",async()=>{
 for(const patient of [true,false]){
  const {directory,env}=scenario({gate:{releaseWhileWaiting:true,pollInitialMilliseconds:5,pollMaximumMilliseconds:40,admissionPatienceMilliseconds:patient?3000:0,claimSweepIntervalMilliseconds:0}});configure(env);
  (await acquireResourceLease({directory,resource:"free:warm",mode:"shared",signal:signal()})).release();
  for(const queued of [false,true]){
   const gate=await holdGate(directory,800),release=setTimeout(()=>gate.release(),800),started=Date.now();
   const options={directory,resource:"free:try",mode:"exclusive" as const,signal:signal(),timeoutMs:0};
   const outcome=await (queued?acquireQueuedResourceLease({...options,owner:"try"+counter}):acquireResourceLease(options)).then(lease=>{lease.release();return "granted";},error=>String(error));
   clearTimeout(release);try{gate.release();}catch{}
   if(patient){expect(outcome).toBe("granted");expect(Date.now()-started).toBeGreaterThanOrEqual(600);}
   else expect(outcome).toContain("timed out");
  }
 }
});

test("a request that must not wait still fails promptly when its resource is busy, even with a patient gate",async()=>{
 const {directory,env}=scenario();configure(env);
 const holder=start({mode:"hold",directory,resource:"free:busy",access:"exclusive"},env);await holder.next("HELD");
 const started=Date.now(),outcome=await acquireResourceLease({directory,resource:"free:busy",mode:"exclusive",signal:signal(),timeoutMs:0}).then(()=>"granted",error=>String(error));
 expect(outcome).toContain("Resource lease timed out: free:busy");expect(Date.now()-started).toBeLessThan(1500);
 holder.send("release");await holder.next("DONE");
});

const wedge=async''')
    out[O+"🧪️tests/📥️processes/🟦️.ts"]=pt
for k in out:print("edit",k)
if "--write" in sys.argv:
    for k,v in out.items():
        q=os.path.join(ROOT,k);open(q+".t","w",encoding="utf8").write(v);os.replace(q+".t",q)
    print("written")
