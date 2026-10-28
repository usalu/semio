import {AsyncLocalStorage} from "node:async_hooks";
import {readFileSync} from "node:fs";
import {validateJsonSchemaSubset} from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import authoredPolicy from "./🔣️policy.json" with {type:"json"};
import {leaseRegistryV1} from "./🗂️registry/🟦️.ts";

import type {HeldLeaseV1, LeaseClassV1, LeaseCycleV1, LeaseModeV1, LeaseNodeV1, LeaseOrderPolicyV1, LeaseOrderRuleV1, LeaseViolationV1} from "./📋️contract/🟦️.ts";
export type {HeldLeaseV1, LeaseClassV1, LeaseCycleV1, LeaseModeV1, LeaseNodeV1, LeaseOrderPolicyV1, LeaseOrderRuleV1, LeaseViolationV1} from "./📋️contract/🟦️.ts";

/** 🛑️ Raised when a request breaks the declared lock order, would wait while holding, or closes a deadlock; carries the violation as data. */
export class LeaseOrderErrorV1 extends Error{
 readonly code="SEMIO_LEASE_ORDER";
 readonly violation:LeaseViolationV1;
 constructor(message:string,violation:LeaseViolationV1){super(message);this.name="LeaseOrderErrorV1";this.violation=violation;}
}

const protocol=schema.definitions.Protocol.const;

/** 🛂️ Requires a value to match one closed definition of the lock order schema. */
export function requireLeaseOrderV1<T>(definition:"Policy"|"Violation"|"Class"|"Held",value:unknown):T{
 const errors=validateJsonSchemaSubset({definitions:schema.definitions,$ref:`#/definitions/${definition}`},value);
 if(errors.length)throw Error(`Closed lease order ${definition} required: ${errors[0]}`);
 return value as T;
}

/** 📖️ Reads a policy and refuses duplicate classes, ambiguous matches and edges between unknown classes. */
export function readLeaseOrderPolicyV1(value:unknown):LeaseOrderPolicyV1{
 const policy=requireLeaseOrderV1<LeaseOrderPolicyV1>("Policy",value),ids=new Set<string>(),matches=new Set<string>();
 for(const row of policy.classes){
  if(ids.has(row.id))throw Error(`Lease order class declared twice: ${row.id}`);
  ids.add(row.id);
  for(const match of row.match){const key=`${match.kind}\0${match.value}`;if(matches.has(key))throw Error(`Lease order match declared twice: ${match.value}`);matches.add(key);}
 }
 for(const edge of policy.allowed)if(!ids.has(edge.from)||!ids.has(edge.to))throw Error(`Lease order edge names an unknown class: ${edge.from} to ${edge.to}`);
 return policy;
}

const cached=new Map<string,LeaseOrderPolicyV1>();
/** 📚️ The policy in force: the authored table, or the file named by `SEMIO_LEASE_ORDER_POLICY`; `SEMIO_LEASE_ORDER=off|observe|enforce` overrides every rule at once. */
export function leaseOrderPolicyV1(environment:Readonly<Record<string,string|undefined>>=process.env):LeaseOrderPolicyV1{
 const path=environment[protocol.policyEnvironment],override=environment[protocol.modeEnvironment],key=`${path??""}\0${override??""}`;
 const known=cached.get(key);if(known)return known;
 let policy=readLeaseOrderPolicyV1(path?JSON.parse(readFileSync(path,"utf8")):authoredPolicy);
 if(override==="off"||override==="observe"||override==="enforce")policy=Object.freeze({...policy,rules:Object.freeze({outOfOrder:override,holdAndWait:override,selfDeadlock:override,cycle:override,strictWait:override})});
 else if(override!==undefined&&override!=="")throw Error("SEMIO_LEASE_ORDER must be off, observe or enforce");
 cached.set(key,policy);return policy;
}

