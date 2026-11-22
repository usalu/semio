import * as raw from "node:fs";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const workspace=process.cwd(),observation=pathToFileURL(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs")).href;
const {observationsHold,observationsHoldWithinRoundV1,withObservationRoundV1,withObservedWorldV1}=await import(observation);
const scratch=join(import.meta.dir,"🗑️generated","metadata-bench");
await mkdir(scratch,{recursive:true});
const root=await mkdtemp(join(scratch,"p-"));
const samples:{mode:string;calls:number;wallMs:number;cpuMs:number}[]=[];
try{
 const names=Array.from({length:512},(_,index)=>join(root,String(index))),directories=new Set(names.filter((_,index)=>index%3===0));
 for(const path of names){if(directories.has(path))await mkdir(path);else await writeFile(path,"x");}
 const owners=[
  names.map(path=>({path,kind:"stat",fact:directories.has(path)?"directory":"file"})),
  names.map(path=>({path,kind:"fileExists",fact:!directories.has(path)})),
  names.map(path=>({path,kind:"directoryExists",fact:directories.has(path)})),
 ];
 let calls=0;
 const world={...raw,statSync(...args:Parameters<typeof raw.statSync>){calls++;return raw.statSync(...args);}};
 await withObservedWorldV1(world,async()=>{
  for(let pass=0;pass<9;pass++)for(const mode of pass%2?["shared","fresh"]:["fresh","shared"]){
   calls=0;const start=performance.now(),cpu=process.cpuUsage();
   const verdict=await withObservationRoundV1(()=>Promise.all(owners.map(owner=>mode==="shared"?observationsHoldWithinRoundV1(owner,64):observationsHold(owner))));
   const used=process.cpuUsage(cpu);
   if(!verdict.every(Boolean)||calls!==(mode==="shared"?names.length:names.length*owners.length))throw Error("The native metadata measurement did not match its file inventory");
   if(pass>0)samples.push({mode,calls,wallMs:performance.now()-start,cpuMs:(used.user+used.system)/1000});
  }
 });
 const median=(numbers:number[])=>numbers.sort((a,b)=>a-b).slice(3,5).reduce((a,b)=>a+b,0)/2;
 for(const mode of ["fresh","shared"]){const rows=samples.filter(row=>row.mode===mode);console.log("[DEBUG] "+JSON.stringify({mode,files:names.length,nativeStats:rows[0]!.calls,medianWallMilliseconds:median(rows.map(row=>row.wallMs)),medianCpuMilliseconds:median(rows.map(row=>row.cpuMs))}));}
}finally{await rm(root,{recursive:true,force:true});}
