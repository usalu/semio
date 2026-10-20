import {readdirSync,rmSync,statSync,statfsSync,truncateSync} from "node:fs";
import {join} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import {listObjectsV1,pruneEntriesV1,removeObjectsV1} from "../💾️store/🟦️.ts";
import {listLanesV1,removeLaneV1} from "../🧵️lane/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.gc;
/** 🧮️ One measured lane. */
export type GcLaneV1=Readonly<{id:string;bytes:number;incrementalBytes:number;lastUsedMs:number;live:boolean}>;
/** 🧮️ One cached object. */
export type GcObjectV1=Readonly<{digest:string;bytes:number;lastUsedMs:number}>;
/** 📋️ The measured state of one cache root. */
export type GcInputV1=Readonly<{capacityBytes:number;freeBytes:number;nowMs:number;lanes:readonly GcLaneV1[];objects:readonly GcObjectV1[]}>;
/** 📋️ What to remove, nothing else. */
export type GcPlanV1=Readonly<{removeLanes:string[];removeIncremental:string[];removeObjects:string[]}>;

const newestFirst=<T extends {lastUsedMs:number}>(key:(row:T)=>string)=>(left:T,right:T)=>right.lastUsedMs-left.lastUsedMs||(key(left)<key(right)?-1:key(left)>key(right)?1:0);
const oldestFirst=<T extends {lastUsedMs:number}>(key:(row:T)=>string)=>(left:T,right:T)=>left.lastUsedMs-right.lastUsedMs||(key(left)<key(right)?-1:key(left)>key(right)?1:0);

/** 🧹️ Plans the bounded cleanup of one cache root. Pure and deterministic.
 * Order: idle lanes; free-space pressure (incremental directories, then lanes, then objects, oldest first); the lane, incremental and object shares of the capacity. Live lanes are never touched and count against their share first. */
export function planGcV1(input:GcInputV1,override:Partial<typeof policy>={}):GcPlanV1{
 const rules={...policy,...override},cap=(share:number)=>Math.floor(input.capacityBytes*share);
 const removeLanes=new Set<string>(),removeIncremental=new Set<string>(),removeObjects=new Set<string>();
 const lanes=[...input.lanes].sort(oldestFirst(row=>row.id)),idle=lanes.filter(row=>!row.live);
 for(const lane of idle)if(input.nowMs-lane.lastUsedMs>rules.laneIdleMilliseconds)removeLanes.add(lane.id);
 const laneBytes=(lane:GcLaneV1)=>removeLanes.has(lane.id)?0:lane.bytes-(removeIncremental.has(lane.id)?lane.incrementalBytes:0);
 const objects=[...input.objects].sort(oldestFirst(row=>row.digest));
 let need=Math.max(0,cap(rules.minimumFreeShare)-input.freeBytes);
 const freed=()=>input.lanes.reduce((sum,lane)=>sum+lane.bytes-laneBytes(lane),0)+objects.filter(row=>removeObjects.has(row.digest)).reduce((sum,row)=>sum+row.bytes,0);
 if(need>0){
  for(const lane of idle){if(freed()>=need)break;if(!removeLanes.has(lane.id)&&!removeIncremental.has(lane.id)&&lane.incrementalBytes>0)removeIncremental.add(lane.id);}
  for(const lane of idle){if(freed()>=need)break;removeLanes.add(lane.id);}
  for(const object of objects){if(freed()>=need)break;removeObjects.add(object.digest);}
 }
 const keepPrefix=<T extends {lastUsedMs:number}>(rows:readonly T[],bytes:(row:T)=>number,budget:number,key:(row:T)=>string):T[]=>{
  const removed:T[]=[];let sum=0,overflow=false;
  for(const row of [...rows].sort(newestFirst(key))){sum+=bytes(row);if(overflow||sum>budget){overflow=true;removed.push(row);}}
  return removed;
 };
 const liveBytes=input.lanes.filter(row=>row.live).reduce((sum,row)=>sum+row.bytes,0);
 for(const lane of keepPrefix(idle.filter(row=>!removeLanes.has(row.id)),laneBytes,Math.max(0,cap(rules.laneShare)-liveBytes),row=>row.id))removeLanes.add(lane.id);
 const liveIncremental=input.lanes.filter(row=>row.live).reduce((sum,row)=>sum+row.incrementalBytes,0);
 const candidates=idle.filter(row=>!removeLanes.has(row.id));
 for(const lane of keepPrefix(candidates,row=>removeIncremental.has(row.id)?0:row.incrementalBytes,Math.max(0,cap(rules.incrementalShare)-liveIncremental),row=>row.id))if(lane.incrementalBytes>0)removeIncremental.add(lane.id);
 for(const object of keepPrefix(objects.filter(row=>!removeObjects.has(row.digest)),row=>row.bytes,cap(rules.casShare),row=>row.digest))removeObjects.add(object.digest);
 return Object.freeze({removeLanes:[...removeLanes].sort(),removeIncremental:[...removeIncremental].filter(id=>!removeLanes.has(id)).sort(),removeObjects:[...removeObjects].sort()});
}

/** 🔭️ Measures a cache root on disk. `capacityBytes` is the budget the root lives in (the Rust image), not the host disk. */
export function measureGcV1(root:string,capacityBytes:number,nowMs:number):GcInputV1{
 let freeBytes=capacityBytes;
 try{const stat=statfsSync(root,{bigint:true});freeBytes=Math.min(capacityBytes,Number(stat.bavail*stat.bsize));}catch{}
 return{capacityBytes,freeBytes,nowMs,lanes:listLanesV1(root,nowMs),objects:listObjectsV1(root)};
}

/** 🧹️ Applies a plan. Live lanes are re-checked at removal time, so a lane that became live since measuring survives. Returns what was removed. */
export function applyGcV1(root:string,plan:GcPlanV1,nowMs:number):{lanes:number;incremental:number;objects:number}{
 let lanes=0,incremental=0;
 for(const id of plan.removeLanes)if(removeLaneV1(root,id,"lane",nowMs))lanes++;
 for(const id of plan.removeIncremental)if(removeLaneV1(root,id,"incremental",nowMs))incremental++;
 const objects=removeObjectsV1(root,plan.removeObjects);
 pruneEntriesV1(root,nowMs);pruneEvents(root);
 return{lanes,incremental,objects};
}

function pruneEvents(root:string):void{
 const directory=join(root,UNIT_CACHE_POLICY_V1.layout.events);let names:string[];try{names=readdirSync(directory);}catch{return;}
 for(const name of names){try{const path=join(directory,name);if(statSync(path).size>policy.eventLogMaximumBytes)truncateSync(path,0);}catch{}}
}

/** 🧹️ Measures, plans and applies in one step. */
export function collectGarbageV1(root:string,capacityBytes:number,nowMs=Date.now()):{plan:GcPlanV1;applied:{lanes:number;incremental:number;objects:number}}{
 const plan=planGcV1(measureGcV1(root,capacityBytes,nowMs));
 return{plan,applied:applyGcV1(root,plan,nowMs)};
}

/** 🗑️ Removes the whole cache root. */
export function removeCacheRootV1(root:string):void{rmSync(root,{recursive:true,force:true});}