type Matcher=Readonly<{exact:Map<string,LeaseClassV1>;prefixes:readonly Readonly<{value:string;row:LeaseClassV1}>[]}>;
const matchers=new WeakMap<LeaseOrderPolicyV1,Matcher>();
const UNCLASSIFIED:LeaseClassV1=Object.freeze({id:"unclassified",rank:0,scope:"session",owner:"unclassified",title:Object.freeze({en:"Unclassified",de:"Nicht eingeordnet"}),match:Object.freeze([])});
/** 🏷️ Finds the class of a resource: an exact name wins, then the longest prefix; anything else is unclassified with rank 0, which no order rule constrains. */
export function classifyLeaseResourceV1(policy:LeaseOrderPolicyV1,resource:string):LeaseClassV1{
 let matcher=matchers.get(policy);
 if(!matcher){
  const exact=new Map<string,LeaseClassV1>(),prefixes:{value:string;row:LeaseClassV1}[]=[];
  for(const row of policy.classes)for(const match of row.match)if(match.kind==="exact")exact.set(match.value,row);else prefixes.push({value:match.value,row});
  prefixes.sort((left,right)=>right.value.length-left.value.length);
  matcher=Object.freeze({exact,prefixes});matchers.set(policy,matcher);
 }
 return matcher.exact.get(resource)??matcher.prefixes.find(entry=>resource.startsWith(entry.value))?.row??UNCLASSIFIED;
}

/** ⚔️ Whether two modes exclude each other: only two shared holds coexist. */
export function leaseModesConflictV1(left:LeaseModeV1,right:LeaseModeV1):boolean{return left==="exclusive"||right==="exclusive";}

/** 🔑️ The identity of one lease inside the cross-process wait-for graph. */
export function leaseKeyV1(directory:string,resource:string):string{return `${directory}\0${resource}`;}

const slash=(path:string):string=>path.replaceAll("\\","/").replace(/\/+$/u,"");
/** 💽️ Whether a hold is the medium of the request: a shared hold of an image (`media` prefix plus directory path) that contains the lease directory the request uses. The lease store is such an image; it is open before any lease exists in it, so it is the ground the leases stand on and never a rank to order against. */
export function isLeaseMediumV1(policy:LeaseOrderPolicyV1,held:HeldLeaseV1,request:Readonly<{directory:string}>):boolean{
 if(held.mode!=="shared")return false;
 return policy.media.some(medium=>held.resource.startsWith(medium.prefix)&&(slash(request.directory)+"/").startsWith(slash(held.resource.slice(medium.prefix.length))+"/"));
}

const edgeAllowed=(policy:LeaseOrderPolicyV1,from:string,to:string,held:HeldLeaseV1,request:Request):boolean=>policy.allowed.some(edge=>{if(edge.from!==from||edge.to!==to)return false;const binding=edge.binding;if(!binding)return true;return held.directory===request.directory&&held.resource.startsWith(binding.fromPrefix)&&request.resource.startsWith(binding.toPrefix)&&held.resource.length>binding.fromPrefix.length&&held.resource.slice(binding.fromPrefix.length)===request.resource.slice(binding.toPrefix.length);});

type Request=Readonly<{directory:string;resource:string;mode:LeaseModeV1;nonblocking:boolean;bounded?:boolean}>;
const violation=(kind:LeaseViolationV1["kind"],policy:LeaseOrderPolicyV1,held:HeldLeaseV1,request:Request,extra:Partial<LeaseViolationV1>={}):LeaseViolationV1=>{
 const heldClass=classifyLeaseResourceV1(policy,held.resource),requestedClass=classifyLeaseResourceV1(policy,request.resource);
 return Object.freeze({version:1,kind,held:held.resource,heldMode:held.mode,heldRank:heldClass.rank,heldClass:heldClass.id,requested:request.resource,requestedMode:request.mode,requestedRank:requestedClass.rank,requestedClass:requestedClass.id,atMs:Date.now(),pid:process.pid,...extra});
};

/** 🔁️ Decides whether a request that has waited `waitedMilliseconds` is a self-deadlock: its own task, or an ancestor task it runs inside, holds the same lease in a conflicting mode, so the wait can only end if that holder releases first. A concurrent sibling task is not the holder. Bounded requests expire on their own and requests that cannot wait are exempt. */
export function checkSelfDeadlockV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request,waitedMilliseconds:number):LeaseViolationV1|undefined{
 if(request.nonblocking||request.bounded===true||policy.rules.selfDeadlock==="off"||waitedMilliseconds<policy.holdAndWait.graceMilliseconds)return undefined;
 const own=held.find(row=>!isLeaseMediumV1(policy,row,request)&&row.directory===request.directory&&row.resource===request.resource&&leaseModesConflictV1(row.mode,request.mode));
 return own?violation("self-deadlock",policy,own,request,{waitedMilliseconds}):undefined;
}

