// usage (from the overlay root): bun census.ts <graph.json> <out.json>  -- ranks hermetic classes over a static task graph with every project admitted.
import {readFileSync,writeFileSync} from "node:fs";
import {HERMETIC_TABLE_V1,hermeticInputsV1,hermeticSelectV1} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏃️execution/🔒️hermetic/🟦️.ts";
import {bazelTaskInputsV1,bazelCacheDecisionV1} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
const graph=JSON.parse(readFileSync(process.argv[2]!,"utf8"));
const open={...HERMETIC_TABLE_V1,classes:HERMETIC_TABLE_V1.classes.map(row=>({...row,select:{...row.select,projects:["**"]}}))};
const count:Record<string,Record<string,number>>={},total:Record<string,number>={};let cacheable=0,tasks=0;
const bump=(key:string,reason:string)=>{(count[key]??={})[reason]=((count[key]??={})[reason]??0)+1;};
for(const[project,owner]of Object.entries(graph.projects as Record<string,any>))for(const[target,definition]of Object.entries(owner.targets as Record<string,any>)){
 tasks++;const task={id:project+":"+target,project,target,root:owner.root,label:"t",dependencies:[],args:[],definition} as any;
 let decision;try{decision=bazelCacheDecisionV1(task,false);}catch{continue;}
 if(!decision.cache)continue;cacheable++;
 const selected=hermeticSelectV1({project,target,root:owner.root},open as never);
 if(selected.kind!=="class"){bump("none","unselected");continue;}
 const id=selected.class.id,execution=definition.execution,commands=execution.commands??(execution.command===undefined?[]:[execution.command]);
 total[id]=(total[id]??0)+1;
 if(execution.kind!=="command"||execution.program!==undefined||execution.ownerArguments!==undefined||execution.readyWhen!==undefined||!commands.length||/\{options\.|\{env\./u.test(JSON.stringify([commands,execution.cwd,execution.env,definition.outputs]))){bump(id,"unsupported-execution");continue;}
 if(!Array.isArray(definition.outputs)){bump(id,"uncached");continue;}
 let inputs:unknown[];try{inputs=bazelTaskInputsV1(graph,task,"ignore");}catch{bump(id,"inputs-error");continue;}
 const host=hermeticInputsV1(inputs,{});if("refused" in host){bump(id,"host-inputs");continue;}
 const whole=(definition.inputs??[]).some((input:unknown)=>typeof input==="string"&&/^(\{workspaceRoot\}\/)?\*\*(\/\*)?$/u.test(input));
 bump(id,whole?"eligible-whole-repo-inputs":"eligible");
}
writeFileSync(process.argv[3]!,JSON.stringify({tasks,cacheable,total,count},null,1));
console.log(JSON.stringify({tasks,cacheable,total,count},null,1));
