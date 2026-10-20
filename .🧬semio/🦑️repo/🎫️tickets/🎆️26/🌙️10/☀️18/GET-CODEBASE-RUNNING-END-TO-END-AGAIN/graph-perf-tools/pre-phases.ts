// ⏱️ Times the pre-project phase `discoverNativeBuildOwnersV1` (owner discovery) on the live tree, read-only, with heap and CPU.
import {join} from "node:path";import {existsSync} from "node:fs";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),strip=(s:string)=>s.replace(/️/g,"");
const inference:any=await import(join(L,"🏗️build/🕸️graph/📥️inference/🟨️.mjs"));await inference.libraryBootstrap;
const observation:any=await import(join(L,"🏗️build/👁️observation/🟨️.mjs"));
const tracked=new TextDecoder().decode(Bun.spawnSync(["git","-c","core.quotepath=false","ls-files","-z"],{cwd:W}).stdout).split("\0").filter(f=>f&&/(?:^|\/)(?:📋️project\.json|Cargo\.toml)$/u.test(f)&&existsSync(join(W,f)));
console.log("configFiles",tracked.length);
const cpu0=process.cpuUsage(),t0=performance.now();let phases=0;
const context={workspaceRoot:W};
const run=async()=>inference.discoverNativeBuildOwnersV1(tracked,W,context);
const result=await observation.observeInference({cachePath:"/private/tmp/gp/pre-phases.json",key:["pre",Date.now()]},()=>observation.settleObservedComputationV1(run,{}));
const cpu=process.cpuUsage(cpu0);
console.log(JSON.stringify({wallS:+((performance.now()-t0)/1000).toFixed(1),cpuS:+((cpu.user+cpu.system)/1e6).toFixed(1),heapMB:Math.round(process.memoryUsage().heapUsed/1e6),rssMB:Math.round(process.memoryUsage().rss/1e6),files:result.files.length,keys:Object.keys(result)}));
process.exit(0);
