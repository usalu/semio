// [graphlat-g] prefetch-vs-build inference cost of chosen owners on the live tree in a scratch record store (graph-free: no lease, no repo run, no live cache writes).
// env: OWNERS=root1,root2 (project roots), PASSES=prefetch,build,build (each: prefetch = analyzeLockfile:false as in prefetchBuildGraphProjectsV1, build = analyzeLockfile:true as in discoverBuildGraphPartV1), GP_SCRATCH, SEMIO_GRAPH_WORKERS
import{installVariant}from"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graph-parallel-tools/harness/variant.mjs";
import{WORKSPACE,LIBRARY,SCRATCH}from"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graph-parallel-tools/harness/variant.mjs";
installVariant();
import{existsSync,mkdirSync,rmSync,readFileSync}from"node:fs";import{join}from"node:path";import{spawnSync}from"node:child_process";
const E=process.env,mb=n=>Math.round(n/1048576);
if(!E.KEEP)rmSync(SCRATCH,{recursive:true,force:true});mkdirSync(SCRATCH,{recursive:true});
const inference=await import(join(LIBRARY,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const projectNodes=await import(join(LIBRARY,"🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs")),observation=await import(join(LIBRARY,"🏗️build/👁️observation/🟨️.mjs"));
const{withImplementationPinV1}=await import(join(LIBRARY,"🏗️build/📥️inputs/🔏️revision/🟨️.mjs")),admission=await import(join(LIBRARY,"🏗️build/🕸️graph/🚪️admission/🟦️.ts"));
const registry=admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(join(WORKSPACE,"📋️build-owners.json"),"utf8")));
const listed=spawnSync("git",["-C",WORKSPACE,"ls-files","-co","--exclude-standard","-z"],{maxBuffer:1<<30}).stdout.toString("utf8").split("\0").filter(Boolean);
const lockNames=["bun.lock"];
const files=listed.filter(f=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")||lockNames.includes(f)||f.endsWith(".patch")).filter(f=>existsSync(join(WORKSPACE,f)));
const ownerRoots=new Set((E.OWNERS??"").split(",").filter(Boolean));
const context={workspaceRoot:WORKSPACE,registry,preparationControl:{advance:async()=>{}}};
globalThis.__asmMark=undefined;
console.error("[DEBUG] files",files.length,"owners",[...ownerRoots].join(","));
for(const[at,pass]of(E.PASSES??"prefetch,build").split(",").entries()){
 const options={analyzeLockfile:pass==="build",catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots,onOwnerObservations:()=>{}};
 const cpu0=process.cpuUsage(),t0=performance.now();let rows=0;
 const result=await withImplementationPinV1(()=>observation.captureObservedSourceInputsV1(async()=>(await projectNodes.emojiProjectJsonNodes(files,{...options,onOwnerObservations:(name,r)=>{rows+=r.length;}},context)).length,undefined,false));
 const cpu=process.cpuUsage(cpu0);
 console.log(JSON.stringify({pass,at,wallS:+((performance.now()-t0)/1000).toFixed(1),cpuS:+((cpu.user+cpu.system)/1e6).toFixed(1),userS:+(cpu.user/1e6).toFixed(1),sysS:+(cpu.system/1e6).toFixed(1),rss:mb(process.memoryUsage().rss),observationRows:rows,captured:result.observations?.length,value:result.value}));
}
process.exit(0);
