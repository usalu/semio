// 🔭 Prints the planner closure size of candidate tasks on the statically reconstructed graph, by the real adapter planner.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const B="../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const api=await import(process.env.BZL_LAZY_ADAPTER??B),dir=join(import.meta.dir,"..","🗑️generated","bzl-lazy"),graph=JSON.parse(readFileSync(process.env.BZL_LAZY_GRAPH??join(dir,"graph.json"),"utf8"));
const ids=process.argv.slice(2);
for(const id of ids){
 try{const plan=api.planBazelTasksV1(graph,{tasks:[id]}),emitted=api.bazelActionsV1(graph,plan,false),cached=[...emitted.values()].filter(a=>a.kind==="task"&&a.cache.cache).length,projects=new Set(Object.values<any>(plan.tasks).map(t=>t.project));
  console.log([id,Object.keys(plan.tasks).length,cached,projects.size].join("\t"));}
 catch(e:any){console.log(id+"\tERR\t"+e.message.slice(0,160));}
}
