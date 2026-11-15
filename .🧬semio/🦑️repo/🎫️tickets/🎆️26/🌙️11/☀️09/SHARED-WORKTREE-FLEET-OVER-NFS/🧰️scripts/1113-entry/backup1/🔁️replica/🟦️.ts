import {createHash,randomBytes} from "node:crypto";
import {createReadStream,lstatSync,readlinkSync,watch,type Stats} from "node:fs";
import {chmod,copyFile,link,lstat,mkdir,mkdtemp,open,opendir,readdir,readFile,readlink,rename,rm,rmdir,stat,symlink,truncate,unlink,utimes,writeFile} from "node:fs/promises";
import {tmpdir,uptime} from "node:os";
import {basename,dirname,join,relative,resolve,sep} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {appendEventsV1,blobPathV1,copyVerifiedV1,touchBlobV1,largeBlobBytesV1,publishFlatHeadV1,readFlatHeadsV1,storeBlobV1,ensureStoreDirectoryV1,foldEventsV1,markStoreSeenV1,formatHlcV1,listOriginsV1,mergeClockV1,parseHlcV1,readEventSpansV1,reclaimStagingV1,readHeadV1,readNewestSnapshotV1,requireOriginIdV1,streamMembersV1,storePathsV1,tickClockV1,winsV1,writeBlobV1,writeSnapshotV1,type FleetClockV1,type FleetCursorV1,type FleetEventV1,type FleetHeadV1,type FleetManifestEntryV1} from "../📒️journal/🟦️.ts";
import {compileIgnoreV1,foldNameV1,gitPrivateV1,hashFileV1,linkRefusalV1,refusePathV1,scanSharedV1,sharedV1,type FleetIgnoreV1,type FleetScanEntryV1} from "../🚧️boundary/🟦️.ts";
import {compactionDefaultsV1,cursorAheadV1,estimateEventsBehindV1,listSnapshotsV1,planRetirementV1,pruneSnapshotsV1,reclaimBlobsV1,reclaimLitterV1,requireCompactionShapeV1,retireSegmentsV1,scanSnapshotV1,snapshotSavesV1,storeReportV1,withCompactionLockV1,writeCompactionRecordV1,type CompactionSettingsV1,type ReclaimResultV1,type SnapshotRefV1,type StoreReportV1} from "../🗜️compaction/🟦️.ts";
import {digestHexV1,foldTermV1,manifestTermV1,createAttesterV1,fleetReportV1,readAttestationV1,readAttestationsV1,readRetirementV1,readRetirementsV1,repairableDriftV1,type AttesterGitV1,type AttesterViewV1,type FleetFallbackV1,type FleetGuardsV1,type FleetReportV1} from "../🩺️attest/🟦️.ts";
import {applyIndexV1,branchRefV1,linearizeRefV1,gitGuardsV1,gitStateV1,gitViewV1,resolveGitV1,INDEX_BASE_V1,INDEX_PREFIX_V1,indexRealmV1,indexSignatureV1,normaliseGitConfigV1,publishIndexV1,stageHealthV1,verifyIndexV1,type IndexHostV1,type StageReasonV1} from "../🌿️git/🟦️.ts";
import {DirectoryTableV1,HotSetV1,LatencyV1,nextDelayV1,takeHintsV1,ensureHintsV1,type LatencyReportV1} from "../🎣️tiers/🟦️.ts";
import {StateCorruptV1,appendWalV1,bytesOfV1,checkpointPartsV1,decodeRecordsV1,partWidthV1,payloadsV1,readCheckpointV1,readWalV1,writeCheckpointV1,type CheckpointV1,type WalRecordV1} from "./💾️state/🟦️.ts";
import {clashKeyV1,clashV1,staleRepublishV1,type ContentEventV1} from "./⚖️clash/🟦️.ts";
import {awaitWorktreeAccessV1,readAccessV1,type AccessRecordV1} from "../🔐️access/🟦️.ts";
import {codeDigestV1,codeRootV1,type RestartCheckV1} from "../🩺️attest/🟦️.ts";
import {codeDirectoryV1,createFailuresV1,createStoreGuardV1,importClosureV1,installBootStageV1,listStoreCodeV1,moduleSourcesV1,pruneLostV1,pruneStoreCodeV1,publishCodeSnapshotV1,readBeatV1,readHealthV1,StoreStalledV1,yieldingV1,readRunHistoryV1,recordRefusedV1,recordRunV1,rotateLogV1,runSelfCheckV1,sealCodeV1,suspectCodeV1,writeHealthV1,type FailuresV1,type HealthV1,type SealedCodeV1} from "../🛡️resilience/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🎚️ The explicit inputs of every replica operation: both directories, the identity of this machine and the tunables that default to the topology's `replica` block. `modeless` (default: Windows) says that the volume cannot represent the executable bit: such a replica never reads the bit from disk, keeps the bit of the manifest entry when a file changes, takes it from the staged index entry for a new file and never writes it. */
export type ReplicaOptionsV1=Readonly<{worktree:string;storeRoot:string;originId:string;groupId:string;pollMilliseconds?:number;scanSeconds?:number;debounceMilliseconds?:number;retentionDays?:number;concurrency?:number;persistMilliseconds?:number;modeless?:boolean;lock?:boolean;now?:()=>number;attestMilliseconds?:number;restartStableMilliseconds?:number;codeRoot?:string;gitState?:(worktree:string,signal?:AbortSignal)=>Promise<AttesterGitV1>;gitGuards?:(worktree:string,signal?:AbortSignal)=>Promise<FleetGuardsV1|null>;guardsMilliseconds?:number;guardsMinimumMilliseconds?:number;gitView?:(worktree:string,signal?:AbortSignal)=>Promise<AttesterViewV1|null>;viewMilliseconds?:number;viewDebounceMilliseconds?:number;watch?:boolean;hintMilliseconds?:number;hotMilliseconds?:number;sweepMilliseconds?:number;walkSeconds?:number;watchRoot?:string;selfCheck?:(digest:string,signal?:AbortSignal)=>Promise<RestartCheckV1>;selfCheckMilliseconds?:number;restartJitterMilliseconds?:number;minimumUptimeMilliseconds?:number;storeDeadlineMilliseconds?:number;storeProbe?:(storeRoot:string)=>Promise<boolean>;storeGate?:()=>Promise<void>;logFiles?:readonly string[];compaction?:CompactionSettingsV1;fallback?:FleetFallbackV1;admit?:(digest:string,signal?:AbortSignal)=>Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>;publishCodeMilliseconds?:number;codeKeep?:number;returnMilliseconds?:number;driftMilliseconds?:number;opened?:(replica:ReplicaV1)=>void}>;

/** 📡️ One NDJSON progress line, shaped by `ProgressEvent` of the schema. */
export type ReplicaEventV1=Readonly<{version:1;event:"started"|"stopped"|"scan"|"progress"|"published"|"applied"|"overwritten"|"kept"|"linearised"|"repaired"|"state-rebuilt"|"state-missing"|"refused"|"skipped"|"blocked"|"unstable"|"store-unreachable"|"store-reachable"|"ignore-reloaded"|"clock-refused"|"clock-skew"|"blob-missing"|"rejoin-required"|"relayed"|"snapshot"|"compacted"|"sync"|"restarting"|"restart-refused"|"code-reverted"|"journal-repaired"|"retired"|"superseded"|"stale-republish"|"drift"|"error";stage?:string;path?:string;origin?:string;originId?:string;groupId?:string;hlc?:string;reason?:string;message?:string;lost?:string;winner?:string;loser?:string;result?:string;count?:number;done?:number;total?:number;bytes?:number;seconds?:number}>;

/** 🚰️ Receives progress events; it must not throw. */
export type ReplicaSinkV1=(event:ReplicaEventV1)=>void;

type Say=(event:Omit<ReplicaEventV1,"version">)=>void;
type Settings=Readonly<{worktree:string;storeRoot:string;originId:string;groupId:string;pollMilliseconds:number;scanSeconds:number;debounceMilliseconds:number;retentionDays:number;concurrency:number;persistMilliseconds:number;modeless:boolean;now:()=>number;compaction:CompactionSettingsV1}>;
type StateEntry=FleetManifestEntryV1&Readonly<{mtimeMs:number;seenMs?:number}>;
type Machine=Readonly<{version:1;originId:string;groupId:string;storeRoot:string;key?:string}>;
type Drift=Readonly<{path:string;kind:"missing-local"|"extra-local"|"content"|"target"|"state";expected?:string;actual?:string}>;

/** 🔁️ An opened replica: the loaded local state, the folded store view and the single-daemon lock; the loops mutate it in place and `closeReplicaV1` persists and unlocks. */
export type ReplicaV1={
 readonly settings:Settings;
 readonly paths:LocalPathsV1;
 readonly book:ReplicaBookV1;
 readonly files:CompactTableV1;
 readonly queue:ReplicaQueueV1;
 readonly manifest:CompactTableV1;
 readonly cursors:Record<string,FleetCursorV1>;
 readonly folds:ReplicaFoldsV1;
 readonly noted:Set<string>;
 readonly failures:FailuresV1;
 readonly key:string;
 applying:number;
 readonly changed:Set<number>;
 readonly held:Map<string,number>;
 readonly hot:HotSetV1;
 readonly warm:HotSetV1;
 readonly dirs:DirectoryTableV1;
 readonly latency:LatencyV1;
 readonly startedMs:number;
 readonly say:Say;
 readonly release:()=>Promise<void>;
 clock:FleetClockV1;
 ignore:FleetIgnoreV1;
 ignoreHash:string;
 dirty:boolean;
 persistedAt:number;
 homeWritten:boolean;
 readonly memo:Map<string,Uint8Array>;
 readonly memoDirty:Set<string>;
 generation:number;
 logBytes:number;
 checkpointBytes:number;
 joined:boolean;
 activity:number;
};

/** 🗄️ The local state files below `<worktree>/.cache/f/`. */
export type LocalPathsV1=Readonly<{directory:string;machine:string;checkpoint:string;wal:string;takeoverState:string;takeoverLog:string;cursor:string;clock:string;lost:string;tmp:string;lock:string;sync:string;gitignore:string;hints:string}>;

/** 🚶️ What one walk of the tree saw: the entries that differ from the recorded state (the only ones a publish needs), a bit per known path that was seen, the number of shared entries and the refused paths. */
export type WalkV1=Readonly<{changed:ReplicaFoundV1;seen:Readonly<{has(id:number):boolean}>;universe:number;scanned:number;refused:readonly Readonly<{path:string;reason:string}>[]}>;

/** 🧾️ What one publish round did: events written, files whose observation was refreshed without an event, paths that were unstable and should be retried. */
export type PublishResultV1=Readonly<{scanned:number;published:number;deleted:number;touched:number;bytes:number;retry:readonly string[]}>;

/** 📬️ What one follow round did: events read, paths materialised, local edits that were newer than the remote event and were published instead, paths still waiting. */
export type FollowResultV1=Readonly<{events:number;applied:number;dropped:number;kept:number;queued:number}>;

/** 🧳️ What a join materialised and what is still queued. */
export type JoinResultV1=Readonly<{files:number;adopted:number;queued:number}>;

/** 🗃️ The store folded to one manifest: the newest snapshot plus every journal event after it. */
export type StoreFoldV1=Readonly<{manifest:CompactTableV1;cursors:Record<string,FleetCursorV1>;hlc:string}>;

/** 🧭️ One disagreement between the local tree, the local state and the folded store. */
export type DriftV1=Drift;

/** 📋️ The report of `verifyV1`, shaped by `VerifyReport` of the schema. */
export type VerifyReportV1=Readonly<{version:1;ok:boolean;checked:number;total:number;truncated:boolean;drift:readonly Drift[];refusedTotal:number;refused:readonly Readonly<{path:string;reason:string}>[]}>;

/** 📊️ The report of `statusV1`, shaped by `Status` of the schema. */
export type StatusReportV1=Readonly<{access?:AccessRecordV1;version:1;originId:string;groupId:string;storeRoot:string;reachable:boolean;daemon:Readonly<{running:boolean;pid:number|null;busySeconds?:number}>;code?:Readonly<{source:"worktree"|"snapshot"|"store";phase:"boot"|"open"|"run"}>;sync:boolean;files:number;queue:Readonly<{incoming:number;outgoing:number|null}>;lost:number;refused:number|null;git:Readonly<{shared:boolean;reason:"shared"|"no-git"|"no-repository"|StageReasonV1;indexTree:string|null}>;origins:readonly Readonly<{origin:string;behindEvents:number;behindBytes:number;lastHlc:string|null;ageSeconds:number|null;rejoinRequired:boolean;retired:boolean}>[];latency:LatencyReportV1|null;retired:Readonly<{by:string;reason:string;wallMs:number}>|null;health:HealthV1|null;store:StoreReportV1|null}>;

/** 🎞️ A written snapshot. */
export type SnapshotResultV1=Readonly<{file:string;hlc:string;entries:number;created:boolean}>;

/** 🧹️ What a compaction removed. */
export type CompactResultV1=Readonly<{snapshot:SnapshotResultV1|undefined;segments:number;blobs:number;snapshots:number;staging:number;bytes:number}>;

/** 🚚️ What one relay round copied. */
export type RelayResultV1=Readonly<{origins:number;blobs:number;segments:number;bytes:number}>;

/** 🛰️ The inputs of the relay loop: the own store, the read-only peer stores by group id and the poll interval. */
export type RelayOptionsV1=Readonly<{ownStore:string;peers:Readonly<Record<string,string>>;pollMilliseconds?:number;concurrency?:number}>;

const originPattern=/^[a-z0-9][a-z0-9-]{0,39}$/u,segmentPattern=/^[0-9]{12}\.jsonl$/u,hexPattern=/^[0-9a-f]{64}$/u,chunkSize=256,driftLimit=1000,futureMilliseconds=60_000,racyMilliseconds=2000,staleStagingMilliseconds=6*3_600_000,collectBatch=2048,walLimitBytes=8*1024*1024,compatWindowMilliseconds=7*86_400_000,checkpointChanges=50_000,paceEntries=2000,heapRounds=20,relayConcurrency=8,stateBytesPerEntry=320,staleLockMilliseconds=60_000,noteCapacity=4096,healthMilliseconds=5000,maintenanceMilliseconds=3_600_000,lostEntries=2000,logLimitFileBytes=64*1024*1024,logKeepBytes=4*1024*1024;
const code=(error:unknown):string|undefined=>(error as {code?:string}).code;
const gone=(error:unknown):boolean=>code(error)==="ENOENT"||code(error)==="ENOTDIR";
const sharing=(error:unknown):boolean=>code(error)==="EBUSY"||code(error)==="EPERM"||code(error)==="EACCES";
const messageOf=(error:unknown):string=>error instanceof Error?error.message:String(error);
const digest=(text:string):string=>createHash("sha256").update(text).digest("hex");
const token=():string=>randomBytes(8).toString("hex");
const sleep=(milliseconds:number):Promise<void>=>new Promise(done=>{setTimeout(done,milliseconds);});
const absolute=(root:string,path:string):string=>join(root,...path.split("/"));
const listed=async(root:string,path:string):Promise<boolean>=>(await readdir(dirname(absolute(root,path))).catch(()=>[] as string[])).some(name=>name.normalize("NFC")===basename(path));
const spelled=async(root:string,path:string):Promise<boolean>=>(await readdir(dirname(absolute(root,path))).catch(()=>[] as string[])).includes(basename(path));
const asciiOnly=/^[\u0000-\u007f]*$/u;
const segmentName=(segment:number):string=>`${String(segment).padStart(12,"0")}.jsonl`;
const plain=(entry:FleetManifestEntryV1):FleetManifestEntryV1=>entry.operation==="put"?{hlc:entry.hlc,operation:"put",sha256:entry.sha256!,bytes:entry.bytes!,executable:entry.executable!}:entry.operation==="link"?{hlc:entry.hlc,operation:"link",target:entry.target!}:{hlc:entry.hlc,operation:"delete"};
const sealed=(path:string):boolean=>path===".cache/f"||path.startsWith(".cache/f/");
export const guarded=(ignore:FleetIgnoreV1):FleetIgnoreV1=>({digest:ignore.digest,ignores:(path,isDirectory)=>sealed(path)||ignore.ignores(path,isDirectory),child:(path,isDirectory)=>sealed(path)||(ignore.child??ignore.ignores)(path,isDirectory)});
const racy=(entry:StateEntry):boolean=>entry.seenMs===undefined||entry.seenMs-entry.mtimeMs<racyMilliseconds;
const unsafePath=(path:string):boolean=>path===""||path.split("/").some(part=>part===""||part==="."||part==="..")||path.includes("\\")||path.startsWith("/");

function widen<T extends Uint8Array|Uint16Array|Uint32Array|Float64Array|Int32Array>(array:T,length:number):T{
 const next=new (array.constructor as new(length:number)=>T)(length);
 next.set(array);
 if(next.byteLength>=settleBytes)queueMicrotask(settleHeap);
 return next;
}

const settleBytes=2<<20;
const hexValue=(code:number):number=>code>=48&&code<=57?code-48:code>=97&&code<=102?code-87:-1;

const hashText=(text:string):number=>{
 let hash=2166136261;
 for(let index=0;index<text.length;index++){hash^=text.charCodeAt(index);hash=Math.imul(hash,16777619);}
 return hash>>>0;
};

const encoder=new TextEncoder(),pageBytes=1<<20;

/** 📇️ The path table of one replica: every path ever seen gets one small integer id, and all tables of the replica store their columns by that id. The paths themselves live as UTF-8 in 1 MiB pages behind an open-addressing hash table of ids, so a path costs its bytes and a few table words, not a JavaScript string, and growing never copies the text. */
export class ReplicaBookV1{
 readonly origins:string[]=[];
 readonly originIds=new Map<string,number>();
 private lastOrigin=-1;
 count=0;
 private pages:Buffer[]=[];
 private used=pageBytes;
 private starts:Float64Array=new Float64Array(1025);
 private lengths:Uint16Array=new Uint16Array(1024);
 private hashes:Uint32Array=new Uint32Array(1024);
 private folded:Uint32Array=new Uint32Array(1024);
 private slots:Int32Array=new Int32Array(2048);
 private scratch=new Uint8Array(1024);
 private encode(path:string):number{
  if(this.scratch.length<path.length*3)this.scratch=new Uint8Array(path.length*3);
  return encoder.encodeInto(path,this.scratch).written;
 }
 private hashOf(length:number):number{
  let hash=2166136261;
  for(let index=0;index<length;index++){hash^=this.scratch[index]!;hash=Math.imul(hash,16777619);}
  return hash>>>0;
 }
 private matches(id:number,length:number):boolean{
  if(this.lengths[id]!==length)return false;
  const start=this.starts[id]!,page=this.pages[Math.floor(start/pageBytes)]!,from=start%pageBytes;
  for(let index=0;index<length;index++)if(page[from+index]!==this.scratch[index])return false;
  return true;
 }
 private locate(hash:number,length:number):number{
  const mask=this.slots.length-1;
  let slot=hash&mask;
  for(;;){
   const held=this.slots[slot]!;
   if(held===0)return slot;
   const id=held-1;
   if(this.hashes[id]===hash&&this.matches(id,length))return slot;
   slot=(slot+1)&mask;
  }
 }
 find(path:string):number|undefined{
  const length=this.encode(path),held=this.slots[this.locate(this.hashOf(length),length)]!;
  return held===0?undefined:held-1;
 }
 intern(path:string):number{
  const length=this.encode(path),hash=this.hashOf(length),slot=this.locate(hash,length),held=this.slots[slot]!;
  if(held!==0)return held-1;
  if(length>=pageBytes||!path.isWellFormed())throw Error("Closed replica path required");
  const id=this.count;
  if(this.pages.length===0||this.used+length>pageBytes){this.pages.push(Buffer.alloc(pageBytes));this.used=0;}
  const page=this.pages[this.pages.length-1]!;
  page.set(this.scratch.subarray(0,length),this.used);
  if(id+2>this.starts.length){
   const size=Math.ceil(this.starts.length*1.5);
   this.starts=widen(this.starts,size);this.lengths=widen(this.lengths,size-1);this.hashes=widen(this.hashes,size-1);this.folded=widen(this.folded,size-1);
  }
  this.starts[id]=(this.pages.length-1)*pageBytes+this.used;this.lengths[id]=length;this.hashes[id]=hash;this.folded[id]=hashText(foldNameV1(path));
  this.used+=length;this.count=id+1;
  this.slots[slot]=id+1;
  if(this.count*2>this.slots.length)this.rehash();
  return id;
 }
 private rehash():void{
  const slots=new Int32Array(this.slots.length*2),mask=slots.length-1;
  for(let id=0;id<this.count;id++){
   let slot=this.hashes[id]!&mask;
   while(slots[slot]!==0)slot=(slot+1)&mask;
   slots[slot]=id+1;
  }
  this.slots=slots;
 }
 reserve(count:number):void{
  if(count+2>this.starts.length){
   this.starts=widen(this.starts,count+2);this.lengths=widen(this.lengths,count+1);this.hashes=widen(this.hashes,count+1);this.folded=widen(this.folded,count+1);
  }
  let size=this.slots.length;
  while(size<count*2+2)size*=2;
  if(size>this.slots.length){this.slots=new Int32Array(size);for(let id=0;id<this.count;id++){let slot=this.hashes[id]!&(size-1);while(this.slots[slot]!==0)slot=(slot+1)&(size-1);this.slots[slot]=id+1;}}
 }
 foldHash(id:number):number{return this.folded[id]!;}
 columns():Readonly<{starts:Float64Array;lengths:Uint16Array;hashes:Uint32Array;folded:Uint32Array;pages:readonly Buffer[];used:number}>{
  const count=this.count;
  return {starts:this.starts.subarray(0,count),lengths:this.lengths.subarray(0,count),hashes:this.hashes.subarray(0,count),folded:this.folded.subarray(0,count),pages:this.pages,used:this.used};
 }
 adopt(columns:Readonly<{starts:Float64Array;lengths:Uint16Array;hashes:Uint32Array;folded:Uint32Array;pages:Buffer[];used:number}>,count:number,origins:readonly string[]):void{
  this.starts=columns.starts;this.lengths=columns.lengths;this.hashes=columns.hashes;this.folded=columns.folded;this.pages=columns.pages;this.used=columns.used;this.count=count;
  this.origins.splice(0,this.origins.length,...origins);this.originIds.clear();origins.forEach((name,index)=>{this.originIds.set(name,index);});this.lastOrigin=-1;
  let size=2048;
  while(size<count*2+2)size*=2;
  this.slots=new Int32Array(size);
  for(let id=0;id<count;id++){let slot=this.hashes[id]!&(size-1);while(this.slots[slot]!==0)slot=(slot+1)&(size-1);this.slots[slot]=id+1;}
 }
 name(id:number):string{
  const start=this.starts[id]!,from=start%pageBytes;
  return this.pages[Math.floor(start/pageBytes)]!.toString("utf8",from,from+this.lengths[id]!);
 }
 originOf(hlc:string):number{
  const last=this.lastOrigin;
  if(last>=0){const name=this.origins[last]!;if(hlc.length-21===name.length&&hlc.startsWith(name,21))return last;}
  return this.lastOrigin=this.origin(hlc.slice(21));
 }
 origin(name:string):number{
  let id=this.originIds.get(name);
  if(id===undefined){
   if(this.origins.length>=65536)throw Error("Closed replica origin count required");
   id=this.origins.length;this.originIds.set(name,id);this.origins.push(name);
  }
  return id;
 }
}

/** 🧊️ A `Map<path, entry>` stored as typed columns instead of one object and two strings per path: hlc as milliseconds, counter and origin index, sha256 as 32 raw bytes, sizes and times as numbers. `get` builds a plain entry on demand; the `*At(id)` readers never allocate. */
export class CompactTableV1{
 flags:Uint8Array=new Uint8Array(0);
 ms:Float64Array=new Float64Array(0);
 counter:Uint32Array=new Uint32Array(0);
 origin:Uint16Array=new Uint16Array(0);
 bytes:Float64Array=new Float64Array(0);
 sha:Uint8Array=new Uint8Array(0);
 mtime:Float64Array=new Float64Array(0);
 seen:Float64Array=new Float64Array(0);
 readonly targets=new Map<number,string>();
 size=0;
 readonly fold=new Uint8Array(32);
 shared=0;
 writes=0;
 constructor(readonly book:ReplicaBookV1,readonly local:boolean){}
 private term(id:number):Uint8Array|undefined{
  if(!this.local||!this.live(id))return undefined;
  const operation=this.operationAt(id)!;
  return manifestTermV1({path:this.book.name(id),operation,value:operation==="put"?this.shaAt(id):this.targets.get(id)??""});
 }
 private swap(before:Uint8Array|undefined,after:Uint8Array|undefined):void{
  if(before!==undefined){foldTermV1(this.fold,before);this.shared--;}
  if(after!==undefined){foldTermV1(this.fold,after);this.shared++;}
 }
 /** 🧮️ The order-independent digest of the shared files, kept incrementally; a delete and a realm path contribute nothing. */
 manifestHex():string{return digestHexV1(this.fold);}
 reserve(count:number):void{this.grow(count,count);}
 private grow(need:number,exact?:number):void{
  if(need<=this.flags.length)return;
  const length=exact??Math.max(need,Math.ceil(this.flags.length*1.5),1024);
  this.flags=widen(this.flags,length);this.ms=widen(this.ms,length);this.counter=widen(this.counter,length);this.origin=widen(this.origin,length);this.bytes=widen(this.bytes,length);this.sha=widen(this.sha,length*32);
  if(this.local){this.mtime=widen(this.mtime,length);this.seen=widen(this.seen,length);}
 }
 live(id:number):boolean{return id<this.flags.length&&(this.flags[id]!&1)===1;}
 operationAt(id:number):"put"|"delete"|"link"|undefined{
  if(!this.live(id))return undefined;
  const code=(this.flags[id]!>>1)&3;
  return code===1?"put":code===2?"delete":"link";
 }
 executableAt(id:number):boolean{return (this.flags[id]!&8)!==0;}
 racyAt(id:number):boolean{return (this.flags[id]!&16)===0||this.seen[id]!-this.mtime[id]!<racyMilliseconds;}
 hlcAt(id:number):string{return `${String(this.ms[id]).padStart(13,"0")}-${String(this.counter[id]).padStart(6,"0")}-${this.book.origins[this.origin[id]!]}`;}
 sameHlcAs(other:CompactTableV1,id:number):boolean{return this.ms[id]===other.ms[id]&&this.counter[id]===other.counter[id]&&this.origin[id]===other.origin[id];}
 shaAt(id:number):string{return Buffer.from(this.sha.buffer,this.sha.byteOffset+id*32,32).toString("hex");}
 shaIs(id:number,hex:string):boolean{return Buffer.from(this.sha.buffer,this.sha.byteOffset+id*32,32).equals(Buffer.from(hex,"hex"));}
 entryAt(id:number):StateEntry{
  const flag=this.flags[id]!,code=(flag>>1)&3,hlc=this.hlcAt(id);
  const entry:Record<string,unknown>=code===1?{hlc,operation:"put",sha256:this.shaAt(id),bytes:this.bytes[id],executable:(flag&8)!==0}:code===3?{hlc,operation:"link",target:this.targets.get(id)}:{hlc,operation:"delete"};
  if(this.local){entry.mtimeMs=this.mtime[id];if((flag&16)!==0)entry.seenMs=this.seen[id];}
  return entry as StateEntry;
 }
 write(id:number,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):void{
  this.writes++;this.grow(id+1);
  const before=this.term(id),hlc=entry.hlc;
  let ms=0,counter=0,valid=hlc.length>=22&&hlc.charCodeAt(13)===45&&hlc.charCodeAt(20)===45;
  for(let index=0;valid&&index<13;index++){const digit=hlc.charCodeAt(index)-48;if(digit<0||digit>9)valid=false;else ms=ms*10+digit;}
  for(let index=14;valid&&index<20;index++){const digit=hlc.charCodeAt(index)-48;if(digit<0||digit>9)valid=false;else counter=counter*10+digit;}
  if(!valid)throw Error(`Closed replica hlc required: ${hlc}`);
  let flag=1;
  if(entry.operation==="put"){
   const hex=entry.sha256;
   if(typeof hex!=="string"||hex.length!==64||typeof entry.bytes!=="number")throw Error("Closed replica entry required: put");
   const base=id*32;
   for(let index=0;index<32;index++){
    const high=hexValue(hex.charCodeAt(index*2)),low=hexValue(hex.charCodeAt(index*2+1));
    if(high<0||low<0)throw Error("Closed replica entry required: put");
    this.sha[base+index]=high*16+low;
   }
   this.bytes[id]=entry.bytes;this.targets.delete(id);
   flag|=2|(entry.executable===true?8:0);
  }else if(entry.operation==="link"){
   if(typeof entry.target!=="string"||entry.target==="")throw Error("Closed replica entry required: link");
   this.targets.set(id,entry.target);this.bytes[id]=0;flag|=6;
  }else{this.targets.delete(id);this.bytes[id]=0;flag|=4;}
  if(this.local){
   this.mtime[id]=entry.mtimeMs??0;
   if(entry.seenMs!==undefined){flag|=16;this.seen[id]=entry.seenMs;}
  }
  if(!this.live(id))this.size++;
  this.flags[id]=flag;this.ms[id]=ms;this.counter[id]=counter;this.origin[id]=this.book.originOf(hlc);
  if(this.local)this.swap(before,this.term(id));
 }
 copyRow(from:CompactTableV1,id:number):void{
  this.grow(id+1);
  const before=this.term(id);
  if(!this.live(id))this.size++;
  this.flags[id]=(from.flags[id]!&~16)|(this.local&&from.local?from.flags[id]!&16:0);
  this.ms[id]=from.ms[id]!;this.counter[id]=from.counter[id]!;this.origin[id]=from.origin[id]!;this.bytes[id]=from.bytes[id]!;
  for(let index=id*32,end=index+32;index<end;index++)this.sha[index]=from.sha[index]!;
  const target=from.targets.get(id);
  if(target===undefined)this.targets.delete(id);else this.targets.set(id,target);
  if(this.local&&from.local){this.mtime[id]=from.mtime[id]!;this.seen[id]=from.seen[id]!;}
  if(this.local)this.swap(before,this.term(id));
 }
 clearId(id:number):boolean{
  if(!this.live(id))return false;
  this.writes++;
  const before=this.term(id);
  this.flags[id]=0;this.targets.delete(id);this.size--;
  this.swap(before,undefined);
  return true;
 }
 columns(rows:number):Readonly<{flags:Uint8Array;ms:Float64Array;counter:Uint32Array;origin:Uint16Array;bytes:Float64Array;sha:Uint8Array;mtime:Float64Array;seen:Float64Array}>{
  return {flags:this.flags.subarray(0,rows),ms:this.ms.subarray(0,rows),counter:this.counter.subarray(0,rows),origin:this.origin.subarray(0,rows),bytes:this.bytes.subarray(0,rows),sha:this.sha.subarray(0,rows*32),mtime:this.mtime.subarray(0,this.local?rows:0),seen:this.seen.subarray(0,this.local?rows:0)};
 }
 adopt(columns:Readonly<{flags:Uint8Array;ms:Float64Array;counter:Uint32Array;origin:Uint16Array;bytes:Float64Array;sha:Uint8Array;mtime:Float64Array;seen:Float64Array}>,fold:Uint8Array,shared:number):void{
  this.writes++;
  this.flags=columns.flags;this.ms=columns.ms;this.counter=columns.counter;this.origin=columns.origin;this.bytes=columns.bytes;this.sha=columns.sha;this.mtime=columns.mtime;this.seen=columns.seen;
  let size=0;
  for(let id=0;id<this.flags.length;id++)size+=this.flags[id]!&1;
  this.size=size;this.fold.set(fold);this.shared=shared;
 }
 adoptRows(ids:Int32Array,columns:Readonly<{flags:Uint8Array;ms:Float64Array;counter:Uint32Array;origin:Uint16Array;bytes:Float64Array;sha:Uint8Array}>):void{
  let top=0;
  for(const id of ids)top=Math.max(top,id+1);
  this.grow(top);
  ids.forEach((id,row)=>{
   if(!this.live(id))this.size++;
   this.flags[id]=columns.flags[row]!;this.ms[id]=columns.ms[row]!;this.counter[id]=columns.counter[row]!;this.origin[id]=columns.origin[row]!;this.bytes[id]=columns.bytes[row]!;
   this.sha.set(columns.sha.subarray(row*32,row*32+32),id*32);
  });
 }
 mirror(from:CompactTableV1,skip:ReplicaQueueV1):void{
  const rows=from.flags.length;
  this.grow(rows);
  for(let id=0;id<this.flags.length;id++){
   if(skip.hasId(id))continue;
   if(id>=rows||(from.flags[id]!&1)===0){
    if((this.flags[id]!&1)===1){this.flags[id]=0;this.targets.delete(id);this.size--;}
    continue;
   }
   if((this.flags[id]!&1)===0)this.size++;
   this.flags[id]=from.flags[id]!&~16;this.ms[id]=from.ms[id]!;this.counter[id]=from.counter[id]!;this.origin[id]=from.origin[id]!;this.bytes[id]=from.bytes[id]!;
   this.sha.set(from.sha.subarray(id*32,id*32+32),id*32);
   const target=from.targets.get(id);
   if(target===undefined)this.targets.delete(id);else this.targets.set(id,target);
  }
 }
 clear():void{this.writes++;this.flags.fill(0);this.targets.clear();this.size=0;this.fold.fill(0);this.shared=0;}
 idOf(path:string):number|undefined{return this.book.find(path);}
 has(path:string):boolean{const id=this.book.find(path);return id!==undefined&&this.live(id);}
 get(path:string):StateEntry|undefined{const id=this.book.find(path);return id===undefined||!this.live(id)?undefined:this.entryAt(id);}
 set(path:string,entry:FleetManifestEntryV1&Partial<Readonly<{mtimeMs:number;seenMs:number}>>):this{this.write(this.book.intern(path),entry);return this;}
 delete(path:string):boolean{const id=this.book.find(path);return id===undefined?false:this.clearId(id);}
 ids():Int32Array{
  const ids=new Int32Array(this.size);
  let count=0;
  for(let id=0;id<this.flags.length&&count<ids.length;id++)if((this.flags[id]!&1)===1)ids[count++]=id;
  return ids;
 }
 *keys():Generator<string>{for(const id of this.ids())yield this.book.name(id);}
 *values():Generator<StateEntry>{for(const id of this.ids())yield this.entryAt(id);}
 *entries():Generator<[string,StateEntry]>{for(const id of this.ids())yield [this.book.name(id),this.entryAt(id)];}
 [Symbol.iterator]():Generator<[string,StateEntry]>{return this.entries();}
 count(operation:"put"|"delete"|"link"):number{let count=0;for(const id of this.ids())if(this.operationAt(id)===operation)count++;return count;}
}

