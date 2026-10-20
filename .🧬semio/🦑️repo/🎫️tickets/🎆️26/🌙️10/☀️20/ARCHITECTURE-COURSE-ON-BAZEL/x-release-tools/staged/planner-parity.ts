/** 🧪️ Runs only the Bazel half of the existing Nx dependency-planner parity corpus (the Nx half needs an installed Nx): the emitted adapter must keep yielding the corpus' expected dependency edges. */
import {expect,test} from "bun:test";
import {resolve} from "node:path";
const corpus=await Bun.file(resolve(import.meta.dir,"../../../../../../../../..","🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧪️tests/🕸️dependencies/🔣️.json")).json();

const api=await import(resolve(import.meta.dir,"../../🗑️generated/x-release/check",process.env.X_RELEASE_MODE??"patched","📜️script.ts"));

for(const row of corpus.cases)test("emitted Bazel planner keeps the Nx-derived edges: "+row.name,()=>{
 const graph={version:1,projects:Object.fromEntries(Object.entries(row.projects).map(([name,targets])=>[name,{name,root:name,targets:Object.fromEntries(Object.entries(targets as Record<string,string[]>).map(([target,dependsOn])=>{const configured="targetConfigurations"in row?(row.targetConfigurations as Record<string,{defaultConfiguration?:string;available:string[]}>)[name+":"+target]:undefined;return[target,{...(configured?{defaultConfiguration:configured.defaultConfiguration,configurations:Object.fromEntries(configured.available.map(configuration=>[configuration,{execution:{}}]))}:{}),execution:{kind:"noop" as const},dependsOn}];}))}])),dependencies:row.edges.map(([source,target])=>({source:source!,target:target!,type:"static"}))},expected=Object.fromEntries(Object.entries(row.expected).map(([id,edges])=>[id,[...(edges as string[])].sort()]));
 const actual=api.planBazelTasksV1(graph,{tasks:row.selected,excludeDependencies:row.excludeDependencies,...("configuration"in row?{configuration:row.configuration}:{})});
 expect(Object.fromEntries(Object.entries(actual.tasks as Record<string,{dependencies:string[]}>).map(([id,task])=>[id,task.dependencies]))).toEqual(expected);
});
