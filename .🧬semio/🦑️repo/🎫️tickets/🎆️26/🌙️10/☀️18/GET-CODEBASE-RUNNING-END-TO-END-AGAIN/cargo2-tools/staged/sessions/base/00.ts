import {createHash,randomBytes} from "node:crypto";
import {closeSync,mkdirSync,openSync,readFileSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeFileSync,writeSync} from "node:fs";
import {join} from "node:path";
import {processAliveV1,tryClaimV1} from "../🔒️lock/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1,layout=policy.layout,idPattern=/^[a-z0-9]{1,12}$/u;
/** 🪪️ Who a lane belongs to: the agent session address and the agent, recorded in the lane's metadata so caches and cleanup can be attributed. */
export type LaneOwnerV1=Readonly<{session:string;agent:string}>;
/** 🗺️ The directories of one lane inside a cache root; compact on purpose so Windows paths stay short. */
export function lanePathsV1(root:string,id:string){
 if(!idPattern.test(id))throw Error("Invalid Cargo lane identifier");
 const base=join(root,layout.lanes,id);
 return Object.freeze({id,base,target:join(base,layout.target),build:join(base,layout.build),live:join(base,layout.live),stamps:join(base,layout.stamps),metadata:join(base,layout.metadata),size:join(base,layout.size)});
}

/** 🔑️ The affinity key of a crate set: the same sorted, de-duplicated names always give the same key, an empty set (a whole-workspace command) gives `*`. */
export function crateKeyV1(crates:readonly string[]):string{
 const names=[...new Set(crates)].sort();return names.length===0?"*":createHash("sha256").update(JSON.stringify(names)).digest("hex").slice(0,12);
}

/** 📖️ A lane's metadata file; `crateKey` is the affinity key of the crate set the lane's incremental state was last built for. */
export type LaneRecordV1=Readonly<{version:1;id:string;createdMs:number;session?:string;agent?:string;crateKey?:string;crates?:readonly string[]}>;

/** 📖️ Reads a lane's metadata file. */
export function readLaneV1(root:string,id:string):LaneRecordV1|undefined{try{return JSON.parse(readFileSync(lanePathsV1(root,id).metadata,"utf8"));}catch{return undefined;}}

/** 📏️ The size last measured for a lane by the collector; `undefined` until it was measured. */
export function readLaneSizeV1(root:string,id:string):Readonly<{bytes:number;measuredMs:number}>|undefined{
 try{const row=JSON.parse(readFileSync(lanePathsV1(root,id).size,"utf8"));return Number.isSafeInteger(row?.bytes)&&Number.isSafeInteger(row?.measuredMs)?{bytes:row.bytes,measuredMs:row.measuredMs}:undefined;}catch{return undefined;}
}

const writeAtomicV1=(path:string,text:string):void=>{const temporary=path+"."+randomBytes(3).toString("hex");writeFileSync(temporary,text);renameSync(temporary,path);};

/** 🧮️ Records the measured size of a lane next to its metadata (a file of its own, so a measurement never overwrites a concurrent acquisition's record). */
export function recordLaneSizeV1(root:string,id:string,bytes:number,nowMs:number):void{try{writeAtomicV1(lanePathsV1(root,id).size,JSON.stringify({bytes,measuredMs:nowMs}));}catch{}}

/** 🚪️ Registers one running command in its lane and records who uses it for which crate set. The marker is kept fresh by a heartbeat and removed on release; garbage collection never touches a lane with a fresh marker. */
export function enterLaneV1(root:string,id:string,owner?:LaneOwnerV1,crates?:readonly string[]):Readonly<{paths:ReturnType<typeof lanePathsV1>;release():void}>{
 const paths=lanePathsV1(root,id);
 for(const directory of[paths.target,paths.build,paths.live,paths.stamps])mkdirSync(directory,{recursive:true});
 const before=readLaneV1(root,id),names=crates===undefined?before?.crates:[...new Set(crates)].sort(),who=owner??(before?.session===undefined?undefined:{session:before.session,agent:before.agent!});
 writeAtomicV1(paths.metadata,JSON.stringify({version:1,id,createdMs:before?.createdMs??Date.now(),...who===undefined?{}:{session:who.session,agent:who.agent},...names===undefined?{}:{crateKey:crateKeyV1(names),crates:names.slice(0,policy.pool.listedCrates)}}));
 const touch=()=>{try{const now=new Date();utimesSync(paths.metadata,now,now);}catch{}};
 const marker=join(paths.live,process.pid+"-"+randomBytes(3).toString("hex")),write=()=>{const fd=openSync(marker,"w");try{writeSync(fd,JSON.stringify({pid:process.pid}));}finally{closeSync(fd);}};write();
 const beat=setInterval(()=>{try{const now=new Date();utimesSync(marker,now,now);}catch{}},policy.slot.heartbeatMilliseconds);beat.unref?.();
 let released=false;
 return Object.freeze({paths,release(){if(released)return;released=true;clearInterval(beat);rmSync(marker,{force:true});touch();}});
}