/** 📥️ The store winners that the tree does not hold yet; the entry of a queued path is its row in the manifest table, so the queue itself costs one byte per path. A parked path (flag 2) is a manifest entry that lost the structural conflict rule: it is not pending and not counted, but its manifest row stays in the persisted queue so that it survives a restart and can come back when its winner goes away. */
export class ReplicaQueueV1{
 flags:Uint8Array=new Uint8Array(0);
 size=0;
 constructor(readonly book:ReplicaBookV1,readonly manifest:CompactTableV1){}
 reserve(count:number):void{if(count>this.flags.length)this.flags=widen(this.flags,count);}
 hasId(id:number):boolean{return id<this.flags.length&&this.flags[id]===1;}
 has(path:string):boolean{const id=this.book.find(path);return id!==undefined&&this.hasId(id);}
 get(path:string):StateEntry|undefined{return this.has(path)?this.manifest.get(path):undefined;}
 heldId(id:number):boolean{return id<this.flags.length&&this.flags[id]!==0;}
 parkId(id:number):void{
  if(id>=this.flags.length)this.flags=widen(this.flags,Math.max(id+1,Math.ceil(this.flags.length*1.5),1024));
  if(this.flags[id]===1)this.size--;
  this.flags[id]=2;
 }
 addId(id:number):boolean{
  if(id>=this.flags.length)this.flags=widen(this.flags,Math.max(id+1,Math.ceil(this.flags.length*1.5),1024));
  if(this.flags[id]===1)return false;
  this.flags[id]=1;this.size++;return true;
 }
 removeId(id:number):boolean{
  if(id>=this.flags.length||this.flags[id]===0)return false;
  const pending=this.flags[id]===1;
  this.flags[id]=0;if(pending)this.size--;return pending;
 }
 heldSnapshot():Int32Array{
  let count=0;
  for(let id=0;id<this.flags.length;id++)if(this.flags[id]!==0)count++;
  const ids=new Int32Array(count);
  count=0;
  for(let id=0;id<this.flags.length&&count<ids.length;id++)if(this.flags[id]!==0)ids[count++]=id;
  return ids;
 }
 snapshot():Int32Array{
  const ids=new Int32Array(this.size);
  let count=0;
  for(let id=0;id<this.flags.length&&count<ids.length;id++)if(this.flags[id]===1)ids[count++]=id;
  return ids;
 }
 *keys():Generator<string>{for(const id of this.snapshot())yield this.book.name(id);}
 *entries():Generator<[string,StateEntry]>{for(const id of this.snapshot())yield [this.book.name(id),this.manifest.entryAt(id)];}
 [Symbol.iterator]():Generator<[string,StateEntry]>{return this.entries();}
}

/** 🔠️ Finds the live path that equals a given one when case is ignored, without keeping a lower-cased copy of every path: a table of ids keyed by the 32-bit hash of the folded name that the path table stored when it met the path, whose candidates are confirmed by folding their real name only when the hashes agree. */
export class ReplicaFoldsV1{
 private slots:Int32Array=new Int32Array(2048);
 private used=0;
 private dirs:Int32Array=new Int32Array(1<<20);
 private parents:Int32Array=new Int32Array(1<<20);
 constructor(readonly book:ReplicaBookV1){}
 private count(id:number,step:number):void{
  const name=this.book.name(id),mask=this.dirs.length-1;
  let hash=2166136261,parent=hash;
  for(let index=0;index<name.length;index++){
   const code=name.charCodeAt(index);
   if(code===47){this.dirs[(hash>>>0)&mask]+=step;parent=hash;}
   hash=Math.imul(hash^code,16777619);
  }
  this.parents[(parent>>>0)&mask]+=step;
 }
 private slot(directory:string):number{
  let hash=2166136261;
  for(let index=0;index<directory.length;index++)hash=Math.imul(hash^directory.charCodeAt(index),16777619);
  return (hash>>>0)&(this.dirs.length-1);
 }
 below(directory:string):number{return this.dirs[this.slot(directory)]!;}
 /** 🏘️ The live paths directly inside a directory (`""` is the worktree root); hash collisions can only raise the count, never lower it. */
 inside(directory:string):number{return this.parents[this.slot(directory)]!;}
 reserve(count:number):void{
  let size=this.slots.length;
  while(size<count*2+2)size*=2;
  if(size>this.slots.length)this.slots=new Int32Array(size);
 }
 columns():Readonly<{slots:Int32Array;used:number;dirs:Int32Array;parents:Int32Array}>{return {slots:this.slots,used:this.used,dirs:this.dirs,parents:this.parents};}
 adopt(slots:Int32Array,used:number,dirs:Int32Array,parents:Int32Array):void{this.slots=slots;this.used=used;this.dirs=dirs;this.parents=parents;}
 find(path:string):string|undefined{
  const fold=foldNameV1(path),hash=hashText(fold),mask=this.slots.length-1;
  for(let slot=hash&mask;;slot=(slot+1)&mask){
   const held=this.slots[slot]!;
   if(held===0)return undefined;
   if(held>0&&this.book.foldHash(held-1)===hash&&foldNameV1(this.book.name(held-1))===fold)return this.book.name(held-1);
  }
 }
 add(id:number):void{
  const hash=this.book.foldHash(id),mask=this.slots.length-1;
  let fold:string|undefined,reusable=-1,slot=hash&mask;
  for(;;slot=(slot+1)&mask){
   const held=this.slots[slot]!;
   if(held===0)break;
   if(held<0){if(reusable<0)reusable=slot;continue;}
   if(this.book.foldHash(held-1)!==hash)continue;
   if(held-1===id){return;}
   if((fold??=foldNameV1(this.book.name(id)))===foldNameV1(this.book.name(held-1))){this.slots[slot]=id+1;this.count(id,1);return;}
  }
  if(reusable>=0){this.slots[reusable]=id+1;this.count(id,1);return;}
  if((this.used+1)*2>this.slots.length){this.grow();this.add(id);return;}
  this.used++;this.slots[slot]=id+1;this.count(id,1);
 }
 remove(id:number):void{
  const hash=this.book.foldHash(id),mask=this.slots.length-1;
  for(let slot=hash&mask;;slot=(slot+1)&mask){
   const held=this.slots[slot]!;
   if(held===0)return;
   if(held===id+1){this.slots[slot]=-1;this.count(id,-1);return;}
  }
 }
 private grow():void{
  const old=this.slots;
  this.slots=new Int32Array(old.length*2);this.used=0;
  const mask=this.slots.length-1;
  for(const held of old){
   if(held<=0)continue;
   let slot=this.book.foldHash(held-1)&mask;
   while(this.slots[slot]!==0)slot=(slot+1)&mask;
   this.slots[slot]=held;this.used++;
  }
 }
}

/** 🗒️ A list of scan entries stored as columns of ids, sizes and times in the path table instead of one object and one string per entry. */
export class ReplicaFoundV1{
 ids=new Int32Array(1024);
 bytes=new Float64Array(1024);
 mtime=new Float64Array(1024);
 flags=new Uint8Array(1024);
 readonly targets=new Map<number,string>();
 length=0;
 constructor(readonly book:ReplicaBookV1){}
 push(entry:FleetScanEntryV1):void{
  const index=this.length++;
  if(index>=this.ids.length){
   const size=Math.ceil(this.ids.length*1.5);
   this.ids=widen(this.ids,size);this.bytes=widen(this.bytes,size);this.mtime=widen(this.mtime,size);this.flags=widen(this.flags,size);
  }
  this.ids[index]=this.book.intern(entry.path);this.bytes[index]=entry.bytes;this.mtime[index]=entry.mtimeMs;
  this.flags[index]=(entry.kind==="link"?1:0)|(entry.executable?2:0);
  if(entry.target!==undefined)this.targets.set(index,entry.target);
 }
 at(index:number):FleetScanEntryV1{
  const flag=this.flags[index]!,base={path:this.book.name(this.ids[index]!),bytes:this.bytes[index]!,mtimeMs:this.mtime[index]!,executable:(flag&2)!==0};
  return (flag&1)!==0?{...base,kind:"link",target:this.targets.get(index)}:{...base,kind:"file"};
 }
}

class Bits{
 bytes=new Uint8Array(0);
 mark(id:number):void{
  if(id>=this.bytes.length)this.bytes=widen(this.bytes,Math.max(id+1,this.bytes.length*2,1024));
  this.bytes[id]=1;
 }
 has(id:number):boolean{return id<this.bytes.length&&this.bytes[id]===1;}
}

const settleHeap=():void=>{(globalThis as {Bun?:{gc?:(force:boolean)=>void}}).Bun?.gc?.(true);};
const pacer=(every:number):(()=>void)=>{let count=0;return ()=>{if(++count%every===0)settleHeap();};};

/** ⚖️ Decides a conflict between an unpublished local edit and a remote event for the same path: the local edit wins only when its modification time is later than the wall time inside the remote hlc; a local delete has no modification time and always loses. */
export function conflictWinnerV1(localMtimeMs:number|undefined,remoteHlc:string):"local"|"remote"{
 return localMtimeMs!==undefined&&localMtimeMs>parseHlcV1(remoteHlc).ms?"local":"remote";
}

const replicaValidators=new Map<string,CompiledJsonSchemaSubset>();

/** 🛡️ Validates one document against a named definition of the replica schema and names the first problem. */
export function requireReplicaShapeV1(definition:"Machine"|"Cursor"|"Clock"|"Lock"|"SyncSwitch"|"Home"|"ProgressEvent"|"Status"|"VerifyReport",value:unknown,message:string):void{
 let validator=replicaValidators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);replicaValidators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🗄️ Resolves the machine-local state files of a worktree. */
export function localPathsV1(worktree:string):LocalPathsV1{
 const directory=join(worktree,".cache","f");
 return {directory,machine:join(directory,"machine.json"),checkpoint:join(directory,"state.ckp"),wal:join(directory,"state.wal"),takeoverState:join(directory,"state.json"),takeoverLog:join(directory,"state.log"),cursor:join(directory,"cursor.json"),clock:join(directory,"clock.json"),lost:join(directory,"lost"),tmp:join(directory,"tmp"),lock:join(directory,"daemon.lock"),sync:join(directory,"sync.json"),gitignore:join(worktree,".gitignore"),hints:join(directory,"hints")};
}

/** 🔌️ Reads the machine-local switch. A missing file means synchronization is on. `false` means this machine neither publishes nor applies. */
export async function syncEnabledV1(worktree:string):Promise<boolean>{
 const value=await readJson(localPathsV1(worktree).sync);
 if(value===undefined)return true;
 requireReplicaShapeV1("SyncSwitch",value,"Closed replica sync switch required");
 return (value as {enabled:boolean}).enabled;
}

/** 🎚️ Turns synchronization on or off for this worktree only. On removes the switch file. Off writes `.cache/f/sync.json`. The running daemon notices it on its next pass. */
export async function setSyncEnabledV1(worktree:string,enabled:boolean):Promise<void>{
 const paths=localPathsV1(worktree);
 if(enabled){await unlink(paths.sync).catch(error=>{if(!gone(error))throw error;});return;}
 await mkdir(paths.directory,{recursive:true});
 await writeJson(paths,paths.sync,{version:1,enabled:false});
}

const probesInFlight=new Map<string,Promise<boolean>>();

/** 📶️ Tells whether the store directory answers; an unmounted export leaves an empty mount point and counts as unreachable, and a store that does not answer within `deadlineMilliseconds` (15 s) counts as unreachable too. A `stat` that a hard mount blocks cannot be cancelled, so concurrent callers share the one probe that is still in flight instead of blocking one more worker thread each. */
export async function storeReachableV1(storeRoot:string,deadlineMilliseconds:number=15_000):Promise<boolean>{
 let probe=probesInFlight.get(storeRoot);
 if(probe===undefined){
  probe=stat(storePathsV1(storeRoot).root).then(info=>{const present=info.isDirectory();if(present)markStoreSeenV1(storeRoot);return present;},()=>false).finally(()=>{probesInFlight.delete(storeRoot);});
  probesInFlight.set(storeRoot,probe);
 }
 let timer:ReturnType<typeof setTimeout>|undefined;
 try{return await Promise.race([probe,new Promise<boolean>(done=>{timer=setTimeout(()=>{done(false);},deadlineMilliseconds);timer.unref?.();})]);}
 finally{clearTimeout(timer);}
}

const indexes=(length:number):Int32Array=>Int32Array.from({length},(_,index)=>index);

async function pool<T>(items:ArrayLike<T>,limit:number,work:(item:T,index:number)=>Promise<void>,signal?:AbortSignal):Promise<void>{
 let next=0;
 await Promise.all(Array.from({length:Math.min(Math.max(1,limit),items.length)},async()=>{
  while(next<items.length){signal?.throwIfAborted();const index=next++;await work(items[index]!,index);}
 }));
}

async function readJson(file:string):Promise<unknown>{
 try{return JSON.parse(await readFile(file,"utf8")) as unknown;}catch(error){if(gone(error))return undefined;throw error;}
}

function syncPath(path:string,directory:boolean):void{
 if(directory&&process.platform==="win32")return;
 try{const descriptor=openSync(path,directory?"r":"r+");try{fsyncSync(descriptor);}finally{closeSync(descriptor);}}catch{}
}

const syncDirectory=async(directory:string):Promise<void>=>{syncPath(directory,true);};

async function writeJson(paths:LocalPathsV1,file:string,value:unknown):Promise<void>{
 await mkdir(paths.tmp,{recursive:true});
 const temporary=join(paths.tmp,token());
 try{
  await writeFile(temporary,JSON.stringify(value));syncPath(temporary,false);
  await rename(temporary,file);await syncDirectory(dirname(file));
 }catch(error){await rm(temporary,{force:true});throw error;}
}

async function readIgnoreText(paths:LocalPathsV1):Promise<string>{
 try{return await readFile(paths.gitignore,"utf8");}catch(error){if(gone(error))return "";throw error;}
}

/** 🔒️ Raised when another live daemon already holds `daemon.lock` of the worktree. */
export class ReplicaBusyV1 extends Error{
 readonly pid:number;
 constructor(pid:number,file:string){super(`Another replica daemon (pid ${pid}) holds ${file}`);this.name="ReplicaBusyV1";this.pid=pid;}
}

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return code(error)==="EPERM";}};

/** 🪪️ Raised when the origin id of this machine is already owned by another machine: `origins/<origin>/home.json` carries the key of the machine that published first. Two machines for one origin would truncate each other's journal, so the second one is refused instead. */
export class OriginTakenV1 extends Error{
 readonly originId:string;
 constructor(originId:string){super(`Origin ${originId} belongs to another machine: its home.json carries another key`);this.name="OriginTakenV1";this.originId=originId;}
}

/** ⏰️ Raised instead of issuing a clock value more than 60 s ahead of the wall clock: the own journal holds events from the future (the wall clock was set forward and corrected), and a new edit stamped lower would lose against them, one stamped higher would freeze this origin on every other machine. Publishing resumes when the wall clock has caught up. */
export class ClockAheadV1 extends Error{
 readonly seconds:number;
 constructor(seconds:number){super(`Own clock is ${seconds} s ahead of the wall clock; publishing waits until the wall clock has caught up`);this.name="ClockAheadV1";this.seconds=seconds;}
}

const bootMilliseconds=():number=>Date.now()-Math.round(uptime()*1000);

async function readLock(file:string):Promise<Readonly<{pid:number;stale:boolean}>|undefined>{
 let text:string;
 try{text=await readFile(file,"utf8");}catch(error){if(gone(error))return undefined;throw error;}
 try{
  const value=JSON.parse(text) as {pid?:unknown;startedMs?:unknown;bootMs?:unknown;hb?:unknown};
  if(typeof value.pid==="number"&&Number.isSafeInteger(value.pid)&&value.pid>0){
   const own=value.pid===process.pid;
   let stale=own?typeof value.startedMs!=="number"||value.startedMs<Date.now()-process.uptime()*1000-1000:!alive(value.pid);
   if(!stale&&!own){
    const age=Date.now()-((await stat(file).catch(()=>undefined))?.mtimeMs??Date.now());
    if(value.hb===true&&age>staleLockMilliseconds)stale=(await readBeatV1(dirname(dirname(dirname(file))),Date.now()))?.pid!==value.pid;
    else if(typeof value.bootMs==="number"&&Math.abs(value.bootMs-bootMilliseconds())>120_000&&age>30_000)stale=true;
   }
   return {pid:value.pid,stale};
  }
 }catch{}
 const info=await stat(file).catch(()=>undefined);
 return {pid:0,stale:info===undefined||Date.now()-info.mtimeMs>5000};
}

async function takeSteal(file:string):Promise<boolean>{
 try{const handle=await open(file,"wx");await handle.close();return true;}
 catch(error){
  if(code(error)!=="EEXIST")throw error;
  const info=await stat(file).catch(()=>undefined);
  if(info!==undefined&&Date.now()-info.mtimeMs>10_000)await rm(file,{force:true});
  return false;
 }
}

async function acquireLock(paths:LocalPathsV1,originId:string):Promise<()=>Promise<void>>{
 await mkdir(paths.directory,{recursive:true});
 const body=JSON.stringify({version:1,pid:process.pid,startedMs:Date.now(),originId,bootMs:bootMilliseconds(),hb:true}),steal=`${paths.lock}.steal`;
 for(let attempt=0;attempt<20;attempt++){
  try{
   const handle=await open(paths.lock,"wx");
   try{await handle.writeFile(body);await handle.sync();}finally{await handle.close();}
   const beat=setInterval(()=>{const now=new Date();utimes(paths.lock,now,now).catch(()=>{});},5000);
   beat.unref();
   return async()=>{clearInterval(beat);const held=await readLock(paths.lock);if(held?.pid===process.pid)await rm(paths.lock,{force:true});};
  }catch(error){
   if(code(error)!=="EEXIST")throw error;
   const held=await readLock(paths.lock);
   if(held===undefined)continue;
   if(held.stale){
    if(!await takeSteal(steal)){await sleep(50);continue;}
    try{const again=await readLock(paths.lock);if(again?.stale===true)await rm(paths.lock,{force:true});}
    finally{await rm(steal,{force:true});}
    continue;
   }
   if(held.pid===0){await sleep(50);continue;}
   throw new ReplicaBusyV1(held.pid,paths.lock);
  }
 }
 throw new ReplicaBusyV1(0,paths.lock);
}

type Tables=Readonly<{book:ReplicaBookV1;files:CompactTableV1;manifest:CompactTableV1;queue:ReplicaQueueV1;folds:ReplicaFoldsV1;dirs:DirectoryTableV1}>;
type Local=Readonly<{machine:Machine|undefined;cursors:Record<string,FleetCursorV1>;clock:FleetClockV1;generation:number;log:Readonly<{size:number;good:number}>;source:"checkpoint"|"takeover"|"empty";memo:Map<string,Uint8Array>;checkpointBytes:number;mixed:boolean}>;

function tablesV1():Tables{
 const book=new ReplicaBookV1(),manifest=new CompactTableV1(book,false);
 return {book,files:new CompactTableV1(book,true),manifest,queue:new ReplicaQueueV1(book,manifest),folds:new ReplicaFoldsV1(book),dirs:new DirectoryTableV1(book)};
}

function storeEntry(tables:Tables,kind:"files"|"queue",path:string,entry:StateEntry|null):void{
 if(kind==="files"){
  if(entry===null){const id=tables.book.find(path);if(id!==undefined){tables.files.clearId(id);tables.folds.remove(id);}return;}
  const id=tables.book.intern(path);
  tables.files.write(id,entry);
  if(entry.operation==="delete"||indexRealmV1(path))tables.folds.remove(id);else tables.folds.add(id);
  return;
 }
 const id=tables.book.intern(path);
 if(entry===null){tables.queue.removeId(id);return;}
 tables.manifest.write(id,entry);
 tables.queue.addId(id);
}

function apply(tables:Tables,memo:Map<string,Uint8Array>,record:WalRecordV1):void{
 if(record.kind==="memo"){if(record.value===null)memo.delete(record.name);else memo.set(record.name,record.value);return;}
 storeEntry(tables,record.kind,record.path,record.entry as StateEntry|null);
}

async function replayLog(paths:LocalPathsV1,generation:number,tables:Tables,memo:Map<string,Uint8Array>):Promise<Readonly<{size:number;good:number}>>{
 const found=await readWalV1(paths.wal);
 if(found===undefined)return {size:0,good:0};
 const {data,scan}=found;
 if(scan.verdict==="corrupt")throw new StateCorruptV1("log");
 if(scan.generation===undefined||scan.generation<generation)return {size:data.length,good:0};
 if(scan.generation>generation)throw new StateCorruptV1("log generation");
 for(const payload of scan.payloads)for(const record of decodeRecordsV1(payload))apply(tables,memo,record);
 return {size:data.length,good:scan.good};
}

const typedArrays={u8:Uint8Array,u16:Uint16Array,u32:Uint32Array,i32:Int32Array,f64:Float64Array,text:Uint8Array} as const;
const headroom=(rows:number):number=>rows+(rows>>4)+1024;
const growing=(name:string):boolean=>(name.startsWith("files.")&&name!=="files.targets")||name.startsWith("book.")||name.startsWith("dirs.");

function targetsOf(bytes:Uint8Array,into:Map<number,string>):void{
 const view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);
 for(let position=0;position<bytes.length;){
  const id=view.getUint32(position,true),length=view.getUint32(position+4,true);
  into.set(id,Buffer.from(bytes.subarray(position+8,position+8+length)).toString("utf8"));position+=8+length;
 }
}

function targetBlob(targets:Iterable<readonly [number,string]>):Uint8Array{
 const parts:Buffer[]=[];
 for(const [id,target] of targets){
  const text=Buffer.from(target,"utf8"),head=Buffer.alloc(8);
  head.writeUInt32LE(id,0);head.writeUInt32LE(text.length,4);parts.push(head,text);
 }
 return Buffer.concat(parts);
}

