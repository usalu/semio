import {join} from "node:path";
import {planBuildTasksV1} from "../../../🧮️task-model/🟦️.ts";
import {affectedBuildProjectsV1} from "../../../🧭️selection/🟦️.ts";
import {conformanceGraphV1} from "../../../📐️conformance/🟦️.ts";
import type {ConformanceDriverV1,ConformancePrepareV1,ConformanceSessionV1} from "../../../📐️conformance/🏃️runner/🟦️.ts";
import {SemioStoreV1} from "../🗄️store/🟦️.ts";
import {semioHttpRemoteV1} from "../🌐️remote/🟦️.ts";
import {semioLocalExecutorV1} from "../🚀️exec/🟦️.ts";
import {runSemioTasksV1,semioInventoryV1} from "../🏁️scheduler/🟦️.ts";
import type {SemioEventV1} from "../🟦️.ts";

/** 🔌️ Binds the in-house engine to the conformance suite: the stub workspace becomes a neutral graph that the engine plans, keys, runs, restores and records in this process. */
export const driver:ConformanceDriverV1=Object.freeze({
 id:"semio",
 async prepare(input:ConformancePrepareV1):Promise<ConformanceSessionV1>{
  const graph=conformanceGraphV1(input.workspace,input.directory,input.runtime,input.stub,input.specPath),store=new SemioStoreV1(join(input.cache,"semio"));
  return {
   observes:new Set(["progress","run-record"]),
   async plan(target){const plan=planBuildTasksV1(graph,{tasks:[target]});return {tasks:Object.keys(plan.tasks),dependencies:Object.fromEntries(Object.entries(plan.tasks).map(([id,task])=>[id,task.dependencies]))};},
   async run(targets,options,signal){
    const started=Date.now(),events:SemioEventV1[]=[],remote=options.remote==="read-write"&&input.remote?semioHttpRemoteV1({url:input.remote.url}):undefined;
    const record=await runSemioTasksV1(graph,{tasks:[...targets],...(options.parallel?{parallel:options.parallel}:{}),...(options.skipCache?{skipCache:true}:{})},{workspace:input.directory,store,...(remote?{remote}:{}),inventory:()=>semioInventoryV1(input.directory),execute:semioLocalExecutorV1(input.directory,()=>({...process.env})),environment:process.env,now:Date.now,write:()=>{}},{signal,onEvent:event=>{events.push(event);}});
    return {exitCode:record.exitCode,milliseconds:Date.now()-started,restored:record.tasks.filter(task=>task.status==="restored").map(task=>task.id),events:events.map(event=>({kind:event.kind,task:event.task,...(event.status?{status:event.status}:{})}))};
   },
   async affected(files){return affectedBuildProjectsV1(graph,files);},
   async clearLocalCache(){await store.clear();},
   async close(){}
  };
 }
});
