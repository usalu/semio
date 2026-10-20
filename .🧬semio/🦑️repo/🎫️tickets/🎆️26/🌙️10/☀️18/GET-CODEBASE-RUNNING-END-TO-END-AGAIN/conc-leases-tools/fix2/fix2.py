import hashlib,json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT=sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else None
if ROOT is None:
    p=HERE
    while p!=os.path.dirname(p):
        if os.path.isfile(os.path.join(p,"bunfig.toml")) and os.path.isfile(os.path.join(p,"bun.lock")):ROOT=p;break
        p=os.path.dirname(p)
WRITE="--write" in sys.argv
P="🧰️framework/🔨️modules/🏃️process/"
NX="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🎟️put-slots/"
class Drift(Exception):pass
def edit(rel,edits,done_marker):
    path=os.path.join(ROOT,rel);text=open(path,encoding="utf8").read()
    if done_marker in text:return rel,"already",None
    for i,(old,new) in enumerate(edits):
        n=text.count(old)
        if n!=1:raise Drift("%s edit %d: anchor found %d times"%(rel,i,n))
        text=text.replace(old,new)
    return rel,"edit",text
results=[]
ORDER=P+"🔒️leases/🧭️order/🟦️.ts"
results.append(edit(ORDER,[
 ('import {readFileSync} from "node:fs";','import {AsyncLocalStorage} from "node:async_hooks";\nimport {readFileSync} from "node:fs";'),
 ('type Live={readonly id:number;','''type LeaseScope={readonly id:number;readonly parent?:LeaseScope};
const scopes=new AsyncLocalStorage<LeaseScope>();
/** 🧵️ Runs a body as its own lease task, a child of the task running now: the child sees what its ancestors hold (it runs inside their extent) but never what a concurrent sibling holds, so two siblings waiting for each other's lease are an ordinary wait or a wait-for cycle, never a self-deadlock. */
export function withLeaseTaskV1<T>(body:()=>T):T{const parent=scopes.getStore();return scopes.run({id:nextTask++,...parent?{parent}:{}},body);}
/** 🧵️ The task running now followed by its ancestors, innermost first; undefined outside any scope. */
export function currentLeaseLineageV1():readonly number[]|undefined{const lineage:number[]=[];for(let scope=scopes.getStore();scope;scope=scope.parent)lineage.push(scope.id);return lineage.length?lineage:undefined;}

type Live={readonly id:number;'''),
 ('export function heldLeasesV1(task:number):readonly HeldLeaseV1[]{\n return [...inheritedLeasesV1(),...[...holds.values()].filter(row=>row.task===task)];','export function heldLeasesV1(task:number,ancestors:readonly number[]=[]):readonly HeldLeaseV1[]{\n return [...inheritedLeasesV1(),...[...holds.values()].filter(row=>row.task===task||ancestors.includes(row.task))];'),
 ('/** 🎒️ What a task holds: its own leases and everything inherited from the parent process. */','/** 🎒️ What a task holds: its own leases, those of its ancestor tasks and everything inherited from the parent process. */'),
 ('''if(policy.rules.selfDeadlock!=="off"&&request.bounded!==true){
  const own=held.find(row=>row.directory===request.directory&&row.resource===request.resource&&leaseModesConflictV1(row.mode,request.mode));
  if(own)return violation("self-deadlock",policy,own,request);
 }
 if(policy.rules.outOfOrder''','''if(policy.rules.outOfOrder'''),
 ('/** 🧭️ Checks one blocking request against what its task already holds: an unbounded request that conflicts with the task\'s own hold of the same lease can never be granted (a request with a timeout is bounded and left to expire), and a request ranked before a lease already held breaks the declared order. Requests that cannot wait (timeout 0) are exempt. */','''/** 🔁️ Decides whether a request that has waited `waitedMilliseconds` is a self-deadlock: its own task, or an ancestor task it runs inside, holds the same lease in a conflicting mode, so the wait can only end if that holder releases first. A concurrent sibling task is not the holder. Bounded requests expire on their own and requests that cannot wait are exempt. */
export function checkSelfDeadlockV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request,waitedMilliseconds:number):LeaseViolationV1|undefined{
 if(request.nonblocking||request.bounded===true||policy.rules.selfDeadlock==="off"||waitedMilliseconds<policy.holdAndWait.graceMilliseconds)return undefined;
 const own=held.find(row=>row.directory===request.directory&&row.resource===request.resource&&leaseModesConflictV1(row.mode,request.mode));
 return own?violation("self-deadlock",policy,own,request,{waitedMilliseconds}):undefined;
}

/** 🧭️ Checks one blocking request against what its task already holds: a request ranked before a lease already held breaks the declared order. Requests that cannot wait (timeout 0) are exempt. */'''),
 ('export function beginLeaseOrderV1(request:Request&Readonly<{task:number;queued:boolean}>,','export function beginLeaseOrderV1(request:Request&Readonly<{task:number;ancestors?:readonly number[];queued:boolean}>,'),
 ('held=heldLeasesV1(request.task),early=checkLeaseOrderV1','held=heldLeasesV1(request.task,request.ancestors),early=checkLeaseOrderV1'),
 ('early.kind==="self-deadlock"?policy.rules.selfDeadlock:policy.rules.outOfOrder','policy.rules.outOfOrder'),
 ('  const blocker=waitedMilliseconds>=policy.holdAndWait.graceMilliseconds?leaseRegistryV1.holderOf(request.directory,request.resource,request.mode):undefined;\n  const refusal=checkHoldAndWaitV1(policy,heldLeasesV1(request.task),request,waitedMilliseconds,blocker);','  const current=heldLeasesV1(request.task,request.ancestors),own=checkSelfDeadlockV1(policy,current,request,waitedMilliseconds);\n  if(own)react(policy,policy.rules.selfDeadlock,own,request.directory);\n  const blocker=waitedMilliseconds>=policy.holdAndWait.graceMilliseconds?leaseRegistryV1.holderOf(request.directory,request.resource,request.mode):undefined;\n  const refusal=checkHoldAndWaitV1(policy,current,request,waitedMilliseconds,blocker);'),
],"export function withLeaseTaskV1"))
LEASES=P+"🔒️leases/🟦️.ts"
results.append(edit(LEASES,[
 ('heldLeasesV1,leaseOrderPolicyV1,leaseTaskIdV1,type LeaseOrderGuardV1} from "./🧭️order/🟦️.ts";','currentLeaseLineageV1,heldLeasesV1,leaseOrderPolicyV1,leaseTaskIdV1,type LeaseOrderGuardV1} from "./🧭️order/🟦️.ts";'),
 ('export {LeaseOrderErrorV1,formatLeaseViolationV1,leaseInheritanceEnvironmentV1} from','export {LeaseOrderErrorV1,formatLeaseViolationV1,leaseInheritanceEnvironmentV1,withLeaseTaskV1} from'),
 ('''function leaseTaskOf(options:Pick<LeaseOptions,"control"|"signal">):number{
 let key:object=options.signal!==null&&typeof options.signal==="object"?options.signal:untrackedTask;
 if(options.control)try{const capabilities=readIssuedResourceControlCapabilitiesV1(options.control);if(capabilities!==null&&typeof capabilities==="object")key=capabilities;}catch{}
 return leaseTaskIdV1(key);
}''','''function leaseTaskOf(options:Pick<LeaseOptions,"control"|"signal">):Readonly<{task:number;ancestors:readonly number[]}>{
 const lineage=currentLeaseLineageV1();
 if(lineage)return{task:lineage[0]!,ancestors:lineage.slice(1)};
 const key:object=options.control!==null&&typeof options.control==="object"?options.control:options.signal!==null&&typeof options.signal==="object"?options.signal:untrackedTask;
 return{task:leaseTaskIdV1(key),ancestors:[]};
}'''),
 ('bounded:options.timeoutMs!==undefined,task:leaseTaskOf(options),queued})','bounded:options.timeoutMs!==undefined,...leaseTaskOf(options),queued})'),
 ('!heldLeasesV1(leaseTaskOf(request)).some(','!(()=>{const owner=leaseTaskOf(request);return heldLeasesV1(owner.task,owner.ancestors);})().some('),
],"currentLeaseLineageV1"))
PKG=P+"📦️packages/🟦️typescript/🔒️leases/🟦️.ts"
results.append(edit(PKG,[('withResourceLeases } from "../../../🔒️leases/🟦️.ts";','withResourceLeases, withLeaseTaskV1 } from "../../../🔒️leases/🟦️.ts";')],"withLeaseTaskV1"))
ROUTING=P+"🧭️routing/📥️invocation/🏃️process/🟦️.ts"
results.append(edit(ROUTING,[
 ('import{AsyncLocalStorage}from"node:async_hooks";','import{AsyncLocalStorage}from"node:async_hooks";\nimport{withLeaseTaskV1}from"../../../🔒️leases/🧭️order/🟦️.ts";'),
 ('const result=await run(control);remaining(child);return result;','const result=await withLeaseTaskV1(()=>run(control));remaining(child);return result;'),
 ('const result=await run(childControl);remaining(child);return result;','const result=await withLeaseTaskV1(()=>run(childControl));remaining(child);return result;'),
],"withLeaseTaskV1"))
PUT=NX+"🟦️.ts"
results.append(edit(PUT,[
 ('type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";','type ResourceLeaseControl,withLeaseTaskV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";'),
 ('/** 🧵️ Runs one publication in a free slot.','''/** 🧵️ Runs one publication as its own lease task, so concurrent publications of one process wait for each other's gate and slot exactly like publications of separate processes. */
export function withNxPutSlotV1<T>(options:Readonly<{root:string;leaseDirectory:string;control:ResourceLeaseControl;slots:number}>,body:(slot:NxPutSlotV1)=>Promise<T>):Promise<T>{return withLeaseTaskV1(()=>admitNxPutSlotV1(options,body));}

/** 🧵️ Runs one publication in a free slot.'''),
 ('export async function withNxPutSlotV1<T>(','async function admitNxPutSlotV1<T>('),
],"admitNxPutSlotV1"))
TEST=NX+"🧪️tests/🟦️.ts"
path=os.path.join(ROOT,TEST);t=open(path,encoding="utf8").read()
marker='process.env.SEMIO_LEASE_ORDER??="observe";/* conc-leases counts concurrent publications of one Original as one task: its self-deadlock rule (enforce) rejects the second put on the unbounded put gate; OPEN CONFLICT 1 in result-integrate2 (round 3) */\n'
results.append((TEST,"edit" if marker in t else "already",t.replace(marker,"") if marker in t else None))
old=json.load(open(os.path.join(HERE,"old-hashes.json"),encoding="utf8"))
STAGED=os.path.join(HERE,"..","staged",P+"🔒️leases/🧭️order/🧪️tests")
def digest(path):return hashlib.sha256(open(path,"rb").read()).hexdigest() if os.path.isfile(path) else None
for name,previous in old.items():
    live=os.path.join(ROOT,P+"🔒️leases/🧭️order/🧪️tests",name);mine=os.path.join(STAGED,name)
    if digest(live)==digest(mine):results.append((P+"🔒️leases/🧭️order/🧪️tests/"+name,"already",None))
    elif digest(live)==previous:results.append((P+"🔒️leases/🧭️order/🧪️tests/"+name,"edit",open(mine,encoding="utf8").read()))
    else:raise Drift("test file changed since staging: "+name)
for rel,state,text in results:print(state.ljust(8),rel)
if WRITE:
    for rel,state,text in results:
        if text is not None:
            path=os.path.join(ROOT,rel);tmp=path+".conc-leases2.tmp";open(tmp,"w",encoding="utf8").write(text);os.replace(tmp,path)
    print("written")
