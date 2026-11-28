import {spawn} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {physicalSourceOriginV1,type PhysicalSourceModuleMetaV1} from "../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import {claimHeldV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {dropSpillObjectsV1,newestBuildRunV1,newestEntryV1,emitUnitEventV1,readSpilledEntryV1,readSpillObjectV1,touchSpilledEntryV1} from "../../💾️store/🟦️.ts";
import {packRunV1} from "../../🏗️buildscript/📦️run-wire/🟦️.ts";
import {canonicalOutputV1,STREAM_NAME_V1} from "../../🔏️key/🔗️sources/🟦️.ts";
import {casRootV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {readCasObjectSyncV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {pinCasV1,releaseCasPinsV1} from "../../../../../🏗️build/🗄️cas/📌️pins/🟦️.ts";
import {CAS_POLICY_V1} from "../../../../../🏗️build/🗄️cas/🧬️schema/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClientV1,type FleetVerdictV1} from "../📡️client/🟦️.ts";
import {remoteCacheFetchV1} from "../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {FLEET_POLICY_V1,portableRootsV1,portableTextV1,type PlatformV1,type ToolRootsV1} from "../🪪️identity/🟦️.ts";
import {adoptFleetUnitV1} from "../📥️adopt/🟦️.ts";
import {registerSpoolV1} from "../📮️registry/🟦️.ts";
import {spoolBoundV1,spoolDirectoryV1,spoolLengthV1} from "../📏️spool-size/🟦️.ts";
import {packUnitV1,type KeyComponentsV1,type RemoteUnitV1} from "../📦️wire/🟦️.ts";
import {countSkipV1,countUnitV1,noteSpoolV1,writeBuildCountersV1} from "../📊️counters/🟦️.ts";
import {publishRootHintByKeyV1} from "../🟦️.ts";
import {publishKeySamplesV1} from "../🔑️key-samples/🟦️.ts";

const fleet=FLEET_POLICY_V1,limits=fleet.limits,sha=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex"),decoder=new TextDecoder("utf-8",{fatal:true}),encoder=new TextEncoder();
type Environment=Readonly<Record<string,string|undefined>>;

/** 🧾️ One unit waiting for upload (`SpoolJob` of the schema). */
export type SpoolJobV1={version:1;key:string;localKey:string;slot:string;crate:string;cwd:string;workspace:string|null;laneRoot:string|null;owner:string;claimed:boolean;offer?:boolean;run?:boolean;tools?:ToolRootsV1;hint?:{neutralKey:string;kinds:string[]};keyComponents?:KeyComponentsV1;costMs:number;platform:PlatformV1;queuedMs:number;attempts?:number;notBeforeMs?:number;deferredBy?:string};

const offeredPath=(root:string,key:string)=>join(root,fleet.layout.state,"offered",key.slice(0,2),key);

/** 🏷️ Whether this machine already settled the unit with the fleet cache (uploaded it, found it there, or installed it from there). */
export function offeredV1(root:string,key:string):boolean{try{statSync(offeredPath(root,key));return true;}catch{return false;}}
/** 🏷️ Remembers that the unit is settled, so later local hits do not offer it again. */
export function markOfferedV1(root:string,key:string):void{try{const path=offeredPath(root,key);mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,"");}catch{}}

/** 📮️ Queues a unit for upload: one small file renamed into the spool, `false` (the unit is not queued; the caller waits for room or counts it) when the spool holds {@link spoolBoundV1} units already; offers of local hits may fill half of it, so they never crowd out a unit that was just compiled. Never waits for the network. */
export function enqueueUploadV1(root:string,job:SpoolJobV1,bound?:number):boolean{
 try{
  const directory=spoolDirectoryV1(root);mkdirSync(directory,{recursive:true});
  const waiting=readdirSync(directory).filter(name=>name.endsWith(".json")),limit=bound??spoolBoundV1(root);
  if(waiting.some(name=>name.endsWith("-"+job.key.slice(0,16)+".json")))return true;
  if(waiting.length>=(job.offer?limit/2:limit))return false;
  const temporary=join(directory,"."+randomBytes(6).toString("hex")+".tmp"),name=(job.offer?"offer-":String(job.queuedMs).padStart(15,"0")+"-")+job.key.slice(0,16)+".json";
  writeFileSync(temporary,JSON.stringify(job));renameSync(temporary,join(directory,name));return true;
 }catch{return false;}
}

/** 🔁️ Rewrites a waiting job under its own name (atomically): a job that must wait keeps its place and never needs a free slot, so back-pressure can never lose it. */
function rewriteJobV1(path:string,job:SpoolJobV1):boolean{
 try{const temporary=path+"."+randomBytes(4).toString("hex")+".tmp";writeFileSync(temporary,JSON.stringify(job));renameSync(temporary,path);return true;}catch{return false;}
}


const scriptOf=()=>{const path=fileURLToPath(new URL("../../📜️script.ts",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)));return existsSync(path)?path:undefined;};