async function loadCheckpoint(paths:LocalPathsV1,tables:Tables):Promise<Readonly<{generation:number;bytes:number;memo:Map<string,Uint8Array>}>|undefined>{
 const arrays=new Map<string,ArrayBufferView>(),rowsOf=new Map<string,number>(),memo=new Map<string,Uint8Array>();
 let pages:Buffer|undefined,pageUsed=0;
 const header=await readCheckpointV1(paths.checkpoint,(name,length)=>{
  if(name.startsWith("memo:")){const bytes=new Uint8Array(length);memo.set(name.slice(5),bytes);return bytes;}
  const kind=checkpointPartsV1[name];
  if(kind===undefined)return undefined;
  const width=partWidthV1[kind]!;
  if(length%width!==0)throw new StateCorruptV1(`part ${name}`);
  if(name==="book.pages"){pages=Buffer.allocUnsafe(Math.ceil(length/pageBytes)*pageBytes);pageUsed=length;return pages.subarray(0,length);}
  const stride=name.endsWith(".sha")?32:1,rows=length/width/stride,capacity=(growing(name)?headroom(rows):rows)*stride+(name==="book.starts"?1:0),array=new typedArrays[kind](capacity);
  arrays.set(name,array);rowsOf.set(name,rows);
  return new Uint8Array(array.buffer,0,length);
 });
 if(header===undefined)return undefined;
 const get=<T extends ArrayBufferView>(name:string):T=>{const value=arrays.get(name);if(value===undefined)throw new StateCorruptV1(`missing part ${name}`);return value as unknown as T;};
 const agree=(names:readonly string[],rows:number):void=>{for(const name of names)if(rowsOf.get(name)!==rows)throw new StateCorruptV1(`rows of ${name}`);};
 const meta=get<Float64Array>("meta"),fileRows=meta[2]!,queued=rowsOf.get("queue.ids")!;
 if(meta.length<3||(rowsOf.get("folds.dirs")??1)%2!==0)throw new StateCorruptV1("meta");
 agree(["book.starts","book.lengths","book.hashes","book.folded"],header.paths);
 agree(["files.flags","files.ms","files.counter","files.origin","files.bytes","files.sha","files.mtime","files.seen"],fileRows);
 agree(["queue.flags","queue.ms","queue.counter","queue.origin","queue.bytes","queue.sha"],queued);
 if(fileRows>header.paths||queued>header.paths)throw new StateCorruptV1("row counts");
 const pageList:Buffer[]=[];
 if(pages!==undefined)for(let offset=0;offset<pages.length;offset+=pageBytes)pageList.push(pages.subarray(offset,offset+pageBytes));
 const origins=Buffer.from(get<Uint8Array>("origins")).toString("utf8");
 tables.book.adopt({starts:get("book.starts"),lengths:get("book.lengths"),hashes:get("book.hashes"),folded:get("book.folded"),pages:pageList,used:pageList.length===0?pageBytes:pageUsed-(pageList.length-1)*pageBytes},header.paths,origins===""?[]:origins.split("\n"));
 const column=(table:"files"|"queue")=>({flags:get<Uint8Array>(`${table}.flags`),ms:get<Float64Array>(`${table}.ms`),counter:get<Uint32Array>(`${table}.counter`),origin:get<Uint16Array>(`${table}.origin`),bytes:get<Float64Array>(`${table}.bytes`),sha:get<Uint8Array>(`${table}.sha`)});
 tables.files.adopt({...column("files"),mtime:get("files.mtime"),seen:get("files.seen")},get("fold"),meta[0]!);
 targetsOf(get<Uint8Array>("files.targets"),tables.files.targets);
 const ids=get<Int32Array>("queue.ids");
 tables.manifest.reserve(tables.files.flags.length);
 tables.manifest.adoptRows(ids,column("queue"));
 targetsOf(get<Uint8Array>("queue.targets"),tables.manifest.targets);
 tables.queue.reserve(ids.reduce((top,id)=>Math.max(top,id+1),0));
 for(const id of ids)tables.queue.addId(id);
 const buckets=(name:string):Int32Array=>{
  const dense=new Int32Array(1<<20),pairs=get<Int32Array>(name);
  for(let index=0;index<rowsOf.get(name)!;index+=2){if(pairs[index]!<0||pairs[index]!>=dense.length)throw new StateCorruptV1("folds");dense[pairs[index]!]=pairs[index+1]!;}
  return dense;
 };
 tables.folds.adopt(get("folds.slots"),meta[1]!,buckets("folds.dirs"),buckets("folds.parents"));
 const members=get<Int32Array>("dirs.members");
 agree(["dirs.present"],rowsOf.get("dirs.mtime")!);
 tables.dirs.adopt(get("dirs.mtime"),get("dirs.present"),members,rowsOf.get("dirs.members")!);
 return {generation:header.generation,bytes:96+header.payloadBytes,memo};
}

async function takeOverState(paths:LocalPathsV1,tables:Tables):Promise<number|undefined>{
 const expected=Math.ceil(((await stat(paths.takeoverState).catch(()=>undefined))?.size??0)/stateBytesPerEntry);
 if(expected===0)return undefined;
 tables.book.reserve(expected);tables.files.reserve(expected);tables.manifest.reserve(expected);tables.queue.reserve(expected);tables.folds.reserve(expected);
 let version:unknown,generation:unknown,sawFiles=false,sawQueue=false;
 const pace=pacer(paceEntries),fail=():never=>{throw new StateCorruptV1("state.json");};
 try{
  await streamMembersV1(paths.takeoverState,new Set(["files","queue"]),(outer,inner,text)=>{
   if(inner===undefined){
    const value=JSON.parse(text) as unknown;
    if(outer==="version")version=value;
    else if(outer==="generation")generation=value;
    else if(outer==="files"||outer==="queue")fail();
    return;
   }
   pace();
   storeEntry(tables,outer==="files"?"files":"queue",inner,JSON.parse(text) as StateEntry);
  },outer=>{if(outer==="files")sawFiles=true;else sawQueue=true;});
 }catch(error){
  if(gone(error))return undefined;
  if(error instanceof StateCorruptV1||error instanceof SyntaxError||(error as Error).message.startsWith("Closed JSON object required")||(error as Error).message.startsWith("Closed replica state"))return fail();
  throw error;
 }
 if(version!==1||typeof generation!=="number"||!sawFiles||!sawQueue)return fail();
 return generation;
}

async function takeOverLog(paths:LocalPathsV1,generation:number,tables:Tables):Promise<void>{
 let data:Buffer;
 try{data=await readFile(paths.takeoverLog);}catch(error){if(gone(error))return;throw error;}
 const good=data.lastIndexOf(10)+1,lines=data.toString("utf8",0,good).split("\n");
 lines.pop();
 if(lines.length===0)return;
 const parse=(line:string):Record<string,unknown>=>{try{return JSON.parse(line) as Record<string,unknown>;}catch{throw new StateCorruptV1("state.log");}};
 if(parse(lines[0]!).generation!==generation)return;
 for(let index=1;index<lines.length;index++){
  const value=parse(lines[index]!);
  if((value.kind!=="files"&&value.kind!=="queue")||typeof value.path!=="string"||value.path===""||typeof value.entry!=="object")throw new StateCorruptV1("state.log");
  storeEntry(tables,value.kind,value.path,value.entry as StateEntry|null);
 }
}

type LoadMode="open"|"read"|"ignore";

async function foldOlderFormat(paths:LocalPathsV1,tables:Tables):Promise<void>{
 const older=tablesV1(),generation=await takeOverState(paths,older);
 if(generation!==undefined)await takeOverLog(paths,generation,older);
 for(const id of older.files.ids()){
  const path=older.book.name(id),entry=older.files.entryAt(id),known=tables.files.get(path);
  if(known===undefined||winsV1(entry.hlc,known.hlc))storeEntry(tables,"files",path,entry);
 }
}

const mtimeOf=async(file:string):Promise<number>=>(await stat(file).catch(()=>undefined))?.mtimeMs??0;
const memoJson=(memo:Map<string,Uint8Array>,name:string):unknown=>{
 const raw=memo.get(name);
 if(raw===undefined)return undefined;
 try{return JSON.parse(Buffer.from(raw).toString("utf8")) as unknown;}catch{throw new StateCorruptV1(`memo ${name}`);}
};

async function readMachine(file:string):Promise<unknown>{
 try{
  const value=await readJson(file);
  if(value!==undefined)requireReplicaShapeV1("Machine",value,"Closed replica machine required");
  return value;
 }catch(error){
  if(error instanceof SyntaxError||messageOf(error).startsWith("Closed replica machine"))return undefined;
  throw error;
 }
}

async function loadLocal(paths:LocalPathsV1,tables:Tables,mode:LoadMode="read"):Promise<Local>{
 const machine=await readMachine(paths.machine);
 let source:Local["source"]="empty",generation=0,checkpointBytes=0,memo=new Map<string,Uint8Array>(),log={size:0,good:0},mixed=false;
 if(mode!=="ignore"){
  const jsonNewer=Math.max(await mtimeOf(paths.takeoverState),await mtimeOf(paths.takeoverLog))>Math.max(await mtimeOf(paths.checkpoint),await mtimeOf(paths.wal));
  const loaded=await loadCheckpoint(paths,tables);
  if(loaded===undefined){
   const taken=await takeOverState(paths,tables);
   if(taken!==undefined){source="takeover";generation=taken;await takeOverLog(paths,taken,tables);}
  }
  if(loaded!==undefined){source="checkpoint";generation=loaded.generation;checkpointBytes=loaded.bytes;memo=loaded.memo;}
  if(source!=="takeover")log=await replayLog(paths,generation,tables,memo);
  if(source==="checkpoint"&&jsonNewer){await foldOlderFormat(paths,tables);mixed=true;}
  if(mode==="open"&&source==="empty"&&machine!==undefined)throw new StateCorruptV1("state missing");
 }
 tables.manifest.mirror(tables.files,tables.queue);
 let cursors:unknown,clock:unknown;
 if(source==="checkpoint"){
  cursors=memoJson(memo,"cursors");clock=memoJson(memo,"clock");
  if(cursors!==undefined)try{requireReplicaShapeV1("Cursor",{version:1,cursors},"Closed replica cursor required");}catch{throw new StateCorruptV1("memo cursors");}
  if(clock!==undefined)try{requireReplicaShapeV1("Clock",{version:1,...(clock as object)},"Closed replica clock required");}catch{throw new StateCorruptV1("memo clock");}
 }
 if(cursors===undefined&&mode!=="ignore"){
  const document=await readJson(paths.cursor);
  if(document!==undefined)requireReplicaShapeV1("Cursor",document,"Closed replica cursor required");
  cursors=(document as {cursors:Record<string,FleetCursorV1>}|undefined)?.cursors;
 }
 if(clock===undefined){
  const document=await readJson(paths.clock);
  if(document!==undefined)requireReplicaShapeV1("Clock",document,"Closed replica clock required");
  clock=document;
 }
 const hybrid=clock as {ms:number;counter:number}|undefined;
 return {machine:machine as Machine|undefined,cursors:{...(cursors as Record<string,FleetCursorV1>|undefined)},clock:{ms:hybrid?.ms??0,counter:hybrid?.counter??0},generation,log,source,memo,checkpointBytes,mixed};
}

export function settle(options:ReplicaOptionsV1):Settings{
 requireOriginIdV1(options.originId);
 if(!originPattern.test(options.groupId))throw Error(`Closed replica group id required: ${options.groupId}`);
 return {worktree:options.worktree,storeRoot:options.storeRoot,originId:options.originId,groupId:options.groupId,pollMilliseconds:options.pollMilliseconds??250,scanSeconds:options.scanSeconds??90,debounceMilliseconds:options.debounceMilliseconds??150,retentionDays:options.retentionDays??14,concurrency:options.concurrency??8,persistMilliseconds:options.persistMilliseconds??5000,modeless:options.modeless??process.platform==="win32",now:options.now??Date.now,compaction:options.compaction??compactionDefaultsV1};
}

export const filesKey=(id:number):number=>id*2,queueKey=(id:number):number=>id*2+1;

const directoryKeys=new WeakMap<ReplicaV1,{index:Map<string,number>;writes:number}>();

/** 🗂️ How many live local files lie below each directory, counted by the fold key of the directory, so that a file or link at the path of a directory spelled in another case finds the files below it. Built on first use from the registers, kept by `setFile` and `forget`, and rebuilt when any other code wrote the registers meanwhile (the write counter of the `files` table moved: adopt, join, takeover, load, retire); it only decides whether the exact scan of `settleClashes` runs. */
function foldedDirectories(replica:ReplicaV1):Map<string,number>{
 let held=directoryKeys.get(replica);
 if(held===undefined||held.writes!==replica.files.writes){
  held={index:new Map(),writes:replica.files.writes};directoryKeys.set(replica,held);
  for(const id of replica.files.ids())if(replica.files.operationAt(id)!=="delete")countBelow(held.index,replica.book.name(id),1);
 }
 return held.index;
}

function countBelow(index:Map<string,number>,path:string,step:number):void{
 const key=clashKeyV1(path);
 for(let cut=key.lastIndexOf("/");cut>0;cut=key.lastIndexOf("/",cut-1)){const parent=key.slice(0,cut);index.set(parent,(index.get(parent)??0)+step);}
}

function setFile(replica:ReplicaV1,path:string,entry:StateEntry):void{
 const id=replica.book.intern(path),held=directoryKeys.get(replica),index=held!==undefined&&held.writes===replica.files.writes?held.index:undefined;
 if(index!==undefined&&replica.files.live(id)&&replica.files.operationAt(id)!=="delete")countBelow(index,path,-1);
 if(index!==undefined&&entry.operation!=="delete")countBelow(index,path,1);
 replica.files.write(id,entry);
 if(held!==undefined)held.writes=index===undefined?-1:replica.files.writes;
 if(entry.operation==="delete")replica.folds.remove(id);else replica.folds.add(id);
 replica.changed.add(filesKey(id));replica.dirty=true;
}

function forget(replica:ReplicaV1,path:string):void{
 const id=replica.book.find(path);
 if(id===undefined||!replica.files.live(id))return;
 const held=directoryKeys.get(replica),index=held!==undefined&&held.writes===replica.files.writes?held.index:undefined;
 if(index!==undefined&&replica.files.operationAt(id)!=="delete")countBelow(index,path,-1);
 replica.folds.remove(id);replica.files.clearId(id);
 if(held!==undefined)held.writes=index===undefined?-1:replica.files.writes;
 replica.changed.add(filesKey(id));replica.dirty=true;
}

const repairRequest=(worktree:string):string=>join(worktree,".cache","f","repair");

/** 🩹 Tells whether this machine was asked to repair its worktree (`.cache/f/repair` exists). */
export async function repairRequestedV1(worktree:string):Promise<boolean>{
 try{await lstat(repairRequest(worktree));return true;}catch{return false;}
}

/** 🧯 Asks the running replica of this machine to repair its worktree on its next pass. */
export async function requestRepairV1(worktree:string):Promise<void>{
 await mkdir(join(worktree,".cache","f"),{recursive:true});
 await writeFile(repairRequest(worktree),"");
}

/** 🧲️ Folds the store and takes over every register this machine does not hold or holds older: the manifest row is replaced and the path is queued, so the follow adopts it when the disk content matches (size first, hash only for the same size) and nothing of it is published as new; returns how many paths were queued. */
export async function takeOverRegistersV1(replica:ReplicaV1,signal?:AbortSignal):Promise<number>{
 const {manifest,queue,book,settings}=replica;
 let count=0,complete=true;
 const folded=await foldStoreV1(settings.storeRoot,signal,event=>{if(event.event==="rejoin-required"||event.event==="clock-skew")complete=false;});
 for(const [path,entry] of folded.manifest.entries()){
  const known=manifest.get(path);
  if(known!==undefined&&!winsV1(entry.hlc,known.hlc))continue;
  manifest.set(path,plain(entry));
  const id=book.intern(path);
  if(!queue.hasId(id)){enqueueId(replica,id);count++;}
 }
 if(complete)for(const id of manifest.ids()){
  const path=book.name(id);
  if(folded.manifest.get(path)!==undefined)continue;
  manifest.clearId(id);queue.removeId(id);forget(replica,path);replica.changed.add(queueKey(id));replica.dirty=true;
  count++;
 }
 return count;
}

/** 🚑 Folds the store and takes over every register this machine does not hold or holds older (events an earlier daemon read past without keeping, the stage realm included), then queues every shared file or link of the manifest that is absent on disk although the local state holds it as applied, so the next follow materialises them, and consumes the request; returns how many were queued. A local deletion that was not published yet is undone by this. */
export async function repairV1(replica:ReplicaV1,signal?:AbortSignal):Promise<number>{
 const {manifest,queue,book,settings}=replica;
 let count=await takeOverRegistersV1(replica,signal);
 for(const id of manifest.ids()){
  signal?.throwIfAborted();
  const path=book.name(id);
  if(indexRealmV1(path)||queue.hasId(id)||manifest.entryAt(id).operation==="delete"||!sharedV1(path,false,replica.ignore))continue;
  try{await lstat(absolute(settings.worktree,path));}
  catch(error){if(!gone(error))throw error;forget(replica,path);enqueueId(replica,id);count++;}
 }
 count+=disagreeing(replica);
 await rm(repairRequest(settings.worktree),{force:true});
 return count;
}

/** 🪢 Brings the applied state back in line with the manifest without touching the disk: a manifest entry whose applied row is missing or carries another hlc is queued (the follow then applies or drops it), an applied row without a manifest entry is forgotten (the next walk publishes the file if it still exists). Returns how many paths it touched. */
function disagreeing(replica:ReplicaV1):number{
 const {manifest,files,queue,book}=replica;
 let count=0;
 for(const id of manifest.ids()){
  if(queue.hasId(id)||indexRealmV1(book.name(id)))continue;
  if(!files.live(id)||!files.sameHlcAs(manifest,id)){enqueueId(replica,id);count++;}
 }
 for(const id of files.ids()){
  if(files.operationAt(id)==="delete"||manifest.live(id)||indexRealmV1(book.name(id)))continue;
  forget(replica,book.name(id));count++;
 }
 return count;
}

export function enqueueId(replica:ReplicaV1,id:number):void{
 replica.queue.addId(id);replica.changed.add(queueKey(id));replica.dirty=true;
}

function dequeue(replica:ReplicaV1,path:string):void{
 const id=replica.book.find(path);
 if(id===undefined)return;
 replica.queue.removeId(id);replica.changed.add(queueKey(id));replica.dirty=true;
}

const journalRepair=(replica:ReplicaV1)=>(repair:Readonly<{segment:number;bytes:number;found:number}>):void=>{replica.say({event:"journal-repaired",count:repair.segment,bytes:repair.bytes,done:repair.found,reason:"segment-shorter-than-head"});};

const indexHosts=new WeakMap<ReplicaV1,IndexHostV1>();

function indexHost(replica:ReplicaV1):IndexHostV1{
 const known=indexHosts.get(replica);
 if(known!==undefined)return known;
 const {manifest,files,queue,book,settings}=replica;
 const host:IndexHostV1={
  worktree:settings.worktree,server:settings.originId===`${settings.groupId}-server`,now:settings.now,
  base:()=>manifest.get(INDEX_BASE_V1),
  each:visit=>{for(const id of manifest.ids()){const path=book.name(id);if(path.startsWith(INDEX_PREFIX_V1))visit(path,manifest.entryAt(id));}},
  held:visit=>{for(const id of files.ids()){const path=book.name(id);if(indexRealmV1(path))visit(path,plain(files.entryAt(id)));}},
  pending:()=>{const found=new Map<string,FleetManifestEntryV1>();for(const id of queue.snapshot()){const path=book.name(id);if(indexRealmV1(path))found.set(path,manifest.entryAt(id));}return found;},
  emit:async events=>{
   try{
    await ensureHome(replica);
    const stamped:FleetEventV1[]=events.map(event=>({version:1,hlc:mint(replica),operation:event.operation,path:event.path,...(event.target===undefined?{}:{target:event.target}),base:null}));
    const head=await appendEventsV1(settings.storeRoot,settings.originId,stamped,journalRepair(replica));
    replica.cursors[settings.originId]={segment:head.segment,bytes:head.bytes};
    for(const event of stamped){
     const entry:StateEntry=event.operation==="delete"?{hlc:event.hlc,operation:"delete",mtimeMs:0}:{hlc:event.hlc,operation:"link",target:event.target!,mtimeMs:0},id=book.intern(event.path);
     files.write(id,entry);manifest.set(event.path,plain(entry));replica.changed.add(filesKey(id));
    }
    replica.dirty=true;
    return stamped.map(event=>event.hlc);
   }catch(error){
    if(!sharing(error))throw error;
    noteOnce(replica,`blocked:journal:${code(error)}`,{event:"blocked",reason:code(error)});
    return undefined;
   }
  },
  settle:paths=>{
   for(const path of paths){
    const id=book.find(path);
    if(id===undefined||!manifest.live(id))continue;
    files.write(id,{...plain(manifest.entryAt(id)),mtimeMs:0});replica.changed.add(filesKey(id));dequeue(replica,path);
   }
   replica.dirty=true;
  },
  enqueue:paths=>{for(const path of paths)enqueueId(replica,book.intern(path));},
  say:event=>{replica.say(event);}
 };
 indexHosts.set(replica,host);
 return host;
}

function requeue(replica:ReplicaV1):void{
 const {files,manifest,queue}=replica;
 for(const id of manifest.ids())if(!queue.hasId(id)&&(!files.live(id)||!files.sameHlcAs(manifest,id)))enqueueId(replica,id);
}

function noteOnce(replica:ReplicaV1,key:string,event:Omit<ReplicaEventV1,"version">):void{
 const {announce,count}=replica.failures.note(key,event.message??event.reason??key);
 if(replica.noted.size>=noteCapacity)replica.noted.clear();
 replica.noted.add(key);
 if(announce)replica.say(count>1?{...event,count}:event);
}

function resolved(replica:ReplicaV1,key:string):void{replica.noted.delete(key);replica.failures.clear(key);}

/** 🧵️ After a crash the own journal can be ahead of the persisted state (events appended, state not yet written): the events after the persisted own cursor are replayed into the state with an unknown modification time, so that the next scan rehashes those files and publishes a delete for each one that left the disk meanwhile. */
async function replayOwnTail(replica:ReplicaV1):Promise<void>{
 const {storeRoot,originId}=replica.settings,cursor=replica.cursors[originId];
 let read:Awaited<ReturnType<typeof readEventSpansV1>>;
 try{
  const head=await readHeadV1(storeRoot,originId);
  if(head===undefined||cursor!==undefined&&cursor.segment===head.segment&&cursor.bytes===head.bytes)return;
  read=await readEventSpansV1(storeRoot,originId,cursor);
 }catch{return;}
 for(const event of read.events){
  const known=replica.files.get(event.path);
  if(known!==undefined&&known.hlc>=event.hlc)continue;
  if(indexRealmV1(event.path)){
   const stage:StateEntry=event.operation==="delete"?{hlc:event.hlc,operation:"delete",mtimeMs:0}:{hlc:event.hlc,operation:"link",target:event.target!,mtimeMs:0},id=replica.book.intern(event.path);
   replica.files.write(id,stage);replica.manifest.set(event.path,plain(stage));replica.changed.add(filesKey(id));
   continue;
  }
  const entry:StateEntry=event.operation==="delete"?{hlc:event.hlc,operation:"delete",mtimeMs:0}:event.operation==="link"?{hlc:event.hlc,operation:"link",target:event.target!,mtimeMs:0}:{hlc:event.hlc,operation:"put",sha256:event.sha256!,bytes:event.bytes!,executable:event.executable!,mtimeMs:0};
  setFile(replica,event.path,entry);replica.manifest.set(event.path,plain(entry));
 }
 replica.cursors[originId]=read.cursor;replica.dirty=true;
}

type HomeRecord=Readonly<{group?:string;key?:string}>;

async function readHome(storeRoot:string,originId:string):Promise<HomeRecord|undefined>{
 try{
  const value=JSON.parse(await readFile(join(storePathsV1(storeRoot).origins,originId,"home.json"),"utf8")) as unknown;
  requireReplicaShapeV1("Home",value,"Closed replica home required");
  return value as HomeRecord;
 }catch{return undefined;}
}

/** 🚪️ Opens a replica: takes the single-daemon lock (stale locks of dead pids are taken over), clears torn staging files, streams machine, state, cursors and clock into the compact tables, recovers the clock from the own head and queues every store winner the tree does not hold yet. */
export async function openReplicaV1(options:ReplicaOptionsV1,sink:ReplicaSinkV1=()=>{}):Promise<ReplicaV1>{
 const settings=settle(options),paths=localPathsV1(settings.worktree);
 await mkdir(paths.tmp,{recursive:true});
 const release=options.lock===false?async()=>{}:await acquireLock(paths,settings.originId);
 try{
  await rm(paths.tmp,{recursive:true,force:true});await mkdir(paths.tmp,{recursive:true});
  if(options.lock!==false)await reclaimStagingV1(settings.storeRoot,{originId:settings.originId});
  let tables=tablesV1(),local:Local,corrupt:string|undefined;
  try{local=await loadLocal(paths,tables,"open");}
  catch(error){if(!(error instanceof StateCorruptV1))throw error;corrupt=error.message;tables=tablesV1();local=await loadLocal(paths,tables,"ignore");}
  if(local.log.size!==local.log.good)await truncate(paths.wal,local.log.good);
  if(local.machine!==undefined&&local.machine.originId!==settings.originId)throw Error(`This worktree belongs to origin ${local.machine.originId}, not ${settings.originId}`);
  let key=local.machine?.key??randomBytes(16).toString("hex");
  const home=await readHome(settings.storeRoot,settings.originId),torn=local.machine===undefined&&await readFile(paths.machine,"utf8").then(()=>true,()=>false);
  if(home?.key!==undefined&&home.key!==key){if(torn||corrupt!==undefined)key=home.key;else throw new OriginTakenV1(settings.originId);}
  const machine:Machine={version:1,originId:settings.originId,groupId:settings.groupId,storeRoot:settings.storeRoot,key};
  await writeJson(paths,paths.machine,machine);
  let clock=local.clock;
  try{const head=await readHeadV1(settings.storeRoot,settings.originId);if(head)clock=mergeClockV1(clock,head.hlc,Number.MAX_SAFE_INTEGER-futureMilliseconds);}catch{}
  const text=await readIgnoreText(paths);
  const replica:ReplicaV1={settings,paths,book:tables.book,files:tables.files,queue:tables.queue,manifest:tables.manifest,cursors:local.cursors,folds:tables.folds,noted:new Set(),failures:createFailuresV1({now:settings.now}),key,applying:0,changed:new Set(),held:new Map(),hot:new HotSetV1(),warm:new HotSetV1(5000),dirs:tables.dirs,latency:new LatencyV1(),startedMs:settings.now(),memo:local.memo,memoDirty:new Set(),say:event=>sink({version:1,...event}),release,clock,ignore:guarded(compileIgnoreV1(text)),ignoreHash:digest(text),dirty:false,persistedAt:settings.now(),homeWritten:false,generation:local.generation,logBytes:local.log.good,activity:0,checkpointBytes:local.checkpointBytes,joined:local.machine!==undefined};
  if(corrupt!==undefined){
   replica.say({event:"state-rebuilt",reason:corrupt.includes("missing")?"missing":"corrupt",message:corrupt});
   await rebuildFromStore(replica,sink);
  }
  restoreHistory(replica);
  const hot=local.memo.get("hot");
  if(hot!==undefined)for(const path of Buffer.from(hot).toString("utf8").split("\0"))if(path!=="")replica.hot.note(path,settings.now());
  requeue(replica);
  await replayOwnTail(replica);
  if(local.machine!==undefined&&corrupt===undefined&&(local.source==="takeover"||local.mixed)){
   try{const count=await takeOverRegistersV1(replica);if(count>0)replica.say({event:"repaired",count,reason:"state-takeover"});}catch{}
  }
  if(corrupt!==undefined||local.source==="takeover"){await rm(paths.wal,{force:true});await checkpointReplicaV1(replica);}
  else if(local.mixed)await checkpointReplicaV1(replica);
  else if(local.source==="empty")await checkpointReplicaV1(replica);
  else{
   const older=Math.max(await mtimeOf(paths.takeoverState),await mtimeOf(paths.takeoverLog));
   if(older>0&&settings.now()-older>compatWindowMilliseconds){await rm(paths.takeoverState,{force:true});await rm(paths.takeoverLog,{force:true});}
  }
  settleHeap();
  const ahead=Math.ceil((replica.clock.ms-settings.now()-futureMilliseconds)/1000);
  if(ahead>0)replica.say({event:"clock-refused",reason:"own-clock-ahead",seconds:ahead});
  return replica;
 }catch(error){await release();throw error;}
}

async function rebuildFromStore(replica:ReplicaV1,sink:ReplicaSinkV1):Promise<"rebuilt"|"empty"|"failed">{
 try{
  if(!await storeReachableV1(replica.settings.storeRoot))return "failed";
  const folded=await foldInto(replica.settings.storeRoot,replica.manifest,undefined,sink,replica.settings.now);
  for(const key of Object.keys(replica.cursors))delete replica.cursors[key];
  Object.assign(replica.cursors,folded.cursors);
  if(folded.hlc!==""){try{replica.clock=mergeClockV1(replica.clock,folded.hlc,replica.settings.now());}catch{}}
  const ids=replica.manifest.ids();
  for(const id of ids)enqueueId(replica,id);
  return ids.length>0?"rebuilt":"empty";
 }catch{replica.manifest.clear();return "failed";}
}

const stormThreshold=1000;

async function stormChecked(replica:ReplicaV1,found:ReplicaFoundV1,signal?:AbortSignal):Promise<boolean>{
 const {files,queue}=replica,unknown:number[]=[];
 for(let index=0;index<found.length;index++){const id=found.ids[index]!;if(!files.live(id)&&!queue.hasId(id))unknown.push(id);}
 if(unknown.length<=stormThreshold)return true;
 try{await takeOverRegistersV1(replica,signal);}
 catch(error){if(signal?.aborted)throw error;noteOnce(replica,"state-missing",{event:"state-missing",reason:"store-fold-failed"});return false;}
 const held=unknown.filter(id=>queue.hasId(id)).length;
 if(held>stormThreshold){replica.say({event:"state-missing",reason:"republish-storm",count:held});replica.say({event:"repaired",count:held,reason:"republish-storm"});}
 return true;
}

async function stateReady(replica:ReplicaV1):Promise<boolean>{
 if(!replica.joined||replica.files.size>0||replica.queue.size>0)return true;
 const outcome=await rebuildFromStore(replica,event=>{replica.say(event);});
 if(outcome==="failed"){noteOnce(replica,"state-missing",{event:"state-missing",reason:"store-fold-failed"});return false;}
 replica.noted.delete("state-missing");replica.joined=false;
 if(outcome==="rebuilt")replica.say({event:"state-rebuilt",reason:"missing"});
 return true;
}

function sparse(dirs:Int32Array):Int32Array{
 let count=0;
 for(const value of dirs)if(value!==0)count++;
 const pairs=new Int32Array(count*2);
 let at=0;
 for(let index=0;index<dirs.length;index++)if(dirs[index]!==0){pairs[at++]=index;pairs[at++]=dirs[index]!;}
 return pairs;
}

