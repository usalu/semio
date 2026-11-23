import {existsSync,lstatSync,readdirSync,rmdirSync} from "node:fs";
import {basename,dirname,join,relative} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {acquireResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {tryLockSessionV1,type IncrementalLockPortV1} from "../🌱️incremental/🟦️.ts";
import {laneLiveV1,lanePathsV1} from "../../🧵️lane/🟦️.ts";
import {allocatedCargoBytesV1,removeCargoGcTreeV1,type CargoGcControlV1} from "../📏️tree/🟦️.ts";

const policy=schema.definitions.Policy.const,messages=schema.definitions.Messages.const;
/** 📐️ The authored policy of the stale build unit collector.
 * @see ./🧬️schema/🔣️.json */
export const UNIT_POLICY_V1=policy;
/** 🧱️ One Cargo build unit (`build/<crate>/<hash>`): its allocated bytes, last use, whether a running compiler holds its lock and whether a running compiler holds its profile. */
export type BuildUnitV1=Readonly<{path:string;crate:string;profile:string;bytes:number;lastUsedMs:number;held:boolean;busy:boolean}>;
/** 🧭️ Why a unit is removed. */
export type UnitReasonV1="superseded"|"pressure";
/** 🗑️ One planned removal. */
export type UnitRemovalV1=Readonly<{path:string;bytes:number;reason:UnitReasonV1}>;
/** 📋️ The measured state the pure planner decides on: `needBytes` is what the image must free to reach its free-space target, `budgetBytes` the most the units may hold together (what they hold beyond it is needed too). */
export type UnitInputV1=Readonly<{needBytes:number;budgetBytes?:number;nowMs:number;units:readonly BuildUnitV1[]}>;
/** 🎫️ A profile of one build directory as the build lease names it. */
export type UnitProfileV1=Readonly<{buildDirectory:string;profileDirectory:string;profile:string}>;
/** 🔌️ Probes the exclusive build lease of a profile without waiting: `busy` while compilers hold it, else a lease this process holds until released. */
export type ProfileLeasePortV1=(profile:UnitProfileV1)=>Promise<"busy"|Readonly<{release():void}>>;

const order=(left:string,right:string)=>left<right?-1:left>right?1:0,byAge=(left:BuildUnitV1,right:BuildUnitV1)=>left.lastUsedMs-right.lastUsedMs||order(left.path,right.path),newestFirst=(left:BuildUnitV1,right:BuildUnitV1)=>right.lastUsedMs-left.lastUsedMs||order(left.path,right.path);

/** 🥇️ The newest `keepNewestPerCrate` units of every crate in every profile: the variants a build uses now. */
export function newestUnitsV1(units:readonly BuildUnitV1[],rules:Pick<typeof policy,"keepNewestPerCrate">=policy):ReadonlySet<string>{
 const groups=new Map<string,BuildUnitV1[]>();for(const unit of units){const key=dirname(unit.path);groups.set(key,[...(groups.get(key)??[]),unit]);}
 return new Set([...groups.values()].flatMap(group=>group.sort(newestFirst).slice(0,rules.keepNewestPerCrate).map(unit=>unit.path)));
}

/** 🛡️ Whether a unit may go: never one a compiler holds; a unit of a profile that compilers hold only when it is not one of the newest variants and idle for the busy minimum, any other unit once it is older than the minimum age. */
export function unitEligibleV1(unit:BuildUnitV1,newest:ReadonlySet<string>,nowMs:number,rules:Pick<typeof policy,"minimumAgeMilliseconds"|"busyMinimumAgeMilliseconds">=policy):boolean{
 if(unit.held)return false;
 return unit.busy?!newest.has(unit.path)&&nowMs-unit.lastUsedMs>=rules.busyMinimumAgeMilliseconds:nowMs-unit.lastUsedMs>=rules.minimumAgeMilliseconds;
}

/** 🧮️ Plans which units to remove, oldest first: units that newer variants of their crate supersede and that were idle for a day, then (while `needBytes` are not freed) the oldest eligible units that are not among the newest variants, then the oldest newest-variants of idle profiles. Pure and deterministic. */
export function planUnitsV1(input:UnitInputV1,override:Partial<typeof policy>={}):UnitRemovalV1[]{
 const rules={...policy,...override},ordered=[...input.units].sort(byAge),newest=newestUnitsV1(input.units,rules),removed=new Map<string,UnitRemovalV1>();
 const eligible=(unit:BuildUnitV1)=>unitEligibleV1(unit,newest,input.nowMs,rules),take=(unit:BuildUnitV1,reason:UnitReasonV1)=>{removed.set(unit.path,{path:unit.path,bytes:unit.bytes,reason});};
 for(const unit of ordered)if(eligible(unit)&&!newest.has(unit.path)&&input.nowMs-unit.lastUsedMs>rules.idleMilliseconds)take(unit,"superseded");
 let freed=[...removed.values()].reduce((sum,row)=>sum+row.bytes,0);
 const needBytes=Math.max(input.needBytes,input.budgetBytes===undefined?0:input.units.reduce((sum,unit)=>sum+unit.bytes,0)-input.budgetBytes);
 for(const wanted of[false,true])for(const unit of ordered){if(freed>=needBytes)break;if(newest.has(unit.path)===wanted&&eligible(unit)&&!removed.has(unit.path)){take(unit,"pressure");freed+=unit.bytes;}}
 return ordered.filter(unit=>removed.has(unit.path)).map(unit=>removed.get(unit.path)!);
}

const directoriesOf=(path:string):string[]=>{try{return readdirSync(path,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>join(path,entry.name));}catch{return [];}};
const isDirectory=(path:string):boolean=>{try{return lstatSync(path).isDirectory();}catch{return false;}};
const modified=(path:string):number=>{try{return lstatSync(path).mtimeMs;}catch{return 0;}};

/** 🗺️ Every profile directory of the compiler image that holds a `build` directory, with the build directory its lease is named by: the declared bases, at most `maximumProfileDepth` levels of profile and target-triple directories below each. Bounded directory reads only. */
export function unitProfilesV1(store:string):UnitProfileV1[]{
 const found=new Map<string,UnitProfileV1>();
 for(const pattern of policy.bases){
  let current=[store];
  for(const segment of pattern.split("/"))current=current.flatMap(dir=>segment==="*"?directoriesOf(dir):isDirectory(join(dir,segment))?[join(dir,segment)]:[]);
  for(const base of current){
   let level=[base];
   for(let depth=0;depth<policy.maximumProfileDepth;depth++){level=level.flatMap(directoriesOf).filter(dir=>basename(dir)!==policy.directory&&!(pattern===policy.laneBase&&depth===0&&policy.excludedLaneChildren.includes(basename(dir))));for(const dir of level)if(isDirectory(join(dir,policy.directory)))found.set(dir,{buildDirectory:base,profileDirectory:dir,profile:basename(dir)});}
  }
 }
 return [...found.values()].sort((left,right)=>order(left.profileDirectory,right.profileDirectory));
}

/** 🫀️ Whether a profile's lane has a live holder: a build that entered the lane and may still read or report any unit below it, however long a unit has been idle. Probed anew at every use, never cached. */
export type ProfileLivePortV1=(profile:UnitProfileV1)=>boolean;

/** 🫀️ The real port: the lane of a profile is the `u/l/<id>` directory above it; a profile outside a lane is never live. */
export function profileLivePortV1(store:string,nowMs:number=Date.now()):ProfileLivePortV1{
 const root=join(store,"u");
 return profile=>{
  const parts=relative(root,profile.profileDirectory).replaceAll("\\","/").split("/");
  if(parts[0]!==policy.laneBase.split("/")[1]||parts.length<2||parts[1]===".."||!/^[a-z0-9]{1,12}$/u.test(parts[1]!))return false;
  return laneLiveV1(lanePathsV1(root,parts[1]!),nowMs);
 };
}

const turn=()=>new Promise<void>(done=>setImmediate(done));
/** 🔭️ Measures every unit below the profiles: allocated bytes, the last time anything in it changed (never an access time, which measuring itself would renew) and whether a running compiler holds its lock or a live build holds its lane. Without a lock backend a unit counts as held until the busy minimum age has passed. */
export async function measureUnitsV1(profiles:readonly UnitProfileV1[],options:CargoGcControlV1&Readonly<{nowMs:number;busy:(profile:UnitProfileV1)=>boolean;live?:ProfileLivePortV1;lock?:IncrementalLockPortV1}>):Promise<BuildUnitV1[]>{
 const lock=options.lock??tryLockSessionV1,rows:BuildUnitV1[]=[];let done=0;
 for(const [index,profile] of profiles.entries()){
  const busy=options.busy(profile),live=options.live?.(profile)??false;
  for(const crate of directoriesOf(join(profile.profileDirectory,policy.directory))){
   options.signal?.throwIfAborted();
   for(const path of directoriesOf(crate)){
    if(!isDirectory(join(path,policy.marker)))continue;
    const used=Math.floor(Math.max(modified(path),modified(join(path,policy.marker,policy.usedMarker)),modified(join(path,policy.outputDirectory)),modified(join(path,policy.lockName)))),probe=lock(join(path,policy.lockName));
    if(typeof probe==="object")probe.release();
    const bytes=await allocatedCargoBytesV1(path,policy.maximumEntriesPerUnit,options);
    rows.push({path,crate:basename(crate),profile:profile.profile,bytes,lastUsedMs:used,held:live||probe==="held"||probe==="unsupported"&&options.nowMs-used<policy.busyMinimumAgeMilliseconds,busy});
    if(++done%policy.yieldEvery===0)await turn();
   }
  }
  await options.progress?.(index+1,profiles.length,"scan",profile.profileDirectory);
 }
 return rows;
}

/** 🔎️ The lease directory of a store: the first ancestor that carries the repository's lease data, undefined when none does (every profile then counts as held). */
export function leaseDirectoryOfStoreV1(store:string):string|undefined{
 let current=store;for(let step=0;step<policy.lease.maximumAncestors;step++){const candidate=join(current,policy.lease.directoryFromStore);if(existsSync(candidate))return candidate;const parent=dirname(current);if(parent===current)return undefined;current=parent;}
 return undefined;
}

/** 🔒️ The build lease resource of a profile, exactly as Cargo builds name it. */
export function profileLeaseResourceV1(profile:UnitProfileV1):string{return JSON.stringify([policy.lease.namespace,profile.buildDirectory.replaceAll("\\","/").normalize("NFC"),profile.profile]);}

/** 👤️ The real port: a non-waiting exclusive try on the profile's build lease, so a compiler that starts meanwhile queues behind the collection; without a lease directory every profile counts as held. */
export function profileLeasePortV1(store:string,signal?:AbortSignal):ProfileLeasePortV1{
 const directory=leaseDirectoryOfStoreV1(store);
 return async profile=>{
  if(directory===undefined)return "busy";
  try{const lease=await acquireResourceLease({directory,resource:profileLeaseResourceV1(profile),mode:"exclusive",signal:signal??new AbortController().signal,timeoutMs:0});return {release:()=>lease.release()};}
  catch(error){if(/timed out/iu.test(String((error as Error).message)))return "busy";throw error;}
 };
}

/** 📊️ What a collection did or, in a dry run, would do. */
export type UnitResultV1=Readonly<{dryRun:boolean;total:number;removed:readonly UnitRemovalV1[];freedBytes:number;held:number;kept:number;busyProfiles:number;profiles:Readonly<Record<string,number>>}>;
/** 💬️ One localized line with its placeholders filled. */
export function unitTextV1(key:keyof typeof messages,language:"en"|"de",variables:Readonly<Record<string,string|number>>={}):string{
 return messages[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}
const size=(bytes:number)=>bytes>=1073741824?(bytes/1073741824).toFixed(1)+" GiB":(bytes/1048576).toFixed(0)+" MiB";

/** 🧹️ Measures, plans and (unless `dryRun`) removes stale build units. A profile whose lane has a live holder is never touched, whatever its units' age or the budget. Profiles are probed first: a profile whose build lease is free is held exclusively while its units go (compilers that start meanwhile queue), a profile compilers hold only loses superseded units idle for the busy minimum; every removal re-takes the unit's own lock and holds it while the unit goes. Pressure removals stop once `satisfied()` says the image reached its free-space target. */
export async function collectUnitsV1(store:string,options:CargoGcControlV1&Readonly<{dryRun:boolean;needBytes:()=>number;budgetBytes?:number;satisfied?:()=>boolean;nowMs?:number;lock?:IncrementalLockPortV1;leases?:ProfileLeasePortV1;live?:ProfileLivePortV1;report?:Readonly<{language:"en"|"de";write:(line:string)=>void}>}>):Promise<UnitResultV1>{
 const nowMs=options.nowMs??Date.now(),lock=options.lock??tryLockSessionV1,leases=options.leases??profileLeasePortV1(store,options.signal),live=options.live??profileLivePortV1(store,nowMs),report=options.report,say=(key:keyof typeof messages,variables:Readonly<Record<string,string|number>>={}):void=>{if(report)report.write(unitTextV1(key,report.language,variables));};
 say("scan",{root:store});
 const profiles=unitProfilesV1(store),busy=new Set<string>();
 for(const profile of profiles){const probe=await leases(profile);if(probe==="busy"){busy.add(profile.profileDirectory);say("busyProfile",{profile:profile.profile,directory:profile.buildDirectory,hours:Math.round(policy.busyMinimumAgeMilliseconds/3600000)});}else probe.release();}
 const units=await measureUnitsV1(profiles,{nowMs,busy:profile=>busy.has(profile.profileDirectory),lock,live,signal:options.signal,progress:options.progress}),plan=planUnitsV1({needBytes:options.needBytes(),budgetBytes:options.budgetBytes,nowMs,units}),excess=options.budgetBytes===undefined?0:Math.max(0,units.reduce((sum,unit)=>sum+unit.bytes,0)-options.budgetBytes),removed:UnitRemovalV1[]=[],crates=new Set<string>(),newest=newestUnitsV1(units),byPath=new Map(units.map(unit=>[unit.path,unit])),profileByDirectory=new Map(profiles.map(profile=>[profile.profileDirectory,profile])),profileOf=(path:string):UnitProfileV1=>profileByDirectory.get(dirname(dirname(dirname(path))))??{buildDirectory:dirname(dirname(dirname(path))),profileDirectory:dirname(dirname(dirname(path))),profile:basename(dirname(dirname(dirname(path))))};
 const reason=(row:UnitRemovalV1)=>report?unitTextV1(`reason.${row.reason}` as keyof typeof messages,report.language,{hours:Math.round(policy.idleMilliseconds/3600000)}):"";
 let held=units.filter(unit=>unit.held).length;
 if(options.dryRun){
  for(const row of plan){removed.push(row);if(removed.length<=policy.reportedRemovals)say("would",{path:row.path,bytes:size(row.bytes),reason:reason(row)});}
 }else{
  const groups=new Map<string,UnitRemovalV1[]>();for(const row of plan){const key=profileOf(row.path).profileDirectory;groups.set(key,[...(groups.get(key)??[]),row]);}
  let done=0;
  for(const[directory,rows]of groups){
   const profile=profileOf(join(directory,policy.directory,"_","_"));if(live(profile))continue;
   const held_=await leases(profile),exclusive=held_!=="busy";
   try{
    for(const row of rows){
     options.signal?.throwIfAborted();
     if(row.reason==="pressure"&&options.satisfied?.()&&removed.reduce((sum,done)=>sum+done.bytes,0)>=excess)break;
     const unit=byPath.get(row.path)!;if(!exclusive&&!unitEligibleV1({...unit,busy:true},newest,nowMs))continue;
     const taken=lock(join(row.path,policy.lockName));
     if(taken==="held"){held++;say("held",{path:row.path});continue;}
     try{await removeCargoGcTreeV1(row.path,options);removed.push(row);crates.add(dirname(row.path));if(removed.length<=policy.reportedRemovals)say("removed",{path:row.path,bytes:size(row.bytes),reason:reason(row)});}
     finally{if(typeof taken==="object")taken.release();}
     await options.progress?.(++done,plan.length,"remove",row.path);if(done%policy.yieldEvery===0)await turn();
    }
   }finally{if(held_!=="busy")held_.release();}
  }
  for(const crate of crates)try{if(!readdirSync(crate).length)rmdirSync(crate);}catch{}
 }
 const freedBytes=removed.reduce((sum,row)=>sum+row.bytes,0);
 say(options.dryRun?"summaryPlan":"summaryApply",{count:removed.length,total:units.length,bytes:size(freedBytes),held,kept:units.length-removed.length-held});
 const gone=new Set(removed.map(row=>row.path)),totals:Record<string,number>={};for(const unit of units)if(!gone.has(unit.path)||options.dryRun){const name=relative(store,profileOf(unit.path).profileDirectory).replaceAll("\\","/");totals[name]=(totals[name]??0)+unit.bytes;}
 return Object.freeze({dryRun:options.dryRun,total:units.length,removed:Object.freeze(removed),freedBytes,held,kept:units.length-removed.length-held,busyProfiles:busy.size,profiles:Object.freeze(totals)});
}
