import {AsyncLocalStorage} from "node:async_hooks";
import {writeSync} from "node:fs";
import {createRequire} from "node:module";

/** 🎛️ The environment variable that switches the phase profile on: `1` prints tab separated rows to stderr when the process exits, `json` prints one JSON document, `0`/`off`/unset print nothing. */
export const REPO_PROFILE_VARIABLE_V1="SEMIO_REPO_PROFILE";
/** 🔤️ A phase name is a language neutral identifier path such as `leases.open`: the profile output carries no translated text, only names, numbers and the legend below. */
export const REPO_PROFILE_PHASE_PATTERN_V1=/^[a-z][a-z0-9]*(?:[.:/-][a-z0-9]+)*$/u;
/** 🗺️ Names every column of the table output in English and German; the rows themselves are neutral. */
export const REPO_PROFILE_LEGEND_V1=Object.freeze({
 seq:Object.freeze({en:"Row number in start order",de:"Zeilennummer in Startreihenfolge"}),
 depth:Object.freeze({en:"Nesting depth of the phase",de:"Verschachtelungstiefe der Phase"}),
 phase:Object.freeze({en:"Phase name",de:"Name der Phase"}),
 start:Object.freeze({en:"Milliseconds from process start to the phase start",de:"Millisekunden vom Prozessstart bis zum Phasenbeginn"}),
 duration:Object.freeze({en:"Milliseconds from phase start to phase end",de:"Millisekunden vom Phasenbeginn bis zum Phasenende"}),
 self:Object.freeze({en:"Duration minus the time covered by nested phases",de:"Dauer abzueglich der von verschachtelten Phasen abgedeckten Zeit"}),
 detail:Object.freeze({en:"Optional neutral detail such as a program name or a count",de:"Optionales neutrales Detail wie ein Programmname oder eine Anzahl"}),
});

const MODES=Object.freeze({"":"off","0":"off",off:"off",false:"off","1":"table",on:"table",true:"table",table:"table",tsv:"table",json:"json"});
/** 🔎️ Reads the profile mode from an environment; an unknown value is refused in both languages instead of silently profiling nothing. */
export function readRepoProfileModeV1(environment=process.env){
 const raw=environment?.[REPO_PROFILE_VARIABLE_V1]??"",mode=Object.hasOwn(MODES,String(raw).toLowerCase())?MODES[String(raw).toLowerCase()]:undefined;
 if(mode===undefined)throw Error(`Expected ${REPO_PROFILE_VARIABLE_V1}=1|json|off, got "${raw}" / Erwartet ${REPO_PROFILE_VARIABLE_V1}=1|json|off, erhalten "${raw}"`);
 return mode;
}

const round=value=>Math.round(value*1000)/1000;
/** ➕️ Merges intervals and returns the covered length inside [from,to]. */
function coveredV1(intervals,from,to){
 const sorted=intervals.map(([a,b])=>[Math.max(a,from),Math.min(b,to)]).filter(([a,b])=>b>a).sort((x,y)=>x[0]-y[0]);
 let total=0,end=-Infinity;
 for(const [a,b] of sorted){if(a>end){total+=b-a;end=b;}else if(b>end){total+=b-end;end=b;}}
 return total;
}

/** 🧮️ Reduces raw phase records to ordered rows with self time (duration minus the union of the direct children, so overlapping children are never counted twice) and the process totals. */
export function summarizeRepoProfileV1(records,totalMilliseconds){
 const children=new Map();
 for(const record of records)if(record.parent!==0){(children.get(record.parent)??children.set(record.parent,[]).get(record.parent)).push([record.start,record.end]);}
 const rows=records.slice().sort((a,b)=>a.start-b.start||a.id-b.id).map((record,index)=>{
  const duration=record.end-record.start,self=duration-coveredV1(children.get(record.id)??[],record.start,record.end);
  return Object.freeze({seq:index+1,id:record.id,parent:record.parent,depth:record.depth,phase:record.phase,start:round(record.start),duration:round(duration),self:round(Math.max(0,self)),...(record.detail===undefined?{}:{detail:record.detail})});
 });
 const top=records.filter(record=>record.parent===0).map(record=>[record.start,record.end]),covered=coveredV1(top,0,totalMilliseconds);
 return Object.freeze({version:1,unit:"ms",total:round(totalMilliseconds),covered:round(covered),uncovered:round(Math.max(0,totalMilliseconds-covered)),rows:Object.freeze(rows)});
}

/** 📄️ Renders a summary as tab separated rows: a comment header with the legend keys, one row per phase and a total row. */
export function renderRepoProfileTableV1(summary,pid){
 const lines=[`#repo-profile\tversion=${summary.version}\tpid=${pid}\tunit=${summary.unit}`,"#"+["seq","depth","phase","start","duration","self","detail"].join("\t")];
 for(const row of summary.rows)lines.push([row.seq,row.depth,row.phase,row.start.toFixed(1),row.duration.toFixed(1),row.self.toFixed(1),row.detail??""].join("\t"));
 lines.push(`#total\t${summary.total.toFixed(1)}\tcovered\t${summary.covered.toFixed(1)}\tuncovered\t${summary.uncovered.toFixed(1)}`);
 return lines.join("\n")+"\n";
}

