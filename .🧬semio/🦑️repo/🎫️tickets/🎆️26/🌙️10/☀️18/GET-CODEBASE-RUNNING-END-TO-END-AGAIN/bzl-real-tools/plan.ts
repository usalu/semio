// 🔭 Prints the planned closure and declared input patterns of candidate tasks against the static graph.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const B="../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const api=await import(B);
const graph=JSON.parse(readFileSync(process.env.BZL_REAL_GRAPH??join(import.meta.dir,"..","🗑️generated","bzl-breadth","graph.json"),"utf8"));
for(const id of process.argv.slice(2)){
 const plan=api.planBazelTasksV1(graph,{tasks:[id],args:[]}),task=plan.tasks[id],inputs=api.bazelTaskInputsV1(graph,task),acts=api.bazelActionsV1(graph,plan,false);
 console.log(JSON.stringify({id,selected:plan.selected,closure:Object.keys(plan.tasks).length,cache:[...acts.values()].map(a=>a.id+"="+(a.cache.cache?"cached":a.cache.reason)),inputs}));
}
