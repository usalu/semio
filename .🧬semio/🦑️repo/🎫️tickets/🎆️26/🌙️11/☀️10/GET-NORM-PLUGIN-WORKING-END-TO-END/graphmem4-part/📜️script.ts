#!/usr/bin/env repo
import{execFileSync}from"node:child_process";
import{readFileSync,appendFileSync}from"node:fs";
import{heapStats,startSamplingProfiler,samplingProfilerStackTraces}from"bun:jsc";
import{createIssuedResourceControlV1,withIssuedResourceControlV1,readScriptPolicy,createScriptProcessEnvelope,withScriptProcessEnvelope}from"/home/deck/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{createBuildBootstrapCapabilitiesV1}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/📤️child/🟦️.ts";
import{issueRepositoryScriptProvidersV1}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🔌️providers/🟦️.ts";
import{withRepositoryBuildGraphFactStoreV1}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs";
const root="/home/deck/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",OUT=process.env.PROBE_OUT!;
const log=(...parts:unknown[])=>{appendFileSync(OUT,"[DEBUG] graphmem4-part "+parts.map(part=>typeof part==="string"?part:JSON.stringify(part)).join(" ")+"\n");};
process.env.GRAPHMEM4_PROBE="1";
let snapped=false;
const snapshotReport=()=>{
 const g=Bun.generateHeapSnapshot() as any,N=g.nodes.length/4,E=g.edges.length/4,c=new Map<string,number[]>();
 for(let i=0;i<g.nodes.length;i+=4){const k=g.nodeClassNames[g.nodes[i+2]];const r=c.get(k)??[0,0];r[0]++;r[1]+=g.nodes[i+1];c.set(k,r);}
 log("SNAP histogram",[...c].sort((a,b)=>b[1][1]-a[1][1]).slice(0,12).map(([k,v])=>Math.round(v[1]/1e6)+"MB "+v[0]+" "+k));
 const per=new Map<string,number[]>();
 for(let e=0;e<E;e++){const f=g.edges[e*4],t=g.edges[e*4+1],tc=g.nodeClassNames[g.nodes[t*4+2]];if(!["string","Object","Array","WeakMap","Uint8Array","Map","Set"].includes(tc))continue;const key=tc+" <- "+g.nodeClassNames[g.nodes[f*4+2]]+" ."+(g.edges[e*4+2]===1?g.edgeNames[g.edges[e*4+3]]:g.edgeTypes[g.edges[e*4+2]]);const row=per.get(key)??[0,0];row[0]++;row[1]+=g.nodes[t*4+1];per.set(key,row);}
 log("SNAP retainers",[...per].sort((a,b)=>b[1][1]-a[1][1]).slice(0,25).map(([k,v])=>Math.round(v[1]/1e6)+"MB "+v[0]+" "+k));
};
startSamplingProfiler();
let seen=0;
const profile=()=>{
 const data:any=samplingProfilerStackTraces(),traces=data.traces.slice(seen);seen=data.traces.length;
 if(!traces.length)return;
 const self=new Map<string,number>(),inclusive=new Map<string,number>(),label=(frame:any)=>frame.name+" "+String(frame.sourceURL).replace(/^.*🏗️build\//,"")+":"+frame.line;
 for(const trace of traces){const frames=trace.frames;if(!frames.length)continue;self.set(label(frames[0]),(self.get(label(frames[0]))??0)+1);for(const key of new Set<string>(frames.map(label)))inclusive.set(key,(inclusive.get(key)??0)+1);}
 const top=(map:Map<string,number>)=>[...map].sort((a,b)=>b[1]-a[1]).slice(0,14).map(([key,count])=>count+" "+key);
 log("profile",{samples:traces.length,self:top(self),inclusive:top(inclusive)});
};
const policy=readScriptPolicy({version:1,owner:"workspace-root",maximumElapsedMilliseconds:0});
issueRepositoryScriptProvidersV1(process.env);
const capabilities=await createBuildBootstrapCapabilitiesV1(root,["run","workspace:components","--","describe","norm"],process.env);
if(import.meta.main)await withScriptProcessEnvelope(createScriptProcessEnvelope(policy,capabilities,Date.now()),async(invocation:any)=>{
 const {discoverBuildGraphPartV1}=await import(B+"🕸️graph/🧭️discovery/🟦️.ts") as any;
 const index=await import(B+"🕸️graph/📥️inference/🟨️.mjs") as any;await index.libraryBootstrap;
 await index.primeSharedClosuresOfWorkspaceV1({workspaceRoot:root});
 const registry=JSON.parse(readFileSync(root+"/📋️build-owners.json","utf8"));
 const files=execFileSync("git",["ls-files","-z"],{cwd:root,maxBuffer:1<<30}).toString().split("\0").filter(Boolean);
 
 const started=performance.now(),tick=setInterval(()=>{const h=heapStats(),m=process.memoryUsage();log("tick",{s:Math.round((performance.now()-started)/1000),rssMB:Math.round(m.rss/1e6),heapMB:Math.round(h.heapSize/1e6),extMB:Math.round(h.extraMemorySize/1e6)});profile();},Number(process.env.PROFILE_MS??30000));const quick=setInterval(()=>{const m=process.memoryUsage();log("mem",Math.round((performance.now()-started)/1000),Math.round(m.rss/1e6));if(!snapped&&m.rss>2.6e9){snapped=true;profile();snapshotReport();}if(m.rss>9e9){profile();log("abort rss",Math.round(m.rss/1e6));process.exit(3);}},500);
 let owners=0;
 try{
  const part=await withIssuedResourceControlV1(invocation,"graphmem4-part",0,(phase:any)=>withRepositoryBuildGraphFactStoreV1(root,invocation,phase,async(_store:any,control:any)=>discoverBuildGraphPartV1(root,registry,invocation,control,files,{cold:true,roots:new Set<string>(),stubs:{},previousEnv:null,ownerDone:()=>{owners++;if(owners%10===0)log("owners",owners,Math.round((performance.now()-started)/1000));}})));
  log("part done",{s:Math.round((performance.now()-started)/1000),owners:part.owners.length,rows:part.rows.length,edges:part.edges.length});part.dispose?.();
 }finally{clearInterval(tick);clearInterval(quick);profile();}
});