function checkpointOf(replica:ReplicaV1,generation:number):CheckpointV1{
 const {book,files,queue,manifest,folds,memo}=replica,known=replica.dirs.columns(),columns=book.columns(),rows=Math.min(files.flags.length,book.count),ids=queue.heldSnapshot(),count=ids.length,held=files.columns(rows);
 const queued={flags:new Uint8Array(count),ms:new Float64Array(count),counter:new Uint32Array(count),origin:new Uint16Array(count),bytes:new Float64Array(count),sha:new Uint8Array(count*32)},targets:[number,string][]=[];
 ids.forEach((id,row)=>{
  queued.flags[row]=manifest.flags[id]!;queued.ms[row]=manifest.ms[id]!;queued.counter[row]=manifest.counter[id]!;queued.origin[row]=manifest.origin[id]!;queued.bytes[row]=manifest.bytes[id]!;
  queued.sha.set(manifest.sha.subarray(id*32,id*32+32),row*32);
  const target=manifest.targets.get(id);
  if(target!==undefined)targets.push([id,target]);
 });
 const places=folds.columns(),last=columns.pages.length-1,part=(name:string,...bytes:Uint8Array[]):Readonly<{name:string;bytes:readonly Uint8Array[]}>=>({name,bytes});
 return {generation,paths:book.count,files:files.size,queue:count,parts:[
  part("meta",bytesOfV1(Float64Array.of(files.shared,places.used,rows,0))),part("origins",encoder.encode(book.origins.join("\n"))),part("fold",files.fold),
  part("book.starts",bytesOfV1(columns.starts)),part("book.lengths",bytesOfV1(columns.lengths)),part("book.hashes",bytesOfV1(columns.hashes)),part("book.folded",bytesOfV1(columns.folded)),
  part("book.pages",...columns.pages.slice(0,Math.max(0,last)),...(last>=0?[columns.pages[last]!.subarray(0,columns.used)]:[])),
  part("files.flags",held.flags),part("files.ms",bytesOfV1(held.ms)),part("files.counter",bytesOfV1(held.counter)),part("files.origin",bytesOfV1(held.origin)),part("files.bytes",bytesOfV1(held.bytes)),part("files.sha",held.sha),part("files.mtime",bytesOfV1(held.mtime)),part("files.seen",bytesOfV1(held.seen)),part("files.targets",targetBlob(files.targets)),
  part("queue.ids",bytesOfV1(ids)),part("queue.flags",queued.flags),part("queue.ms",bytesOfV1(queued.ms)),part("queue.counter",bytesOfV1(queued.counter)),part("queue.origin",bytesOfV1(queued.origin)),part("queue.bytes",bytesOfV1(queued.bytes)),part("queue.sha",queued.sha),part("queue.targets",targetBlob(targets)),
  part("folds.slots",bytesOfV1(places.slots)),part("folds.dirs",bytesOfV1(sparse(places.dirs))),part("folds.parents",bytesOfV1(sparse(places.parents))),
  part("dirs.mtime",bytesOfV1(known.mtime)),part("dirs.present",known.present),part("dirs.members",bytesOfV1(known.members)),
  ...[...memo].map(([name,value])=>part(`memo:${name}`,value))
 ]};
}

/** 💽️ Writes a checkpoint of the whole local state now (the delta log restarts empty). `persist` calls it when the log has outgrown its limit; tests and tools call it directly. */
export async function checkpointReplicaV1(replica:ReplicaV1):Promise<void>{
 keepState(replica);
 const generation=replica.generation+1,{paths}=replica;
 await mkdir(paths.tmp,{recursive:true});
 replica.checkpointBytes=await writeCheckpointV1(paths.checkpoint,join(paths.tmp,token()),checkpointOf(replica,generation));
 replica.generation=generation;
 await writeFile(paths.wal,"");
 replica.logBytes=0;replica.memoDirty.clear();
}

async function appendLog(replica:ReplicaV1,keys:readonly number[]):Promise<void>{
 const {book,files,queue,manifest,memo}=replica;
 const records=function*():Generator<WalRecordV1>{
  for(const key of keys){
   const id=Math.floor(key/2),isFiles=key%2===0;
   yield {kind:isFiles?"files":"queue",path:book.name(id),entry:isFiles?(files.live(id)?files.entryAt(id):null):(queue.heldId(id)?manifest.entryAt(id):null)};
  }
  for(const name of replica.memoDirty)yield {kind:"memo",name,value:memo.get(name)??null};
 };
 replica.logBytes+=await appendWalV1(replica.paths.wal,replica.generation,replica.logBytes===0,payloadsV1(records()));
 replica.memoDirty.clear();
}

export async function persist(replica:ReplicaV1,force:boolean):Promise<void>{
 const limit=Math.max(walLimitBytes,replica.checkpointBytes>>3),roll=replica.logBytes>limit||replica.changed.size>checkpointChanges;
 if(!replica.dirty&&!roll)return;
 const now=replica.settings.now();
 if(!force&&now-replica.persistedAt<replica.settings.persistMilliseconds)return;
 const keys=[...replica.changed];
 replica.dirty=false;replica.persistedAt=now;replica.changed.clear();
 try{
  keepState(replica);keepHistory(replica,force);
  if(roll)await checkpointReplicaV1(replica);
  else if(keys.length>0||replica.memoDirty.size>0){await appendLog(replica,keys);if(replica.logBytes>limit)await checkpointReplicaV1(replica);}
 }catch(error){
  replica.dirty=true;replica.logBytes=Number.MAX_SAFE_INTEGER;
  for(const key of keys)replica.changed.add(key);
  throw error;
 }
 if(roll)settleHeap();
}

function keepState(replica:ReplicaV1):void{
 for(const [name,value] of [["clock",{ms:replica.clock.ms,counter:replica.clock.counter}],["cursors",replica.cursors]] as const){
  const bytes=Buffer.from(JSON.stringify(value)),held=replica.memo.get(name);
  if(held===undefined||Buffer.compare(held,bytes)!==0){replica.memo.set(name,bytes);replica.memoDirty.add(name);}
 }
}

/** 🧠️ Keeps a named blob (at most 18 characters of `a-z0-9.-`) with the local state: it joins the delta log on the next persist and the next checkpoint, and is back after a restart; `undefined` forgets it. */
export function rememberV1(replica:ReplicaV1,name:string,value:Uint8Array|undefined):void{
 if(!/^[a-z0-9.-]{1,18}$/u.test(name))throw Error(`Closed replica memo name required: ${name}`);
 if(value===undefined)replica.memo.delete(name);else replica.memo.set(name,value);
 replica.memoDirty.add(name);replica.dirty=true;
}

/** 🧠️ Reads a blob kept with `rememberV1`. */
export function recallV1(replica:ReplicaV1,name:string):Uint8Array|undefined{return replica.memo.get(name);}

/** 🔚️ Persists the clock, the pending delta of the state (appended to the log; a checkpoint is written only when the log has outgrown its limit) and the cursors, then releases the daemon lock. */
export async function closeReplicaV1(replica:ReplicaV1):Promise<void>{
 try{
  const hot=replica.hot.snapshot(replica.settings.now());
  if(hot.length>0||replica.memo.has("hot"))rememberV1(replica,"hot",hot.length>0?encoder.encode(hot.join("\0")):undefined);
  keepHistory(replica,true);
  await persist(replica,true);
 }finally{await replica.release();}
}

const ignoreStamps=new WeakMap<ReplicaV1,{signature:string;at:number}>(),ignoreRevalidateMilliseconds=2000;

async function refreshIgnore(replica:ReplicaV1):Promise<boolean>{
 const info=await stat(replica.paths.gitignore).catch(()=>undefined),signature=info===undefined?"":`${info.ino}:${info.size}:${info.mtimeMs}`,known=ignoreStamps.get(replica),now=Date.now();
 if(known!==undefined&&known.signature===signature&&now-known.at<ignoreRevalidateMilliseconds)return false;
 ignoreStamps.set(replica,{signature,at:now});
 const text=await readIgnoreText(replica.paths),hash=digest(text);
 if(hash===replica.ignoreHash)return false;
 if(text===""&&replica.files.get(".gitignore")?.operation==="put")return false;
 replica.ignore=guarded(compileIgnoreV1(text));replica.ignoreHash=hash;
 replica.say({event:"ignore-reloaded",count:text===""?0:text.split("\n").length});
 requeue(replica);
 return true;
}

function mint(replica:ReplicaV1):string{
 const wall=replica.settings.now();
 if(replica.clock.ms>wall+futureMilliseconds)throw new ClockAheadV1(Math.ceil((replica.clock.ms-wall)/1000));
 replica.clock=tickClockV1(replica.clock,wall);replica.dirty=true;
 return formatHlcV1(replica.clock,replica.settings.originId);
}

async function ensureHome(replica:ReplicaV1):Promise<void>{
 if(replica.homeWritten)return;
 const {origins,tmp}=storePathsV1(replica.settings.storeRoot),directory=join(origins,replica.settings.originId),file=join(directory,"home.json");
 for(let attempt=0;attempt<3;attempt++){
  const existing=await readHome(replica.settings.storeRoot,replica.settings.originId);
  if(existing?.key!==undefined&&existing.key!==replica.key)throw new OriginTakenV1(replica.settings.originId);
  if(existing?.key===replica.key){replica.homeWritten=true;return;}
  await ensureStoreDirectoryV1(replica.settings.storeRoot,directory);await ensureStoreDirectoryV1(replica.settings.storeRoot,tmp);
  const temporary=join(tmp,`${replica.settings.originId}-${process.pid}-${token()}`);
  await writeFile(temporary,JSON.stringify({version:1,group:replica.settings.groupId,key:replica.key}));
  try{
   if(existing===undefined)await link(temporary,file);else await rename(temporary,file);
   replica.homeWritten=true;return;
  }catch(error){if(code(error)!=="EEXIST")throw error;}
  finally{await rm(temporary,{force:true});}
 }
 throw new OriginTakenV1(replica.settings.originId);
}

type Job=Readonly<{entry:FleetScanEntryV1;current:StateEntry|undefined}>;
type Prepared=Readonly<{job:Job;sha256?:string;bytes?:number;executable?:boolean;touch:boolean}>;

const throttled=(emit:(done:number,total:number)=>void):((done:number,total:number)=>void)=>{let last=0;return (done,total)=>{const now=Date.now();if(done>=total||now-last>=250){last=now;emit(done,total);}};};

/** 🔑️ The executable bit of a path as the git index of the fleet records it (`100755` in its register): the only source a machine that cannot represent the bit consults for a file it creates; a path without an entry is not executable, and a POSIX machine sets the bit later. */
const stagedExecutable=(replica:ReplicaV1,path:string):boolean=>{
 const staged=replica.files.get(`${INDEX_PREFIX_V1}${path}`);
 return staged?.operation==="link"&&staged.target!.startsWith("100755 ");
};

async function prepare(replica:ReplicaV1,job:Job,signal?:AbortSignal):Promise<Prepared|undefined>{
 const {entry,current}=job;
 if(entry.kind==="link")return {job,touch:false};
 const target=absolute(replica.settings.worktree,entry.path),{storeRoot,originId}=replica.settings;
 const progress=entry.bytes>=largeBlobBytesV1?throttled((done,total)=>replica.say({event:"progress",stage:"publish-blob",path:entry.path,done,total})):undefined;
 const unstable=():undefined=>{noteOnce(replica,`unstable:${entry.path}`,{event:"unstable",path:entry.path});return undefined;};
 try{
  const executable=replica.settings.modeless?(current?.operation==="put"?current.executable!:stagedExecutable(replica,entry.path)):entry.executable;
  if(current?.operation==="put"&&current.bytes===entry.bytes){
   const hash=await hashFileV1(target);
   if(!hash.stable)return unstable();
   if(current.sha256===hash.sha256&&current.executable===executable)return {job,sha256:hash.sha256,bytes:hash.bytes,executable,touch:true};
   const stored=await storeBlobV1(storeRoot,originId,target,{known:{sha256:hash.sha256,bytes:hash.bytes},signal,progress});
   return stored.stable?{job,sha256:hash.sha256,bytes:hash.bytes,executable,touch:false}:unstable();
  }
  const stored=await storeBlobV1(storeRoot,originId,target,{signal,progress});
  return stored.stable?{job,sha256:stored.sha256,bytes:stored.bytes,executable,touch:false}:unstable();
 }catch(error){
  if(gone(error)||code(error)==="EISDIR")return undefined;
  if(sharing(error)){noteOnce(replica,`blocked:${entry.path}:${code(error)}`,{event:"blocked",path:entry.path,reason:code(error)});return undefined;}
  throw error;
 }
}

const parentOf=(path:string):string=>{const cut=path.lastIndexOf("/");return cut<0?"":path.slice(0,cut);};

/** 🧭️ Tells whether a proper ancestor directory of a worktree-relative path is a symbolic link (or not a directory at all): such a path is reached through the link, it is not a file of this tree. Each component is inspected with `lstat`, never followed; `verified` remembers the ancestors that were real directories. */
async function ancestryOf(root:string,path:string,verified:Set<string>):Promise<"ok"|"symlink-in-the-way"|"parent-is-a-file">{
 for(let cut=path.indexOf("/");cut>0;cut=path.indexOf("/",cut+1)){
  const directory=path.slice(0,cut);
  if(verified.has(directory))continue;
  let info:Stats;
  try{info=await lstat(absolute(root,directory));}catch(error){if(gone(error))return "ok";throw error;}
  if(info.isSymbolicLink())return "symlink-in-the-way";
  if(!info.isDirectory())return "parent-is-a-file";
  verified.add(directory);
 }
 return "ok";
}

async function relist(replica:ReplicaV1,directory:string,queue:Set<string>,shallow:Set<string>,vanished:string[]):Promise<void>{
 const root=replica.settings.worktree,target=directory===""?root:absolute(root,directory);
 let info:Stats;
 try{
  if(directory!==""&&await ancestryOf(root,directory,new Set())==="symlink-in-the-way"){replica.dirs.delete(directory);return;}
  info=await lstat(target);
 }catch(error){
  if(!gone(error))throw error;
  replica.dirs.delete(directory);
  if(directory!==""&&!replica.files.has(directory))vanished.push(directory);
  return;
 }
 if(!info.isDirectory()){queue.add(directory);if(!replica.files.has(directory))vanished.push(directory);return;}
 if(directory!==""&&!sharedV1(directory,true,replica.ignore)){replica.dirs.delete(directory);return;}
 replica.dirs.set(directory,info.mtimeMs);shallow.add(directory);
 for(const child of await readdir(target,{withFileTypes:true})){
  const name=child.name.normalize("NFC"),next=directory===""?name:`${directory}/${name}`;
  if(!child.isDirectory())queue.add(next);
  else if(!replica.dirs.has(next))queue.add(`${next}/`);
 }
}

async function observe(replica:ReplicaV1,paths:readonly string[],flat:readonly string[]=[]):Promise<Readonly<{entries:FleetScanEntryV1[];missing:string[]}>>{
 const entries:FleetScanEntryV1[]=[],missing:string[]=[],root=replica.settings.worktree,queue=new Set(paths),expanded=new Set<string>(),shallow=new Set<string>(),vanished:string[]=[],reals=new Set<string>();
 for(const directory of flat)await relist(replica,directory,queue,shallow,vanished);
 for(const marked of queue){
  const expand=marked.endsWith("/"),path=expand?marked.slice(0,-1):marked;
  if(unsafePath(path))continue;
  if(await ancestryOf(root,path,reals)==="symlink-in-the-way"){
   if(expand)continue;
   missing.push(path);replica.dirs.delete(path);
   if(!replica.files.has(path))vanished.push(path);
   continue;
  }
  const target=absolute(root,path);
  let info:Stats;
  try{info=await lstat(target);}catch(error){
   if(code(error)==="ENAMETOOLONG"){if(!expand)noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"too-long"});continue;}
   if(!gone(error))throw error;
   if(expand)continue;
   missing.push(path);replica.dirs.delete(path);
   if(!replica.files.has(path))vanished.push(path);
   continue;
  }
  if(info.isDirectory()){
   if(sharedV1(path,true,replica.ignore)){expanded.add(path);replica.dirs.set(path,info.mtimeMs);for(const child of await readdir(target)){const next=`${path}/${child.normalize("NFC")}`;queue.add(next);queue.add(`${next}/`);}}
   continue;
  }
  if(expand)continue;
  if(!sharedV1(path,false,replica.ignore)){
   const nestedGit=path.split("/").includes(".git")&&!path.startsWith(".git/")&&path!==".git";
   const reason=gitPrivateV1(path)||nestedGit||replica.ignore.ignores(path,false)?undefined:refusePathV1(path);
   if(reason!==undefined)noteOnce(replica,`refused:${path}:${reason}`,{event:"refused",path,reason});
   continue;
  }
  if(!asciiOnly.test(path)&&!await spelled(root,path)){noteOnce(replica,`refused:${path}:not-nfc`,{event:"refused",path,reason:"not-nfc"});continue;}
  const other=replica.folds.find(path);
  if(other!==undefined&&other!==path){
   if(await listed(root,other))continue;
   missing.push(other);
  }
  if(info.isSymbolicLink()){
   const link=await readlink(target);
   if(linkRefusalV1(path,link)===undefined)entries.push({path,kind:"link",bytes:info.size,mtimeMs:info.mtimeMs,executable:false,target:link});
  }else if(info.isFile())entries.push({path,kind:"file",bytes:info.size,mtimeMs:info.mtimeMs,executable:!replica.settings.modeless&&(info.mode&0o111)!==0});
 }
 const kept=new Map<string,number>();
 if(expanded.size>0)for(const entry of entries){
  const id=replica.book.find(entry.path);
  if(id===undefined||!replica.files.live(id)||replica.files.operationAt(id)==="delete")continue;
  for(let cut=entry.path.lastIndexOf("/");cut>0;cut=entry.path.lastIndexOf("/",cut-1)){const parent=entry.path.slice(0,cut);if(expanded.has(parent))kept.set(parent,(kept.get(parent)??0)+1);}
 }
 if([...expanded].some(directory=>replica.folds.below(directory)>(kept.get(directory)??0))){
  let seen=0;
  for(const id of replica.files.ids()){
   if(++seen%20000===0)await new Promise<void>(done=>{setImmediate(done);});
   if(replica.files.operationAt(id)==="delete"||replica.queue.hasId(id))continue;
   const name=replica.book.name(id);
   if(queue.has(name))continue;
   for(let cut=name.lastIndexOf("/");cut>0;cut=name.lastIndexOf("/",cut-1))if(expanded.has(name.slice(0,cut))){missing.push(name);break;}
  }
 }
 if(shallow.size>0){
  const kept=new Map<string,number>();
  for(const entry of entries){
   const parent=parentOf(entry.path),id=shallow.has(parent)?replica.book.find(entry.path):undefined;
   if(id!==undefined&&replica.files.live(id)&&replica.files.operationAt(id)!=="delete")kept.set(parent,(kept.get(parent)??0)+1);
  }
  const lacking=new Set([...shallow].filter(directory=>replica.folds.inside(directory)>(kept.get(directory)??0)));
  if(lacking.size>0){
   let seen=0;
   for(const id of replica.files.ids()){
    if(++seen%20000===0)await new Promise<void>(done=>{setImmediate(done);});
    if(replica.files.operationAt(id)==="delete"||replica.queue.hasId(id))continue;
    const name=replica.book.name(id);
    if(lacking.has(parentOf(name))&&!queue.has(name))missing.push(name);
   }
  }
 }
 if(vanished.length>0){
  const gonePaths=new Set(vanished);
  let seen=0;
  for(const id of replica.files.ids()){
   if(++seen%20000===0)await new Promise<void>(done=>{setImmediate(done);});
   if(replica.files.operationAt(id)==="delete")continue;
   const name=replica.book.name(id);
   for(let cut=name.lastIndexOf("/");cut>0;cut=name.lastIndexOf("/",cut-1))if(gonePaths.has(name.slice(0,cut))){missing.push(name);break;}
  }
 }
 return {entries,missing};
}

const unchangedSince=(replica:ReplicaV1,id:number,entry:FleetScanEntryV1):boolean=>{
 const {files,queue}=replica;
 if(queue.hasId(id))return true;
 const operation=files.operationAt(id);
 if(entry.kind==="link")return operation==="link"&&files.targets.get(id)===entry.target;
 return operation==="put"&&files.bytes[id]===entry.bytes&&files.mtime[id]===entry.mtimeMs&&!files.racyAt(id)&&(replica.settings.modeless||files.executableAt(id)===entry.executable);
};

/** 🚶️ Walks the shared tree once and keeps only what a publish needs: the entries that differ from the recorded state, and one bit per known path that was seen. The tree is never held as a list, so a walk of 270k files costs memory in proportion to what changed. */
export async function walkLocalV1(replica:ReplicaV1,signal?:AbortSignal):Promise<WalkV1>{
 const seen=new Bits(),changed=new ReplicaFoundV1(replica.book),universe=replica.book.count;
 let scanned=0;
 const pace=pacer(paceEntries),{refused}=await scanSharedV1(replica.settings.worktree,replica.ignore,signal,entry=>{
  scanned++;pace();
  const id=replica.book.find(entry.path);
  if(id!==undefined){seen.mark(id);if(unchangedSince(replica,id,entry))return;}
  changed.push(entry);
 },(path,mtimeMs)=>{replica.dirs.set(path,mtimeMs);});
 return {changed,seen,universe,scanned,refused};
}

/** 🎭️ Publishes the stage of this machine; a failure is a progress event, never an exception. A modeless replica does it before the files too, so that the staged mode of a new file is known when the file is published. */
async function publishStage(replica:ReplicaV1,signal?:AbortSignal):Promise<void>{
 try{await publishIndexV1(indexHost(replica),signal);}
 catch(error){if(signal?.aborted)throw error;noteOnce(replica,`error:git:${messageOf(error)}`,{event:"error",stage:"git",message:messageOf(error)});}
}

/** 📤️ One publish round: walks the shared tree (or only the given touched paths), compares by size and mtime and then by hash, writes blobs before events, appends batched events with `base` = the previous sha256, and updates state and clock; deletions are paths in state that are gone from disk. A `walked` result from `walkLocalV1` replaces the walk. */
export async function publishOnceV1(replica:ReplicaV1,signal?:AbortSignal,paths?:readonly string[],walked?:WalkV1,flat?:readonly string[],cold?:boolean):Promise<PublishResultV1>{
 if(replica.settings.modeless&&(paths===undefined||paths.includes(".git/index")))await publishStage(replica,signal);
 const {worktree,concurrency,storeRoot,originId}=replica.settings;
 await refreshIgnore(replica);
 if(!await stateReady(replica))return {scanned:0,published:0,deleted:0,touched:0,bytes:0,retry:[]};
 let found:ReplicaFoundV1,candidates:string[],scanned:number;
 if(paths===undefined){
  const walk=walked??await walkLocalV1(replica,signal);
  for(const refusal of walk.refused)noteOnce(replica,`refused:${refusal.path}:${refusal.reason}`,{event:"refused",path:refusal.path,reason:refusal.reason});
  found=walk.changed;scanned=walk.scanned;candidates=[];
  for(const id of replica.files.ids())if(id<walk.universe&&!walk.seen.has(id)&&replica.files.operationAt(id)!=="delete"&&!replica.queue.hasId(id)&&!indexRealmV1(replica.book.name(id)))candidates.push(replica.book.name(id));
  replica.say({event:"scan",count:scanned,total:candidates.length});
 }else{
  const seen=await observe(replica,paths,flat);
  found=new ReplicaFoundV1(replica.book);for(const entry of seen.entries)found.push(entry);scanned=found.length;candidates=[...new Set(seen.missing)].filter(path=>{const state=replica.files.get(path);return state!==undefined&&state.operation!=="delete"&&!replica.queue.has(path);});
 }
 if(paths===undefined&&!await stormChecked(replica,found,signal))return {scanned,published:0,deleted:0,touched:0,bytes:0,retry:[]};
 const deletions:string[]=[];
 await pool(candidates,concurrency,async path=>{
  if(!sharedV1(path,false,replica.ignore)){forget(replica,path);return;}
  try{
   const info=await lstat(absolute(worktree,path));
   if(info.isDirectory()||!await listed(worktree,path))deletions.push(path);else forget(replica,path);
  }catch(error){if(gone(error))deletions.push(path);else throw error;}
 },signal);
 deletions.sort();
 let touched=0,published=0,bytes=0,deleted=0,cursor=0,rounds=0;
 const retry:string[]=[],noted:string[]=[];
 while(cursor<found.length){
  signal?.throwIfAborted();
  const slice:Job[]=[];
  while(cursor<found.length&&slice.length<chunkSize){
   const entry=found.at(cursor++);
   if(replica.queue.has(entry.path))continue;
   const current=replica.files.get(entry.path);
   if(entry.kind==="link"){if(current?.operation==="link"&&current.target===entry.target)continue;}
   else if(current?.operation==="put"&&current.bytes===entry.bytes&&current.mtimeMs===entry.mtimeMs&&!racy(current)&&(replica.settings.modeless||current.executable===entry.executable))continue;
   slice.push({entry,current});
  }
  if(slice.length===0)continue;
  const prepared:(Prepared|undefined)[]=new Array(slice.length).fill(undefined);
  const seenMs=replica.settings.now();
  await pool(slice,concurrency,async (job,index)=>{prepared[index]=await prepare(replica,job,signal);},signal);
  const events:FleetEventV1[]=[],updates:[string,StateEntry,boolean][]=[];
  prepared.forEach((item,index)=>{
   const job=slice[index]!,{entry,current}=job;
   if(item===undefined){retry.push(entry.path);return;}
   if(item.touch){updates.push([entry.path,{...plain(current!),mtimeMs:entry.mtimeMs,bytes:item.bytes!,seenMs},false]);touched++;return;}
   const hlc=mint(replica),base=current?.operation==="put"?current.sha256!:null;
   if(paths!==undefined&&cold!==true)replica.latency.publish(seenMs-entry.mtimeMs);
   noted.push(entry.path);
   if(entry.kind==="link"){
    events.push({version:1,hlc,operation:"link",path:entry.path,target:entry.target!,base});
    updates.push([entry.path,{hlc,operation:"link",target:entry.target!,mtimeMs:entry.mtimeMs},true]);
   }else{
    events.push({version:1,hlc,operation:"put",path:entry.path,sha256:item.sha256!,bytes:item.bytes!,executable:item.executable!,base});
    updates.push([entry.path,{hlc,operation:"put",sha256:item.sha256!,bytes:item.bytes!,executable:item.executable!,mtimeMs:entry.mtimeMs,seenMs},true]);
    bytes+=item.bytes!;
   }
  });
  if(events.length>0){
   try{
    await ensureHome(replica);
    const head=await appendEventsV1(storeRoot,originId,events,journalRepair(replica));
    replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
    for(const event of events)track(replica,event);
    published+=events.length;
   }catch(error){
    if(!sharing(error))throw error;
    for(const [path,,fresh] of updates)if(fresh)retry.push(path);
    updates.splice(0,updates.length,...updates.filter(update=>!update[2]));
    noteOnce(replica,`blocked:journal:${code(error)}`,{event:"blocked",reason:code(error)});
   }
  }
  for(const [path,state,fresh] of updates){
   setFile(replica,path,state);if(fresh)replica.manifest.set(path,plain(state));
  }
  await persist(replica,false);
  if(++rounds%8===0)settleHeap();
  replica.say({event:"progress",stage:"publish",done:cursor,total:found.length});
 }
 for(let start=0;start<deletions.length;start+=chunkSize*2){
  signal?.throwIfAborted();
  const slice=deletions.slice(start,start+chunkSize*2),events:FleetEventV1[]=[];
  for(const path of slice){
   const current=replica.files.get(path)!,hlc=mint(replica);
   events.push({version:1,hlc,operation:"delete",path,base:current.operation==="put"?current.sha256!:null});
  }
  let head:FleetHeadV1;
  try{await ensureHome(replica);head=await appendEventsV1(storeRoot,originId,events,journalRepair(replica));}
  catch(error){
   if(!sharing(error))throw error;
   retry.push(...slice);noteOnce(replica,`blocked:journal:${code(error)}`,{event:"blocked",reason:code(error)});
   continue;
  }
  replica.cursors[originId]={segment:head.segment,bytes:head.bytes};
  for(const event of events)track(replica,event);
  for(const event of events){setFile(replica,event.path,{hlc:event.hlc,operation:"delete",mtimeMs:0});replica.manifest.set(event.path,{hlc:event.hlc,operation:"delete"});noted.push(event.path);}
  deleted+=events.length;
  for(const dir of new Set(events.map(event=>dirname(absolute(worktree,event.path)))))await prune(replica,dir);
 }
 replica.hot.noteMany(noted,noted.length,replica.settings.now());
 if(published>0||deleted>0)replica.activity++;
 if(published>0)replica.say({event:"published",count:published,bytes});
 if(deleted>0)replica.say({event:"published",stage:"delete",count:deleted});
 if(paths===undefined||paths.includes(".git/index")){
  await publishStage(replica,signal);
 }
 await persist(replica,false);
 return {scanned,published,deleted,touched,bytes,retry};
}

const BUSY=Symbol("busy");

async function retrying<T>(job:()=>Promise<T>):Promise<T|typeof BUSY>{
 for(let attempt=0;;attempt++){
  try{return await job();}
  catch(error){
   if(!sharing(error))throw error;
   if(attempt>=5)return BUSY;
   await sleep(20*2**attempt);
  }
 }
}