/** ⛔️ Decides whether a request that has already waited `waitedMilliseconds` must be refused because its task holds a lease of a declared strict pair: the writer custody of an image while it waits for the Cargo registry, its fetch lease or the toolchain, or the fetch lease while it waits for the writer. Any hold mode counts; the oldest such hold is named. Requests that cannot wait are exempt. */
export function checkStrictWaitV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request,waitedMilliseconds:number):LeaseViolationV1|undefined{
 if(request.nonblocking||(policy.rules.strictWait??"off")==="off"||waitedMilliseconds<policy.holdAndWait.graceMilliseconds)return undefined;
 for(const pair of policy.strictWaits??[]){
  if(!pair.requested.some(prefix=>request.resource.startsWith(prefix)))continue;
  const hold=held.filter(row=>!(row.directory===request.directory&&row.resource===request.resource)&&pair.held.some(prefix=>row.resource.startsWith(prefix))).sort((left,right)=>left.atMs-right.atMs)[0];
  if(hold)return violation("out-of-order",policy,hold,request,{waitedMilliseconds});
 }
 return undefined;
}

/** 🧭️ Checks one blocking request against what its task already holds: a request ranked before a lease already held breaks the declared order. Requests that cannot wait (timeout 0) are exempt. */
export function checkLeaseOrderV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request):LeaseViolationV1|undefined{
 if(request.nonblocking)return undefined;
 if(policy.rules.outOfOrder!=="off"){
  const wanted=classifyLeaseResourceV1(policy,request.resource);
  if(wanted.rank>0){
   const later=held.filter(row=>!isLeaseMediumV1(policy,row,request)).filter(row=>{const now=classifyLeaseResourceV1(policy,row.resource);return now.rank>wanted.rank&&!edgeAllowed(policy,now.id,wanted.id,row,request);}).sort((left,right)=>left.atMs-right.atMs)[0];
   if(later)return violation("out-of-order",policy,later,request);
  }
 }
 return undefined;
}

/** ⏳️ Decides whether a request that has already waited `waitedMilliseconds` must be refused because its task holds an exclusive lease others may need: the oldest such hold is named. Declared edges and requests that cannot wait are exempt. */
export function checkHoldAndWaitV1(policy:LeaseOrderPolicyV1,held:readonly HeldLeaseV1[],request:Request,waitedMilliseconds:number,blocker?:Readonly<{pid:number;heldSeconds:number}>):LeaseViolationV1|undefined{
 if(request.nonblocking||policy.rules.holdAndWait==="off"||waitedMilliseconds<policy.holdAndWait.graceMilliseconds)return undefined;
 const wanted=classifyLeaseResourceV1(policy,request.resource);
 const hold=held.filter(row=>row.mode==="exclusive"&&!(row.directory===request.directory&&row.resource===request.resource)&&!edgeAllowed(policy,classifyLeaseResourceV1(policy,row.resource).id,wanted.id,row,request)).sort((left,right)=>left.atMs-right.atMs)[0];
 return hold?violation("hold-and-wait",policy,hold,request,{waitedMilliseconds,...blocker?{blockerPid:blocker.pid,blockerHeldSeconds:blocker.heldSeconds}:{}}):undefined;
}

/** ⛓️ The participants a node waits for: every holder whose mode conflicts and, in a queue, every earlier conflicting waiter. */
export function leaseBlockersV1(nodes:readonly LeaseNodeV1[],node:LeaseNodeV1):readonly string[]{
 const wait=node.wait;if(!wait)return [];const found=new Set<string>();
 for(const other of nodes){
  if(other.id===node.id)continue;
  if(other.holds.some(hold=>hold.key===wait.key&&leaseModesConflictV1(hold.mode,wait.mode)))found.add(other.id);
  else if(wait.queued&&other.wait?.queued&&other.wait.key===wait.key&&(other.wait.sinceMs<wait.sinceMs||other.wait.sinceMs===wait.sinceMs&&other.id<node.id)&&leaseModesConflictV1(other.wait.mode,wait.mode))found.add(other.id);
 }
 return [...found].sort();
}

