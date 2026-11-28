import {createHash,randomBytes} from "node:crypto";
import {appendFileSync,chmodSync,constants,copyFileSync,existsSync,lstatSync,mkdirSync,readFileSync,readdirSync,readlinkSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {basename,dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {buildDependencyPathsV1,differingComponentsV1,extraNeutralV1,extraRestoreV1,laneNeutralV1,laneRestoreV1,parseDepInfoV1,sha256V1,UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import {compilerWrittenV1} from "../🔏️key/🔗️sources/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {casObjectPathV1,casRootV1} from "../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {hasCasObjectSyncV1,putCasObjectSyncV1,readCasObjectSyncV1,touchCasObjectSyncV1} from "../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {casHolderIdV1,keepCasObjectsSyncV1,registerCasHolderSyncV1} from "../../../../🏗️build/🗄️cas/🫂️holders/🟦️.ts";
import {CAS_POLICY_V1} from "../../../../🏗️build/🗄️cas/🧬️schema/🟦️.ts";
import {claimUnitV1} from "../🧹️gc/🪢️supersession/🙋️claim/🟦️.ts";
import {uncoveredOutputsV1} from "../✅️complete/🟦️.ts";
import {declareV1,workspaceRowsV1} from "../../../../🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {type DeclarationV1} from "../../../../🏗️build/🪢️slots/🟦️.ts";
import buildPolicy from "../../../../🏗️build/🔣️policy.json" with {type:"json"};
import {pruneSpillV1,touchSpillObjectV1,type SpillPruneV1} from "./🧽️spill-bound/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1,layout=policy.layout;
type Definitions=typeof schema.definitions;
/** 📦️ One cached compilation: the files it read, the environment it observed, the files it wrote and the compiler's own messages. */
export type UnitEntryV1={version:1;key:string;slot?:string;session?:true;retention?:DeclarationV1;crate:string;dependencies:{path:string;sha256:string|null}[];environment:{name:string;value:string|null}[];outputs:{name:string;sha256:string;bytes:number;executable:boolean;template?:boolean}[];stderr:string|null;stdout:string|null;createdMs:number;provenance?:{peakRssBytes:number;userMs:number;sysMs:number;wallMs:number;rmetaOffsetMs:number|null}|null};
/** 🧾️ One measured cache decision (`Event` in the schema). */
export type UnitEventV1={version:1;atMs:number;lane:string;pid:number;kind:Definitions["Event"]["properties"]["kind"]["enum"][number];crate:string;key:string|null;milliseconds:number;waitedMs:number;reason?:string;peakRssBytes?:number};

/** 🗄️ Where one cache root keeps each kind of state. */
export function storePathsV1(root:string){return Object.freeze({root,objects:join(root,layout.objects),entries:join(root,layout.entries),locks:join(root,layout.locks),events:join(root,layout.events),temporary:join(root,layout.temporary)});}

function ensure(path:string):string{mkdirSync(path,{recursive:true});return path;}
/** 🗄️ Compiled objects live in the host-wide content-addressed store, shared and deduplicated by every workspace and bounded by its cap; an entry whose object the host collected is a miss. */
const objectPath=(sha:string)=>casObjectPathV1(casRootV1(),sha);

/** 🕳️ Whether a recorded entry lost an object it replays: an output or, for a build script run, one of its two streams. */
const lostObjectV1=(root:string,entry:Readonly<{outputs:readonly{sha256:string}[];stdout?:string|null;stderr?:string|null}>):boolean=>[...entry.outputs.map(row=>row.sha256),entry.stdout,entry.stderr].some(sha=>typeof sha==="string"&&!hasCasObjectSyncV1(casRootV1(),sha));
const entryDirectory=(root:string,key:string)=>join(root,layout.entries,key.slice(0,2),key);

/** 🫂️ The paths inside an entry file whose values are host-store digests: the outputs and, for a build script run, its two streams. */
const HELD_REFERENCES_V1=Object.freeze(["outputs.*.sha256","stdout","stderr"]);
/** 🫂️ Registers (or renews, at most every `holderRenewMs`) this cache root as a reference holder of the host store: the objects its live entries name are protected from the host collector and are the last it evicts, entry and objects together, when the cap still needs bytes. A root that stops renewing, or vanishes, ages out. Best effort, never throws.
 * @see ../../../../🏗️build/🗄️cas/🧬️schema/🔣️.json */
export function holdHostObjectsV1(root:string):void{
 try{registerCasHolderSyncV1(casRootV1(),CAS_POLICY_V1,{kind:"unit-cache",root:resolve(root),indexes:[layout.entries,layout.runs].map(name=>({directory:join(resolve(root),name),references:HELD_REFERENCES_V1})),evictionLog:logFiles(resolve(root),policy.evictions.file)[0]});}catch{}
}

/** 🔗️ Gives the objects an entry names their keeper link in the host store (a second hard link that every collector generation honours), so that no process, whatever code it runs, can collect what a live entry replays. Best effort, never throws.
 * @see ../../../../🏗️build/🗄️cas/🫂️holders/🟦️.ts */
export function keepHostObjectsV1(root:string,entry:Readonly<{outputs:readonly{sha256:string}[];stdout?:string|null;stderr?:string|null}>):void{
 try{keepCasObjectsSyncV1(casRootV1(),CAS_POLICY_V1,casHolderIdV1("unit-cache",resolve(root)),[...entry.outputs.map(row=>row.sha256),entry.stdout,entry.stderr].filter((sha):sha is string=>typeof sha==="string"));}catch{}
}

/** 🔏️ Content digest of a file, memoised per lane by (size, mtime, inode) so unchanged dependency artifacts are hashed once. */
export function digestFileV1(path:string,stampDirectory:string|undefined):string|null{
 let stat;try{stat=statSync(path,{bigint:true});}catch{return null;}
 if(!stat.isFile())return null;
 const stamp=stampDirectory?join(stampDirectory,sha256V1(path).slice(0,2),sha256V1(path).slice(2,34)):undefined,mark=`${stat.size}:${stat.mtimeNs}:${stat.ino}`;
 if(stamp){try{const line=readFileSync(stamp,"utf8"),at=line.indexOf(" ");if(at>0&&line.slice(0,at)===mark)return line.slice(at+1);}catch{}}
 const digest=sha256V1(readFileSync(path));
 if(stamp){try{ensure(dirname(stamp));const temporary=stamp+"."+randomBytes(3).toString("hex");writeFileSync(temporary,mark+" "+digest);rmSync(stamp,{force:true});renameSync(temporary,stamp);}catch{}}
 return digest;
}

/** 📤️ Appends one decision to the lane's event log; one small append is atomic across processes. */
export function emitUnitEventV1(root:string,event:Omit<UnitEventV1,"version"|"atMs"|"pid">):void{
 try{const directory=ensure(join(root,layout.events)),file=join(directory,event.lane+".jsonl");appendFileSync(file,JSON.stringify({version:1,atMs:Date.now(),pid:process.pid,...event})+"\n");}catch{}
}

/** 🔬️ The portable text behind the digests of one lookup, recorded only when `SEMIO_UNIT_CACHE_TRACE_KEYS` is set: the normalized arguments, the observed environment and the platform identity, so the keys of one crate on two machines can be diffed by value (see {@link traceDifferencesV1}). */
export type UnitTraceV1={args:string[];environment:Array<[string,string]>;platform?:string};

/** 🔬️ What the traced keys of one crate do not share: every argument and every environment entry that is absent from at least one key, with the number of keys it is in. */
export function traceDifferencesV1(rows:readonly Readonly<{trace?:UnitTraceV1}>[]):{keys:number;arguments:Array<{text:string;in:number}>;environment:Array<{text:string;in:number}>;platform:string[]}{
 const traced=rows.map(row=>row.trace).filter((trace):trace is UnitTraceV1=>trace!==undefined),count=(lists:string[][])=>{const found=new Map<string,number>();for(const list of lists)for(const text of new Set(list))found.set(text,(found.get(text)??0)+1);return[...found].filter(([,seen])=>seen<lists.length).map(([text,seen])=>({text,in:seen})).sort((left,right)=>left.text.localeCompare(right.text));};
 return{keys:traced.length,arguments:count(traced.map(trace=>trace.args)),environment:count(traced.map(trace=>trace.environment.map(([name,value])=>name+"="+value))),platform:[...new Set(traced.map(trace=>trace.platform).filter((text):text is string=>text!==undefined))]};
}

/** 🔎️ One unit lookup with the named components of its key (`Lookup` in the schema). */
export type UnitLookupV1={version:1;atMs:number;lane:string;pid:number;crate:string;key:string;remote?:string;outcome:"local-hit"|"fleet-hit"|"miss";slot?:string;components:Record<string,string>;values?:Record<string,string>;trace?:UnitTraceV1};

const logFiles=(root:string,name:string):[string,string]=>{const directory=join(root,layout.lookups);return[join(directory,name),join(directory,name.replace(/\.jsonl$/u,".1.jsonl"))];};
const lookupFiles=(root:string):[string,string]=>logFiles(root,policy.lookups.file);

function appendBounded(root:string,name:string,line:string):void{
 const [file,older]=logFiles(root,name);ensure(dirname(file));
 try{if(statSync(file).size>=policy.lookups.maximumFileBytes)renameSync(file,older);}catch{}
 appendFileSync(file,line+"\n");
}

/** ♻️ What a collection or a forget removed, by kind (`Eviction` in the schema): lanes, lane incremental sets, incremental sessions, stale build units, entries retired because they are stale against the synchronized tree, entries that lost an object, entries forgotten on request. */
export type EvictionCountsV1=Partial<Record<"lanes"|"incremental"|"sessions"|"units"|"retired"|"entries"|"forgotten",number>>;

/** ♻️ Appends one eviction record to the bounded eviction log beside the lookup log; all-zero counts are not written; `cause` names why (`lost`: an object vanished behind the entry, `capacity`: written by the host store, see the holders) and the role of the writer. Best effort, never throws.
 * @see ../🧬️schema/🔣️.json */
export function recordEvictionV1(root:string,counts:EvictionCountsV1,cause:Readonly<{reason?:"capacity"|"lost";role?:string}>={}):void{
 try{
  const kept=Object.fromEntries(Object.entries(counts).filter(([,value])=>Number.isSafeInteger(value)&&(value as number)>0));
  if(Object.keys(kept).length===0)return;
  appendBounded(root,policy.evictions.file,JSON.stringify({version:1,atMs:Date.now(),pid:process.pid,...cause.reason===undefined?{}:{reason:cause.reason},...cause.role===undefined?{}:{role:cause.role},...kept}));
 }catch{}
}

/** ♻️ The evictions recorded so far, summed by kind, with the time of the first and the last record. */
export function readEvictionsV1(root:string):{records:number;totals:Record<string,number>;firstMs:number|null;lastMs:number|null}{
 const totals:Record<string,number>={};let records=0,firstMs:number|null=null,lastMs:number|null=null;
 for(const file of[...logFiles(root,policy.evictions.file)].reverse()){
  let text:string;try{text=readFileSync(file,"utf8");}catch{continue;}
  for(const line of text.split("\n")){if(!line)continue;try{const row=JSON.parse(line) as Record<string,number>;if(row.version!==1)continue;records++;firstMs=firstMs===null?row.atMs!:Math.min(firstMs,row.atMs!);lastMs=lastMs===null?row.atMs!:Math.max(lastMs,row.atMs!);for(const [name,value] of Object.entries(row))if(!["version","atMs","pid"].includes(name)&&typeof value==="number")totals[name]=(totals[name]??0)+value;}catch{}}
 }
 return{records,totals,firstMs,lastMs};
}

/** 🔎️ Appends one lookup to the machine's bounded, rotating lookup log: when the file passes its bound it becomes the single older file and a new one starts, so two files of at most `maximumFileBytes` are all there is. Best effort, never throws.
 * @see ../🧬️schema/🔣️.json */
export function recordLookupV1(root:string,row:Omit<UnitLookupV1,"version"|"atMs"|"pid">):void{
 try{appendBounded(root,policy.lookups.file,JSON.stringify({version:1,atMs:Date.now(),pid:process.pid,...row}));}catch{}
}

/** 🔎️ Reads the recorded lookups, oldest first, optionally only those at or after `sinceMs`; a torn or foreign line is skipped. */
export function readLookupsV1(root:string,sinceMs=0):UnitLookupV1[]{
 const rows:UnitLookupV1[]=[];
 for(const file of[...lookupFiles(root)].reverse()){
  let text:string;try{text=readFileSync(file,"utf8");}catch{continue;}
  for(const line of text.split("\n")){if(!line)continue;try{const row=JSON.parse(line) as UnitLookupV1;if(row.version===1&&row.atMs>=sinceMs&&typeof row.crate==="string"&&typeof row.key==="string"&&row.components!==null&&typeof row.components==="object")rows.push(row);}catch{}}
 }
 return rows;
}

/** 🔀️ The churn of a window of lookups: how they ended, and for every crate that was looked up under more than one key, WHICH named components differ between its keys (with the texts seen for the small classes), plus the crates grouped by the set of differing components, most frequent first. A key is the fleet key when the lookup knew it, else the local key. */
export type ChurnReportV1={lookups:number;outcomes:Record<string,number>;crates:number;multiKey:number;causes:{components:string[];crates:number;examples:string[]}[];perCrate:{crate:string;keys:number;differs:string[];samples:Record<string,string[]>;trace?:ReturnType<typeof traceDifferencesV1>}[]};

export function churnReportV1(rows:readonly UnitLookupV1[],limit=50):ChurnReportV1{
 const outcomes:Record<string,number>={},byCrate=new Map<string,Map<string,UnitLookupV1>>();
 for(const row of rows){outcomes[row.outcome]=(outcomes[row.outcome]??0)+1;let keys=byCrate.get(row.crate);if(!keys){keys=new Map();byCrate.set(row.crate,keys);}keys.set(row.remote??row.key,row);}
 const perCrate:ChurnReportV1["perCrate"]=[],causes=new Map<string,{components:string[];crates:string[]}>();
 for(const [crate,keys] of byCrate){
  if(keys.size<2)continue;
  const list=[...keys.values()],differs=differingComponentsV1(list.map(row=>row.components)),samples:Record<string,string[]>={};
  for(const name of differs){const seen=[...new Set(list.map(row=>row.values?.[name]).filter((value):value is string=>value!==undefined))];if(seen.length>0)samples[name]=seen.slice(0,4);}
  const trace=traceDifferencesV1(list);perCrate.push({crate,keys:keys.size,differs,samples,...trace.keys>1?{trace}:{}});
  const id=differs.join(","),cause=causes.get(id)??{components:differs,crates:[]};cause.crates.push(crate);causes.set(id,cause);
 }
 perCrate.sort((left,right)=>right.keys-left.keys||left.crate.localeCompare(right.crate));
 return{lookups:rows.length,outcomes,crates:byCrate.size,multiKey:perCrate.length,causes:[...causes.values()].map(row=>({components:row.components,crates:row.crates.length,examples:row.crates.sort().slice(0,3)})).sort((left,right)=>right.crates-left.crates||left.components.join().localeCompare(right.components.join())),perCrate:perCrate.slice(0,limit)};
}

const putObjectCas=(_root:string,bytes:Uint8Array|string):string=>putCasObjectSyncV1(casRootV1(),bytes);
const putObject=putObjectCas;

const readObject=(_root:string,sha:string):Buffer|undefined=>readCasObjectSyncV1(casRootV1(),sha);

const spillDirectory=(root:string)=>join(root,layout.temporary,"spill");
/** 💧️ Keeps one object of a unit that could not enter the host store (the write guard of a nearly full disk refused it) in a spill folder of the unit cache until the fleet upload has read it; returns its sha256. */
export function putSpillObjectV1(root:string,bytes:Uint8Array|string):string{
 const data=typeof bytes==="string"?Buffer.from(bytes):Buffer.from(bytes),sha=createHash("sha256").update(data).digest("hex"),directory=ensure(spillDirectory(root)),temporary=join(directory,sha+"."+randomBytes(4).toString("hex")+".tmp");
 writeFileSync(temporary,data);renameSync(temporary,join(directory,sha));return sha;
}
/** 💧️ Reads a spilled object, `undefined` when absent or when its bytes do not hash to its name. */
export function readSpillObjectV1(root:string,sha:string):Buffer|undefined{
 try{const path=join(spillDirectory(root),sha),data=readFileSync(path);if(createHash("sha256").update(data).digest("hex")!==sha)return undefined;touchSpillObjectV1(path);return data;}catch{return undefined;}
}
/** 🧽️ Bounds the spill folder of a cache root to what its volume can spare, least recently used entries first, keeping whatever a build or upload used lately ({@link pruneSpillV1}); `incomingBytes` is what the caller is about to write. */
export function pruneSpillObjectsV1(root:string,options:Readonly<{nowMs?:number;incomingBytes?:number;boundBytes?:number}>={}):SpillPruneV1{return pruneSpillV1(spillDirectory(root),options);}
const spilledEntryPath=(root:string,key:string)=>join(spillDirectory(root),"entries",key+".json");
/** 💧️ Removes the spilled objects of an entry, and its spilled entry file, once its upload is over, whatever the outcome. */
export function dropSpillObjectsV1(root:string,entry:Readonly<{key?:string;outputs:readonly{sha256:string}[];stdout?:string|null;stderr?:string|null}>):void{
 for(const sha of[...entry.outputs.map(row=>row.sha256),entry.stdout,entry.stderr])if(typeof sha==="string")rmSync(join(spillDirectory(root),sha),{force:true});
 if(entry.key!==undefined)rmSync(spilledEntryPath(root,entry.key),{force:true});
}
/** 🕰️ Marks the spilled entry of a waiting upload and its objects as read now, so the bound of the spill folder keeps a unit that waits longer than `protectMilliseconds` for the fleet cache (a refused or busy server) instead of evicting it before it was uploaded. */
export function touchSpilledEntryV1(root:string,key:string):boolean{
 const entry=readSpilledEntryV1(root,key);if(!entry)return false;
 touchSpillObjectV1(spilledEntryPath(root,key));
 for(const sha of[...entry.outputs.map(row=>row.sha256),entry.stdout,entry.stderr])if(typeof sha==="string")touchSpillObjectV1(join(spillDirectory(root),sha));
 return true;
}
/** 💧️ The entry of a unit whose objects wait in the spill folder (written beside them, outside the entry index the host store's holders read: its objects are not in the host store, so a collector would take it for dead and a lookup can never serve it); `undefined` when there is none. */
export function readSpilledEntryV1(root:string,key:string):UnitEntryV1|undefined{
 try{const entry=JSON.parse(readFileSync(spilledEntryPath(root,key),"utf8")) as UnitEntryV1;return entry.version===1&&entry.key===key?entry:undefined;}catch{return undefined;}
}

/** 🔎️ Returns the newest entry of `key` whose recorded files and environment still hold, `undefined` on a miss. A missing object, an unreadable entry or an entry that lacks one of the `expected` output names (a hit must leave every file a dependant names) is a miss, never an error. */
export function lookupUnitV1(root:string,key:string,context:Readonly<{cwd:string;laneRoot:string|undefined;environment:Readonly<Record<string,string|undefined>>;stampDirectory:string|undefined;expected?:readonly string[]}>):UnitEntryV1|undefined{
 const directory=entryDirectory(root,key);let names:string[];
 try{names=readdirSync(directory).filter(name=>name.endsWith(".json")).sort().reverse();}catch{return undefined;}
 for(const name of names.slice(0,policy.key.maximumVariants)){
  let entry:UnitEntryV1;try{entry=JSON.parse(readFileSync(join(directory,name),"utf8"));}catch{continue;}
  if(entry.version!==1||entry.key!==key)continue;
  if(context.expected&&uncoveredOutputsV1(entry.outputs.map(row=>row.name),context.expected).length>0)continue;
  if(entry.dependencies.some(row=>digestFileV1(resolveStored(context.cwd,generatedRestoreV1(laneRestoreV1(row.path,context.laneRoot),context.environment["OUT_DIR"])),context.stampDirectory)!==row.sha256))continue;
  if(entry.environment.some(row=>{const value=context.environment[row.name];return(value===undefined?null:laneNeutralV1(value,context.laneRoot))!==row.value;}))continue;
  if(lostObjectV1(root,entry))continue;
  holdHostObjectsV1(root);keepHostObjectsV1(root,entry);
  if(entry.slot!==undefined)claimUnitV1(root,key);
  return entry;
 }
 return undefined;
}

function resolveStored(cwd:string,path:string):string{return isAbsolute(path)?path:resolve(cwd,path);}

/** 🧷️ Puts a file in place under its final name in one step (written aside, renamed over), so a compiler that reads the output directory while a cached unit is restored sees the old file or the new one and never an absent name. */
function placeV1(destination:string,write:(temporary:string)=>void):void{
 const temporary=destination+"."+randomBytes(4).toString("hex")+".tmp";
 try{write(temporary);renameSync(temporary,destination);}catch(error){rmSync(temporary,{force:true});throw error;}
}

/** ♻️ Writes a cached compilation into `outDir` as the compiler would have: output files by reflink where the filesystem supports it (a copy elsewhere, never a shared inode) stamped with the restore time as a compile writes them (a reflink or Windows copy keeps the cache object's last-use time, which Cargo reads as an output older than a dependency's and re-runs the unit on every later build; see https://doc.rust-lang.org/nightly/nightly-rustc/cargo/core/compiler/fingerprint/index.html), the dependency file rewritten for this lane, and the compiler's messages returned for replay. */
export function restoreUnitV1(root:string,entry:UnitEntryV1,context:Readonly<{outDir:string;laneRoot:string|undefined;stampDirectory:string|undefined;extra?:string;generated?:string}>):{stdout:string;stderr:string}{
 const extra=context.extra??"",render=(text:string)=>extraRestoreV1(laneRestoreV1(generatedRestoreV1(text.split(policy.key.outputToken).join(context.outDir),context.generated),context.laneRoot),extra);
 ensure(context.outDir);
 const staged:Array<{row:UnitEntryV1["outputs"][number];destination:string;temporary:string}>=[];
 try{
  for(const row of entry.outputs){
   const destination=join(context.outDir,extraRestoreV1(row.name,extra)),temporary=destination+"."+randomBytes(4).toString("hex")+".tmp",source=objectPath(row.sha256);
   staged.push({row,destination,temporary});
   if(row.template)writeFileSync(temporary,render(readFileSync(source,"utf8")));
   else{copyFileSync(source,temporary,constants.COPYFILE_FICLONE);chmodSync(temporary,row.executable?0o755:0o644);}
  }
 }catch(error){for(const item of staged)rmSync(item.temporary,{force:true});throw error;}
 for(const {row,destination,temporary} of staged){
  renameSync(temporary,destination);
  touchCasObjectSyncV1(casRootV1(),row.sha256);const now=new Date();
  if(!row.template)utimesSync(destination,now,now);
  if(!row.template&&context.stampDirectory)stampFile(destination,row.sha256,context.stampDirectory);
 }
 const text=(sha:string|null)=>sha===null?"":render(readObject(root,sha)?.toString("utf8")??"");
 return{stdout:text(entry.stdout),stderr:text(entry.stderr)};
}

/** 🧬️ The text with the generated-source directory of the unit's build script (`OUT_DIR`) replaced by its token: Cargo's `<hash>` in `build/<package>/<hash>/out` differs between machines, and the dependency file names it. */
export const generatedNeutralV1=(text:string,generated:string|undefined):string=>generated===undefined||generated===""?text:text.split(generated).join(policy.key.generatedToken);

/** 🧬️ The inverse of {@link generatedNeutralV1} on the machine that restores the unit. */
export const generatedRestoreV1=(text:string,generated:string|undefined):string=>generated===undefined||generated===""?text:text.split(policy.key.generatedToken).join(generated);

function stampFile(path:string,sha:string,stampDirectory:string):void{
 try{const stat=statSync(path,{bigint:true}),hash=sha256V1(path),stamp=join(stampDirectory,hash.slice(0,2),hash.slice(2,34));ensure(dirname(stamp));const temporary=stamp+"."+randomBytes(3).toString("hex");writeFileSync(temporary,`${stat.size}:${stat.mtimeNs}:${stat.ino} ${sha}`);rmSync(stamp,{force:true});renameSync(temporary,stamp);}catch{}
}

/** 📥️ Publishes one successful compilation. Objects are content-addressed and the entry is renamed into place last, so a reader sees all of it or none of it. Returns the digests, `undefined` when the dependency file is missing, an `expected` output is absent (an entry that cannot serve its dependants is never stored) or the outputs exceed the bound. */
export function publishUnitV1(root:string,input:Readonly<{key:string;crate:string;cwd:string;outDir:string;generated?:string;laneRoot:string|undefined;outputs:readonly string[];stdout:string;stderr:string;stampDirectory:string|undefined;provenance?:UnitEntryV1["provenance"];slot?:string;retention?:Readonly<{workspace:string|undefined;externs:readonly string[];units?:readonly string[];toolchain:string}>;spill?:boolean;session?:boolean;expected?:readonly string[];extra?:string}>):UnitEntryV1|undefined{
 const putObject=input.spill===true?putSpillObjectV1:putObjectCas;
 if(input.expected&&uncoveredOutputsV1(input.outputs.map(path=>basename(path)),input.expected).length>0)return undefined;
 const depInfoPath=input.outputs.find(path=>path.endsWith(".d"));if(!depInfoPath||!existsSync(depInfoPath))return undefined;
 const info=parseDepInfoV1(readFileSync(depInfoPath,"utf8"));
 if(info.dependencies.length>policy.key.maximumDependencyFiles)return undefined;
 const extra=input.extra??"",neutral=(text:string)=>extraNeutralV1(laneNeutralV1(generatedNeutralV1(text.split(input.outDir).join(policy.key.outputToken),input.generated),input.laneRoot),extra);
 let total=0;const outputs:UnitEntryV1["outputs"]=[];
 if(input.spill===true){let incoming=0;for(const path of input.outputs)try{incoming+=statSync(path).size;}catch{return undefined;}if(incoming>pruneSpillObjectsV1(root,{incomingBytes:incoming}).boundBytes)return undefined;}
 for(const path of input.outputs){
  const stat=statSync(path);total+=stat.size;if(total>policy.key.maximumOutputBytes)return undefined;
  const template=path===depInfoPath,bytes=template?Buffer.from(neutral(readFileSync(path,"utf8"))):readFileSync(path),sha=putObject(root,bytes);
  outputs.push({name:extraNeutralV1(basename(path),extra),sha256:sha,bytes:bytes.length,executable:(stat.mode&0o111)!==0&&!template,...(template?{template:true}:{})});
  if(!template&&input.stampDirectory&&input.spill!==true)stampFile(path,sha,input.stampDirectory);
 }
 const dependencies=info.dependencies.filter(path=>!compilerWrittenV1(path)).map(path=>({path:laneNeutralV1(generatedNeutralV1(path,input.generated),input.laneRoot),sha256:digestFileV1(resolveStored(input.cwd,path),input.stampDirectory)}));
 const retention=input.retention===undefined?undefined:declareV1({kind:"rust-unit",toolchain:input.retention.toolchain,sources:workspaceRowsV1({dependencies,cwd:input.cwd,workspace:input.retention.workspace,generatedRoots:buildPolicy.remoteCache.server.staleness.generatedRoots}),externs:input.retention.externs,self:input.key,units:input.retention.units});
 const entry:UnitEntryV1={version:1,key:input.key,...(input.slot===undefined?{}:{slot:input.slot}),...(input.session===true?{session:true as const}:{}),...(retention===undefined?{}:{retention}),crate:input.crate,dependencies,environment:info.environment.map(row=>({name:row.name,value:row.value===null?null:laneNeutralV1(row.value,input.laneRoot)})),outputs,stderr:input.stderr?putObject(root,neutral(input.stderr)):null,stdout:input.stdout?putObject(root,neutral(input.stdout)):null,createdMs:Date.now(),provenance:input.provenance??null};
 if(input.spill===true){const path=spilledEntryPath(root,entry.key),temporary=path+"."+randomBytes(4).toString("hex")+".tmp";ensure(dirname(path));writeFileSync(temporary,JSON.stringify(entry));renameSync(temporary,path);}
 else writeEntryV1(root,entry);
 return entry;
}

/** 📝️ Writes one entry as the newest variant of its key (written aside and renamed into place, the oldest variants beyond the bound dropped); shared by a compile that publishes and by a unit installed from the fleet cache. */
export function writeEntryV1(root:string,entry:UnitEntryV1):void{
 holdHostObjectsV1(root);
 const directory=ensure(entryDirectory(root,entry.key)),temporary=join(ensure(join(root,layout.temporary)),randomBytes(8).toString("hex")),name=String(entry.createdMs).padStart(15,"0")+"-"+randomBytes(3).toString("hex")+".json";
 writeFileSync(temporary,JSON.stringify(entry));renameSync(temporary,join(directory,name));
 keepHostObjectsV1(root,entry);
 try{const names=readdirSync(directory).filter(file=>file.endsWith(".json")).sort();for(const old of names.slice(0,Math.max(0,names.length-policy.key.maximumVariants)))rmSync(join(directory,old),{force:true});}catch{}
}

/** 📖️ The newest recorded variant of `key` as written, without checking anything; `undefined` when there is none. */
export function newestEntryV1(root:string,key:string):UnitEntryV1|undefined{
 const directory=entryDirectory(root,key);let names:string[];
 try{names=readdirSync(directory).filter(name=>name.endsWith(".json")).sort().reverse();}catch{return undefined;}
 for(const name of names){try{const entry=JSON.parse(readFileSync(join(directory,name),"utf8")) as UnitEntryV1;if(entry.version===1&&entry.key===key)return entry;}catch{}}
 return undefined;
}

/** 🫥️ Forgets the local entries (compiled units and build script runs) of one crate, by the crate name the entries record, and nothing else: the objects stay in the host store for other entries, the fleet copy stays where it is. Returns the number of entry files removed. */
export function forgetCrateEntriesV1(root:string,crate:string):{entries:number}{
 if(!crate||crate==="-")throw Error("Crate name required");
 let entries=0;
 for(const kind of[layout.entries,layout.runs]){
  const base=join(root,kind);let shards:string[];try{shards=readdirSync(base);}catch{continue;}
  for(const shard of shards){let keys:string[];try{keys=readdirSync(join(base,shard));}catch{continue;}
   for(const key of keys){
    const directory=join(base,shard,key);let files:string[];try{files=readdirSync(directory);}catch{continue;}
    for(const file of files){try{if((JSON.parse(readFileSync(join(directory,file),"utf8")) as {crate?:string}).crate===crate){rmSync(join(directory,file),{force:true});entries++;}}catch{}}
    try{if(readdirSync(directory).length===0)rmSync(directory,{recursive:true,force:true});}catch{}
   }
  }
 }
 recordEvictionV1(root,{forgotten:entries});
 return{entries};
}

/** 🧽️ Drops entries that lost an object (host collected it behind the entry, or removed by hand; a capacity eviction removes the entry with its objects and leaves none), empty entry directories and temporary files of writers that died an hour ago. */
export function pruneEntriesV1(root:string,nowMs:number):{entries:number;temporary:number}{
 let entries=0,temporary=0;
 for(const kind of[layout.entries,layout.runs]){
  const base=join(root,kind);let shards:string[];try{shards=readdirSync(base);}catch{shards=[];}
  for(const shard of shards)for(const key of readdirSync(join(base,shard))){
   const directory=join(base,shard,key);let files:string[];try{files=readdirSync(directory);}catch{continue;}
   for(const file of files){try{const entry=JSON.parse(readFileSync(join(directory,file),"utf8")) as {outputs:{sha256:string}[];stdout?:string|null;stderr?:string|null};if(lostObjectV1(root,entry)){rmSync(join(directory,file),{force:true});entries++;}}catch{rmSync(join(directory,file),{force:true});entries++;}}
   try{if(readdirSync(directory).length===0)rmSync(directory,{recursive:true,force:true});}catch{}
  }
 }
 const work=join(root,layout.temporary);let names:string[];try{names=readdirSync(work);}catch{names=[];}
 for(const name of names){try{if(nowMs-statSync(join(work,name)).mtimeMs>3600000){rmSync(join(work,name),{force:true});temporary++;}}catch{}}
 recordEvictionV1(root,{entries},{reason:"lost",role:"prune"});
 return{entries,temporary};
}

/** 🌳️ Digest of a directory tree for build-script facts: sorted relative paths with file digests (links by target text). `undefined` beyond the file bound. */
export function treeDigestV1(root:string,options:Readonly<{ignoredDirectories:readonly string[];ignoredExtensions:readonly string[];maximumFiles:number;stampDirectory:string|undefined}>):string|undefined{
 const rows:string[][]=[];let overflow=false;
 const walk=(directory:string):void=>{
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  for(const entry of entries.sort((left,right)=>left.name<right.name?-1:1)){
   if(overflow)return;
   const path=join(directory,entry.name),name=relative(root,path).split(sep).join("/");
   if(entry.isDirectory()){if(!options.ignoredDirectories.includes(entry.name))walk(path);continue;}
   if(options.ignoredExtensions.some(extension=>entry.name.endsWith(extension)))continue;
   if(rows.length>=options.maximumFiles){overflow=true;return;}
   if(entry.isSymbolicLink()){try{rows.push([name,"link:"+readlinkSync(path)]);}catch{rows.push([name,"link:"]);}continue;}
   if(entry.isFile())rows.push([name,digestFileV1(path,options.stampDirectory)??"unreadable"]);
  }
 };
 walk(root);
 return overflow?undefined:sha256V1(JSON.stringify(rows));
}

/** 📋️ The facts a build script run depends on, recomputed from the disk: every declared path (a missing one means Cargo reruns always, so nothing is cacheable), the package tree, and the declared environment variables. */
export function buildFactsV1(input:Readonly<{cwd:string;directives:Readonly<{files:readonly string[];variables:readonly string[]}>;laneRoot:string|undefined;environment:Readonly<Record<string,string|undefined>>;stampDirectory:string|undefined}>):{facts:{path:string;kind:"file"|"tree"|"package";sha256:string}[];environment:{name:string;value:string|null}[]}|undefined{
 const facts:{path:string;kind:"file"|"tree"|"package";sha256:string}[]=[],build=policy.buildScript,declared=input.directives.files.length+input.directives.variables.length>0;
 for(const path of [...new Set(input.directives.files)].sort()){
  const absolute=resolve(input.cwd,path);let stat;try{stat=lstatSync(absolute);}catch{return undefined;}
  if(stat.isDirectory()){const sha=treeDigestV1(absolute,{ignoredDirectories:[],ignoredExtensions:[],maximumFiles:build.maximumPackageFiles,stampDirectory:input.stampDirectory});if(sha===undefined)return undefined;facts.push({path:laneNeutralV1(path,input.laneRoot),kind:"tree",sha256:sha});}
  else{const sha=digestFileV1(absolute,input.stampDirectory);if(sha===null)return undefined;facts.push({path:laneNeutralV1(path,input.laneRoot),kind:"file",sha256:sha});}
 }
 const root=input.environment.CARGO_MANIFEST_DIR??input.cwd,sha=treeDigestV1(root,{ignoredDirectories:build.ignoredDirectories,ignoredExtensions:declared?build.undeclaredIgnoredExtensions:[],maximumFiles:build.maximumPackageFiles,stampDirectory:input.stampDirectory});
 if(sha===undefined)return undefined;
 facts.push({path:".",kind:"package",sha256:sha});
 const environment=[...new Set(input.directives.variables)].sort().map(name=>({name,value:input.environment[name]===undefined?null:laneNeutralV1(input.environment[name]!,input.laneRoot)}));
 return{facts,environment};
}

/** 🔨️ One cached build script run: the facts it depends on, its OUT_DIR tree, and the two output streams (lane-neutral). */
export type BuildEntryV1={version:1;key:string;retention?:DeclarationV1;package:string;facts:{path:string;kind:"file"|"tree"|"package";sha256:string}[];environment:{name:string;value:string|null}[];outputs:{path:string;sha256:string;bytes:number;executable:boolean;template:boolean}[];stdout:string|null;stderr:string|null;createdMs:number};
const runDirectory=(root:string,key:string)=>join(root,layout.runs,key.slice(0,2),key);

/** ▶️ Returns the newest recorded run of `key` whose facts and environment still hold. */
export function lookupBuildRunV1(root:string,key:string,context:Readonly<{cwd:string;laneRoot:string|undefined;environment:Readonly<Record<string,string|undefined>>;stampDirectory:string|undefined}>):BuildEntryV1|undefined{
 const directory=runDirectory(root,key);let names:string[];
 try{names=readdirSync(directory).filter(name=>name.endsWith(".json")).sort().reverse();}catch{return undefined;}
 for(const name of names.slice(0,policy.key.maximumVariants)){
  let entry:BuildEntryV1;try{entry=JSON.parse(readFileSync(join(directory,name),"utf8"));}catch{continue;}
  if(entry.version!==1||entry.key!==key)continue;
  if(lostObjectV1(root,entry))continue;
  holdHostObjectsV1(root);keepHostObjectsV1(root,entry);
  const directives={files:entry.facts.filter(row=>row.kind!=="package").map(row=>laneRestoreV1(row.path,context.laneRoot)),variables:entry.environment.map(row=>row.name)};
  const now=buildFactsV1({cwd:context.cwd,directives,laneRoot:context.laneRoot,environment:context.environment,stampDirectory:context.stampDirectory});
  if(!now)continue;
  const packageFact=entry.facts.find(row=>row.kind==="package"),nowPackage=now.facts.find(row=>row.kind==="package");
  if(!packageFact||packageFact.sha256!==nowPackage?.sha256)continue;
  const same=(left:{path:string;sha256:string},right:{path:string;sha256:string})=>left.path===right.path&&left.sha256===right.sha256;
  const recorded=entry.facts.filter(row=>row.kind!=="package"),current=now.facts.filter(row=>row.kind!=="package");
  if(recorded.length!==current.length||!recorded.every((row,index)=>same(row,current[index]!)&&row.kind===current[index]!.kind))continue;
  if(entry.environment.length!==now.environment.length||!entry.environment.every((row,index)=>row.name===now.environment[index]!.name&&row.value===now.environment[index]!.value))continue;
  return entry;
 }
 return undefined;
}

function isText(bytes:Buffer):boolean{if(bytes.includes(0))return false;try{new TextDecoder("utf-8",{fatal:true}).decode(bytes);return true;}catch{return false;}}

/** 📢️ Publishes one successful build script run. `undefined` when the run is not cacheable: a declared path is missing, the OUT_DIR holds a link, is too large, or a binary file embeds the lane path. */
export function publishBuildRunV1(root:string,input:Readonly<{key:string;package:string;cwd:string;laneRoot:string|undefined;directives:Readonly<{files:readonly string[];variables:readonly string[]}>;environment:Readonly<Record<string,string|undefined>>;outDir:string;stdout:string;stderr:string;stampDirectory:string|undefined;retention?:DeclarationV1}>):BuildEntryV1|undefined{
 const known=buildFactsV1({cwd:input.cwd,directives:input.directives,laneRoot:input.laneRoot,environment:input.environment,stampDirectory:input.stampDirectory});if(!known)return undefined;
 const outputs:BuildEntryV1["outputs"]=[],dependencies=buildDependencyPathsV1(input.environment);let total=0,refused=false;
 const walk=(directory:string):void=>{
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  for(const entry of entries.sort((left,right)=>left.name<right.name?-1:1)){
   const path=join(directory,entry.name),name=relative(input.outDir,path).split(sep).join("/");
   if(entry.isDirectory()){walk(path);continue;}
   if(!entry.isFile()){refused=true;return;}
   const stat=statSync(path);total+=stat.size;if(total>policy.buildScript.maximumOutputBytes){refused=true;return;}
   const bytes=readFileSync(path),text=isText(bytes),neutral=text?buildRunNeutralV1(bytes.toString("utf8"),input.outDir,input.laneRoot,dependencies):undefined;
   if(!text&&[input.outDir,...(input.laneRoot?[input.laneRoot]:[])].some(own=>bytes.includes(Buffer.from(own)))){refused=true;return;}
   const template=neutral!==undefined&&neutral!==bytes.toString("utf8"),stored=template?Buffer.from(neutral!):bytes;
   outputs.push({path:name,sha256:putObject(root,stored),bytes:stored.length,executable:(stat.mode&0o111)!==0,template});
  }
 };
 walk(input.outDir);
 if(refused)return undefined;
 const neutral=(text:string)=>buildRunNeutralV1(text,input.outDir,input.laneRoot,dependencies);
 if(input.laneRoot&&[input.stdout,input.stderr].some(stream=>foreignLanePathV1(neutral(stream),input.laneRoot!)))return undefined;
 const entry:BuildEntryV1={version:1,key:input.key,...(input.retention===undefined?{}:{retention:input.retention}),package:input.package,facts:known.facts,environment:known.environment,outputs,stdout:input.stdout?putObject(root,neutral(input.stdout)):null,stderr:input.stderr?putObject(root,neutral(input.stderr)):null,createdMs:Date.now()};
 writeBuildRunV1(root,entry);
 return entry;
}

/** 📝️ Writes one build script run as the newest variant of its key (written aside and renamed into place, the oldest variants beyond the bound dropped); shared by a run that publishes and by a run installed from the fleet cache. */
export function writeBuildRunV1(root:string,entry:BuildEntryV1):void{
 holdHostObjectsV1(root);
 const directory=ensure(runDirectory(root,entry.key)),temporary=join(ensure(join(root,layout.temporary)),randomBytes(8).toString("hex")),name=String(entry.createdMs).padStart(15,"0")+"-"+randomBytes(3).toString("hex")+".json";
 writeFileSync(temporary,JSON.stringify(entry));renameSync(temporary,join(directory,name));
 keepHostObjectsV1(root,entry);
 try{const names=readdirSync(directory).filter(file=>file.endsWith(".json")).sort();for(const old of names.slice(0,Math.max(0,names.length-policy.key.maximumVariants)))rmSync(join(directory,old),{force:true});}catch{}
}

/** 📖️ The newest recorded run of `key` as written, without checking anything; `undefined` when there is none. */
export function newestBuildRunV1(root:string,key:string):BuildEntryV1|undefined{
 const directory=runDirectory(root,key);let names:string[];
 try{names=readdirSync(directory).filter(name=>name.endsWith(".json")).sort().reverse();}catch{return undefined;}
 for(const name of names){try{const entry=JSON.parse(readFileSync(join(directory,name),"utf8")) as BuildEntryV1;if(entry.version===1&&entry.key===key)return entry;}catch{}}
 return undefined;
}

/** 🧭️ The text of a build script run without the paths of the lane that ran it: the script's own OUT_DIR (which its `cargo:rustc-link-search` and generated sources name) becomes the output token, then the lane root becomes the lane token, so a replay in any lane names that lane's own OUT_DIR and never the directory of a lane that a collector may remove. Neutralizing the lane root alone is not enough: it is only the lane when the caller names the lane, and a build driver names its run directory. */
export function buildRunNeutralV1(text:string,outDir:string,laneRoot:string|undefined,dependencies:readonly (readonly [string,string])[]=[]):string{
 let result=text.split(outDir).join(policy.key.outputToken);
 for(const [token,path] of dependencies)result=result.split(path).join(token);
 return laneNeutralV1(result,laneRoot);
}

/** ⏮️ The inverse of {@link buildRunNeutralV1} in the lane that replays the run. */
export function buildRunRestoreV1(text:string,outDir:string,laneRoot:string|undefined,dependencies:readonly (readonly [string,string])[]=[]):string{
 let result=laneRestoreV1(text,laneRoot).split(policy.key.outputToken).join(outDir);
 for(const [token,path] of dependencies)result=result.split(token).join(path);
 return result;
}

/** 🚧️ Whether a text still names the directory of another lane of the same pool once this lane's root is neutralized, which a replay in a third lane could never resolve. */
export function foreignLanePathV1(neutralText:string,laneRoot:string):boolean{
 const siblings=dirname(laneRoot)+sep;
 return new RegExp(siblings.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")+"[a-z0-9]+","u").test(neutralText);
}

/** ⏪️ Writes a cached run's OUT_DIR tree, and returns the streams to replay, as the script would have produced them; throws (the caller runs the script) when a recorded file is not in place afterwards. */
export function restoreBuildRunV1(root:string,entry:BuildEntryV1,context:Readonly<{outDir:string;laneRoot:string|undefined;environment?:Readonly<Record<string,string|undefined>>}>):{stdout:string;stderr:string}{
 ensure(context.outDir);const dependencies=buildDependencyPathsV1(context.environment??{});
 for(const row of entry.outputs){
  const destination=join(context.outDir,row.path),source=objectPath(row.sha256);ensure(dirname(destination));
  if(row.template){const text=buildRunRestoreV1(readFileSync(source,"utf8"),context.outDir,context.laneRoot,dependencies);placeV1(destination,temporary=>writeFileSync(temporary,text));}else placeV1(destination,temporary=>{copyFileSync(source,temporary,constants.COPYFILE_FICLONE);chmodSync(temporary,0o644);});
  if(row.executable)chmodSync(destination,0o755);
  touchCasObjectSyncV1(casRootV1(),row.sha256);
 }
 for(const row of entry.outputs)if(!lstatSync(join(context.outDir,row.path)).isFile())throw Error("Restored OUT_DIR file absent: "+row.path);
 const text=(sha:string|null)=>sha===null?"":buildRunRestoreV1(readObject(root,sha)?.toString("utf8")??"",context.outDir,context.laneRoot,dependencies);
 const streams={stdout:text(entry.stdout),stderr:text(entry.stderr)},foreign=context.laneRoot===undefined?undefined:[streams.stdout,streams.stderr].find(stream=>foreignLanePathV1(buildRunNeutralV1(stream,context.outDir,context.laneRoot,dependencies),context.laneRoot!)),missing=missingLinkSearchPathV1(streams.stdout);
 if(foreign!==undefined)throw Error("Replayed output names the directory of another lane");
 if(missing!==undefined)throw Error("Replayed link search path absent: "+missing);
 return streams;
}

/** 🧭️ The first absolute `cargo:rustc-link-search` directory a build script's output names that does not exist, so a replay that would send the linker to a path of a lane that is gone is a miss instead; `undefined` when every one exists. */
export function missingLinkSearchPathV1(stdout:string):string|undefined{
 for(const line of stdout.split(/\r?\n/u)){
  const match=/^cargo::?rustc-link-search=(?:[a-z]+=)?(.+)$/u.exec(line);
  if(match&&isAbsolute(match[1]!)&&!existsSync(match[1]!))return match[1];
 }
 return undefined;
}
