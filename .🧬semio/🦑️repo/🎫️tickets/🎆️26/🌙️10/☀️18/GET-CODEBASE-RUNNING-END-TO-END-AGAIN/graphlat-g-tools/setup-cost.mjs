// [graphlat-g] cost of the whole-workspace setup `emojiProjectJsonNodes` repeats on every call (native owner discovery over every Cargo.toml), twice in one process: is the second call cheaper (memo) or the same (the prefetch and the build each pay it)?
import{installVariant}from"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graph-parallel-tools/harness/variant.mjs";
import{WORKSPACE,LIBRARY,SCRATCH}from"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graph-parallel-tools/harness/variant.mjs";
installVariant();
import{existsSync,mkdirSync,rmSync}from"node:fs";import{join}from"node:path";import{spawnSync}from"node:child_process";
rmSync(SCRATCH,{recursive:true,force:true});mkdirSync(SCRATCH,{recursive:true});
const inference=await import(join(LIBRARY,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const discovery=await import(join(LIBRARY,"🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs")),observation=await import(join(LIBRARY,"🏗️build/👁️observation/🟨️.mjs")),{withImplementationPinV1}=await import(join(LIBRARY,"🏗️build/📥️inputs/🔏️revision/🟨️.mjs"));
const listed=spawnSync("git",["-C",WORKSPACE,"ls-files","-co","--exclude-standard","-z"],{maxBuffer:1<<30}).stdout.toString("utf8").split("\0").filter(Boolean);
const files=listed.filter(f=>f.endsWith("📋️project.json")||f.endsWith("Cargo.toml")).filter(f=>existsSync(join(WORKSPACE,f)));
const context={workspaceRoot:WORKSPACE,preparationControl:{advance:async()=>{}}};
for(let pass=0;pass<3;pass++){
 const cpu0=process.cpuUsage(),t0=performance.now();
 const r=await withImplementationPinV1(()=>observation.captureObservedSourceInputsV1(async()=>(await discovery.discoverNativeBuildOwnersV1(files,WORKSPACE,context)).files.length,undefined,false));
 const cpu=process.cpuUsage(cpu0);console.log(JSON.stringify({pass,wallS:+((performance.now()-t0)/1000).toFixed(1),cpuS:+((cpu.user+cpu.system)/1e6).toFixed(1),files:r.value,observationRows:r.observations.length,rss:Math.round(process.memoryUsage().rss/1048576)}));
}
process.exit(0);
