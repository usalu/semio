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
    assert t.count(o)==1,"anchor: "+o[:70]
    return t.replace(o,n)
out={}
m=rd(O+"🟦️.ts")
if "holdsSharedInProcessV1" not in m:
    m=once(m,"fairness:Readonly<{queuedWithResourceLeases:boolean}>;","fairness:Readonly<{queuedWithResourceLeases:boolean;reentrantShared?:boolean}>;")
    m=once(m,'''  else if(wait.queued&&other.wait?.queued&&other.wait.key===wait.key&&(other.wait.sinceMs<wait.sinceMs||other.wait.sinceMs===wait.sinceMs&&other.id<node.id)&&leaseModesConflictV1(other.wait.mode,wait.mode))found.add(other.id);
 }
 return [...found].sort();''','''  else if(wait.queued&&other.wait?.queued&&other.wait.key===wait.key&&(other.wait.sinceMs<wait.sinceMs||other.wait.sinceMs===wait.sinceMs&&other.id<node.id)&&leaseModesConflictV1(other.wait.mode,wait.mode))found.add(other.id);
 }
 return [...found].sort();
}

/** 🔁️ The participants a node's own process waits on for the node to finish: a task that holds a lease shared depends on a sibling task of the same process that is queued for the same lease shared, because that is a re-entrant request stuck behind writers that wait for the very hold; the process cannot finish either. */
export function leaseProcessDependenciesV1(nodes:readonly LeaseNodeV1[],node:LeaseNodeV1):readonly string[]{
 const owner=node.id.slice(0,node.id.indexOf("/")),found=new Set<string>();
 for(const hold of node.holds)if(hold.mode==="shared")for(const other of nodes)if(other.id!==node.id&&other.id.slice(0,other.id.indexOf("/"))===owner&&other.wait?.queued&&other.wait.mode==="shared"&&other.wait.key===hold.key)found.add(other.id);
 return [...found].sort();''')
    m=once(m,"const victim=[...path].sort((left,right)=>{const a=byId.get(left)!.wait!.sinceMs,b=byId.get(right)!.wait!.sinceMs;return b-a||(left<right?1:left>right?-1:0);})[0]!;","const victim=[...path].filter(id=>byId.get(id)!.wait).sort((left,right)=>{const a=byId.get(left)!.wait!.sinceMs,b=byId.get(right)!.wait!.sinceMs;return b-a||(left<right?1:left>right?-1:0);})[0]!;")
    m=m.replace("for(const next of leaseBlockersV1(nodes,byId.get(id)!)){","for(const next of [...leaseBlockersV1(nodes,byId.get(id)!),...leaseProcessDependenciesV1(nodes,byId.get(id)!)]){")
    m=once(m,"/** 🔓️ Drops the bookkeeping of a lease","""/** ♻️ Whether this process, or the parent that started it, already holds a lease shared: a further shared request for it is re-entrant and must be granted at once, never queued behind writers that wait for the very hold. */
export function holdsSharedInProcessV1(directory:string,resource:string):boolean{
 return [...holds.values()].some(row=>row.mode==="shared"&&row.directory===directory&&row.resource===resource)||inheritedLeasesV1().some(row=>row.mode==="shared"&&row.directory===directory&&row.resource===resource);
}

/** 🔓️ Drops the bookkeeping of a lease""")
    out[O+"🟦️.ts"]=m
s=json.loads(rd(O+"🧬️schema/🔣️.json"))
fair=s["definitions"]["Policy"]["properties"]["fairness"]
if "reentrantShared" not in fair["properties"]:
    fair["properties"]["reentrantShared"]={"type":"boolean"}
    out[O+"🧬️schema/🔣️.json"]=json.dumps(s,ensure_ascii=False,indent=2)+"\n"
l=rd(P+"🟦️.ts")
if "holdsSharedInProcessV1" not in l:
    l=once(l,"currentLeaseLineageV1,heldLeasesV1,","currentLeaseLineageV1,heldLeasesV1,holdsSharedInProcessV1,")
    l=once(l,"  const guard=orderGuard(options,true);\n  if(!guard)return acquireQueuedResourceLeaseBody(options);","  if(options.mode===\"shared\"&&typeof options.resource===\"string\"&&!internalLeaseResource(options.resource)&&leaseOrderPolicyV1().fairness.reentrantShared!==false&&holdsSharedInProcessV1(resolve(options.directory),options.resource)&&exclusiveTicketQueued(options.directory,options.resource)){const{owner:_owner,sharedCohort:_cohort,...raw}=options;return acquireResourceLease(raw);}\n  const guard=orderGuard(options,true);\n  if(!guard)return acquireQueuedResourceLeaseBody(options);")
    l=once(l,"/** ⬆️ Converts a held queued shared lease","""/** 🎟️ Whether an exclusive request is queued on a lease: only then can a further shared request of a holder be stuck behind a writer, so only then does it skip the queue; otherwise it queues normally and stays upgradeable. */
function exclusiveTicketQueued(directory:string,resource:string):boolean{
  try{const queue=resourceLeasePathV1(directory,resource).replace(/\\.sqlite$/u,".queue");for(const name of readdirSync(queue))if(queueTicketRow(queue,name)?.mode==="exclusive")return true;}catch{}
  return false;
}
/** ⬆️ Converts a held queued shared lease""")
    out[P+"🟦️.ts"]=l
