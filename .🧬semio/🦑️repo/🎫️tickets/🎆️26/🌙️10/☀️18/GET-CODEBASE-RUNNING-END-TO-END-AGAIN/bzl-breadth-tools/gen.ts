// 🪡 Feeds the statically reconstructed graph to the Bazel adapter generator and writes the workspace.
import {mkdirSync,readFileSync,writeFileSync,rmSync,existsSync} from "node:fs";
import {join} from "node:path";
import {receiveScriptProcessInvocation,createIssuedResourceControlV1} from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const B="../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const api=await import(B);
const root=process.cwd(),dir=join(import.meta.dir,"..","🗑️generated","bzl-breadth"),graphPath=process.env.BZL_BREADTH_GRAPH??join(dir,"graph.json"),ws=process.env.BZL_BREADTH_WS!,report:any={dangling:[],cycles:[],notes:[]};
if(!ws)throw Error("BZL_BREADTH_WS required");
const graph=JSON.parse(readFileSync(graphPath,"utf8"));
const names=new Set<string>();for(const p of Object.values<any>(graph.projects))for(const t of Object.keys(p.targets))names.add(p.name+":"+t);
const prune=process.env.BZL_BREADTH_PRUNE!=="0";
if(prune)for(const p of Object.values<any>(graph.projects))for(const [tn,t] of Object.entries<any>(p.targets)){
 if(!Array.isArray(t.dependsOn))continue;
 const kept=t.dependsOn.filter((d:any)=>{const spec=typeof d==="string"?d:d.target;const op=spec.replace(/^\^/,"");if(!op.includes(":"))return true;const id=op.split(":").slice(0,2).join(":");if(names.has(id))return true;report.dangling.push({task:p.name+":"+tn,dependency:spec});return false;});
 if(kept.length!==t.dependsOn.length){if(kept.length)t.dependsOn=kept;else delete t.dependsOn;}
}
const all=Object.values<any>(graph.projects).flatMap(p=>Object.keys(p.targets).map(t=>p.name+":"+t));
const request={tasks:all,args:[] as string[]};
let plan:any;
for(let attempt=0;attempt<200;attempt++){
 try{plan=api.planBazelTasksV1(graph,request);break;}catch(error:any){
  const m=/^Build task dependency cycle: (.+)$/.exec(error.message);
  if(!m||!prune)throw error;
  const chain=m[1]!.split(" -> "),from=chain.at(-2)!,to=chain.at(-1)!,[project,target]=[from.split(":")[0]!,from.split(":")[1]!],t=graph.projects[project].targets[target];
  report.cycles.push({chain:m[1]});
  t.dependsOn=(t.dependsOn??[]).filter((d:any)=>{const spec=typeof d==="string"?d:d.target;const op=spec.replace(/^\^/,"");return !(op===to||(!op.includes(":")&&project+":"+op===to&&false));});
  if(!t.dependsOn.length)delete t.dependsOn;
 }
}
if(!plan)throw Error("no plan");
report.plan={tasks:Object.keys(plan.tasks).length,selected:plan.selected.length};
mkdirSync(ws,{recursive:true});
await receiveScriptProcessInvocation(process.env,async original=>{
 const phase=createIssuedResourceControlV1(original,"cache:entry",0);
 const started=performance.now();
 const implementation=await api.publishBazelImplementationV1(root,ws,original);
 report.implementationMs=Math.round(performance.now()-started);
 const t1=performance.now();
 await api.generateBazelWorkspaceV1(root,graph,request,plan,ws,original,phase,implementation,()=>{});
 report.generateMs=Math.round(performance.now()-t1);
});
const emitted=api.bazelActionsV1(graph,plan,false),kinds:Record<string,number>={},cache:Record<string,number>={};
for(const a of emitted.values()){kinds[a.kind]=(kinds[a.kind]??0)+1;const k=a.cache.cache?"cached":"uncached:"+a.cache.reason;cache[k]=(cache[k]??0)+1;}
report.actions={emitted:emitted.size,kinds,cache};
writeFileSync(join(dir,"gen-report.json"),JSON.stringify(report,null,1));
console.log(JSON.stringify({...report,dangling:report.dangling.length,cycles:report.cycles.length}));