/** 🚀️ Starts the upload worker when none is alive (`true` when it started one) and registers the root with the machine's sidecar, which starts the worker again whenever the spool holds jobs and none lives (a build that ends, a worker that is killed or crashes, a machine that restarts): one detached process per machine (a lock file with a heartbeat names it), without any inherited descriptor, so Cargo never waits for it and the compiler's exit is not delayed by uploads. A start storm is throttled per cache identity: the stamp names the identity of the last start, so a build whose credential, endpoint or generation changed within `uploader.spawnThrottleMilliseconds` of that start still hands the spool over to a successor at once, and the drainer of the old identity (which lives as long as any job waits) never keeps the lock. */
export function startUploaderV1(root:string,environment:Environment):boolean{
 try{
  const directory=join(root,fleet.layout.state);mkdirSync(directory,{recursive:true});
  const stamp=join(directory,fleet.layout.spawned),identityFile=join(directory,fleet.layout.uploaderIdentity),mine=readFleetSettingsV1(root,environment)?.identity;
  if(mine!==undefined)registerSpoolV1(root,environment,mine);
  let current:string|undefined;try{current=readFileSync(identityFile,"utf8");}catch{}
  const alive=claimHeldV1(directory,"uploader");
  const succeeding=alive&&mine!==undefined&&current!==mine;
  if(alive&&!succeeding)return false;
  try{if(Date.now()-statSync(stamp).mtimeMs<fleet.uploader.spawnThrottleMilliseconds&&readFileSync(stamp,"utf8")===(mine??""))return false;}catch{}
  writeFileSync(stamp,mine??"");if(mine!==undefined)writeFileSync(identityFile,mine);
  const script=scriptOf(),env:Record<string,string>={};
  for(const [name,value] of Object.entries(environment)){if(value!==undefined&&!/^(CARGO_MAKEFLAGS|MAKEFLAGS|MFLAGS)$/u.test(name))env[name]=value;}
  const child=spawn(process.execPath,[...(script?[script]:[]),"--semio-unit-cache","fleet-upload","--root",root,...succeeding?["--takeover"]:[]],{env,detached:true,stdio:"ignore",windowsHide:true});
  child.on("error",()=>{});child.unref();return true;
 }catch{return false;}
}

async function bounded<T>(items:readonly T[],concurrency:number,work:(item:T)=>Promise<boolean>):Promise<boolean>{
 let next=0,ok=true;
 await Promise.all(Array.from({length:Math.min(concurrency,items.length)},async()=>{for(;;){const index=next++;if(index>=items.length)return;if(!await work(items[index]!))ok=false;}}));
 return ok;
}