/** 🔒️ The participants a node's own process waits on for the node to finish: a task that holds a lease shared depends on a sibling task of the same process that is queued for the same lease shared, because that is a re-entrant request stuck behind writers that wait for the very hold; the process cannot finish either. A task that holds any lease also depends on every descendant task that waits: it awaits its child, so it releases nothing while the child is stuck; a child that waits for a lease its own ancestor holds is the self-deadlock rule's case, not a cycle. */
export function leaseProcessDependenciesV1(nodes:readonly LeaseNodeV1[],node:LeaseNodeV1):readonly string[]{
 const owner=node.id.slice(0,node.id.indexOf("/")),found=new Set<string>();
 for(const hold of node.holds)if(hold.mode==="shared")for(const other of nodes)if(other.id!==node.id&&other.id.slice(0,other.id.indexOf("/"))===owner&&other.wait?.queued&&other.wait.mode==="shared"&&other.wait.key===hold.key)found.add(other.id);
 if(node.holds.length)for(const other of nodes)if(other.id!==node.id&&other.wait&&other.ancestors?.includes(node.id)&&!node.holds.some(hold=>hold.key===other.wait!.key&&leaseModesConflictV1(hold.mode,other.wait!.mode)))found.add(other.id);
 return [...found].sort();
}

/** 🕸️ Finds a deadlock cycle through a node, if any. The victim is the participant that began waiting last (the highest id on a tie), so exactly one member fails and the others proceed. */
export function findLeaseCycleV1(nodes:readonly LeaseNodeV1[],from:string):LeaseCycleV1|undefined{
 const byId=new Map(nodes.map(node=>[node.id,node] as const)),path:string[]=[from],seen=new Set<string>([from]);
 if(!byId.has(from))return undefined;
 const walk=(id:string):boolean=>{
  for(const next of [...leaseBlockersV1(nodes,byId.get(id)!),...leaseProcessDependenciesV1(nodes,byId.get(id)!)]){
   if(next===from)return true;
   if(seen.has(next)||!byId.has(next))continue;
   seen.add(next);path.push(next);if(walk(next))return true;path.pop();
  }
  return false;
 };
 if(!walk(from))return undefined;
 const victim=[...path].filter(id=>byId.get(id)!.wait).sort((left,right)=>{const a=byId.get(left)!.wait!.sinceMs,b=byId.get(right)!.wait!.sinceMs;return b-a||(left<right?1:left>right?-1:0);})[0]!;
 return Object.freeze({members:Object.freeze([...path]),victim});
}

/** 🗣️ Picks the message language: `SEMIO_LANGUAGE`, then the locale, then English. */
export function leaseLanguageV1(environment:Readonly<Record<string,string|undefined>>=process.env):"en"|"de"{
 for(const value of [environment.SEMIO_LANGUAGE,environment.LC_ALL,environment.LC_MESSAGES,environment.LANG]){const tag=value?.toLowerCase().split(/[._@-]/u)[0];if(tag==="en"||tag==="de")return tag;}
 return "en";
}

const fill=(template:string,values:Readonly<Record<string,string|number>>):string=>template.replace(/\{([A-Za-z]+)\}/gu,(match,key:string)=>key in values?String(values[key]):match);
/** ✂️ Shortens a long resource name from the left so a diagnostic stays readable. */
export function shortLeaseResourceV1(resource:string):string{const limit=protocol.maximumReportedResourceCharacters;return resource.length>limit?"…"+resource.slice(-(limit-1)):resource;}
/** 🖨️ Renders a violation as a diagnostic in the chosen language. */
export function formatLeaseViolationV1(language:"en"|"de",row:LeaseViolationV1):string{
 const catalogue=schema.definitions.Messages.const,messages={outOfOrder:catalogue.outOfOrder[language],holdAndWait:catalogue.holdAndWait[language],selfDeadlock:catalogue.selfDeadlock[language],cycle:catalogue.cycle[language],blocker:catalogue.blocker[language]},values={held:shortLeaseResourceV1(row.held),heldClass:row.heldClass,heldRank:row.heldRank,heldMode:row.heldMode,requested:shortLeaseResourceV1(row.requested),requestedClass:row.requestedClass,requestedRank:row.requestedRank,requestedMode:row.requestedMode,seconds:Math.floor((row.waitedMilliseconds??0)/1000),cycle:(row.cycle??[]).join(" → "),blocker:row.blockerPid===undefined?"":fill(messages.blocker,{pid:row.blockerPid,heldSeconds:row.blockerHeldSeconds??0})};
 return fill(row.kind==="out-of-order"?messages.outOfOrder:row.kind==="hold-and-wait"?messages.holdAndWait:row.kind==="self-deadlock"?messages.selfDeadlock:messages.cycle,values);
}

