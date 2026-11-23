import {readFileSync} from "node:fs";
import {pathToFileURL} from "node:url";
const build="C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const workspaceRoot="C:/semio",memoPath=process.argv[2],from=Number(process.argv[3]??0),count=Number(process.argv[4]??6),wanted=process.argv[5];
const observation=await import(pathToFileURL(build+"👁️observation/🟨️.mjs").href);
const producer=await import(pathToFileURL(build+"🕸️graph/📥️inference/🟨️.mjs").href);
const admission=await import(pathToFileURL(build+"🕸️graph/🚪️admission/🟦️.ts").href);
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(workspaceRoot)));
const memo=JSON.parse(readFileSync(memoPath,"utf8"));
let owners=memo.owners.map((owner:any)=>{const at=owner.suffix.indexOf("\0");return{name:owner.suffix.slice(0,at),spec:JSON.parse(owner.suffix.slice(at+1)),rows:owner.rows.length};});
if(wanted)owners=owners.filter((owner:any)=>wanted.split(",").includes(owner.name));else owners=owners.filter((owner:any)=>owner.rows>1500&&owner.rows<9000&&owner.spec[0].length>0).slice(from,from+count);
await producer.libraryBootstrap;
const probes=()=>observation.observationProbeCounts();
const nativeCache=new Map(),nativeClosures=new Map(),timings:any[]=[];
const t0=performance.now(),c0=process.cpuUsage();
const run=()=>observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
 for(const entry of owners){
  const owner=registry.owners.find((row:any)=>row.name===entry.name);if(!owner){timings.push({owner:entry.name,missing:true});continue;}
  const started=performance.now();let rows=-1,error:string|undefined;
  try{const captured=await observation.captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(owner,entry.spec[0],registry,{workspaceRoot,nativeCache,nativeClosures},entry.spec[1]),undefined,false);rows=captured.observations.length;}catch(failure:any){error=String(failure?.message??failure).slice(0,120);}
  timings.push({owner:entry.name,targets:entry.spec[0].length,ms:Math.round(performance.now()-started),rows,memoRows:entry.rows,error});console.error(JSON.stringify(timings.at(-1)));
 }
}),undefined,false);
import fs from "node:fs";const counts:Record<string,number>={};const port=new Proxy(fs,{get:(target,key)=>typeof key==="string"&&["readdirSync","lstatSync","statSync","existsSync","readFileSync"].includes(key)?(...rest:any[])=>{counts[key]=(counts[key]??0)+1;return (target as any)[key](...rest);}:(target as any)[key]});await observation.withObservedWorldV1(port,run);
const cpu=process.cpuUsage(c0);
console.log(JSON.stringify({owners:owners.length,ms:Math.round(performance.now()-t0),cpuMs:Math.round((cpu.user+cpu.system)/1000),counts,timings}));