async function differs(target:string,here:Stats,known:StateEntry|undefined):Promise<boolean>{
 if(known===undefined||known.operation==="delete")return true;
 if(known.operation==="link")return !here.isSymbolicLink()||await readlink(target)!==known.target;
 if(!here.isFile()||here.size!==known.bytes)return true;
 if(here.mtimeMs===known.mtimeMs&&!racy(known))return false;
 return (await hashFileV1(target)).sha256!==known.sha256;
}

async function keepNewer(replica:ReplicaV1,path:string,entry:FleetManifestEntryV1,here:Stats):Promise<Outcome>{
 setFile(replica,path,entry.operation==="delete"?{hlc:entry.hlc,operation:"delete",mtimeMs:0}:{...plain(entry),mtimeMs:0});
 replica.say({event:"kept",path,hlc:entry.hlc,reason:"newer-than-remote",seconds:Math.max(0,Math.ceil((here.mtimeMs-parseHlcV1(entry.hlc).ms)/1000))});
 return "kept";
}

async function keepLost(replica:ReplicaV1,path:string,target:string,here:Stats,hlc:string):Promise<void>{
 const copy=join(replica.paths.lost,hlc,...path.split("/"));
 await mkdir(dirname(copy),{recursive:true});
 try{if(here.isSymbolicLink())await writeFile(copy,await readlink(target));else await copyFile(target,copy);}
 catch(error){if(gone(error))return;throw error;}
 replica.say({event:"overwritten",path,hlc,lost:`lost/${hlc}/${path}`});
}

async function prune(replica:ReplicaV1,directory:string):Promise<void>{
 const root=dirname(join(replica.settings.worktree,"x"));
 let current=directory;
 while(current.startsWith(root+sep)){
  try{await rmdir(current);}catch{return;}
  verifiedDirectories(replica).clear();
  current=dirname(current);
 }
}

function refusalOf(replica:ReplicaV1,path:string,entry:FleetManifestEntryV1):string|undefined{
 if(unsafePath(path))return "unsafe-path";
 const refused=refusePathV1(path);
 if(refused!==undefined)return refused;
 if(!sharedV1(path,false,replica.ignore))return "ignored";
 if(entry.operation==="link"){const unsafe=linkRefusalV1(path,entry.target!);if(unsafe!==undefined)return unsafe;}
 const other=replica.folds.find(path);
 if(other!==undefined&&other!==path&&entry.operation!=="delete")return "case-collision";
 return undefined;
}

type Outcome="applied"|"queued"|"dropped"|"kept";
const objectId=/^[0-9a-f]{40}(?:[0-9a-f]{24})?$/u;

/** 🔱 Hooks the linearisation of concurrent commits into the apply of a remote branch ref: only when the local ref value was written on this machine (published by this origin or not yet published) and is neither equal to nor an ancestor of the incoming commit, the local commits are re-created on top of the incoming one and published as an ordinary ref change; every other case stays last writer wins. `undefined` continues with the plain apply, `again` continues after the ref file changed. */
async function linearise(replica:ReplicaV1,path:string,entry:FleetManifestEntryV1,known:StateEntry|undefined,target:string,here:Stats):Promise<Outcome|"again"|undefined>{
 const {worktree,storeRoot,originId}=replica.settings;
 const local=(await readFile(target,"utf8").catch(()=>"")).trim(),incoming=(await readFile(blobPathV1(storeRoot,entry.sha256!),"utf8").catch(()=>"")).trim();
 if(!objectId.test(local)||!objectId.test(incoming)||local===incoming)return undefined;
 const mine=known===undefined||known.operation!=="put"||parseHlcV1(known.hlc).originId===originId||(await hashFileV1(target)).sha256!==known.sha256;
 if(!mine)return undefined;
 const result=await linearizeRefV1(worktree,path.slice(".git/".length),local,incoming);
 if(result.state==="retry"){
  noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"objects-missing"});
  if(result.waitedMs>300_000)noteOnce(replica,`blocked-long:${path}`,{event:"blocked",path,reason:"objects-missing-timeout",seconds:Math.floor(result.waitedMs/1000)});
  return "queued";
 }
 resolved(replica,`blocked:${path}`);
 if(result.state==="linear")return undefined;
 if(result.state==="ahead")return keepNewer(replica,path,entry,here);
 if(result.state==="duplicate"){replica.say({event:"linearised",path,hlc:entry.hlc,winner:result.winner,loser:result.loser,result:result.winner,reason:"duplicate"});return "again";}
 setFile(replica,path,{...plain(entry),mtimeMs:0});
 replica.say({event:"linearised",path,hlc:entry.hlc,winner:result.winner,loser:result.loser,result:result.result,count:result.conflicts.length});
 return "kept";
}

/** 🕳️ Tells whether a directory holds nothing but directories: empty directories are not replicated, so a tree of them never blocks a file or link at its path. */
async function onlyDirectories(directory:string):Promise<boolean>{
 try{
  for(const item of await readdir(directory,{withFileTypes:true})){
   if(!item.isDirectory()||!await onlyDirectories(join(directory,item.name)))return false;
  }
  return true;
 }catch{return false;}
}

async function materialise(replica:ReplicaV1,path:string,entry:FleetManifestEntryV1,signal?:AbortSignal):Promise<Outcome>{
 const {worktree,storeRoot}=replica.settings,target=absolute(worktree,path),known=replica.files.get(path),seenMs=replica.settings.now();
 const refusal=refusalOf(replica,path,entry);
 if(refusal!==undefined){noteOnce(replica,`refused:${path}:${refusal}`,{event:"refused",path,reason:refusal,hlc:entry.hlc});return refusal==="case-collision"?"queued":"dropped";}
 let here:Stats|undefined;
 const ancestry=await ancestryOf(worktree,path,verifiedDirectories(replica));
 if(ancestry==="symlink-in-the-way"&&entry.operation!=="delete"){noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:ancestry});return "queued";}
 if(ancestry==="ok"){try{here=await lstat(target);}catch(error){if(!gone(error))throw error;}}
 if(entry.operation==="put"&&here?.isFile()&&branchRefV1(path)){
  const forked=await linearise(replica,path,entry,known,target,here);
  if(forked==="again")here=await lstat(target).catch(()=>undefined);
  else if(forked!==undefined)return forked;
 }
 if(entry.operation==="delete"){
  const other=replica.folds.find(path);
  if(other!==undefined&&other!==path)here=undefined;
  if(here!==undefined&&!here.isDirectory()){
   const lose=await differs(target,here,known).catch(error=>{if(gone(error))return false;throw error;});
   if(lose&&conflictWinnerV1(here.mtimeMs,entry.hlc)==="local")return keepNewer(replica,path,entry,here);
   if(lose)await keepLost(replica,path,target,here,entry.hlc);
   if(await retrying(()=>unlink(target).catch(error=>{if(!gone(error))throw error;}))===BUSY){noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"sharing-violation"});return "queued";}
   await prune(replica,dirname(target));
  }
  setFile(replica,path,{hlc:entry.hlc,operation:"delete",mtimeMs:0});
  return "applied";
 }
 if(here?.isDirectory()){
  try{await rmdir(target);here=undefined;}
  catch{
   if(await onlyDirectories(target)){try{await rm(target,{recursive:true});here=undefined;}catch{}}
   if(here!==undefined){noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"directory-in-the-way"});return "queued";}
  }
 }
 let same=false,lose=false;
 if(here!==undefined){
  same=entry.operation==="put"?here.isFile()&&here.size===entry.bytes&&(await hashFileV1(target)).sha256===entry.sha256:here.isSymbolicLink()&&await readlink(target)===entry.target;
  if(!same)lose=await differs(target,here,known);
 }
 if(lose&&conflictWinnerV1(here!.mtimeMs,entry.hlc)==="local")return keepNewer(replica,path,entry,here!);
 const mark=entry.operation==="put"?stales.get(replica)?.get(replica.book.find(path)!):undefined;
 if(mark!==undefined&&!same&&!lose&&here?.isFile()===true&&known?.operation==="put"&&shortSha(known.sha256)===mark){
  stales.get(replica)!.delete(replica.book.find(path)!);
  replica.say({event:"stale-republish",path,hlc:entry.hlc,winner:known.sha256,loser:entry.sha256,reason:"content-superseded"});
  return keepNewer(replica,path,entry,here);
 }
 if(same){
  if(entry.operation==="put"&&!replica.settings.modeless&&((here!.mode&0o111)!==0)!==entry.executable)await chmod(target,entry.executable?0o755:0o644);
  const info=await lstat(target);
  setFile(replica,path,{...plain(entry),mtimeMs:info.mtimeMs,seenMs});
  return "applied";
 }
 if(entry.operation==="link"){
  try{
   if(lose)await keepLost(replica,path,target,here!,entry.hlc);
   for(let attempt=0;;attempt++){
    try{
     await mkdir(dirname(target),{recursive:true});
     if(here!==undefined)await rm(target,{force:true});
     await symlink(entry.target!,target);
     break;
    }catch(error){
     if(attempt<5&&(gone(error)||code(error)==="EEXIST"||code(error)==="EINVAL")){here=await lstat(target).catch(()=>undefined);await sleep(5*2**attempt);continue;}
     if(code(error)==="ENOTDIR"||code(error)==="EEXIST"){noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"parent-is-a-file"});return "queued";}
     throw error;
    }
   }
  }catch(error){
   if(code(error)==="EPERM"||code(error)==="EACCES"||code(error)==="ENOSYS"||code(error)==="UNKNOWN"){noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"symlink-unsupported"});return "dropped";}
   throw error;
  }
  setFile(replica,path,{...plain(entry),mtimeMs:(await lstat(target)).mtimeMs});
  return "applied";
 }
 const temporary=join(replica.paths.tmp,token());
 await mkdir(replica.paths.tmp,{recursive:true});
 let verdict:"ok"|"corrupt";
 try{
  verdict=await copyVerifiedV1(blobPathV1(storeRoot,entry.sha256!),temporary,entry.sha256!,{signal,mode:!replica.settings.modeless&&entry.executable?0o755:0o644,clone:true,progress:entry.bytes!>=largeBlobBytesV1?throttled((done,total)=>replica.say({event:"progress",stage:"apply-blob",path,done,total})):undefined});
 }catch(error){
  if(signal?.aborted)throw error;
  if(!gone(error))throw error;
  if(!await storeReachableV1(storeRoot))return "queued";
  noteOnce(replica,`blob-missing:${path}:${entry.sha256}`,{event:"blob-missing",path,hlc:entry.hlc,reason:entry.sha256});
  return "queued";
 }
 if(verdict==="corrupt"){noteOnce(replica,`corrupt:${path}:${entry.sha256}`,{event:"blob-missing",path,hlc:entry.hlc,reason:"blob-corrupt"});return "queued";}
 let stamp=0;
 try{
  stamp=(await lstat(temporary)).mtimeMs;
  if(lose)await keepLost(replica,path,target,here!,entry.hlc);
  for(let attempt=0;;attempt++){
   try{await mkdir(dirname(target),{recursive:true});}
   catch(error){
    if(attempt<3&&(code(error)==="ENOTDIR"||code(error)==="EEXIST"||code(error)==="EINVAL")){await sleep(5*2**attempt);continue;}
    if(code(error)==="ENOTDIR"||code(error)==="EEXIST"){await rm(temporary,{force:true});noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"parent-is-a-file"});return "queued";}
    throw error;
   }
   try{
    if(await retrying(()=>rename(temporary,target))===BUSY){await rm(temporary,{force:true});noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"sharing-violation"});return "queued";}
    break;
   }catch(error){
    if((gone(error)||code(error)==="EINVAL")&&attempt<3){await sleep(5*2**attempt);continue;}
    if(code(error)==="EISDIR"||code(error)==="ENOTEMPTY"){await rm(temporary,{force:true});noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:"directory-in-the-way"});return "queued";}
    throw error;
   }
  }
 }catch(error){await rm(temporary,{force:true});throw error;}
 setFile(replica,path,{...plain(entry),mtimeMs:stamp,seenMs:stamp%1000===0?seenMs:Math.max(seenMs,stamp+racyMilliseconds)});
 return "applied";
}

const retirements=new WeakMap<ReplicaV1,Set<number>>(),realDirectories=new WeakMap<ReplicaV1,Set<string>>();
const manifestLive=(replica:ReplicaV1,id:number):boolean=>replica.manifest.live(id)&&replica.manifest.operationAt(id)!=="delete";
const holds=(replica:ReplicaV1,id:number):boolean=>replica.files.live(id)&&replica.files.operationAt(id)!=="delete";
const verifiedDirectories=(replica:ReplicaV1):Set<string>=>{let held=realDirectories.get(replica);if(held===undefined){held=new Set();realDirectories.set(replica,held);}return held;};

/** 🪦️ The live manifest entries this machine has retired under the structural conflict rule: their local register is a delete with the very hlc of the live entry, which the persisted state already carries, so the set is rebuilt from the registers and entries that no longer agree are dropped. */
function retiredIds(replica:ReplicaV1):Set<number>{
 let held=retirements.get(replica);
 if(held===undefined){
  held=new Set();
  for(const id of replica.files.ids())if(replica.files.operationAt(id)==="delete"&&manifestLive(replica,id)&&replica.files.sameHlcAs(replica.manifest,id)){held.add(id);if(replica.queue.hasId(id))replica.queue.parkId(id);}
  retirements.set(replica,held);
 }
 for(const id of [...held])if(!manifestLive(replica,id)||replica.files.operationAt(id)!=="delete"||!replica.files.sameHlcAs(replica.manifest,id))held.delete(id);
 return held;
}

/** ⚖️ Retires a manifest entry that lost the structural conflict rule: the local copy, if there is one, goes to `.cache/f/lost/<winner hlc>/<path>` with an `overwritten` event, the register becomes a delete with the hlc of the retired entry (so the path is neither queued nor published), and a `superseded` event names the winner. */
async function retire(replica:ReplicaV1,id:number,winnerId:number):Promise<void>{
 const {book,manifest}=replica,path=book.name(id),winner=book.name(winnerId),target=absolute(replica.settings.worktree,path);
 if(holds(replica,id)){
  const here=await lstat(target).catch(()=>undefined);
  if(here!==undefined&&!here.isDirectory()){
   await keepLost(replica,path,target,here,manifest.hlcAt(winnerId));
   if(await retrying(()=>unlink(target))!==BUSY)await prune(replica,dirname(target));
  }
 }
 setFile(replica,path,{hlc:manifest.hlcAt(id),operation:"delete",mtimeMs:0});
 retiredIds(replica).add(id);replica.queue.parkId(id);replica.changed.add(queueKey(id));replica.dirty=true;replica.noted.delete(`blocked:${path}`);
 const key=clashKeyV1(path),other=clashKeyV1(winner);
 replica.say({event:"superseded",path,hlc:manifest.hlcAt(id),winner,reason:key===other?"case-collision":other.startsWith(`${key}/`)?"directory-replaces-file":"file-replaces-directory"});
}

/** 🕸️ Applies the structural conflict rule (`retiredV1`) to the queue and to this machine's tree: the rivals of an entry are the live manifest entries that clash with it, found through the queued and retired entries (indexed by fold key), the local registers (`folds`) and the ancestors in the manifest. A queued entry that loses is retired without being materialised; a queued winner retires the local copies of its older rivals first, so that the later writes never meet them; a retired entry whose winner is gone is queued again. Returns the ids that still have to be materialised. */
async function settleClashes(replica:ReplicaV1,items:Int32Array):Promise<Int32Array>{
 const {book,manifest,files,folds}=replica,retired=retiredIds(replica);
 if(items.length===0&&retired.size===0)return items;
 const keys=new Map<number,string>(),byKey=new Map<string,number[]>(),below=new Map<string,number[]>(),scans=new Map<string,number[]>(),rivals=new Map<number,number[]>(),beaten=new Map<number,number|undefined>(),turn=yieldingV1();
 const push=(map:Map<string,number[]>,key:string,id:number):void=>{const list=map.get(key);if(list===undefined)map.set(key,[id]);else list.push(id);};
 let ordered:{key:string;id:number}[]|undefined;
 const enrol=(id:number):void=>{
  const key=clashKeyV1(book.name(id));
  keys.set(id,key);push(byKey,key,id);
  for(let cut=key.lastIndexOf("/");cut>0;cut=key.lastIndexOf("/",cut-1))push(below,key.slice(0,cut),id);
 };
 for(const id of items){if(manifestLive(replica,id))enrol(id);await turn();}
 for(const id of retired){if(!keys.has(id))enrol(id);await turn();}
 if([...keys.values()].some(key=>(foldedDirectories(replica).get(key)??0)>0)){
  ordered=[];
  for(const other of files.ids()){if(files.operationAt(other)!=="delete")ordered.push({key:clashKeyV1(book.name(other)),id:other});await turn();}
  ordered.sort((left,right)=>left.key<right.key?-1:left.key>right.key?1:0);
 }
 const rivalsOf=(id:number):number[]=>{
  const known=rivals.get(id);
  if(known!==undefined)return known;
  const path=book.name(id),key=keys.get(id)??clashKeyV1(path),found=new Set<number>();
  const take=(other:number|undefined):void=>{if(other!==undefined&&other!==id&&manifestLive(replica,other)&&clashV1(path,book.name(other)))found.add(other);};
  for(const other of byKey.get(key)??[])take(other);
  const local=folds.find(path);
  if(local!==undefined)take(book.find(local));
  for(let cut=path.lastIndexOf("/");cut>0;cut=path.lastIndexOf("/",cut-1)){
   const prefix=path.slice(0,cut),near=folds.find(prefix);
   take(book.find(prefix));
   if(near!==undefined)take(book.find(near));
  }
  for(let cut=key.lastIndexOf("/");cut>0;cut=key.lastIndexOf("/",cut-1))for(const other of byKey.get(key.slice(0,cut))??[])take(other);
  for(const other of below.get(key)??[])take(other);
  if((foldedDirectories(replica).get(key)??0)>0){
   let inside=scans.get(key);
   if(inside===undefined){
    inside=[];
    if(ordered===undefined){
     ordered=[];
     for(const other of files.ids())if(files.operationAt(other)!=="delete")ordered.push({key:clashKeyV1(book.name(other)),id:other});
     ordered.sort((left,right)=>left.key<right.key?-1:left.key>right.key?1:0);
    }
    const prefix=`${key}/`;
    let low=0,high=ordered.length;
    while(low<high){const middle=(low+high)>>>1;if(ordered[middle]!.key<prefix)low=middle+1;else high=middle;}
    for(;low<ordered.length&&ordered[low]!.key.startsWith(prefix);low++)inside.push(ordered[low]!.id);
    scans.set(key,inside);
   }
   for(const other of inside)take(other);
  }
  const list=[...found];
  rivals.set(id,list);
  return list;
 };
 const winnerOf=(id:number):number|undefined=>{
  if(beaten.has(id))return beaten.get(id);
  const hlc=manifest.hlcAt(id);
  let winner:number|undefined;
  for(const other of rivalsOf(id))if(manifest.hlcAt(other)>hlc&&winnerOf(other)===undefined){winner=other;break;}
  beaten.set(id,winner);
  return winner;
 };
 const remaining:number[]=[],former=[...retired];
 for(const id of items){
  await turn();
  if(retired.has(id))continue;
  if(!manifestLive(replica,id)){remaining.push(id);continue;}
  const winner=winnerOf(id);
  if(winner!==undefined){await retire(replica,id,winner);continue;}
  remaining.push(id);
  for(const other of rivalsOf(id))if(holds(replica,other)&&!retired.has(other)&&manifest.hlcAt(other)<manifest.hlcAt(id))await retire(replica,other,id);
 }
 for(const id of former){
  await turn();
  if(!retired.has(id)||winnerOf(id)!==undefined)continue;
  retired.delete(id);files.clearId(id);replica.changed.add(filesKey(id));replica.dirty=true;
  enqueueId(replica,id);remaining.push(id);
 }
 return Int32Array.from(remaining);
}

type Applied=Readonly<{applied:number;dropped:number;kept:readonly string[]}>;

const cooling=new WeakMap<ReplicaV1,Map<string,Readonly<{until:number;attempts:number}>>>();

export async function applyQueue(replica:ReplicaV1,signal:AbortSignal|undefined,stage?:string):Promise<Applied>{
 replica.applying++;
 try{return await applyQueueWork(replica,signal,stage);}finally{replica.applying--;}
}

async function applyQueueWork(replica:ReplicaV1,signal:AbortSignal|undefined,stage?:string):Promise<Applied>{
 verifiedDirectories(replica).clear();
 let items=replica.queue.snapshot().filter(id=>!indexRealmV1(replica.book.name(id)));
 const kept:string[]=[],appliedPaths:string[]=[];
 try{items=await settleClashes(replica,items);}
 catch(error){if(signal?.aborted)throw error;noteOnce(replica,`error:clash:${messageOf(error)}`,{event:"error",stage:"clash",message:messageOf(error)});}
 const removals=items.filter(id=>replica.manifest.operationAt(id)==="delete"),creations=items.filter(id=>replica.manifest.operationAt(id)!=="delete");
 let applied=0,dropped=0,done=0;
 const work=async(id:number):Promise<void>=>{
  const path=replica.book.name(id),entry=replica.manifest.entryAt(id),cool=cooling.get(replica)??new Map<string,Readonly<{until:number;attempts:number}>>(),wait=cool.get(path);
  cooling.set(replica,cool);
  if(wait!==undefined&&replica.settings.now()<wait.until)return;
  let outcome:Outcome;
  try{outcome=await materialise(replica,path,entry,signal);}
  catch(error){
   if(signal?.aborted)throw error;
   if(sharing(error))noteOnce(replica,`blocked:${path}`,{event:"blocked",path,reason:code(error)});
   else if(code(error)==="ENAMETOOLONG"){noteOnce(replica,`skipped:${path}`,{event:"skipped",path,reason:"too-long"});dequeue(replica,path);dropped++;return;}
   else noteOnce(replica,`error:${path}:${messageOf(error)}`,{event:"error",path,message:messageOf(error)});
   outcome="queued";
  }
  if(outcome==="queued"&&replica.noted.has(`blocked:${path}`)){const attempts=(wait?.attempts??0)+1;cool.set(path,{attempts,until:replica.settings.now()+Math.min(30_000,250*2**attempts)});}
  else cool.delete(path);
  if(outcome==="applied"){applied++;appliedPaths.push(path);if(parseHlcV1(entry.hlc).ms>=replica.startedMs)replica.latency.apply(replica.settings.now()-parseHlcV1(entry.hlc).ms);resolved(replica,`blocked:${path}`);dequeue(replica,path);}
  else if(outcome==="dropped"){dropped++;dequeue(replica,path);}
  else if(outcome==="kept"){kept.push(path);dequeue(replica,path);}
  if(stage!==undefined&&++done%200===0)replica.say({event:"progress",stage,done,total:items.length});
 };
 await pool(removals,replica.settings.concurrency,work,signal);
 await pool(creations,replica.settings.concurrency,work,signal);
 if(stage!==undefined)replica.say({event:"progress",stage,done:items.length,total:items.length});
 if(applied>0){replica.activity++;replica.hot.noteMany(appliedPaths,applied,replica.settings.now());replica.say({event:"applied",count:applied});}
 try{await applyIndexV1(indexHost(replica),signal);}
 catch(error){if(signal?.aborted)throw error;noteOnce(replica,`error:git:${messageOf(error)}`,{event:"error",stage:"git",message:messageOf(error)});}
 return {applied,dropped,kept};
}

const histories=new WeakMap<ReplicaV1,Map<number,ContentEventV1[]>>(),stales=new WeakMap<ReplicaV1,Map<number,string>>(),savedHistory=new WeakMap<ReplicaV1,{dirty:boolean;at:number}>(),historyDepth=6,historyPaths=20_000,historySaved=1000,historySaveMilliseconds=30_000,historyMemo="stale.history";

const shortSha=(value:string|null|undefined):string|null=>value==null?null:value.slice(0,32);
const contentOf=(event:FleetEventV1):string|null=>event.operation==="put"?shortSha(event.sha256):event.operation==="link"?`link ${event.target}`:null;

/** 🕰️ Remembers one content event of a path for the stale rule: its hlc, the content it leaves and its `base`, the newest `historyDepth` events of each of the newest `historyPaths` paths. A path without history is seeded with the manifest entry it holds, so the events that follow have a predecessor. The memory is a cache of journal events and is kept with the local state (`stale.history`); a replica that lost it decides from the events it still reads. */
function track(replica:ReplicaV1,event:FleetEventV1):void{
 if(sealed(event.path)||event.path.startsWith(".git/"))return;
 let held=histories.get(replica);
 if(held===undefined){held=new Map();histories.set(replica,held);}
 const id=replica.book.intern(event.path),list=held.get(id)??[];
 if(list.length===0&&replica.manifest.live(id)){
  const prior=replica.manifest.operationAt(id)!,hlc=replica.manifest.hlcAt(id);
  if(prior!=="delete"&&hlc!==event.hlc)list.push({hlc,sha256:prior==="put"?shortSha(replica.manifest.shaAt(id)):`link ${replica.manifest.targets.get(id)}`,base:null});
 }
 if(list.some(item=>item.hlc===event.hlc))return;
 list.push({hlc:event.hlc,sha256:contentOf(event),base:shortSha(event.base)});
 list.sort((left,right)=>left.hlc<right.hlc?-1:left.hlc>right.hlc?1:0);
 if(list.length>historyDepth)list.splice(0,list.length-historyDepth);
 held.delete(id);held.set(id,list);
 if(held.size>historyPaths)held.delete(held.keys().next().value!);
 savedHistory.set(replica,{dirty:true,at:savedHistory.get(replica)?.at??0});
}

/** 🪢 Writes the stale-rule memory into the local state memo, at most every 30 s unless `force`: the newest 1000 paths. */
function keepHistory(replica:ReplicaV1,force:boolean):void {
 const saved=savedHistory.get(replica),held=histories.get(replica);
 if(saved===undefined||!saved.dirty||held===undefined)return;
 const now=replica.settings.now();
 if(!force&&now-saved.at<historySaveMilliseconds)return;
 const rows:[string,ContentEventV1[]][]=[];
 for(const [id,list] of [...held].slice(-historySaved))rows.push([replica.book.name(id),list]);
 replica.memo.set(historyMemo,encoder.encode(JSON.stringify({version:1,paths:rows})));replica.memoDirty.add(historyMemo);
 if(force)replica.dirty=true;
 savedHistory.set(replica,{dirty:false,at:now});
}

function restoreHistory(replica:ReplicaV1):void {
 const raw=replica.memo.get(historyMemo);
 if(raw===undefined)return;
 try{
  const value=JSON.parse(Buffer.from(raw).toString("utf8")) as {version:number;paths:[string,ContentEventV1[]][]};
  if(value.version!==1||!Array.isArray(value.paths))return;
  const held=new Map<number,ContentEventV1[]>();
  for(const [path,list] of value.paths)if(typeof path==="string"&&Array.isArray(list)&&path.isWellFormed())held.set(replica.book.intern(path),list.filter(item=>typeof item?.hlc==="string"&&(item.sha256===null||typeof item.sha256==="string")&&(item.base===null||typeof item.base==="string")).slice(-historyDepth));
  histories.set(replica,held);savedHistory.set(replica,{dirty:false,at:replica.settings.now()});
 }catch{}
}

function ingest(replica:ReplicaV1,origin:string,events:readonly FleetEventV1[]):void{
 const wall=replica.settings.now(),accepted:FleetEventV1[]=[];
 for(const event of events){
  try{replica.clock=mergeClockV1(replica.clock,event.hlc,wall);accepted.push(event);}
  catch{replica.say({event:"clock-refused",origin,hlc:event.hlc,path:event.path});}
 }
 for(const event of accepted)track(replica,event);
 for(const event of foldEventsV1(replica.manifest,accepted)){
  const id=replica.book.intern(event.path);
  stales.get(replica)?.delete(id);
  const stale=event.operation==="put"&&!event.path.startsWith(".git/")?staleRepublishV1(histories.get(replica)?.get(id)??[],event.hlc):undefined;
  if(stale?.winner!=null){
   let marks=stales.get(replica);
   if(marks===undefined){marks=new Map();stales.set(replica,marks);}
   marks.set(id,stale.winner);
  }
  if(!replica.files.live(id)||!replica.files.sameHlcAs(replica.manifest,id))enqueueId(replica,id);
 }
 replica.dirty=true;
}

async function adoptSnapshot(replica:ReplicaV1,origin:string|undefined,heads?:ReadonlyMap<string,FleetHeadV1|undefined>):Promise<boolean>{
 const {storeRoot,originId,compaction}=replica.settings,newest=(await listSnapshotsV1(storeRoot))[0];
 if(newest===undefined)return false;
 const useful=(cursors:Readonly<Record<string,FleetCursorV1>>):boolean=>origin!==undefined?cursors[origin]!==undefined&&cursorAheadV1(cursors[origin]!,replica.cursors[origin]):heads!==undefined&&snapshotSavesV1(replica.cursors,cursors,heads,originId,compaction.catchUpEvents);
 if(!useful(newest.cursors))return false;
 const staged=new CompactTableV1(replica.book,false),snapshot=await readNewestSnapshotV1(storeRoot,undefined,()=>{staged.clear();return staged;});
 if(snapshot===undefined||!useful(snapshot.cursors))return false;
 try{replica.clock=mergeClockV1(replica.clock,snapshot.hlc,replica.settings.now());}catch{replica.say({event:"clock-refused",origin:origin??"",hlc:snapshot.hlc});}
 for(const id of staged.ids()){
  const entry=staged.entryAt(id);
  if(!winsV1(entry.hlc,replica.manifest.live(id)?replica.manifest.hlcAt(id):undefined))continue;
  replica.manifest.write(id,entry);
  if(!replica.files.live(id)||!replica.files.sameHlcAs(replica.manifest,id))enqueueId(replica,id);
 }
 for(const [other,cursor] of Object.entries(snapshot.cursors))if(other!==originId&&cursorAheadV1(cursor,replica.cursors[other]))replica.cursors[other]=cursor;
 replica.dirty=true;
 replica.say({event:"progress",stage:origin===undefined?"jump":"rejoin",origin:origin??"",done:staged.size,total:staged.size});
 return true;
}

