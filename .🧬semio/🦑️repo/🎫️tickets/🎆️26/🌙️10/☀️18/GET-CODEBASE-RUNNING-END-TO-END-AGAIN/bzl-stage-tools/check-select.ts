// Compares the base adapter's source selection with the staged selector + directory cover on real tasks.
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const oldApi=await import(join("/Users/ueli/Documents/semio",LIB));
const newApi=await import(join("/Users/ueli/Documents/semio/.cache/bzs-m",LIB));
const graph=JSON.parse(readFileSync(join(import.meta.dir,"..","🗑️generated","bzl-stage","graph.json"),"utf8"));
const names=new Set<string>();for(const p of Object.values<any>(graph.projects))for(const t of Object.keys(p.targets))names.add(p.name+":"+t);
for(const p of Object.values<any>(graph.projects))for(const t of Object.values<any>(p.targets)){if(!Array.isArray(t.dependsOn))continue;t.dependsOn=t.dependsOn.filter((d:any)=>{const s=typeof d==="string"?d:d.target;const op=s.replace(/^\^/,"");return !op.includes(":")||names.has(op.split(":").slice(0,2).join(":"));});}
const all=Object.values<any>(graph.projects).flatMap(p=>Object.keys(p.targets).map(t=>p.name+":"+t));
const plan=newApi.planBazelTasksV1(graph,{tasks:all,args:[]});
const em=newApi.bazelActionsV1(graph,plan,false);
const gl=spawnSync("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z"],{cwd:"/Users/ueli/Documents/semio",maxBuffer:1<<30});
const files=[...new Set(gl.stdout.toString().split("\0").filter(Boolean))].filter(p=>!p.startsWith(".🧬semio/")&&!p.startsWith(".git/")).sort();
console.log("files",files.length);
const declared=[...em.values()].filter(a=>a.kind==="task"&&a.cache.cache);
console.log("declared",declared.length);
const sample=declared.filter((_a,i)=>i%Number(process.env.STEP??"60")===0);
let t0=performance.now();const index=newApi.bazelSourceIndexV1(files);let fresh=0;
let bad=0;
for(const a of sample){
 const inputsNew=newApi.bazelTaskInputsV1(graph,plan.tasks[a.id]);
 const t1=performance.now();const o=oldApi.bazelTaskSourcePathsV1(inputsNew,files);const t2=performance.now();
 const {marks,extra}=index.select(inputsNew);const n=[...new Set([...index.files.filter((_p,i)=>marks[i]),...extra])].sort();const t3=performance.now();
 const same=o.length===n.length&&o.every((v:string,i:number)=>v===n[i]);
 if(!same){bad++;console.log("MISMATCH",a.id,o.length,n.length);}
 console.log(a.id.slice(0,70).padEnd(70),String(o.length).padStart(7),"old",Math.round(t2-t1)+"ms","new",Math.round(t3-t2)+"ms",same?"ok":"BAD");
}
console.log("sample",sample.length,"mismatches",bad,"total ms",Math.round(performance.now()-t0));
