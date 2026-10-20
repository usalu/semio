import {createHash,randomBytes} from "node:crypto";
import {closeSync,mkdirSync,openSync,readFileSync,readdirSync,rmSync,statSync,utimesSync,writeFileSync,writeSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import {processAliveV1,tryClaimV1,type ClaimV1} from "../🔒️lock/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1,layout=policy.layout,idPattern=/^[a-z0-9]{1,12}$/u;
/** 🧵️ The default lane every unattributed command shares. */
export const DEFAULT_LANE_ID_V1="d";
/** 🧭️ One ancestor process as the lane resolver sees it. */
export type AncestorV1=Readonly<{pid:number;started:string;command:string}>;
/** 🔌️ Reads the parent chain of the current process; replaceable so resolution is testable on any platform. */
export type AncestryPortV1=()=>readonly AncestorV1[];
const clients=/(claude|codex|cursor|windsurf|copilot|antigravity|droid|kiro)/iu;

/** 🪪️ Reads the parent chain with `ps` where it exists; platforms without it report no ancestry and fall back to the default lane. */
export function systemAncestryV1():readonly AncestorV1[]{
 if(process.platform==="win32")return[];
 const rows:AncestorV1[]=[];let pid=process.ppid;
 for(let depth=0;depth<16&&pid>1;depth++){
  const result=spawnSync("ps",["-o","ppid=,lstart=,command=","-p",String(pid)],{encoding:"utf8"});if(result.status!==0)break;
  const match=/^\s*(\d+)\s+(.{24})\s+(.*)$/u.exec(result.stdout.trim());if(!match)break;
  rows.push({pid,started:match[2]!,command:match[3]!});pid=Number(match[1]);
 }
 return rows;
}

/** 🏷️ Chooses the lane: an explicit flag, then `SEMIO_CARGO_LANE`, then the nearest agent-client ancestor (one lane per agent session), then the default lane. Lanes never share Cargo build directories, so Cargo's own locks never span two agents. */
export function resolveLaneIdV1(input:Readonly<{flag?:string;environment:Readonly<Record<string,string|undefined>>;ancestry:AncestryPortV1}>):string{
 const explicit=input.flag??input.environment.SEMIO_CARGO_LANE;
 if(explicit!==undefined){if(!idPattern.test(explicit))throw Error("Cargo lane identifiers are 1 to 12 lowercase letters or digits");return explicit;}
 const client=input.ancestry().find(row=>clients.test(row.command));
 return client?"a"+BigInt("0x"+createHash("sha256").update(client.pid+"\0"+client.started).digest("hex").slice(0,12)).toString(36).slice(0,10):DEFAULT_LANE_ID_V1;
}

/** 🗺️ The directories of one lane inside a cache root; compact on purpose so Windows paths stay short. */
export function lanePathsV1(root:string,id:string){
 if(!idPattern.test(id))throw Error("Invalid Cargo lane identifier");
 const base=join(root,layout.lanes,id);
 return Object.freeze({id,base,target:join(base,layout.target),build:join(base,layout.build),live:join(base,layout.live),stamps:join(base,layout.stamps),metadata:join(base,layout.metadata)});
}

/** 🚪️ Registers one running command in its lane. The marker is kept fresh by a heartbeat and removed on release; garbage collection never touches a lane with a fresh marker. */
export function enterLaneV1(root:string,id:string):Readonly<{paths:ReturnType<typeof lanePathsV1>;release():void}>{
 const paths=lanePathsV1(root,id);
 for(const directory of[paths.target,paths.build,paths.live,paths.stamps])mkdirSync(directory,{recursive:true});
 const touch=()=>{try{const now=new Date();utimesSync(paths.metadata,now,now);}catch{writeFileSync(paths.metadata,JSON.stringify({version:1,id,createdMs:Date.now()}));}};touch();
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

/** 🔭️ Measures every lane of a cache root for the garbage collector (logical bytes; reflinked files count in full, which only makes collection earlier). */
export function listLanesV1(root:string,nowMs:number):{id:string;bytes:number;incrementalBytes:number;lastUsedMs:number;live:boolean}[]{
 const base=join(root,layout.lanes);let ids:string[];try{ids=readdirSync(base).filter(name=>idPattern.test(name));}catch{return[];}
 return ids.map(id=>{const paths=lanePathsV1(root,id),limit={files:2_000_000};let lastUsedMs=0;try{lastUsedMs=Math.floor(statSync(paths.metadata).mtimeMs);}catch{}
  const incrementalBytes=incrementalDirectoriesV1(paths.build).reduce((sum,path)=>sum+treeBytes(path,limit),0);
  return{id,bytes:treeBytes(paths.base,limit),incrementalBytes,lastUsedMs,live:laneLiveV1(paths,nowMs)};});
}

/** 🗑️ Removes a whole lane or only its incremental directories; refuses while a live marker exists. */
export function removeLaneV1(root:string,id:string,scope:"lane"|"incremental",nowMs:number):boolean{
 const paths=lanePathsV1(root,id);if(laneLiveV1(paths,nowMs))return false;
 if(scope==="lane")rmSync(paths.base,{recursive:true,force:true});else for(const directory of incrementalDirectoriesV1(paths.build))rmSync(directory,{recursive:true,force:true});
 return true;
}

/** 📖️ Reads a lane's metadata file. */
export function readLaneV1(root:string,id:string):{version:1;id:string;createdMs:number}|undefined{try{return JSON.parse(readFileSync(lanePathsV1(root,id).metadata,"utf8"));}catch{return undefined;}}

/** 🧵️ The most simultaneous Cargo processes one agent session can run: its lane and this many overflow lanes. */
export const MAXIMUM_OVERFLOW_LANES_V1=31;

/** 🔐️ Enters the first free lane of an agent: the lane itself, else `<lane>1` … `<lane>31`. A lane is held exclusively by one Cargo process, so no two Cargo processes ever share a build directory and Cargo's own unit locks can never form a cycle between processes; an overflow lane starts empty and fills from the shared unit cache. */
export function acquireLaneV1(root:string,base:string):Readonly<{paths:ReturnType<typeof lanePathsV1>;overflow:number;release():void}>{
 if(!idPattern.test(base))throw Error("Invalid Cargo lane identifier");
 const locks=join(root,layout.locks);
 for(let overflow=0;overflow<=MAXIMUM_OVERFLOW_LANES_V1;overflow++){
  const id=overflow===0?base:base.slice(0,12-String(overflow).length)+String(overflow),claim:ClaimV1|undefined=tryClaimV1(locks,"lane-"+id);
  if(!claim)continue;
  const entered=enterLaneV1(root,id);
  return Object.freeze({paths:entered.paths,overflow,release(){try{entered.release();}finally{claim.release();}}});
 }
 throw Error("Every overflow Cargo lane of this agent is busy");
}
