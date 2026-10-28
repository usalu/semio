import {existsSync,readFileSync,readdirSync,statSync} from "node:fs";
import {join,resolve} from "node:path";
import {readLeaseReportV1,type LeaseHolderV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/📋️status/🟦️.ts";
import {leaseRegistryRootV1,readLeaseHistoryV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts";
import {leaseHistoryKeyV1} from "../../../../../../../🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts";
import {describeHostReasonV1,hostCapacityDirectoryV1,observeTreeMegabytesV1,readHostCapacityStatusV1,readHostEstimateStoreV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🟦️.ts";
import {agentSessionRegistryRootV1,agentSessionStoreV1,parseAgentSessionAddressV1,readAgentSessionV1} from "../../../../../../../🔨️modules/🏃️process/🪪️agent-sessions/🗄️store/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1,createCapacityControlV1} from "../../🌡️host-capacity/🟦️.ts";
import {cacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {machinePolicyV1,planMachineBudgetV1,resolveMachineTotalV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";
import {cacheRunnerPortsV1,machineHostFreeBytesV1,machineUsesV1,reportCacheLedgerV1} from "../../🧹️cache-gc/🏃️runner/🟦️.ts";
import {repositoryCachePartitionRootDataV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import {bazelServerDirectoryV1,listBazelSlotsV1,readBazelServerPidSyncV1,readBazelSlotStateSyncV1} from "../../🔧️tools/🏗️bazel/🧵️sessions/📖️status/🟦️.ts";
import sessionSchema from "../../🔧️tools/🏗️bazel/🧵️sessions/🧬️schema/🔣️.json" with {type:"json"};
import {implementationModeV1,readSealedBlockedV1,readSealedPointerV1,readSealedReceiptV1,sealedEntryOfV1,sealedPointerOfV1,sealedStoreDirectoryV1,sealedTreeDriftV1} from "../../🏛️sealed/🟦️.ts";
import generationSchema from "../../🕸️graph/🕰️generations/🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {fleetTextV1} from "../🟦️.ts";
import type {FleetBazelSlotFactV1,FleetCapacityWaiterFactV1,FleetFactsV1,FleetGrantFactV1,FleetHolderFactV1,FleetLanguageV1,FleetSealedEntryFactV1,FleetSourceV1,FleetStandingFactV1,FleetWaiterFactV1} from "../🟦️.ts";

/** 📥️ The readers of the fleet status: each one asks an existing status reader of the build (leases, capacity, Bazel slots, cache ledger, sealed store, graph generation pointer) and turns the answer into the facts the aggregation reads. A reader never takes a lease, never loads the graph and never writes; a reader that fails is reported in `sources` and leaves its section out.
 * @see ../🟦️.ts the aggregation, ../🧬️schema/🔣️.json the facts */
const policy=schema.definitions.Policy.const;
export const REPOSITORY_LEASE_DIRECTORY_FLEET_V1=".cache/b/l/m/data";
/** 🔌️ What a reader may use from its surroundings; tests replace it. */
export type FleetReaderContextV1=Readonly<{root:string;environment:NodeJS.ProcessEnv;language:FleetLanguageV1;now:()=>number}>;
export type FleetReaderV1=(context:FleetReaderContextV1)=>Promise<Partial<FleetFactsV1>>;
export type FleetReadersV1=Readonly<Record<FleetSourceV1["id"],FleetReaderV1>>;

const standingOf=(row:NonNullable<LeaseHolderV1["standing"]>):FleetStandingFactV1=>({state:row.state,...row.priority===undefined?{}:{priority:row.priority},...row.phase===undefined?{}:{phase:row.phase},...row.done===undefined?{}:{done:row.done},...row.total===undefined?{}:{total:row.total},...row.etaMinutes===undefined?{}:{etaMinutes:row.etaMinutes},...row.etaOverdue?{etaOverdue:true}:{},...row.implementationDigest===undefined?{}:{implementationDigest:row.implementationDigest},...row.implementationAgeSeconds===undefined?{}:{implementationAgeSeconds:row.implementationAgeSeconds},...row.parentPid===undefined?{}:{parentPid:row.parentPid},...row.chain===undefined?{}:{chain:[...row.chain]},...row.silentSeconds===undefined?{}:{silentSeconds:row.silentSeconds},...row.staleIds===undefined?{}:{staleIds:[...row.staleIds]}});

/** 🔒️ Leases: holders, waiters and cycles of the checkout's lease directory, with the history of the kinds of hold the waiters queue behind. */
export const readFleetLeasesV1:FleetReaderV1=async context=>{
 const directory=resolve(join(context.root,REPOSITORY_LEASE_DIRECTORY_FLEET_V1)),report=readLeaseReportV1(directory,context.environment,context.now(),0),registry=leaseRegistryRootV1(context.environment);
 const holders:FleetHolderFactV1[]=report.holders.map(row=>({pid:row.pid,resource:row.resource,mode:row.mode,class:row.class,heldSeconds:row.heldSeconds,...row.command===undefined?{}:{command:row.command},...row.session===undefined?{}:{session:row.session},...row.agent===undefined?{}:{agent:row.agent},...row.standing===undefined?{}:{standing:standingOf(row.standing)}}));
 const waiters:FleetWaiterFactV1[]=report.waiters.map(row=>({pid:row.pid,resource:row.resource,mode:row.mode,class:row.class,waitedSeconds:row.waitedSeconds,blockers:[...row.blockers],...row.position===undefined?{}:{position:row.position},...row.command===undefined?{}:{command:row.command},...row.session===undefined?{}:{session:row.session},...row.agent===undefined?{}:{agent:row.agent},...row.priority===undefined?{}:{priority:row.priority},historyKey:leaseHistoryKeyV1(row.resource,row.command??"")}));
 const history:Record<string,readonly number[]>={};
 for(const row of waiters){const key=row.historyKey!;if(!(key in history))history[key]=readLeaseHistoryV1(registry,directory,key);}
 return {leases:{holders,waiters,cycles:report.cycles.map(cycle=>cycle.members.join(" → "))},history};
};

const agentOf=(environment:NodeJS.ProcessEnv,address:string|undefined,cache:Map<string,string|undefined>):string|undefined=>{
 if(address===undefined)return undefined;
 if(cache.has(address))return cache.get(address);
 const parsed=parseAgentSessionAddressV1(address),found=parsed===undefined?undefined:readAgentSessionV1(agentSessionStoreV1(agentSessionRegistryRootV1(environment),parsed.worktree),parsed.id)?.agent.id;
 cache.set(address,found);return found;
};

const agentField=(environment:NodeJS.ProcessEnv,address:string|undefined,cache:Map<string,string|undefined>):{agent?:string}=>{const agent=agentOf(environment,address,cache);return agent===undefined?{}:{agent};};

/** 🌡️ Capacity: the host sample, the live grants (reserved against really used), the queue in decision order and what earlier runs of each operation took. */
export const readFleetCapacityV1:FleetReaderV1=async context=>{
 const control=createCapacityControlV1();
 try{
  const directory=hostCapacityDirectoryV1(context.environment),policy=BUILD_HOST_CAPACITY_POLICY_V1,snapshot=await readHostCapacityStatusV1({directory,policy,control:control.control,now:context.now}),now=context.now(),agents=new Map<string,string|undefined>();
  const grants:FleetGrantFactV1[]=snapshot.grants.map(row=>({id:row.id,operation:row.operation,pid:row.pid,jobs:row.jobs,reservedMegabytes:row.reservedMegabytes,observedMegabytes:row.observedMegabytes,heldSeconds:Math.max(0,Math.floor((now-row.admittedAtMs)/1000)),...row.phase===undefined?{}:{phase:row.phase},...row.lentSinceMs===undefined?{}:{lent:true},...row.session===undefined?{}:{session:row.session},...agentField(context.environment,row.session,agents)}));
  const order=new Map(snapshot.evaluation.decisions.map((row,index)=>[row.id,index] as const)),reasons=new Map(snapshot.evaluation.decisions.map(row=>[row.id,row.reason] as const));
  const waiters:FleetCapacityWaiterFactV1[]=[...snapshot.waiters].sort((left,right)=>(order.get(left.request.id)??Infinity)-(order.get(right.request.id)??Infinity)||left.arrivedAtMs-right.arrivedAtMs).map((row,index)=>{
   const reason=reasons.get(row.request.id)??"queue-order",text=reason==="fits"||reason==="reduced"||reason==="override"?reason:describeHostReasonV1(context.language,reason);
   return {id:row.request.id,operation:row.request.operation,pid:row.pid,needMegabytes:row.request.fixedMegabytes+row.request.minimumJobs*row.request.perJobMegabytes,waitedSeconds:Math.max(0,Math.floor((now-row.arrivedAtMs)/1000)),reason,reasonText:text,position:index+1,...row.session===undefined?{}:{session:row.session},...agentField(context.environment,row.session,agents)};
  });
  const store=readHostEstimateStoreV1(directory),wanted=new Set([...grants.map(row=>row.operation),...waiters.map(row=>row.operation)]),active:Record<string,readonly number[]>={};
  for(const operation of wanted){const samples=store.classes[operation]?.samples;if(samples)active[operation]=samples.map(row=>row.activeMilliseconds).reverse();}
  const host=snapshot.host;
  return {host:{platform:host.platform,cores:host.cores,totalMegabytes:host.totalMegabytes,availableMegabytes:host.availableMegabytes,pressure:host.pressure,swapUsedMegabytes:host.swapUsedMegabytes,load:host.load},capacity:{floorMegabytes:snapshot.evaluation.floorMegabytes,headroomMegabytes:snapshot.evaluation.headroomMegabytes,capacityMegabytes:Math.max(0,host.totalMegabytes-snapshot.evaluation.floorMegabytes),grants,waiters},activeMilliseconds:active};
 }finally{control.dispose();}
};

/** 🏗️ Bazel: every slot with an output base, the server process living in it and its memory, and what the slot remembers. */
export const readFleetBazelV1:FleetReaderV1=async context=>{
 const server=bazelServerDirectoryV1(repositoryCachePartitionRootDataV1(context.root,"products")),slots:(Omit<FleetBazelSlotFactV1,"treeMegabytes">&{treeMegabytes:Promise<number|null>|null})[]=[];
 for(const slot of listBazelSlotsV1(server)){
  const pid=readBazelServerPidSyncV1(join(server,slot)),state=readBazelSlotStateSyncV1(server,slot);
  slots.push({slot,pid:pid??null,treeMegabytes:pid===undefined?null:observeTreeMegabytesV1(pid).catch(()=>null),...state===undefined?{}:{key:state.key,usedAt:state.usedAt,...state.session===undefined?{}:{session:state.session},...state.agent===undefined?{}:{agent:state.agent}}});
 }
 const trees=await Promise.all(slots.map(row=>row.treeMegabytes));
 return {bazel:{slotPrefix:sessionSchema.definitions.Policy.const.leases.slotPrefix,slots:slots.map((row,index)=>({...row,treeMegabytes:trees[index]!}))}};
};

/** 💾️ Storage: the images and areas of the cache ledger from statfs and the size ledger (no tree is walked) and the machine budget plan. */
export const readFleetStorageV1:FleetReaderV1=async context=>{
 const ledger=cacheLedgerV1(),ports=cacheRunnerPortsV1({signal:new AbortController().signal,progress:async()=>{},exclusive:async()=>undefined}),report=await reportCacheLedgerV1(context.root,ledger,ports),declared=machinePolicyV1();
 const fileText=(()=>{try{return readFileSync(join(context.root,...ledger.roots.repository.directory.split("/"),declared.file),"utf8");}catch{return undefined;}})(),total=resolveMachineTotalV1({environment:context.environment,...fileText===undefined?{}:{fileText}},declared);
 const plan=planMachineBudgetV1({totalBytes:total.bytes,hostFreeBytes:machineHostFreeBytesV1(context.root),uses:machineUsesV1(report,report.roots.reduce((sum,row)=>sum+row.usedBytes,0),null)},declared);
 return {storage:{hostFreeBytes:machineHostFreeBytesV1(context.root),hostFreeFloorBytes:declared.hostFreeFloorBytes,images:report.partitions.map(row=>({id:row.id,kind:row.kind,state:row.state,usedBytes:row.usedBytes,capacityBytes:row.capacityBytes,budgetBytes:row.budgetBytes,source:row.source})),overAreas:report.areas.filter(row=>row.over).map(row=>row.id),machine:{totalBytes:plan.totalBytes,effectiveTotalBytes:plan.effectiveTotalBytes,usedBytes:plan.usedBytes,footprintBytes:plan.footprintBytes,overBytes:plan.overBytes,evictBytes:plan.evictBytes,blockedBytes:plan.blockedBytes,source:total.source}}};
};

const promotionOf=(path:string):FleetSealedEntryFactV1["last"]=>{
 try{const lines=readFileSync(path,"utf8").split("\n").filter(Boolean),row=JSON.parse(lines[lines.length-1]!) as {atMs?:unknown;added?:unknown;removed?:unknown;changed?:unknown};return Number.isSafeInteger(row.atMs)&&Number.isSafeInteger(row.added)&&Number.isSafeInteger(row.removed)&&Number.isSafeInteger(row.changed)?{atMs:row.atMs as number,added:row.added as number,removed:row.removed as number,changed:row.changed as number}:undefined;}catch{return undefined;}
};
/** 🏛️ Sealed implementation: the mode, and for every entry of the store its pointer, whether the live tree still equals the tree it was captured from (stat only, nothing is hashed unless a stamp moved), a blocked promotion and the last promotion. */
export const readFleetSealedV1:FleetReaderV1=async context=>{
 const store=sealedStoreDirectoryV1(context.root),entries:FleetSealedEntryFactV1[]=[];
 let names:string[]=[];try{names=readdirSync(store,{withFileTypes:true}).filter(row=>row.isDirectory()).map(row=>row.name).sort();}catch{}
 for(const name of names){
  let pointer;try{pointer=sealedPointerOfV1(JSON.parse(readFileSync(join(store,name,"current.json"),"utf8")));}catch{pointer=undefined;}
  if(!pointer)continue;
  const entry=sealedEntryOfV1(context.root,pointer.entry),current=readSealedPointerV1(entry);if(!current)continue;
  const receipt=readSealedReceiptV1(entry,current.digest),drift=receipt?.tree?sealedTreeDriftV1(context.root,receipt.tree):undefined,blocked=readSealedBlockedV1(entry),last=promotionOf(join(entry.directory,"promotions.jsonl")),parts=current.entry.split("/");
  entries.push({entry:parts.slice(-3,-1).join("/")||current.entry,digest:current.digest,files:current.files,bytes:current.bytes,promotedAtMs:current.promotedAtMs,fresh:drift===undefined?null:drift.fresh,changed:drift?.changed.length??0,...blocked===undefined?{}:{blocked:{atMs:blocked.atMs,rows:blocked.rows.length,...blocked.rows[0]===undefined?{}:{first:`${blocked.rows[0].path.split("/").slice(-2).join("/")}:${blocked.rows[0].line}:${blocked.rows[0].column} ${blocked.rows[0].message}`.slice(0,160)}}},...last===undefined?{}:{last}});
 }
 return {sealed:{mode:implementationModeV1(context.environment),entries}};
};

const popcount=(base64:string):number=>{let count=0;for(const byte of Buffer.from(base64,"base64")){let value=byte;while(value){count+=value&1;value>>=1;}}return count;};
/** 🕰️ Generation: the published pointer and manifest of the graph store, read as plain files under their schema (the owner table of the manifest is counted, not walked), with how many owners the last verification covered. The graph itself is never loaded. */
export const readFleetGenerationV1:FleetReaderV1=async context=>{
 const defs=generationSchema.definitions,policy=defs.Policy.const as {pointer:string;manifests:string},directory=join(repositoryCachePartitionRootDataV1(context.root,"graph"),"generations");
 if(!existsSync(join(directory,policy.pointer)))return {};
 const pointer=JSON.parse(readFileSync(join(directory,policy.pointer),"utf8")) as {generation:number;manifest:string|null;verified?:{atMs:number;count:number;owners:string}};
 const bad=validateJsonSchemaSubset(defs.Pointer as never,pointer,generationSchema as never);if(bad.length)throw Error(`generation pointer: ${JSON.stringify(bad[0])}`);
 if(pointer.manifest===null)return {};
 const path=join(directory,policy.manifests,pointer.manifest+".json"),manifest=JSON.parse(readFileSync(path,"utf8")) as {owners:Record<string,unknown>;implementation:string;built:{kind:"cold"|"delta"|"refresh";dirty:readonly string[]}};
 const shape=validateJsonSchemaSubset(defs.Manifest as never,{...manifest,owners:{}},generationSchema as never);if(shape.length)throw Error(`generation manifest: ${JSON.stringify(shape[0])}`);
 return {generation:{number:pointer.generation,built:{kind:manifest.built.kind,dirty:manifest.built.dirty.length},owners:Object.keys(manifest.owners).length,implementation:manifest.implementation,publishedAtMs:Math.floor(statSync(path).mtimeMs),verified:pointer.verified===undefined?null:{atMs:pointer.verified.atMs,covered:popcount(pointer.verified.owners),count:pointer.verified.count}}};
};

/** 🏭️ The readers of the real workspace. */
export const FLEET_READERS_V1:FleetReadersV1=Object.freeze({leases:readFleetLeasesV1,capacity:readFleetCapacityV1,bazel:readFleetBazelV1,storage:readFleetStorageV1,sealed:readFleetSealedV1,generation:readFleetGenerationV1});

/** 🧺️ Runs every reader at once, times it and merges what it returned; a reader that throws is a failed source, never a failed command. */
export async function collectFleetFactsV1(context:FleetReaderContextV1,readers:FleetReadersV1=FLEET_READERS_V1,timeoutMilliseconds:number=policy.readers.timeoutMilliseconds):Promise<FleetFactsV1>{
 const ids=Object.keys(readers) as FleetSourceV1["id"][],sources:FleetSourceV1[]=[],parts:Partial<FleetFactsV1>[]=[];
 await Promise.all(ids.map(async id=>{
  const started=performance.now(),limit=timeoutMilliseconds;
  let timer:ReturnType<typeof setTimeout>|undefined;
  try{parts.push(await Promise.race([readers[id](context),new Promise<never>((_resolve,reject)=>{timer=setTimeout(()=>reject(Error(fleetTextV1(context.language,"error.timeout",{seconds:limit/1000}))),limit);})]));sources.push({id,ok:true,milliseconds:Math.round(performance.now()-started)});}
  catch(error){sources.push({id,ok:false,milliseconds:Math.round(performance.now()-started),error:String((error as Error).message??error).slice(0,200)});}
  finally{if(timer!==undefined)clearTimeout(timer);}
 }));
 const merged:Record<string,unknown>={};
 for(const part of parts)Object.assign(merged,part);
 return Object.freeze({...merged,version:1,atMs:context.now(),workspace:context.root,sources:ids.map(id=>sources.find(row=>row.id===id)!)}) as FleetFactsV1;
}