s=rd(P+"📋️status/🟦️.ts")
if "holderRun" not in s:
    s=once(s,"  const id=`${ticket.pid}/q`,asHolder=ticket.position===1;","  const id=`${ticket.pid}/q`,run=holderRun(tickets),asHolder=ticket.position<=run;")
    s=once(s,"const hash=(resource:string)","/** 🎒️ How many leading tickets of an arrival queue hold the lease: the exclusive head alone, or the whole run of shared tickets at the head. */\nconst holderRun=(tickets:readonly Readonly<{mode:LeaseModeV1}>[]):number=>{if(tickets[0]?.mode==='exclusive')return 1;let count=0;while(count<tickets.length&&tickets[count]!.mode==='shared')count++;return count;};\nconst hash=(resource:string)")
    out[P+"📋️status/🟦️.ts"]=s
ut=rd(O+"🧪️tests/🟦️.ts")
if "leaseProcessDependenciesV1" not in ut:
    ut=once(ut,"leaseBlockersV1,leaseOrderPolicyV1,","leaseBlockersV1,leaseOrderPolicyV1,leaseProcessDependenciesV1,")
    ut=once(ut,"for(const to of leaseBlockersV1(nodes,node))insert.run(node.id,to);","for(const to of [...leaseBlockersV1(nodes,node),...leaseProcessDependenciesV1(nodes,node)])insert.run(node.id,to);")
    out[O+"🧪️tests/🟦️.ts"]=ut
# fixtures
fx=json.loads(rd(O+"🧪️tests/🧫️fixtures/🔣️.json"))
if not any(r["id"]=="reentrant-shared-wedge" for r in fx["cycles"]):
    fx["cycles"].append({"id":"reentrant-shared-wedge","from":"1/2","nodes":[{"id":"1/1","holds":[{"key":"G","mode":"shared"}]},{"id":"1/2","holds":[],"wait":{"key":"G","mode":"shared","sinceMs":500,"queued":True}},{"id":"2/1","holds":[],"wait":{"key":"G","mode":"exclusive","sinceMs":100,"queued":True}},{"id":"3/1","holds":[],"wait":{"key":"G","mode":"exclusive","sinceMs":110,"queued":True}}],"expect":{"members":["1/2","2/1","1/1"],"victim":"1/2"}})
    fx["cycles"].append({"id":"shared-holder-with-no-sibling-request-is-no-cycle","from":"2/1","nodes":[{"id":"1/1","holds":[{"key":"G","mode":"shared"}]},{"id":"2/1","holds":[],"wait":{"key":"G","mode":"exclusive","sinceMs":100,"queued":True}}],"expect":None})
    out[O+"🧪️tests/🧫️fixtures/🔣️.json"]=json.dumps(fx,ensure_ascii=False,indent=1)+"\n"
pt=rd(O+"🧪️tests/📥️processes/🟦️.ts")
if "re-entrant shared" not in pt:
    new='''
const wedge=async(reentrantShared:boolean)=>{
 const {directory,env}=scenario({fairness:{queuedWithResourceLeases:true,reentrantShared},holdAndWait:{graceMilliseconds:60000,pollMilliseconds:50}});configure(env);
 const shared=(owner:string)=>acquireQueuedResourceLease({directory,resource:"free:g",mode:"shared",sharedCohort:true,owner,signal:signal()});
 const first=await withLeaseTaskV1(()=>shared("held"));
 const writers=[0,1,2].map(()=>start({mode:"wait",queued:true,directory,resource:"free:g",access:"exclusive",timeoutMs:40000},env));
 await tick(900);
 const started=Date.now(),second=await withLeaseTaskV1(()=>shared("second")).then(lease=>lease,error=>error);
 return {first,second,writers,elapsed:Date.now()-started};
};

test("re-entrant shared: a process that holds a shared lease gets a second shared request at once, even with exclusive requests queued on it",async()=>{
 const {first,second,writers,elapsed}=await wedge(true);
 expect(second).not.toBeInstanceOf(Error);expect(elapsed).toBeLessThan(800);
 second.release();first.release();
 for(const writer of writers)await writer.next("ACQUIRED");
});

test("the exact wedge shape without re-entrancy is detected as a deadlock cycle through the queued-ahead writers, and only the stuck second request fails",async()=>{
 const {first,second,writers,elapsed}=await wedge(false);
 expect(second).toBeInstanceOf(LeaseOrderErrorV1);expect(second.violation.kind).toBe("cycle");expect(elapsed).toBeLessThan(8000);
 expect(second.violation.cycle.length).toBeGreaterThanOrEqual(3);
 first.release();
 for(const writer of writers)await writer.next("ACQUIRED");
});
'''
    pt=once(pt,'test("a child started with the inherited environment',new.lstrip("\n")+'\ntest("a child started with the inherited environment')
    out[O+"🧪️tests/📥️processes/🟦️.ts"]=pt
for k in out:print("edit",k)
if "--write" in sys.argv:
    for k,v in out.items():
        p=os.path.join(ROOT,k);open(p+".t","w",encoding="utf8").write(v);os.replace(p+".t",p)
    print("written")