const jumpTried=new WeakMap<ReplicaV1,number>(),jumpRetryMilliseconds=30_000;

async function jumpIfFar(replica:ReplicaV1,heads:ReadonlyMap<string,FleetHeadV1|undefined>):Promise<void>{
 const {compaction,originId}=replica.settings,now=replica.settings.now();
 if(!compaction.automatic||now-(jumpTried.get(replica)??-Infinity)<jumpRetryMilliseconds||estimateEventsBehindV1(replica.cursors,heads,originId)<compaction.catchUpEvents)return;
 jumpTried.set(replica,now);
 try{await adoptSnapshot(replica,undefined,heads);}catch(error){noteOnce(replica,`jump:${messageOf(error)}`,{event:"error",stage:"jump",message:messageOf(error)});}
}

type Spans=Awaited<ReturnType<typeof readEventSpansV1>>;
type Admitted=Readonly<{events:readonly FleetEventV1[];cursor:FleetCursorV1|undefined;late:FleetEventV1|undefined}>;

function admit(read:Spans,after:FleetCursorV1|undefined,wall:number):Admitted{
 const stop=read.events.findIndex(event=>parseHlcV1(event.hlc).ms>wall+futureMilliseconds);
 if(stop<0)return {events:read.events,cursor:read.cursor,late:undefined};
 return {events:read.events.slice(0,stop),cursor:stop===0?after:read.ends[stop-1],late:read.events[stop]};
}

function skew(late:FleetEventV1,origin:string,wall:number):Omit<ReplicaEventV1,"version">{
 return {event:"clock-skew",origin,hlc:late.hlc,path:late.path,seconds:Math.ceil((parseHlcV1(late.hlc).ms-wall)/1000)};
}

const originLists=new WeakMap<ReplicaV1,Readonly<{at:number;list:readonly string[]}>>();

async function originsOf(replica:ReplicaV1):Promise<readonly string[]>{
 const known=originLists.get(replica),now=replica.settings.now();
 if(known!==undefined&&now-known.at<Math.max(250,replica.settings.pollMilliseconds*8))return known.list;
 const list=await listOriginsV1(replica.settings.storeRoot);
 originLists.set(replica,{at:now,list});
 return list;
}

const audited=new WeakMap<ReplicaV1,number>(),auditMilliseconds=10_000,followBudgetMilliseconds=500,sliceEvents=1024;
const behind=(cursor:FleetCursorV1|undefined,head:FleetHeadV1):boolean=>cursor===undefined||cursor.segment<head.segment||(cursor.segment===head.segment&&cursor.bytes<head.bytes);
const aheadOf=(left:FleetHeadV1,right:FleetHeadV1):boolean=>left.segment>right.segment||(left.segment===right.segment&&left.bytes>right.bytes);

async function followHeads(replica:ReplicaV1,origins:readonly string[],signal?:AbortSignal):Promise<Map<string,FleetHeadV1|undefined>>{
 const {storeRoot,concurrency}=replica.settings,flat=await readFlatHeadsV1(storeRoot).catch(()=>new Map<string,FleetHeadV1>()),now=Date.now(),audit=now-(audited.get(replica)??0)>=auditMilliseconds,heads=new Map<string,FleetHeadV1|undefined>(),polled:string[]=[];
 if(audit)audited.set(replica,now);
 for(const origin of origins){
  const hint=flat.get(origin);
  if(hint!==undefined&&!audit)heads.set(origin,hint);else polled.push(origin);
 }
 await pool(polled,concurrency,async origin=>{
  try{
   const head=await readHeadV1(storeRoot,origin),hint=flat.get(origin);
   heads.set(origin,head!==undefined&&hint!==undefined&&aheadOf(hint,head)?hint:head??hint);
  }catch(error){
   if(signal?.aborted)throw error;
   noteOnce(replica,`error:${origin}:${messageOf(error)}`,{event:"error",origin,message:messageOf(error)});
  }
 },signal);
 return heads;
}

async function followSlice(replica:ReplicaV1,signal?:AbortSignal,reuse:boolean=false):Promise<FollowResultV1&Readonly<{more:boolean}>>{
 const {storeRoot,originId}=replica.settings;
 await refreshIgnore(replica);
 const origins=(reuse?await originsOf(replica):await listOriginsV1(storeRoot)).filter(origin=>origin!==originId),heads=await followHeads(replica,origins,signal);
 await jumpIfFar(replica,heads);
 let events=0,more=false;
 for(const origin of origins){
  signal?.throwIfAborted();
  const head=heads.get(origin),cursor=replica.cursors[origin];
  if(head===undefined||cursor!==undefined&&cursor.segment===head.segment&&cursor.bytes===head.bytes)continue;
  const wall=replica.settings.now(),until=replica.held.get(origin);
  if(until!==undefined&&wall<until)continue;
  let read:Spans|undefined;
  for(let attempt=0;attempt<2&&read===undefined;attempt++){
   try{read=await readEventSpansV1(storeRoot,origin,replica.cursors[origin],1,head,sliceEvents);}
   catch(error){
    if(signal?.aborted)throw error;
    if(gone(error)){
     if(attempt===0&&await adoptSnapshot(replica,origin))continue;
     noteOnce(replica,`rejoin:${origin}`,{event:"rejoin-required",origin});
    }else noteOnce(replica,`${messageOf(error).includes("unsupported event version")?"unsupported":"error"}:${origin}:${messageOf(error)}`,{event:"error",origin,message:messageOf(error)});
    break;
   }
  }
  if(read===undefined)continue;
  resolved(replica,`rejoin:${origin}`);
  const taken=admit(read,replica.cursors[origin],wall);
  if(taken.late===undefined){replica.held.delete(origin);resolved(replica,`skew:${origin}`);}
  else{replica.held.set(origin,parseHlcV1(taken.late.hlc).ms-futureMilliseconds);noteOnce(replica,`skew:${origin}`,skew(taken.late,origin,wall));}
  ingest(replica,origin,taken.events);
  const advanced=taken.cursor!==undefined&&(cursor===undefined||taken.cursor.segment!==cursor.segment||taken.cursor.bytes!==cursor.bytes);
  if(taken.cursor!==undefined)replica.cursors[origin]=taken.cursor;
  events+=taken.events.length;
  if(taken.late===undefined&&advanced&&behind(replica.cursors[origin],head))more=true;
 }
 const {applied,dropped,kept}=await applyQueue(replica,signal);
 if(kept.length>0)await publishOnceV1(replica,signal,kept);
 await persist(replica,false);
 return {events,applied,dropped,kept:kept.length,queued:replica.queue.size,more};
}

/** 📥️ One follow round: learns which origins published from the flat heads (one `stat` of `v1/heads` when idle, see `readFlatHeadsV1`; origins without a flat head and, every 10 seconds, all origins are polled through `head.json`), reads the new events of every other origin from its cursor (byte offset) and at most one segment and 1024 events per origin and slice, merges the clock, folds, and materialises each winner through `.cache/f/tmp` and a verified rename; overwritten unpublished local edits are kept in `.cache/f/lost/<hlc>/<path>`; state is updated so the observer never echoes a change. An origin that is far behind is consumed slice by slice: the round continues with the next slice only while less than half a second has passed, so the caller regains control (to publish local changes, to attest) after a bounded time whatever the backlog. An origin whose next event is more than 60 s ahead of the wall clock is consumed only up to that event (the cursor stays in front of it, `clock-skew` is reported once per outage, the local clock is not merged with it) and is read again once the wall clock has caught up. */
export async function followOnceV1(replica:ReplicaV1,signal?:AbortSignal,reuse:boolean=false):Promise<FollowResultV1>{
 const started=Date.now();
 let events=0,applied=0,dropped=0,kept=0;
 for(;;){
  const slice=await followSlice(replica,signal,reuse);
  events+=slice.events;applied+=slice.applied;dropped+=slice.dropped;kept+=slice.kept;
  if(!slice.more||Date.now()-started>=followBudgetMilliseconds)break;
 }
 return {events,applied,dropped,kept,queued:replica.queue.size};
}

/** 🎲️ The wait until the next safety-net scan in milliseconds: `scanSeconds` spread uniformly over 75 % to 125 % (`random` in [0,1)), so that machines started together (a fleet-wide restart, a power cut) do not scan their trees in the same second for ever. */
export function scanDelayV1(scanSeconds:number,random:number=Math.random()):number{
 return Math.round(scanSeconds*1000*(0.75+0.5*random));
}

const hintMilliseconds=50,hotMilliseconds=300,sweepMilliseconds=2000,hotLimit=20_000,hotSliceMilliseconds=6,sweepSliceMilliseconds=10,hotDuty=8,sweepDuty=10,fileDuty=5,walkDuty=5,viewDuty=5,defaultWalkSeconds=1800,sliceGapMilliseconds=1;

const cpuCost=():(()=>number)=>{
 const cpu=process.cpuUsage();
 return ()=>{const used=process.cpuUsage(cpu);return (used.user+used.system)/1000;};
};

const meter=():(()=>number)=>{
 const wall=performance.now(),cpu=process.cpuUsage();
 return ()=>{const used=process.cpuUsage(cpu);return Math.min(performance.now()-wall,(used.user+used.system)/1000);};
};

const ancestors=(path:string):string[]=>{const found:string[]=[];for(let cut=path.lastIndexOf("/");cut>0;cut=path.lastIndexOf("/",cut-1))found.push(path.slice(0,cut));return found;};

function hotChanged(replica:ReplicaV1,path:string,now:number,target:string=absolute(replica.settings.worktree,path)):boolean{
 const {files,queue}=replica,id=replica.book.find(path);
 let info:Stats;
 try{info=lstatSync(target);}
 catch(error){return gone(error)&&id!==undefined&&files.live(id)&&files.operationAt(id)!=="delete"&&!queue.hasId(id);}
 if(info.isDirectory())return false;
 if(id===undefined||!files.live(id)||files.operationAt(id)==="delete")return (info.isFile()||info.isSymbolicLink())&&!queue.has(path)&&sharedV1(path,false,replica.ignore);
 if(queue.hasId(id))return false;
 if(info.isSymbolicLink()){
  try{return files.operationAt(id)!=="link"||files.targets.get(id)!==readlinkSync(target);}catch{return true;}
 }
 if(!info.isFile())return false;
 return !(files.operationAt(id)==="put"&&files.bytes[id]===info.size&&files.mtime[id]===info.mtimeMs&&(!files.racyAt(id)||now-info.mtimeMs<racyMilliseconds)&&(replica.settings.modeless||files.executableAt(id)===((info.mode&0o111)!==0)));
}

function directoryMoved(replica:ReplicaV1,directory:string):boolean{
 const recorded=replica.dirs.get(directory);
 if(recorded===undefined)return false;
 try{return lstatSync(directory===""?replica.settings.worktree:absolute(replica.settings.worktree,directory)).mtimeMs!==recorded;}catch(error){return gone(error);}
}

const viewPattern=/^[0-9a-f]{64}$/u;
function seedView(replica:ReplicaV1):Readonly<{staged:number;modified:number;deleted:number;untracked:number;digest:string}>|undefined{
 const raw=recallV1(replica,"view");
 if(raw===undefined)return undefined;
 try{
  const value=JSON.parse(Buffer.from(raw).toString("utf8")) as Record<string,unknown>;
  const counts=["staged","modified","deleted","untracked"].map(key=>value[key]);
  if(counts.every(count=>Number.isSafeInteger(count)&&(count as number)>=0)&&typeof value.digest==="string"&&viewPattern.test(value.digest))return {staged:counts[0] as number,modified:counts[1] as number,deleted:counts[2] as number,untracked:counts[3] as number,digest:value.digest};
 }catch{}
 return undefined;
}

const defaultLogFiles=(worktree:string):readonly string[]=>[...process.platform==="darwin"?["/var/log/semio-fleet-replica.log"]:[],join(worktree,".cache","f","native-bootstrap.log")];

async function maintain(replica:ReplicaV1,options:ReplicaOptionsV1):Promise<void>{
 const {settings,paths}=replica,now=Date.now();
 await pruneLostV1(paths.lost,{now,retainMs:settings.retentionDays*86_400_000,maxEntries:lostEntries}).catch(()=>0);
 for(const file of options.logFiles??defaultLogFiles(settings.worktree))await rotateLogV1(file,{maxBytes:logLimitFileBytes,keepBytes:logKeepBytes}).catch(()=>false);
 replica.failures.forgetOlderThan(3_600_000);
 const cool=cooling.get(replica);
 if(cool!==undefined)for(const path of cool.keys()){const id=replica.book.find(path);if(id===undefined||!replica.queue.hasId(id))cool.delete(path);}
}

/** 🏃️ The long-running daemon. Changes are found by four tiers that never depend on operating-system file events: a drop directory of hints (`.cache/f/hints/`, polled every 50 ms, published at once), the hot set (every path published, applied, hinted or seen by the watcher in the last ten minutes, re-`lstat`ed about every 300 ms together with the directories that contain them and the stat signature of `.git/index`), the directory sweep (a duty-limited `lstat` of every known shared directory, about 12 % of one core at most and never more often than every 2 s, which re-lists only directories whose modification time moved and so finds creates, deletes and atomic-rename saves anywhere within seconds) and the full file sweep every `scanSeconds` (jittered, in the background; in-place edits of cold files), which also runs at start, after an outage, after `.gitignore` changes and after a hint or watcher overflow. The recursive native watch (when the platform has it and `watch` is not `false`) with a 150 ms debounce stays an accelerator. Every tier hands paths to one publish step per round, so a path found twice is published once, and each tier works in slices of a few milliseconds so follow and apply never wait for it. Follow runs every `pollMilliseconds`; the sync switch, the retirement record, repair requests, store reachability and the origin list are read at slower cadences; an unreachable store is reported once per outage while local work continues; every 5 s (and at most once a second when a value changed) it writes its attestation into the store, also while sync is off, and when the code on disk has differed from the code at start for 5 s it persists, reports `restarting` and returns so that the service manager starts the new code; while the own origin carries a retirement record with `retired: true` it reports one `retired` event, publishes and applies nothing and keeps attesting with sync off, until `retire --undo` re-admits it; `.gitignore` changes reload the matcher; `.cache/f/sync.json` with `enabled: false` holds publish and apply until the file is removed; cancelling the signal persists state and releases the lock.  Every store turn runs under a deadline guard: a store call that a hard mount blocks cannot be cancelled, so when the store has not answered a probe within `storeDeadlineMilliseconds` (10 s) the round is abandoned and aborted, `store-unreachable` is reported (reason `stalled`), no further store call is issued while one is outstanding, the loop, the hint channel, the attestation to memory and the state persistence keep running, and the daemon resumes by itself when the store answers again. New code is adopted only after a fresh process proved that it starts (`restart-refused` otherwise), after a deterministic per-origin delay of 0 to 60 s and while the incoming queue is empty. Recurring failures are counted per kind, announced at most once a minute and shown in `status`, the health file and the attestation. */
export async function runV1(options:ReplicaOptionsV1,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 await awaitWorktreeAccessV1({worktree:options.worktree,storeRoot:options.storeRoot,originId:options.originId},signal,(state,error)=>{sink({version:1,event:"blocked",reason:"worktree-access-denied",path:options.worktree,message:`${state} ${error}`});});
 const replica=await openReplicaV1(options,sink),settings=replica.settings,touched=new Set<string>(),hinted=new Set<string>();
 const hintEvery=options.hintMilliseconds??hintMilliseconds,hotEvery=options.hotMilliseconds??hotMilliseconds,sweepEvery=options.sweepMilliseconds??sweepMilliseconds,slow=Math.max(settings.pollMilliseconds,Math.min(1000,settings.pollMilliseconds*4));
 let rounds=0,walk:Promise<void>|undefined,walked:WalkV1|undefined,wake:(()=>void)|undefined,lastTouch=0,full=false,nextWalk=0,outage=false,recovering=true,watcher:ReturnType<typeof watch>|undefined,suspended=false,ignoreSeen=replica.ignoreHash,indexSeen:string|undefined,hinting=false,settledAt=-1,failed=0,healthAt=-Infinity,maintainedAt=-Infinity;
 const hot={list:[] as string[],dirs:[] as string[],cursor:0,active:false,spent:0,next:0},sweep={active:false,spent:0,next:0},file={active:false,ids:new Int32Array(0),index:0,spent:0,next:0},walkEvery=options.walkSeconds??defaultWalkSeconds,walkNext=(cost:number):number=>{const delay=nextDelayV1(scanDelayV1(walkEvery),cost,walkDuty);replica.latency.tiers.record("walk",cost,delay);return settings.now()+delay;};
 if(replica.dirs.size>0)nextWalk=settings.now()+scanDelayV1(walkEvery);
 const pause=(milliseconds:number)=>new Promise<void>(done=>{
  const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);wake=undefined;done();};
  const timer=setTimeout(finish,milliseconds);
  signal.addEventListener("abort",finish,{once:true});wake=finish;
 });
 const cached=<T>(milliseconds:number,read:()=>Promise<T>)=>{
  let at=-Infinity,value:T;
  return {get:async(fresh:boolean=false):Promise<T>=>{if(fresh||settings.now()-at>=milliseconds){value=await read();at=settings.now();}return value;},expire:():void=>{at=-Infinity;}};
 };
 const guard=createStoreGuardV1({probe:()=>(options.storeProbe??storeReachableV1)(settings.storeRoot),deadlineMs:options.storeDeadlineMilliseconds??10_000,now:Date.now,changed:()=>{}});
 const baseDrift=options.driftMilliseconds??120_000,drift={at:settings.now()+baseDrift,wait:baseDrift,attempts:0};
 const drifted=async(signal:AbortSignal):Promise<readonly string[]>=>{
  const now=settings.now();
  if(now<drift.at)return [];
  const fields=repairableDriftV1(await bounded(()=>fleetReportV1(settings.storeRoot,now,signal)),settings.originId);
  if(fields.length>0&&drift.attempts>=3){drift.at=now+3_600_000;return [];}
  drift.attempts=fields.length===0?0:drift.attempts+1;
  drift.wait=fields.length===0?baseDrift:Math.min(baseDrift*2**drift.attempts,3_600_000);
  drift.at=now+drift.wait;
  return fields;
 };
 const switchOn=cached(slow,()=>syncEnabledV1(settings.worktree)),reachable=cached(slow,()=>guard.reachable()),retirement=cached(slow*2,()=>readRetirementV1(settings.storeRoot,settings.originId)),repairAsked=cached(slow,()=>repairRequestedV1(settings.worktree));
 const down=()=>{if(!outage){outage=true;replica.say({event:"store-unreachable",path:settings.storeRoot,reason:guard.state()});}};
 let syncOn=true,restart=false,parked=false;
 const history={current:await readRunHistoryV1(settings.worktree)};
 const running=options.codeRoot??codeRootV1,watching=options.watchRoot??running;
 const checkCode=async(digest:string,aborted?:AbortSignal):Promise<RestartCheckV1>=>{
  const result=await runSelfCheckV1({command:process.execPath,args:[join(watching,"🔁️replica","📜️script.ts"),"self-check"],cwd:settings.worktree,timeoutMs:options.selfCheckMilliseconds??180_000,signal:aborted});
  if(!result.ok)return {ok:false,reason:result.reason};
  if(result.digest!==digest)return {ok:false,reason:"code changed during the check",again:true};
  const sealed=await sealCodeV1({root:settings.worktree,moduleRoot:watching,digest,digestOf:codeDigestV1,signal:aborted});
  if(sealed.reason==="code-changed-while-copying")return {ok:false,reason:sealed.reason,again:true};
  if(sealed.reason!==null)noteOnce(replica,`seal:${sealed.reason}`,{event:"error",stage:"seal",message:sealed.reason});
  return {ok:true};
 };
 const bounded=async<T>(work:()=>Promise<T>):Promise<T>=>{
  if(guard.state()!=="ok")throw new StoreStalledV1();
  let timer:ReturnType<typeof setTimeout>|undefined;
  try{return await Promise.race([work(),new Promise<never>((_,fail)=>{timer=setTimeout(()=>{fail(new StoreStalledV1());},options.storeDeadlineMilliseconds??10_000);timer.unref?.();})]);}
  finally{clearTimeout(timer);}
 };
 const attester=createAttesterV1({storeRoot:settings.storeRoot,originId:settings.originId,groupId:settings.groupId,now:settings.now,heartbeatMilliseconds:options.attestMilliseconds,stableMilliseconds:options.restartStableMilliseconds,codeRoot:options.codeRoot,watchRoot:options.watchRoot,check:options.selfCheck??checkCode,admit:options.admit??(options.selfCheck===undefined?(digest,aborted)=>admitCodeV1(replica,{digest,running:attester.running(),moduleRoot:watching,bounded,signal:aborted}):undefined),fallback:options.fallback,returnMilliseconds:options.returnMilliseconds,busy:()=>replica.queue.size>0||replica.applying>0,jitterSpanMilliseconds:options.restartJitterMilliseconds,minimumUptimeMilliseconds:options.minimumUptimeMilliseconds,refused:(digest,reason)=>{replica.say({event:"restart-refused",reason:digest,message:reason});void recordRefusedV1(settings.worktree,digest).then(next=>{history.current=next;}).catch(()=>{});},suspect:(digest,runningDigest)=>suspectCodeV1(history.current,runningDigest,digest),suspicious:(digest,reason)=>{replica.say({event:"code-reverted",reason:digest,message:reason});},ran:digest=>{void recordRunV1(settings.worktree,digest).then(next=>{history.current=next;}).catch(()=>{});},failures:()=>replica.failures.summary(),store:work=>guard.call(work),git:signal=>(options.gitState??gitStateV1)(settings.worktree,signal),view:signal=>(options.gitView??gitViewV1)(settings.worktree,signal),viewMilliseconds:options.viewMilliseconds,viewDebounceMilliseconds:options.viewDebounceMilliseconds,guards:async signal=>{if(options.gitGuards!==undefined)return options.gitGuards(settings.worktree,signal);const found=await gitGuardsV1(settings.worktree,{signal,modeless:settings.modeless});if(found===null)return null;const {samples:_,...guards}=found;return guards;},guardsMilliseconds:options.guardsMilliseconds,guardsMinimumMilliseconds:options.guardsMinimumMilliseconds,reachable:()=>reachable.get(),prepare:()=>ensureHome(replica),latency:()=>replica.latency.report(),timed:(name,cost,delay)=>{replica.latency.tiers.record(name,cost,delay);},viewSeed:()=>seedView(replica),onView:view=>{rememberV1(replica,"view",encoder.encode(JSON.stringify(view)));}});
 options.opened?.(replica);
 replica.say({event:"started",originId:settings.originId,groupId:settings.groupId});
 const booted=attester.start(signal).catch(()=>{});
 void booted.then(()=>{if(!relative(settings.worktree,running).startsWith("..")&&!relative(settings.worktree,running).startsWith(".cache")&&attester.running()!=="")void sealCodeV1({root:settings.worktree,moduleRoot:running,digest:attester.running(),digestOf:codeDigestV1,signal}).catch(()=>{}).then(()=>installBootStageV1(settings.worktree,join(running,"🔁️replica","📜️script.ts"))).catch(()=>{});});
 const bootWall=Date.now(),publishAfter=options.publishCodeMilliseconds??600_000;
 let shared=false;
 const shareCode=async():Promise<void>=>{
  const digest=attester.running();
  await bounded(()=>publishCodeSnapshotV1({root:settings.worktree,store:settings.storeRoot,digest,originId:settings.originId}));
  const live=new Set(Object.values(await bounded(()=>readAttestationsV1(settings.storeRoot))).map(item=>item.code));
  live.add(digest);
  await bounded(()=>pruneStoreCodeV1(settings.storeRoot,options.codeKeep??4,live));
 };
 let configured=false;
 await ensureHintsV1(settings.worktree).catch(()=>{});
 const ticker=setInterval(()=>{
  if(hinting||signal.aborted)return;
  hinting=true;
  void takeHintsV1(settings.worktree,undefined,settings.now()).then(paths=>{
   if(paths.length===0)return;
   for(const path of paths){if(hinted.size<hotLimit)hinted.add(path);else full=true;}
   wake?.();
  }).finally(()=>{hinting=false;});
 },hintEvery);
 const watchFailed=(error:unknown):void=>{watcher?.close();watcher=undefined;noteOnce(replica,"watch:disabled",{event:"error",stage:"watch",message:`file watching stopped, only the scans remain: ${messageOf(error)}`});};
 try{
  if(options.watch!==false)watcher=watch(settings.worktree,{recursive:true,persistent:false},(kind,name)=>{
   if(name===null||name===undefined)return;
   const path=String(name).split(sep).join("/").normalize("NFC");
   if(path===".cache/f"||path.startsWith(".cache/f/")||(gitPrivateV1(path)&&path!==".git/index")||(path.split("/").includes(".git")&&!path.startsWith(".git/")&&path!==".git")||(!(path===".git"||path.startsWith(".git/"))&&replica.ignore.ignores(path,false)))return;
   if(path===".gitignore"||touched.size>5000){full=true;touched.clear();}
   else{touched.add(path);if(kind==="rename")touched.add(`${path}/`);replica.hot.note(path,settings.now());}
   replica.activity++;lastTouch=settings.now();wake?.();
  });
  watcher?.on("error",watchFailed);
 }catch(error){watchFailed(error);}
 try{
  while(!signal.aborted){
   try{
    const enabled=await switchOn.get();syncOn=enabled;
    if(!enabled){if(!suspended){suspended=true;replica.say({event:"sync",reason:"off"});}}
    else{
    if(suspended){suspended=false;full=true;replica.say({event:"sync",reason:"on"});}
    if(!await reachable.get(outage))down();
    else{
     if(outage){outage=false;replica.say({event:"store-reachable"});full=true;nextWalk=0;}
     const turn=await guard.turn(async signal=>{
     await options.storeGate?.();
     const stopped=await retirement.get();
     if(stopped?.retired===true){if(!parked){parked=true;replica.say({event:"retired",originId:settings.originId,origin:stopped.by,reason:"retired",message:stopped.reason});}}
     else{
     if(parked){parked=false;full=true;replica.say({event:"sync",reason:"on"});}
     const retry:string[]=[];
     const asked=await repairAsked.get(),apart=asked?[]:await drifted(signal);
     if(asked||apart.length>0){
      if(apart.length>0)replica.say({event:"drift",reason:apart.join(","),count:drift.attempts});
      replica.say({event:"repaired",count:await repairV1(replica,signal),...apart.length>0?{reason:"drift"}:{}});repairAsked.expire();full=true;
     }
     if(recovering){recovering=false;await followOnceV1(replica,signal,true);}
     if(replica.ignoreHash!==ignoreSeen){ignoreSeen=replica.ignoreHash;full=true;}
     const now=settings.now();
     if(full&&now>=nextWalk){full=false;walked=undefined;touched.clear();hinted.clear();const spent=cpuCost();retry.push(...(await publishOnceV1(replica,signal)).retry);nextWalk=walkNext(spent());file.next=settings.now()+scanDelayV1(settings.scanSeconds);}
     else{
      if(walked!==undefined){const scan=walked;walked=undefined;retry.push(...(await publishOnceV1(replica,signal,undefined,scan)).retry);}
      else if(walk===undefined&&now>=nextWalk){
       nextWalk=Infinity;
       const digest=replica.ignoreHash,spent=cpuCost();
       walk=walkLocalV1(replica,signal).then(scan=>{walk=undefined;nextWalk=walkNext(spent());if(replica.ignoreHash===digest)walked=scan;else full=true;settleHeap();},error=>{walk=undefined;nextWalk=walkNext(spent());if(!signal.aborted)noteOnce(replica,`scan:${messageOf(error)}`,{event:"error",message:messageOf(error)});});
      }
      const batch=new Set<string>(),flat=new Set<string>();
      if(touched.size>0&&settings.now()-lastTouch>=settings.debounceMilliseconds){for(const path of touched)batch.add(path);touched.clear();}
      if(hinted.size>0){replica.hot.noteMany(hinted,hinted.size,now);replica.activity++;for(const path of hinted)batch.add(path);hinted.clear();}
      if(!hot.active&&now>=hot.next){hot.list=replica.hot.snapshot(now);hot.dirs=[...new Set([...hot.list.map(parentOf),...replica.warm.snapshot(now),...replica.dirs.shallow()])].filter(directory=>replica.dirs.has(directory));hot.cursor=0;hot.spent=0;hot.active=true;}
      if(hot.active){
       const started=performance.now(),stop=meter(),total=hot.list.length+hot.dirs.length;
       while(hot.cursor<total){
        const index=hot.cursor++;
        if(index<hot.list.length){const path=hot.list[index]!;if(hotChanged(replica,path,now))batch.add(path);}
        else{const directory=hot.dirs[index-hot.list.length]!;if(directoryMoved(replica,directory))flat.add(directory);}
        if((hot.cursor&63)===0&&performance.now()-started>=hotSliceMilliseconds)break;
       }
       hot.spent+=stop();
       if(hot.cursor>=total){
        hot.active=false;
        const signature=await indexSignatureV1(settings.worktree);
        if(indexSeen!==undefined&&signature!==indexSeen)batch.add(".git/index");
        indexSeen=signature;
        const delay=nextDelayV1(hotEvery,hot.spent,hotDuty);
        hot.next=settings.now()+delay;replica.latency.tiers.record("hot",hot.spent,delay);
       }
      }
      if(!sweep.active&&now>=sweep.next){replica.dirs.begin();sweep.spent=0;sweep.active=true;}
      if(sweep.active){
       const started=performance.now(),stop=meter(),vanished:string[]=[],prefix=`${settings.worktree}${sep}`;
       let count=0;
       for(;;){
        const id=replica.dirs.next();
        if(id<0){sweep.active=false;break;}
        const name=replica.dirs.nameAt(id);
        try{
         const info=lstatSync(name===""?settings.worktree:prefix+name,{throwIfNoEntry:false});
         if(info===undefined){replica.dirs.delete(name);vanished.push(name);}
         else if(info.mtimeMs!==replica.dirs.mtimeAt(id))flat.add(name);
        }catch(error){if(gone(error)){replica.dirs.delete(name);vanished.push(name);}}
        if((++count&63)===0&&performance.now()-started>=sweepSliceMilliseconds)break;
       }
       sweep.spent+=stop();
       if(!sweep.active){const delay=nextDelayV1(sweepEvery,sweep.spent,sweepDuty);sweep.next=settings.now()+delay;replica.latency.tiers.record("sweep",sweep.spent,delay);}
       const lost=new Set(vanished);
       for(const name of vanished)if(!ancestors(name).some(parent=>lost.has(parent)))batch.add(name);
      }
      const cold:string[]=[];
      if(!file.active&&now>=file.next){file.ids=replica.files.ids();file.index=0;file.spent=0;file.active=true;}
      if(file.active){
       const started=performance.now(),stop=meter(),prefix=`${settings.worktree}${sep}`;
       while(file.index<file.ids.length){
        const id=file.ids[file.index++]!;
        if(replica.files.operationAt(id)==="delete")continue;
        const name=replica.book.name(id);
        if(!indexRealmV1(name)&&hotChanged(replica,name,now,prefix+name))cold.push(name);
        if((file.index&63)===0&&performance.now()-started>=sweepSliceMilliseconds)break;
       }
       file.spent+=stop();
       if(file.index>=file.ids.length){file.active=false;file.ids=new Int32Array(0);const delay=nextDelayV1(scanDelayV1(settings.scanSeconds),file.spent,fileDuty);file.next=settings.now()+delay;replica.latency.tiers.record("file",file.spent,delay);}
      }
      if(cold.length>0)retry.push(...(await publishOnceV1(replica,signal,cold,undefined,undefined,true)).retry);
      if(batch.size>0||flat.size>0){
       const list=[...batch],directories=[...flat];
       if(directories.length>0){replica.activity++;replica.warm.noteMany(directories,directories.length,now);}
       try{retry.push(...(await publishOnceV1(replica,signal,list,undefined,directories)).retry);}catch(error){for(const path of list)touched.add(path);lastTouch=settings.now();throw error;}
      }
     }
     if(retry.length>0){for(const path of retry)touched.add(path);lastTouch=settings.now();}
     await followOnceV1(replica,signal,true);
     }
     },signal);
     if(turn.status==="stalled")down();
    }
    }
    failed=0;
   }catch(error){
    if(signal.aborted)break;
    if(error instanceof OriginTakenV1)throw error;
    reachable.expire();
    if(!await reachable.get())down();
    else{failed++;noteOnce(replica,`round:${messageOf(error)}`,{event:"error",message:messageOf(error)});}
   }
   if(!signal.aborted){
    try{await booted;if(await attester.tick({sync:syncOn&&!parked,files:replica.files.shared,manifest:replica.files.manifestHex(),cursors:replica.cursors,incoming:replica.queue.size,hlc:formatHlcV1(replica.clock,settings.originId),activity:replica.activity},signal)==="restart"){restart=true;break;}}
    catch(error){if(!signal.aborted)noteOnce(replica,`attest:${messageOf(error)}`,{event:"error",message:messageOf(error)});}
    const wall=Date.now();
    if(wall-healthAt>=(options.attestMilliseconds??healthMilliseconds)){healthAt=wall;await writeHealthV1(settings.worktree,{version:1,wallMs:wall,pid:process.pid,code:attester.running()===""?"0".repeat(64):attester.running(),store:guard.state(),storeSinceMs:guard.since(),pending:attester.pending(),failures:[...replica.failures.summary()]}).catch(()=>{});}
    if(wall-maintainedAt>=maintenanceMilliseconds){maintainedAt=wall;await maintain(replica,options).catch(()=>{});}
    if(!shared&&wall-bootWall>=publishAfter&&guard.state()==="ok"&&attester.running()!==""&&!parked&&replica.failures.summary().every(item=>wall-item.lastMs>=publishAfter)){shared=true;void shareCode().catch(()=>{shared=false;});}
    void autoCompactV1(replica,signal);
   }
   if(!configured&&!signal.aborted){configured=true;void normaliseGitConfigV1(settings.worktree,undefined,signal).catch(()=>[]);}
   if(++rounds%heapRounds===0&&replica.activity!==settledAt){settledAt=replica.activity;settleHeap();}
   await pause(Math.max(hot.active||sweep.active||file.active?sliceGapMilliseconds:touched.size>0?Math.min(settings.pollMilliseconds,settings.debounceMilliseconds):settings.pollMilliseconds,failed===0?0:Math.min(30_000,250*2**Math.min(failed,7))));
  }
 }finally{
  clearInterval(ticker);
  watcher?.close();
  await closeReplicaV1(replica);
  if(restart)replica.say({event:"restarting"});
  replica.say({event:"stopped"});
 }
}