/** 🏁️ How one upload ended: `true` (stored), `"diverged"` (the fleet already holds this key with other bytes, a determinism bug or an attack; the caller adopts the fleet's copy), `"stale"` (the sources of the unit differ from the synchronized tree), `"refused"` (the server will never take these bytes: 400, 413), `"oversize"` (an object or the whole unit is beyond `limits.maximumBlobBytes` / `limits.maximumEntryBytes`, the largest entry the fleet cache admits), `"capacity"` (its store is full: 507), `"retry"` (network or server fault that can pass, or a local read that threw), `"gone"` (the local entry or one of its objects is no longer there), `false` (a unit that is never shared: an incremental session, a build script run that names a directory of this machine). */
export type UploadOutcomeV1=boolean|"diverged"|"stale"|"refused"|"oversize"|"capacity"|"retry"|"gone";

let bytesInFlight=0;const bytesWaiters:(()=>void)[]=[];

/** 🚦️ Runs one upload while the bytes all uploads of this process hold in memory stay within `limits.maximumUploadBytesInFlight` (a unit larger than the budget runs alone), so the sliding pool never holds several gigabyte-sized units at once. */
async function withUploadBytesV1<T>(bytes:number,work:()=>Promise<T>):Promise<T>{
 while(bytesInFlight>0&&bytesInFlight+bytes>limits.maximumUploadBytesInFlight)await new Promise<void>(done=>bytesWaiters.push(done));
 bytesInFlight+=bytes;
 try{return await work();}finally{bytesInFlight-=bytes;for(const wake of bytesWaiters.splice(0))wake();}
}

const RANK:Readonly<Record<FleetVerdictV1,number>>={stored:0,diverged:0,stale:0,down:1,busy:1,capacity:2,refused:3};

/** 📨️ Sends the blobs the server lacks and then the action result (which makes the server wake every waiter), keeping the claim alive meanwhile; the worst verdict of the blobs decides how the job ends. */
async function sendV1(client:FleetClientV1,job:SpoolJobV1,upload:Readonly<{blobs:ReadonlyMap<string,Uint8Array>;result:Uint8Array}>):Promise<UploadOutcomeV1>{
 const renewing=job.claimed?setInterval(()=>{void client.renew(job.key,job.owner,fleet.claim.ttlMilliseconds);},fleet.claim.heartbeatMilliseconds):undefined;renewing?.unref?.();
 try{
  let worst=("stored" as FleetVerdictV1);
  await bounded([...upload.blobs.keys()],limits.maximumUploadConcurrency,async hash=>{
   if(await client.hasBlob(hash)===true)return true;
   const verdict=await client.storeBlob(hash,upload.blobs.get(hash)!);
   if(RANK[verdict]>RANK[worst])worst=verdict;
   return verdict==="stored";
  });
  if(worst==="refused"||worst==="capacity")return worst;
  if(worst!=="stored")return "retry";
  const verdict=await client.storeResult(job.key,upload.result,job.slot);
  return verdict==="stored"?true:verdict==="diverged"||verdict==="stale"||verdict==="refused"||verdict==="capacity"?verdict:"retry";
 }finally{if(renewing)clearInterval(renewing);}
}

/** 📤️ Builds and uploads one spooled unit: reads the local entry and its objects (within the in-flight byte budget, {@link withUploadBytesV1}), turns every path into its token, uploads the blobs the server lacks and then the action result. See {@link UploadOutcomeV1} for the ends; `"retry"`, `"capacity"` queue the job again (see {@link retryJobV1}). */
export async function uploadUnitV1(client:FleetClientV1,root:string,job:SpoolJobV1,environment:Environment=process.env):Promise<UploadOutcomeV1>{
 const roots=portableRootsV1({cwd:job.cwd,workspace:job.workspace??undefined,...job.tools}),cas=casRootV1(environment);
 if(job.run===true)return uploadRunV1(client,root,job,roots,cas);
 const entry=newestEntryV1(root,job.localKey)??readSpilledEntryV1(root,job.localKey);if(!entry)return "gone";
 if(entry.session===true)return false;
 const declared=entry.outputs.reduce((sum,output)=>sum+(Number(output.bytes)||0),0);
 if(declared>limits.maximumEntryBytes||entry.outputs.some(output=>output.bytes>limits.maximumBlobBytes))return "oversize";
 return withUploadBytesV1(declared,()=>sendEntryV1(client,root,job,entry,roots,cas));
}