const taskIds=new WeakMap<object,number>();
let nextTask=1;
/** 🧵️ Gives one logical task (the invocation behind a control, or a bare signal) a small stable number inside this process. */
export function leaseTaskIdV1(key:object):number{let id=taskIds.get(key);if(id===undefined){id=nextTask++;taskIds.set(key,id);}return id;}

type LeaseScope={readonly id:number;readonly parent?:LeaseScope};
const scopes=new AsyncLocalStorage<LeaseScope>();
/** ▶️ Runs a body as its own lease task, a child of the task running now: the child sees what its ancestors hold (it runs inside their extent) but never what a concurrent sibling holds, so two siblings waiting for each other's lease are an ordinary wait or a wait-for cycle, never a self-deadlock. */
export function withLeaseTaskV1<T>(body:()=>T):T{const parent=scopes.getStore();return scopes.run({id:nextTask++,...parent?{parent}:{}},body);}
/** 🌊️ The task running now followed by its ancestors, innermost first; undefined outside any scope. */
export function currentLeaseLineageV1():readonly number[]|undefined{const lineage:number[]=[];for(let scope=scopes.getStore();scope;scope=scope.parent)lineage.push(scope.id);return lineage.length?lineage:undefined;}

type Live={readonly id:number;readonly directory:string;readonly resource:string;readonly mode:LeaseModeV1;readonly atMs:number;readonly task:number};
type Waiting={readonly id:number;readonly directory:string;readonly resource:string;readonly mode:LeaseModeV1;readonly sinceMs:number;readonly task:number;readonly queued:boolean};
let nextId=1;
const holds=new Map<number,Live>(),waits=new Map<number,Waiting>(),handles=new WeakMap<object,()=>void>(),ancestors=new Map<number,readonly number[]>();
let inherited:readonly HeldLeaseV1[]|undefined,inheritedKey:string|undefined;

/** 👪️ The leases a parent process declared in `SEMIO_LEASE_INHERITED`; they count as held by every task of this process. */
export function inheritedLeasesV1(environment:Readonly<Record<string,string|undefined>>=process.env):readonly HeldLeaseV1[]{
 const text=environment[protocol.inheritedEnvironment]??"";
 if(inherited&&inheritedKey===text)return inherited;
 let rows:HeldLeaseV1[]=[];
 if(text){try{const value=JSON.parse(text);if(Array.isArray(value))rows=value.slice(0,protocol.maximumInherited).filter(row=>row&&typeof row.directory==="string"&&typeof row.resource==="string"&&(row.mode==="shared"||row.mode==="exclusive")).map(row=>Object.freeze({directory:row.directory,resource:row.resource,mode:row.mode,atMs:Number(row.atMs)||0,task:0}));}catch{rows=[];}}
 inherited=Object.freeze(rows);inheritedKey=text;return inherited;
}

/** 📤️ Adds what this process holds to a child's environment so the child's order checks start from the parent's position. */
export function leaseInheritanceEnvironmentV1(environment:Readonly<Record<string,string|undefined>>):Record<string,string|undefined>{
 const rows=[...inheritedLeasesV1(environment),...[...holds.values()].map(row=>({directory:row.directory,resource:row.resource,mode:row.mode,atMs:row.atMs,task:0}))].slice(0,protocol.maximumInherited);
 return rows.length===0?{...environment}:{...environment,[protocol.inheritedEnvironment]:JSON.stringify(rows)};
}

