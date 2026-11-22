import {test} from "bun:test";
import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {join} from "node:path";
import {withCommandSourceControlV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🧰️support/🟦️.ts";
const workspaceRoot="/Users/ueli/Documents/semio",B=workspaceRoot+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const OUT=process.env.E40_OUT??"/private/tmp/claude-501/-Users-ueli--claude-spares-spare-kfaIfm/ccee8eb5-3363-4a84-af9e-53e38d236627/scratchpad/E40/measure.json";
const N=Number(process.env.E40_N??20);
test("measure",async()=>{
 const inf=await import(B+"/🕸️graph/📥️inference/🟨️.mjs"),adm=await import(B+"/🕸️graph/🚪️admission/🟦️.ts");
 await inf.libraryBootstrap;
 const registry=adm.readBuildOwnerRegistryV1(JSON.parse(inf.readBuildOwnerRegistrySourceV1(workspaceRoot)));
 const crates=registry.owners.filter((o:any)=>o.manifest.endsWith("Cargo.toml")),full=registry.owners.filter((o:any)=>!o.manifest.endsWith("Cargo.toml")&&existsSync(join(workspaceRoot,o.root,"Cargo.toml")));
 const pick=(a:any[],n:number)=>a.filter((_,i)=>i%Math.max(1,Math.floor(a.length/n))===0).slice(0,n);
 const sample={crates:pick(crates,N),full:pick(full,Math.min(N,10))};
 const result:any={counts:{crates:crates.length,full:full.length},probe:{},infer:{}};
 const reason=Error("stop");
 for(const kind of ["crates","full"] as const){
  const rows:any[]=[];
  await withCommandSourceControlV1(workspaceRoot,"e40-probe",3600000,reason,async(d:any)=>{
   const nativeCache=new Map(),nativeClosures=new Map();
   for(const owner of (sample as any)[kind]){
    const before=d.events.length,t=performance.now(),c=process.cpuUsage();
    let facts:any;try{facts=await inf.discoverBuildOwnerAdmissionFactsV1(owner,[],registry,{workspaceRoot,invocation:d.invocation,phaseControl:d.phaseControl,nativeCache,nativeClosures,onOwnerDiscoveryPhase(){}},[]);}catch(e:any){rows.push({root:owner.root,error:String(e.message).slice(0,200)});continue;}
    const u=process.cpuUsage(c);rows.push({root:owner.root,wall:Math.round(performance.now()-t),cpu:Math.round((u.user+u.system)/1000),advances:d.events.length-before,files:facts.files.length,deps:facts.dependencies.length,digest:Bun.hash(JSON.stringify(facts)).toString(16)});
   }
  });
  result.probe[kind]=rows;
 }
 const h=await (await import(B+"/🕸️graph/📥️inference/🔬️owner-harness/🟨️.mjs")).openOwnerHarnessV1({workspaceRoot});
 try{
  const ct=await import(B+"/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs"),nodes=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"),service=await import(B+"/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️service/🟨️.mjs"),obs=await import(B+"/👁️observation/🟨️.mjs"),st=await import(B+"/📥️inputs/💾️storage/🟨️.mjs");
  const env=h.environment,globals={registryDigest:"harness",commandRevision:"harness",late:false};
  const run=async(owner:any,selected?:any)=>{
   let json:any;const pj=join(workspaceRoot,owner.root,"📋️project.json");
   if(existsSync(pj))json={...JSON.parse(readFileSync(pj,"utf8")),root:owner.root};
   else json={name:owner.name,root:owner.root,projectType:"library",tags:["language:cargo","discovery:manifest"],targets:{...await ct.cargoTargets(owner.root,workspaceRoot,env.commandInputs,env.scripts),...ct.componentBuildTargets(owner.root,workspaceRoot)}};
   const job=nodes.ownerJobV1(env,globals,{name:json.name,json,...selected===undefined?{}:{admission:{targets:selected.targets,inputs:[]}}});
   const t=performance.now(),c=process.cpuUsage();await service.run({...job,...selected===undefined?{}:{admission:true}});const u=process.cpuUsage(c),wall=performance.now()-t;
   const cachePath=join(service.stagingDirectoryV1(),"inference","np",job.cacheKey+".json"),rec=st.withBuildGraphStagingV1({root:service.stagingDirectoryV1()},()=>obs.exportProjectRecordV1(cachePath));
   return {root:owner.root,wall:Math.round(wall),cpu:Math.round((u.user+u.system)/1000),rows:rec?.observations.length??0,bytes:rec?.image.length??0,digest:rec?Bun.hash(rec.image).toString(16):"none"};
  };
  for(const [label,list,sel] of [["crates-full",sample.crates,undefined],["crates-selected-none",sample.crates,{targets:[]}],["full-full",sample.full,undefined]] as const){
   const rows:any[]=[];for(const o of list as any[]){try{rows.push(await run(o,sel));}catch(e:any){rows.push({root:o.root,error:String(e.message).slice(0,200)});}}result.infer[label]=rows;
  }
 }finally{h.close();}
 writeFileSync(OUT,JSON.stringify(result,null,1));
},3000000);