/** 🌱️ Seeds the store from the first machine: creates the store directories and publishes the whole shared tree with batched events, reporting done/total. */
export async function seedV1(options:ReplicaOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<PublishResultV1>{
 const store=storePathsV1(options.storeRoot);
 for(const directory of [store.root,store.origins,store.blobs,store.tmp,store.snapshots])await mkdir(directory,{recursive:true});
 const replica=await openReplicaV1(options,sink);
 try{return await publishOnceV1(replica,signal);}finally{await closeReplicaV1(replica);}
}

export async function foldInto(storeRoot:string,manifest:CompactTableV1,signal:AbortSignal|undefined,sink:ReplicaSinkV1,now:()=>number):Promise<Readonly<{cursors:Record<string,FleetCursorV1>;hlc:string}>>{
 const snapshot=await readNewestSnapshotV1(storeRoot,undefined,()=>{manifest.clear();return manifest;}),cursors:Record<string,FleetCursorV1>={...snapshot?.cursors},origins=await listOriginsV1(storeRoot);
 let hlc=snapshot?.hlc??"";
 for(const [index,origin] of origins.entries()){
  signal?.throwIfAborted();
  if(await readHeadV1(storeRoot,origin)===undefined)continue;
  let recovered=false;
  for(;;){
   let read:Spans;
   try{read=await readEventSpansV1(storeRoot,origin,cursors[origin],1);}
   catch(error){
    if(!gone(error))throw error;
    const staged=new CompactTableV1(manifest.book,false),own=recovered?undefined:await readNewestSnapshotV1(storeRoot,undefined,()=>{staged.clear();return staged;}),boundary=own?.cursors[origin],at=cursors[origin];
    recovered=true;
    if(own!==undefined&&boundary!==undefined&&(at===undefined||at.segment<boundary.segment||at.segment===boundary.segment&&at.bytes<boundary.bytes)){
     for(const id of staged.ids()){
      const entry=staged.entryAt(id);
      if(winsV1(entry.hlc,manifest.live(id)?manifest.hlcAt(id):undefined))manifest.write(id,entry);
     }
     for(const [other,cursor] of Object.entries(own.cursors))if(cursorAheadV1(cursor,cursors[other]))cursors[other]=cursor;
     if(own.hlc>hlc)hlc=own.hlc;
     continue;
    }
    sink({version:1,event:"rejoin-required",origin});
    break;
   }
   const wall=now(),taken=admit(read,cursors[origin],wall),before=cursors[origin];
   if(taken.late!==undefined)sink({version:1,...skew(taken.late,origin,wall)});
   foldEventsV1(manifest,taken.events);
   for(const event of taken.events)if(event.hlc>hlc)hlc=event.hlc;
   if(taken.cursor!==undefined)cursors[origin]=taken.cursor;
   const after=cursors[origin];
   if(taken.late!==undefined||after===undefined||before!==undefined&&before.segment===after.segment&&before.bytes===after.bytes)break;
  }
  sink({version:1,event:"progress",stage:"fold",done:index+1,total:origins.length});
 }
 return {cursors,hlc};
}

/** 🧮️ Folds the store to one manifest: the newest snapshot (streamed), then every event after its cursors, read one segment at a time; an origin is folded only up to its first event more than 60 s ahead of the wall clock (reported as `clock-skew`), and its cursor stays in front of that event, so snapshots and compaction never pass it. The manifest is a compact table that behaves like a `Map`. */
export async function foldStoreV1(storeRoot:string,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},now:()=>number=Date.now):Promise<StoreFoldV1>{
 const manifest=new CompactTableV1(new ReplicaBookV1(),false),{cursors,hlc}=await foldInto(storeRoot,manifest,signal,sink,now);
 return {manifest,cursors,hlc};
}

/** 🧳️ Joins a machine: folds the newest snapshot plus the journal, refuses a non-empty worktree whose shared files do not already match by hash, adopts matching files, then materialises the rest while reporting done/total; a half-finished join resumes from the persisted queue. */
export async function joinV1(options:ReplicaOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<JoinResultV1>{
 const settings=settle(options);
 if(!await storeReachableV1(settings.storeRoot))throw Error(`Replica store not found: ${storePathsV1(settings.storeRoot).root}`);
 const replica=await openReplicaV1(options,sink);
 try{
  await normaliseGitConfigV1(settings.worktree,undefined,signal);
  let adopted=0;
  if(replica.files.size===0&&replica.queue.size===0){
   const folded=await foldInto(settings.storeRoot,replica.manifest,signal,sink,settings.now),{manifest,files,book}=replica;
   Object.assign(replica.cursors,folded.cursors);
   if(folded.hlc!==""){try{replica.clock=mergeClockV1(replica.clock,folded.hlc,settings.now());}catch{}}
   const rules=manifest.get(".gitignore");
   if(rules?.operation==="put"){
    const text=await readFile(blobPathV1(settings.storeRoot,rules.sha256!),"utf8");
    replica.ignore=guarded(compileIgnoreV1(text));replica.ignoreHash=digest(text);
   }
   const seenMs=settings.now(),found=new ReplicaFoundV1(book),conflicts:string[]=[],present=new Bits(),adoptions=new CompactTableV1(book,true);
   await scanSharedV1(settings.worktree,replica.ignore,signal,entry=>{found.push(entry);});
   const pace=pacer(paceEntries);
   await pool(indexes(found.length),settings.concurrency,async index=>{
    pace();
    const entry=found.at(index),id=book.find(entry.path);
    if(id!==undefined)present.mark(id);
    const operation=id===undefined?undefined:manifest.operationAt(id);
    let match=false;
    if(operation==="link")match=entry.kind==="link"&&entry.target===manifest.targets.get(id!);
    else if(operation==="put"&&entry.kind==="file"&&entry.bytes===manifest.bytes[id!])match=manifest.shaIs(id!,(await hashFileV1(absolute(settings.worktree,entry.path))).sha256);
    if(match)adoptions.write(id!,{...manifest.entryAt(id!),mtimeMs:entry.mtimeMs,seenMs});
    else conflicts.push(entry.path);
   },signal);
   if(conflicts.length>0)throw new JoinRefusedV1(conflicts.sort((left,right)=>left<right?-1:left>right?1:0));
   for(const id of adoptions.ids()){
    files.copyRow(adoptions,id);replica.folds.add(id);replica.changed.add(filesKey(id));adopted++;
   }
   for(const id of manifest.ids()){
    if(files.live(id))continue;
    if(manifest.operationAt(id)==="delete"&&!present.has(id)){
     files.write(id,{...manifest.entryAt(id),mtimeMs:0});replica.changed.add(filesKey(id));
    }else enqueueId(replica,id);
   }
   replica.dirty=true;
   await persist(replica,true);
  }
  const total=replica.files.size+replica.queue.size;
  await applyQueue(replica,signal,"join");
  await persist(replica,true);
  return {files:total,adopted,queued:replica.queue.size};
 }finally{await closeReplicaV1(replica);}
}

/** ⛔️ Raised by a join into a non-empty worktree whose shared files differ from the store. */
export class JoinRefusedV1 extends Error{
 readonly conflicts:readonly string[];
 constructor(conflicts:readonly string[]){super(`Worktree holds ${conflicts.length} shared files that differ from the store: ${conflicts.slice(0,5).join(", ")}`);this.name="JoinRefusedV1";this.conflicts=conflicts;}
}

/** 📸️ Writes a snapshot of the folded store as `snapshots/<originId>/<hlc>.json`; the hlc is the greatest folded hlc, so an unchanged store yields the same file. */
export async function snapshotV1(storeRoot:string,originId:string,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<SnapshotResultV1|undefined>{
 requireOriginIdV1(originId);
 let incomplete=false;
 const folded=await foldStoreV1(storeRoot,signal,event=>{if(event.event==="rejoin-required")incomplete=true;sink(event);});
 if(incomplete){sink({version:1,event:"refused",stage:"snapshot",reason:"rejoin-required"});return undefined;}
 if(folded.hlc===""||folded.manifest.size===0)return undefined;
 const existing=join(storePathsV1(storeRoot).snapshots,originId,`${folded.hlc}.json`);
 if(await stat(existing).then(()=>true,()=>false))return {file:existing,hlc:folded.hlc,entries:folded.manifest.size,created:false};
 const file=await writeSnapshotV1(storeRoot,originId,folded.hlc,folded.manifest,folded.cursors);
 sink({version:1,event:"snapshot",path:file,hlc:folded.hlc,count:folded.manifest.size});
 return {file,hlc:folded.hlc,entries:folded.manifest.size,created:true};
}

async function snapshotCursors(file:string):Promise<Record<string,FleetCursorV1>>{
 let cursors:Record<string,FleetCursorV1>={};
 await streamMembersV1(file,new Set(["manifest"]),(outer,_inner,text)=>{if(outer==="cursors")cursors=JSON.parse(text) as Record<string,FleetCursorV1>;});
 return cursors;
}

async function referencesSinceV1(storeRoot:string,cursors:Record<string,FleetCursorV1>,into:Set<string>):Promise<boolean>{
 for(const origin of await listOriginsV1(storeRoot)){
  for(;;){
   const before=cursors[origin]??{segment:0,bytes:0};
   let read:Spans;
   try{read=await readEventSpansV1(storeRoot,origin,before,1);}catch(error){if(gone(error))return false;throw error;}
   for(const event of read.events)if(event.operation==="put")into.add(event.sha256!);
   cursors[origin]=read.cursor;
   if(read.cursor.segment===before.segment&&read.cursor.bytes===before.bytes)break;
  }
 }
 return true;
}

/** 🗜️ Compacts the own part of a store: writes a snapshot, then deletes this origin's sealed segments below the snapshot cursor and older own snapshots, and with `collectBlobs` (group servers only, default off, because a worker's view of the other origins may lag) also the blobs unreferenced by the folded manifest, all only when older than `retentionDays`; another origin's directory is never touched. Blobs are collected as a stream: each blob directory is listed entry by entry and at most 2,048 doomed blobs per worker wait for deletion, so memory follows the live tree and not the garbage; before every deletion batch the events published since the fold are added to the references, and a segment that vanished meanwhile stops the collection. `progress` (stage `compact`, directories done of 256, `count` blobs and `bytes` removed so far) is reported per directory and the signal cancels between files. Every compaction also removes staging files in `<store>/v1/tmp` older than 6 hours (leftovers of a `kill -9`, also those of the relay and of other origins). */
export async function compactV1(storeRoot:string,originId:string,retentionDays:number,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},now:()=>number=Date.now,options:Readonly<{collectBlobs?:boolean}>={}):Promise<CompactResultV1>{
 requireOriginIdV1(originId);
 let incomplete=false;
 const track:ReplicaSinkV1=event=>{if(event.event==="rejoin-required")incomplete=true;sink(event);};
 const snapshot=await snapshotV1(storeRoot,originId,signal,track),limit=now()-retentionDays*86_400_000,store=storePathsV1(storeRoot);
 let segments=0,blobs=0,snapshots=0,bytes=0;
 const remove=async (file:string,info:Stats):Promise<boolean>=>{
  if(info.mtimeMs>limit)return false;
  try{await unlink(file);bytes+=info.size;return true;}catch(error){if(gone(error))return false;throw error;}
 };
 const staging=await reclaimStagingV1(storeRoot,{olderThanMilliseconds:staleStagingMilliseconds,now:now()});
 bytes+=staging.bytes;
 if(snapshot===undefined)return {snapshot,segments,blobs,snapshots,staging:staging.files,bytes};
 const folded=await foldStoreV1(storeRoot,signal,track),own=(await snapshotCursors(snapshot.file))[originId],directory=join(store.origins,originId,"segments");
 if(own!==undefined&&!incomplete){
  for(const name of await readdir(directory).catch(()=>[] as string[])){
   if(!segmentPattern.test(name)||Number(name.slice(0,12))>=own.segment)continue;
   const file=join(directory,name);
   if(await remove(file,await stat(file)))segments++;
  }
 }
 const owned=join(store.snapshots,originId),names=(await readdir(owned).catch(()=>[] as string[])).filter(name=>name.endsWith(".json")).sort();
 for(const name of names.slice(0,-2)){const file=join(owned,name);if(await remove(file,await stat(file)))snapshots++;}
 if(options.collectBlobs===true){
  const referenced=new Set<string>(),seen={...folded.cursors};
  for(const entry of folded.manifest.values())if(entry.operation==="put")referenced.add(entry.sha256!);
  const prefixes=(await readdir(store.blobs).catch(()=>[] as string[])).filter(name=>/^[0-9a-f]{2}$/u.test(name));
  let done=0,syncing:Promise<void>=Promise.resolve();
  const refresh=():Promise<void>=>syncing=syncing.then(async()=>{if(!await referencesSinceV1(storeRoot,seen,referenced))incomplete=true;});
  const collect=async (doomed:[string,Stats][]):Promise<void>=>{
   await refresh();
   for(const [file] of doomed.splice(0)){
    signal?.throwIfAborted();
    if(incomplete)return;
    const fresh=await stat(file).catch(()=>undefined);
    if(fresh!==undefined&&!referenced.has(file.slice(file.length-64))&&await remove(file,fresh))blobs++;
   }
  };
  await pool(prefixes,8,async prefix=>{
   const folder=join(store.blobs,prefix),doomed:[string,Stats][]=[];
   for await(const item of await opendir(folder)){
    signal?.throwIfAborted();
    if(!hexPattern.test(item.name)||referenced.has(item.name))continue;
    const file=join(folder,item.name),info=await stat(file).catch(()=>undefined);
    if(info!==undefined&&info.mtimeMs<=limit)doomed.push([file,info]);
    if(doomed.length>=collectBatch)await collect(doomed);
   }
   await collect(doomed);
   sink({version:1,event:"progress",stage:"compact",done:++done,total:prefixes.length,count:blobs,bytes});
  },signal);
 }
 sink({version:1,event:"compacted",count:segments+blobs+snapshots+staging.files,bytes});
 return {snapshot,segments,blobs,snapshots,staging:staging.files,bytes};
}

/** 🥁️ What a fleet compaction did, or in `plan` mode would do: the snapshot it rests on, the journal segments and blobs removed (or removable), the blobs left because they are younger than the safety age, the old snapshots removed, and why it did nothing (`locked` by another machine, `refused` because the fold needs a rejoin, `empty` store, `nosnapshot` in plan mode when no snapshot exists yet, `aborted` blob reclaim). */
export type FleetCompactResultV1=Readonly<{mode:"plan"|"apply";snapshot:string|null;entries:number;segments:number;segmentBytes:number;blobs:number;blobBytes:number;pending:number;pendingBytes:number;snapshots:number;ignored:Readonly<Record<string,string>>;skipped:"locked"|"refused"|"empty"|"nosnapshot"|"aborted"|null}>;

/** 🎴️ Compacts the whole store so that the cost of joining, catching up and rebuilding follows the live paths and not the history: writes a snapshot of the fold (one machine; two writing at once is safe, each snapshot is a complete fold written by tmp, fsync and rename), checks the snapshot by reading it whole, removes the journal segments of every origin that the snapshot covers, that every fresh origin with synchronisation on has attested to have passed and that are older than `safetyHours` (origins silent for `staleHours`, off or retired do not hold it back and resume from the snapshot when they return), reclaims blobs that neither the snapshot nor a later event names and that are older than `safetyHours` (see `reclaimBlobsV1`), removes snapshots beyond the newest two and writes `compaction.json`. In `plan` mode nothing is written: it measures against the newest existing snapshot. `apply` holds the store-wide compaction lock; a held lock gives `locked`. Cancellable, with `progress` events of the stages `retire` and `reclaim`. */
export async function fleetCompactV1(storeRoot:string,originId:string,settings:CompactionSettingsV1,mode:"plan"|"apply",signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},now:()=>number=Date.now,options:Readonly<{blobs?:boolean}>={}):Promise<FleetCompactResultV1>{
 requireOriginIdV1(originId);
 const safety=settings.safetyHours*3_600_000,stale=settings.staleHours*3_600_000,blank:FleetCompactResultV1={mode,snapshot:null,entries:0,segments:0,segmentBytes:0,blobs:0,blobBytes:0,pending:0,pendingBytes:0,snapshots:0,ignored:{},skipped:null};
 const work=async():Promise<FleetCompactResultV1>=>{
  let ref:SnapshotRefV1|undefined;
  if(mode==="apply"){
   let incomplete=false;
   const made=await snapshotV1(storeRoot,originId,signal,event=>{if(event.event==="rejoin-required")incomplete=true;sink(event);});
   if(made===undefined||incomplete)return {...blank,skipped:incomplete?"refused":"empty"};
   ref=(await listSnapshotsV1(storeRoot)).find(item=>item.file===made.file);
  }else ref=(await listSnapshotsV1(storeRoot))[0];
  if(ref===undefined)return {...blank,skipped:"nosnapshot"};
  const referenced=new Set<string>(),entries=await scanSnapshotV1(ref.file,ref.hlc,options.blobs===false?undefined:sha=>{referenced.add(sha);});
  const plan=await planRetirementV1(storeRoot,ref,{now:now(),safetyMilliseconds:safety,staleMilliseconds:stale,signal});
  let segments=plan.segments,segmentBytes=plan.bytes,reclaimed:ReclaimResultV1={scanned:0,blobs:0,bytes:0,pending:0,pendingBytes:0,restored:0,aborted:false},snapshots=0;
  if(mode==="apply"){const removed=await retireSegmentsV1(storeRoot,plan,{signal,progress:(done,total)=>sink({version:1,event:"progress",stage:"retire",done,total})});segments=removed.segments;segmentBytes=removed.bytes;}
  if(options.blobs!==false)reclaimed=await reclaimBlobsV1(storeRoot,ref,{now:now(),safetyMilliseconds:safety,apply:mode==="apply",signal,referenced,progress:(done,total,blobs,bytes)=>sink({version:1,event:"progress",stage:"reclaim",done,total,count:blobs,bytes})});
  if(mode==="apply"){
   snapshots=(await pruneSnapshotsV1(storeRoot,{keep:2,now:now(),safetyMilliseconds:safety,signal})).files;
   await reclaimStagingV1(storeRoot,{olderThanMilliseconds:staleStagingMilliseconds,now:now()});
   const litter=await reclaimLitterV1(storeRoot,{olderThanMilliseconds:safety,now:now(),signal});
   if(litter.files>0)sink({version:1,event:"progress",stage:"litter",count:litter.files,bytes:litter.bytes});
   await writeCompactionRecordV1(storeRoot,{version:1,wallMs:now(),originId,snapshot:{hlc:ref.hlc,origin:ref.origin,entries},segments:{removed:segments,bytes:segmentBytes},blobs:{removed:reclaimed.blobs,bytes:reclaimed.bytes,pending:reclaimed.pending,pendingBytes:reclaimed.pendingBytes},snapshots});
   sink({version:1,event:"compacted",count:segments+reclaimed.blobs+snapshots,bytes:segmentBytes+reclaimed.bytes});
  }
  return {mode,snapshot:ref.file,entries,segments,segmentBytes,blobs:reclaimed.blobs,blobBytes:reclaimed.bytes,pending:reclaimed.pending,pendingBytes:reclaimed.pendingBytes,snapshots,ignored:plan.ignored,skipped:reclaimed.aborted?"aborted":null};
 };
 if(mode==="plan")return work();
 const held=await withCompactionLockV1(storeRoot,originId,work,now);
 return held===undefined?{...blank,skipped:"locked"}:held.value;
}

const compacting=new WeakSet<ReplicaV1>(),compactedAt=new WeakMap<ReplicaV1,number>(),autoCompactMilliseconds=600_000;

/** 🃏️ The automatic compaction of the group server (origin `<group>-server`), off unless `replica.compaction.automatic` is set: at most every ten minutes it compares the newest snapshot with the heads and runs `fleetCompactV1` in `apply` mode when there is none, when it is `snapshotEvents` events behind, or when it is behind at all and `safetyHours` old (so that segments and blobs that have come of age are removed once a day). Returns the running promise, or `undefined` when nothing was started; never throws. */
export function autoCompactV1(replica:ReplicaV1,signal?:AbortSignal):Promise<void>|undefined{
 const {settings}=replica,{compaction}=settings,now=settings.now();
 if(!compaction.automatic||settings.originId!==`${settings.groupId}-server`||compacting.has(replica)||now-(compactedAt.get(replica)??-Infinity)<autoCompactMilliseconds)return undefined;
 compactedAt.set(replica,now);compacting.add(replica);
 return (async()=>{
  const heads=new Map<string,FleetHeadV1|undefined>();
  for(const origin of await listOriginsV1(settings.storeRoot))heads.set(origin,await readHeadV1(settings.storeRoot,origin));
  const newest=(await listSnapshotsV1(settings.storeRoot))[0],behind=estimateEventsBehindV1(newest?.cursors??{},heads);
  if(newest===undefined||behind>=compaction.snapshotEvents||behind>0&&now-newest.mtimeMs>=compaction.safetyHours*3_600_000)await fleetCompactV1(settings.storeRoot,settings.originId,compaction,"apply",signal,event=>replica.say(event as Omit<ReplicaEventV1,"version">),settings.now);
 })().catch(error=>{if(signal?.aborted!==true)noteOnce(replica,`compaction:${messageOf(error)}`,{event:"error",stage:"compaction",message:messageOf(error)});}).finally(()=>{compacting.delete(replica);});
}

