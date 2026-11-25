/** 🔬️ W15: bounded reproduction of the file-inventory phase of the workers' selected read (the scoped discovery of the owner admission closure of the Ready workspace selection, as `inventory()` runs it: `closeBuildOwnerAdmissionV1` + `discoverBuildOwnerAdmissionFactsV1` per owner inside one observation capture, facts remembered per owner with their shared rows), on the coordinator's real tree; stops itself at LIMIT owners or RSS above CAP MB. Then names the retainers by bytes: each holder is dropped in turn and a full collection measures what it alone kept (heap, external, strings, objects). */
import {heapStats} from "bun:jsc";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const producer=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await producer.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs"),adm=await import(L+"🕸️graph/🚪️admission/🟦️.ts"),si=await import(L+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const MB=1048576,LIMIT=Number(process.env.LIMIT??1e9),CAP=Number(process.env.CAP??3000);
const P=["@semio-tech/norm-din4108","@semio-tech/norm-din4108-rs","repo-mcp"],selection={kind:"owners",projects:P,exclude:[],targets:["build"]} as never;
const sample=()=>{const m=process.memoryUsage(),h=heapStats();return {rssMB:Math.round(m.rss/MB),heapMB:Math.round(m.heapUsed/MB),externalMB:Math.round(m.external/MB),strings:h.objectTypeCounts.string,objects:h.objectCount,functions:h.objectTypeCounts.Function,arrays:h.objectTypeCounts.Array};};
const mark=(label:string,extra:Record<string,unknown>={})=>{Bun.gc(true);console.log(JSON.stringify({label,...sample(),peakMB:Math.round(process.resourceUsage().maxRSS/1024),...extra}));};
mark("bootstrap");
const registry=adm.readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root)));
const nativeCache=new Map(),nativeClosures=new Map();let facts:Map<string,{facts:unknown;rows:unknown[]}>|undefined=new Map(),done=0,stopped="",failed=0;const t0=performance.now();
let captured:any=await obs.captureObservedSourceInputsV1(async()=>adm.closeBuildOwnerAdmissionV1(selection,registry,async(owner:any,targets:any,inputs:any)=>{
 if(stopped||done>=LIMIT||process.memoryUsage().rss/MB>CAP){stopped||=done>=LIMIT?"limit":"cap";throw Object.assign(Error("[DEBUG] probe stop"),{code:"probe-stop"});}
 const fresh=await obs.captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,{workspaceRoot:root,selection,nativeCache,nativeClosures},inputs),undefined,false);
 const rows=obs.shareObservationRowsV1(fresh.observations);obs.retainObservedRowsV1(rows);facts!.set(owner.name+"\0"+JSON.stringify([targets,inputs]),{facts:fresh.value,rows});
 if(++done%50===0){Bun.gc(true);console.log(JSON.stringify({label:"owners",done,seconds:Math.round((performance.now()-t0)/1000),...sample()}));}
 return fresh.value as never;
},failure=>{failed++;},()=>true,undefined,()=>0)).catch((error:any)=>({error:String(error?.message??error).slice(0,160)}));
mark("discovered",{done,failed,stopped,captureRows:captured?.observations?.length,error:captured?.error,seconds:Math.round((performance.now()-t0)/1000)});
let refs=0;const distinct=new Set();for(const e of facts.values()){refs+=e.rows.length;for(const r of e.rows)distinct.add(r);}const distinctRows=distinct.size;distinct.clear();
mark("counted",{entries:facts.size,rowRefs:refs,distinctRows});
captured=undefined;mark("drop:capture");
facts.clear();facts=undefined;mark("drop:ownerFacts");
nativeCache.clear();nativeClosures.clear();mark("drop:nativeCaches");
obs.releaseInferenceMemoryV1();mark("drop:inferenceMemory");
si.resetClosureEntriesV1();mark("drop:closureEntries");
console.log(JSON.stringify(obs.observationCollectionsV1().filter((c:any)=>c.entries>0).map((c:any)=>[c.name,c.entries])));
