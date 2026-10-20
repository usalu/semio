// Proves the staged BUILD declares, per sampled cacheable task, exactly the files the base selector + classifier name (Bazel query vs base adapter).
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {createHash} from "node:crypto";
import {spawnSync} from "node:child_process";
import {receiveScriptProcessInvocation} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const LIB="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const oldApi=await import(LIB),api=await import(process.env.BZL_STAGE_ADAPTER??LIB);
const dir=join(import.meta.dir,"..","🗑️generated","bzl-stage"),graph=JSON.parse(readFileSync(join(dir,"graph.json"),"utf8"));
const names=new Set<string>();for(const p of Object.values<any>(graph.projects))for(const t of Object.keys(p.targets))names.add(p.name+":"+t);
for(const p of Object.values<any>(graph.projects))for(const t of Object.values<any>(p.targets)){if(!Array.isArray(t.dependsOn))continue;t.dependsOn=t.dependsOn.filter((d:any)=>{const s=typeof d==="string"?d:d.target;const op=s.replace(/^\^/,"");return !op.includes(":")||names.has(op.split(":").slice(0,2).join(":"));});}
const all=Object.values<any>(graph.projects).flatMap(p=>Object.keys(p.targets).map(t=>p.name+":"+t));
const plan=api.planBazelTasksV1(graph,{tasks:all,args:[]}),em=api.bazelActionsV1(graph,plan,false);
const files=[...new Set(spawnSync("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z"],{cwd:"/Users/ueli/Documents/semio",maxBuffer:1<<30}).stdout.toString().split("\0").filter(Boolean))].filter(p=>!p.startsWith(".🧬semio/")&&!p.startsWith(".git/"));
const declared=[...em.values()].filter((a:any)=>a.kind==="task"&&a.cache.cache);
const step=Number(process.env.STEP??"60"),sample=declared.filter((_a:any,i:number)=>i%step===0);
const sizes=declared.map((a:any)=>[a,api.bazelTaskInputsV1(graph,plan.tasks[a.id]).length] as const).sort((x,y)=>y[1]-x[1]).slice(0,5).map(x=>x[0]);
let bad=0,checked=0;
await receiveScriptProcessInvocation(process.env,async original=>{
 const classes=new Map(),boundaries=new Map();
 for(const a of [...sample,...sizes]){
  const values=api.bazelTaskInputsV1(graph,plan.tasks[a.id]),paths=oldApi.bazelTaskSourcePathsV1(values,files),cls=await oldApi.bazelSourceClassesV1("/Users/ueli/Documents/semio",paths,classes,boundaries,original),expected=[...cls.files].sort();
  const label="t_"+createHash("sha256").update(a.id).digest("hex").slice(0,16);
  const q=spawnSync(join(import.meta.dir,"bz.sh"),["query",`kind('source file', deps(labels(srcs, //:${label})))`,"--output=label"],{encoding:"utf8",maxBuffer:1<<30});
  const actual=q.stdout.split("\n").filter((l:string)=>l.startsWith("//:s/")).map((l:string)=>l.slice(5)).sort();
  const same=actual.length===expected.length&&actual.every((v:string,i:number)=>v===expected[i]);checked++;if(!same){bad++;console.log("MISMATCH",a.id,expected.length,actual.length,q.stderr.slice(0,200));}
  else console.log("ok",a.id.slice(0,60).padEnd(60),expected.length);
 }
});
console.log("checked",checked,"mismatches",bad);