/** 🔍️ Rehashes the local shared tree and compares it with the persisted state and with the folded store; the drift list is empty exactly when all three agree on every shared path. Every entry is compared as soon as it is hashed, so the tree is never held as hashes. */
export async function verifyV1(options:ReplicaOptionsV1,signal?:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<VerifyReportV1>{
 const settings=settle(options),paths=localPathsV1(settings.worktree),ignore=guarded(compileIgnoreV1(await readIgnoreText(paths))),tables=tablesV1(),{book,files,manifest,queue}=tables;
 await loadLocal(paths,tables);
 await foldInto(settings.storeRoot,manifest,signal,sink,settings.now);
 const found=new ReplicaFoundV1(book),seen=new Bits(),drifted=new Bits(),drift:Drift[]=[];
 const {refused}=await scanSharedV1(settings.worktree,ignore,signal,entry=>{found.push(entry);});
 let total=0;
 for(let index=0;index<found.length;index++)if((found.flags[index]!&1)===0)total++;
 let hashed=0;
 const pace=pacer(paceEntries);
 await pool(indexes(found.length),settings.concurrency,async index=>{
  pace();
  const entry=found.at(index),path=entry.path;
  let sha:string|undefined;
  if(entry.kind==="file"){
   try{sha=(await hashFileV1(absolute(settings.worktree,path))).sha256;}catch(error){if(!gone(error))throw error;}
   if(++hashed%500===0)sink({version:1,event:"progress",stage:"verify",done:hashed,total});
  }
  const id=book.find(path),operation=id===undefined?undefined:manifest.operationAt(id);
  if(id!==undefined)seen.mark(id);
  let bad=true;
  if(operation===undefined||operation==="delete")drift.push({path,kind:"extra-local",expected:"absent"});
  else if(operation==="link"){
   if(entry.kind!=="link"||entry.target!==manifest.targets.get(id!))drift.push({path,kind:"target",expected:manifest.targets.get(id!),actual:entry.target??"file"});else bad=false;
  }else if(entry.kind!=="file"||sha===undefined||!manifest.shaIs(id!,sha))drift.push({path,kind:"content",expected:manifest.shaAt(id!),actual:sha??"link"});
  else bad=false;
  if(bad){if(id!==undefined)drifted.mark(id);return;}
  if(!files.live(id!)||queue.hasId(id!)||files.operationAt(id!)==="delete")return;
  if(!files.sameHlcAs(manifest,id!))drift.push({path,kind:"state",expected:manifest.hlcAt(id!),actual:files.hlcAt(id!)});
  else if(files.operationAt(id!)==="put"&&(sha===undefined||!files.shaIs(id!,sha)))drift.push({path,kind:"state",expected:files.shaAt(id!),actual:sha});
 },signal);
 sink({version:1,event:"progress",stage:"verify",done:total,total});
 for(const id of manifest.ids()){
  const path=book.name(id),operation=manifest.operationAt(id);
  if(seen.has(id)||operation==="delete"||!sharedV1(path,false,ignore))continue;
  if(files.live(id)&&files.operationAt(id)==="delete"&&files.sameHlcAs(manifest,id))continue;
  drift.push({path,kind:"missing-local",expected:operation==="put"?manifest.shaAt(id):manifest.targets.get(id)});
  drifted.mark(id);
 }
 for(const id of files.ids()){
  if(seen.has(id)||drifted.has(id)||queue.hasId(id)||files.operationAt(id)==="delete"||indexRealmV1(book.name(id)))continue;
  drift.push({path:book.name(id),kind:"state",expected:manifest.live(id)&&files.sameHlcAs(manifest,id)?undefined:manifest.live(id)?manifest.hlcAt(id):undefined,actual:files.hlcAt(id)});
 }
 const stage=await verifyIndexV1({worktree:settings.worktree,server:false,now:settings.now,base:()=>manifest.get(INDEX_BASE_V1),each:visit=>{for(const id of manifest.ids()){const path=book.name(id);if(path.startsWith(INDEX_PREFIX_V1))visit(path,manifest.entryAt(id));}},held:()=>{},pending:()=>new Map(),emit:async()=>undefined,settle:()=>{},enqueue:()=>{},say:()=>{}},signal);
 if(!stage.ok)drift.push({path:".git/index",kind:"content",expected:stage.expected??undefined,actual:stage.actual??undefined});
 drift.sort((left,right)=>left.path<right.path?-1:left.path>right.path?1:0);
 return {version:1,ok:drift.length===0,checked:found.length,total:drift.length,truncated:drift.length>driftLimit,drift:drift.slice(0,driftLimit),refusedTotal:refused.length,refused:refused.slice(0,driftLimit)};
}

async function countLines(file:string,start:number,end:number):Promise<number>{
 let count=0;
 for await(const chunk of createReadStream(file,{start,end:end-1}))for(const byte of chunk as Buffer)if(byte===10)count++;
 return count;
}

async function lagOf(storeRoot:string,origin:string,cursor:FleetCursorV1|undefined,head:FleetHeadV1):Promise<Readonly<{events:number;bytes:number;rejoin:boolean}>>{
 const from=cursor??{segment:0,bytes:0},directory=join(storePathsV1(storeRoot).origins,origin,"segments");
 let events=0,bytes=0;
 if(from.segment>head.segment||from.segment===head.segment&&from.bytes>=head.bytes)return {events,bytes,rejoin:false};
 try{
  for(let segment=from.segment;segment<=head.segment;segment++){
   const file=join(directory,segmentName(segment)),start=segment===from.segment?from.bytes:0,end=segment===head.segment?head.bytes:(await stat(file)).size;
   if(end<=start)continue;
   bytes+=end-start;events+=await countLines(file,start,end);
  }
 }catch(error){if(gone(error))return {events,bytes,rejoin:true};throw error;}
 return {events,bytes,rejoin:false};
}

async function countFiles(directory:string):Promise<number>{
 let count=0;
 for(const item of await readdir(directory,{withFileTypes:true}).catch(()=>[])){
  if(item.isDirectory())count+=await countFiles(join(directory,item.name));else count++;
 }
 return count;
}

/** 🩺 Reports per origin how far behind this machine is (events and bytes after its cursor, age of the last hlc), the queue sizes (`deep` also counts local changes not yet published), whether synchronization is on, and the number of preserved overwritten files. */
export async function statusV1(options:ReplicaOptionsV1,deep:boolean=false,signal?:AbortSignal):Promise<StatusReportV1>{
 const settings=settle(options),paths=localPathsV1(settings.worktree),tables=tablesV1(),local=await loadLocal(paths,tables),reachable=await storeReachableV1(settings.storeRoot),held=await readLock(paths.lock),sync=await syncEnabledV1(settings.worktree),beat=await readHealthV1(settings.worktree),pulse=await readBeatV1(settings.worktree,settings.now());
 const origins:StatusReportV1["origins"][number][]=[];
 const retirements=reachable?await readRetirementsV1(settings.storeRoot,signal):{},own=retirements[settings.originId];
 if(reachable){
  for(const origin of await listOriginsV1(settings.storeRoot)){
   signal?.throwIfAborted();
   const head=await readHeadV1(settings.storeRoot,origin);
   if(head===undefined)continue;
   const lag=origin===settings.originId||retirements[origin]?.retired===true?{events:0,bytes:0,rejoin:false}:await lagOf(settings.storeRoot,origin,local.cursors[origin],head);
   origins.push({origin,behindEvents:lag.events,behindBytes:lag.bytes,lastHlc:head.hlc,ageSeconds:Math.max(0,Math.floor((settings.now()-parseHlcV1(head.hlc).ms)/1000)),rejoinRequired:lag.rejoin,retired:retirements[origin]?.retired===true});
  }
 }
 let outgoing:number|null=null,refusedCount:readonly unknown[]|undefined;
 if(deep){
  const {book,files}=tables,seen=new Bits();
  outgoing=0;
  ({refused:refusedCount}=await scanSharedV1(settings.worktree,guarded(compileIgnoreV1(await readIgnoreText(paths))),signal,entry=>{
   const id=book.find(entry.path);
   if(id!==undefined)seen.mark(id);
   const operation=id===undefined?undefined:files.operationAt(id);
   if(entry.kind==="link"?operation!=="link"||files.targets.get(id!)!==entry.target:operation!=="put"||files.bytes[id!]!==entry.bytes||files.mtime[id!]!==entry.mtimeMs)outgoing!++;
  }));
  for(const id of files.ids())if(files.operationAt(id)!=="delete"&&!seen.has(id)&&!indexRealmV1(book.name(id)))outgoing++;
 }
 const stage=await gitStateV1(settings.worktree),health=await stageHealthV1(settings.worktree),git:StatusReportV1["git"]=await resolveGitV1()===null?{shared:false,reason:"no-git",indexTree:null}:!stage.present?{shared:false,reason:"no-repository",indexTree:null}:!tables.files.has(INDEX_BASE_V1)?{shared:false,reason:"waiting-for-base",indexTree:stage.indexTree}:health.reason!==null?{shared:false,reason:health.reason,indexTree:stage.indexTree}:{shared:true,reason:"shared",indexTree:stage.indexTree};
 const access=reachable?await readAccessV1(settings.storeRoot,settings.originId):undefined;
 const report:StatusReportV1={...access===undefined?{}:{access},version:1,originId:settings.originId,groupId:settings.groupId,storeRoot:settings.storeRoot,reachable,daemon:{running:held!==undefined&&!held.stale||pulse!==undefined,pid:held!==undefined&&!held.stale&&held.pid>0?held.pid:pulse?.pid??null,...pulse!==undefined&&pulse.lagMs>=5000?{busySeconds:Math.floor(pulse.lagMs/1000)}:{}},...pulse===undefined?{}:{code:{source:pulse.source,phase:pulse.phase}},sync,files:tables.files.size-tables.files.count("delete"),queue:{incoming:tables.queue.size,outgoing},lost:await countFiles(paths.lost),refused:refusedCount?.length??null,git,origins,latency:reachable?(await readAttestationV1(settings.storeRoot,settings.originId))?.latency??null:null,retired:own?.retired===true?{by:own.by,reason:own.reason,wallMs:own.wallMs}:null,health:beat===undefined||!(held!==undefined&&!held.stale||pulse!==undefined)?null:{wallMs:beat.wallMs,store:beat.store,storeSinceMs:beat.storeSinceMs,code:beat.code,pending:beat.pending,failures:beat.failures},store:reachable?await storeReportV1(settings.storeRoot,settings.compaction,settings.now(),signal,deep).catch(()=>null):null};
 if(report.store!==null)requireCompactionShapeV1("Store",report.store,"Closed compaction store report required");
 requireReplicaShapeV1("Status",report,"Closed replica status required");
 return report;
}

/** 🔬️ Loads the current local state of a worktree (checkpoint, log, cursors, clock, machine) into throw-away tables without taking the lock and without writing anything; `null` when this code can read it, else the reason. The self-check uses it so that a daemon never restarts into code that cannot read the state of its machine. */
export async function dryOpenStateV1(worktree:string):Promise<string|null>{
 try{await loadLocal(localPathsV1(worktree),tablesV1());return null;}catch(error){return messageOf(error);}
}

const homes=new Map<string,string>(),verifiedBlobs=new Set<string>();

async function homeOf(store:string,origin:string):Promise<string|undefined>{
 const key=`${store}\0${origin}`,known=homes.get(key);
 if(known!==undefined)return known;
 try{
  const value=JSON.parse(await readFile(join(storePathsV1(store).origins,origin,"home.json"),"utf8")) as unknown;
  requireReplicaShapeV1("Home",value,"Closed replica home required");
  if(homes.size>=4096)homes.clear();
  homes.set(key,(value as {group:string}).group);
  return (value as {group:string}).group;
 }catch{return undefined;}
}

/** 🏠️ Lists the origins of a store whose recorded home group is the given group: the origins a relay may pull from that store. */
export async function homedOriginsV1(store:string,group:string):Promise<readonly string[]>{
 const homed:string[]=[];
 for(const origin of await listOriginsV1(store))if(await homeOf(store,origin)===group)homed.push(origin);
 return homed;
}

async function copyRange(source:string,from:number,to:number):Promise<Buffer>{
 const handle=await open(source,"r");
 try{
  const length=Math.max(0,to-from),data=Buffer.alloc(length);
  let done=0;
  while(done<length){const {bytesRead}=await handle.read(data,done,length-done,from+done);if(bytesRead===0)break;done+=bytesRead;}
  return data.subarray(0,done);
 }finally{await handle.close();}
}

async function replaceText(store:string,tmp:string,target:string,text:string):Promise<void>{
 await ensureStoreDirectoryV1(store,tmp);await ensureStoreDirectoryV1(store,dirname(target));
 const temporary=join(tmp,`relay-${token()}`);
 try{
  const handle=await open(temporary,"w");
  try{await handle.writeFile(text);await handle.sync();}finally{await handle.close();}
  await rename(temporary,target);
 }catch(error){await rm(temporary,{force:true});throw error;}
}

const auxiliaryChecked=new Map<string,number>(),auxiliaryMilliseconds=30_000,relayAudited=new Map<string,number>();
const snapshotNamePattern=/^[0-9]{13}-[0-9]{6}-[a-z0-9][a-z0-9-]{0,39}\.json$/u;

async function relayBlob(ownStore:string,peerStore:string,sha256:string,origin:string,sink:ReplicaSinkV1,signal?:AbortSignal):Promise<boolean>{
 const destination=blobPathV1(ownStore,sha256),own=storePathsV1(ownStore);
 if(await stat(destination).then(()=>true,()=>false)){
  if(verifiedBlobs.has(destination)||(await hashFileV1(destination).catch(()=>undefined))?.sha256===sha256){if(verifiedBlobs.size>=100_000)verifiedBlobs.clear();verifiedBlobs.add(destination);if(await touchBlobV1(destination))return false;}
  sink({version:1,event:"blob-missing",origin,reason:`blob-corrupt:${sha256}`});
 }
 const temporary=join(own.tmp,`relay-${token()}`);
 try{
  await ensureStoreDirectoryV1(ownStore,own.tmp);await ensureStoreDirectoryV1(ownStore,dirname(destination));
  const emit=throttled((done,total)=>sink({version:1,event:"progress",stage:"relay-blob",origin,reason:sha256,done,total}));
  if(await copyVerifiedV1(blobPathV1(peerStore,sha256),temporary,sha256,{signal,sync:true,clone:true,progress:(done,total)=>{if(total>=largeBlobBytesV1)emit(done,total);}})==="corrupt")throw Error(`Relay blob differs from its hash: ${sha256}`);
  await rename(temporary,destination);
  return true;
 }catch(error){
  await rm(temporary,{force:true});
  if(!gone(error)||!await storeReachableV1(peerStore)||!await storeReachableV1(ownStore))throw error;
  sink({version:1,event:"blob-missing",origin,reason:sha256});
  return false;
 }
}

async function relaySnapshot(ownStore:string,peerStore:string,origin:string,signal:AbortSignal|undefined,sink:ReplicaSinkV1):Promise<number>{
 const source=join(storePathsV1(peerStore).snapshots,origin),own=storePathsV1(ownStore);
 const newest=(await readdir(source).catch(()=>[] as string[])).filter(name=>snapshotNamePattern.test(name)).sort().slice(-1)[0];
 if(newest===undefined)return 0;
 const target=join(own.snapshots,origin,newest);
 if(await stat(target).then(()=>true,()=>false))return 0;
 const file=join(source,newest),found=new Set<string>();
 try{
  await streamMembersV1(file,new Set(["manifest"]),(outer,_inner,text)=>{
   if(outer!=="manifest")return;
   const entry=JSON.parse(text) as {operation?:string;sha256?:unknown};
   if(entry.operation==="put"&&typeof entry.sha256==="string"&&hexPattern.test(entry.sha256))found.add(entry.sha256);
  });
 }catch{return 0;}
 const shas=[...found];
 let copied=0;
 await pool(shas,8,async sha256=>{if(await relayBlob(ownStore,peerStore,sha256,origin,sink,signal))copied++;},signal);
 await ensureStoreDirectoryV1(ownStore,own.tmp);await ensureStoreDirectoryV1(ownStore,dirname(target));
 const temporary=join(own.tmp,`relay-${token()}`);
 try{
  await copyFile(file,temporary);
  const handle=await open(temporary,"r+");
  try{await handle.sync();}finally{await handle.close();}
  await rename(temporary,target);
 }catch(error){await rm(temporary,{force:true});throw error;}
 return copied;
}

async function relayOriginFile(ownStore:string,peerStore:string,origin:string,name:string):Promise<void>{
 const own=storePathsV1(ownStore),peer=storePathsV1(peerStore),text=await readFile(join(peer.origins,origin,name),"utf8").catch(()=>undefined);
 if(text!==undefined&&text!==await readFile(join(own.origins,origin,name),"utf8").catch(()=>undefined))await replaceText(ownStore,own.tmp,join(own.origins,origin,name),text);
}

async function relayAttestation(ownStore:string,peerStore:string,origin:string):Promise<void>{
 await relayOriginFile(ownStore,peerStore,origin,"view.txt");
 await relayOriginFile(ownStore,peerStore,origin,"attest.json");
}

async function relayRetirement(ownStore:string,peerStore:string,origin:string):Promise<void>{
 const peer=await readRetirementV1(peerStore,origin);
 if(peer===undefined)return;
 const own=await readRetirementV1(ownStore,origin);
 if(own===undefined||peer.hlc>own.hlc)await replaceText(ownStore,storePathsV1(ownStore).tmp,join(storePathsV1(ownStore).origins,origin,"retired.json"),JSON.stringify(peer));
}

/** 🔀️ One relay round from a read-only peer store into the own store, for the origins homed at that peer: missing blobs first (verified by hash), then only the missing bytes of the segments (appended, earlier bytes are never rewritten; a blob that cannot be read because a store vanished aborts the round instead of letting the events overtake their blobs), then `head.json` through tmp + rename; `attest.json` of every origin is copied as a whole file on every round, also when the origin has no head yet; `retired.json` of every origin of the peer store, homed there or not, is copied when its `hlc` is newer than the own record (so a retirement and its undo both spread); repeating it copies nothing. The own store root is created only when `create` is true (a fresh store); the relay loop sets it false once the store has been seen. */
export async function relayOnceV1(ownStore:string,peerStore:string,originsHomedAtPeer:readonly string[],signal?:AbortSignal,sink:ReplicaSinkV1=()=>{},create:boolean=true,concurrency:number=relayConcurrency):Promise<RelayResultV1>{
 const own=storePathsV1(ownStore),peer=storePathsV1(peerStore);
 if(create)await mkdir(own.root,{recursive:true});
 let copiedOrigins=0,blobs=0,segments=0,bytes=0;
 const noFlat=():ReadonlyMap<string,FleetHeadV1>=>new Map<string,FleetHeadV1>(),peerFlat=await readFlatHeadsV1(peerStore).catch(noFlat),ownFlat=await readFlatHeadsV1(ownStore).catch(noFlat),pair=`${ownStore}\0${peerStore}`,now=Date.now(),audit=now-(relayAudited.get(pair)??0)>=auditMilliseconds;
 if(audit){if(relayAudited.size>=64)relayAudited.clear();relayAudited.set(pair,now);}
 await pool(originsHomedAtPeer,concurrency,async origin=>{
  signal?.throwIfAborted();
  requireOriginIdV1(origin);
  await relayAttestation(ownStore,peerStore,origin);
  const hint=peerFlat.get(origin),polled=audit||hint===undefined?await readHeadV1(peerStore,origin):undefined,head=polled!==undefined&&hint!==undefined&&aheadOf(hint,polled)?hint:polled??hint;
  if(head===undefined)return;
  const have=await readHeadV1(ownStore,origin),auxiliary=`${ownStore}\0${peerStore}\0${origin}`;
  if(have!==undefined&&(ownFlat.get(origin)===undefined||aheadOf(have,ownFlat.get(origin)!)))await publishFlatHeadV1(ownStore,origin,have).catch(()=>{});
  if(have!==undefined&&have.segment===head.segment&&have.bytes===head.bytes&&Date.now()-(auxiliaryChecked.get(auxiliary)??0)<auxiliaryMilliseconds)return;
  if(auxiliaryChecked.size>=4096)auxiliaryChecked.clear();
  auxiliaryChecked.set(auxiliary,Date.now());
  const ownHome=join(own.origins,origin,"home.json"),home=await readFile(join(peer.origins,origin,"home.json"),"utf8").catch(()=>undefined);
  if(home!==undefined&&home!==await readFile(ownHome,"utf8").catch(()=>undefined))await replaceText(ownStore,own.tmp,ownHome,home);
  blobs+=await relaySnapshot(ownStore,peerStore,origin,signal,sink);
  if(have!==undefined&&have.segment===head.segment&&have.bytes===head.bytes)return;
  const sourceDirectory=join(peer.origins,origin,"segments"),targetDirectory=join(own.origins,origin,"segments");
  const numbers=(await readdir(sourceDirectory)).filter(name=>segmentPattern.test(name)).map(name=>Number(name.slice(0,12))).filter(number=>number<=head.segment&&number>=(have?.segment??0)).sort((left,right)=>left-right);
  let moved=0;
  for(const number of numbers){
   const source=join(sourceDirectory,segmentName(number)),target=join(targetDirectory,segmentName(number)),limit=number===head.segment?head.bytes:(await stat(source)).size,offset=(await stat(target).catch(()=>undefined))?.size??0;
   if(offset>=limit)continue;
   const delta=await copyRange(source,offset,limit),shas=new Set<string>();
   for(const line of delta.toString("utf8").split("\n")){
    if(line==="")continue;
    try{const event=JSON.parse(line) as {operation?:string;sha256?:string};if(event.operation==="put"&&typeof event.sha256==="string"&&hexPattern.test(event.sha256))shas.add(event.sha256);}catch{}
   }
   await pool([...shas],concurrency,async sha256=>{if(await relayBlob(ownStore,peerStore,sha256,origin,sink,signal))blobs++;},signal);
   await ensureStoreDirectoryV1(ownStore,targetDirectory);
   const handle=await open(target,"a");
   try{await handle.write(delta);await handle.sync();}finally{await handle.close();}
   segments++;bytes+=delta.length;moved+=delta.length;
  }
  await replaceText(ownStore,own.tmp,join(own.origins,origin,"head.json"),JSON.stringify(head));
  await publishFlatHeadV1(ownStore,origin,head).catch(()=>{});
  copiedOrigins++;
  sink({version:1,event:"relayed",origin,bytes:moved});
 },signal);
 await pool(await listOriginsV1(peerStore),concurrency,async origin=>{signal?.throwIfAborted();await relayRetirement(ownStore,peerStore,origin);},signal);
 return {origins:copiedOrigins,blobs,segments,bytes};
}

/** 🛰️ The relay loop of a group server: for every peer store it lists the origins homed there and relays them into the own store every poll interval; an unreachable peer is reported once per outage and retried. Once the own store has been seen, a vanished own store (unmounted share) pauses the relay instead of being recreated as a local directory. */
export async function relayV1(options:RelayOptionsV1,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 const down=new Set<string>(),interval=options.pollMilliseconds??1000;
 let ownSeen=await storeReachableV1(options.ownStore),ownDown=false;
 sink({version:1,event:"started",stage:"relay",count:Object.keys(options.peers).length});
 while(!signal.aborted){
  if(ownSeen&&!await storeReachableV1(options.ownStore)){
   if(!ownDown){ownDown=true;sink({version:1,event:"store-unreachable",path:options.ownStore});}
  }else{
   if(ownDown){ownDown=false;sink({version:1,event:"store-reachable",path:options.ownStore});}
  await Promise.all(Object.entries(options.peers).map(async ([group,store])=>{
   if(signal.aborted)return;
   try{
    if(!await storeReachableV1(store)){
     if(!down.has(group)){down.add(group);sink({version:1,event:"store-unreachable",groupId:group,path:store});}
     return;
    }
    if(down.delete(group))sink({version:1,event:"store-reachable",groupId:group});
    await relayOnceV1(options.ownStore,store,await homedOriginsV1(store,group),signal,sink,!ownSeen,options.concurrency);
   }catch(error){
    if(signal.aborted)return;
    if(!await storeReachableV1(store)){if(!down.has(group)){down.add(group);sink({version:1,event:"store-unreachable",groupId:group,path:store});}}
    else if(!ownSeen||await storeReachableV1(options.ownStore))sink({version:1,event:"error",groupId:group,message:messageOf(error)});
   }
  }));
  ownSeen=ownSeen||await storeReachableV1(options.ownStore);
  }
  await new Promise<void>(done=>{
   const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,interval);signal.addEventListener("abort",finish,{once:true});
  });
 }
 sink({version:1,event:"stopped",stage:"relay"});
}

/** ✍️ Tells whether every file of the code on disk that differs from the code that runs was last written by this machine: the files of the module and of everything it imports are compared with the sealed copy of the running code (`.cache/f/code/<running>/sealed.json`), and each one that differs, is new or is gone must have been published by this origin (the holder of its register in the local state). Code that arrived from another origin, even in part, is therefore never judged authored here, and a running code that was never sealed cannot be compared, so nothing is authored. */
export async function authoredCodeV1(replica:ReplicaV1,moduleRoot:string,running:string):Promise<boolean>{
 const root=replica.settings.worktree;
 let seal:SealedCodeV1;
 try{seal=JSON.parse(await readFile(join(codeDirectoryV1(root),running,"sealed.json"),"utf8")) as SealedCodeV1;}catch{return false;}
 const before=new Map(seal.files.map(file=>[file.path,file.sha256])),sources=await moduleSourcesV1(moduleRoot),paths=[...sources,...await importClosureV1(moduleRoot,sources)].map(path=>relative(root,resolve(moduleRoot,...path.split("/"))).split(sep).join("/")),now=new Set(paths);
 const mine=(path:string,deleted:boolean):boolean=>{const id=replica.book.find(path);return id!==undefined&&replica.files.live(id)&&(replica.files.operationAt(id)==="delete")===deleted&&replica.book.origins[replica.files.origin[id]!]===replica.settings.originId;};
 let changed=0;
 for(const path of paths){
  const data=await readFile(join(root,...path.split("/"))).catch(()=>undefined);
  if(data===undefined)return false;
  if(createHash("sha256").update(data).digest("hex")===before.get(path))continue;
  changed++;
  if(!mine(path,false))return false;
 }
 for(const path of before.keys())if(!now.has(path)){changed++;if(!mine(path,true))return false;}
 return changed>0;
}

/** 🚪️ The admission of new code on disk, asked before its self-check: a daemon switches only to code that some origin attests to run (any other origin whose attestation carries the digest, or whose healthy run left a code snapshot of it in the store) or that was authored on this machine (`authoredCodeV1`). Anything else is a change set that arrived from elsewhere and that nobody has proven: a mix of new and stale files, or a change whose rest is still on its way. It waits, is asked again every heartbeat, and the digest changes as more files arrive, so the decision is made again for every state of the module. `bounded` runs one store read under a deadline. */
export async function admitCodeV1(replica:ReplicaV1,input:Readonly<{digest:string;running:string;moduleRoot:string;bounded:<T>(work:()=>Promise<T>)=>Promise<T>;signal?:AbortSignal}>):Promise<Readonly<{ok:true}>|Readonly<{ok:false;reason:string}>>{
 const {storeRoot,originId}=replica.settings;
 if(input.digest===input.running)return {ok:true};
 const attestations=await input.bounded(()=>readAttestationsV1(storeRoot,input.signal));
 if(Object.values(attestations).some(item=>item.originId!==originId&&item.code===input.digest))return {ok:true};
 if((await input.bounded(()=>listStoreCodeV1(storeRoot))).some(item=>item.digest===input.digest))return {ok:true};
 if(await authoredCodeV1(replica,input.moduleRoot,input.running))return {ok:true};
 return {ok:false,reason:"no origin runs this code yet and it was not all written on this machine"};
}

/** 🔬️ Opens the state of this machine the way the daemon does, on a private mirror: the checkpoint (linked, never written), the log, the cursors, the clock and the identity are mirrored below a temporary directory and `openReplicaV1` runs on it without the lock, so everything the open does (loading every table, replaying the own tail, requeueing) happens with the checked code while the real state and the store stay untouched. `null` when it opens, else the reason. A machine without an identity has no state to open, and only the load is tried. */
export async function dryOpenReplicaV1(worktree:string):Promise<string|null>{
 const paths=localPathsV1(worktree),machine=await readMachine(paths.machine) as Machine|undefined;
 if(machine===undefined)return dryOpenStateV1(worktree);
 const mirror=await mkdtemp(join(tmpdir(),"fleet-dry-open-")),target=localPathsV1(mirror);
 try{
  await mkdir(target.directory,{recursive:true});
  for(const name of ["checkpoint","wal","takeoverState","takeoverLog","cursor","clock","machine"] as const){
   if(await stat(paths[name]).then(()=>false,()=>true))continue;
   if(name==="checkpoint"||name==="takeoverState")await symlink(paths[name],target[name]).catch(()=>copyFile(paths[name],target[name]));
   else await copyFile(paths[name],target[name]);
  }
  await openReplicaV1({worktree:mirror,storeRoot:machine.storeRoot,originId:machine.originId,groupId:machine.groupId,lock:false});
  return null;
 }catch(error){return messageOf(error);}
 finally{await rm(mirror,{recursive:true,force:true}).catch(()=>{});}
}

/** 💨️ Boots the daemon of the checked code once in a throw-away worktree and store: seeds three files, opens and closes the state (which writes a checkpoint), then runs `runV1` on it until its first attestation reaches the scratch store, so that a daemon that cannot start, cannot load its own checkpoint or cannot attest is found before any real machine restarts into it. `null` when it booted, else the reason. */
export async function smokeBootV1(signal?:AbortSignal,milliseconds:number=60_000):Promise<string|null>{
 const base=await mkdtemp(join(tmpdir(),"fleet-smoke-")),worktree=join(base,"w"),store=join(base,"s");
 try{
  await mkdir(worktree,{recursive:true});await writeFile(join(worktree,"smoke.txt"),"smoke\n");
  const options:ReplicaOptionsV1={worktree,storeRoot:store,originId:"smoke",groupId:"g0",pollMilliseconds:20,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,watch:false,selfCheck:async()=>({ok:true}),gitState:async()=>({present:false,head:null,commit:null,indexTree:null}),gitView:async()=>null,hintMilliseconds:20,logFiles:[],restartStableMilliseconds:3_600_000};
  await seedV1(options);
  await closeReplicaV1(await openReplicaV1(options));
  const controller=new AbortController(),events:ReplicaEventV1[]=[],abort=():void=>{controller.abort();};
  signal?.addEventListener("abort",abort,{once:true});
  let failure:string|null=null;
  const done=runV1(options,controller.signal,event=>{events.push(event);}).catch(error=>{failure=messageOf(error);});
  const end=Date.now()+milliseconds;
  while(failure===null&&!controller.signal.aborted&&await readAttestationV1(store,"smoke").then(found=>found===undefined,()=>true)){
   const bad=events.find(event=>event.event==="error");
   if(bad!==undefined){failure=bad.message??"error event";break;}
   if(Date.now()>end){failure=`the daemon wrote no attestation within ${milliseconds} ms`;break;}
   await new Promise<void>(next=>setTimeout(next,25));
  }
  controller.abort();signal?.removeEventListener("abort",abort);
  await done;
  return failure;
 }catch(error){return messageOf(error);}
 finally{await rm(base,{recursive:true,force:true}).catch(()=>{});}
}
