import {pathToFileURL} from "node:url";
import fs from "node:fs";
import {join} from "node:path";
const root="C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const observation=await import(pathToFileURL(root+"👁️observation/🟨️.mjs").href);
const {observedFs}=observation;
const start=process.argv[2],owners=Number(process.argv[3]??3);
const counts={readdir:0,lstat:0};
const port=new Proxy(fs,{get:(target,key)=>key==="readdirSync"?(...a:any[])=>{counts.readdir++;return (target.readdirSync as any)(...a);}:key==="lstatSync"?(...a:any[])=>{counts.lstat++;return (target.lstatSync as any)(...a);}:(target as any)[key]});
const walk=(dir:string)=>{
 const pending=[dir];let n=0;
 while(pending.length){const d=pending.pop()!;try{if(!observedFs.lstatSync(d).isDirectory())continue;}catch{continue;}
  for(const e of observedFs.readdirSync(d,{withFileTypes:true})){n++;if(e.isSymbolicLink())continue;if(e.isDirectory()&&!["node_modules","target",".git"].includes(e.name))pending.push(join(d,e.name));}}
 return n;
};
const t0=performance.now(),c0=process.cpuUsage();
const captured=await observation.withObservedWorldV1(port,()=>observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
 let n=0;for(let i=0;i<owners;i++)n+=walk(start);return n;
}),undefined,false));
const cpu=process.cpuUsage(c0);
console.log(JSON.stringify({value:captured.value,rows:captured.observations.length,counts,ms:Math.round(performance.now()-t0),cpuMs:Math.round((cpu.user+cpu.system)/1000)}));
