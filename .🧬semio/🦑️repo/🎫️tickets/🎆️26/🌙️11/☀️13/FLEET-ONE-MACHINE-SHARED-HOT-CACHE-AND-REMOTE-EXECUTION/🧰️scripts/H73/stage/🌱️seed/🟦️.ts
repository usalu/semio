import {createHash,randomBytes} from "node:crypto";
import {appendFileSync,closeSync,existsSync,fstatSync,mkdirSync,openSync,readFileSync,readSync,renameSync,rmSync,unlinkSync,writeFileSync} from "node:fs";
import {join,relative,sep} from "node:path";
import {decodeProtobufV1,encodeProtobufV1,REAPI_V1} from "../../../../../🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {declareV1} from "../../../../../🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {casRootV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {hasCasObjectSyncV1,readCasObjectSyncV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import type {FleetClientV1} from "../📡️client/🟦️.ts";
import {FLEET_POLICY_V1} from "../🪪️identity/🟦️.ts";
import {manifestIdV1,obtainManifestV1} from "../📋️manifest/🟦️.ts";
import {decodeActionResultV1} from "../📦️wire/🟦️.ts";

const seed=FLEET_POLICY_V1.seed,HEX=/^[a-f0-9]{64}$/u,sha256=(text:string|Uint8Array):string=>createHash("sha256").update(text).digest("hex");

/** 🌱️ The part of one unit of Cargo's compilation graph that names it independently of any checkout, machine or edit: where it comes from (a workspace member by its manifest below the workspace root, anything else by its source identity), its package, target, kind, operation, platform, features, effective profile and compiler. Structural, so the graph type of the launcher is not imported. */
export type SeedUnitV1=Readonly<{provenance:Readonly<{kind:string;identity:string}>;manifest:string;packageName:string;packageVersion:string;name:string;kind:string;operation:string;platform:string|null;features:readonly string[];profileDigest:string;compiler:Readonly<{digest:string}>|null}>;
/** 🌱️ The graph a Cargo request plans: its units and the workspace root. */
export type SeedGraphV1=Readonly<{workspaceRoot:string;units:readonly SeedUnitV1[]}>;

/** 🌱️ The fingerprint of a Cargo request: the sorted rows of its units (see {@link SeedUnitV1}), never a source byte, a path of this machine or a key, so every machine and every edit of one request gets the same fingerprint. */
export function seedFingerprintV1(graph:SeedGraphV1):string{
 const rows=graph.units.map(unit=>JSON.stringify([unit.provenance.kind,unit.provenance.kind==="workspace"?relative(graph.workspaceRoot,unit.manifest).split(sep).join("/"):unit.provenance.identity,unit.packageName,unit.packageVersion,unit.name,unit.kind,unit.operation,unit.platform??"",[...unit.features].sort(),unit.profileDigest,unit.compiler?.digest??""])).sort();
 return sha256(JSON.stringify(["semio-seed-fingerprint-v1",rows]));
}

/** 🆔️ The address of the closure record of a request: the fleet slot under which its versions are listed. */
export const seedClosureIdV1=(fingerprint:string):string=>sha256(seed.namespace+"\0"+fingerprint);

/** 📂️ Where a lane keeps what the seed prefetched and what its compiler wrappers observed. */
export const seedDirectoryV1=(laneRoot:string):string=>join(laneRoot,seed.directory);

const recorded=new Set<string>();
/** 📝️ A compiler wrapper (or build-script runner) notes the slot of a unit it looked up, once per process. A lane without a root, or a slot that is no digest, records nothing. Never throws. */
export function recordSeedSlotV1(laneRoot:string|undefined,slot:string):void{
 if(laneRoot===undefined||!HEX.test(slot)||recorded.has(laneRoot+"\0"+slot))return;
 recorded.add(laneRoot+"\0"+slot);
 try{const directory=seedDirectoryV1(laneRoot);mkdirSync(directory,{recursive:true});appendFileSync(join(directory,seed.slotsFile),slot+"\n");}catch{}
}

/** 📖️ The distinct slots the wrappers of a lane noted, in the order they first appeared. */
export function observedSeedSlotsV1(laneRoot:string):readonly string[]{
 try{return[...new Set(readFileSync(join(seedDirectoryV1(laneRoot),seed.slotsFile),"utf8").split("\n").filter(slot=>HEX.test(slot)))];}catch{return[];}
}

/** 📦️ The closure record as the fleet cache stores it: a blob (`{version, retention, files, env, slots}`, the input-manifest shape the server already retains, with no source so it never goes stale) and the action result that names it as `inputs.json` and `stage.json` and carries it inline, so a lookup is one request. The key is the digest of the closure id and the blob, so every version has its own key and the slot lists them newest first. */
export function seedRecordWireV1(closureId:string,slots:readonly string[],declaration:ReturnType<typeof declareV1>):Readonly<{blob:Uint8Array;hash:string;result:Uint8Array;key:string}>{
 const blob=Buffer.from(JSON.stringify({version:1,retention:declaration,files:[],env:[],slots})),hash=sha256(blob),digest={hash,size_bytes:blob.length};
 return Object.freeze({blob,hash,key:sha256(closureId+"\0"+hash),result:encodeProtobufV1(REAPI_V1.actionResult,{output_files:[{path:"inputs.json",digest},{path:"stage.json",digest}],stdout_raw:blob,exit_code:0})});
}

/** 📦️ The slots an action result carries; `undefined` for anything that is not of the closed shape. */
export function parseSeedRecordV1(bytes:Uint8Array):readonly string[]|undefined{
 try{
  const result=decodeProtobufV1(REAPI_V1.actionResult,bytes),value=JSON.parse(Buffer.from(result.stdout_raw as Uint8Array).toString("utf8")) as {version?:unknown;slots?:unknown};
  if(value.version!==1||!Array.isArray(value.slots)||value.slots.length>seed.maximumSlots||!value.slots.every(slot=>typeof slot==="string"&&HEX.test(slot)))return undefined;
  return Object.freeze([...new Set(value.slots as string[])]);
 }catch{return undefined;}
}

/** 🔎️ The newest closure record of a request: `slots`; `absent` when the fleet has none; `unknown` when it could not say. Never throws. */
export async function obtainSeedRecordV1(client:FleetClientV1,closureId:string):Promise<Readonly<{state:"found";slots:readonly string[]}|{state:"absent"|"unknown"}>>{
 try{
  if(!client.usable()||client.slotKeys===undefined)return{state:"unknown"};
  const rows=await client.slotKeys(closureId);if(rows===undefined)return{state:"unknown"};
  const newest=[...rows].sort((left,right)=>right.storedAtMs-left.storedAtMs)[0];if(newest===undefined)return{state:"absent"};
  const answer=await client.getResult(newest.key);if(answer==="miss")return{state:"absent"};if(typeof answer==="string")return{state:"unknown"};
  const slots=parseSeedRecordV1(answer);return slots===undefined?{state:"unknown"}:{state:"found",slots};
 }catch{return{state:"unknown"};}
}

/** 📮️ Publishes a closure record (blob, then result under the slot of the request). `stored`, or `unshared` when this machine may not write or the cache cannot be reached. Never throws. */
export async function publishSeedRecordV1(client:FleetClientV1,closureId:string,slots:readonly string[],declaration:ReturnType<typeof declareV1>):Promise<"stored"|"unshared">{
 try{
  if(!client.settings.upload||!client.usable()||slots.length===0||slots.length>seed.maximumSlots)return "unshared";
  const wire=seedRecordWireV1(closureId,slots,declaration);if(wire.blob.length>seed.maximumRecordBytes)return "unshared";
  if(await client.storeBlob(wire.hash,wire.blob)!=="stored")return "unshared";
  return await client.storeResult(wire.key,wire.result,closureId)==="stored"?"stored":"unshared";
 }catch{return "unshared";}
}

/** 📊️ What a seed did. `fetched` units have their action result and blobs in the lane (or already local), `absent` have no entry the fleet can serve, `blobs`/`bytes` count what was downloaded. */
export type SeedStatsV1={state:"seeded"|"no-record"|"off"|"cancelled"|"timeout";slots:number;done:number;fetched:number;absent:number;blobs:number;bytes:number;manifests:number;stopped:boolean;milliseconds:number};

const atomic=(file:string,bytes:Uint8Array):void=>{mkdirSync(join(file,".."),{recursive:true});const temporary=file+"."+randomBytes(3).toString("hex");writeFileSync(temporary,bytes);renameSync(temporary,file);};
const present=(directory:string,name:string):boolean=>existsSync(join(directory,name));

/** 🌱️ Seeds a lane before Cargo starts: reads the closure record of the request, then, `seed.concurrency` slots at a time, takes the unit manifest of each slot into the local store and, for the newest entry of the slot, its action result and the blobs this machine does not hold into the lane's seed directory, bounded by `seed.maximumBytes` and `seed.deadlineMilliseconds`. The wrappers then answer their fleet lookups from these files (see {@link seededClientV1}). Entries that turn out to be another key than the one a unit computes are never used and cost only their download; correctness never depends on the seed. With `prefetch:false` (a lane that already holds its units) only the record is read. Reports progress, ends at once when `signal` aborts, and never throws. */
export async function seedLaneV1(input:Readonly<{client:FleetClientV1|undefined;cacheRoot:string;laneRoot:string;closureId:string;signal?:AbortSignal;prefetch?:boolean;progress?:(stats:Readonly<SeedStatsV1>)=>void;casRoot?:string;nowMs?:()=>number}>):Promise<Readonly<{stats:SeedStatsV1;record:readonly string[]}>>{
 const now=input.nowMs??Date.now,began=now(),stats:SeedStatsV1={state:"off",slots:0,done:0,fetched:0,absent:0,blobs:0,bytes:0,manifests:0,stopped:false,milliseconds:0};
 const finish=(record:readonly string[]=[])=>{stats.milliseconds=now()-began;return{stats:{...stats},record};};
 const client=input.client;if(client===undefined)return finish();
 try{
  const found=await obtainSeedRecordV1(client,input.closureId);
  if(found.state!=="found"){stats.state="no-record";return finish();}
  stats.state="seeded";stats.slots=found.slots.length;if(input.prefetch===false)return finish(found.slots);
  const root=seedDirectoryV1(input.laneRoot),ac=join(root,seed.acDirectory),cas=join(root,seed.casDirectory),local=input.casRoot??casRootV1(),deadline=began+seed.deadlineMilliseconds;
  mkdirSync(ac,{recursive:true});mkdirSync(cas,{recursive:true});
  const over=()=>input.signal?.aborted===true||now()>=deadline;let next=0,reported=began;
  const tick=()=>{if(input.progress!==undefined&&now()-reported>=seed.progressMilliseconds){reported=now();input.progress({...stats});}};
  const one=async(slot:string):Promise<void>=>{
   const [manifest,rows]=await Promise.all([obtainManifestV1(input.cacheRoot,manifestIdV1(slot),client),client.slotKeys===undefined?undefined:client.slotKeys(slot)]);
   if(manifest.from==="fleet")stats.manifests++;
   const newest=rows===undefined?undefined:[...rows].sort((left,right)=>right.storedAtMs-left.storedAtMs)[0];
   if(newest===undefined||now()-newest.storedAtMs>seed.maximumKeyAgeMilliseconds){stats.absent++;return;}
   if(present(ac,newest.key)){stats.fetched++;return;}
   const answer=await client.getResult(newest.key);if(typeof answer==="string"){stats.absent++;return;}
   const result=decodeActionResultV1(answer),wanted=new Map<string,number>();
   for(const digest of[...result.files.map(file=>file.digest),...[result.stdout,result.stderr].filter((stream):stream is NonNullable<typeof stream>=>stream!==undefined)])wanted.set(digest.hash,digest.size);
   if(wanted.size>FLEET_POLICY_V1.limits.maximumBlobs+3||[...wanted.values()].some(size=>size>seed.maximumBlobBytes)){stats.absent++;return;}
   for(const [hash,size] of [...wanted.entries()].sort((left,right)=>right[1]-left[1])){
    if(over())return;
    if(present(cas,hash)||hasCasObjectSyncV1(local,hash))continue;
    if(stats.bytes+size>seed.maximumBytes){stats.stopped=true;return;}
    stats.bytes+=size;const blob=await client.getBlob(hash,size);
    if(blob===undefined){stats.bytes-=size;stats.absent++;return;}
    atomic(join(cas,hash),blob);stats.blobs++;
   }
   atomic(join(ac,newest.key),answer);stats.fetched++;
  };
  await Promise.all(Array.from({length:Math.min(seed.concurrency,found.slots.length)},async()=>{
   for(;;){
    if(over())return;
    const index=next++;if(index>=found.slots.length)return;
    try{await one(found.slots[index]!);}catch{stats.absent++;}
    stats.done++;tick();
   }
  }));
  if(input.signal?.aborted===true)stats.state="cancelled";else if(now()>=deadline&&stats.done<found.slots.length){stats.state="timeout";stats.stopped=true;}
  return finish(found.slots);
 }catch{return finish();}
}

const hexKey=(value:string):boolean=>HEX.test(value);
const mark=(laneRoot:string,line:string):void=>{try{appendFileSync(join(seedDirectoryV1(laneRoot),seed.usedFile),line+"\n");}catch{}};
const sizedAndDigested=(bytes:Uint8Array,hash:string,size:number):boolean=>bytes.length===size&&sha256(bytes)===hash;

/** 🪞️ The fleet client of a wrapper whose lane was seeded: an action result the seed holds is answered from its file, a blob from the seed's file (then deleted: the unit installs it into the local store) or from the local object store when this machine already holds the same bytes, each verified against its digest; everything else goes to the fleet as before. Blobs from the local store spare a download even without a seed. */
export function seededClientV1(client:FleetClientV1,laneRoot:string,casRoot:string=casRootV1()):FleetClientV1{
 const root=seedDirectoryV1(laneRoot),ac=join(root,seed.acDirectory),cas=join(root,seed.casDirectory);
 return Object.freeze<FleetClientV1>({
  ...client,
  async getResult(key){
   if(hexKey(key)){try{const bytes=new Uint8Array(readFileSync(join(ac,key)));mark(laneRoot,"ac "+key);return bytes;}catch{}}
   return client.getResult(key);
  },
  async getBlob(hash,size){
   if(hexKey(hash)){
    try{const file=join(cas,hash),bytes=new Uint8Array(readFileSync(file));if(sizedAndDigested(bytes,hash,size)){mark(laneRoot,"cas "+hash);try{unlinkSync(file);}catch{}return bytes;}}catch{}
    try{const stored=readCasObjectSyncV1(casRoot,hash);if(stored!==undefined){const bytes=new Uint8Array(stored);if(sizedAndDigested(bytes,hash,size)){mark(laneRoot,"local "+hash);return bytes;}}}catch{}
   }
   return client.getBlob(hash,size);
  },
 });
}

/** 🏁️ After the run: publishes the closure record when the run succeeded, this machine may write and the lane noted a slot the record lacks (the record only grows, newest slots first, bounded by `seed.maximumSlots`). `added` is how many slots were new. Never throws. */
export async function finishSeedV1(input:Readonly<{client:FleetClientV1|undefined;laneRoot:string;closureId:string;record:readonly string[];toolchain:string;succeeded:boolean}>):Promise<Readonly<{published:boolean;slots:number;added:number}>>{
 try{
  if(!input.succeeded||input.client===undefined)return{published:false,slots:input.record.length,added:0};
  const known=new Set(input.record),observed=observedSeedSlotsV1(input.laneRoot),fresh=observed.filter(slot=>!known.has(slot));
  if(fresh.length===0)return{published:false,slots:input.record.length,added:0};
  const merged=[...fresh,...input.record].slice(0,seed.maximumSlots),stored=await publishSeedRecordV1(input.client,input.closureId,merged,declareV1({kind:"input-manifest",toolchain:input.toolchain,sources:[],externs:[]}));
  return{published:stored==="stored",slots:merged.length,added:stored==="stored"?fresh.length:0};
 }catch{return{published:false,slots:input.record.length,added:0};}
}

/** 🧹️ Removes what a seed left in the lane. */
export function clearSeedV1(laneRoot:string):void{try{rmSync(seedDirectoryV1(laneRoot),{recursive:true,force:true});}catch{}}

/** 🧾️ What one run did with its units. `fleet` counts units installed from the fleet cache, `prefetched` of them answered from the lane seed; `compiled` units had a miss and no offload. */
export type SeedRunReportV1=Readonly<{total:number;local:number;fleet:number;prefetched:number;compiled:number;offloaded:number;scripts:number}>;

function tailRowsV1(file:string,bytes:number):readonly Record<string,unknown>[]{
 try{
  const descriptor=openSync(file,"r");
  try{
   const size=fstatSync(descriptor).size,start=Math.max(0,size-bytes),buffer=Buffer.alloc(size-start);readSync(descriptor,buffer,0,buffer.length,start);
   const lines=buffer.toString("utf8").split("\n");if(start>0)lines.shift();
   return lines.flatMap(line=>{try{return line===""?[]:[JSON.parse(line) as Record<string,unknown>];}catch{return[];}});
  }finally{closeSync(descriptor);}
 }catch{return[];}
}

/** 🧾️ Tallies the unit events of one lane since `sinceMs` (the lane's own event file and the offload log) and the seed's `used.log`. */
export function seedRunReportV1(input:Readonly<{cacheRoot:string;lane:string;laneRoot:string;sinceMs:number}>):SeedRunReportV1{
 const events=join(input.cacheRoot,UNIT_CACHE_POLICY_V1.layout.events),rows=[...tailRowsV1(join(events,input.lane+".jsonl"),seed.eventTailBytes),...tailRowsV1(join(events,"offload.jsonl"),seed.eventTailBytes)].filter(row=>row.lane===input.lane&&Number(row.atMs)>=input.sinceMs);
 const units=new Map<string,string>(),offloaded=new Set<string>();let scripts=0,off=0;
 for(const row of rows){
  const kind=String(row.kind);
  if(kind==="hit"||kind==="remote-hit"||kind==="miss")units.set(String(row.crate)+"\0"+String(row.key),kind);
  else if(kind==="script-hit")scripts++;
  else if(kind==="offload-run"){offloaded.add(String(row.key));if(!String(row.crate).endsWith("build script"))off++;}
 }
 let prefetched=0;try{prefetched=new Set(readFileSync(join(seedDirectoryV1(input.laneRoot),seed.usedFile),"utf8").split("\n").filter(line=>line.startsWith("ac "))).size;}catch{}
 const kinds=[...units.entries()],count=(kind:string)=>kinds.filter(([,value])=>value===kind).length;
 return Object.freeze({total:units.size,local:count("hit"),fleet:count("remote-hit"),prefetched,compiled:kinds.filter(([name,value])=>value==="miss"&&!offloaded.has(name.split("\0")[1]!)).length,offloaded:off,scripts});
}
