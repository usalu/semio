// R196: view cost measurement. Run: BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 bun measure.ts (scratch tree under /private/tmp, views under a short directory on the same volume)
import {openTaskViewV1,digestTaskViewInputsV1,TASK_VIEW_POLICY_V1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🟦️.ts";
import {mkdirSync,writeFileSync,rmSync} from "node:fs";
import {randomBytes} from "node:crypto";
const base="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R196/exp/m";const sizes=[2500,30000];
for(const n of sizes){
 const w=base+"/live"+n;rmSync("/private/tmp/claude-501/R196v/"+n,{recursive:true,force:true});
 const paths:string[]=[];
 if(!Bun.file(w+"/.done").size){rmSync(w,{recursive:true,force:true});for(let i=0;i<n;i++){const p="d"+(i%200)+"/e"+(i%7)+"/f"+i+".rs";mkdirSync(w+"/"+p.slice(0,p.lastIndexOf("/")),{recursive:true});writeFileSync(w+"/"+p,randomBytes(3000).toString("hex"));}writeFileSync(w+"/.done","1");}
 const all:string[]=[];for(let i=0;i<n;i++)all.push("d"+(i%200)+"/e"+(i%7)+"/f"+i+".rs");
 let t=performance.now();const inputs=await digestTaskViewInputsV1(w,all);const hashMs=performance.now()-t;
 for(const mode of ["digest","stat"] as const){
  const policy={...TASK_VIEW_POLICY_V1,materialise:{...TASK_VIEW_POLICY_V1.materialise,verify:mode}} as typeof TASK_VIEW_POLICY_V1;
  t=performance.now();const r=await openTaskViewV1({source:w,directory:"/private/tmp/claude-501/R196v/"+n,hash:"h"+mode+n+"xx",inputs,policy});const ms=performance.now()-t;
  if(r.kind!=="ready")throw Error(JSON.stringify(r));
  t=performance.now();const again=await openTaskViewV1({source:w,directory:"/private/tmp/claude-501/R196v/"+n,hash:"h"+mode+n+"xx",inputs,policy});const reuse=performance.now()-t;
  console.log(JSON.stringify({files:n,verify:mode,strategy:r.view.receipt.strategy,openMs:Math.round(ms),reuseMs:Math.round(reuse),digestOfLiveMs:Math.round(hashMs),bytes:r.view.receipt.bytes}));
 }
}