/** 🎒️ What a task holds: its own leases, those of its ancestor tasks and everything inherited from the parent process. */
export function heldLeasesV1(task:number,ancestors:readonly number[]=[]):readonly HeldLeaseV1[]{
 return [...inheritedLeasesV1(),...[...holds.values()].filter(row=>row.task===task||ancestors.includes(row.task))];
}

/** 📸️ The leases and waits of this process, for the registry. */
export function leaseLedgerSnapshotV1():Readonly<{holds:readonly Live[];waits:readonly Waiting[];ancestry:Readonly<Record<string,readonly number[]>>}>{const active=new Set<number>([...holds.values()].map(row=>row.task).concat([...waits.values()].map(row=>row.task))),ancestry:Record<string,readonly number[]>={};for(const task of active){const chain=ancestors.get(task);if(chain?.length)ancestry[task]=chain;}for(const task of [...ancestors.keys()])if(!active.has(task)&&ancestors.size>4096)ancestors.delete(task);return Object.freeze({holds:[...holds.values()],waits:[...waits.values()],ancestry:Object.freeze(ancestry)});}

/** 🔬️ Lets tests start from an empty ledger. */
export function resetLeaseLedgerV1():void{holds.clear();waits.clear();ancestors.clear();inherited=undefined;inheritedKey=undefined;}

/** 🧷️ Everything one acquisition needs: the early order check, the contention check while it waits, and the hold bookkeeping once it succeeds. */
export type LeaseOrderGuardV1=Readonly<{contention(waitedMilliseconds:number):void|Promise<void>;finish():void;hold(lease:{release():void}):void}>;
const NOOP:LeaseOrderGuardV1=Object.freeze({contention:()=>undefined,finish:()=>undefined,hold:()=>undefined});

const seen=new Set<string>();
function react(policy:LeaseOrderPolicyV1,rule:LeaseOrderRuleV1,row:LeaseViolationV1,directory:string):void{
 if(rule==="off")return;
 const recorded=Object.freeze({...row,enforced:rule==="enforce"}),language=leaseLanguageV1(),message=formatLeaseViolationV1(language,row);
 try{leaseRegistryV1.recordViolation(directory,recorded,policy);}catch{}
 if(rule==="enforce")throw new LeaseOrderErrorV1(message,recorded);
 const key=`${row.kind}\0${row.heldClass}\0${row.requestedClass}`;
 if(!seen.has(key)){seen.add(key);try{process.stderr.write(fill(schema.definitions.Messages.const.observed[language],{message})+"\n");}catch{}}
}