async function sendEntryV1(client:FleetClientV1,root:string,job:SpoolJobV1,entry:NonNullable<ReturnType<typeof newestEntryV1>>,roots:ReturnType<typeof portableRootsV1>,cas:string):Promise<UploadOutcomeV1>{
 const contents=new Map<string,Uint8Array>();
 let total=0,missing=false;
 const convert=(sha256:string,text:boolean,name=""):string|undefined=>{
  const bytes=readCasObjectSyncV1(cas,sha256)??readSpillObjectV1(root,sha256);if(!bytes){missing=true;return undefined;}
  const out=text?encoder.encode(portableTextV1(canonicalOutputV1(name,decoder.decode(bytes)),roots)):new Uint8Array(bytes);total+=out.length;
  if(out.length>limits.maximumBlobBytes||total>limits.maximumEntryBytes)return undefined;
  const hash=sha(out);contents.set(hash,out);return hash;
 };
 const failed=():UploadOutcomeV1=>missing?"gone":"oversize";
 try{
  const outputs:RemoteUnitV1["outputs"]=[];
  for(const output of entry.outputs){
   const hash=convert(output.sha256,output.template===true,output.name);if(hash===undefined)return failed();
   outputs.push({name:output.name,sha256:hash,bytes:contents.get(hash)!.length,executable:output.executable,...(output.template?{template:true}:{})});
  }
  const stream=(value:string|null|undefined):string|null|undefined=>value==null?null:convert(value,true,STREAM_NAME_V1);
  const stdout=stream(entry.stdout),stderr=stream(entry.stderr);if(stdout===undefined||stderr===undefined)return failed();
  const unit:RemoteUnitV1={version:1,key:job.key,localKey:job.localKey,slot:job.slot,crate:job.crate,platform:job.platform,createdMs:entry.createdMs,costMs:job.costMs,dependencies:entry.dependencies.map(row=>({path:portableTextV1(row.path,roots),sha256:row.sha256})),environment:entry.environment.map(row=>({name:row.name,value:row.value===null?null:portableTextV1(row.value,roots)})),outputs,stdout,stderr,...(job.keyComponents===undefined?{}:{keyComponents:job.keyComponents}),...(entry.retention===undefined?{}:{retention:entry.retention})};
  return await sendV1(client,job,packUnitV1(unit,contents));
 }catch{return "retry";}
}

/** 📤️ {@link uploadUnitV1} for the run of a build script: the local run entry and its objects become one manifest (`run.json` and one `f:` output per OUT_DIR file) with the roots of the machine as tokens. A run that still names a directory of this machine is not uploaded. */
async function uploadRunV1(client:FleetClientV1,root:string,job:SpoolJobV1,roots:ReturnType<typeof portableRootsV1>,cas:string):Promise<UploadOutcomeV1>{
 const entry=newestBuildRunV1(root,job.localKey);if(!entry)return "gone";
 const packed=packRunV1(entry,{key:job.key,localKey:job.localKey,slot:job.slot,platform:job.platform,costMs:job.costMs},roots,digest=>readCasObjectSyncV1(cas,digest)??readSpillObjectV1(root,digest));
 if(typeof packed==="string")return packed==="too-large"?"oversize":packed==="unreadable"?"gone":false;
 try{return await sendV1(client,job,packUnitV1(packed.unit,packed.contents));}catch{return "retry";}
}

/** 🔁️ The job to queue again after an upload that can still succeed (the server was busy, down or full), `undefined` once it is older than `uploader.maximumJobAgeMilliseconds`: a settled offer (the next drain first asks the server whether another machine stored the unit meanwhile) due after an exponentially growing delay that stops growing at `uploader.maximumBackoffMilliseconds`, without the claim (a waiting machine takes the unit over). The number of tries is no bound: the age is. */
export function retryJobV1(job:SpoolJobV1,now:number):SpoolJobV1|undefined{
 const attempts=(job.attempts??0)+1,policy=fleet.uploader,{deferredBy:_,...rest}=job;
 if(now-job.queuedMs>policy.maximumJobAgeMilliseconds)return undefined;
 return{...rest,claimed:false,offer:true,attempts,notBeforeMs:now+Math.min(policy.maximumBackoffMilliseconds,policy.retryBackoffMilliseconds*2**Math.min(attempts-1,16))};
}

