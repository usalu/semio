// 📊 Counts the declared input files of every cacheable task on the reconstructed graph with the adapter's own input expansion and source index; the cost drivers of Bazel analysis and input hashing.
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const B="../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const api=await import(process.env.BZL_LAZY_ADAPTER??B),root="/Users/ueli/Documents/semio",dir=join(import.meta.dir,"..","🗑️generated","input-precision"),graph=JSON.parse(readFileSync(join(dir,"graph.json"),"utf8"));
const git=spawnSync("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z"],{cwd:root,maxBuffer:512*1024*1024});
const inventory=[...new Set(git.stdout.toString().split("\0").filter(Boolean))].filter(path=>!path.startsWith(".🧬semio/")&&!path.startsWith(".git/")).sort();
const t0=performance.now(),index=api.bazelSourceIndexV1(inventory),indexMs=Math.round(performance.now()-t0);
const names=new Set<string>();for(const p of Object.values<any>(graph.projects))for(const t of Object.keys(p.targets))names.add(p.name+":"+t);
for(const p of Object.values<any>(graph.projects))for(const t of Object.values<any>(p.targets)){if(!Array.isArray(t.dependsOn))continue;const kept=t.dependsOn.filter((d:any)=>{const op=(typeof d==="string"?d:d.target).replace(/^\^/,"");return !op.includes(":")||names.has(op.split(":").slice(0,2).join(":"));});if(kept.length)t.dependsOn=kept;else delete t.dependsOn;}
const ids=Object.values<any>(graph.projects).flatMap((p:any)=>Object.keys(p.targets).map(t=>p.name+":"+t));
const plan=api.planBazelTasksV1(graph,{tasks:ids,args:[]}),emitted=api.bazelActionsV1(graph,plan,false);
const rows:{id:string;project:string;target:string;files:number;patterns:number}[]=[];
for(const action of emitted.values()){
 if(!((action.kind==="task"||action.kind==="test")&&action.cache.cache))continue;
 const task=plan.tasks[action.id],values=api.bazelTaskInputsV1(graph,task,"ignore"),patterns=values.filter((v:unknown)=>typeof v==="string"&&!(v as string).startsWith("!")).length,{marks,extra}=index.selectAll(values);
 let files=extra.length;for(let i=0;i<marks.length;i++)if(marks[i])files++;
 rows.push({id:action.id,project:task.project,target:task.target,files,patterns});
}
rows.sort((a,b)=>b.files-a.files);
const bucket=(n:number)=>rows.filter(r=>r.files>=n).length;
const summary={inventory:inventory.length,indexMs,cachedTasks:rows.length,over100:bucket(100),over1000:bucket(1000),over10000:bucket(10000),over50000:bucket(50000),over100000:bucket(100000),median:rows[Math.floor(rows.length/2)]?.files,top:rows.slice(0,40)};
writeFileSync(join(dir,"inputs-census.json"),JSON.stringify({...summary,all:rows}));
console.log(JSON.stringify(summary,null,1));
