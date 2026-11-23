const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",root="/Users/ueli/Documents/semio";
const obs=await import(G+"/👁️observation/🟨️.mjs"),producer=await import(G+"/🕸️graph/📥️inference/🟨️.mjs"),{readBuildOwnerRegistryV1}=await import(G+"/🕸️graph/🚪️admission/🟦️.ts");
await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),name=process.argv[2],steps=JSON.parse(process.argv[3]);
import {startSamplingProfiler,samplingProfilerStackTraces} from "bun:jsc";
const dump=()=>{const traces:string=samplingProfilerStackTraces().traces??"";const counts=new Map<string,number>();for(const block of String(traces).split("\n\n")){const lines=block.split("\n").filter(Boolean);const top=lines.slice(0,3).join(" < ");if(top)counts.set(top,(counts.get(top)??0)+1);}console.log("PROFILE",[...counts].sort((a,b)=>b[1]-a[1]).slice(0,25).map(([k,v])=>v+" "+k.slice(0,300)).join("\n"));};
startSamplingProfiler();setTimeout(()=>{dump();},Number(process.env.DUMP_MS??90000));
const owner=registry.owners.find((o:any)=>o.name===name)!,selection={kind:"tasks",tasks:[{project:"@semio-tech/framework-os-dev",target:"dev-generation3d-react-prototype"}]};
for(const [targets,inputs] of steps){
 const nativeCache=new Map(),nativeClosures=new Map(),phases:[string,number][]=[];let last=performance.now(),lastPhase="start";const t0=last;
 const r=await obs.captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,selection,nativeCache,nativeClosures,preparationControl:{advance:async()=>{}},onOwnerDiscoveryPhase:(p:string)=>{const n=performance.now();phases.push([lastPhase,Math.round(n-last)]);console.log("phase",lastPhase,Math.round(n-last),"ms; now",p);last=n;lastPhase=p;}},inputs),undefined,false);
 phases.push([lastPhase,Math.round(performance.now()-last)]);
 console.log(JSON.stringify(targets),JSON.stringify(inputs),Math.round(performance.now()-t0)+"ms","rows",r.observations.length,"files",r.value.files.length,JSON.stringify(phases));
}