/** 📌️ The host-store objects of a waiting unit (its outputs and streams), which the drainer pins so that no collector, sweep or capacity eviction takes a unit before it reached the fleet; empty for a build script run and for a unit whose entry is gone. */
function jobDigestsV1(root:string,job:SpoolJobV1):string[]{
 if(job.run===true)return[];
 const entry=newestEntryV1(root,job.localKey)??readSpilledEntryV1(root,job.localKey);if(!entry)return[];
 return[...entry.outputs.map(row=>row.sha256),entry.stdout,entry.stderr].filter((sha):sha is string=>typeof sha==="string");
}

/** 🕰️ How often the drainer marks the spilled entries of its waiting jobs as read: a third of the spill protection, so a unit that waits for a refusing or busy fleet cache is never evicted from the spill folder before its upload. */
const SPILL_TOUCH_MILLISECONDS=300000;

/** 🚰️ The summary of one drain. */
export type DrainResultV1=Readonly<{uploaded:number;failed:number;dropped:number;retried?:number;deferred?:number}>;

/** 🫧️ Gives the heap of an idle uploader back: a drain that ran a busy period leaves the buffers of its uploads as garbage the allocator keeps (an uploader that waited 9 hours for work held 2.1 GB of footprint, nearly all of it dirty and compressed), so one full collection and a shrink run when the spool becomes empty. */
const releaseHeapV1=():void=>{try{(globalThis as {Bun?:{gc(force:boolean):void;shrink?:()=>void}}).Bun?.gc(true);(globalThis as {Bun?:{shrink?:()=>void}}).Bun?.shrink?.();}catch{}};

/** 🚰️ Drains the spool until it has been empty for `idleMilliseconds` (default `uploader.idleMilliseconds`, `SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS` overrides; or once, with `idleMilliseconds` 0): one drainer per machine holds the lock (on an idle end it releases the lock, then clears the spawn stamp, then looks at the spool again: a unit queued while it was ending either is seen by it or finds neither a worker nor a throttle, so it is never left without a worker), takes units oldest first, uploads a few at a time in a sliding pool (`maximumUploadJobs`, so one slow upload never stalls the others), and lives as long as any job waits, so a cache that is down for minutes costs no unit. A job leaves the spool only when it is uploaded, settled by the fleet, refused for good, or too old; a job that must wait (cache down or busy or full) is rewritten in place with a later due time and keeps its slot. The claim of a unit that was not uploaded is released so a waiter can take over. A drainer lives with the cache identity it started with (endpoint, generation, token): `takeover` names a new identity in `uploader.identity`, the old drainer finishes its running uploads and leaves, and this one replaces it (the next compile with another identity does the same through {@link startUploaderV1}). */
export async function drainUploadsV1(root:string,environment:Environment,options:Readonly<{idleMilliseconds?:number;signal?:AbortSignal;takeover?:boolean}>={}):Promise<DrainResultV1|"busy">{
 let total:DrainResultV1|undefined;
 for(let pass=0;;pass++){
  const drained=await drainPassV1(root,environment,pass===0?options:{...options,takeover:false});
  if(drained==="busy")return total??"busy";
  total=total===undefined?drained.result:sumDrainV1(total,drained.result);
  if(!drained.idled||options.signal?.aborted)return total;
  rmSync(join(root,fleet.layout.state,fleet.layout.spawned),{force:true});
  if(spoolLengthV1(root)===0)return total;
 }
}

