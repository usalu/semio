import {readFileSync} from "node:fs";
import {join} from "node:path";
const B=process.env.BZL_STAGE_ADAPTER??"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const api=await import(B);
const graph=JSON.parse(readFileSync(join(import.meta.dir,"..","🗑️generated","bzl-stage","graph.json"),"utf8"));
const names=new Set<string>();for(const p of Object.values<any>(graph.projects))for(const t of Object.keys(p.targets))names.add(p.name+":"+t);
for(const p of Object.values<any>(graph.projects))for(const t of Object.values<any>(p.targets)){if(!Array.isArray(t.dependsOn))continue;t.dependsOn=t.dependsOn.filter((d:any)=>{const s=typeof d==="string"?d:d.target;const op=s.replace(/^\^/,"");return !op.includes(":")||names.has(op.split(":").slice(0,2).join(":"));});}
const all=Object.values<any>(graph.projects).flatMap(p=>Object.keys(p.targets).map(t=>p.name+":"+t));
const plan=api.planBazelTasksV1(graph,{tasks:all,args:[]});
const em=api.bazelActionsV1(graph,plan,false);
const pat=new Map<string,number>(),objs=new Map<string,number>();let cacheable=0,total=0;
for(const a of em.values()){if(a.kind!=="task"||!a.cache.cache)continue;cacheable++;const ins=api.bazelTaskInputsV1(graph,plan.tasks[a.id]);for(const i of ins){if(typeof i==="string"){total++;pat.set(i,(pat.get(i)??0)+1);}else{const k=JSON.stringify(i);objs.set(k,(objs.get(k)??0)+1);}}}
console.log({cacheable,totalStringInputs:total,distinctPatterns:pat.size,objectKinds:objs.size});
const top=[...pat].sort((a,b)=>b[1]-a[1]);
console.log(top.slice(0,25));
const shapes=new Map<string,number>();for(const [p] of pat){const s=p.replace(/^!/,"").replace(/[^/*?{}\[\]()!+@]+/g,(m)=>m.length>12?"X":m);}
const special=[...pat.keys()].filter(p=>/[{}\[\]()]/.test(p)&&!p.includes("{projectRoot}"));console.log("special",special.length,special.slice(0,20));
console.log([...objs].sort((a,b)=>b[1]-a[1]).slice(0,15));
// ---- keyed grouping prototype
import {spawnSync} from "node:child_process";
const gl=spawnSync("git",["-c","core.fsmonitor=false","ls-files","-co","--exclude-standard","-z"],{cwd:"/Users/ueli/Documents/semio",maxBuffer:1<<30});
const files=[...new Set(gl.stdout.toString().split("\0").filter(Boolean))].filter(p=>!p.startsWith(".🧬semio/")&&!p.startsWith(".git/")).sort();
console.log("files",files.length);
const prefixOf=(p:string)=>{const i=p.search(/[?*{[()!+@]/);const pre=i<0?p:p.slice(0,i);return i<0?pre:pre.slice(0,pre.lastIndexOf("/")+1);};
const keyed=new Map<string,string[]>();let tasks=0,labelsOld=0,labelsKeyed=0,t0=performance.now();
const seenSel=new Map<string,string[]>();
const keyFiles=new Map<string,string[]>();
for(const a of em.values()){if(a.kind!=="task"||!a.cache.cache)continue;tasks++;
 const ins=api.bazelTaskInputsV1(graph,plan.tasks[a.id]).filter((v:any)=>typeof v==="string") as string[];
 const inc=ins.filter(v=>!v.startsWith("!")),exc=ins.filter(v=>v.startsWith("!"));
 const excBy=exc.map(v=>[prefixOf(v.slice(1)),v] as [string,string]);
 const gs=new Set<string>();
 for(const p of new Set(inc)){const pre=prefixOf(p);const rel=excBy.filter(([q])=>q.startsWith(pre)||pre.startsWith(q)).map(x=>x[1]).sort();const key=JSON.stringify([p,rel]);gs.add(key);if(!keyFiles.has(key)){keyFiles.set(key,api.bazelTaskSourcePathsV1([p,...rel],files));}}
}
let tot=0;const content=new Set<string>();let uniq=0;for(const [k,l] of keyFiles){tot+=l.length;const h=l.join("\0");if(!content.has(h)){content.add(h);uniq+=l.length;}}
console.log({tasks,keys:keyFiles.size,totalLabels:tot,distinctContents:content.size,uniqLabels:uniq,ms:Math.round(performance.now()-t0)});