/** ⏱️ Creates one phase profile: `phase(name, body)` times a synchronous or asynchronous body and nests it under the surrounding phase of the same asynchronous context; disabled profiles return the body untouched. */
export function createRepoProfileV1({mode="off",now=()=>performance.now(),pid=process.pid,write=text=>{try{writeSync(2,text);}catch{}}}={}){
 const enabled=mode!=="off",context=new AsyncLocalStorage(),records=[];let counter=0,flushed=false;
 const open=(phase,detail)=>{
  if(!REPO_PROFILE_PHASE_PATTERN_V1.test(phase))throw Error(`Expected a neutral profile phase name, got "${phase}" / Erwartet einen neutralen Profilphasennamen, erhalten "${phase}"`);
  const parent=context.getStore(),record={id:++counter,parent:parent?.id??0,depth:(parent?.depth??-1)+1,phase,start:now(),end:NaN,detail};
  records.push(record);return record;
 };
 const close=record=>{if(Number.isNaN(record.end))record.end=now();};
 const profile={
  enabled,mode,
  phase(phase,body,detail){
   if(!enabled)return body();
   const record=open(phase,detail);
   try{
    const result=context.run(record,body);
    if(result&&typeof result.then==="function")return result.then(value=>{close(record);return value;},error=>{close(record);throw error;});
    close(record);return result;
   }catch(error){close(record);throw error;}
  },
  event(phase,durationMilliseconds,detail){
   if(!enabled)return;
   const record=open(phase,detail),end=now();record.start=Math.max(0,end-durationMilliseconds);record.end=end;
  },
  mark(phase,detail){profile.event(phase,0,detail);},
  summary(){const end=now();for(const record of records)if(Number.isNaN(record.end))record.end=end;return summarizeRepoProfileV1(records,end);},
  flush(){
   if(!enabled||flushed)return;flushed=true;
   const summary=profile.summary();write(mode==="json"?JSON.stringify({pid,...summary})+"\n":renderRepoProfileTableV1(summary,pid));
  },
 };
 return Object.freeze(profile);
}

let shared;
/** 🌐️ Returns the process wide profile, created on first use from the environment and flushed when the process exits. */
export function repoProfileV1(){
 if(shared===undefined){
  shared=createRepoProfileV1({mode:readRepoProfileModeV1(process.env)});
  if(shared.enabled){process.on("exit",()=>shared.flush());installRepoProfileSpawnCensusV1(shared);}
 }
 return shared;
}
/** 🧪️ Drops the shared profile so a test can create another one. */
export function resetRepoProfileV1(){shared=undefined;}
/** ⏲️ Times one body as a phase of the process wide profile; a disabled profile costs one function call. */
export function profilePhaseV1(phase,body,detail){return repoProfileV1().phase(phase,body,detail);}
/** 📍️ Records a zero length marker in the process wide profile. */
export function profileMarkV1(phase,detail){repoProfileV1().mark(phase,detail);}

/** 🏷️ Names a child process event after its program: the base name, lower case, with every character outside `a-z0-9-` folded to `-`, so `/usr/bin/vm_stat` becomes `spawn.vm-stat` and `C:\\tools\\Python3.11.exe` becomes `spawn.python3-11-exe`. */
export function repoProfileSpawnLabelV1(command){
 const list=Array.isArray(command)?command:[command],base=String(list[0]??"").split(/[\\/]/u).pop().toLowerCase().replace(/[^a-z0-9]+/gu,"-").replace(/^-+|-+$/gu,"");
 return "spawn."+(/^[a-z]/u.test(base)?base:base===""?"unknown":"x"+base);
}

let census=false;
/** 🐚️ Counts every child process started through `Bun.spawn`, `Bun.spawnSync` and the callable members of `node:child_process` that are bound after this call, as `spawn` events named after the program; consumers that bound a named import earlier are only visible to a `--preload` census. */
export function installRepoProfileSpawnCensusV1(profile){
 if(census||!profile.enabled)return;census=true;
 const emit=(command,start)=>{try{profile.event(repoProfileSpawnLabelV1(command),performance.now()-start);}catch{}};
 const wrapSync=(owner,name,extract)=>{const original=owner?.[name];if(typeof original!=="function")return;owner[name]=function(...args){const start=performance.now();try{return original.apply(this,args);}finally{emit(extract(args),start);}};};
 const wrapAsync=(owner,name,extract)=>{const original=owner?.[name];if(typeof original!=="function")return;owner[name]=function(...args){const start=performance.now(),child=original.apply(this,args);const done=()=>emit(extract(args),start);if(typeof child?.once==="function")child.once("close",done);else if(child?.exited?.then)child.exited.then(done,done);return child;};};
 try{
  const require=createRequire(import.meta.url),cp=require("node:child_process");
  for(const name of ["spawnSync","execFileSync","execSync"])wrapSync(cp,name,args=>args[0]);
  for(const name of ["spawn","execFile","exec"])wrapAsync(cp,name,args=>args[0]);
  if(typeof Bun!=="undefined"){wrapSync(Bun,"spawnSync",args=>args[0]?.cmd??args[0]);wrapAsync(Bun,"spawn",args=>args[0]?.cmd??args[0]);}
 }catch{}
}
