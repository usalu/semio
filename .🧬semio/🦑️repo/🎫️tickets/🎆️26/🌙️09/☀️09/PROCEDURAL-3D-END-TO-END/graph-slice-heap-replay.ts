import {readFileSync} from "node:fs";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",root="/Users/ueli/Documents/semio";
const [memoPath,arm,retain,from,count]=process.argv.slice(2);
const obs=await import(G+"/👁️observation/🟨️.mjs"),producer=await import(G+"/🕸️graph/📥️inference/🟨️.mjs"),{readBuildOwnerRegistryV1}=await import(G+"/🕸️graph/🚪️admission/🟦️.ts");
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),selection={kind:"tasks",tasks:[{project:"@semio-tech/framework-os-dev",target:"dev-generation3d-react-prototype"}]};
let suffixes:string[]=JSON.parse(readFileSync(memoPath,"utf8")).owners.map((o:any)=>o.suffix);Bun.gc(true);
suffixes=suffixes.slice(Number(from),Number(from)+Number(count));
const heap=()=>{Bun.gc(true);const m=process.memoryUsage();return Math.round(m.heapUsed/1048576)+"MB heap "+Math.round(m.rss/1048576)+"MB rss"};
import {heapStats} from "bun:jsc";
const types=()=>{Bun.gc(true);const c=heapStats().objectTypeCounts as Record<string,number>;return c;};
const base=types();
console.log("start",heap());
const kept:unknown[]=[],shared={native:new Map(),closures:new Map()};let n=0;
for(const suffix of suffixes){
 const [name,rest]=[suffix.slice(0,suffix.indexOf("\0")),suffix.slice(suffix.indexOf("\0")+1)],[targets,inputs]=JSON.parse(rest),owner=registry.owners.find((o:any)=>o.name===name);if(!owner)continue;
 const t=performance.now(),nativeCache=arm==="shared"?shared.native:new Map(),nativeClosures=arm==="shared"?shared.closures:new Map();
 const r=await obs.captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,selection,nativeCache,nativeClosures,preparationControl:{advance:async()=>{}}},inputs),undefined,false);
 if(retain==="1")kept.push({facts:r.value,rows:r.observations});n++;
 if(n%20===0){const c=types();console.log("types",n,"Set",c["Set"],"Map",c["Map"],"Array",c["Array"],"Object",c["Object"],"string",c["string"],"WeakRef",c["WeakRef"]);}
 if(n%5===0||performance.now()-t>1000)console.log(n,name,Math.round(performance.now()-t)+"ms","rows",r.observations.length,heap(),"nativeCache",shared.native.size,shared.closures.size);
}
{const now=types(),diff=Object.entries(now).map(([k,v])=>[k,v-(base[k]??0)] as [string,number]).sort((a,b)=>b[1]-a[1]).slice(0,14);console.log("Set",now["Set"],"base",base["Set"],"Map",now["Map"],"Array",now["Array"],"base",base["Array"]);console.log("type growth",JSON.stringify(diff));console.log("collections",JSON.stringify(obs.observationCollectionsV1().map((c:any)=>[c.name.slice(0,40),c.entries,Math.round((c.estimatedBytes??c.bytes??0)/1048576)]).filter((c:any)=>c[2]>0||c[1]>1000)));console.log("caches",JSON.stringify(obs.inferenceCacheSizesV1()));}
const total=kept.reduce((a:number,e:any)=>a+e.rows.length,0),ids=new Set<unknown>(),keys=new Set<string>();let chars=0;for(const e of kept as any[])for(const r of e.rows){ids.add(r);const k=r.kind+"\0"+r.path+"\0"+r.fact;if(!keys.has(k)){keys.add(k);chars+=k.length;}}
console.log("end",heap(),"kept",kept.length,"rows",total,"distinct objects",ids.size,"distinct keys",keys.size,"key chars",chars);