/** 🫀️ Whether any marker of the lane belongs to a live, recently heartbeating process. */
export function laneLiveV1(paths:ReturnType<typeof lanePathsV1>,nowMs:number):boolean{
 let names:string[];try{names=readdirSync(paths.live);}catch{return false;}
 for(const name of names){
  try{const age=nowMs-statSync(join(paths.live,name)).mtimeMs,pid=Number(name.split("-")[0]);if(age<=policy.gc.liveMarkerStaleMilliseconds&&processAliveV1(pid))return true;if(age>policy.gc.liveMarkerStaleMilliseconds||!processAliveV1(pid))rmSync(join(paths.live,name),{force:true});}catch{}
 }
 return false;
}

function treeBytes(path:string,limit:{files:number}):number{
 let total=0,entries;try{entries=readdirSync(path,{withFileTypes:true});}catch{return 0;}
 for(const entry of entries){
  if(limit.files--<=0)return total;
  const child=join(path,entry.name);
  if(entry.isDirectory())total+=treeBytes(child,limit);else if(entry.isFile()){try{total+=statSync(child).size;}catch{}}
 }
 return total;
}

/** 📏️ The incremental-compilation directories of a lane's build directory (`<profile>/incremental` and `<triple>/<profile>/incremental`). */
export function incrementalDirectoriesV1(build:string):string[]{
 const found:string[]=[];
 const walk=(directory:string,depth:number)=>{let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}for(const entry of entries){if(!entry.isDirectory())continue;const path=join(directory,entry.name);if(entry.name==="incremental")found.push(path);else if(depth<2)walk(path,depth+1);}};
 walk(build,0);return found;
}

/** 🪪️ The identifiers of the lanes that exist in a cache root. */
export function laneIdsV1(root:string):string[]{try{return readdirSync(join(root,layout.lanes)).filter(name=>idPattern.test(name)).sort();}catch{return[];}}

/** 🔭️ Measures every lane of a cache root for the garbage collector (logical bytes; reflinked files count in full, which only makes collection earlier) and records each size beside the lane, where the pool reads it. */
export function listLanesV1(root:string,nowMs:number):{id:string;bytes:number;incrementalBytes:number;lastUsedMs:number;live:boolean}[]{
 const base=join(root,layout.lanes);let ids:string[];try{ids=readdirSync(base).filter(name=>idPattern.test(name));}catch{return[];}
 return ids.map(id=>{const paths=lanePathsV1(root,id),limit={files:2_000_000};let lastUsedMs=0;try{lastUsedMs=Math.floor(statSync(paths.metadata).mtimeMs);}catch{}
  const incrementalBytes=incrementalDirectoriesV1(paths.build).reduce((sum,path)=>sum+treeBytes(path,limit),0),bytes=treeBytes(paths.base,limit);recordLaneSizeV1(root,id,bytes,nowMs);
  return{id,bytes,incrementalBytes,lastUsedMs,live:laneLiveV1(paths,nowMs)};});
}

/** 🗑️ Removes a whole lane or only its incremental directories. It takes the lane's own claim first, so it can never race an acquirer, and refuses while the claim is held or a live marker exists. */
export function removeLaneV1(root:string,id:string,scope:"lane"|"incremental",nowMs:number):boolean{
 const paths=lanePathsV1(root,id),claim=tryClaimV1(join(root,layout.locks),"lane-"+id);if(!claim)return false;
 try{
  if(laneLiveV1(paths,nowMs))return false;
  if(scope==="lane")rmSync(paths.base,{recursive:true,force:true});else for(const directory of incrementalDirectoriesV1(paths.build))rmSync(directory,{recursive:true,force:true});
  return true;
 }finally{claim.release();}
}