/** 🚦️ Begins one acquisition: runs the early checks, which throw under `enforce`, and returns the guard the wait loop and the success path use. */
export function beginLeaseOrderV1(request:Request&Readonly<{task:number;ancestors?:readonly number[];queued:boolean}>,environment:Readonly<Record<string,string|undefined>>=process.env):LeaseOrderGuardV1{
 if(environment[protocol.modeEnvironment]==="off")return NOOP;
 const policy=leaseOrderPolicyV1(environment),started=Date.now(),held=heldLeasesV1(request.task,request.ancestors),early=checkLeaseOrderV1(policy,held,request);
 if(request.ancestors?.length)ancestors.set(request.task,request.ancestors);
 if(early&&!(request.mode==="shared"&&held.some(row=>row.mode==="shared"&&row.directory===request.directory&&row.resource===request.resource)))react(policy,policy.rules.outOfOrder,early,request.directory);
 let waitId:number|undefined,lastCycle=0,finished=false;
 const finish=():void=>{if(finished)return;finished=true;if(waitId!==undefined){waits.delete(waitId);waitId=undefined;leaseRegistryV1.changed(request.directory,policy);}};
 const contention=async(waitedMilliseconds:number):Promise<void>=>{
  if(finished||request.nonblocking)return;
  if(waitId===undefined){waitId=nextId++;waits.set(waitId,Object.freeze({id:waitId,directory:request.directory,resource:request.resource,mode:request.mode,sinceMs:started,task:request.task,queued:request.queued}));leaseRegistryV1.changed(request.directory,policy,true);}
  const current=heldLeasesV1(request.task,request.ancestors),own=checkSelfDeadlockV1(policy,current,request,waitedMilliseconds);
  if(own)react(policy,policy.rules.selfDeadlock,own,request.directory);
  const now=Date.now();
  if(policy.rules.cycle!=="off"&&now-lastCycle>=policy.cycle.intervalMilliseconds){
   lastCycle=now;
   const found=leaseRegistryV1.findCycle(request.directory,`${process.pid}/${request.task}`);
   if(found&&found.victim===`${process.pid}/${request.task}`){
    const names=found.members.map(member=>leaseRegistryV1.describeNode(request.directory,member));
    react(policy,policy.rules.cycle,violation("cycle",policy,held[0]??{directory:request.directory,resource:request.resource,mode:request.mode,atMs:now,task:request.task},request,{waitedMilliseconds,cycle:names}),request.directory);
   }
  }
  const blocker=waitedMilliseconds>=policy.holdAndWait.graceMilliseconds?leaseRegistryV1.holderOf(request.directory,request.resource,request.mode):undefined;
  const refusal=checkHoldAndWaitV1(policy,current,request,waitedMilliseconds,blocker);
  if(refusal)react(policy,policy.rules.holdAndWait,refusal,request.directory);
  const strict=checkStrictWaitV1(policy,current,request,waitedMilliseconds);
  if(strict)react(policy,policy.rules.strictWait??"off",strict,request.directory);
 };
 const hold=(lease:{release():void}):void=>{
  finish();
  const id=nextId++,live:Live=Object.freeze({id,directory:request.directory,resource:request.resource,mode:request.mode,atMs:Date.now(),task:request.task});
  holds.set(id,live);leaseRegistryV1.changed(request.directory,policy);
  const release=lease.release;let open=true;
  const drop=():void=>{if(!open)return;open=false;holds.delete(id);leaseRegistryV1.held(request.directory,request.resource,request.mode,live.atMs);leaseRegistryV1.changed(request.directory,policy);};
  handles.set(lease,drop);
  lease.release=function(){try{release.call(lease);}finally{drop();}};
 };
 return Object.freeze({contention,finish,hold});
}

/** ♻️ Whether this process, the parent that started it or a build invocation that owns it (`SEMIO_BUILD_INVOCATION_PID`, such as the Bazel loader behind an action, or any pid of the chain `SEMIO_BUILD_INVOCATION_PIDS`) already holds a lease shared: a further shared request for it is re-entrant and must be granted at once, never queued behind writers that wait for the very hold. */
export function holdsSharedInProcessV1(directory:string,resource:string,environment:Readonly<Record<string,string|undefined>>=process.env):boolean{
 if([...holds.values()].some(row=>row.mode==="shared"&&row.directory===directory&&row.resource===resource)||inheritedLeasesV1(environment).some(row=>row.mode==="shared"&&row.directory===directory&&row.resource===resource))return true;
 const owners=[environment[protocol.invocationEnvironment],...(environment[protocol.invocationChainEnvironment]??"").split(",")].map(Number);
 return owners.some(owner=>Number.isSafeInteger(owner)&&owner>0&&owner!==process.pid&&leaseRegistryV1.holdsShared(directory,resource,owner));
}

/** 👥️ The other live processes that hold a lease of this resource in any mode, from the lease registry; an Nx daemon counts the clients attached to its namespace with it. */
export function otherLeaseHoldersV1(directory:string,resource:string):readonly number[]{return leaseRegistryV1.otherHolders(directory,resource);}

/** 🔗️ The chain of build invocations a child answers to, this process appended: a nested loader holds its own pinned graph snapshot, so the Nx client it starts must count the nested loader, not only the outermost invocation, as an owner whose shared hold it may join at once. */
export function leaseInvocationChainV1(environment:Readonly<Record<string,string|undefined>>=process.env,pid:number=process.pid):string{
 const chain=(environment[protocol.invocationChainEnvironment]??"").split(",").filter(owner=>/^[1-9][0-9]*$/u.test(owner));
 return [...new Set([...chain,String(pid)])].slice(-protocol.maximumInvocationChain).join(",");
}

/** 🔓️ Drops the bookkeeping of a lease that was consumed without its own release, such as a queued upgrade. */
export function forgetLeaseV1(lease:object):void{handles.get(lease)?.();}

leaseRegistryV1.attach({snapshot:leaseLedgerSnapshotV1 as never,cycle:findLeaseCycleV1});