/** ➕️ The summary of several passes of one drain. */
const sumDrainV1=(left:DrainResultV1,right:DrainResultV1):DrainResultV1=>({uploaded:left.uploaded+right.uploaded,failed:left.failed+right.failed,dropped:left.dropped+right.dropped,...left.retried===undefined&&right.retried===undefined?{}:{retried:(left.retried??0)+(right.retried??0)},...left.deferred===undefined&&right.deferred===undefined?{}:{deferred:(left.deferred??0)+(right.deferred??0)}});

/** 🔂️ One pass of {@link drainUploadsV1} under the lock; `idled` when it ended because the spool was empty, so the caller looks at the spool again after the lock is released (a job queued while this pass was ending found the lock still held and started no worker; the releaser is the one that must see it). */
async function drainPassV1(root:string,environment:Environment,options:Readonly<{idleMilliseconds?:number;signal?:AbortSignal;takeover?:boolean}>):Promise<Readonly<{result:DrainResultV1;idled:boolean}>|"busy">{
 const settings=readFleetSettingsV1(root,environment);if(!settings)return "busy";
 const directory=join(root,fleet.layout.state);mkdirSync(directory,{recursive:true});
 const identityFile=join(directory,fleet.layout.uploaderIdentity),identity=()=>{try{return readFileSync(identityFile,"utf8");}catch{return undefined;}};
 let lock=tryClaimV1(directory,"uploader");
 if(!lock&&options.takeover===true){if(identity()!==settings.identity)try{writeFileSync(identityFile,settings.identity);}catch{}const until=Date.now()+fleet.uploader.takeoverMilliseconds;while(!lock&&Date.now()<until&&identity()===settings.identity){await new Promise(done=>setTimeout(done,500));lock=tryClaimV1(directory,"uploader");}}
 if(!lock)return "busy";
 const workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT,client=createFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:workspace||root})}),idle=options.idleMilliseconds??(Number(environment.SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS)||fleet.uploader.idleMilliseconds),spool=spoolDirectoryV1(root),policy=fleet.uploader,result:{uploaded:number;failed:number;dropped:number;retried?:number;deferred?:number}={uploaded:0,failed:0,dropped:0};
 const cas=casRootV1(environment),pinHolder="unit-spool-"+sha(encoder.encode(root)).slice(0,16),pinned=new Map<string,string[]>(),spilled=new Map<string,string>();let pinnedAt=0,touchedAt=0,pinsChanged=false,pinning:Promise<void>=Promise.resolve();
 const pin=(now:number):void=>{
  if(!(pinsChanged&&now-pinnedAt>=policy.pinRefreshMilliseconds)&&now-pinnedAt<Math.min(CAS_POLICY_V1.leaseMaxMs/3,SPILL_TOUCH_MILLISECONDS))return;
  pinnedAt=now;pinsChanged=false;
  const digests=[...pinned.keys()].sort().slice(0,policy.pinMaximumJobs).flatMap(name=>pinned.get(name)!);
  if(now-touchedAt>=SPILL_TOUCH_MILLISECONDS){touchedAt=now;for(const key of spilled.values())touchSpilledEntryV1(root,key);}
  pinning=pinning.then(()=>digests.length===0?releaseCasPinsV1(cas,pinHolder):pinCasV1(cas,CAS_POLICY_V1,pinHolder,digests,CAS_POLICY_V1.leaseMaxMs,now)).catch(()=>undefined);
 };
 let quietSince=Date.now(),listedAt=0,idled=false,busyBefore=false;const dueAt=new Map<string,number>(),inFlight=new Set<string>(),queued=new Set<string>(),running=new Set<Promise<boolean>>(),queue:string[]=[];
 const pause=(milliseconds:number)=>new Promise(done=>setTimeout(done,milliseconds));
 const refill=():number=>{
  let names:string[]=[];try{names=readdirSync(spool).filter(name=>name.endsWith(".json")).sort();}catch{}
  listedAt=Date.now();noteSpoolV1(workspace,names.length,spoolBoundV1(root));
  const now=listedAt,present=new Set(names);for(const name of dueAt.keys())if(!present.has(name))dueAt.delete(name);
  for(const name of pinned.keys())if(!present.has(name)){pinned.delete(name);spilled.delete(name);pinsChanged=true;}
  for(const name of names){
   if(inFlight.has(name)||queued.has(name))continue;
   let at=dueAt.get(name);
   if(at===undefined){try{const waiting=JSON.parse(readFileSync(join(spool,name),"utf8")) as SpoolJobV1;at=waiting.deferredBy!==undefined&&waiting.deferredBy!==settings.identity?0:waiting.notBeforeMs??0;if(!pinned.has(name)){pinned.set(name,jobDigestsV1(root,waiting));pinsChanged=true;if(waiting.run!==true&&readSpilledEntryV1(root,waiting.localKey))spilled.set(name,waiting.localKey);}}catch{at=0;}dueAt.set(name,at);}
   if(at<=now){queue.push(name);queued.add(name);}
  }
  pin(now);
  return names.length;
 };
 const start=(name:string):void=>{
  inFlight.add(name);
  const task=(async():Promise<boolean>=>{
   const path=join(spool,name);let job:SpoolJobV1;
   try{job=JSON.parse(readFileSync(path,"utf8")) as SpoolJobV1;}catch{rmSync(path,{force:true});result.dropped++;return false;}
   const started=Date.now(),event=(kind:"remote-publish"|"remote-skip",reason:string)=>emitUnitEventV1(root,{lane:"uploader",kind,crate:job.crate,key:job.key,milliseconds:Date.now()-started,waitedMs:Math.max(0,started-job.queuedMs),reason});
   const release=async()=>{if(job.claimed)await client.release(job.key,job.owner);};
   const finish=()=>{rmSync(path,{force:true});dueAt.delete(name);const spilled=readSpilledEntryV1(root,job.localKey);if(spilled)dropSpillObjectsV1(root,spilled);};
   const wait=(at:number,reason:string,deferredBy?:string)=>{
    const {deferredBy:_,...rest}=job;
    if(!rewriteJobV1(path,{...rest,claimed:false,notBeforeMs:at,...deferredBy===undefined?{}:{deferredBy}}))return;
    dueAt.set(name,at);result.deferred=(result.deferred??0)+1;event("remote-skip",reason);
   };
   if(started-job.queuedMs>policy.maximumJobAgeMilliseconds){finish();result.dropped++;countSkipV1(workspace,client.usable()?"upload-expired":"upload-expired-"+client.state());event("remote-skip","upload-expired");await release();return false;}
   if(!client.usable()){wait(started+Math.round(policy.retryBackoffMilliseconds*(0.5+Math.random())),"upload-deferred: "+client.state(),settings.identity);await release();return false;}
   if(job.hint!==undefined){
    const named=await publishRootHintByKeyV1(client,job.hint.neutralKey,job.hint.kinds);
    if(named===undefined){wait(started+policy.retryBackoffMilliseconds,"root-hint-deferred");return false;}
    if(!job.hint.kinds.every(kind=>(named as readonly string[]).includes(kind))||named.length!==job.hint.kinds.length){finish();result.dropped++;countSkipV1(workspace,"root-key-conflict");event("remote-skip","root-key-conflict: the fleet names other directories for this unit");return false;}
   }
   if(job.offer===true&&await client.hasResult(job.key)===true){markOfferedV1(root,job.key);finish();result.dropped++;event("remote-skip","already-stored");return true;}
   const outcome=await Promise.race([uploadUnitV1(client,root,job,environment),new Promise<"retry">(done=>{const timer=setTimeout(()=>done("retry"),policy.jobMilliseconds);timer.unref?.();})]);
   if(outcome===true){markOfferedV1(root,job.key);finish();result.uploaded++;countUnitV1(workspace,"stored");event("remote-publish","uploaded");return true;}
   if(outcome==="retry"||outcome==="capacity"){
    const again=client.usable()||outcome==="capacity"?retryJobV1(job,Date.now()):{...job,claimed:false,notBeforeMs:Date.now()+policy.retryBackoffMilliseconds,deferredBy:settings.identity};
    if(again!==undefined&&rewriteJobV1(path,again)){dueAt.set(name,again.notBeforeMs??0);result.retried=(result.retried??0)+1;event("remote-skip",(outcome==="capacity"?"upload-capacity: the fleet cache is full, ":"upload-retry: ")+"next try in "+Math.round(((again.notBeforeMs??0)-Date.now())/1000)+" s (try "+(again.attempts??0)+")");await release();return false;}
    finish();result.dropped++;countSkipV1(workspace,"upload-expired");event("remote-skip","upload-expired");await release();return false;
   }
   finish();result.failed++;await release();
   if(outcome==="stale"){countSkipV1(workspace,"stale-on-arrival");event("remote-skip","stale-on-arrival: the fleet cache refused the unit, its sources differ from the synchronized tree");}
   else if(outcome==="diverged"){const adopted=await adoptFleetUnitV1(client,root,job);countSkipV1(workspace,"diverged");event("remote-skip",adopted?"diverged: the fleet holds other bytes for this key, its copy replaces the local entry":"diverged: the fleet holds other bytes for this key and its copy could not be fetched");}
   else if(outcome==="refused"){countSkipV1(workspace,"upload-refused");event("remote-skip","upload-refused: the fleet cache will never take these bytes");}
   else if(outcome==="oversize"){countSkipV1(workspace,"upload-oversize");event("remote-skip","upload-oversize: the unit is larger than the largest entry the fleet cache admits");}
   else if(outcome==="gone"){countSkipV1(workspace,"upload-entry-gone");event("remote-skip","upload-entry-gone: the local entry or one of its objects was removed before the upload");}
   else if(outcome===false){countSkipV1(workspace,"upload-unshareable");event("remote-skip","upload-unshareable: an incremental session or a run that names a directory of this machine is never shared");}
   else{countSkipV1(workspace,"upload-failed");event("remote-skip","upload-failed");}
   return false;
  })().catch(()=>false);
  running.add(task);void task.finally(()=>{running.delete(task);inFlight.delete(name);});
 };
 try{
  for(;;){
   if(options.signal?.aborted)break;
   if(options.idleMilliseconds!==0){const named=identity();if(named!==undefined&&named!==settings.identity)break;}
   if(running.size===0&&client.state()==="down"){if(idle===0){result.deferred=(result.deferred??0)+spoolLengthV1(root);break;}quietSince=Date.now();await pause(policy.waitingPollMilliseconds);continue;}
   if(Date.now()-listedAt>=policy.pinRefreshMilliseconds)refill();
   while(running.size<limits.maximumUploadJobs&&queue.length>0){const name=queue.shift()!;queued.delete(name);start(name);}
   if(running.size>=limits.maximumUploadJobs){await Promise.race([...running,pause(policy.waitingPollMilliseconds)]);continue;}
   if(queue.length===0&&Date.now()-listedAt>=(running.size>0?policy.pollMilliseconds*5:0)){
    const files=refill();
    if(files>0||running.size>0){quietSince=Date.now();busyBefore=true;}
    if(queue.length>0)continue;
    if(running.size===0){
     if(files===0){if(Date.now()-quietSince>=idle){idled=true;break;}if(busyBefore){busyBefore=false;releaseHeapV1();}await pause(policy.pollMilliseconds);continue;}
     if(idle===0)break;
     await pause(policy.waitingPollMilliseconds);continue;
    }
   }
   await Promise.race([...running,pause(policy.pollMilliseconds)]);
  }
 }finally{
  await Promise.allSettled([...running]);
  await pinning;await releaseCasPinsV1(cas,pinHolder).catch(()=>undefined);
  if(workspace)await publishKeySamplesV1(client,{root,workspace});
  lock.release();if(workspace){noteSpoolV1(workspace,spoolLengthV1(root),spoolBoundV1(root),true);writeBuildCountersV1(workspace);}
 }
 return {result,idled};
}
